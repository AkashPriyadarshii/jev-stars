use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub full_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub stargazers_count: i64,
    #[serde(default)]
    pub forks_count: i64,
    #[serde(default)]
    pub pushed_at: Option<String>,
    #[serde(default)]
    pub archived: bool,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub topics: Vec<String>,
    pub license: Option<License>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct License {
    #[serde(default)]
    pub spdx_id: Option<String>,
}

pub fn open(path: &str) -> Result<Connection> {
    let db = Connection::open(path).with_context(|| format!("open {path}"))?;
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS repos(
            full_name TEXT PRIMARY KEY, description TEXT, language TEXT,
            stars INTEGER, forks INTEGER, pushed_at TEXT, archived INTEGER,
            url TEXT, topics TEXT, license TEXT
        );
        CREATE VIRTUAL TABLE IF NOT EXISTS repos_fts USING fts5(
            full_name, description, language, topics,
            content='repos', content_rowid='rowid'
        );
        CREATE TABLE IF NOT EXISTS decisions(
            repo TEXT, content_hash TEXT, schema_v INTEGER, model TEXT,
            choice TEXT, score REAL, tags TEXT, evidence TEXT, ts TEXT,
            PRIMARY KEY(repo, content_hash, schema_v, model)
        );",
    )?;
    Ok(db)
}

pub fn upsert(db: &Connection, repos: &[Repo]) -> Result<usize> {
    let tx = db.unchecked_transaction()?;
    {
        let mut stmt = tx.prepare(
            "INSERT INTO repos(full_name,description,language,stars,forks,pushed_at,archived,url,topics,license)
             VALUES(?,?,?,?,?,?,?,?,?,?)
             ON CONFLICT(full_name) DO UPDATE SET
               description=excluded.description, language=excluded.language,
               stars=excluded.stars, forks=excluded.forks, pushed_at=excluded.pushed_at,
               archived=excluded.archived, url=excluded.url, topics=excluded.topics,
               license=excluded.license",
        )?;
        for r in repos {
            stmt.execute(rusqlite::params![
                r.full_name,
                r.description,
                r.language,
                r.stargazers_count,
                r.forks_count,
                r.pushed_at,
                r.archived as i64,
                r.html_url,
                r.topics.join(" "),
                r.license.as_ref().and_then(|l| l.spdx_id.clone()),
            ])?;
        }
    }
    tx.execute_batch(
        "INSERT INTO repos_fts(rowid,full_name,description,language,topics)
         SELECT rowid,full_name,description,language,topics FROM repos
         WHERE rowid NOT IN (SELECT rowid FROM repos_fts);",
    )?;
    tx.commit()?;
    Ok(repos.len())
}

pub fn counts(db: &Connection) -> Result<(i64, i64, i64)> {
    let total: i64 = db.query_row("SELECT COUNT(*) FROM repos", [], |r| r.get(0))?;
    let archived: i64 =
        db.query_row("SELECT COUNT(*) FROM repos WHERE archived=1", [], |r| {
            r.get(0)
        })?;
    let with_push: i64 = db.query_row(
        "SELECT COUNT(*) FROM repos WHERE pushed_at IS NOT NULL",
        [],
        |r| r.get(0),
    )?;
    Ok((total, archived, with_push))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo(name: &str) -> Repo {
        Repo {
            full_name: name.into(),
            description: Some("fast rust grep".into()),
            language: Some("Rust".into()),
            stargazers_count: 10,
            forks_count: 1,
            pushed_at: Some("2026-09-01T00:00:00Z".into()),
            archived: false,
            html_url: format!("https://github.com/{name}"),
            topics: vec!["grep".into()],
            license: None,
        }
    }

    #[test]
    fn upsert_is_idempotent_and_counts_match() {
        let db = open(":memory:").unwrap();
        let repos = vec![repo("a/b"), repo("c/d")];
        assert_eq!(upsert(&db, &repos).unwrap(), 2);
        assert_eq!(upsert(&db, &repos).unwrap(), 2); // re-sync, no dupes
        assert_eq!(counts(&db).unwrap(), (2, 0, 2));
    }
}
