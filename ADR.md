# ADR — tokenledger Architecture Decision Records

**Last Updated**: 2026-03-26

---

## ADR-001: Rust Core for Aggregation Engine

- **Status**: Accepted
- **Context**: The initial prototype used Python scripting. For operators ingesting tens to hundreds of thousands of events, Python's per-record overhead and GIL made live audits noticeably slow. The target is sub-second for <=100k events.
- **Decision**: Build the analytics core, cost engine, and CLI in Rust (2021 edition) using async Tokio for I/O-bound ingest and synchronous tight loops for CPU-bound aggregation.
- **Alternatives Considered**:
  - Python + pandas: fast for batch but slow for streaming; large binary.
  - TypeScript/Bun: acceptable speed but harder to distribute as a zero-dep binary.
- **Consequences**:
  - Higher throughput and lower latency than scripted alternatives.
  - Single statically-linked binary; no runtime dependency on Python/Node.
  - Provider adapter bridges can still be polyglot (the ingest adapter emits JSONL, the Rust core consumes it).
- **Implementation**: `Cargo.toml` (package name `tokenledger`), `src/main.rs`, `src/cost.rs`, `src/analytics.rs`

---

## ADR-002: Canonical Normalized Event Contract (JSONL)

- **Status**: Accepted
- **Context**: Claude Code, OpenAI, Gemini, Cursor, and Copilot each expose different log schemas. A cost engine that understands every raw format directly would be brittle and hard to extend.
- **Decision**: Define a single canonical `UsageEvent` schema (provider, model, session_id, timestamp, token breakdown). All provider-specific parsing is isolated in adapter modules under `src/ingest/adapters.rs`. Adapters emit canonical JSONL; the cost engine consumes only canonical JSONL.
- **Alternatives Considered**:
  - Native per-provider parsers in the cost engine: coupling, hard to add providers.
  - Protobuf binary format: efficient but harder to inspect and debug.
- **Consequences**:
  - New provider support = one new adapter; zero changes to cost engine.
  - JSONL format is human-readable and debuggable with standard tools.
  - `IngestSummary` provides per-adapter observability.
- **Implementation**: `src/models.rs::UsageEvent`, `src/ingest/adapters.rs`, `src/ingest/parser.rs`

---

## ADR-003: Blended Cost Model (Variable + Subscription Allocation)

- **Status**: Accepted
- **Context**: Pure token-variable-cost metrics understate real spend for providers with mandatory subscriptions (Copilot $19/month, Cursor Pro $20/month, Claude Pro $20/month). An operator comparing providers on $/MTok alone would get a misleading picture.
- **Decision**: Monthly total = variable token cost + subscription cost allocated proportionally to token usage weight within the provider. `subscription_allocated = sub_usd * (event_tokens / provider_monthly_tokens)`. Blended $/MTok = monthly_total / total_mtok.
- **Alternatives Considered**:
  - Separate subscription line item only (not allocated): harder to blend across providers.
  - Uniform per-session subscription split: unfair to sessions with large token counts.
- **Consequences**:
  - Fair cross-provider comparison including fixed costs.
  - Subscription allocation assumption is explicit and auditable in output.
  - `CostBreakdown` always exposes both `variable_cost_usd` and `subscription_allocated_usd` separately.
- **Implementation**: `src/cost.rs::allocate_subscription`, `src/cost.rs::compute_costs`, `src/models.rs::CostBreakdown`

---

## ADR-004: Pricing Book as Operator-Managed JSON File with Reconcile Workflow

- **Status**: Accepted
- **Context**: Provider pricing changes frequently. Automating pricing fetches from provider websites is fragile and may violate ToS. Operators need a simple, auditable record of what rates were used for cost attribution.
- **Decision**: Pricing is stored as a versioned JSON file (`pricing.json`) edited by operators. The `pricing-reconcile` command auto-generates stub entries for missing models and applies them, with mandatory human review of the stub rates before they are non-zero. A `meta` block tracks source and age.
- **Alternatives Considered**:
  - Auto-fetch from provider pricing APIs: brittle, requires auth, may break on API changes.
  - Embedded hardcoded rate table: cannot be updated without recompiling.
  - External database: overkill for a CLI tool; complicates deployment.
- **Consequences**:
  - Pricing book is a first-class artifact committed to source control alongside usage data.
  - `pricing-audit` enforces that the book is not stale (`--max-age-days` default 30).
  - Stub rates (0.0) are intentionally obvious in `pricing-lint` output to force operator action.
- **Implementation**: `src/models.rs::PricingBook`, `src/pricing.rs::execute_pricing_reconcile`, `src/pricing.rs::execute_pricing_audit`

---

## ADR-005: Alias Resolution for Provider and Model Name Variants

- **Status**: Accepted
- **Context**: AI providers use inconsistent model naming: `claude-3-5-sonnet-20241022`, `claude-3-5-sonnet`, `anthropic/claude-3-5-sonnet` may all refer to the same billing rate. Without aliases, every model string variant requires a separate pricing entry.
- **Decision**: The pricing book supports `provider_aliases` (top-level) and `model_aliases` (per-provider). During normalization, all events are resolved to canonical names before rate lookup. Reconcile and coverage commands suggest aliases based on fuzzy string matching.
- **Alternatives Considered**:
  - Regex patterns in the pricing book: powerful but complex to author and audit.
  - Normalize at ingest time: pushes business logic into adapters; harder to update without re-ingesting.
- **Consequences**:
  - One canonical entry per model covers many variant strings.
  - `coverage` output includes `suggested_provider_aliases` and `suggested_model_aliases_by_provider` to speed up operator alias authoring.
  - Alias resolution is deterministic and unit-testable.
- **Implementation**: `src/models.rs::ProviderPricing::model_aliases`, `src/models.rs::PricingBook::provider_aliases`, `src/utils.rs::normalize_events`

---

## ADR-006: Orchestration Pipeline with Aggregate Cache

- **Status**: Accepted
- **Context**: Running ingest + pricing-reconcile + monthly + daily + bench sequentially on every invocation is expensive (seconds for large datasets). Operators running the pipeline frequently (e.g., in CI, in a dashboard refresh loop) need fast incremental results.
- **Decision**: The `orchestrate` command manages an aggregate cache keyed on a content-addressable fingerprint of (month_filter, providers, models, on_unpriced, pricing_hash, events_fingerprint). Cache hits skip the aggregation stages entirely and return persisted `CostBreakdown` and `DailyReport` JSON. Cache is invalidated when any input changes.
- **Alternatives Considered**:
  - mtime-based cache: simpler but brittle when files are touched without content change.
  - No cache: simpler implementation; unacceptable latency for large datasets at dashboard refresh rates.
- **Consequences**:
  - Near-instant repeat invocations when inputs have not changed.
  - Cache invalidation is explicit and auditable (`invalidate_count` in pipeline summary).
  - Cache files are human-readable JSON.
- **Implementation**: `src/orchestrate.rs`, `src/models.rs::OrchestrateAggregateCache`, `src/models.rs::OrchestrateAggregateCacheKey`

---

## ADR-007: Performance Gate as First-Class CI Artifact

- **Status**: Accepted
- **Context**: The cost engine is a hot path. Refactors that improve code clarity but regress throughput are unacceptable for production. Without a programmatic gate, regressions are caught only by noticing slow queries.
- **Decision**: Bench scenarios and perf gate thresholds are defined in a config file (`configs/perf-gates.json`). The `bench` command measures throughput; the `orchestrate` pipeline includes a perf-gate stage that fails non-zero when thresholds are violated. Trend reports accumulate historical runs.
- **Alternatives Considered**:
  - Manual timing only: not machine-enforceable in CI.
  - External profiling tool only: no regression detection built into the binary.
- **Consequences**:
  - CI can fail on throughput regression automatically.
  - Correctness assertions in bench output prevent silent numerical errors from optimization changes.
  - Trend reports provide a historical performance record.
- **Implementation**: `src/bench.rs`, `src/benchmarks/`, `src/models.rs::PerfGateConfig`, `configs/`

---

## ADR-008: CLI-First Design with Machine-Readable JSON Output

- **Status**: Accepted
- **Context**: tokenledger is a backend analytics tool consumed by operators and CI pipelines, not primarily by end-users in a GUI. All commands must be scriptable and composable.
- **Decision**: All subcommands support `--output json` for fully machine-parseable pretty-printed JSON. Exit codes are deterministic: 0 = pass, non-zero = failure. Structured summary artifacts are written to configurable paths for downstream consumption (e.g., AgilePlus dashboard `UiSnapshot`).
- **Alternatives Considered**:
  - HTTP API server: heavier deployment model; not needed for local-first use.
  - Binary/protobuf output: not human-readable; harder to debug.
- **Consequences**:
  - Dashboard integrations consume `UiSnapshot` JSON without invoking the CLI at runtime.
  - CI pipelines gate on exit codes without parsing output.
  - All output formats (table, json, markdown) are implemented in `src/format.rs`.
- **Implementation**: `src/cli.rs`, `src/format.rs`, `src/models.rs::UiSnapshot`
