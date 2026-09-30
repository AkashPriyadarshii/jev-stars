---
title: "jev-stars: Local-First GitHub Stars Memory for AI Coding Agents (Rust CLI + MCP)"
description: "Open-source Rust CLI turning 1244 GitHub stars into agent-ready context. Offline-first SQLite + FTS5, deterministic repo health, cached Jev AI tags, built-in MCP server for Claude Code, Cursor, pi. MIT."
canonical: "https://github.com/AkashPriyadarshii/jev-stars"
image: "https://github.com/AkashPriyadarshii/jev-stars/raw/main/assets/jev-stars.svg"
author: "Akash Priyadarshi"
license: "MIT"
language: "en"
topic: "developer-tools"
tags:
  - github-stars-manager
  - github-stars-organizer
  - mcp-server-rust
  - ai-coding-agent-memory
  - claude-code-mcp
  - sqlite-fts5
  - jev-ai
  - starred-repos-cli
  - offline-first-cli
  - rust-cli
  - model-context-protocol
  - developer-tools
keywords:
  - github stars manager
  - github stars organizer
  - mcp server rust
  - ai coding agent memory
  - claude code mcp
  - sqlite fts5
  - starred repos cli
  - offline first cli
---

<!--
Title: jev-stars - Local-First GitHub Stars Memory for AI Coding Agents (Rust CLI + MCP)
Description: Open-source Rust CLI turning 1244 GitHub stars into agent-ready context. Offline-first SQLite + FTS5, deterministic repo health, cached Jev AI tags, built-in MCP server for Claude Code, Cursor, pi. MIT.
Keywords: github stars manager, github stars organizer, mcp server rust, ai coding agent memory, claude code mcp, sqlite fts5, jev ai, starred repos cli, offline-first cli, rust cli
-->

**Support:** fuel the next build: [![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/AkashPriyadarshi)

<div align="center">
  <img src="assets/jev-stars.svg" alt="jev-stars logo: gold star over terminal prompt" width="96">
  <h1>jev-stars</h1>
  <p><strong>Your 1,244 GitHub stars, turned into agent-ready memory. One Rust binary. Offline-first.</strong></p>
  <p>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-0055ff.svg?style=flat-square" alt="MIT License" /></a>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.70+-ce422b.svg?style=flat-square&logo=rust" alt="Rust 1.70+" /></a>
    <a href="#how-far-to-trust-it"><img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-2a3138.svg?style=flat-square" alt="Windows, Linux, macOS" /></a>
  </p>
  <p>By <strong>Akash Priyadarshi</strong> · MIT · Rust · zero runtime services</p>
  <p>
    <a href="#why-it-earns-a-slot">Why</a> ·
    <a href="#quickstart">Quickstart</a> ·
    <a href="#command-reference">Commands</a> ·
    <a href="#how-far-to-trust-it">Trust</a> ·
    <a href="#mcp">MCP</a> ·
    <a href="#architecture">Layout</a> ·
    <a href="#limits-and-non-goals">Limits</a> ·
    <a href="#ecosystem">Ecosystem</a>
  </p>
</div>

[![stars](https://img.shields.io/github/stars/AkashPriyadarshii/jev-stars?style=flat-square&label=stars)](https://github.com/AkashPriyadarshii/jev-stars/stargazers) [![crates.io](https://img.shields.io/crates/v/jev-stars?style=flat-square)](https://crates.io/crates/jev-stars) [![downloads](https://img.shields.io/crates/d/jev-stars?style=flat-square)](https://crates.io/crates/jev-stars) [![release](https://img.shields.io/github/v/release/AkashPriyadarshii/jev-stars?style=flat-square&label=release)](https://github.com/AkashPriyadarshii/jev-stars/releases)

---

## Direct answer

**What is jev-stars?** A free, open-source Rust CLI for developers who starred
too much. You sync once. It stores every star in local SQLite with full-text
search, deterministic health (alive, archived, license), and cached Jev AI tags.
Your coding agent queries it and gets facts plus judgment in one bounded object.

**Who is it for?** Developers with 500+ stars whose coding agent (pi, Claude Code,
Cursor, Codex) keeps recommending dead or wrong repos. Solo FOSS users who want
local-first, no account, no telemetry.

**What does it cost?** Free. MIT. Search, rank, and export work with no API key.
Jev tagging needs `$TYPESAFE_API_KEY` once; results cache forever keyed by content hash.
No key: every command except `curate` works at full power.

```bash
# bash/zsh (~/.bashrc or ~/.zshrc)
alias jst='jev-stars'
```

```powershell
# PowerShell ($PROFILE)
Set-Alias jst jev-stars
```

```cmd
:: cmd (doskey macro)
doskey jst=jev-stars $*
```

```console
$ jst similar "video clip cutter" --limit 1
zhouxiaoka/autoclip  0.0476
$ jst note memvid/memvid "agentic long-term memory API, reference build"
noted memvid/memvid
```

---

## Why it earns a slot

Dashboards organize stars for humans. Agents need memory with opinions.

| What you get | Why it matters |
|---|---|
| One binary, local SQLite | `cargo install jev-stars`. No server, no account, works on flight wifi after sync |
| `context`, not `search` | Returns health + Jev score + evidence excerpt, ~2k tokens. Agent reasons, never guesses |
| Deterministic health | Archived, last push, license computed locally. No LLM in the scoring path |
| Cached Jev ledger | Same content hash + schema = zero re-inference. 1,244 repos cost once, not every run |
| Built-in MCP stdio | Same functions as CLI. pi connects in one config block |

---

## Quickstart

```bash
cargo install jev-stars
jev-stars sync
jev-stars search "rust mcp"
```

With AI tags (one-time enrichment, then cached):

```bash
export TYPESAFE_API_KEY=...
jev-stars curate
jev-stars context "rust mcp server" --json
```

---

## Command Reference

| Command | Effect |
|-------|--------|
| `sync` | Pull stars via `gh api`, upsert SQLite (net) |
| `search <q>` | FTS + `--lang`/`--topic`/`--alive`, ranked table (offline) |
| `similar <q>` | Hybrid FTS + vector RRF (`--hash` for zero-download fallback) |
| `embed` | Build BGE-small-en-v1.5 index, batched + resumable (net once for model) |
| `context <q>` | Bounded agent JSON: health + Jev + note + evidence (offline) |
| `note <repo> [why]` | Save or read why you starred it. `--list` dumps all |
| `status` | Dead/alive/license rollup (offline) |
| `curate` | Jev Choice + Score batch, cache-first (net, optional) |
| `export` | `STARS.md` grouped by tags (offline) |
| `mcp` | stdio MCP server: `search, similar, context, note, status, sync, export` |

Global: `--json`, `--limit N` (default 10, max 50). Exit codes: 0 ok, 1 no match, 2 error.

---

## MCP

```json
{
  "mcpServers": {
    "jev-stars": { "command": "jev-stars", "args": ["mcp"] }
  }
}
```

7 tools: `search, similar, context, note, status, sync, export`. Start with `context`.

---

## How far to trust it

Every number below is measured on this machine (i3-1115G4, 8GB, Windows 11), never a claim.

| Check | Result | Rerun |
|---|---|---|
| Sync | 1,244 repos, 8 archived | `jst sync` + `jst status` |
| Search latency | ~16-20ms p50 on 1,244 rows | `jst search "rust" --limit 5` |
| Hybrid `similar` | ~330ms p50 (BGE query-encode dominates, scan is microseconds) | `jst similar "rust mcp"` |
| Hybrid `context` | ~345ms p50 | `jst context "rust mcp"` |
| Vectors | 1,244/1,244 BGE-small-en-v1.5 384D, DB 4.03MB total | `jst embed --limit 10` |
| Test suite | `cargo test` green, 8 tests | per-file `cargo test` |
| Clippy | `cargo clippy --all-targets --locked -- -D warnings` clean | same |
| Curate ledger | rerun on done rows = zero Jev calls | `jst curate --limit 50` twice |
| Jev confidence | 0.72-0.79 with candidates in state, 0.37-0.46 without | see HANDOFF |

Jev scores arrive 0-3 from the API and read normalized 0-1 in `context`.
Confidence below 0.5 means you verify the top 2 by hand.

## Architecture

- **Offline-first.** Net only in `sync` (GitHub) and `curate` (Jev). Queries never touch net.
- **Ledger, not calls.** `(repo, readme_hash, schema_v, model)` gates every inference.
- **Health without LLM.** `scoring.rs` computes alive/archived/push-age/license in one file.

```
src/
  main.rs     - CLI wiring, exit codes
  sync.rs     - gh api pull, pagination, ETag
  store.rs    - schema, FTS5, ledger CRUD
  scoring.rs  - deterministic health (no LLM)
  curate.rs   - Jev batch, hash-gated
  query.rs    - FTS + vector RRF fuse + context pack
  vector.rs   - BGE-small-en-v1.5 embed, hash384 fallback, exact-scan KNN
  notes.rs    - why-you-starred-it, one table
  export.rs   - STARS.md writer
  mcp.rs      - hand-rolled stdio server (no rmcp dep, same fns as CLI)
```

---

## Development

```bash
cargo test
cargo clippy --all-targets --locked -- -D warnings
```

Tests live beside the code they cover. Non-trivial logic ships with one runnable check. Keep diffs small: stdlib and installed crates before new deps.

---

## Limits and non-goals

- No ANN index. 1,244 x 384 exact scan runs in microseconds; sqlite-vec adds risk for zero gain at this size. Revisit past 50k rows.
- No quantized vectors. int8 saves ~1.4MB and risks quality. Revisit when DB size hurts.
- No TUI, no dashboard, no release tracker, no browser ext. My-Starred-Repos already serves humans.
- No regex search. Literal + FTS + vectors only.
- Not a GitHub client. No star/unstar. Read-only memory over your stars.

---

## Ecosystem

- [jev-seo](https://github.com/AkashPriyadarshii/jev-seo)
- [jev-superpowers](https://github.com/AkashPriyadarshii/jev-superpowers)
- [jev-curate](https://github.com/AkashPriyadarshii/jev-curate)
- [jev-git](https://github.com/AkashPriyadarshii/jev-git)
- [tdlib-android](https://github.com/AkashPriyadarshii/tdlib-android)
- [kharcha](https://github.com/AkashPriyadarshii/kharcha)

---

## Author

MIT. Built by Akash Priyadarshi (Patna, Bihar, India).

- GitHub: [AkashPriyadarshii](https://github.com/AkashPriyadarshii)
- Portfolio: [akashpriyadarshi.vercel.app](https://akashpriyadarshi.vercel.app)
- LinkedIn: [akash-priyadarshi-1aa51b37a](https://linkedin.com/in/akash-priyadarshi-1aa51b37a)
- Resume: [akashpriyadarshii.github.io/Resume](https://akashpriyadarshii.github.io/Resume/)

Social: [X/Twitter](https://x.com/Akash__ydv001) · [Threads](https://www.threads.net/@akash.priyadarshii) · [Instagram](https://www.instagram.com/akash.priyadarshii/) · [Reddit](https://reddit.com/user/akashpriyadarshi)

---

## Contributors

See [CONTRIBUTING.md](CONTRIBUTING.md). PRs welcome. Keep it boring: smallest diff that holds, stdlib and installed crates before new deps, one runnable check for non-trivial logic. Run `cargo test` and `cargo clippy -- -D warnings` before you push.

---

*One binary. Zero services. Your stars remember why.*
