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
