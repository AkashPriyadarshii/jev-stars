use anyhow::Result;
use rusqlite::Connection;
use std::fmt::Write as _;

/// Offline awesome-list export grouped by language. Jev tags upgrade grouping in v0.2.
pub fn export(db: &Connection, path: &str) -> Result<usize> {
    let mut stmt = db.prepare(
        "SELECT language, full_name, stars, description, url, archived
         FROM repos ORDER BY language, stars DESC",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, Option<String>>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, i64>(2)?,
            r.get::<_, Option<String>>(3)?,
            r.get::<_, String>(4)?,
            r.get::<_, i64>(5)?,
        ))
    })?;
    let mut out = String::from("# Stars\n\nCurated from my GitHub stars via `jev-stars`.\n");
    let mut count = 0;
    let mut last_lang = String::new();
    for row in rows {
        let (lang, name, stars, desc, url, archived) = row?;
        if archived == 1 {
            continue; // ponytail: dead repos rot an awesome-list; skip.
        }
        let lang = lang.unwrap_or_else(|| "Other".into());
        if lang != last_lang {
            write!(out, "\n## {lang}\n\n").unwrap();
            last_lang = lang.clone();
        }
        let desc = desc.unwrap_or_default();
        writeln!(out, "- [{name}]({url}) ({stars}) - {desc}").unwrap();
        count += 1;
    }
    std::fs::write(path, out)?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store;

    #[test]
    fn export_skips_archived_and_groups() {
        let db = store::open(":memory:").unwrap();
        store::upsert(
            &db,
            &[
                store::Repo {
                    full_name: "a/live".into(),
                    description: Some("live tool".into()),
                    language: Some("Rust".into()),
                    stargazers_count: 9,
                    forks_count: 0,
                    pushed_at: None,
                    archived: false,
                    html_url: "https://github.com/a/live".into(),
                    topics: vec![],
                    license: None,
                },
                store::Repo {
                    full_name: "a/dead".into(),
                    description: Some("dead tool".into()),
                    language: Some("Rust".into()),
                    stargazers_count: 99,
                    forks_count: 0,
                    pushed_at: None,
                    archived: true,
                    html_url: "https://github.com/a/dead".into(),
                    topics: vec![],
                    license: None,
                },
            ],
        )
        .unwrap();
        let n = export(&db, "/tmp/jev-stars-test.md").unwrap();
        assert_eq!(n, 1);
        let text = std::fs::read_to_string("/tmp/jev-stars-test.md").unwrap();
        assert!(text.contains("## Rust"));
        assert!(!text.contains("a/dead"));
    }
}
