# Unified E2E Research and Architecture (2026-02-21)

## Objective

Define an end-to-end architecture for usage/cost analytics across Cursor, Droid, Codex, and Claude with:

- blended subscription + token economics,
- provider/model canonical mappings,
- low-overhead extension/statusbar integration,
- realtime-friendly performance,
- governance-ready artifact trails.

## Key Findings

1. Existing fast path in Rust is viable for local realtime use.
2. Current SQLite ingestion is best-effort JSON extraction; true DB-backed adapter parsing is still required.
3. UI integration should be file-based first (`ui-snapshot.json`) for minimal overhead and broad compatibility.
4. Governance needs immutable per-run artifacts + rolling summary pointers.
5. Cache layer is the next major performance/latency lever (parsed-event cache + aggregate cache).

## Reference Architecture

1. Ingest adapters:
- Claude JSONL
- Codex JSONL
- Cursor logs + real SQLite tables
- Droid JSON/JSONL

2. Normalization:
- Canonical `UsageEvent`
- Provider/model alias resolution
- Contract and conformance tests

3. Pricing engine:
- Coverage/patch/apply/check lifecycle
- Metadata audit and provenance controls
- Recompute-friendly immutable token events

4. Analytics:
- Monthly/daily blended economics
- Per-provider and per-model drilldown
- Suggestions/optimization signals

5. Snapshot/UI:
- Compact snapshot JSON for statusbar/extension use
- Optional richer local API later if needed

6. Perf and governance:
- Bench (cold/warm/burst), baseline and trend checks
- Correctness goldens
- Reconcile and pipeline artifacts per run

## Integration Model for CodexBar/OpenCode-style UX

1. Producer:
- `tokenledger orchestrate --ui-snapshot-path <path>`

2. Consumer:
- Extension polls snapshot file at low interval (1-5s) or on fs-watch.

3. Contract:
- fixed schema versioning recommended before publishing external extension.

4. Overhead controls:
- snapshot generation only when flag enabled,
- small payload (totals + top rows + suggestions),
- no daemon/network required for baseline integration.

## Next Technical Priorities

1. Real Cursor SQLite adapter with deterministic table mapping.
2. Disk-backed caches:
- parsed normalized event cache,
- aggregate snapshot cache keyed by month/filter/pricing hash.
3. UI payload schema version + compatibility policy.
4. Orchestrate pipeline summary artifact for machine consumers (in addition to reconcile summary).
