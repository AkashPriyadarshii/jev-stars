# jev-stars ARCHITECTURE

## Modules

- `main.rs`: parse args (clap), dispatch, exit codes 0/1/2.
- `sync.rs`: shell out to `gh api` (zero token UI), paginate, ETag-gated, upsert.
- `store.rs`: rusqlite schema. `repos` + FTS5 + `decisions` ledger. One file, ~6MB.
- `scoring.rs`: deterministic health from stored fields. No net, no LLM.
- `curate.rs` (optional): hash-gated Jev batch, miss only. Entire module inert without key; queries treat Jev columns as NULL.
- `query.rs`: FTS query + filters + rank merge + context pack with token cap. Works with zero Jev rows.
- `export.rs`: tag-grouped `STARS.md` writer.
- `mcp.rs`: rmcp stdio. Thin wrappers over `query`/`store` fns. Same code paths as CLI.

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
