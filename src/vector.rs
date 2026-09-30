use anyhow::Result;
use fastembed::{EmbeddingModel, TextEmbedding, TextInitOptions};
use rusqlite::Connection;
use std::sync::Mutex;

// ponytail: lazy singleton; model loads once per process, not per query.
static MODEL: Mutex<Option<TextEmbedding>> = Mutex::new(None);
const DIM: usize = 384; // BGESmallENV15
/// Canonical MiniLM-free model id. Old "minilm-l6-v2" rows are stale.
pub const MODEL_ID: &str = "bge-small-en-v1.5";

pub fn embed_one(text: &str) -> Result<Vec<f32>> {
    let mut g = MODEL.lock().unwrap();
    if g.is_none() {
        *g = Some(TextEmbedding::try_new(TextInitOptions::new(
            EmbeddingModel::BGESmallENV15,
        ))?);
    }
    let out = g.as_mut().unwrap().embed(vec![text.to_string()], None)?;
    Ok(out.into_iter().next().unwrap_or_default())
}

/// Offline fallback: 384-dim signed token hash, L2-normalized. No synonymy
/// (so weaker than MiniLM), but keeps `similar` working with zero download.
pub fn hash_embed(text: &str, dim: usize) -> Vec<f32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut v = vec![0f32; dim];
    for tok in text.split_whitespace() {
        let t = tok.trim_matches(|c: char| !c.is_alphanumeric());
        if t.len() < 2 {
            continue;
        }
        let mut h = DefaultHasher::new();
        t.to_lowercase().hash(&mut h);
        let hv = h.finish();
        v[(hv % dim as u64) as usize] += if (hv >> 32) & 1 == 0 { 1.0 } else { -1.0 };
    }
    let n: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if n > 0.0 {
        v.iter_mut().for_each(|x| *x /= n);
    }
    v
}

pub fn ensure_table(db: &Connection) -> Result<()> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS embeddings(repo TEXT PRIMARY KEY, model TEXT, dim INTEGER, vec BLOB);",
    )?;
    Ok(())
}

pub fn store(db: &Connection, repo: &str, model: &str, vec: &[f32]) -> Result<()> {
    let bytes: &[u8] =
        unsafe { std::slice::from_raw_parts(vec.as_ptr() as *const u8, vec.len() * 4) };
    db.execute(
        "INSERT OR REPLACE INTO embeddings(repo,model,dim,vec) VALUES(?,?,?,?)",
        rusqlite::params![repo, model, vec.len() as i64, bytes],
    )?;
    Ok(())
}

pub fn load_all(db: &Connection, model: &str) -> Result<Vec<(String, Vec<f32>)>> {
    ensure_table(db)?;
    let mut s = db.prepare("SELECT repo, vec FROM embeddings WHERE model=?1")?;
    let rows = s.query_map([model], |r| {
        let b: Vec<u8> = r.get(1)?;
        Ok((
            r.get::<_, String>(0)?,
            b.chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect(),
        ))
    })?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn indexed_count(db: &Connection, model: &str) -> Result<i64> {
    ensure_table(db)?;
    Ok(db.query_row(
        "SELECT COUNT(*) FROM embeddings WHERE model=?1",
        [model],
        |r| r.get(0),
    )?)
}

/// Embed every repo missing a vector. Batched, resumable.
pub fn index_repos(db: &Connection, limit: i64, use_hash: bool) -> Result<(usize, usize)> {
    ensure_table(db)?;
    let model_name = if use_hash { "hash384" } else { MODEL_ID };
    let mut s = db.prepare(
        "SELECT full_name, COALESCE(description,''), COALESCE(language,'') FROM repos
         WHERE full_name NOT IN (SELECT repo FROM embeddings WHERE model=?2) LIMIT ?1",
    )?;
    let todo: Vec<(String, String, String)> = s
        .query_map(rusqlite::params![limit, model_name], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?))
        })
        .and_then(|it| it.collect())
        .map_err(|e| anyhow::anyhow!(e))?;
    if todo.is_empty() {
        return Ok((0, 0));
    }
    let mut n = 0;
    for (name, desc, lang) in &todo {
        let vec = if use_hash {
            hash_embed(&format!("{name} {desc} {lang}"), DIM)
        } else {
            embed_one(&format!("{name} {desc} {lang}"))?
        };
        store(db, name, model_name, &vec)?;
        n += 1;
    }
    Ok((n, todo.len()))
}

/// Cosine KNN over stored vectors. Vectors are L2-normalized, so dot = cosine.
pub fn knn(db: &Connection, qvec: &[f32], limit: i64, model: &str) -> Result<Vec<(String, f64)>> {
    let all = load_all(db, model)?;
    let mut scored: Vec<(String, f64)> = all
        .into_iter()
        .map(|(repo, v)| {
            let d = v.iter().zip(qvec.iter()).map(|(a, b)| a * b).sum::<f32>();
            (repo, d as f64)
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(limit as usize);
    Ok(scored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_embed_is_deterministic_and_normalized() {
        let a = hash_embed("rust grep tool", 384);
        assert_eq!(a.len(), 384);
        assert_eq!(hash_embed("rust grep tool", 384), a);
        let n: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((n - 1.0).abs() < 1e-3);
        assert_ne!(hash_embed("python web scraper", 384), a);
    }

    #[test]
    fn knn_ranks_nearest_first() {
        let db = crate::store::open(":memory:").unwrap();
        ensure_table(&db).unwrap();
        store(&db, "a/grep", "hash384", &hash_embed("rust grep tool", 384)).unwrap();
        store(&db, "b/py", "hash384", &hash_embed("python scraper", 384)).unwrap();
        let r = knn(&db, &hash_embed("rust grep tool", 384), 1, "hash384").unwrap();
        assert_eq!(r[0].0, "a/grep");
    }
}
