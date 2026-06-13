# VIBECODING_GUARD_BASELINE_2026_06_12.md

Protected paths for Tokn (vibecoding-guard do-not-touch zones).

1. `Cargo.lock` — Rust dependency lockfile; regenerating it silently changes the transitive dependency graph
2. `Cargo.toml` — Rust workspace manifest; changes affect crate structure and feature flags
3. `SPEC.md` — Canonical specification; changes here must be reviewed against formal requirements
4. `AGENTS.md` — Agent governance file; protects working conventions and branch discipline
5. `validate_governance.py` — Governance validator; changes to the validator script affect CI enforcement
