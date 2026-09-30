# jev-stars HANDOFF

## Status

v0.1 shipped. v0.2 code complete, local only, nothing pushed. Waiting on "go" for release.

## What shipped (v0.2, unreleased)

1. `similar` + `embed`: BGE-small-en-v1.5 384D, RRF k=20 fuse, hash384 fallback.
2. `note`: one-table memory, CLI + MCP, surfaces in `context`.
3. `context` hybrid. MCP 7 tools. 8 tests green, clippy clean.
4. Docs: README, AGENTS, PRD, ARCHITECTURE, STATE, CHANGELOG, site all match v0.2.

## Remaining for v0.2 release

- Version bump + tag + push (needs "go").
- `cargo publish` after tag.
- Full `curate` remainder optional, `--limit` batches.

## Proven inputs

- ziggygrep MDs: brevity pattern for CLAUDE/AGENTS/docs.
- My-Starred-Repos: pipeline shape (fetch -> store -> build/export), 13-category taxonomy seed.
- v0.2 research: exact scan beats sqlite-vec at 1,244 rows. BGE beats MiniLM on MTEB retrieval.

## Open questions

- ANN past 50k rows. int8 when DB size hurts. repo_events when notes prove short.
