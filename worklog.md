
### 2026-06-12 — V20-05 — V20.5 (adopt pheno-cli-base in focus repo)
**chore: declare pheno-cli-base in [workspace.dependencies] (FLEET_100TASK_DAG_V4 §5 L4 hex)**
- Branched `chore/l1-cli-base-adoption-2026-06-12` from `chore/l1-vibecoding-guard-2026-06-12` (working tree clean, 1 unpushed vibecoding-guard commit inherited as base).
- Added `pheno-cli-base = { path = "../pheno-cli-base" }` to `[workspace.dependencies]` in `Cargo.toml` (root, only Cargo.toml in repo).
- Ran `cargo metadata --offline`: **succeeded (exit 0)** — no errors, no pheno-cli-base mentions in resolved graph.
- **L1 signal (kept, not reverted)**: pheno-cli-base ships only AGENTS.md/CHANGELOG.md/WORKLOG.md/llms.txt/Cargo.lock — no `Cargo.toml` and no `src/lib.rs` (per `pheno-cli-base/WORKLOG.md`, source files pending cherry-pick from `chore/l3-50-pheno-cli-base-2026-06-11`). The dep is dormant (not referenced by `crates/pareto-rs` or `crates/tokenledger`); a real adoption (`pheno-cli-base.workspace = true` in a member) would fail with "no Cargo.toml in /…/pheno-cli-base". Declaration kept; downstream user will hit the failure.
- Branch: `chore/l1-cli-base-adoption-2026-06-12` (1 commit, do-not-push).
# Tokn Worklog

## Recent Entries

### 2026-06-12 — V20-04 — V20.4 (adopt pheno-vibecoding-guard in focus repo)
**chore: wire pheno-vibecoding-guard as local pre-commit hook (FLEET_DAG_v3 §100.4)**
- Appended a `repo: local` block to `.pre-commit-config.yaml` running `pheno-vibecoding-guard scan`.
- Replaces manual AI-drift detection; CI-blocking via the guard's 4 checks (a615f2f).
- Branch: `chore/l1-vibecoding-guard-2026-06-12` (1 commit, do-not-push).

### %Y->- (HEAD -> main) — GOVERNANCE

**ci(workflows): add quality-gate + fr-coverage CI pipeline**

Quality gates and functional requirement coverage tracking added to CI pipeline.

---

## Categories

- **ARCHITECTURE**: ADRs, library extraction, design patterns
- **DUPLICATION**: Cross-project duplication identification
- **DEPENDENCIES**: External deps, forks, modernization
- **INTEGRATION**: External integrations, MCP, plugins
- **PERFORMANCE**: Optimization, benchmarking
- **RESEARCH**: Starred repo analysis, audits
- **GOVERNANCE**: Policy, evidence, quality gates

