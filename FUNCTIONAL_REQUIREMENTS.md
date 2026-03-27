# Functional Requirements — tokenledger

**ID Format**: FR-{CAT}-{NNN}
**Traces To**: PRD.md epics E1–E7
**Last Updated**: 2026-03-26

---

## FR-ING — Ingestion

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-ING-001 | System SHALL ingest normalized events from JSONL files and directories recursively. | E1.1 | `src/ingest/mod.rs`, `src/ingest/aggregation.rs` |
| FR-ING-002 | System SHALL normalize each raw provider log record into a canonical `UsageEvent` with fields: provider (string), model (string), session_id (string), timestamp (UTC RFC3339), and token breakdown (input, output, cache_write, cache_read, tool_input, tool_output as u64). | E1.1 | `src/models.rs::UsageEvent`, `src/ingest/parser.rs` |
| FR-ING-003 | System SHALL deduplicate ingested events using a composite key of (provider, session_id, timestamp_millis, model, token_total). Duplicate records SHALL be silently dropped and their count reported in the summary. | E1.2 | `src/models.rs::IngestDedupeKey`, `src/ingest/aggregation.rs` |
| FR-ING-004 | System SHALL support incremental ingest with a mtime-based cache. On subsequent runs with the same cache path, source files whose mtime has not changed SHALL be skipped. | E1.3 | `src/orchestrate.rs::OrchestrateIngestCache` |
| FR-ING-005 | System SHALL support at minimum the following provider adapters: Anthropic Claude Code, OpenAI, Gemini. Additional adapters SHALL follow the `IngestProvider` enum extension pattern. | E1.4 | `src/ingest/adapters.rs`, `src/cli.rs::IngestProvider` |
| FR-ING-006 | System SHALL emit an `IngestSummary` JSON containing: per-provider stats (scanned, emitted, skipped), incremental_sources_skipped, emitted_total, deduped_total, output path, started_at, finished_at, duration_ms. | E1.5 | `src/models.rs::IngestSummary` |
| FR-ING-007 | System SHALL apply `--since <RFC3339>` and `--limit <N>` filters during ingest to restrict the event window. | E1.1 | `src/models.rs::IngestEmitCtx` |

---

## FR-PRICE — Pricing Book and Governance

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-PRICE-001 | System SHALL load a pricing book from a JSON file conforming to the `PricingBook` schema: `{providers: {<name>: {subscription_usd_month, models: {<name>: ModelRate}, model_aliases}}, provider_aliases, meta}`. | E2.1 | `src/models.rs::PricingBook`, `src/utils.rs::load_pricing` |
| FR-PRICE-002 | Each `ModelRate` SHALL contain: input_usd_per_mtok (f64), output_usd_per_mtok (f64), and optional cache_write, cache_read, tool_input, tool_output fields (all per million tokens). | E2.1 | `src/models.rs::ModelRate` |
| FR-PRICE-003 | System SHALL resolve provider aliases: if an event's provider string matches a key in `provider_aliases`, the canonical provider name SHALL be used for rate lookup. | E2.2 | `src/utils.rs::normalize_events` |
| FR-PRICE-004 | System SHALL resolve model aliases within a resolved provider: if a model string matches a key in `model_aliases`, the canonical model name SHALL be used for rate lookup. | E2.2 | `src/utils.rs::normalize_events` |
| FR-PRICE-005 | `pricing-check` SHALL scan all events for a month, report priced_count and unpriced_count, emit the list of missing provider:model pairs, and exit non-zero when unpriced_count > 0 unless `--allow-unpriced` is set. | E2.3 | `src/pricing.rs::run_pricing_check` |
| FR-PRICE-006 | `pricing-reconcile` SHALL: (1) generate a patch JSON listing missing providers and models with stub rates, (2) apply the patch to the pricing book file, (3) re-check coverage, (4) write a `PricingReconcileSummary` to workdir artifacts. | E2.4 | `src/pricing.rs::execute_pricing_reconcile` |
| FR-PRICE-007 | `pricing-reconcile` SHALL support `--dry-run` (patch computed and applied in-memory only, no file writes) and `--write-backup` (pricing file backed up before overwrite). | E2.4 | `src/pricing.rs::execute_pricing_reconcile` |
| FR-PRICE-008 | `pricing-apply` SHALL merge a patch JSON into the pricing book. By default it SHALL not overwrite existing model rates; `--allow-overwrite-model-rates` SHALL enable overwriting. | E2.5 | `src/pricing.rs::run_pricing_apply` |
| FR-PRICE-009 | `pricing-lint` SHALL detect model entries with both input_usd_per_mtok and output_usd_per_mtok equal to 0.0 as placeholder violations. It SHALL exit non-zero when violations exist unless `--allow-placeholders`. | E2.6 | `src/pricing.rs::run_pricing_lint` |
| FR-PRICE-010 | `pricing-audit` SHALL verify: (1) `meta` block exists, (2) `meta.source` is a non-empty string, (3) `meta.updated_at` is a valid RFC3339 timestamp, (4) age of `updated_at` does not exceed `--max-age-days`. It SHALL exit non-zero on any unwaived violation. | E2.7 | `src/pricing.rs::execute_pricing_audit` |
| FR-PRICE-011 | `pricing-audit` SHALL support `--allow-stale` to downgrade staleness from violation to warning, and `--allow-missing-source` to downgrade missing source from violation to warning. | E2.7 | `src/pricing.rs::execute_pricing_audit` |

---

## FR-COST — Cost Computation

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-COST-001 | System SHALL compute variable token cost per event as: `sum_over_token_types(token_count * rate_per_mtok / 1_000_000)` for each applicable token category in the model's `ModelRate`. | E3.1 | `src/cost.rs::calc_variable_cost` |
| FR-COST-002 | System SHALL allocate provider subscription cost per event proportionally: `subscription_usd_month * (event_tokens / provider_total_tokens_for_month)`. | E3.3 | `src/cost.rs::allocate_subscription` |
| FR-COST-003 | System SHALL compute monthly total cost as: `variable_cost_usd + subscription_allocated_usd`. | E3.1 | `src/cost.rs::compute_costs` |
| FR-COST-004 | System SHALL compute blended cost per million tokens: `monthly_total_usd / total_mtok`, rounded to 4 decimal places. | E3.1 | `src/cost.rs::compute_costs`, `src/format.rs::round4` |
| FR-COST-005 | System SHALL produce per-provider and per-model `NamedMetric` breakdowns within the `CostBreakdown`, each including tokens, mtok, variable_cost_usd, subscription_allocated_usd, total_cost_usd, blended_usd_per_mtok, session_count, tool_share. | E3.1 | `src/analytics.rs`, `src/cost.rs` |
| FR-COST-006 | System SHALL count unique sessions using a hash-based `HashSet` on session identifiers; session_count SHALL reflect deduplicated unique sessions. | E3.1 | `src/models.rs::Acc::sessions` |
| FR-COST-007 | System SHALL support `--on-unpriced error|skip|warn` to control behavior when events reference missing pricing entries. Default is `error`. | E3.5 | `src/cli.rs::OnUnpricedAction`, `src/cost.rs::compute_costs` |

---

## FR-TOK — Token Reporting

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-TOK-001 | System SHALL report all six token category counts in every `CostBreakdown`: input_tokens, output_tokens, cache_write_tokens, cache_read_tokens, tool_input_tokens, tool_output_tokens. | E3.1 | `src/models.rs::CostBreakdown` |
| FR-TOK-002 | System SHALL compute total_tokens as the sum of all six token categories per event. | E3.1 | `src/models.rs::TokenUsage::total` |
| FR-TOK-003 | System SHALL compute tool_share per dimension as `(tool_input_tokens + tool_output_tokens) / total_tokens`. | E3.1 | `src/models.rs::NamedMetric::tool_share` |

---

## FR-SES — Session Reporting

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-SES-001 | System SHALL report unique monthly session counts globally, per provider, and per model in every aggregation output. | E3.1 | `src/models.rs::Acc::sessions` |
| FR-SES-002 | Session identity for deduplication SHALL be based on a hash of the session_id string. | E3.1 | `src/cost.rs`, `src/models.rs::Acc` |

---

## FR-RPT — Reporting Commands

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-RPT-001 | `monthly` command SHALL accept `--events <path>...`, `--pricing <path>`, `--month <YYYY-MM>`, `--provider <name>...`, `--model <name>...`, `--output table|json|markdown`, `--on-unpriced`. | E3.1 | `src/cli.rs::MonthlyArgs` |
| FR-RPT-002 | `daily` command SHALL accept the same base args as `monthly` and produce a `DailyReport` with a `DailyEntry` per calendar day within the month. | E3.2 | `src/cli.rs::DailyArgs`, `src/models.rs::DailyReport` |
| FR-RPT-003 | `coverage` command SHALL produce a `CoverageReport` and support `--write-patch` and `--write-unpriced-events` output flags. | E4.1 | `src/cli.rs::CoverageArgs`, `src/pricing.rs::build_coverage_report` |

---

## FR-OUT — Output Formats

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-OUT-001 | System SHALL support `--output table` rendering with right-aligned numeric columns and currency formatted to 4 decimal places. | E7.1 | `src/format.rs` |
| FR-OUT-002 | System SHALL support `--output json` rendering as pretty-printed, valid JSON with no trailing commas. | E7.1 | `src/format.rs` |
| FR-OUT-003 | System SHALL support `--output markdown` rendering as a valid GitHub-flavored Markdown table. | E7.1 | `src/format.rs` |

---

## FR-ORCH — Orchestration

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-ORCH-001 | `orchestrate` command SHALL execute: ingest -> pricing-reconcile -> monthly -> daily -> bench -> perf-gate -> ui-snapshot as a sequential pipeline. | E5.1 | `src/orchestrate.rs::run_orchestrate` |
| FR-ORCH-002 | `orchestrate` SHALL support per-stage skip flags: `--skip-ingest`, `--skip-pricing-reconcile`. | E5.1 | `src/cli.rs::OrchestrateArgs` |
| FR-ORCH-003 | `orchestrate` SHALL emit an `OrchestratePipelineSummary` JSON with `schema_version`, `generated_at`, `duration_ms`, per-stage summaries. | E5.1 | `src/models.rs::OrchestratePipelineSummary` |
| FR-ORCH-004 | `orchestrate` SHALL maintain an aggregate cache keyed on (month_filter, providers, models, on_unpriced, pricing_hash, events_fingerprint). Cache hits SHALL skip monthly/daily recomputation. | E5.2 | `src/orchestrate.rs`, `src/models.rs::OrchestrateAggregateCache` |
| FR-ORCH-005 | `orchestrate` SHALL emit a `UiSnapshot` JSON at a configurable path upon completion. | E5.3 | `src/models.rs::UiSnapshot`, `src/orchestrate.rs` |

---

## FR-BENCH — Benchmarking and Perf Gate

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-BENCH-001 | `bench` command SHALL measure elapsed_ms and events_per_sec for each configured scenario. | E6.1 | `src/bench.rs::execute_bench` |
| FR-BENCH-002 | `bench` SHALL produce a `BenchReport` with per-scenario results including optional correctness assertions. | E6.1 | `src/models.rs::BenchReport`, `src/models.rs::BenchScenarioCorrectness` |
| FR-BENCH-003 | Perf gate config SHALL define per-scenario thresholds: max_ms, min_events_per_sec, optional max_elapsed_regression_pct, max_eps_drop_pct. | E6.2 | `src/models.rs::PerfGateConfig`, `src/models.rs::PerfGateThreshold` |
| FR-BENCH-004 | Perf gate SHALL fail (exit non-zero) when latest bench result exceeds a threshold, unless `require_baseline_for_regression_checks` is true and no baseline exists. | E6.2 | `src/bench.rs` |
| FR-BENCH-005 | Trend report SHALL aggregate bench results from a configured directory and emit p50, p95 latency and median EPS per scenario. | E6.3 | `src/models.rs::BenchTrendReport`, `src/benchmarks/` |
| FR-BENCH-006 | Correctness assertions in bench output SHALL match expected totals within a tolerance of 1e-6 relative difference. | E6.4 | `src/bench.rs` |

---

## FR-TIP — Optimization Suggestions

| ID | Requirement | PRD Trace | Implementation |
|----|-------------|-----------|----------------|
| FR-TIP-001 | System SHALL generate optimization tips based on measured cost signals (e.g., high cache miss rate, high tool token share, expensive model dominance) and include them in the `suggestions` field of `CostBreakdown` and `UiSnapshot`. | E3.1, E5.3 | `src/analytics.rs` |
