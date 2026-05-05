# Testing Strategy

## Completed

- `git diff --check`
- README badge presence with `rg`
- `cargo fmt --all --check`
- `cargo clippy --workspace --offline -- -D warnings`

## Blocked or Failing

- `cargo test --workspace` is blocked by DNS resolution for `index.crates.io`.
- `cargo test --workspace --offline` compiles but fails
  `format::tests::test_round2_basic` in `pareto-rs`.
- `cargo test -p pareto-rs --lib --offline -- --nocapture` reproduces the same
  rounding assertion failure at `crates/pareto-rs/src/format.rs:22`.
