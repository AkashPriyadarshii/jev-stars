mod curate;
mod export;
mod mcp;
mod query;
mod scoring;
mod store;
mod sync;

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
                println!("{}", serde_json::to_string_pretty(&query::context(&db, &query, limit)?)?);
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
            println!("{}", serde_json::to_string_pretty(&query::context(&db, &query, limit)?)?);
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
    }
    Ok(())
}
