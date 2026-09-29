# jev-stars HANDOFF

## Status

MD skeleton complete. No code, no remote. Waiting on "go" for `gh repo create`.

## What shipped (docs only)

1. `CLAUDE.md` + `AGENTS.md` (build, commands, search architecture).
2. `README.md` (SEO block, Why, quickstart, MCP, limits, ecosystem, author).
3. `docs/PRD.md` (v0.1 scope, non-goals, success criteria).
4. `docs/ARCHITECTURE.md` (modules, ledger, data flow).
5. `docs/DESIGN.md` (output shapes, truncation, dials).
6. `STATE.md` + `CHANGELOG.md`.

## Remaining for v0.1

- `gh repo create jev-stars --public` + description/topics/homepage.
- Chunk 1: `sync + store`. Check: row count == 1,243.
- Chunk 2: `search + rank + context`. Check: <100ms.
- Chunk 3: `curate + export`. Check: rerun zero Jev calls.
- Chunk 4: `mcp` stdio. Check: pi `context` returns rows.

## Proven inputs

- ziggygrep MDs: brevity pattern for CLAUDE/AGENTS/docs.
- My-Starred-Repos: pipeline shape (fetch -> store -> build/export), 13-category taxonomy seed, `escapeHTML`-style output hygiene reminder.
- sqlite-vec benchmark: FTS5-only correct for v0.1, hybrid in v0.2.

## Open questions

- `gh api` subprocess vs direct HTTPS. Lean subprocess first.
- Fixed tag taxonomy vs free tags. Lean fixed (13 categories) first.
