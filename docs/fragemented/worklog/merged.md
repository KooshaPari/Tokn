# Merged Fragmented Markdown

## Source: worklog/INDEX.md

# Consolidated Index

## Files

* `INDEX.md`
* `WORKLOG_2026-02-21_ADAPTER_FORK_STRATEGY_AND_WEB_RESEARCH.md`
* `WORKLOG_2026-02-21_E2E_PLAN_AND_UI_INTEGRATION.md`
* `WORKLOG_2026-02-21_MODEL_DB_SEED_NORMALIZATION.md`
* `WORKLOG_2026-02-21_UNIFIED_MODEL_PROVIDER_LEDGER_AND_CLIPROXYAPI_FEEDER.md`

## Subdirectories


---

## Source: worklog/WORKLOG_2026-02-21_ADAPTER_FORK_STRATEGY_AND_WEB_RESEARCH.md

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

---

## Source: worklog/WORKLOG_2026-02-21_E2E_PLAN_AND_UI_INTEGRATION.md

# Worklog 2026-02-21: Unified E2E Plan and UI Integration

## Context

User requested full end-to-end planning and unified docs structure following thegent-style worklog/research organization.

## Completed in this cycle

1. Added orchestrate-based UI snapshot export for statusbar/extension integration:
- `--ui-snapshot-path`
- compact payload with totals, top providers/models, suggestions.

2. Added reconcile governance artifacts:
- per-run reconcile directories,
- reconcile summary outputs,
- rolling latest reconcile summary.

3. Hardened orchestration with additional regression tests around:
- allow-unpriced logic,
- baseline month matching,
- perf gate strict/missing scenario failures.

4. Added and validated docs + task shortcuts for operator workflows.

## Planning outcome captured

1. Research artifact:
- `docs/research/UNIFIED_E2E_RESEARCH_AND_ARCHITECTURE_2026-02-21.md`

2. Execution plan:
- `docs/plans/UNIFIED_E2E_EXECUTION_PLAN.md`

3. Governance policy:
- `docs/governance/PIPELINE_GOVERNANCE_AND_ARTIFACT_POLICY.md`

4. Docs indices:
- `docs/index.md`
- `docs/research/INDEX.md`
- `docs/plans/INDEX.md`
- `docs/worklog/INDEX.md`

## Next do-all-next priorities

1. Implement true Cursor SQLite table parser with deterministic fixtures.
2. Add disk-backed caching for normalized events and aggregate views.
3. Add pipeline-level summary artifact consolidating ingest/reconcile/report/bench/gate in one JSON.

## Closeout update (implemented)

1. Cursor SQLite table-backed ingestion + deterministic selection tests are implemented in `src/main.rs`.
2. Disk-backed ingest caching for orchestrate is implemented via `--ingest-cache-path` and cache-hit reuse behavior.
3. Pipeline-level summary artifact is implemented via `--pipeline-summary-path` and includes ingest/reconcile/monthly/daily/bench/gate stage results.

## Validation notes

1. Keep using:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `task do:all:next`
- `task orchestrate:ui`

2. Verify artifacts after each run:
- `benchmarks/results/reconcile-*/`
- `benchmarks/results/reconcile-latest-summary.json`
- `benchmarks/results/ui-snapshot.json` (when enabled)

## Follow-up closeout (command reliability)

1. Added explicit artifact bootstrap task and dependency wiring:
- `task artifacts:ensure` now creates `./artifacts` and `./benchmarks/results` before orchestrate-style task runs.
- `task do:all:next`, `task orchestrate`, and `task orchestrate:ui` depend on `artifacts:ensure`.

2. Moved orchestrate sample ingest output to artifact-managed paths:
- `--events-out ./artifacts/ingested.sample.jsonl` for codex orchestrate flows.
- committed keepfile at `artifacts/.gitkeep` so artifact output location exists in clean checkouts.

3. Added proxyapi validation shortcuts for fast execution checks:
- `task ingest:proxyapi:validate`
- `task orchestrate:proxyapi:validate`

## Follow-up closeout (contracts + pricing governance CI)

1. Added normalized event schema contract v1:
- `docs/contracts/NORMALIZED_EVENT_SCHEMA_CONTRACT_V1.md`

2. Added UI snapshot schema contract v1 with compatibility policy:
- `docs/contracts/UI_SNAPSHOT_SCHEMA_CONTRACT_V1.md`

3. Added extension integration examples for file-based consumers:
- `docs/integrations/EXTENSION_FILE_READ_INTEGRATION_EXAMPLES.md`

4. Added pricing governance CI workflow:
- `.github/workflows/pricing-governance-check.yml`
- stages: `pricing-lint`, `pricing-audit`, `pricing-check`

5. Added operator shortcut for local parity with CI:
- `task pricing:ci`

## Follow-up closeout (baseline + cache contract guards)

1. Added golden baseline lock flow:
- `task bench:golden:lock`
- `task bench:golden:lock:verify`

2. Bench/perf CI now verifies golden fixture correctness before strict perf gating:
- `.github/workflows/bench-perf-gate.yml`

3. Hardened cache contract handling:
- stale ingest cache versions are ignored,
- stale aggregate cache versions are reset to empty before reuse.

---

## Source: worklog/WORKLOG_2026-02-21_MODEL_DB_SEED_NORMALIZATION.md

# Worklog 2026-02-21: Model DB Seed Normalization

## Context

`tokenledger/models.csv` was user-pasted baseline data but not valid CSV. It is a markdown-style pipe table and must be treated as text input.

## Completed

1. Added reproducible generator:
- `scripts/build_model_seed.py`

2. Generated review/query artifacts from `models.csv`:
- `models_normalized.csv` (valid long-format CSV)
- `models_schema_seed.sql` (DDL + inserts for SQL workflows)

3. Added README instructions under:
- `Model Database Seed (CSV + SQL)`

## Normalization rules

1. Missing tokens: `''`, `—`, `-`, `N/A`, `n/a`, `na`, `NA`, `null`, `NULL`.
2. Split mixed values once on `/` into `value_primary` + `value_secondary`.
3. Preserve raw source token in `raw_value`.
4. Track provenance columns: `source_row_index`, `source_col_index`, `source_col_name`.

## Validation

1. SQL seed loads in SQLite memory DB.
2. Inserted rows match normalized CSV row count.

---

## Source: worklog/WORKLOG_2026-02-21_UNIFIED_MODEL_PROVIDER_LEDGER_AND_CLIPROXYAPI_FEEDER.md

# Worklog 2026-02-21: Unified Model/Provider Ledger and CLIProxyAPI Feeder

## Context

User requested a unified model+provider ledger and explicit planning/worklog coverage, with cliproxyapi telemetry as an additional feeder.

## Completed

1. Added unified ledger generator:
- `scripts/build_unified_ledger.py`

2. Generated deterministic ledger artifacts:
- `ledger/unified_model_provider_ledger.csv`
- `ledger/unified_model_provider_ledger.schema.sql`
- `ledger/unified_model_provider_ledger.seed.sql`

3. Added plan coverage:
- updated `docs/plans/UNIFIED_E2E_EXECUTION_PLAN.md` with Phase 5 + WP-E
- added `docs/plans/UNIFIED_MODEL_PROVIDER_LEDGER_PLAN_2026-02-21.md`

4. Added index links:
- `docs/index.md`
- `docs/plans/INDEX.md`
- `docs/worklog/INDEX.md`

## Ledger characteristics

1. One-row surface for model/provider joins with:
- pricing fields,
- benchmark prior coverage counts,
- mapping provenance fields (`rule` + `confidence`).

2. Deterministic generation from:
- `pricing.example.json`
- `models_normalized.csv`

3. SQL-loadable schema/seed for DB review and analytics.

## Validation outcomes

1. Generator run produced:
- `ledger_rows=56`
- `priors_aggregation_rows=13`

2. SQLite load check passed with matching counts.

## Feeder alignment notes

1. `tokenledger` now supports `proxyapi` ingest normalization for OTEL-like span attributes and usage payload shapes.
2. Unified ledger plan includes runtime join model for proxyapi-ingested canonical events.

## Next steps

1. Add explicit Pareto scoring view (`cost x latency x quality`) derived from unified ledger + runtime metrics.
2. Wire cliproxyapi management export/metrics snapshot import path into recurring ledger refresh flow.
3. Add CI check that regenerates and diff-checks ledger artifacts.

## Closeout update (implemented)

1. Pareto scoring artifacts added:
- `scripts/refresh_ledger.py`
- `ledger/unified_model_provider_pareto.csv`
- `ledger/unified_model_provider_pareto.view.sql`

2. CLIProxyAPI snapshot import wired into recurring refresh flow:
- snapshot auto-discovery + explicit `--cliproxyapi-snapshot` support in `scripts/refresh_ledger.py`
- normalized runtime snapshot export at `ledger/cliproxyapi_runtime_metrics_snapshot.csv`

3. CI diff check added:
- `.github/workflows/ledger-diff-check.yml`
- deterministic regeneration command: `python3 ./scripts/refresh_ledger.py --allow-missing-snapshot --skip-runtime-discovery`

## Follow-up closeout (validation shortcut)

1. Added Taskfile SQL validation shortcut for ledger artifacts:
- `task ledger:sql:validate`
- loads `ledger/unified_model_provider_ledger.schema.sql` + `ledger/unified_model_provider_ledger.seed.sql` into in-memory SQLite and prints `ledger_rows=<count>`.

2. Exposed this shortcut in operator docs (`README.md`) under Task shortcuts to make recurring SQL validation explicit and repeatable.

---
