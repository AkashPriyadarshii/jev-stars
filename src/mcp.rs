use anyhow::Result;
use rusqlite::Connection;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

// ponytail: hand-rolled JSON-RPC stdio, copied from jev-scout's proven mcp.rs.
// No rmcp dep for 5 thin tools over the same fns as CLI.
const PROTOCOL_VERSION: &str = "2025-03-26";
const SUPPORTED: &[&str] = &["2024-11-05", "2025-03-26", "2025-06-18"];

pub fn run(db: &Connection) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let req: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if req.get("id").is_none() {
            continue; // notification, never answer
        }
        let id = req["id"].clone();
        let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
        match method {
            "initialize" => {
                let want = req
                    .pointer("/params/protocolVersion")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let ver = if SUPPORTED.contains(&want) {
                    want
                } else {
                    PROTOCOL_VERSION
                };
                respond(
                    &mut stdout,
                    &json!({
                        "jsonrpc": "2.0", "id": id,
                        "result": {
                            "protocolVersion": ver,
                            "serverInfo": { "name": "jev-stars", "version": "0.1.0" },
                            "capabilities": { "tools": {} },
                        }
                    }),
                )?;
            }
            "tools/list" => respond(
                &mut stdout,
                &json!({
                    "jsonrpc": "2.0", "id": id,
                    "result": { "tools": [
                        tool("search", "Search your GitHub stars (FTS + filters). Offline.", &["query"]),
                        tool("context", "Agent-ready bounded object: health + Jev + evidence. Offline after sync. Start here.", &["query"]),
                        tool("status", "Dead/alive rollup over all stars. Offline.", &[]),
                        tool("sync", "Pull latest stars via gh api. Needs net.", &[]),
                        tool("export", "Write STARS.md awesome-list. Offline.", &[]),
                    tool("similar", "Hybrid FTS+vector RRF over stars by meaning. Needs `embed` first.", &["query"]),
                    tool("note", "Save why you starred a repo, or read it back. Memory layer.", &["repo"]),
                    ]}
                }),
            )?,
            "tools/call" => {
                let name = req
                    .pointer("/params/name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("");
                let args = req
                    .pointer("/params/arguments")
                    .cloned()
                    .unwrap_or(json!({}));
                let out = call(db, name, &args);
                match out {
                    Ok(v) => respond(
                        &mut stdout,
                        &json!({
                            "jsonrpc": "2.0", "id": id,
                            "result": {
                                "isError": false,
                                "content": [{ "type": "text", "text": serde_json::to_string_pretty(&v)? }],
                                "structuredContent": v,
                            }
                        }),
                    )?,
                    Err(e) => respond(
                        &mut stdout,
                        &json!({
                            "jsonrpc": "2.0", "id": id,
                            "error": { "code": -32000, "message": e.to_string() }
                        }),
                    )?,
                }
            }
            _ => respond(
                &mut stdout,
                &json!({
                    "jsonrpc": "2.0", "id": id,
                    "error": { "code": -32601, "message": format!("method '{method}' not found") }
                }),
            )?,
        }
    }
    Ok(())
}

fn tool(name: &str, desc: &str, required: &[&str]) -> Value {
    json!({
        "name": name, "description": desc,
        "inputSchema": {
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Search query, e.g. 'rust mcp server'" },
                "limit": { "type": "integer", "minimum": 1, "maximum": 50, "description": "Max rows (default 10)" },
                "lang": { "type": "string", "description": "Language filter, e.g. 'Rust'" },
                "topic": { "type": "string", "description": "Topic filter, e.g. 'mcp'" },
                "alive": { "type": "boolean", "description": "Only non-archived repos" },
                "path": { "type": "string", "description": "Export path (export only)" },
            },
            "required": required,
        }
    })
}

fn call(db: &Connection, name: &str, args: &Value) -> Result<Value> {
    let limit = args
        .get("limit")
        .and_then(|l| l.as_i64())
        .unwrap_or(10)
        .clamp(1, 50);
    match name {
        "search" => {
            let q = req_str(args, "query")?;
            let hits = crate::query::search(
                db,
                &q,
                args.get("lang").and_then(|v| v.as_str()),
                args.get("topic").and_then(|v| v.as_str()),
                args.get("alive").and_then(|v| v.as_bool()).unwrap_or(false),
                limit,
            )?;
            Ok(json!(hits
                .iter()
                .map(|h| json!({
                    "repo": h.full_name, "stars": h.stars,
                    "language": h.language, "license": h.license,
                    "archived": h.archived, "url": h.url,
                }))
                .collect::<Vec<_>>()))
        }
        "context" => {
            let q = req_str(args, "query")?;
            crate::query::context(db, &q, limit)
        }
        "status" => {
            let (total, archived, with_push) = crate::store::counts(db)?;
            Ok(json!({ "total": total, "archived": archived, "with_push": with_push }))
        }
        "sync" => {
            let repos = crate::sync::fetch_stars()?;
            let n = crate::store::upsert(db, &repos)?;
            Ok(json!({ "synced": n }))
        }
        "export" => {
            let path = args
                .get("path")
                .and_then(|p| p.as_str())
                .unwrap_or("STARS.md");
            let n = crate::export::export(db, path)?;
            Ok(json!({ "exported": n, "path": path }))
        }
        "similar" => {
            let q = req_str(args, "query")?;
            let hits = crate::query::hybrid(db, &q, limit, false)?;
            Ok(json!(hits
                .iter()
                .map(|h| json!({ "repo": h.full_name, "rrf": h.rrf, "stars": h.hit.stars }))
                .collect::<Vec<_>>()))
        }
        "note" => {
            let repo = req_str(args, "repo")?;
            match args.get("why").and_then(|v| v.as_str()) {
                Some(w) => {
                    crate::notes::add(db, &repo, w)?;
                    Ok(json!({ "noted": repo }))
                }
                None => match crate::notes::get(db, &repo)? {
                    Some((w, ts)) => Ok(json!({ "repo": repo, "why": w, "ts": ts })),
                    None => anyhow::bail!("no note for '{repo}'"),
                },
            }
        }
        _ => anyhow::bail!("tool '{name}' not found"),
    }
}

fn req_str(args: &Value, key: &str) -> Result<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("missing required argument: {key}"))
}

fn respond(stdout: &mut io::Stdout, v: &Value) -> io::Result<()> {
    writeln!(stdout, "{}", serde_json::to_string(v)?)?;
    stdout.flush()
}
