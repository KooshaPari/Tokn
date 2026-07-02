# Merged Fragmented Markdown

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

# Consolidated Index

## Files

* `ADAPTER_BASE_LIBS_AND_FORK_STRATEGY_2026-02-21.md`
* `DEEP_RESEARCH_AND_OPTIMIZATION_PLAN.md`
* `INDEX.md`
* `UNIFIED_E2E_RESEARCH_AND_ARCHITECTURE_2026-02-21.md`
* `USAGE_AUDIT_AND_MARKET_SCAN.md`

## Subdirectories


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
