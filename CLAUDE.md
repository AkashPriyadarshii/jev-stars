# jev-stars

## What is this?

Local-first Rust CLI turning 1,243 GitHub stars into agent-ready memory.
One binary. SQLite + FTS5. No dashboard, no daemon, no vectors in v0.1.

## Build

```bash
cargo build --release
```

## Test

```bash
cargo test
cargo clippy --all-targets --locked -- -D warnings
```

## Commands

```
jev-stars sync              # pull stars via gh api, upsert SQLite
jev-stars search <q>        # FTS + filters, human table
jev-stars context <q>       # bounded agent object (JSON)
jev-stars status            # dead/alive rollup, offline
jev-stars curate            # Jev Choice+Score, cached, net
jev-stars export            # STARS.md awesome-list, offline
```

Global: `--json` machine output, `--limit N` (default 10, max 50).

## Architecture

- Offline-first: `sync` + `curate` need net. Everything else reads local SQLite.
- `store.rs`: repos table + FTS5 + decision ledger.
- `scoring.rs`: deterministic health, no LLM. One file.
- `curate.rs`: Jev batched, content-hash cached. N changed = N calls.
- `query.rs`: FTS + rank, <100ms on 1,243 rows.
- `mcp.rs`: stdio server, same fns as CLI. 5 tools.
- No key = search + rank + export still work. Jev optional.

## Project structure

```
src/
  main.rs     - CLI wiring, exit codes
  sync.rs     - gh api paginated pull, ETag
  store.rs    - rusqlite schema, FTS5, ledger
  scoring.rs  - alive/archived/last_push_days/license
  curate.rs   - Jev Choice+Score, cache-first
  query.rs    - FTS search, rank, context pack
  export.rs   - STARS.md generation
  mcp.rs      - rmcp stdio, 5 tools
docs/         - PRD, ARCHITECTURE, DESIGN, HANDOFF
memory/       - session notes
```

## Conventions

- Small diffs. Stdlib + installed crates before new deps.
- `anyhow` in binary, no `unwrap` outside tests.
- Tests beside code. `cargo test` green before push.
- Git: HTTPS via gh credential manager. Identity: `272530059+AkashPriyadarshii@users.noreply.github.com`
- MD skeleton before code. No push without "go".
