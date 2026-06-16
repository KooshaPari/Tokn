# Block-C Consolidation Plan — KooshaPari/Tokn

**Date:** 2026-06-16  
**Status:** Approved for execution  
**Audit source:** `docs/audit/BLOCK-C-AUDIT.md` (2026-06-15)  
**Scope:** Tokn (`tokenledger` + `pareto-rs` workspace)

---

## Goal

Consolidate Tokn from an audit-identified **dual-tree, doc-heavy, under-tested** state into a single canonical `crates/` layout with measurable quality gates — without breaking the green `cargo build --workspace` baseline.

---

## Current baseline (verified)

| Check | Result |
|-------|--------|
| `cargo build --workspace` | PASS (1m 06s) |
| `cargo audit` | 0 CVEs |
| Open consolidation PR | #64 `integration/consolidate` (DRAFT — do not merge blindly) |
| Duplicate source trees | `src/` mirrors `crates/tokenledger/src/` (~10k LoC) |

---

## Phase 1 — Source-tree consolidation (P0)

**Objective:** One authoritative crate tree.

| ID | Task | Acceptance |
|----|------|------------|
| C1.1 | Delete orphan root `src/` tree (`git rm -r src`) | `git ls-files src/` empty; workspace still builds |
| C1.2 | Confirm workspace members in `Cargo.toml` only reference `crates/*` | No path outside `crates/` compiled |
| C1.3 | Close or supersede PR #64 after reconciling its 52-file diff against this plan | Either closed with rationale or rebased to match C1.x scope |

**Risk:** Low — audit confirms trees are byte-identical today.

---

## Phase 2 — Module consolidation (P1)

**Objective:** Break the 1,757-line `ingest/mod.rs` god module.

| ID | Task | Acceptance |
|----|------|------------|
| C2.1 | Move logic into existing stubs: `ingest/aggregation.rs`, `parser.rs`, `validation.rs` | Each submodule >50 LoC with clear boundary |
| C2.2 | Keep `ingest/mod.rs` as thin re-export / orchestration only | `ingest/mod.rs` < 200 LoC |
| C2.3 | Replace `ARCHITECTURE.md` skeleton with real doc naming `routing/` as hexagonal exemplar | Doc references actual `crates/` layout |

---

## Phase 3 — Test & CI consolidation (P1)

**Objective:** Coverage matches critical paths.

| ID | Task | Acceptance |
|----|------|------------|
| C3.1 | Add unit tests for `ingest/`, `orchestrate.rs`, `cli.rs`, `pricing.rs` | Each module has ≥1 `#[test]` |
| C3.2 | Wire `just coverage` / `cargo-llvm-cov` gate in CI | CI fails below 60% line coverage (threshold tunable) |
| C3.3 | Remove committed runtime artifacts: `.validation_logs_*`, `benchmarks/results/` | Paths gitignored; removed from index |

---

## Phase 4 — Dependency & workspace hygiene (P2)

| ID | Task | Acceptance |
|----|------|------------|
| C4.1 | Remove 7 unused deps flagged by `cargo machete` | `cargo machete` clean |
| C4.2 | Resolve `pheno-cli-base` phantom workspace dep (add crate or remove entry) | `cargo metadata` resolves without dead paths |
| C4.3 | Either wire `fuzz/` into workspace with `src/lib.rs` or delete stub | No orphan `fuzz/Cargo.toml` |

---

## Phase 5 — Cross-repo Block-C alignment (P2)

Sibling Block-C audits (same wave, separate PRs):

| Repo | Branch / state | Next action |
|------|----------------|-------------|
| `KooshaPari/services` | Local `audit/block-c` (+ audit commit, not pushed) | PR audit to default branch; fix Taskfile YAML + SBOM path hygiene |
| `KooshaPari/McpKit` | Audited in `bc-audit-blockc` | Triage after Tokn P1 complete |
| `KooshaPari/PhenoObservability` | Audited in `bc-audit-blockc` | Triage after Tokn P1 complete |
| `KooshaPari/phenoAI` | Audited in `bc-audit-blockc` | Triage after Tokn P1 complete |

---

## Execution order (DAG)

```
C1.1 → C1.2 → C2.1 → C2.2 → C3.1 → C3.2
              ↘ C4.1, C4.2, C4.3 (parallel after C1.2)
C2.2 → C2.3
C3.3 (anytime after C1.1)
Phase 5 (after Tokn Phase 1 merged)
```

---

## Out of scope (this plan)

- Merging PR #64 wholesale (52 files, deletes licenses/README — requires explicit review)
- Code changes in this docs-only PR
- PhenoDevOps monorepo consolidation (separate initiative)

---

## Success criteria

1. Single `crates/` source tree; zero duplicate `src/` at repo root  
2. `ingest/` split complete with tests on all four critical untested modules  
3. CI coverage gate enforced  
4. Block-C audit + this plan live on `main` under `docs/`
