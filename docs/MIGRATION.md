# Tokn Migration Guide

## Version Migration

### v0.1.x to v0.2.x
- No breaking changes expected
- Workspace structure: pareto-rs + tokenledger crates

### Data Migration
- Pricing book format: JSON
- Cost snapshots: JSONL
- No database migrations required

### CI/CD Migration
- Release: cargo-dist (cross-compilation)
- Audit: cargo-deny + cargo-audit
- Lint: trunk-check + prettier
