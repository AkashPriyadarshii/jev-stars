use anyhow::Result;
use rusqlite::Connection;
use serde_json::json;
use std::time::Duration;

// ponytail: ureq sync POST, same shape as jev-scout. No async runtime for one batch call.
const ENDPOINT: &str = "https://api.typesafe.ai/v1/systemone";
const MODEL: &str = "jev-1.13.0";
const SCHEMA_V: i64 = 1;
const CATEGORIES: &[&str] = &[
    "AI & Agents",
    "Dev Tools",
    "Mobile",
    "Web Automation",
    "LLM & RAG",
    "Web Dev",
    "Security",
    "Media",
    "Databases",
    "DevOps",
    "Other",
];

/// Optional Jev enrichment. No key = exit 0, curated 0, nothing touched.
/// Ledger key (repo, content_hash, schema_v, model): rerun on unchanged
/// corpus = zero HTTP calls.
pub fn curate(db: &Connection, limit: i64) -> Result<(usize, usize)> {
    let key = std::env::var("TYPESAFE_API_KEY").unwrap_or_default();
    if key.is_empty() {
        println!("curated: 0, skipped: no-key (set TYPESAFE_API_KEY to enable)");
        return Ok((0, 0));
    }
    let mut stmt = db.prepare(
        "SELECT full_name, description, language, topics, stars
         FROM repos WHERE full_name NOT IN
           (SELECT repo FROM decisions WHERE schema_v = ?1 AND model = ?2)
         ORDER BY stars DESC LIMIT ?3",
    )?;
    let pending: Vec<(String, Option<String>, Option<String>, String, i64)> = stmt
        .query_map(rusqlite::params![SCHEMA_V, MODEL, limit], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get::<_, Option<String>>(3)?.unwrap_or_default(), r.get(4)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    if pending.is_empty() {
        println!("curated: 0, skipped: up-to-date");
        return Ok((0, 0));
    }
    let mut done = 0;
    // Jev drops questions past ~15/call (jev-scout): 3 repos x 5 questions.
    for chunk in pending.chunks(3) {
        if let Err(e) = curate_chunk(db, &key, chunk) {
            eprintln!("jev-stars: chunk failed: {e}");
            break;
        }
        done += chunk.len();
    }
    println!("curated: {done}, skipped: done-or-failed");
    Ok((done, 0))
}

fn curate_chunk(
    db: &Connection,
    key: &str,
    chunk: &[(String, Option<String>, Option<String>, String, i64)],
) -> Result<()> {
    let mut criteria = serde_json::Map::new();
    let mut questions = serde_json::Map::new();
    for (i, (name, desc, lang, topics, _)) in chunk.iter().enumerate() {
        criteria.insert(
            format!("repo_{i}"),
            json!(format!(
                "{} [{}] {} {}",
                name,
                lang.as_deref().unwrap_or("?"),
                desc.as_deref().unwrap_or(""),
                topics
            )),
        );
        questions.insert(
            format!("cat_{i}"),
            json!({
                "type": "choice",
                "instructions": format!("Best category for '{}'?", name),
                "criteria": CATEGORIES.iter().map(|c| (c.to_string(), json!(c))).collect::<serde_json::Map<String, serde_json::Value>>(),
            }),
        );
        questions.insert(
            format!("fit_{i}"),
            json!({
                "type": "score",
                "instructions": format!("How useful is '{}' as a reusable tool/library?", name),
                "criteria": [
                    "Dead, joke, or list-only repo",
                    "Niche or single-use",
                    "Solid reusable tool",
                    "Essential reference implementation",
                ],
            }),
        );
    }
    let payload = json!({
        "model": MODEL,
        "state": { "count": chunk.len() },
        "questions": questions,
    });
    let resp: serde_json::Value = ureq::post(ENDPOINT)
        .set("Authorization", &format!("Bearer {key}"))
        .set("Content-Type", "application/json")
        .timeout(Duration::from_secs(20))
        .send_json(payload)
        .map_err(|e| anyhow::anyhow!("jev api: {e}"))?
        .into_json()
        .map_err(|e| anyhow::anyhow!("jev parse: {e}"))?;
    let answers = resp
        .get("answers")
        .ok_or_else(|| anyhow::anyhow!("jev response missing answers"))?;
    for (i, (name, desc, _, _, _)) in chunk.iter().enumerate() {
        let cat = answers
            .get(format!("cat_{i}"))
            .and_then(|v| v.get("choice"))
            .and_then(|v| v.as_str())
            .unwrap_or("Other");
        let score = answers
            .get(format!("fit_{i}"))
            .and_then(|v| v.get("score"))
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0);
        let hash = content_hash(name, desc.as_deref());
        db.execute(
            "INSERT OR REPLACE INTO decisions(repo,content_hash,schema_v,model,choice,score,tags,evidence)
             VALUES(?,?,?,?,?,?,?,?)",
            rusqlite::params![name, hash, SCHEMA_V, MODEL, cat, score, cat, desc],
        )?;
    }
    let _ = criteria;
    Ok(())
}

fn content_hash(name: &str, desc: Option<&str>) -> String {
    // ponytail: std hash, not crypto. Cache key only, not security.
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    name.hash(&mut h);
    desc.hash(&mut h);
    format!("{:016x}", h.finish())
}
