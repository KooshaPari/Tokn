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
