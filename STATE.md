# STATE

> Updated: 2026-09-30 (IST). Status: v0.2 code complete, local only. Nothing pushed.

## Done

- v0.1 shipped (crates.io + GitHub release). Corpus 1,244 stars.
- v0.2 chunk 1: BGE-small-en-v1.5 swap. 1,244/1,244 embedded. DB 4.03MB.
- v0.2 chunk 2: RRF fuse (FTS top-50 + vector top-50, k=20). `similar` + `context` hybrid.
- v0.2 chunk 3: `notes` one-table memory. `note` CLI + MCP tool. `context` carries `note`.
- v0.2 chunk 4: measured. search p50 ~20ms, similar p50 ~330ms, context p50 ~345ms.
- 8 tests green, clippy clean. `.fastembed_cache/` gitignored.
- Direction: agentic-first, Jev optional. `curate` is the only key-gated command.

## Next

1. Release v0.2 (needs "go").
2. Full `curate` remainder in `--limit` batches (optional).
3. v0.3: temporal queries on notes + pushed_at.

## Rules

- No push, no release without explicit "go".
- Repo holds zero personal paths, keys, or machine-local config.
