# Known Issues

## Deferred Integration

Canonical Tokn stayed unchanged because repo-local instructions prohibit direct
commits to `main`.

## Superseded Branch

The older `docs/tokn-sladge-current` branch at `11b3fb3` diverged from current
local `main` and should be treated as stale evidence after this refresh.

## Pre-Existing Pareto Test Drift

`cargo test --workspace --offline` fails existing test
`format::tests::test_round2_basic` in `pareto-rs` because
`round2(PI)` does not satisfy the current assertion against `PI.round()`.
