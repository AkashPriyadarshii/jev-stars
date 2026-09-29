use anyhow::{Context, Result};
use std::process::Command;

use crate::store::Repo;

/// Pull all starred repos via `gh api` (auth + pagination handled by gh).
/// Returns raw repo objects; store layer upserts.
pub fn fetch_stars() -> Result<Vec<Repo>> {
    let out = Command::new("gh")
        .args(["api", "user/starred", "--paginate", "--jq", "."])
        .output()
        .context("run `gh api user/starred --paginate` (need gh authed)")?;
    if !out.status.success() {
        anyhow::bail!("gh failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    // --paginate concatenates one JSON array per page; merge them.
    let text = String::from_utf8_lossy(&out.stdout);
    let mut repos: Vec<Repo> = Vec::new();
    let mut depth = 0;
    let mut start = None;
    for (i, c) in text.char_indices() {
        match c {
            '[' => {
                if depth == 0 {
                    start = Some(i);
                }
                depth += 1;
            }
            ']' => {
                depth -= 1;
                if depth == 0 {
                    let chunk = &text[start.unwrap()..=i];
                    let mut page: Vec<Repo> =
                        serde_json::from_str(chunk).context("parse gh page JSON")?;
                    repos.append(&mut page);
                }
            }
            _ => {}
        }
    }
    Ok(repos)
}
