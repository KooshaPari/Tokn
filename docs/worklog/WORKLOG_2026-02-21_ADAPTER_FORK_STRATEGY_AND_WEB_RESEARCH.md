# Worklog 2026-02-21: Adapter Fork Strategy and Web Research

## Context

User requested an explicit decision on whether to fork a full Rust monolith versus separate provider-focused solutions, plus a written-down methodology that supports future provider expansion.

## What was completed

1. Performed wider web scan for:
- multi-provider usage trackers and UI references,
- Rust/Go parser and pipeline candidates,
- performance-oriented crate options for tail/parse/cache/bench.

2. Codified architecture decision:
- no monolithic Rust fork,
- yes to separate provider-focused adapter imports/forks,
- `tokenledger` remains canonical core.

3. Added formal research/policy document:
- `docs/research/ADAPTER_BASE_LIBS_AND_FORK_STRATEGY_2026-02-21.md`

4. Captured subscription and token pricing methodology in a unified blended Mtok model, including derivation metadata modes and comparability outputs.

## Key policy outcome

1. Provider adapters are replaceable modules that emit normalized JSONL.
2. Fork only narrow parser slices when needed, never whole product monoliths.
3. Enforce adapter conformance + performance gates under governance.
4. Keep pricing derivation transparent and auditable (`token`, `subscription_derived`, `subscription_manual`).

## Sources reviewed (primary)

1. OpenUsage: `https://www.openusage.ai/`
2. CodexBar: `https://github.com/steipete/CodexBar`
3. opencode: `https://github.com/sst/opencode`
4. ccusage: `https://github.com/ryoppippi/ccusage`
5. ccstat: `https://crates.io/crates/ccstat`
6. cxusage: `https://github.com/johanneserhardt/cxusage`
7. tokscale: `https://github.com/junhoyeo/tokscale`

## Next execution priorities

1. Land real adapters (`~/.claude`, `~/.codex`, Cursor DB/logs, Droid logs) to normalized JSONL.
2. Add per-adapter fixture conformance tests with golden outputs.
3. Add benchmark/perf gates for cold backfill, warm tail, and burst ingest in CI.
4. Add optional SQLite + disk cache acceleration with bounded overhead.

## Closeout update (implemented)

1. Adapter coverage is active for Claude/Codex/Cursor (logs + SQLite), ProxyAPI/CLIProxyAPI, and Droid sources in ingest discovery + normalization paths.
2. Provider fixture/conformance coverage is present in `src/main.rs` tests, including adapter-shape normalization and Cursor SQLite deterministic selection/fallback behavior.
3. CI benchmark/perf gating added at `.github/workflows/bench-perf-gate.yml`:
- runs `bench --scenario all`
- enforces strict perf gates via `scripts/perf_gate.sh` with baseline.
4. Optional SQLite + disk cache acceleration is active via SQLite ingestion path and orchestrate ingest cache controls.
