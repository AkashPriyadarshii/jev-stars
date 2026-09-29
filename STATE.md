# STATE

> Updated: 2026-09-29 (IST). Status: v0.1 shipped. Remote live.

## Done

- Researched landscape (mine + ChatGPT deep research, 14 tools mapped).
- Verified name: `jev-stars` free on crates.io (404), PyPI (404), no exact GitHub hit.
- Corpus: 1,244 starred repos synced (Link header said 1,243; live pull = 1,244).
- Chunk 1: `sync + store` green. Chunk 2: `search + context` green (47ms).
- Chunk 3: `curate` live against Jev (3 repos tagged), `export` 1,236 rows.
- Chunk 4: `mcp` stdio green, 5 tools. Binary at `~/.cargo/bin/jev-stars.exe`, DB at `~/.jev-stars.db`, pi settings wired.
- Alias `jst` locked. `target/` purged + gitignored.
- Direction: agentic-first, Jev optional. `curate` is the only key-gated command.

## Next

1. Push v0.1 (needs "go").
2. Full `curate` run: 1,244 repos ~415 Jev calls. Recommend `--limit` batches.
3. v0.2: sqlite-vec hybrid + `similar` tool (only when FTS falls short).

## Open questions

- sqlite-vec in v0.2: yes per benchmark, not now.

## Identity

- Git: `AkashPriyadarshii` + `272530059+AkashPriyadarshii@users.noreply.github.com`, HTTPS via gh credential manager.
- No push without explicit "go".
