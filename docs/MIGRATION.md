# Tokn Migration Guide

This document covers migration procedures for upgrading between Tokn versions, including data format changes, CI/CD adjustments, and workspace structure shifts.

---

## Version Migration

### v0.1.x to v0.2.x

> This section applies when v0.2.x is released.

**Breaking changes:**

- Pricing book JSON schema adds optional `cache_write_usd_per_mtok`, `cache_read_usd_per_mtok`, `tool_input_usd_per_mtok`, and `tool_output_usd_per_mtok` fields. Existing books without these fields will use fallback defaults (input rate for cache write, 10% of input for cache read, input rate for tool input, output rate for tool output). No manual migration is required.
- The `meta` block in the pricing book becomes semi-required: `tokn pricing-audit` will fail if `meta.source` or `meta.updated_at` are missing. Add them before upgrading:

```json
{
  "meta": {
    "source": "pricing-manual",
    "updated_at": "2026-08-21T00:00:00Z"
  }
}
```

**Non-breaking additions:**

- New CLI commands: `tokn pricing-reconcile`, `tokn pricing-lint`, `tokn pricing-audit`.
- New `tokn bench` and `tokn perf-gate` subcommands for performance benchmarking.
- Workspace adds `pareto-rs` crate for pure pricing/routing analytics (no I/O).

### Workspace Structure Changes

```
# Pre-v0.2 layout
src/
  main.rs
  pricing.rs
  cost.rs
  ...

# v0.2+ layout (additive)
src/          # CLI + orchestration
crates/
  pareto-rs/  # Pure analytics engine (new)
  tokenledger/# Blockchain ledger (existing)
```

No existing paths change. The `pareto-rs` crate is a new addition for separation of concerns.

---

## Data Format Changes

### Pricing Book JSON

The pricing book is the primary configuration for per-model cost rates. Key structure:

```json
{
  "meta": {
    "source": "pricing-manual",
    "updated_at": "2026-08-21T00:00:00Z",
    "version": "1"
  },
  "providers": {
    "openai": {
      "subscription_usd_month": 0.0,
      "models": {
        "gpt-4o": {
          "input_usd_per_mtok": 2.5,
          "output_usd_per_mtok": 10.0,
          "cache_write_usd_per_mtok": 2.5,
          "cache_read_usd_per_mtok": 1.25,
          "tool_input_usd_per_mtok": 2.5,
          "tool_output_usd_per_mtok": 10.0
        }
      },
      "model_aliases": {
        "gpt-4o-2024-08-06": "gpt-4o"
      }
    }
  },
  "provider_aliases": {
    "claude": "anthropic"
  }
}
```

**v0.1.x format (simplified):**

```json
{
  "providers": {
    "openai": {
      "subscription_usd_month": 0.0,
      "models": {
        "gpt-4o": {
          "input_usd_per_mtok": 2.5,
          "output_usd_per_mtok": 10.0
        }
      }
    }
  }
}
```

**Migration steps:**

1. Add the `meta` block (required by `tokn pricing-audit`).
2. Add `model_aliases` entries for any model name variants observed in your event logs.
3. Optionally add `cache_write_usd_per_mtok`, `cache_read_usd_per_mtok`, `tool_input_usd_per_mtok`, `tool_output_usd_per_mtok` for cache/tool cost tracking. Defaults apply if omitted.
4. Add `provider_aliases` for provider name normalization (e.g., `"claude" -> "anthropic"`).
5. Run `tokn pricing-check --events events.jsonl --pricing pricing.json --on-unpriced warn` to verify coverage.

### Cost Snapshots (JSONL)

Each line in a cost snapshot file is a JSON object:

```json
{
  "id": "snapshot-001",
  "provider": "openai",
  "model": "gpt-4o",
  "input_tokens": 1500,
  "output_tokens": 800,
  "input_cost": 0.00375,
  "output_cost": 0.008,
  "total_cost": 0.01175,
  "latency_ms": 142.5,
  "timestamp": "2026-08-21T12:00:00Z",
  "routing_criteria": "balanced",
  "routing_score": 0.87
}
```

**No schema changes between v0.1.x and v0.2.x.** The format is stable.

### Events JSONL (Ingested)

Each line is a normalized usage event:

```json
{
  "provider": "openai",
  "model": "gpt-4o",
  "session_id": "sess-abc123",
  "timestamp": "2026-08-21T12:00:00Z",
  "usage": {
    "input_tokens": 1500,
    "output_tokens": 800,
    "cache_write_tokens": 0,
    "cache_read_tokens": 0,
    "tool_input_tokens": 200,
    "tool_output_tokens": 50
  }
}
```

All token fields default to 0 when absent (via `#[serde(default)]`).

---

## CI/CD Migration Notes

### Release Pipeline

| Component           | Tool             | Notes                                                                                 |
| ------------------- | ---------------- | ------------------------------------------------------------------------------------- |
| Cross-compilation   | `cargo-dist`     | Produces binaries for linux-x64, linux-aarch64, macos-x64, macos-aarch64, windows-x64 |
| License audit       | `cargo-deny`     | Configured via `deny.toml`                                                            |
| Vulnerability audit | `cargo-audit`    | Runs in CI on every push to main                                                      |
| Lint                | `trunk-check`    | Clippy + rustfmt + additional lints                                                   |
| Formatting          | `prettier`       | For YAML, JSON, MD files                                                              |
| Tests               | `cargo-nextest`  | Parallel test runner                                                                  |
| Coverage            | Codecov          | Uploads from CI, see `codecov.yml`                                                    |
| Attestation         | `gh attestation` | Binary provenance for release assets                                                  |

### Updating CI Workflows

If migrating from a different CI system:

1. Ensure `rust-toolchain.toml` is present (specifies the Rust version).
2. Add the following secrets:
   - `CARGO_REGISTRY_TOKEN` — for crates.io publishing on tag.
   - `GITHUB_TOKEN` — standard Actions token for release creation.
3. The `binaries.yml` workflow requires `fail_on_unmatched_files: true` to prevent silent build failures.
4. The `release-crates.yml` workflow runs on tag pushes matching `v*`.

### Pre-commit Hooks

Run `pre-commit install` after cloning to enable local hooks (trailing-whitespace, yaml-check, etc.).

---

## Workspace Structure Changes

### Crate Layout

```
Tokn/
  Cargo.toml          # Workspace root
  VERSION              # Single source of truth for version
  src/                 # Main binary crate
    main.rs            # CLI entry point
    cli.rs             # Clap argument definitions
    cost.rs            # Cost calculation engine
    pricing.rs         # Pricing reconciliation / audit / lint
    analytics.rs       # Monthly/daily/coverage reports
    format.rs          # Output formatting (table, markdown, JSON)
    cache.rs           # Coverage reports, unpriced event handling
    ingest.rs          # Event ingestion from JSONL files
    models.rs          # Shared data types (UsageEvent, PricingBook, etc.)
    orchestrate.rs     # Pipeline orchestration
    bench.rs           # Performance benchmarking
  crates/
    pareto-rs/         # Pure analytics engine (no I/O)
      src/
        lib.rs         # Module root
        cost.rs        # Cost calculations (pure)
        pricing.rs     # Pareto-optimal routing (pure)
        format.rs      # Formatting helpers (pure)
        models.rs      # Shared model types (pure)
        utils.rs       # CSV parsing, formatting (pure)
    tokenledger/       # Blockchain-backed token state
  configs/             # Configuration files
  examples/            # Example data files
  tests/               # Integration tests
  fuzz/                # Fuzz testing targets
  benches/             # Criterion benchmarks
```

### Key Configuration Files

| File                  | Purpose                                                |
| --------------------- | ------------------------------------------------------ |
| `Cargo.toml`          | Workspace dependencies and package metadata            |
| `VERSION`             | Single version string (read by CI and release scripts) |
| `deny.toml`           | cargo-deny license and advisory config                 |
| `rust-toolchain.toml` | Pinned Rust toolchain version                          |
| `clippy.toml`         | Clippy lint configuration                              |
| `rustfmt.toml`        | Rustfmt configuration                                  |
| `codecov.yml`         | Coverage upload configuration                          |
| `mutants.toml`        | cargo-mutants configuration                            |
| `nextest.toml`        | cargo-nextest configuration                            |
| `justfile`            | Task runner recipes                                    |
| `Taskfile.yml`        | Task runner (alternative to justfile)                  |

### Adding a New Crate

1. Create `crates/<name>/Cargo.toml` with `version.workspace = true`.
2. Add it to the workspace members in root `Cargo.toml`.
3. Keep pure logic in the crate; CLI orchestration stays in `src/`.
4. Add integration tests under `tests/` at the workspace root.
