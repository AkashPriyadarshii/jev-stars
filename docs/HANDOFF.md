# jev-stars HANDOFF

## Status

v0.1 + v0.2.0 shipped (tag + GitHub Release + crates.io). Site v2 live on Pages from `docs/`.

## What shipped (v0.2.0, released)

1. `similar` + `embed`: BGE-small-en-v1.5 384D, RRF k=20 fuse, hash384 fallback.
2. `note`: one-table memory, CLI + MCP, surfaces in `context`.
3. `context` hybrid. MCP 7 tools. 8 tests green, clippy clean.
4. Docs: README, AGENTS, PRD, ARCHITECTURE, STATE, CHANGELOG, site all match v0.2.

## Remaining

- Full `curate` remainder optional, `--limit` batches.
- v0.3: temporal queries + `context --mine`.

## Proven inputs

- ziggygrep MDs: brevity pattern for CLAUDE/AGENTS/docs.
- My-Starred-Repos: pipeline shape (fetch -> store -> build/export), 13-category taxonomy seed.
- v0.2 research: exact scan beats sqlite-vec at 1,244 rows. BGE beats MiniLM on MTEB retrieval.

## Open questions

- ANN past 50k rows. int8 when DB size hurts. repo_events when notes prove short.
