<!--
Title: jev-stars - Local-First GitHub Stars Memory for AI Coding Agents (Rust CLI + MCP)
Description: Open-source Rust CLI turning 1243 GitHub stars into agent-ready context. Offline-first SQLite + FTS5, deterministic repo health, cached Jev AI tags, built-in MCP server for Claude Code, Cursor, pi. MIT.
Keywords: github stars manager, github stars organizer, mcp server rust, ai coding agent memory, claude code mcp, sqlite fts5, jev ai, starred repos cli, offline-first cli, rust cli
-->

<div align="center">
  <h1>jev-stars</h1>
  <p><strong>Your 1,243 GitHub stars, turned into agent-ready memory. One Rust binary. Offline-first.</strong></p>
  <p>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-0055ff.svg?style=flat-square" alt="MIT License" /></a>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.70+-ce422b.svg?style=flat-square&logo=rust" alt="Rust 1.70+" /></a>
    <a href="#limits-and-non-goals"><img src="https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-2a3138.svg?style=flat-square" alt="Windows, Linux, macOS" /></a>
  </p>
  <p>By <strong>Akash Priyadarshi</strong> · MIT · Rust · zero runtime services</p>
  <p>
    <a href="#why-it-earns-a-slot">Why</a> ·
    <a href="#quickstart">Quickstart</a> ·
    <a href="#command-reference">Commands</a> ·
    <a href="#mcp">MCP</a> ·
    <a href="#architecture">Layout</a> ·
    <a href="#limits-and-non-goals">Limits</a> ·
    <a href="#ecosystem">Ecosystem</a>
  </p>
</div>

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

```console
$ jev-stars context "rust mcp server" --limit 3
modelcontextprotocol/rust-sdk  alive  MIT  score 0.91  pushed 3d ago
why: FTS(mcp^3,rust^2) + Jev(tags=[mcp,rust-sdk], relevance=0.91)
```

---

## Why it earns a slot

Dashboards organize stars for humans. Agents need memory with opinions.

| What you get | Why it matters |
|---|---|
| One binary, local SQLite | `cargo install jev-stars`. No server, no account, works on flight wifi after sync |
| `context`, not `search` | Returns health + Jev score + evidence excerpt, ~2k tokens. Agent reasons, never guesses |
| Deterministic health | Archived, last push, license computed locally. No LLM in the scoring path |
| Cached Jev ledger | Same content hash + schema = zero re-inference. 1,243 repos cost once, not every run |
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
| `context <q>` | Bounded agent JSON: health + Jev + evidence (offline after curate) |
| `status` | Dead/alive/license rollup (offline) |
| `curate` | Jev Choice + Score batch, cache-first (net, optional) |
| `export` | `STARS.md` grouped by tags (offline) |
| `mcp` | stdio MCP server: `search, context, status, sync, export` |

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

5 tools: `search, context, status, sync, export`. Start with `context`.

---

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
  query.rs    - FTS + rank + context pack
  export.rs   - STARS.md writer
  mcp.rs      - rmcp stdio server
```

---

## Limits and non-goals

- No vectors in v0.1. FTS5 + Jev tags cover 1-2k rows. sqlite-vec lands in v0.2 when FTS measurably falls short.
- No TUI, no dashboard, no release tracker, no browser ext. My-Starred-Repos already serves humans.
- No regex search. Literal + FTS only in v0.1.
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

PRs welcome. Keep it boring: smallest diff that holds, stdlib and installed crates before new deps, one runnable check for non-trivial logic. Run `cargo test` and `cargo clippy -- -D warnings` before you push.

---

*One binary. Zero services. Your stars remember why.*
