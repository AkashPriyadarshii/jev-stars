# Contributing

## How to contribute

1. Fork, branch, open a PR with the template below.
2. Keep the diff small. One concern per PR.
3. Stdlib and installed crates before new deps. Justify every new dep.
4. Non-trivial logic ships with one runnable check (`cargo test`).
5. Update docs when behavior changes. README describes the present, CHANGELOG records the past.

## PR template

- What + why (one paragraph).
- `cargo test` green, `cargo clippy --all-targets --locked -- -D warnings` clean.
- Measured numbers for perf claims, with rerun commands.

## Checks before push

```bash
cargo test
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --check
```

No push without explicit "go". PR authors skip the changelog; the maintainer adds the entry and thanks you at land.
