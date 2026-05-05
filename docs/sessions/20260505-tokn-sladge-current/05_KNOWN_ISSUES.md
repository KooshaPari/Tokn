# Known Issues

- Canonical Tokn is locally ahead 1 and behind `origin/main` by 1.
- The stale `Tokn-wtrees/sladge-badge` branch is far behind current local
  `main` and should not be treated as current evidence.
- Online Cargo validation cannot resolve `index.crates.io` in this sandbox.
- Offline Rust tests fail one pre-existing `pareto-rs` unit assertion:
  `format::tests::test_round2_basic`.
