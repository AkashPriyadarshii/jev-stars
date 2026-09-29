# jev-stars DESIGN

CLI tool. No web UI. This file records interface decisions so output stays stable.

## Output shape

Default `search`: human table, one repo per line.

```
full_name  stars  lang  pushed  license  tags
```

`--json`: array of repo objects, same fields as `context.results[]`.
`context`: single JSON object `{query, count, results[]}`, each result
carries `why_matched` + `readme_excerpt` (capped 300 chars).
Jev fields (`choice`, `score`, `tags`) are null when never curated.

## Truncation

`--limit N` caps rows (default 10, max 50). Excerpts cut on UTF-8
boundary. Explicit beats surprise: no silent server-side top-k beyond limit.

## Alias

`jst`, never `js`. Shell alias / symlink only; one binary ships.
Help text still shows `jev-stars`; clap follows argv[0] at runtime.

## Errors

`jev-stars: reason` on stderr. Exit 0 ok, 1 no match, 2 usage or I/O error.

## Dials

ENERGY 1 / RHYTHM 1 / MOTION 1. Terminal output has no motion
or decoration. Clarity is the design.
