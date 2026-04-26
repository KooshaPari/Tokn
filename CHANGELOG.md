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
