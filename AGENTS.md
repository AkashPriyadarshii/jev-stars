# jev-stars

## What is this?

Local-first Rust CLI turning 1,243 GitHub stars into agent-ready memory.
You star repos, you forget why. pi asks `context`, gets facts + judgment
in ~2k tokens instead of 50 README dumps.

Offline-first. One static binary. SQLite + FTS5. Windows-first;
builds for Linux/macOS.

Jev enriches, never blocks: no key still gives search + rank + context + status + export. `curate` is the only key-gated command.

## Build

```bash
cargo build --release
```

Binary lands at `target/release/jev-stars` (`jev-stars.exe` on Windows).

## Test

```bash
cargo test
cargo clippy --all-targets --locked -- -D warnings
```

Tests live beside the code they cover. Non-trivial logic ships
with one runnable check.

## Commands

| Command | What it does |
|---------|--------------|
| `sync` | Pull stars via `gh api`, paginated + ETag, upsert SQLite |
| `search <q>` | FTS + `--lang`/`--topic`/`--alive` filters, ranked table |
| `context <q>` | Bounded agent JSON: repo + health + Jev + evidence |
| `status` | Dead/alive/license rollup, fully offline |
| `curate` | Jev Choice + Score batch, cache-first, needs net, optional (skipped without key) |
| `export` | `STARS.md` awesome-list grouped by Jev tags, offline |
| `mcp` | stdio MCP server for pi, same fns as CLI |

Global flags: `--json`, `--limit N` (default 10, max 50).

Short alias is `jst`, never `js` (JavaScript collision). Docs-only shell
alias; binary stays `jev-stars`. No code change: clap is argv[0]-agnostic.

## Search architecture (read before editing)

- Single SQLite file: `repos` table + FTS5 index + `decisions` ledger.
- Ledger key: `(repo, readme_hash, schema_v, model)`. Hit = skip inference.
- Deterministic health in `scoring.rs`, one file, no LLM in path:
  `alive, archived, last_push_days, license, language, topics`.
- `context` merges FTS rank + health + cached Jev into one bounded
  object with `why_matched` + README excerpt. Token-capped by construction.
  Jev fields are null when never curated; the object still answers.
- Net only in `sync` (GitHub) and `curate` (Jev). Query paths never touch net.
- MCP layer sells `context`, not `search`. 5 tools max:
  `search, context, status, sync, export`. `similar`/vectors wait for v0.2.

## Exit codes and errors

- 0: ok. 1: no match. 2: usage or I/O error.
- Errors: `jev-stars: reason` on stderr. Sync failures never wipe cache.

## Project structure

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
build         - cargo, no build script in v0.1
docs/         - PRD, ARCHITECTURE, DESIGN, HANDOFF
memory/       - session notes
```
