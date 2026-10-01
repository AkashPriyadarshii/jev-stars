# Changelog

All notable changes. Keep a Changelog format: Added, Changed, Fixed per release.

## [Unreleased]

- Site v2: asymmetric split hero with live terminal, proof cards + ledger strip, 7-cell bento + starfield, Find/Remember/Serve accordion. `site/` is source, `docs/` is the Pages deploy root.

## [0.2.0] - 2026-09-30

- Added `similar`: hybrid FTS top-50 + BGE-small-en-v1.5 top-50 fused by RRF k=20.
- Added `embed`: batched + resumable BGE index build. hash384 `--hash` fallback needs zero download.
- Added `note`: one-table memory `(repo, why, ts)`. `context` + MCP surface it.
- Changed `context` to hybrid retrieval with `hybrid-RRF` why_matched.
- Changed MCP to 7 tools: `search, similar, context, note, status, sync, export`.
- Measured: search p50 ~20ms, similar p50 ~330ms, context p50 ~345ms. DB 4.03MB.
- Skipped: ANN index, int8 quant, repo_events table.

## [0.1.0] - 2026-09-29

- `sync`: 1,244 stars via `gh api` into SQLite + FTS5 (porter).
- `search`/`context`/`status`: offline, <50ms. `context` returns bounded agent JSON with Jev fields (null until curated).
- `curate` (optional, key-gated): Jev Choice category + Score fit, hash-ledgered, 3 repos/call. Rerun = zero calls.
- `export`: `STARS.md`, archived skipped, grouped by language.
- `mcp`: stdio server, 5 tools.
- Alias: `jst` (shell-only, `js` rejected).
