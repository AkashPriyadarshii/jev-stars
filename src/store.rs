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
            full_name UNINDEXED, description, language, topics, tokenize='porter'
        );
        CREATE TABLE IF NOT EXISTS decisions(
            repo TEXT, content_hash TEXT, schema_v INTEGER, model TEXT,
            choice TEXT, score REAL, confidence REAL, tags TEXT, evidence TEXT, ts TEXT,
            PRIMARY KEY(repo, content_hash, schema_v, model)
        );
        CREATE INDEX IF NOT EXISTS idx_decisions_repo ON decisions(repo);",
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
        "INSERT INTO repos_fts(full_name,description,language,topics)
         SELECT full_name,description,language,topics FROM repos
         WHERE full_name NOT IN (SELECT full_name FROM repos_fts);",
    )?;
    tx.commit()?;
    Ok(repos.len())
}

/// Search hit: stored fields only, no Jev join yet (chunk 3).
#[derive(Debug)]
pub struct Hit {
    pub full_name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: i64,
    pub url: String,
    pub topics: String,
    pub license: Option<String>,
    pub archived: bool,
    pub pushed_at: Option<String>,
}

fn row_to_hit(r: &rusqlite::Row) -> rusqlite::Result<Hit> {
    Ok(Hit {
        full_name: r.get(0)?,
        description: r.get(1)?,
        language: r.get(2)?,
        stars: r.get(3)?,
        url: r.get(4)?,
        topics: r.get::<_, Option<String>>(5)?.unwrap_or_default(),
        license: r.get(6)?,
        archived: r.get::<_, i64>(7)? == 1,
        pushed_at: r.get(8)?,
    })
}

const COLS: &str = "repos.full_name,repos.description,repos.language,repos.stars,repos.url,repos.topics,repos.license,repos.archived,repos.pushed_at";

pub fn search(
    db: &Connection,
    q: &str,
    lang: Option<&str>,
    topic: Option<&str>,
    alive_only: bool,
    limit: i64,
) -> Result<Vec<Hit>> {
    // ponytail: FTS5 MATCH on the raw query; quote each token to avoid syntax errors.
    let fts_q: String = q
        .split_whitespace()
        .map(|t| format!("\"{}\"", t.replace('"', "")))
        .collect::<Vec<_>>()
        .join(" ");
    let mut sql = format!(
        "SELECT {COLS} FROM repos JOIN repos_fts ON repos.full_name = repos_fts.full_name
         WHERE repos_fts MATCH ?1"
    );
    if lang.is_some() {
        sql.push_str(" AND repos.language = ?2");
    }
    if topic.is_some() {
        sql.push_str(" AND repos.topics LIKE '%' || ?3 || '%'");
    }
    if alive_only {
        sql.push_str(" AND repos.archived = 0");
    }
    sql.push_str(" ORDER BY rank, stars DESC LIMIT ?4");
    let mut stmt = db.prepare(&sql)?;
    let rows = stmt.query_map(
        rusqlite::params![fts_q, lang, topic, limit],
        row_to_hit,
    )?;
    rows.collect::<std::result::Result<Vec<_>, _>>().map_err(anyhow::Error::from)
}

/// Latest Jev decision per repo: (choice, score, confidence). Missing = never curated.
/// Scores arrive 0-3 from the API; normalized to 0-1 here so agents compare sanely.
pub type Decision = (Option<String>, Option<f64>, Option<f64>);

pub fn decisions_for(
    db: &Connection,
    repos: &[String],
) -> Result<std::collections::HashMap<String, Decision>> {
    // ponytail: one IN query, not N round-trips.
    if repos.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    let placeholders = repos.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT repo, choice, score / 3.0, confidence FROM decisions WHERE repo IN ({placeholders})"
    );
    let mut stmt = db.prepare(&sql)?;
    let rows = stmt.query_map(
        rusqlite::params_from_iter(repos.iter()),
        |r| Ok((r.get::<_, String>(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
    )?;
    let mut map = std::collections::HashMap::new();
    for row in rows {
        let (repo, choice, score, conf): (String, Option<String>, Option<f64>, Option<f64>) = row?;
        map.insert(repo, (choice, score, conf));
    }
    Ok(map)
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
