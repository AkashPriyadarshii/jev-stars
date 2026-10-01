# jev-stars

## What is this?

Local-first Rust CLI turning 1,244 GitHub stars into agent-ready memory.
One binary. SQLite + FTS5 + BGE vectors. No dashboard, no daemon.

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
jev-stars curate            # Jev Choice+Score, cached, net, optional (needs key)
jev-stars export            # STARS.md awesome-list, offline
jev-stars similar <q>       # hybrid FTS + vector RRF, offline after embed
jev-stars embed              # build BGE index, batched + resumable, net once
jev-stars note <repo> [why] # why you starred it, surfaces in context
```

Global: `--json` machine output, `--limit N` (default 10, max 50).

Alias: `jst` (`js` collides with JavaScript). Shell alias only, no second
binary. clap reads argv[0], so a symlink named `jst` also works.

## Architecture

- Offline-first: `sync` + `curate` need net. Everything else reads local SQLite.
- Agentic-first: `context` works with zero Jev rows (health + FTS + evidence). Jev only upgrades tags/scores when present.
- `store.rs`: repos table + FTS5 + decision ledger.
- `scoring.rs`: deterministic health, no LLM. One file.
- `curate.rs` (optional, needs key): Jev batched, content-hash cached. N changed = N calls. Skipped entirely without key.
- `query.rs`: FTS top-50 + vector top-50 fused by RRF k=20, <350ms on 1,244 rows. Jev columns nullable.
- `notes.rs`: one-table memory `(repo, why, ts)`, surfaces in `context`.
- `mcp.rs`: stdio server, same fns as CLI. 7 tools.
- No key = full search + rank + context + status + export. `curate` is the only gated command.

## Project structure

```
src/
  main.rs     - CLI wiring, exit codes
  sync.rs     - gh api paginated pull, ETag
  store.rs    - rusqlite schema, FTS5, ledger
  scoring.rs  - alive/archived/last_push_days/license
  curate.rs   - Jev Choice+Score, cache-first
  query.rs    - FTS + vector RRF fuse, rank, context pack
  vector.rs   - BGE embed, hash384 fallback, exact-scan KNN
  notes.rs    - one-table memory
  export.rs   - STARS.md generation
  mcp.rs      - hand-rolled stdio, 7 tools
docs/         - PRD, ARCHITECTURE, DESIGN, HANDOFF
memory/       - session notes
```

## Conventions

- Small diffs. Stdlib + installed crates before new deps.
- `anyhow` in binary, no `unwrap` outside tests.
- Tests beside code. `cargo test` green before push.
- MD skeleton before code. No push without "go".
