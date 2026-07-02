# Merged Fragmented Markdown

## Source: contracts/NORMALIZED_EVENT_SCHEMA_CONTRACT_V1.md

# Normalized Event Schema Contract v1

Status: Active  
Contract version: `1`

## Scope

This contract defines the normalized JSONL event shape used by `monthly`, `daily`, `coverage`, `pricing-check`, `pricing-reconcile`, `bench`, and `orchestrate`.

## Artifact form

- Format: JSON Lines (`.jsonl`)
- Encoding: UTF-8
- One JSON object per line

## Required event shape (v1)

```json
{
  "provider": "claude",
  "model": "claude-sonnet-4-5",
  "session_id": "abc",
  "timestamp": "2026-02-19T22:12:00Z",
  "usage": {
    "input_tokens": 1200,
    "output_tokens": 800,
    "cache_write_tokens": 400,
    "cache_read_tokens": 3200,
    "tool_input_tokens": 0,
    "tool_output_tokens": 0
  }
}
```

## Field contract

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `provider` | string | yes | Provider key (alias values are allowed on input; canonicalized against pricing aliases before filtering/costing). |
| `model` | string | yes | Model key (alias values are allowed on input; canonicalized against provider model aliases before filtering/costing). |
| `session_id` | string | yes | Logical session identifier used in session counts and dedupe keys. |
| `timestamp` | RFC3339 datetime string | yes | Must parse as UTC datetime (`DateTime<Utc>`). |
| `usage` | object | yes | Token usage payload (see subfields below). |
| `usage.input_tokens` | non-negative integer | yes | Input prompt tokens. |
| `usage.output_tokens` | non-negative integer | yes | Output/completion tokens. |
| `usage.cache_write_tokens` | non-negative integer | yes | Cache write tokens. |
| `usage.cache_read_tokens` | non-negative integer | yes | Cache read tokens. |
| `usage.tool_input_tokens` | non-negative integer | yes | Tool input tokens. |
| `usage.tool_output_tokens` | non-negative integer | yes | Tool output tokens. |

## Invariants

1. `usage_total_tokens` is defined as:
`input_tokens + output_tokens + cache_write_tokens + cache_read_tokens + tool_input_tokens + tool_output_tokens`.
2. Consumers must treat the payload as append-only for unknown keys (ignore unknown fields).
3. Empty event sets after month/provider/model filters are treated as an execution error for report/snapshot generation.

## Compatibility policy

1. `v1` is additive-forward-compatible:
- adding optional fields is non-breaking,
- adding top-level metadata fields is non-breaking.
2. Breaking changes require `v2`:
- renaming/removing required fields,
- changing numeric semantics,
- changing timestamp format away from RFC3339 UTC.

## Producer/consumer responsibilities

1. Producers (ingest adapters) should emit the exact required keys above.
2. Consumers should not rely on field order.
3. Downstream UI/extensions should consume normalized artifacts, not provider-native raw logs.


---


## Source: contracts/UI_SNAPSHOT_SCHEMA_CONTRACT_V1.md

# UI Snapshot Schema Contract v1

Status: Active  
Contract version: `1`

## Scope

This contract defines the JSON payload written by:

`tokenledger orchestrate --ui-snapshot-path <path>`

It is intended for file-based extension and statusbar integrations (CodexBar/OpenCode style).

## Artifact form

- Format: JSON (`.json`)
- Encoding: UTF-8
- Producer: `orchestrate` command
- Current `schema_version`: `1`

## Required shape (v1)

```json
{
  "schema_version": 1,
  "generated_at": "2026-02-21T02:00:00Z",
  "month": "2026-02",
  "mode": "compact",
  "totals": {
    "cost_usd": 2.0,
    "tokens": 200000,
    "blended_usd_per_mtok": 10.0,
    "session_count": 3,
    "skipped_unpriced_count": 1
  },
  "top_providers": [
    {
      "name": "provider-a",
      "tokens": 120000,
      "total_cost_usd": 1.4,
      "blended_usd_per_mtok": 11.67,
      "session_count": 2
    }
  ],
  "top_models": [
    {
      "name": "model-a",
      "tokens": 150000,
      "total_cost_usd": 1.6,
      "blended_usd_per_mtok": 10.67,
      "session_count": 2
    }
  ],
  "suggestions": [
    "tip"
  ],
  "reconcile_latest_summary_path": "benchmarks/results/reconcile-latest-summary.json"
}
```

## Field contract

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `schema_version` | integer | yes | Compatibility gate for consumers. |
| `generated_at` | RFC3339 datetime string | yes | Snapshot generation timestamp (UTC). |
| `month` | string | yes | Snapshot month in `YYYY-MM`. |
| `mode` | enum | yes | `compact` or `extended`. |
| `totals` | object | yes | Aggregate cost/token/session metrics. |
| `top_providers` | array | yes | Provider-level rows sorted by token volume. |
| `top_models` | array | yes | Model-level rows sorted by token volume. |
| `suggestions` | array of string | yes | Optimization suggestions from report pipeline. |
| `reconcile_latest_summary_path` | string | no | Present when latest reconcile summary pointer exists. |

`totals` fields:
- `cost_usd` (number)
- `tokens` (non-negative integer)
- `blended_usd_per_mtok` (number)
- `session_count` (non-negative integer)
- `skipped_unpriced_count` (non-negative integer)

Row fields in `top_providers[]` and `top_models[]`:
- `name` (string)
- `tokens` (non-negative integer)
- `total_cost_usd` (number)
- `blended_usd_per_mtok` (number)
- `session_count` (non-negative integer)

## Mode semantics

1. `compact`:
- `top_providers` and `top_models` are top-N slices (currently N=5).
2. `extended`:
- `top_providers` and `top_models` include full breakdowns.

## Compatibility policy

1. Consumers must hard-check `schema_version == 1` before strict field assumptions.
2. Additive fields in `v1` are non-breaking; consumers must ignore unknown keys.
3. `mode` additions are non-breaking if existing values and field semantics are preserved.
4. Any of the following requires a major contract bump (`schema_version: 2`):
- required field removal/rename,
- type change for existing fields,
- semantic redefinition of totals or row metrics.

## Consumer guidance

1. Prefer fail-open behavior for missing file/temporary parse errors (show stale/empty UI state).
2. Use atomic read strategy (`readFile` + parse; retry on parse error).
3. If `schema_version` is unsupported, surface a clear integration error and ignore payload contents.


---


## Source: governance/PIPELINE_GOVERNANCE_AND_ARTIFACT_POLICY.md

# Pipeline Governance and Artifact Policy

## Policy Goals

1. Every pipeline run should leave auditable machine-readable artifacts.
2. Artifact paths should support both immutable history and easy latest lookup.
3. Mutating pricing workflows must remain explicit and reviewable.

## Required Artifacts

1. Ingest summary:
- `--summary-json-path`

2. Reconcile artifacts:
- per-run directory `reconcile-YYYYMMDD-HHMMSS/`
- `pricing-patch.reconcile.json`
- `unpriced-events.reconcile.jsonl`
- `reconcile-summary.json`
- rolling pointer: `reconcile-latest-summary.json`

3. Bench artifacts:
- timestamped `bench-*.json`
- rolling `latest-summary.json`

4. UI artifact (optional):
- `ui-snapshot.json`

5. Cache artifacts (optional but recommended for operator runs):
- ingest cache metadata: `--ingest-cache-path`
- aggregate cache entries: `--aggregate-cache-path`

## Safety Defaults

1. `task do:all:next` and `task orchestrate` run pricing reconcile in dry-run mode.
2. Static artifact mode must be explicit (`--pricing-reconcile-static-artifacts`).
3. Mutating pricing writes require explicit flags.

## Retention Guidance

1. Keep rolling pointers for integrations.
2. Keep timestamped directories for audit and diffing.
3. Periodically archive old run artifacts when volume grows.

## Compliance Checks

1. Artifact presence check after pipeline run.
2. Schema check for summary JSON payloads.
3. Pricing audit freshness and source provenance checks.
4. Perf and correctness gate checks.

## Cache Key and Metrics Semantics

### Ingest cache key (`--ingest-cache-path`)

The ingest cache entry is reusable only when all fields match:

1. provider set
2. `since` value
3. `limit` value
4. output path (`--events-out`)
5. source file `mtime` map collected from provider adapters

Operational behavior:

1. `hit`: ingest is skipped and existing `--events-out` file is reused.
2. `miss`: ingest runs and cache metadata is overwritten with the new key.
3. `invalidate`: represented as a miss caused by any key mismatch or missing output file.

### Aggregate cache key (`--aggregate-cache-path`)

Each aggregate cache entry is keyed by:

1. selector:
- `month`
- provider/model filters
- `on_unpriced` mode
2. pricing hash (from `--pricing` content)
3. events fingerprint (from `--events-out` content/metadata)

Operational behavior:

1. `hit`: monthly and daily reports are reused from cache with no recompute.
2. `miss`: no selector entry exists; monthly/daily recompute and cache write occur.
3. `invalidate`: selector entry exists but pricing hash or events fingerprint changed; stale entry is replaced after recompute.

### Pipeline summary metrics (`aggregate_cache`)

`orchestrate --pipeline-summary-path` records aggregate cache metrics:

1. `hit_count`
2. `miss_count`
3. `invalidate_count`
4. `enabled`
5. `cache_path`


---


## Source: index.md

# Consolidated Index

## Files


## Subdirectories

* `contracts/index.md`
* `contracts/merged.md`
* `governance/index.md`
* `governance/merged.md`
* `integrations/index.md`
* `integrations/merged.md`
* `plans/index.md`
* `plans/merged.md`
* `research/index.md`
* `research/merged.md`
* `worklog/index.md`
* `worklog/merged.md`


---


## Source: integrations/EXTENSION_FILE_READ_INTEGRATION_EXAMPLES.md

# Extension Integration Examples (File-Based Reads)

## Purpose

Provide practical integration examples for CodexBar/OpenCode-style consumers that read `tokenledger` artifacts from disk (no service dependency).

## Producer command

```bash
cargo run -- orchestrate \
  --providers codex \
  --limit 500 \
  --events-out ./artifacts/ingested.sample.jsonl \
  --state-file ./benchmarks/results/ingest-state.json \
  --summary-json-path ./benchmarks/results/ingest-summary.json \
  --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json \
  --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json \
  --pricing-reconcile-dry-run \
  --ui-snapshot-path ./benchmarks/results/ui-snapshot.json
```

## Files to read

1. Primary UI payload:
- `./benchmarks/results/ui-snapshot.json`
2. Optional reconcile details pointer:
- `reconcile_latest_summary_path` inside snapshot
3. Optional pipeline status:
- `./benchmarks/results/orchestrate-summary.json`

## Example A: Polling reader (TypeScript/Node)

```ts
import { readFile } from "node:fs/promises";

type UiSnapshotV1 = {
  schema_version: 1;
  generated_at: string;
  month: string;
  mode: "compact" | "extended";
  totals: {
    cost_usd: number;
    tokens: number;
    blended_usd_per_mtok: number;
    session_count: number;
    skipped_unpriced_count: number;
  };
  top_providers: Array<{
    name: string;
    tokens: number;
    total_cost_usd: number;
    blended_usd_per_mtok: number;
    session_count: number;
  }>;
  top_models: Array<{
    name: string;
    tokens: number;
    total_cost_usd: number;
    blended_usd_per_mtok: number;
    session_count: number;
  }>;
  suggestions: string[];
  reconcile_latest_summary_path?: string;
};

async function loadSnapshot(path: string): Promise<UiSnapshotV1 | null> {
  try {
    const raw = await readFile(path, "utf8");
    const parsed = JSON.parse(raw);
    if (parsed?.schema_version !== 1) return null;
    return parsed as UiSnapshotV1;
  } catch {
    return null;
  }
}

setInterval(async () => {
  const snapshot = await loadSnapshot("./benchmarks/results/ui-snapshot.json");
  if (!snapshot) return;
  console.log(snapshot.month, snapshot.totals.cost_usd, snapshot.top_models[0]?.name);
}, 2000);
```

## Example B: fs-watch reader with debounce (TypeScript/Node)

```ts
import { watch } from "node:fs";
import { readFile } from "node:fs/promises";

const path = "./benchmarks/results/ui-snapshot.json";
let timer: NodeJS.Timeout | undefined;

async function refresh() {
  try {
    const payload = JSON.parse(await readFile(path, "utf8"));
    if (payload?.schema_version !== 1) return;
    // render/update extension UI state
  } catch {
    // ignore transient partial-write/parse errors
  }
}

watch(path, () => {
  if (timer) clearTimeout(timer);
  timer = setTimeout(() => void refresh(), 120);
});
```

## Integration checklist

1. Validate `schema_version` before consuming fields.
2. Ignore unknown keys (forward compatibility).
3. Handle missing file and parse errors without crashing extension process.
4. Keep reads local-only and low frequency (1-5s polling) unless fs-watch is available.
5. Optionally read `reconcile_latest_summary_path` when present for detail views.


---


## Source: plans/INDEX.md

# Plans Index

- `docs/plans/UNIFIED_E2E_EXECUTION_PLAN.md`
- `docs/plans/UNIFIED_MODEL_PROVIDER_LEDGER_PLAN_2026-02-21.md`


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


## Source: research/ADAPTER_BASE_LIBS_AND_FORK_STRATEGY_2026-02-21.md

# Adapter Base Libs and Fork Strategy (2026-02-21)

## Decision

Do not fork a single monolithic Rust "ccusage-equivalent" that attempts to own Claude + Codex + Cursor + Droid end-to-end.

Adopt a split strategy:

1. Keep `tokenledger` as the canonical core (normalized schema, pricing, reconciliation, formulas, benchmark gates, UI snapshot artifacts).
2. Use provider-specific adapters as thin import/fork units per source.
3. Prefer upstream library import first; if missing, fork only the parser/adapter slice (not UI/CLI monoliths).

## Why this is the right architecture

1. Provider surfaces differ materially (auth/session, log shape, local storage format, CLI semantics).
2. A monolith fork couples unrelated churn and slows update velocity.
3. Per-provider adapters preserve blast radius and make provider updates incremental.
4. `tokenledger` remains the stable integration point for cross-provider blended cost analytics.

## Web market scan summary (practical starting points)

### Aggregators / UX references

1. OpenUsage (plugin-style multi-provider tracker): `https://www.openusage.ai/`
2. CodexBar (menu bar + multi-provider integrations): `https://github.com/steipete/CodexBar`
3. opencode (UI/UX reference for low-overhead coding workflows): `https://github.com/sst/opencode`

### Provider-oriented usage parsers / trackers

1. ccusage (existing baseline behavior and UX to match/improve): `https://github.com/ryoppippi/ccusage`
2. ccstat (Rust Claude usage/cost parsing): `https://crates.io/crates/ccstat`
3. cxusage (Go Codex usage parser/reporting): `https://github.com/johanneserhardt/cxusage`
4. tokscale (Rust multi-provider usage/cost tracker): `https://github.com/junhoyeo/tokscale`

Inference: useful pieces exist, but there is no single mature Rust monolith that robustly covers Claude + Codex + Cursor + Droid together with high-quality adapter depth and stable governance. Split adapters are still the lower-risk path.

## Adapter contract (required)

Each provider adapter must emit normalized JSONL records directly consumable by `tokenledger`:

1. `event_ts`
2. `provider`
3. `model_raw`
4. `model_canonical`
5. `session_id`
6. `request_id` (if available)
7. `input_tokens`
8. `output_tokens`
9. `cache_write_tokens` / `cache_read_tokens` (if provider exposes)
10. `tool_tokens` / tool-usage metadata (if provider exposes)
11. provenance (`source_path`, `offset`, parser version)

## Fork/import policy per provider

1. Import upstream parser/lib when:
- data contract is stable,
- dependency/license is acceptable,
- integration surface is small.

2. Fork adapter slice when:
- parser quality is good but tightly coupled to its own CLI/UI,
- only a narrow extraction layer is needed.

3. Re-implement in-house when:
- upstream churn is high or project is abandonware,
- licensing blocks intended distribution,
- parser logic is smaller than integration overhead.

## Governance for imported/forked adapter slices

1. Pin every external import/fork to explicit commit/tag.
2. Keep one adapter directory per provider with:
- source attribution,
- patch log,
- update script/checklist,
- fixture corpus.
3. Run adapter conformance tests into normalized golden JSONL.
4. Enforce bench gates on:
- cold backfill,
- warm tail,
- burst ingest.
5. Treat model-provider mapping and pricing mapping as separate audited layers.

## Performance-first implementation guidance

### SQLite / local state

1. `rusqlite` as default embedded checkpoint/index DB.
2. WAL mode + deterministic migrations.
3. Optional read-heavy pooling wrapper where license/governance permits.

### Disk cache

1. Add optional disk-backed cache layer for:
- normalized-event dedupe indexes,
- monthly aggregate materializations,
- provider model metadata snapshots.

2. Keep cache optional and bounded; no mandatory overhead for minimal runs.

### Tail/parse pipeline

1. Provider adapters should support both backfill and incremental tail.
2. Use append-safe checkpoints (`source file + inode/hash + offset`).
3. Ensure deterministic resume after restarts.

## Pricing methodology alignment (subscription + token-based providers)

1. Token-priced providers:
- direct `USD / Mtok` using configured input/output/cache/tool rates.

2. Subscription/session-priced providers:
- derive effective monthly budget:
  - either explicit user-entered monthly USD value, or
  - programmatic derivation from plan price and billing metadata.
- allocate monthly budget proportionally over measured token usage.
- compute effective blended `USD / Mtok` per model and provider.

3. Keep both in one comparable surface:
- `effective_cost_usd`
- `effective_usd_per_mtok`
- `blended_provider_usd_per_mtok`
- `blended_global_usd_per_mtok`

4. Mark derivation method in output metadata:
- `pricing_mode = token|subscription_derived|subscription_manual`

## Concrete next steps (execution order)

1. Build real adapters:
- `~/.claude`
- `~/.codex`
- Cursor DB/logs
- Droid logs

2. Emit normalized JSONL directly to `tokenledger ingest`.
3. Add fixture-backed conformance tests for each adapter.
4. Add benchmark harness perf gates and enforce in CI:
- cold backfill,
- warm tail,
- burst.

5. Add low-overhead UI integration path (statusbar/extension) using `--ui-snapshot-path`.
6. Add optional SQLite + disk-cache acceleration behind feature flags/config.

## Explicit conclusion

Yes, separate provider-focused imports/forks are preferred.

No, do not fork a full monolith.

Keep `tokenledger` as the core, and treat adapters as replaceable, benchmarked, governed modules.


---


## Source: research/DEEP_RESEARCH_AND_OPTIMIZATION_PLAN.md

# Deep Research and Optimization Plan

Date: 2026-02-20

## 1) Ecosystem Scan

### 1.1 ccusage Family (strongest baseline for multi-provider usage tooling)

- Core project: [ccusage](https://github.com/ryoppippi/ccusage)
- Codex integration package: [@ccusage/codex](https://github.com/ccusage/codex)
- MCP adapter package: [@ccusage/mcp](https://github.com/ccusage/mcp)

Patterns to adopt:

- Provider adapters isolated from aggregation/reporting logic.
- Multiple ingestion paths (CLI, local files, MCP) converging into one normalized schema.
- Cache-friendly read patterns for repeated queries.

### 1.2 CodexBar Pattern (status-first UX)

- Project: [codexbar](https://github.com/azat-io/codexbar)

Patterns to adopt:

- Real-time, low-latency snapshot rendering over full-history recalculation.
- Small-footprint local state, optimized for frequent refresh.
- User-visible “live now” metrics with background historical rollups.

### 1.3 OpenCode Pattern (model/provider ops surface)

- Project: [OpenCode](https://github.com/sst/opencode)

Patterns to adopt:

- Provider/model abstraction layer with consistent capabilities metadata.
- Runtime observability (request/response/error/accounting paths) as first-class concerns.
- Separation between control-plane config and data-plane events.

### 1.4 Rust/Go Performance Architecture Patterns

Rust references:

- Async runtime model: [Tokio](https://tokio.rs/)
- Ergonomic async services: [Axum](https://github.com/tokio-rs/axum)
- Streaming RPC option: [Tonic](https://github.com/hyperium/tonic)
- Rust benchmarking: [Criterion.rs](https://github.com/bheisler/criterion.rs)

Go references:

- Context propagation/cancellation: [context package](https://pkg.go.dev/context)
- Profiling primitives: [runtime/pprof](https://pkg.go.dev/runtime/pprof)
- Benchmarking primitives: [testing package benchmarks](https://pkg.go.dev/testing)

Practical architecture takeaway:

- Rust for high-throughput ingest + aggregation hot path (predictable latency, memory control).
- Go for lightweight services where rapid iteration and strong profiling/tooling matter.
- Avoid per-event allocations in hot loops; prefer pooled buffers, batch writes, and bounded queues.

## 2) Pricing and Data Source Strategy

Primary official pricing sources:

- OpenAI API pricing: [openai.com/api/pricing](https://openai.com/api/pricing/)
- Anthropic pricing: [anthropic.com/pricing#api](https://www.anthropic.com/pricing#api)
- Google Gemini API pricing: [ai.google.dev/gemini-api/docs/pricing](https://ai.google.dev/gemini-api/docs/pricing)
- OpenRouter model pricing: [openrouter.ai/models](https://openrouter.ai/models)

Recommended strategy:

- Maintain a versioned pricing registry (daily refresh + manual override).
- Track source confidence per price point (`official`, `aggregator`, `inferred`).
- Resolve costs at event-time against effective-dated price tables.
- Keep raw token/unit counts immutable; recompute cost views when pricing changes.

Data source hierarchy:

1. Official provider APIs/docs (authoritative).
2. Aggregators (OpenRouter/model catalogs) for discovery and cross-checking.
3. Local inferred mappings only as fallback, always flagged.

## 3) Real-Time Architecture Options

### Option A: Single-process local-first stream engine

- Ingest tailers + parser workers + in-memory aggregators + periodic snapshot persistence.
- Best for desktop/single-node workflows with minimal ops burden.

### Option B: Split ingest and query services

- Ingest service appends normalized events to log store; query service serves low-latency reads.
- Better isolation under heavy scan/parse load; easier horizontal scale.

### Option C: Event bus fan-out (future)

- Ingest publishes normalized events to bus (NATS/Kafka style), multiple consumers for billing/alerts/UX.
- Best long-term extensibility, highest operational complexity.

Recommendation now:

- Start with Option A + strict interface boundaries so Option B migration is incremental.

## 4) Benchmark Methodology

Workloads:

- Cold backfill scan: large historical datasets.
- Warm incremental tail: steady stream ingestion.
- Burst mode: short, high-rate event spikes.
- Mixed provider mode: heterogeneous schemas and pricing maps.

KPIs:

- Throughput (events/sec)
- p50/p95/p99 ingest-to-visible latency
- Peak RSS memory
- CPU per 10k events
- Correctness drift (cost/token deltas vs golden fixtures)

Method:

- Use deterministic fixture corpus + replay harness.
- Run A/B baselines for each optimization (single-variable changes).
- Capture profiles each run (`pprof` for Go paths, Rust profiler + allocation stats).
- Enforce regression gates in CI for latency/memory/correctness thresholds.

## 5) Prioritized Roadmap and Optimization Levers

### Phase 0 (Immediate: highest ROI)

- Introduce normalized event contract and adapter conformance tests.
- Add pricing registry with effective dates and provenance tags.
- Add replay benchmark harness with golden outputs.

Optimization levers:

- Batch parsing and batched storage writes.
- Deduplicate repeated metadata lookups via bounded caches.
- Precompile regex/parsers and remove dynamic parser construction in hot paths.

### Phase 1 (Near-term: performance hardening)

- Parallelize ingestion pipeline with bounded worker pools.
- Implement incremental materialized aggregates for “current session” views.
- Add structured telemetry for queue depth, parse failures, and lag.

Optimization levers:

- Zero-copy parsing where feasible.
- Arena/pool allocation strategy for transient objects.
- Adaptive backpressure (drop/defer non-critical enrichments first).

### Phase 2 (Scale path)

- Split ingest/query planes when single-process p95 latency cannot hold target.
- Add pluggable storage backends (embedded local + remote analytical store).
- Add real-time push channel (SSE/WebSocket) for UI updates.

Optimization levers:

- Log-structured append path + compaction for historical windows.
- Tiered aggregation (minute/hour/day) to cap query cost.
- Cost recomputation engine decoupled from ingest path.

## 6) Decision Checklist

- Is every provider adapter outputting the same normalized schema?
- Can costs be recomputed for any historical window after pricing updates?
- Are p95 latency and memory bounded under mixed-provider burst workloads?
- Can we migrate from local-first to split-service without parser rewrites?


---


## Source: research/INDEX.md

# Research Index

- `docs/research/USAGE_AUDIT_AND_MARKET_SCAN.md`
- `docs/research/DEEP_RESEARCH_AND_OPTIMIZATION_PLAN.md`
- `docs/research/UNIFIED_E2E_RESEARCH_AND_ARCHITECTURE_2026-02-21.md`
- `docs/research/ADAPTER_BASE_LIBS_AND_FORK_STRATEGY_2026-02-21.md`


---


## Source: research/UNIFIED_E2E_RESEARCH_AND_ARCHITECTURE_2026-02-21.md

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


---


## Source: research/USAGE_AUDIT_AND_MARKET_SCAN.md

# Usage Audit and Market Scan

Date: 2026-02-20

## Local Findings (kush)

- `usage/` already contains a broad ccusage-derived implementation with Python migration work and hybrid Python bridge support.
- `usage/usage-main/apps/ccusage` includes provider loaders for Claude/Codex/Cursor/Droid and gRPC/subprocess bridges.
- Current pain point is real-time performance in heavy multi-provider scans, especially with large file/database traversals.

## External Tools to Learn From

- `ccusage`: mature usage CLI and ecosystem (`ccusage`, `@ccusage/codex`, `@ccusage/mcp`).
- `OpenCode`: model/provider usage tracking patterns and model management UX.
- `CodexBar`: status-bar-first UX for agent usage visibility.

## Architecture Direction

- Keep provider scraping adapters separate from analytics core.
- Use Rust for streaming ingest + aggregation path.
- Support normalized events so existing local collectors can pipe data into core immediately.


---


## Source: worklog/INDEX.md

# Worklog Index

- `docs/worklog/WORKLOG_2026-02-21_E2E_PLAN_AND_UI_INTEGRATION.md`
- `docs/worklog/WORKLOG_2026-02-21_ADAPTER_FORK_STRATEGY_AND_WEB_RESEARCH.md`
- `docs/worklog/WORKLOG_2026-02-21_MODEL_DB_SEED_NORMALIZATION.md`
- `docs/worklog/WORKLOG_2026-02-21_UNIFIED_MODEL_PROVIDER_LEDGER_AND_CLIPROXYAPI_FEEDER.md`


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
