<<<<<<< HEAD
<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->
![Downloads](https://img.shields.io/github/downloads/KooshaPari/Tokn/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/Tokn?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/Tokn?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.
<!-- AI-DD-META:END -->
> **Work state:** ACTIVE · **Progress:** `█████████░ 80%` · **Workspace version:** `0.1.5` (next tag `v0.1.5`; local tags through `v0.1.4`; not published to crates.io)
> Token-ledger / usage accounting (tokenledger) **and the canonical Phenotype Rust ROUTING substrate** — `tokenledger::routing` (hexagonal: pareto_router/ports/adapters). Consumed by OmniRoute per ADR-001. · updated 2026-07-18
>
> **Release prep (T1):** see [`docs/guides/cutting-a-release.md`](docs/guides/cutting-a-release.md) for exact tag / `gh release` commands.

![Build Status](https://github.com/KooshaPari/Tokn/actions/workflows/ci.yml/badge.svg)
![Security Audit](https://github.com/KooshaPari/Tokn/actions/workflows/audit.yml/badge.svg)
![Policy Compliance](https://github.com/KooshaPari/Tokn/actions/workflows/deny.yml/badge.svg)

# Tokn (tokenledger)

**Status:** alpha · **crate versions:** `0.1.5` (workspace)

Enterprise-grade token management and pricing governance system for AI coding agents.

This repository works with Claude and other AI agents as autonomous software engineers.

## Install

```bash
cargo install --path crates/tokenledger --locked
```

Installs the `tokenledger` binary from package `tokenledger-rs`. No crates.io publish required.

## Workspace

This is a Rust workspace with two main crates:

- **tokenledger** (`crates/tokenledger`, package `tokenledger-rs`) — Enterprise-grade token management and pricing governance system. Provides unified token and cost tracking across multiple AI provider APIs with optimization recommendations.
- **pareto-rs** (`crates/pareto-rs`) — Pareto-optimal cost engine for AI coding agents. Delivers cost optimization and resource allocation algorithms for multi-provider agent orchestration.

## Quick Start

```bash
# Install CLI (from repo root)
cargo install --path crates/tokenledger --locked

# Development
cargo run -p tokenledger-rs

# Testing
cargo test --workspace

# Linting
cargo clippy --workspace
```

## Environment

```bash
# Required environment variables
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-..."
```

---

## Development Philosophy

### Extend, Never Duplicate

- NEVER create a v2 file. Refactor the original.
- NEVER create a new class if an existing one can be made generic.
- NEVER create custom implementations when an OSS library exists.
- Before writing ANY new code: search the codebase for existing patterns.

### Primitives First

- Build generic building blocks before application logic.
- A provider interface + registry is better than N isolated classes.
- Template strings > hardcoded messages. Config-driven > code-driven.

### Research Before Implementing

- Check crates.io for existing libraries.
- Search GitHub for 80%+ implementations to fork/adapt.

---

## Library Preferences (DO NOT REINVENT)

| Need | Use | NOT |
|------|-----|-----|
| Async runtime | tokio | custom async |
| HTTP client | reqwest | custom wrappers |
| Logging | tracing | print() or log::logger |
| CLI | clap | manual arg parsing |
| Validation | validator | manual if/else |
| Database | sqlx | raw SQL strings |
| Rate limiting | governor | custom rate limiter |

---

## Code Quality Non-Negotiables

- Zero new lint suppressions without inline justification
- All new code must pass: cargo clippy, cargo fmt, tests
- Max function: 40 lines
- No placeholder TODOs in committed code
- All Rust code must be clippy-clean

---

## Verifiable Constraints

| Metric | Threshold | Enforcement |
|--------|-----------|-------------|
| Test coverage | >= 80% | cargo-tarpaulin |
| Security findings | 0 high/critical | cargo-audit |
| Clippy warnings | 0 | CI gate |

---

## Domain-Specific Patterns

### What tokenledger Is

tokenledger is a **token and cost tracking system** for AI coding agents. The core domain is: provide unified token and cost tracking across multiple providers with optimization recommendations.

### Key Interfaces

| Interface | Responsibility |
|-----------|---------------|
| CLI commands | report, costs, optimize |
| Provider traits | Multi-provider abstraction |
| Storage | SQLite, PostgreSQL support |

---

## Integration

### With thegent

```python
# thegent config
llm:
  provider: cliproxy
  base_url: http://localhost:8317/v1
```

### With agentapi

```bash
agentapi --cliproxy http://localhost:8317
```

---

## Governance & Development

**Project Name**: tokenledger (code name: Tokn)  
**AgilePlus Tracking**: All work tracked in `/repos/AgilePlus`. Review `CLAUDE.md` for development policies.

**Quality Standards**:
- **Zero new lint suppressions** without inline justification
- **80% test coverage minimum** (cargo-tarpaulin)
- **Zero high/critical security findings** (cargo-audit)
- **Max 40 lines per function**, zero placeholder TODOs

**Quick Commands**:
```bash
cargo install --path crates/tokenledger --locked   # Install CLI
cargo build --workspace                            # Development build
cargo test --workspace                             # Test suite
cargo clippy --workspace                           # Lint check
cargo audit                                        # Security scan
cargo tarpaulin                                    # Coverage report
```

## Integration & Adoption

**With thegent**: Configure as LLM provider proxy for agent token routing and cost optimization.

**With agentapi**: Use as cost-tracking backend for multi-agent orchestration across provider networks.

**Extensibility**: Implement `Provider` trait to add new token/cost models.

## Related Phenotype Projects

- **[Sidekick](../Sidekick)** — Agent dispatch & routing
- **[cheap-llm-mcp](../cheap-llm-mcp)** — Cost-optimized LLM routing
- **[AgilePlus](../AgilePlus)** — Specification hub
- **[thegent](../thegent)** — Agent execution framework

## License

MIT License - see LICENSE file

**Status**: Active development (agent provider expansion)  
**Maintained by**: Phenotype Org  
**Last Updated**: 2026-07-18

## Cutting a release

See **[`docs/guides/cutting-a-release.md`](docs/guides/cutting-a-release.md)** for the
operator checklist (annotate tag `v0.1.5`, push tag, verify `gh release` assets). Do not
create the tag until `main` holds the release commit.

## Documentation

This repository includes the following cross-cutting documents:

- [`AGENTS.md`](AGENTS.md) — operating instructions for AI agents and human contributors
- [`SPEC.md`](SPEC.md) — formal specification of behavior and contracts
- [`ARCHITECTURE.md`](ARCHITECTURE.md) — system architecture and component overview
- [`docs/`](docs/) — design notes, ADRs, and supporting documentation (see [`docs/index.md`](docs/index.md))
- [`docs/guides/cutting-a-release.md`](docs/guides/cutting-a-release.md) — how to cut `v0.1.5`

=======
# tokenledger

Fast usage/cost analytics for `cursor`, `droid`, `codex`, `claude`, and `proxyapi` with blended subscription + token economics.

## Why

Existing ccusage-style paths are feature-rich but can be slow for real-time, cross-provider audits. `tokenledger` is a Rust-first aggregation core intended to:

- stream large JSONL inputs quickly,
- normalize provider/session/model usage,
- compute monthly blended costs (`subscription + variable token costs`),
- expose per-model and per-provider `$ / MTok` with practical optimization suggestions.

## Current Scope

- Monthly report command with:
  - token breakdown (input/output/cache/tool)
  - variable token cost
  - subscription allocation
  - blended `$ / MTok`
  - session counts
  - per-provider + per-model blended economics
  - suggestion engine based on tool/cache/cost signals
- Daily report command with per-day breakdowns plus monthly totals
- Normalized JSONL event ingestion from files/directories

## Event Schema (JSONL)

Each line:

```json
{"provider":"claude","model":"claude-sonnet-4-5","session_id":"abc","timestamp":"2026-02-19T22:12:00Z","usage":{"input_tokens":1200,"output_tokens":800,"cache_write_tokens":400,"cache_read_tokens":3200,"tool_input_tokens":0,"tool_output_tokens":0}}
```

Provider/model values are normalized to canonical pricing keys before filtering and cost aggregation. Alias values in events are supported via pricing config.

## Contracts (v1)

- Normalized event schema contract:
  - `docs/contracts/NORMALIZED_EVENT_SCHEMA_CONTRACT_V1.md`
- UI snapshot schema contract + compatibility policy:
  - `docs/contracts/UI_SNAPSHOT_SCHEMA_CONTRACT_V1.md`
- Extension integration examples (CodexBar/OpenCode-style file reads):
  - `docs/integrations/EXTENSION_FILE_READ_INTEGRATION_EXAMPLES.md`

## Pricing Schema (aliases)

`pricing.example.json` supports:

- top-level `provider_aliases`: `{ "<alias>": "<canonical_provider>" }`
- per-provider `model_aliases`: `{ "<alias>": "<canonical_model>" }`

All alias targets must exist in `providers` / `providers.<provider>.models`.

## Usage

```bash
cd tokenledger

# Monthly table output
cargo run -- monthly --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02

# Monthly markdown output with filters and top-N breakdowns
cargo run -- monthly --events ./examples/events.jsonl --pricing ./pricing.example.json --output markdown --provider claude --provider codex --model claude-sonnet-4-5 --top-providers 3 --top-models 5

# Monthly report that skips unpriced events instead of failing
cargo run -- monthly --events ./examples/events.jsonl --pricing ./pricing.example.json --on-unpriced skip

# Alias filters are accepted too (provider/model aliases map to canonical names)
cargo run -- monthly --events ./examples/events.jsonl --pricing ./pricing.example.json --output markdown --provider anthropic --model sonnet

# Daily report (per-day + month totals)
cargo run -- daily --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --output table

# Daily JSON output
cargo run -- daily --events ./examples/events.jsonl --pricing ./pricing.example.json --output json

# Coverage report for pricing gaps (table)
cargo run -- coverage --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02

# Coverage report for automation (json)
cargo run -- coverage --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --json-output

# Coverage report + artifact generation for pricing updates
cargo run -- coverage --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --write-patch ./benchmarks/results/pricing-patch.json --write-unpriced-events ./benchmarks/results/unpriced-events.jsonl --json-output

# Dry-run pricing patch merge (summary only; no file writes)
cargo run -- pricing-apply --pricing ./pricing.example.json --patch ./benchmarks/results/pricing-patch.json --dry-run

# Apply pricing patch and write backup first
cargo run -- pricing-apply --pricing ./pricing.example.json --patch ./benchmarks/results/pricing-patch.json --write-backup

# CI pricing check (fails if any unpriced events are present)
cargo run -- pricing-check --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02

# CI pricing check but do not fail build on unpriced rows
cargo run -- pricing-check --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --allow-unpriced

# Lint pricing for placeholder model rates + alias integrity
cargo run -- pricing-lint --pricing ./pricing.example.json

# Lint pricing but allow placeholder rates temporarily
cargo run -- pricing-lint --pricing ./pricing.example.json --allow-placeholders

# Audit pricing metadata freshness and source provenance (strict mode)
cargo run -- pricing-audit --pricing ./pricing.example.json

# Audit with explicit staleness threshold and machine-readable output
cargo run -- pricing-audit --pricing ./pricing.example.json --max-age-days 14 --json-output

# Allow stale metadata and missing source metadata during controlled rollout windows
cargo run -- pricing-audit --pricing ./pricing.example.json --allow-stale --allow-missing-source

# Reconcile pricing lifecycle in one in-process pipeline:
# coverage -> patch/unpriced artifacts -> pricing-apply -> pricing-check
cargo run -- pricing-reconcile --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --workdir ./benchmarks/results

# Reconcile with dry-run apply and backup/write controls preserved
cargo run -- pricing-reconcile --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --dry-run --write-backup --allow-overwrite-model-rates

# Ingest from all local adapters and write normalized JSONL
cargo run -- ingest --output ./artifacts/local-events.jsonl

# Ingest only Claude + Codex records since a timestamp with a hard cap
cargo run -- ingest --provider claude --provider codex --since 2026-02-01T00:00:00Z --limit 20000 --output ./artifacts/claude-codex-since.jsonl

# Ingest incrementally with checkpointing and append mode
cargo run -- ingest --provider codex --output ./artifacts/codex-events.jsonl --append --state-file ./artifacts/ingest-state.json --incremental

# Ingest and emit structured machine-readable summary stats
cargo run -- ingest --provider codex --output ./artifacts/codex-events.jsonl --summary-json-path ./artifacts/ingest-summary.json

# Ingest with request-level dedupe for this run
cargo run -- ingest --provider codex --output ./artifacts/codex-events.jsonl --dedupe-by-request --summary-json-path ./artifacts/ingest-summary.json

# Benchmark all scenarios with JSON output
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --json-output

# Benchmark while skipping unpriced events
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --on-unpriced skip

# Benchmark and write JSON report directly to a file
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --json-output-path ./benchmarks/results/latest-summary.json

# Benchmark and persist history: timestamped report + latest-summary.json
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --record --label ci-main

# Benchmark against a baseline report and include deltas/regression flags
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --baseline ./benchmarks/results/latest-summary.json --json-output

# Benchmark correctness against a golden fixture (ignores elapsed-time fields)
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --golden ./benchmarks/fixtures/bench-golden.json --golden-epsilon 0.0001 --json-output

# Aggregate historical benchmark reports into per-scenario trend stats
cargo run -- bench --trend-dir ./benchmarks/results

# Trend mode with regression failure against perf-gates thresholds
cargo run -- bench --trend-dir ./benchmarks/results --trend-fail-on-regression

# Run performance gate thresholds
./scripts/perf_gate.sh

# Run performance gates with baseline-aware regression checks
./scripts/perf_gate.sh ./benchmarks/results/latest-summary.json
# or
PERF_BASELINE=./benchmarks/results/latest-summary.json ./scripts/perf_gate.sh

# Strict perf gate mode: fail when regression thresholds are configured but baseline is missing
PERF_STRICT=1 PERF_BASELINE=./benchmarks/results/latest-summary.json ./scripts/perf_gate.sh
# or
./scripts/perf_gate.sh --strict ./benchmarks/results/latest-summary.json

# Perf gate + benchmark golden correctness validation
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --golden ./benchmarks/fixtures/bench-golden.json --golden-epsilon 0.0001 --json-output > /tmp/tokenledger-bench-golden.json
./scripts/perf_gate.sh

# Run the full next-step pipeline
task do:all:next

# Run pipeline directly inside tokenledger (no Taskfile/shell orchestration)
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --pricing-reconcile-dry-run

# Orchestrate pipeline and skip unpriced events across monthly/daily/bench
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --on-unpriced skip

# Orchestrate with pre-flight pricing lint before monthly/daily/bench
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --pricing-lint

# Orchestrate with strict pricing metadata audit before monthly/daily/bench
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --pricing-audit --pricing-max-age-days 30

# Orchestrate with full pricing reconcile apply (writes pricing unless --pricing-reconcile-dry-run is set)
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --pricing-reconcile-write-backup

# Orchestrate with static reconcile artifact paths (disable per-run timestamped reconcile subdirectories)
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --pricing-reconcile-static-artifacts

# Orchestrate without pricing reconcile stage
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --skip-pricing-reconcile

# Orchestrate and emit compact menu/statusbar UI snapshot JSON
cargo run -- orchestrate --providers codex --limit 500 --events-out ./artifacts/ingested.sample.jsonl --state-file ./benchmarks/results/ingest-state.json --summary-json-path ./benchmarks/results/ingest-summary.json --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json --aggregate-cache-path ./benchmarks/results/orchestrate-aggregate-cache.json --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json --ui-snapshot-path ./benchmarks/results/ui-snapshot.json
```

## Benchmark Harness

`bench` reuses the same aggregation pipeline used by reporting commands and emits wall-time + throughput metrics:

- `cold-backfill`: includes JSONL load, normalize, month filter, and monthly cost aggregation.
- `warm-tail`: re-runs aggregation on the tail window (`--warm-tail-events`) for `--warm-iterations` loops.
- `burst`: aggregates each batch (`--burst-batch-events`) across the filtered event set.
- `all`: executes all scenarios in sequence.

Examples:

```bash
# Human-readable benchmark output
cargo run -- bench --scenario cold-backfill --events ./examples/events.jsonl --pricing ./pricing.example.json

# JSON report for automation/perf gates
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --json-output

# Write JSON report to a file (table output still printed unless --json-output is also set)
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --json-output-path ./benchmarks/results/latest-summary.json

# Write timestamped history report in benchmarks/results and update latest-summary.json
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --record --label nightly

# Compare to a prior report; output includes:
# elapsed_ms_delta, events_per_sec_delta, elapsed_regression, events_per_sec_regression
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --baseline ./benchmarks/results/latest-summary.json --json-output

# Validate elapsed-independent correctness metrics against a golden fixture
cargo run -- bench --scenario all --events ./examples/events.jsonl --pricing ./pricing.example.json --golden ./benchmarks/fixtures/bench-golden.json --golden-epsilon 0.0001 --json-output

# Summarize trend stats from historical JSON reports:
# latest, median, p95 elapsed_ms; latest + median events_per_sec; run_count (per scenario)
cargo run -- bench --trend-dir ./benchmarks/results --json-output

# Fail trend run when latest-vs-median regression exceeds benchmarks/perf-gates.json thresholds
cargo run -- bench --trend-dir ./benchmarks/results --trend-fail-on-regression
```

Performance gates are defined in `benchmarks/perf-gates.json`. The gate script runs `bench --scenario all --json-output` and fails when any scenario exceeds `max_ms` or drops below `min_events_per_sec`. When a baseline is supplied (via `./scripts/perf_gate.sh <baseline>` or `PERF_BASELINE=<baseline>`), it also enforces regression thresholds (`max_elapsed_regression_pct`, `max_eps_drop_pct`). Set `PERF_STRICT=1` (or `--strict`) to fail fast if baseline-backed regression checks are configured but no baseline path is provided. Golden correctness fixtures are enforced via `bench --golden <path> --golden-epsilon <eps>`, which compares elapsed-independent fields (`events_processed` + computed totals/costs).

## Notes

- `ingest` emits normalized `UsageEvent` JSONL and reports per-provider `scanned / emitted / skipped` counts.
- `ingest --append` appends JSONL records instead of truncating output.
- `ingest --state-file <path>` persists checkpoint JSON (`source_path -> last_modified_unix`).
- `ingest --incremental` skips unchanged sources where `mtime <= checkpoint`.
- `ingest --dedupe-by-request` deduplicates emitted events for this run by `(provider, session_id, timestamp, model, token totals)`.
- `ingest --summary-json-path <path>` writes structured ingest stats JSON (`providers.scanned/emitted/skipped`, `incremental_sources_skipped`, `emitted_total`, `deduped_total`, `output`, `started_at`, `finished_at`, `duration_ms`).
- `ingest` applies provider-specific normalization paths for Claude/Codex/Proxyapi/Cursor/Droid first, then falls back to generic key-based extraction for unknown record shapes.
- `monthly`, `daily`, `bench`, and `orchestrate` support `--on-unpriced <error|skip>` (default `error`).
- `coverage` reports pricing coverage and unknown provider/model mappings, including alias suggestions.
- `coverage --write-patch <path>` writes a machine-readable pricing template with:
  - `missing_providers` scaffolds (empty models + optional alias suggestions)
  - `missing_models_by_provider` placeholder rate objects
  - `suggested_aliases` for providers/models inferred from coverage analysis
  - `metadata` (`generated_at`, `source_events_count`, `month`)
- `coverage --write-unpriced-events <path>` writes canonicalized unpriced events as JSONL.
- `pricing-check` is a CI-focused coverage gate that exits non-zero when `unpriced_count > 0` (unless `--allow-unpriced` is set).
- `pricing-lint` validates alias integrity and flags placeholder model rates (`<= 0`) and placeholder-like alias/model markers.
- `pricing-lint --allow-placeholders` keeps validation output but does not fail the command on placeholder violations.
- `pricing-audit` validates pricing metadata policy from top-level `meta`:
  - requires `meta.updated_at` (RFC3339)
  - requires `meta.source` unless `--allow-missing-source`
  - fails on stale metadata (`age_days > --max-age-days`, default 30) unless `--allow-stale`
  - emits structured report fields: `pricing_path`, `checked_at`, `metadata_present`, `source_present`, `updated_at_present`, `age_days`, `stale`, `pass`, `violations`, `warnings`
- `pricing-apply --pricing <path> --patch <path>` merges patch suggestions into pricing and prints:
  - `providers_added`, `models_added`, `aliases_added`, `models_skipped_existing`
- `pricing-apply --dry-run` validates merge output + aliases without modifying pricing.
- `pricing-apply --write-backup` writes `<pricing>.bak.<timestamp>.json` before mutating.
- `pricing-apply --allow-overwrite-model-rates` allows replacing existing model rates and alias mappings.
- `pricing-reconcile` runs `coverage -> artifact write -> pricing-apply -> pricing-check` in-process and emits structured summary JSON with stage results and artifact paths.
- `pricing-reconcile` stamps pricing metadata when a non-dry-run write occurs:
  - updates `meta.updated_at` to current UTC
  - sets `meta.source` to `tokenledger:pricing-reconcile` only when source is missing/empty
  - includes `pricing_apply.metadata_updated` in summary output
- `pricing-reconcile --allow-unpriced` returns success even if post-apply pricing-check still reports unpriced events.
- `orchestrate` runs pricing reconcile (`coverage -> apply -> check`) before monthly/daily/bench unless `--skip-pricing-reconcile` is set.
- `orchestrate` writes reconcile artifacts under per-run timestamped subdirectories (for example `./benchmarks/results/reconcile-YYYYMMDD-HHMMSS`) by default.
- `orchestrate` also writes reconcile summaries to:
  - `<reconcile_workdir>/reconcile-summary.json`
  - `./benchmarks/results/reconcile-latest-summary.json` (when timestamped subdirectories are enabled)
- `orchestrate --pricing-reconcile-static-artifacts` writes reconcile artifacts directly to `--pricing-reconcile-workdir` without per-run subdirectories.
- `orchestrate --pricing-reconcile-dry-run` keeps reconcile non-mutating while still validating the pricing lifecycle.
- `orchestrate --pricing-reconcile-allow-unpriced` allows pipeline continuation even if reconcile still reports unpriced events.
- `orchestrate --pricing-lint` runs `pricing-lint` before monthly/daily/bench stages.
- `orchestrate --pricing-audit --pricing-max-age-days <n>` runs strict pricing metadata audit before monthly/daily/bench stages.
- `orchestrate --ingest-cache-path <path>` enables deterministic disk-backed ingest cache metadata; if provider sources/mtimes and ingest args match and `--events-out` already exists, ingest is skipped.
- `orchestrate --aggregate-cache-path <path>` enables disk-backed monthly/daily aggregate reuse keyed by:
  - selector: month + provider/model filters + `on_unpriced` mode
  - pricing hash (contents of `--pricing`)
  - events fingerprint (contents and metadata of `--events-out`)
- Aggregate cache metrics in pipeline summary (`aggregate_cache`) semantics:
  - `hit_count`: cache entry reused with no monthly/daily recompute
  - `miss_count`: no reusable entry found and fresh monthly/daily results computed
  - `invalidate_count`: selector matched but pricing hash or events fingerprint changed, forcing recompute and cache overwrite
- `orchestrate --pipeline-summary-path <path>` writes a structured end-to-end pipeline summary JSON (stage timings, skip/cache status, reconcile summary, bench/perf results).
- `orchestrate --ui-snapshot-path <path>` writes a compact JSON payload for menu/statusbar UIs with:
  - `generated_at`, `month`
  - `totals` (`cost_usd`, `tokens`, `blended_usd_per_mtok`, `session_count`, `skipped_unpriced_count`)
  - top providers/models, optimization suggestions
  - optional `reconcile_latest_summary_path` when `./benchmarks/results/reconcile-latest-summary.json` exists
- Adapter sources:
  - Claude: `~/.claude/projects/**/*.jsonl`
  - Codex: `~/.codex/sessions/**/*.jsonl`
  - Proxyapi: common CLIProxyAPI local telemetry/usage paths, including `~/.cliproxyapi`, `~/.proxyapi`, `~/.config/cliproxyapi`, `~/.config/proxyapi`, `~/.local/share/cliproxyapi`, `~/.local/share/proxyapi`, `~/.cache/cliproxyapi`, `~/Library/Application Support/CLIProxyAPI`, and `~/Library/Logs/CLIProxyAPI` (`.json/.jsonl/.ndjson/.log/.txt`)
  - Cursor: `~/.cursor` and `~/Library/Application Support/Cursor/workspaceStorage` (`.log/.json/.jsonl` plus best-effort `.sqlite/.sqlite3/.db` JSON extraction)
  - Droid: `~/.factory/sessions/**/*.json` and `~/.factory/sessions/**/*.jsonl`

## Model Database Seed (CSV + SQL)

`models.csv` is treated as a raw text baseline (markdown-style pipe table), not as a strict CSV parser input.

Generate normalized artifacts:

```bash
python3 ./scripts/build_model_seed.py \
  --input ./models.csv \
  --csv-out ./models_normalized.csv \
  --sql-out ./models_schema_seed.sql
```

Artifacts:
- `models_normalized.csv`: valid long-format CSV, one row per `(benchmark, model)` cell.
- `models_schema_seed.sql`: SQLite/Postgres-friendly table DDL + inserts for review/query workflows.

Quick SQL review:

```bash
sqlite3 :memory: ".read ./models_schema_seed.sql" \
  "SELECT benchmark, model, value_primary, value_secondary FROM model_benchmark_values LIMIT 20;"
```

## Unified Model + Provider Ledger

Generate unified provider/model ledger artifacts from pricing + benchmark seeds:

```bash
python3 ./scripts/build_unified_ledger.py
# or run full recurring refresh (ledger + optional CLIProxyAPI runtime metrics + Pareto outputs)
python3 ./scripts/refresh_ledger.py --allow-missing-snapshot
```

Generated outputs:
- `ledger/unified_model_provider_ledger.csv`
- `ledger/unified_model_provider_ledger.schema.sql`
- `ledger/unified_model_provider_ledger.seed.sql`
- `ledger/cliproxyapi_runtime_metrics_snapshot.csv`
- `ledger/unified_model_provider_pareto.csv`
- `ledger/unified_model_provider_pareto.view.sql`

Quick SQL validation:

```bash
sqlite3 :memory: \
  ".read ./ledger/unified_model_provider_ledger.schema.sql" \
  ".read ./ledger/unified_model_provider_ledger.seed.sql" \
  "SELECT COUNT(*) AS ledger_rows FROM unified_model_provider_ledger;"
```

CLIProxyAPI management/runtime snapshot integration:

```bash
# Use explicit snapshot(s)
python3 ./scripts/refresh_ledger.py \
  --cliproxyapi-snapshot ./benchmarks/results/cliproxyapi-metrics-snapshot.json \
  --allow-missing-snapshot

# Deterministic CI mode (no home-directory discovery)
python3 ./scripts/refresh_ledger.py --allow-missing-snapshot --skip-runtime-discovery
```

## Task Shortcuts

```bash
task bench:record          # timestamped benchmark report in ./benchmarks/results
task bench:golden          # benchmark + golden correctness validation
task bench:golden:lock     # regenerate benchmark golden fixture from canonical sample inputs
task bench:golden:lock:verify # regenerate fixture then verify with --golden
task bench:trend           # trend summary from benchmark history
task bench:trend:gate      # trend summary + regression fail on threshold breach
task artifacts:ensure      # create ./artifacts and ./benchmarks/results if missing
task perf:gate             # absolute perf gates only
task perf:gate:baseline    # absolute + baseline regression gates
task perf:gate:strict      # strict mode (requires baseline for regression checks)
task perf:gate:golden      # perf gate + golden correctness validation
task pricing:coverage      # pricing coverage report for sample events
task pricing:patch         # write pricing patch template + unpriced JSONL artifacts
task pricing:apply         # dry-run patch merge against ./benchmarks/results/pricing-patch.json
task pricing:apply:write   # apply patch + write backup
task pricing:check         # CI pricing gate (fails on unpriced events)
task pricing:lint          # pricing lint (placeholder-rate + alias-integrity guard)
task pricing:audit         # strict pricing metadata governance audit
task pricing:audit:allow-stale # pricing metadata audit with stale allowed
task pricing:ci            # pricing-lint + pricing-audit + pricing-check (CI parity)
task pricing:reconcile     # in-process coverage/apply/check lifecycle with artifacts
task orchestrate           # ingest(cache-aware) -> pricing-reconcile(dry-run) -> monthly -> daily -> bench -> perf gates + orchestrate-summary.json
task orchestrate:ui        # orchestrate + compact UI snapshot JSON output
task orchestrate:cache     # orchestrate aggregate-cache warm run on sample data
task orchestrate:cache:metrics:validate # validates aggregate cache miss/hit/invalidate summary metrics
task ingest:proxyapi:validate # proxyapi-only ingest sample + structured summary artifacts
task orchestrate:proxyapi:validate # proxyapi-only orchestrate validation run with UI snapshot
task ledger:proxyapi:e2e:validate # runs proxyapi ingest + proxyapi orchestrate validation chain
task ledger:refresh        # regenerate ledger + runtime metrics snapshot CSV + Pareto artifacts
task ledger:check          # deterministic regeneration + git diff check for ledger artifacts
task ledger:sql:validate   # load ledger schema/seed in sqlite and print row count
```

## Pricing Patch Workflow

```bash
# 1) Generate patch + unpriced artifacts from observed coverage gaps
cargo run -- coverage --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --write-patch ./benchmarks/results/pricing-patch.json --write-unpriced-events ./benchmarks/results/unpriced-events.jsonl --json-output

# 2) Validate patch merge plan without writing pricing
cargo run -- pricing-apply --pricing ./pricing.example.json --patch ./benchmarks/results/pricing-patch.json --dry-run

# 3) Apply patch with backup
cargo run -- pricing-apply --pricing ./pricing.example.json --patch ./benchmarks/results/pricing-patch.json --write-backup

# 4) Re-run CI pricing gate
cargo run -- pricing-check --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02
```

## Pricing Lifecycle (Lint + Reconcile)

```bash
# 0) Optional: run placeholder/alias guard before patch work
cargo run -- pricing-lint --pricing ./pricing.example.json

# 1) Reconcile end-to-end and emit structured JSON summary to stdout
cargo run -- pricing-reconcile --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --workdir ./benchmarks/results

# 2) If placeholders are intentionally present during rollout
cargo run -- pricing-lint --pricing ./pricing.example.json --allow-placeholders
```

## Pricing Governance Flow (Audit + Reconcile)

```bash
# 1) Enforce metadata freshness + provenance policy (strict defaults)
cargo run -- pricing-audit --pricing ./pricing.example.json

# 2) Reconcile coverage/apply/check in one pass
cargo run -- pricing-reconcile --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02 --workdir ./benchmarks/results

# 3) Confirm metadata stamp was refreshed after reconcile write
cargo run -- pricing-audit --pricing ./pricing.example.json --json-output
```

## Closeout Notes (2026-02-21)

Completed closure items:

1. Added normalized event schema contract v1 (`docs/contracts/NORMALIZED_EVENT_SCHEMA_CONTRACT_V1.md`).
2. Added UI snapshot schema contract v1 with compatibility policy (`docs/contracts/UI_SNAPSHOT_SCHEMA_CONTRACT_V1.md`).
3. Added extension file-read integration examples (`docs/integrations/EXTENSION_FILE_READ_INTEGRATION_EXAMPLES.md`).
4. Added CI workflow (`.github/workflows/pricing-governance-check.yml`) running:
   - `cargo run -- pricing-lint --pricing ./pricing.example.json`
   - `cargo run -- pricing-audit --pricing ./pricing.example.json`
   - `cargo run -- pricing-check --events ./examples/events.jsonl --pricing ./pricing.example.json --month 2026-02`
5. Added local CI-parity shortcut: `task pricing:ci`.
>>>>>>> 5631926 (fix(tokenledger): resolve compile errors after utils.rs module split)
