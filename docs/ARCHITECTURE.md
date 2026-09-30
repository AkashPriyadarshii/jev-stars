# jev-stars ARCHITECTURE

## Modules

- `main.rs`: parse args (clap), dispatch, exit codes 0/1/2.
- `sync.rs`: shell out to `gh api` (zero token UI), paginate, ETag-gated, upsert.
- `store.rs`: rusqlite schema. `repos` + FTS5 + `decisions` ledger + `embeddings` + `notes`. One file, 4.03MB.
- `scoring.rs`: deterministic health from stored fields. No net, no LLM.
- `curate.rs` (optional): hash-gated Jev batch, miss only. Entire module inert without key; queries treat Jev columns as NULL.
- `vector.rs`: BGE-small-en-v1.5 via fastembed, exact-scan cosine KNN. hash384 fallback needs zero download.
- `query.rs`: FTS top-50 + vector top-50 fused by RRF k=20, then context pack. Works with zero Jev rows.
- `notes.rs`: one table `(repo, why, ts)`. Memory layer.
- `export.rs`: tag-grouped `STARS.md` writer.
- `mcp.rs`: hand-rolled stdio. Thin wrappers over `query`/`store` fns. Same code paths as CLI.

## Data flow

```
sync -> store -> scoring -> curate -> query/export -> mcp
gh api   SQLite   local     Jev       FTS+rank      stdio
         FTS5     facts     cached
                + ledger
```

## Ledger

Key `(repo, readme_hash, schema_v, model)`. Row holds `choice, score,
tags, evidence, timestamp`. Same key + same content = no second inference.
N new/changed repos = exactly N Jev calls.

## Concurrency

Sync pages sequentially (rate-limit friendly). Curate batches bounded
(8 in flight). Queries single-reader, no pool. 2C/4T safe.

## I/O

One SQLite file under user data dir. `--json` everywhere for pipes.
Errors to stderr, never wipe cache on failed sync.

## Error handling

`anyhow` in binary. No `unwrap` outside `#[cfg(test)]`. Failed page =
retry once, then abort with partial-cache-preserved error.

## v0.2 numbers (measured 2026-09-30)

- `search` p50 ~16-20ms. `similar` p50 ~330ms (BGE query-encode dominates).
- `context` p50 ~345ms. Vectors 1,244/1,244 BGE 384D.
- No ANN: 1,244 x 384 exact scan is microseconds. No quant: int8 saves ~1.4MB, unproven quality.
