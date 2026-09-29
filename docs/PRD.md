# jev-stars PRD

## Problem

Developers star 1,000+ repos and lose them. GitHub's stars page is a
paginated list with weak search. Coding agents recommending libraries
guess from stale training data: they suggest archived repos, wrong
licenses, dead forks. 1,243 stars (verified 2026-09-29 via `gh api`)
sit unused because no tool turns them into agent-queryable memory.

## Product

One static Rust binary. Stars in, agent context out. Local SQLite is
the truth. Jev adds judgment once, cached by content hash.

## Users

- Akash daily-driving pi with 1,243 stars.
- FOSS devs with 500+ stars, 8GB-class machines, no budget for dashboards.
- Coding agents (pi, Claude Code, Cursor, Codex) needing bounded, trustworthy repo context over HTTP-free stdio.

## v0.1 scope

- `sync`: `gh api` paginated pull + ETag, upsert 1,243 rows, preserves ledger.
- `search`: FTS5 + `--lang/--topic/--alive`, ranked, <100ms.
- `context`: bounded JSON (repo, url, starred_at, last_push, archived, license, language, topics, maintenance, Jev choice/score/tags, why_matched, readme excerpt).
- `status`: offline dead/alive/license rollup.
- `curate` (optional, needs `$TYPESAFE_API_KEY`): Jev Choice + Score batch, `(repo, readme_hash, schema_v, model)` gate. Rerun = zero calls on unchanged corpus. Without a key the command exits 0 with `curated: 0, skipped: no-key`, and `context` still serves health + FTS.
- `export`: `STARS.md` grouped by Jev tags.
- `mcp`: stdio server, 5 tools (`search, context, status, sync, export`).
- `cargo test` green. `cargo clippy -D warnings` clean.

## Non-goals (v0.1)

- Vectors, sqlite-vec, LanceDB. v0.2 when FTS falls short.
- TUI, dashboard, web UI. My-Starred-Repos covers humans.
- Release tracking, notifications, browser ext.
- star/unstar mutation. Read-only memory.
- Regex search.

## Roadmap

- v0.1 ships the six commands + MCP. Tag it, lock CLI shape.
- v0.2 retrieval: sqlite-vec hybrid (FTS5 + vectors + RRF), `similar` tool.
- v0.3 memory: notes (`why I starred this`), temporal queries ("starred 6mo ago, still alive?").
- v1.0 stable: frozen CLI + MCP contract, release matrix, man page.

## Success criteria

- Correct: sync count == GitHub starred count (1,243 on 2026-09-29).
- Fast: `search` p50 <100ms on 8GB i3. Rerun `curate` = 0 Jev calls.
- Small: one binary, SQLite file ~6MB, no daemon.
- Agent check: pi `context "rust mcp"` returns alive-first ranked rows with evidence, with and without Jev rows present.
