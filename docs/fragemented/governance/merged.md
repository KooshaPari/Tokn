# Merged Fragmented Markdown

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
