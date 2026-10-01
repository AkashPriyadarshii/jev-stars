# jev-stars PRD

## Problem

Developers star 1,000+ repos and lose them. GitHub's stars page is a
paginated list with weak search. Coding agents recommending libraries
guess from stale training data: they suggest archived repos, wrong
licenses, dead forks. 1,244 stars (verified 2026-09-29 via `gh api`)
sit unused because no tool turns them into agent-queryable memory.

## Product

One static Rust binary. Stars in, agent context out. Local SQLite is
the truth. Jev adds judgment once, cached by content hash.

## Users

Two POVs, one binary.

**Akash (pattern mining).** 1,244 stars = vetted working systems across
Python/TS/Rust/Go. Same problem solved 5 ways in 5 langs. Before any
build: `similar` finds by meaning, `context` top 3-5 gives alive/license/
Jev/evidence, clone best, read arch + prod patterns, then design. Agents
apprentice on proven code instead of hallucinating architecture. `note`
saves which repo shaped which decision.

**FOSS (zero-friction memory).** Devs with 500+ stars on weak machines.
No account, no telemetry, works keyless. One binary + one SQLite file.
Backup = copy file. Only `curate` wants a key; everything else offline.

**Agents.** pi, Claude Code, Cursor, Codex needing bounded, trustworthy
repo context over HTTP-free stdio.

## v0.1 scope

- `sync`: `gh api` paginated pull + ETag, upsert 1,244 rows, preserves ledger.
- `search`: FTS5 + `--lang/--topic/--alive`, ranked, <100ms.
- `context`: bounded JSON (repo, url, starred_at, last_push, archived, license, language, topics, maintenance, Jev choice/score/tags, why_matched, readme excerpt).
- `status`: offline dead/alive/license rollup.
- `curate` (optional, needs `$TYPESAFE_API_KEY`): Jev Choice + Score batch, `(repo, readme_hash, schema_v, model)` gate. Rerun = zero calls on unchanged corpus. Without a key the command exits 0 with `curated: 0, skipped: no-key`, and `context` still serves health + FTS.
- `export`: `STARS.md` grouped by Jev tags.
- `mcp`: stdio server, 5 tools (`search, context, status, sync, export`).
- `cargo test` green. `cargo clippy -D warnings` clean.

## v0.2 scope (shipped 2026-09-30)

- `similar`: FTS top-50 + BGE-small-en-v1.5 top-50 fused by RRF k=20. Exact scan, no sqlite-vec.
- `embed`: batched + resumable index build. Model downloads once, then offline.
- `note`: one-table memory `(repo, why, ts)`. Surfaces in `context` + MCP.
- `context` now runs hybrid retrieval and carries `note` per result.
- MCP grows to 7 tools: `search, similar, context, note, status, sync, export`.

## Non-goals (v0.2)

- ANN index (sqlite-vec, HNSW). Exact scan wins at 1,244 rows. Revisit past 50k.
- Quantized vectors. int8 saves ~1.4MB, quality unproven. Revisit when size hurts.
- repo_events table. `notes` + `pushed_at` cover temporal queries until proven short.
- TUI, dashboard, web UI. My-Starred-Repos covers humans.
- Release tracking, notifications, browser ext.
- star/unstar mutation. Read-only memory.
- Regex search.

## Roadmap

- v0.1 shipped the six commands + MCP. Tag it, lock CLI shape.
- v0.2.0 shipped hybrid + notes + 7-tool MCP.
- v0.3 mining: temporal queries on notes + pushed_at ("starred 6mo ago,
still alive?"). `context --mine` dumps top-3 READMEs side-by-side for
agent arch diff.
- v0.4 gaps: `gaps "<need>"` answers from stars, flags what the web must
fill. Jev rerank top-10 on demand.
- v1.0 stable: frozen CLI + MCP contract, release matrix, man page.

## Never (both POVs)

- ANN/HNSW. Exact scan wins to 50k rows.
- int8 quant. Saves ~1.4MB, risks quality.
- Graph memory, sessions, repo_events. Notes + pushed_at suffice.
- TUI, dashboard, cloud sync, release tracker, browser ext.
- star/unstar mutation. Read-only memory.

## Success criteria

- Correct: sync count == GitHub starred count (1,244 on 2026-09-29).
- Fast: `search` p50 ~20ms, `similar` p50 ~330ms, `context` p50 ~345ms (measured 2026-09-30).
- Small: one binary, SQLite file 4.03MB, no daemon.
- Agent check: pi `context "rust mcp"` returns alive-first ranked rows with evidence, with and without Jev rows present.
