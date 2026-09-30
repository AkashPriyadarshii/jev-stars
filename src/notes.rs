use anyhow::Result;
use rusqlite::Connection;

/// One table, not two. `why` = why you saved it. Temporal queries reuse
/// existing pushed_at/health. repo_events waits until notes prove insufficient.
pub fn ensure_table(db: &Connection) -> Result<()> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS notes(repo TEXT PRIMARY KEY, why TEXT NOT NULL, ts TEXT DEFAULT (datetime('now')));",
    )?;
    Ok(())
}

pub fn add(db: &Connection, repo: &str, why: &str) -> Result<()> {
    ensure_table(db)?;
    let n: i64 = db.query_row("SELECT COUNT(*) FROM repos WHERE full_name=?1", [repo], |r| {
        r.get(0)
    })?;
    if n == 0 {
        anyhow::bail!("unknown repo '{repo}' (not in stars)");
    }
    db.execute(
        "INSERT INTO notes(repo,why,ts) VALUES(?,?,datetime('now'))
         ON CONFLICT(repo) DO UPDATE SET why=excluded.why, ts=datetime('now')",
        rusqlite::params![repo, why],
    )?;
    Ok(())
}

pub fn get(db: &Connection, repo: &str) -> Result<Option<(String, String)>> {
    ensure_table(db)?;
    Ok(db
        .query_row("SELECT why, ts FROM notes WHERE repo=?1", [repo], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .ok())
}

pub fn list(db: &Connection) -> Result<Vec<(String, String, String)>> {
    ensure_table(db)?;
    let mut s = db.prepare("SELECT repo, why, ts FROM notes ORDER BY ts DESC")?;
    let rows = s.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    Ok(rows.collect::<std::result::Result<Vec<(String, String, String)>, _>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_get_roundtrip_rejects_unknown() {
        let db = crate::store::open(":memory:").unwrap();
        crate::store::upsert(
            &db,
            &[crate::store::Repo {
                full_name: "a/b".into(),
                description: None,
                language: None,
                stargazers_count: 1,
                forks_count: 0,
                pushed_at: None,
                archived: false,
                html_url: "https://github.com/a/b".into(),
                topics: vec![],
                license: None,
            }],
        )
        .unwrap();
        assert!(add(&db, "zzz/nope", "why").is_err());
        add(&db, "a/b", "for v0.2 RRF test").unwrap();
        let (why, _) = get(&db, "a/b").unwrap().unwrap();
        assert_eq!(why, "for v0.2 RRF test");
        assert_eq!(list(&db).unwrap().len(), 1);
    }
}
