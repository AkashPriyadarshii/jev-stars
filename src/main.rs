mod curate;
mod export;
mod mcp;
mod notes;
mod query;
mod scoring;
mod store;
mod sync;
mod vector;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jev-stars", version)]
struct Cli {
    #[arg(long, default_value = "jev-stars.db")]
    db: String,
    #[arg(long)]
    json: bool,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Pull stars via `gh api`, upsert SQLite (net)
    Sync,
    /// FTS search, ranked table (offline)
    Search {
        query: String,
        #[arg(long)]
        lang: Option<String>,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        alive: bool,
        #[arg(long, default_value = "10")]
        limit: i64,
    },
    /// Bounded agent JSON: health + Jev(null in v0.1) + evidence (offline)
    Context {
        query: String,
        #[arg(long, default_value = "10")]
        limit: i64,
    },
    /// Dead/alive/license rollup (offline)
    Status,
    /// Jev Choice + Score batch, cache-first (net, optional, needs key)
    Curate {
        #[arg(long, default_value = "50")]
        limit: i64,
    },
    /// STARS.md awesome-list grouped by language (offline)
    Export {
        #[arg(default_value = "STARS.md")]
        path: String,
    },
    /// stdio MCP server: search, context, status, sync, export
    Mcp,
    /// Vector KNN over stars. Needs `embed` first. Offline after index.
    Similar {
        query: String,
        #[arg(long, default_value = "10")]
        limit: i64,
        #[arg(long)]
        hash: bool,
    },
    /// Build vector index for all repos (net on first run to fetch model)
    Embed {
        #[arg(long, default_value = "200")]
        limit: i64,
        /// Offline token-hash embeddings instead of BGE
        #[arg(long)]
        hash: bool,
    },
    /// Why you saved it. One table. Shows in context/MCP.
    Note {
        repo: Option<String>,
        why: Option<String>,
        #[arg(long)]
        list: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let db = store::open(&cli.db)?;
    match cli.cmd {
        Cmd::Sync => {
            let repos = sync::fetch_stars()?;
            let n = store::upsert(&db, &repos)?;
            println!("synced {n} repos");
        }
        Cmd::Search {
            query,
            lang,
            topic,
            alive,
            limit,
        } => {
            let hits = query::search(&db, &query, lang.as_deref(), topic.as_deref(), alive, limit)?;
            if hits.is_empty() {
                std::process::exit(1);
            }
            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&query::context(&db, &query, limit)?)?
                );
            } else {
                for h in &hits {
                    println!(
                        "{}  {}  {}  {}  {}",
                        h.full_name,
                        h.stars,
                        h.language.as_deref().unwrap_or("-"),
                        h.license.as_deref().unwrap_or("-"),
                        if h.archived { "archived" } else { "alive" },
                    );
                }
            }
        }
        Cmd::Context { query, limit } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&query::context(&db, &query, limit)?)?
            );
        }
        Cmd::Status => {
            let (total, archived, with_push) = store::counts(&db)?;
            println!("total={total} archived={archived} with_push={with_push}");
        }
        Cmd::Curate { limit } => {
            curate::curate(&db, limit)?;
        }
        Cmd::Export { path } => {
            let n = export::export(&db, &path)?;
            println!("exported {n} repos to {path}");
        }
        Cmd::Mcp => {
            mcp::run(&db)?;
        }
        Cmd::Similar { query, limit, hash } => {
            let hits = query::hybrid(&db, &query, limit, hash)?;
            if hits.is_empty() {
                std::process::exit(1);
            }
            if cli.json {
                let repos: Vec<String> =
                    hits.iter().map(|h| h.full_name.clone()).collect();
                let jev = store::decisions_for(&db, &repos)?;
                let out: Vec<serde_json::Value> = hits
                    .iter()
                    .map(|h| {
                        serde_json::json!({
                            "repo": h.full_name,
                            "rrf": h.rrf,
                            "stars": h.hit.stars,
                            "language": h.hit.language,
                            "jev_choice": jev.get(&h.full_name).and_then(|v| v.0.clone()),
                            "jev_score": jev.get(&h.full_name).and_then(|v| v.1),
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&out)?);
            } else {
                for h in &hits {
                    println!("{:50}  {:.4}", h.full_name, h.rrf);
                }
            }
        }
        Cmd::Note { repo, why, list } => {
            if list {
                for (r, w, ts) in notes::list(&db)? {
                    println!("{r}\n  [{ts}] {w}");
                }
            } else {
                let repo = repo.unwrap_or_else(|| std::process::exit(2));
                match why {
                    Some(w) => {
                        notes::add(&db, &repo, &w)?;
                        println!("noted {repo}");
                    }
                    None => match notes::get(&db, &repo)? {
                        Some((w, ts)) => println!("[{ts}] {w}"),
                        None => std::process::exit(1),
                    },
                }
            }
        }
        Cmd::Embed { limit, hash } => {
            let (n, total) = vector::index_repos(&db, limit, hash)?;
            println!(
                "embedded {n} of {total} pending (indexed now: {})",
                vector::indexed_count(&db, if hash { "hash384" } else { crate::vector::MODEL_ID })?
            );
        }
    }
    Ok(())
}
