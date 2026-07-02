# PRD — tokenledger

**Product**: tokenledger
**Version**: 0.1.0
**Stack**: Rust 2021 (async/Tokio), Clap v4 CLI, serde_json, reqwest, chrono
**Last Updated**: 2026-03-26

---

## Problem

Teams running multiple AI coding agents (Claude Code, Cursor, GitHub Copilot, Codex, Gemini) accumulate token and session usage across providers with incompatible log formats. Existing tooling is:

- Fragmented: each provider has its own log schema and no cross-provider view.
- Incomplete: subscription costs (e.g., $20/month Copilot) are invisible in token-only metrics.
- Slow: scripted approaches are too slow for live audits at tens of thousands of events.
- Ungovernanced: no automated check that all events have a matching pricing entry.

---

## Goals

1. Provide a single monthly economic view across all AI providers and models.
2. Include both variable token cost and amortized subscription cost in a blended model.
3. Surface blended `$/MTok` per model and provider with actionable optimization suggestions.
4. Enable pricing governance through check, reconcile, lint, and audit CLI subcommands.
5. Support an orchestration pipeline that produces machine-readable JSON for downstream dashboards.
6. Provide sub-second throughput for <=100k events; single-digit seconds for ~1M events.

---

## Non-Goals (v0.1)

1. Web UI / HTTP API server.
2. Billing API write-back to provider billing portals.
3. Real-time streaming ingest from live provider APIs.
4. Multi-user RBAC or tenant isolation.
5. Automatic pricing data fetch from provider pricing APIs (manual reconcile only).

---

## Users

| User | Context | Primary Need |
|------|---------|-------------|
| Operator | Runs 5–20 concurrent coding agents | Cross-provider cost summary |
| Team Lead | Controls model spend across team | Per-model $/MTok and session counts |
| FinOps / Infra Engineer | Audits usage policy and pricing data | Pricing governance, staleness checks |
| CI/CD Pipeline | Automated quality gate | Structured JSON output, exit codes |

---

## Epics and User Stories

### E1 — Event Ingestion

**E1.1** As an operator, I want to ingest raw usage events from Claude Code JSONL logs so that session-level token counts are normalized into a canonical `UsageEvent` format.

**E1.2** As an operator, I want deduplication of ingested events (keyed by provider + session_id + timestamp_millis + model + token_total) so that re-runs on the same logs do not double-count sessions.

**E1.3** As an operator, I want incremental ingest with mtime-based cache so that only newly-modified source files are re-read on subsequent runs.

**E1.4** As an operator, I want multi-provider ingest (Anthropic Claude Code, OpenAI, Gemini, Cursor, Copilot) so that all AI coding tool usage is captured in a single ledger.

**E1.5** As an operator, I want an ingest summary JSON with per-provider stats (scanned, emitted, skipped), deduped_total, and duration_ms so that I can verify ingest completeness.

**Acceptance Criteria E1**:
- `tokenledger ingest` writes a canonical JSONL file of `UsageEvent` records.
- Each `UsageEvent` contains: provider, model, session_id, timestamp (UTC), and token breakdown (input, output, cache_write, cache_read, tool_input, tool_output).
- Duplicate events (same dedupe key) are silently dropped; count reported in summary.
- Incremental mode reads cached mtime map; skips sources with unchanged mtime.
- `IngestSummary` JSON is written to configured path with all required fields.

---

### E2 — Pricing Governance

**E2.1** As an operator, I want a versioned JSON pricing book mapping provider -> model -> per-million-token rates (input, output, cache write, cache read, tool input, tool output) plus an optional subscription_usd_month.

**E2.2** As an operator, I want provider-level and model-level alias maps in the pricing book so that variant model strings resolve to canonical rates.

**E2.3** As an operator, I want `pricing-check` to scan all events in a month, report which provider:model pairs are missing from the pricing book, and fail loudly unless `--allow-unpriced` is set.

**E2.4** As an operator, I want `pricing-reconcile` to auto-generate a patch JSON from unpriced events, apply it to the pricing book, then re-check coverage — with `--dry-run` and `--write-backup` flags.

**E2.5** As an operator, I want `pricing-apply` to merge a manually-edited patch file into the pricing book with explicit `--dry-run`, `--write-backup`, and `--allow-overwrite-model-rates` controls.

**E2.6** As an operator, I want `pricing-lint` to detect placeholder (all-zero) rates in the pricing book and fail the command unless `--allow-placeholders` is set.

**E2.7** As an operator, I want `pricing-audit` to check that the pricing book `meta` block contains a valid RFC3339 `updated_at` timestamp and a non-empty `source` field, and to fail if the book is older than `--max-age-days`.

**Acceptance Criteria E2**:
- `pricing-check` exits 0 when all events are priced; exits non-zero otherwise (unless `--allow-unpriced`).
- `pricing-reconcile` writes `pricing-patch.reconcile.json` and `unpriced-events.reconcile.jsonl` to workdir, applies patch, re-checks, reports `PricingReconcileSummary`.
- `pricing-lint` emits `placeholder_violations` array; exits non-zero on non-empty unless `--allow-placeholders`.
- `pricing-audit` emits `age_days`, `stale`, `violations`, `warnings`; exits non-zero on any unwaived violation.
- Patch files use stub rates (0.0) and are human-editable before applying.

---

### E3 — Cost Aggregation

**E3.1** As an operator, I want `monthly` to produce a `CostBreakdown` for a YYYY-MM month including: variable_cost_usd, subscription_allocated_usd, monthly_total_usd, blended_usd_per_mtok, all six token category counts, session_count, skipped_unpriced_count, per-provider breakdown, per-model breakdown.

**E3.2** As an operator, I want `daily` to produce a `DailyReport` with a per-calendar-day `CostBreakdown` so I can identify high-spend days within a month.

**E3.3** As an operator, I want subscription costs allocated proportionally by token usage weight within each provider so that fixed monthly subscriptions are fairly distributed across sessions.

**E3.4** As an operator, I want `--provider` and `--model` filter flags so I can scope aggregations to a specific subset.

**E3.5** As an operator, I want `--on-unpriced error|skip|warn` to control how unpriced events are handled during aggregation without a separate pricing-check step.

**Acceptance Criteria E3**:
- `monthly` output satisfies: `monthly_total_usd = variable_cost_usd + subscription_allocated_usd`.
- `blended_usd_per_mtok = monthly_total_usd / total_mtok` (rounded to 4 decimal places).
- Subscription allocation per event: `provider_sub_usd * (event_tokens / provider_total_tokens)`.
- `daily` output: array of `DailyEntry { day: "YYYY-MM-DD", breakdown: CostBreakdown }`.
- Filter flags correctly exclude non-matching provider/model events before aggregation.
- `--on-unpriced error` (default) fails the command when any unpriced event exists.

---

### E4 — Coverage Reporting

**E4.1** As an operator, I want `coverage` to report which provider:model pairs in the event file are missing from the pricing book, with fuzzy alias suggestions, so I can patch the pricing book proactively.

**Acceptance Criteria E4**:
- Coverage report contains: `month`, `totals {events, tokens}`, `priced_count`, `unpriced_count`, `missing_providers`, `missing_models_by_provider`, `suggested_provider_aliases`, `suggested_model_aliases_by_provider`.
- Alias suggestions are computed by substring / edit-distance matching against known canonical names.
- `--write-patch` flag writes a ready-to-apply patch JSON from coverage findings.
- `--write-unpriced-events` flag writes a JSONL of only the unpriced events.

---

### E5 — Orchestration Pipeline

**E5.1** As an operator, I want `orchestrate` to execute the full pipeline (ingest -> pricing-reconcile -> monthly -> daily -> bench -> perf-gate -> ui-snapshot) in a single command with per-stage skip flags and a structured `OrchestratePipelineSummary` JSON.

**E5.2** As an operator, I want an aggregate cache keyed on (month_filter, providers, models, on_unpriced, pricing_hash, events_fingerprint) so that unchanged runs return cached breakdowns instantly.

**E5.3** As an operator, I want `orchestrate` to emit a `UiSnapshot` JSON at a configurable path so that dashboards (AgilePlus) can consume pre-computed totals without re-running the pipeline.

**Acceptance Criteria E5**:
- `OrchestratePipelineSummary` is written to configured artifacts path with all stage summaries, `schema_version`, `generated_at`, `duration_ms`.
- Aggregate cache hit is indicated in summary; invalidation occurs when pricing hash or events fingerprint changes.
- `UiSnapshot` contains: `schema_version`, `generated_at`, `month`, `mode`, `totals {cost_usd, tokens, blended_usd_per_mtok, session_count, skipped_unpriced_count}`, `top_providers`, `top_models`, `suggestions`.
- Stage-skip flags (`--skip-ingest`, `--skip-pricing-reconcile`) suppress respective pipeline stages.

---

### E6 — Performance Benchmarking and Perf Gate

**E6.1** As a developer, I want `bench` to measure throughput (events/sec) and elapsed time for aggregation scenarios so that regressions are detected before merge.

**E6.2** As a developer, I want per-scenario perf gate thresholds (max_ms, min_events_per_sec, max_elapsed_regression_pct, max_eps_drop_pct) in a config file so that the pipeline fails CI if throughput drops below acceptable limits.

**E6.3** As a developer, I want bench trend reports aggregating multiple runs to provide p50/p95 percentile latencies and EPS per scenario.

**E6.4** As a developer, I want correctness assertions in bench output (expected cost totals, token counts) so that optimization changes cannot silently break numerical accuracy.

**Acceptance Criteria E6**:
- `bench` emits `BenchReport` with per-scenario: elapsed_ms, events_per_sec, optional correctness block.
- Perf gate compares latest bench result against config thresholds; exits non-zero on regression.
- Trend report emits `p50_elapsed_ms`, `p95_elapsed_ms`, `median_events_per_sec` per scenario.
- Correctness block values match expected totals within float tolerance (1e-6).

---

### E7 — Output Formats

**E7.1** As an operator, I want `--output table` for human-readable aligned terminal output, `--output json` for machine-readable pretty-printed JSON, and `--output markdown` for embedding in reports.

**Acceptance Criteria E7**:
- All query commands (`monthly`, `daily`, `coverage`) support `--output table|json|markdown`.
- JSON output is pretty-printed, fully machine-parseable, with no trailing commas.
- Table output aligns numeric columns right-justified, with currency formatted to 4 decimal places.
- Markdown output renders as a valid GitHub-flavored Markdown table.

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Monthly report latency | <1s for <=100k events; <10s for ~1M events |
| Pricing coverage | 100% of events priced after reconcile |
| CLI exit codes | Deterministic: 0=pass, non-zero=fail for all commands |
| Blended cost accuracy | Within 0.01% of manual calculation |
| Ingest deduplication | 0 duplicate events in output for identical input |
