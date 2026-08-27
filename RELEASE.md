# Tokn v0.2.0 — Release Notes

**Release Date:** 2026-08-26  
**Previous Version:** v0.1.5

---

## Highlights

Tokn v0.2.0 is a major feature release that transforms Tokn from a CLI analytics tool into a pluggable, observable, and web-accessible platform. This release introduces a plugin system, web dashboard, full observability stack, concurrency support, structured error handling, and infrastructure-as-code for container and Kubernetes deployments.

---

## New Features

### Plugin System (`plugin.rs`)
- Introduced the `Plugin` trait and `PluginRegistry` for extensible architecture.
- Three built-in plugins ship by default:
  - **CSV Exporter** — exports analytics data to CSV format.
  - **Slack Notifier** — sends alerts and summaries to Slack channels.
  - **S3 Uploader** — uploads generated reports to S3 buckets.
- Plugins register themselves via the `PluginRegistry` and are invoked during report generation.

### Web Dashboard (`web/`)
- Added a lightweight HTTP dashboard for viewing analytics in the browser.
- `index.html` — landing page with navigation to dashboard and API docs.
- `dashboard.html` — interactive dashboard showing cost breakdowns, token usage, and session trends.
- Served via the existing Axum HTTP server on `/` routes.

### CLI Improvements
- Enhanced CLI argument parsing and subcommand handling.
- New `--format` options for controlling output (table, JSON, CSV).
- Improved help text and usage examples across all subcommands.

### Migration Support (`schema/`, `migrations/`)
- Added `schema/` directory for defining and evolving the normalized event schema.
- New `migrations/runner.py` — a Python-based migration runner that applies schema migrations to event data stores.
- Supports forward migrations with dry-run capability.

### Observability (`metrics.rs`)
- New `metrics.rs` module with Prometheus-compatible metrics exposition.
- **HTTP Endpoints:**
  - `GET /health` — liveness and readiness probe returning service status.
  - `GET /metrics` — Prometheus scrape endpoint with request counters, latency histograms, and error rates.
- Metrics cover ingestion, aggregation, report generation, and plugin execution.

### Concurrency (`concurrent.rs`)
- New `concurrent.rs` module for safe concurrent access to shared state.
- Uses `tokio::sync::RwLock` for read-heavy workloads with write coalescing.
- `tokio::spawn` used for parallel report generation across independent provider data sets.

### Error Handling (`error.rs`)
- Centralized error types in `error.rs` using `thiserror` for ergonomic `Error` trait implementations.
- Structured `ToknError` enum covering IO, parsing, schema, plugin, and HTTP errors.
- All public API functions return `Result<T, ToknError>`.

### Event Bus (`event.rs`)
- New `EventBus` trait defining a publish/subscribe interface for internal events.
- Events include: `IngestionComplete`, `ReportGenerated`, `PluginInvoked`, `ErrorOccurred`.
- Enables decoupled components to react to pipeline lifecycle events.

---

## Architecture & Documentation

### Cost Analysis (`COST_MODEL.md`)
- Detailed documentation of the blended cost model, including per-model rate cards, subscription allocation, and cache-aware pricing.

### Memory Profiling (`MEMORY.md`)
- Guide for profiling Tokn's memory usage with `dhat`, `tracing`, and `jemalloc`.
- Includes benchmarks for large-scale ingestion pipelines.

### Architecture Documentation (`ARCHITECTURE.md`)
- Comprehensive architecture overview covering module boundaries, data flow, plugin lifecycle, and extension points.
- Includes ASCII diagrams of the ingestion → aggregation → reporting pipeline.

---

## Infrastructure

### Docker Support
- `Dockerfile` — multi-stage build producing a minimal runtime image.
- `docker-compose.yml` — one-command local development stack with Postgres, Redis, and the Tokn web server.

### Kubernetes (`k8s/`)
- `k8s/deployment.yaml` — production-grade Deployment with resource limits, probes, and rolling update strategy.
- `k8s/service.yaml` — ClusterIP Service for internal access.
- `k8s/configmap.yaml` — externalized configuration via ConfigMap.

---

## Internal Improvements

- Modularized codebase continued evolution from v0.1.x baseline.
- All new modules include doc comments and unit tests.
- CI pipeline updated to validate plugin loading, web dashboard serving, and Kubernetes manifest syntax.

---

## Upgrade Notes

- **Breaking:** The internal event schema has been versioned; existing JSONL files from v0.1.x are compatible but new fields are optional.
- **New dependency:** `thiserror` added for structured error handling.
- **New dependency:** `tokio` features expanded to include `sync` and `macros` for concurrency support.
- **Docker:** Users deploying via Docker should rebuild images from the new `Dockerfile`.

---

## Assets

| Asset | Platform | Size |
|-------|----------|------|
| `tokn-linux-amd64` | Linux x86_64 | ~12 MB |
| `tokn-linux-arm64` | Linux aarch64 | ~12 MB |
| `tokn-darwin-amd64` | macOS x86_64 | ~13 MB |
| `tokn-darwin-arm64` | macOS aarch64 | ~13 MB |
| `tokn-windows-amd64.exe` | Windows x86_64 | ~13 MB |

---

**Full Changelog:** See [CHANGELOG.md](CHANGELOG.md) for the detailed list of changes.
