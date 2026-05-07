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
