# Merged Fragmented Markdown

## Source: plans/INDEX.md

# Consolidated Index

## Files

* `INDEX.md`
* `UNIFIED_E2E_EXECUTION_PLAN.md`
* `UNIFIED_MODEL_PROVIDER_LEDGER_PLAN_2026-02-21.md`

## Subdirectories


---

## Source: plans/UNIFIED_E2E_EXECUTION_PLAN.md

# Unified E2E Execution Plan

Date: 2026-02-21

## Scope

Deliver a production-ready local-first analytics pipeline with extension-ready UX integration and governance-grade artifacts.

## Phase Plan

## Phase 0: Contracts and Baselines

1. Freeze normalized event schema and UI snapshot schema.
2. Add schema version fields to exported artifacts.
3. Lock benchmark correctness golden baselines.

Exit criteria:
- Contract docs and tests exist,
- schema changes require explicit migration notes.

## Phase 1: Adapter Hardening

1. Implement true Cursor SQLite adapter using table-level queries.
2. Keep best-effort JSON extraction as fallback with telemetry.
3. Expand provider conformance fixtures/tests.

Exit criteria:
- Cursor DB fixtures parse deterministically,
- adapter test matrix covers all providers.

## Phase 2: Cache Layer (Disk-backed)

1. Add parsed-event cache keyed by source fingerprint.
2. Add aggregate cache keyed by month/filter/pricing hash.
3. Add cache metrics (hit/miss, invalidate counts).

Exit criteria:
- warm runs materially faster than cold,
- correctness equivalent to uncached path.

## Phase 3: UI/UX Integration

1. Stabilize `ui-snapshot` schema and document compatibility policy.
2. Add optional compact/extended snapshot modes.
3. Provide extension integration examples (CodexBar/OpenCode style).

Exit criteria:
- extension reads file artifact only,
- no additional runtime service required.

## Phase 4: Governance + CI Gates

1. Persist per-run pipeline summaries and rolling pointers.
2. Enforce pricing audit/lint/check in CI profiles.
3. Enforce perf and correctness gates with baseline handling.

Exit criteria:
- do:all:next outputs complete artifact bundle,
- CI catches correctness/perf regressions.

## Phase 5: Unified Model + Provider Ledger

1. Build deterministic ledger generator from:
- `pricing.example.json`
- `models_normalized.csv`

2. Emit machine-consumable artifacts:
- unified ledger CSV
- SQL schema
- SQL seed

3. Include mapping provenance:
- provider mapping rule + confidence,
- model mapping rule + confidence,
- benchmark prior coverage stats.

4. Feed cliproxyapi telemetry (Proxyapi adapter) into the same canonical ledger join model.

Exit criteria:
- unified ledger artifacts regenerate deterministically from one command,
- ledger provides pricing + benchmark priors on one row surface,
- feeder contracts documented for cliproxyapi/OTEL-derived usage.

## Work Packages

1. WP-A: Cursor SQLite parser module and tests.
2. WP-B: Disk cache module + orchestration wiring.
3. WP-C: Snapshot schema versioning + extension docs.
4. WP-D: Pipeline summary artifact + CI job templates.
5. WP-E: Unified provider/model ledger generator + schema/seed governance.

## Risks and Mitigations

1. Provider schema churn:
- Mitigation: versioned adapter mappers + fixture updates.

2. Cache staleness:
- Mitigation: strict fingerprint keys + bypass flag.

3. UI contract drift:
- Mitigation: schema version and compatibility table.

4. Perf false regressions:
- Mitigation: baseline month/workload matching and trend windows.

5. Ledger mapping drift:
- Mitigation: explicit mapping rule/confidence fields and deterministic regeneration from source seeds.

## Milestone Acceptance

1. `task do:all:next` passes with full artifact outputs.
2. `task orchestrate:ui` produces stable snapshot.
3. Perf gate and golden correctness checks pass.
4. Reconcile summaries are machine-consumable and auditable.
5. Unified ledger artifacts exist and validate in SQLite load tests.

## Closeout Update (2026-02-21)

The following closure items are now completed:

1. Normalized event schema contract v1:
- `docs/contracts/NORMALIZED_EVENT_SCHEMA_CONTRACT_V1.md`

2. UI snapshot schema contract v1 + compatibility policy:
- `docs/contracts/UI_SNAPSHOT_SCHEMA_CONTRACT_V1.md`

3. Extension integration examples for CodexBar/OpenCode-style file reads:
- `docs/integrations/EXTENSION_FILE_READ_INTEGRATION_EXAMPLES.md`

4. CI workflow for pricing governance checks:
- `.github/workflows/pricing-governance-check.yml`
- runs `pricing-lint`, `pricing-audit`, and `pricing-check`

5. Operator shortcuts and docs references:
- `task pricing:ci`
- `cargo run -- pricing-lint --pricing ./pricing.example.json`
- `cargo run -- pricing-audit --pricing ./pricing.example.json`
- `cargo run -- pricing-check --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02`

## Closeout Batch: Cache-Layer Operator Wiring (2026-02-21)

The following cache-layer closure items are completed:

1. Task wiring for aggregate cache orchestration:
- `task orchestrate`, `task orchestrate:ui`, and `task do:all:next` now pass `--aggregate-cache-path`.
- Added `task orchestrate:cache` for sample-data aggregate cache warm runs.

2. Cache metrics validation operator task:
- Added `task orchestrate:cache:metrics:validate`.
- Validates `miss -> hit -> invalidate` transitions via `aggregate_cache` metrics in pipeline summaries.

3. README operator command updates:
- Orchestrate command examples now include `--aggregate-cache-path`.

4. Cache semantics documentation:
- Added cache key and metrics semantics (`hit/miss/invalidate`) in governance docs.

5. CI cache-path exercise profile:
- Added pricing-governance CI step that runs sample-data orchestrate with aggregate cache and validates cache metrics on second run.

## Closeout Batch: Baseline and Contract Guards (2026-02-21)

The following governance gaps are now closed:

1. Benchmark golden baseline lock procedure:
- Added `task bench:golden:lock` to regenerate fixture from canonical sample inputs.
- Added `task bench:golden:lock:verify` to regenerate and immediately verify with `--golden`.

2. Golden baseline enforcement in CI:
- `.github/workflows/bench-perf-gate.yml` now runs a golden verification step before strict perf-gate checks.

3. Golden fixture strictness hardening:
- runtime verification now fails when golden contains scenarios absent from current report output.

4. Cache contract version guards:
- ingest and aggregate cache loaders ignore/reset stale versioned cache payloads instead of reusing incompatible serialized state.

---

## Source: plans/UNIFIED_MODEL_PROVIDER_LEDGER_PLAN_2026-02-21.md

# Unified Model + Provider Ledger Plan (2026-02-21)

## Objective

Create one canonical ledger that joins:

1. Provider/model pricing data,
2. model benchmark priors,
3. mapping provenance (rules/confidence),
4. runtime telemetry feeder compatibility (including cliproxyapi/OTEL-normalized usage).

## Inputs

1. `pricing.example.json`
2. `models_normalized.csv`
3. Ingested normalized JSONL events (including `provider=proxyapi`)

## Outputs

1. `ledger/unified_model_provider_ledger.csv`
2. `ledger/unified_model_provider_ledger.schema.sql`
3. `ledger/unified_model_provider_ledger.seed.sql`

## Generation pipeline

1. Parse pricing and canonical provider/model maps.
2. Parse normalized model benchmark rows.
3. Infer provider and canonical model candidates for each source model.
4. Attach benchmark priors and pricing deltas.
5. Emit CSV + SQL schema + SQL inserts.

## Mapping methodology

1. Provider mapping:
- deterministic token/prefix-based heuristics,
- explicit `provider_mapping_rule`,
- integer `provider_mapping_confidence`.

2. Model mapping:
- exact pricing model match first,
- alias/model-family heuristics second,
- explicit `model_mapping_rule`,
- integer `model_mapping_confidence`.

3. Unknown/unmapped models:
- retained in output with null pricing,
- benchmark priors preserved for analysis.

## Runtime integration scope

1. `tokenledger ingest --provider proxyapi` emits normalized JSONL from cliproxyapi/OTEL and usage payloads.
2. Runtime events join to ledger on canonicalized provider/model.
3. Blended cost + benchmark-aware Pareto scoring can be computed from unified rows.

## Governance

1. Ledger artifacts are generated, not manually edited.
2. Regeneration command must be deterministic.
3. Mapping confidence/rule fields are mandatory for auditability.
4. SQL seed must load cleanly in SQLite checks.

## Execution checklist

1. Run generator:
- `python3 ./scripts/build_unified_ledger.py`

2. Validate load:
- `sqlite3 :memory: ".read ./ledger/unified_model_provider_ledger.schema.sql" ".read ./ledger/unified_model_provider_ledger.seed.sql" "SELECT COUNT(*) FROM unified_model_provider_ledger;"`

3. Validate feeder:
- `cargo run -- ingest --provider proxyapi --output ./artifacts/proxyapi-events.jsonl`

4. Validate end-to-end orchestrate:
- `cargo run -- orchestrate --providers proxyapi --events-out ./artifacts/proxyapi-events.jsonl --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --pricing-reconcile-dry-run`

## Closeout update (2026-02-21)

1. ProxyAPI feeder validation taskchain is now explicit and repeatable:
- `task ingest:proxyapi:validate`
- `task orchestrate:proxyapi:validate`
- `task ledger:proxyapi:e2e:validate` (runs both in sequence)

2. End-to-end orchestrate validation now persists summary artifacts under:
- `benchmarks/results/proxyapi-ingest-summary.json`
- `benchmarks/results/proxyapi-orchestrate-summary.json`
- `benchmarks/results/proxyapi-ui-snapshot.json`

---
