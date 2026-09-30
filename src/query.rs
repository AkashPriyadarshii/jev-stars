use anyhow::Result;
use rusqlite::Connection;

use crate::scoring;
use crate::store::Hit;

/// RRF-fused hybrid hit: FTS rank + vector rank -> one score.
/// k=20 per Elastic hybrid experiment; depth 50/50, final top-10 default.
pub struct HybridHit {
    pub full_name: String,
    pub rrf: f64,
    pub hit: Hit,
}

pub fn hybrid(
    db: &Connection,
    q: &str,
    limit: i64,
    use_hash: bool,
) -> Result<Vec<HybridHit>> {
    const DEPTH: i64 = 50;
    const K: f64 = 20.0;
    let fts = crate::store::search(db, q, None, None, false, DEPTH).unwrap_or_default();
    let model = if use_hash {
        "hash384"
    } else {
        crate::vector::MODEL_ID
    };
    let vec_hits = if crate::vector::indexed_count(db, model).unwrap_or(0) == 0 {
        Vec::new() // no vectors yet: FTS-only, still answers
    } else {
        let qv = if use_hash {
            crate::vector::hash_embed(q, 384)
        } else {
            crate::vector::embed_one(q)?
        };
        crate::vector::knn(db, &qv, DEPTH, model).unwrap_or_default()
    };
    let vec_rank: std::collections::HashMap<&str, usize> = vec_hits
        .iter()
        .enumerate()
        .map(|(i, (r, _))| (r.as_str(), i))
        .collect();
    let mut fused: Vec<HybridHit> = fts
        .into_iter()
        .enumerate()
        .map(|(fi, h)| {
            let mut s = 1.0 / (K + fi as f64 + 1.0);
            if let Some(&vi) = vec_rank.get(h.full_name.as_str()) {
                s += 1.0 / (K + vi as f64 + 1.0);
            }
            HybridHit {
                full_name: h.full_name.clone(),
                rrf: s,
                hit: h,
            }
        })
        .collect();
    // vector-only repos FTS missed: still candidates, single-list score.
    if !vec_hits.is_empty() {
        let fts_names: std::collections::HashSet<&str> =
            fused.iter().map(|h| h.full_name.as_str()).collect();
        let extra = crate::store::repos_by_name(
            db,
            &vec_hits
                .iter()
                .enumerate()
                .filter(|(_, (r, _))| !fts_names.contains(r.as_str()))
                .map(|(vi, (r, _))| (r.clone(), 1.0 / (K + vi as f64 + 1.0)))
                .collect::<Vec<_>>(),
        )?;
        fused.extend(extra);
    }
    fused.sort_by(|a, b| b.rrf.partial_cmp(&a.rrf).unwrap_or(std::cmp::Ordering::Equal));
    fused.truncate(limit as usize);
    Ok(fused)
}

/// Ranked search over FTS5 + filters. Offline. Jev-agnostic.
pub fn search(
    db: &Connection,
    q: &str,
    lang: Option<&str>,
    topic: Option<&str>,
    alive_only: bool,
    limit: i64,
) -> Result<Vec<Hit>> {
    crate::store::search(db, q, lang, topic, alive_only, limit)
}

/// Bounded agent object. Jev fields fill when curated, else null.
pub fn context(db: &Connection, q: &str, limit: i64) -> Result<serde_json::Value> {
    let fused = hybrid(db, q, limit, false).unwrap_or_default();
    let hits: Vec<Hit> = fused.iter().map(|h| h.hit.clone()).collect();
    let jev = crate::store::decisions_for(
        db,
        &hits.iter().map(|h| h.full_name.clone()).collect::<Vec<_>>(),
    )?;
    let results: Vec<serde_json::Value> = hits
        .iter()
        .map(|h| {
            let days = days_ago(&h.pushed_at);
            serde_json::json!({
                "repo": h.full_name,
                "url": h.url,
                "stars": h.stars,
                "language": h.language,
                "topics": h.topics,
                "license": h.license,
                "archived": h.archived,
                "last_push": h.pushed_at,
                "maintenance": if scoring::alive(h.archived, days) { "active" } else { "stale" },
                "jev_choice": jev.get(&h.full_name).and_then(|d| d.0.clone()),
                "jev_score": jev.get(&h.full_name).and_then(|d| d.1),
                "jev_confidence": jev.get(&h.full_name).and_then(|d| d.2),
                "jev_tags": jev.get(&h.full_name).and_then(|d| d.0.clone()),
                "why_matched": format!("hybrid-RRF({q})"),
                "note": crate::notes::get(db, &h.full_name).ok().flatten().map(|(w, ts)| format!("[{ts}] {w}")),
                "readme_excerpt": excerpt(h.description.as_deref()),
            })
        })
        .collect();
    Ok(serde_json::json!({ "query": q, "count": results.len(), "results": results }))
}

fn days_ago(pushed_at: &Option<String>) -> Option<i64> {
    // pushed_at: "2026-09-01T00:00:00Z". Day-granularity only.
    let s = pushed_at.as_ref()?;
    let date = s.get(..10)?;
    let (y, rest) = date.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    let (y, m, d): (i64, i64, i64) = (y.parse().ok()?, m.parse().ok()?, d.parse().ok()?);
    // days_from_civil (Howard Hinnant). No chrono dep.
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    // ponytail: fixed "today" would rot; compute now via std time.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|t| t.as_secs() as i64 / 86400)
        .unwrap_or(days);
    Some(now - days)
}

fn excerpt(description: Option<&str>) -> Option<String> {
    // ponytail: description stands in for README until chunk 3 stores readmes.
    description.map(|d| {
        let mut s: String = d.chars().take(300).collect();
        if d.chars().count() > 300 {
            s.push('…');
        }
        s
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store;

    fn test_db() -> Connection {
        let db = store::open(":memory:").unwrap();
        store::upsert(
            &db,
            &[
                store::Repo {
                    full_name: "acme/mcp-rs".into(),
                    description: Some("rust mcp server sdk".into()),
                    language: Some("Rust".into()),
                    stargazers_count: 500,
                    forks_count: 10,
                    pushed_at: Some("2026-09-01T00:00:00Z".into()),
                    archived: false,
                    html_url: "https://github.com/acme/mcp-rs".into(),
                    topics: vec!["mcp".into()],
                    license: None,
                },
                store::Repo {
                    full_name: "acme/dead-py".into(),
                    description: Some("old python scraper".into()),
                    language: Some("Python".into()),
                    stargazers_count: 5,
                    forks_count: 0,
                    pushed_at: Some("2020-01-01T00:00:00Z".into()),
                    archived: true,
                    html_url: "https://github.com/acme/dead-py".into(),
                    topics: vec![],
                    license: None,
                },
            ],
        )
        .unwrap();
        db
    }

    #[test]
    fn search_finds_by_description_and_filters_alive() {
        let db = test_db();
        let hits = search(&db, "mcp rust", None, None, false, 10).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].full_name, "acme/mcp-rs");
        let alive_only = search(&db, "python", None, None, true, 10).unwrap();
        assert!(alive_only.is_empty());
    }

    #[test]
    fn context_is_bounded_json_with_null_jev() {
        let db = test_db();
        let ctx = context(&db, "mcp", 10).unwrap();
        assert_eq!(ctx["count"], 1);
        let r = &ctx["results"][0];
        assert_eq!(r["repo"], "acme/mcp-rs");
        assert!(r["jev_score"].is_null());
        assert!(r["why_matched"].as_str().unwrap().contains("mcp"));
    }
}
