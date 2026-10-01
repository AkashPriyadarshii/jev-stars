# STATE

> Updated: 2026-10-01. Status: v0.2.0 released (tag + GitHub Release + crates.io). Site v2 live.

## Done

- v0.1 shipped (crates.io + GitHub release). Corpus 1,244 stars.
- v0.2.0 shipped: BGE-small-en-v1.5 swap (1,244/1,244 embedded, DB 4.03MB), RRF fuse (FTS top-50 + vector top-50, k=20), `notes` one-table memory, MCP 7 tools.
- Measured: search p50 ~20ms, similar p50 ~330ms, context p50 ~345ms. 8 tests green, clippy clean.
- Site v2: asymmetric split hero with live terminal, proof cards + ledger strip, 7-cell bento + starfield, Find/Remember/Serve accordion. `site/` is source, `docs/` is the Pages deploy root (mirror on every site change).

## Next

1. Full `curate` remainder in `--limit` batches (optional).
2. v0.3: temporal queries on notes + pushed_at.

## Rules

- No push, no release without explicit "go".
- Repo holds zero personal paths, keys, or machine-local config.
