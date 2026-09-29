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
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Pull stars via `gh api`, upsert SQLite (net)
    Sync,
    /// Dead/alive/license rollup (offline)
    Status,
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
        Cmd::Status => {
            let (total, archived, with_push) = store::counts(&db)?;
            println!("total={total} archived={archived} with_push={with_push}");
        }
    }
    Ok(())
}
