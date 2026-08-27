# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-20

### Added

- **Core Analytics**
  - Monthly variable token cost computation from per-model rate cards (FR-COST-001)
  - Provider subscription allocation in blended totals (FR-COST-002)
  - Blended `$ / MTok` computation globally, per provider, per model (FR-COST-003)

- **Token Reporting**
  - Token type breakdown: input, output, cache write/read, tool input/output (FR-TOK-001)

- **Session Tracking**
  - Unique monthly session counts globally and by dimension (FR-SES-001)

- **Data Ingestion**
  - Normalized event ingestion from JSONL files and directories recursively (FR-ING-001)

- **Output Formats**
  - Human-readable table output support (FR-OUT-001)
  - JSON output support (FR-OUT-001)

- **Optimization Tips**
  - Tip engine generating optimization suggestions from measured telemetry (FR-TIP-001)

- **Architecture**
  - Rust core for high-throughput aggregation (ADR-001)
  - Normalized event contract for multi-provider ingestion (ADR-002)
  - Blended cost model including subscriptions (ADR-003)

### Changed

- Modularized codebase from single 8759-line main.rs into 10 focused modules

### Fixed

### Security

---

## [0.1.1] - 2026-04-25

### Fixed

- **Licensing**
  - Add Unicode-3.0 to deny.toml allow list (W-79: cargo-deny fix)

### Changed

- CI: Add monthly SBOM workflow per org standard
- Rust 2024 edition migration completed
- Align tokio + serde to org baseline (phenotype-versions.toml)

---

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

---

## [0.2.0] - 2026-08-26

### Added

- **Plugin System**
  - `Plugin` trait and `PluginRegistry` for extensible architecture (`plugin.rs`)
  - 3 built-in plugins: CSV Exporter, Slack Notifier, S3 Uploader

- **Web Dashboard**
  - `web/index.html` — landing page and navigation
  - `web/dashboard.html` — interactive analytics dashboard with cost breakdowns, token usage, and session trends

- **Observability**
  - `metrics.rs` — Prometheus-compatible metrics exposition
  - `GET /health` — liveness and readiness probe
  - `GET /metrics` — Prometheus scrape endpoint with request counters, latency histograms, and error rates

- **Concurrency**
  - `concurrent.rs` — `tokio::sync::RwLock` for shared state with read-heavy workloads
  - Parallel report generation via `tokio::spawn`

- **Error Handling**
  - `error.rs` — centralized `ToknError` enum using `thiserror`
  - Structured errors covering IO, parsing, schema, plugin, and HTTP failure modes

- **Event Bus**
  - `event.rs` — `EventBus` trait with publish/subscribe interface
  - Events: `IngestionComplete`, `ReportGenerated`, `PluginInvoked`, `ErrorOccurred`

- **Migration Support**
  - `schema/` directory for normalized event schema definitions
  - `migrations/runner.py` — Python-based migration runner with dry-run support

- **Architecture & Documentation**
  - `COST_MODEL.md` — blended cost model documentation
  - `MEMORY.md` — memory profiling guide with `dhat`, `tracing`, `jemalloc`
  - `ARCHITECTURE.md` — module boundaries, data flow, plugin lifecycle, ASCII diagrams

- **Infrastructure**
  - `Dockerfile` — multi-stage build for minimal runtime image
  - `docker-compose.yml` — local dev stack (Postgres, Redis, Tokn web server)
  - `k8s/deployment.yaml` — production Deployment with probes and rolling updates
  - `k8s/service.yaml` — ClusterIP Service
  - `k8s/configmap.yaml` — externalized configuration

- **Release Documentation**
  - `RELEASE.md` — comprehensive release notes for v0.2.0

### Changed

- **CLI** — enhanced argument parsing, new `--format` options (table, JSON, CSV), improved help text
- Internal event schema versioned; v0.1.x JSONL files remain compatible

### Security

- CI pipeline updated to validate plugin loading, web dashboard serving, and Kubernetes manifest syntax

---

## [0.1.5] - 2026-07-18

### Added

- Release cut guide: [`docs/guides/cutting-a-release.md`](docs/guides/cutting-a-release.md)
  (tag + `gh release` commands for operators; no tag created in-repo)

### Changed

- **Version:** workspace `package.version` aligned to **0.1.5** (next release after
  local tags `v0.1.2`–`v0.1.4`; crates were previously stuck at `0.1.1` in Cargo.toml)
- **Release workflows:** `binaries.yml` fails hard if the packaged binary is missing
  (`fail_on_unmatched_files: true`); attestation staging no longer soft-fails empty
  executable copies
- **crates.io publish:** `release-crates.yml` requires `CARGO_REGISTRY_TOKEN` on tag
  publishes; soft-fail retained only for PR dry-runs under empty Actions billing and
  for idempotent “already published” crate versions

### Fixed

- Ensure `aarch64-apple-darwin` remains in the binary matrix and release asset upload
  path is verified before attach
