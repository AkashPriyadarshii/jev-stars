# STATE

> Updated: 2026-09-29 (IST). Status: MD skeleton. No code yet. No remote yet.

## Done

- Researched landscape (mine + ChatGPT deep research, 14 tools mapped).
- Verified name: `jev-stars` free on crates.io (404), PyPI (404), no exact GitHub hit.
- Verified corpus: 1,243 starred repos via `gh api` Link header (`rel="last" page=1243`).
- Read ziggygrep MDs (CLAUDE/AGENTS/PRD/ARCHITECTURE/DESIGN/HANDOFF/README/CHANGELOG) + My-Starred-Repos MDs (CLAUDE/USAGE/STATE_HANDOFF/CHANGELOG) as templates.
- Wrote MD skeleton in this folder. No code, no `gh repo create` yet.
- Direction locked 2026-09-29: agentic-first, Jev optional. `context` serves health + FTS + evidence with zero Jev rows; `curate` is the only key-gated command.

## Next (needs "go")

1. `gh repo create jev-stars --public` + metadata (description, topics, homepage).
2. Chunk 1: `sync + store` (1243 rows round-trip, `cargo test` green).
3. Chunk 2: `search + rank + context` (`search "mcp rust"` <100ms).
4. Chunk 3: `curate + export` (rerun = zero Jev calls).
5. Chunk 4: `mcp` stdio (pi `context` returns rows).

## Open questions

- Default sync auth: shell out to `gh api` (zero token UI) vs direct HTTPS with stored token. Lean `gh` first.
- Jev schema v1 tags: fixed taxonomy (reuse My-Starred-Repos 13 categories) vs free tags. Lean fixed first.
- sqlite-vec in v0.2: yes per benchmark, not now.

## Identity

- Git: `AkashPriyadarshii` + `272530059+AkashPriyadarshii@users.noreply.github.com`, HTTPS via gh credential manager.
- No push without explicit "go".
