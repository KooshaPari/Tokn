# Tokn Gate Repair Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restore a reproducible, reviewable Tokn baseline in which local quality gates and GitHub workflow syntax are valid, without changing product behavior or discarding any history.

**Architecture:** Keep the repair as a baseline-gates branch off live `origin/main`. Make format-only changes mechanically; correct only demonstrated GitHub Actions defects. Hosted execution remains a distinct, post-PR evidence gate because local validation cannot prove hosted checkout, queue, or runner behavior.

**Tech Stack:** Rust/Cargo, Prettier 3.6.2, GitHub Actions, actionlint, cargo-deny, cargo-mutants.

---

## Scope and non-goals

- Preserve all existing branches, stashes, worktrees, and the published `wip/preserve-20260826-tokn/*` recovery refs.
- Do not change application logic, dependency versions, protections, or merge policy.
- Do not claim Mutation/Scorecard/Infisical hosted success before a fresh GitHub run completes.
- Keep Dependabot PR #121 separate; it must receive its own review and hosted evidence.

## Current implementation status

Tasks 1-3 and the CI parts of Task 4 are implemented and locally verified on this branch. The Mutation portion of Task 4 is deliberately not changed here: the current full-workspace execution is both unbounded and result-masking, while a formatting-only change can still appear as a large source diff to `cargo-mutants`. Replacing it safely requires an explicit coverage policy (for example, semantic-diff normalization plus shard/baseline rules) and its own reviewable PR. This branch must remain a draft until that P1 decision is resolved.

## File map

| File                                                     | Responsibility                                                        |
| -------------------------------------------------------- | --------------------------------------------------------------------- |
| `audit_scorecard.json`                                   | Generated project audit data formatted by Prettier.                   |
| `crates/pareto-rs/**/*.rs`, `crates/tokenledger/**/*.rs` | Rust source that must conform to current rustfmt.                     |
| `.github/workflows/scorecard.yml`                        | Valid OpenSSF Scorecard job configuration.                            |
| `.github/workflows/ci.yml`                               | CI checkout and gitleaks history availability.                        |
| `.github/workflows/mutation.yml`                         | Bounded mutation analysis with an honest failure result.              |
| `crates/pareto-rs/src/pricing.rs`                        | Existing single-item routing test made compatible with strict Clippy. |

### Task 1: Establish red baseline evidence

**Files:** no changes.

- [ ] Run `cargo fmt --all -- --check` and record the failing files.
- [ ] Run `npx --yes prettier@3.6.2 --check audit_scorecard.json` and observe the documented failure.
- [ ] Run `actionlint .github/workflows/scorecard.yml` and observe the expected `unexpected key "security"` failure.
- [ ] Run `cargo test --workspace` and `cargo deny check` before editing; record their exit states as regression baselines.

### Task 2: Make mechanical format fixes

**Files:**

- Modify: `audit_scorecard.json`
- Modify: every Rust file reported by `cargo fmt --all -- --check`

- [ ] Run `npx --yes prettier@3.6.2 --write audit_scorecard.json`.
- [ ] Run `cargo fmt --all`.
- [ ] Verify green with `npx --yes prettier@3.6.2 --check audit_scorecard.json` and `cargo fmt --all -- --check`.
- [ ] Inspect `git diff --check` and `git diff --stat`; confirm no behavior-bearing non-format files changed.

### Task 3: Correct Scorecard workflow schema

**Files:**

- Modify: `.github/workflows/scorecard.yml:23-24`

- [ ] Treat the `actionlint` syntax failure from Task 1 as the red proof.
- [ ] Replace the invalid job-level `security:` mapping with the supported `permissions:` mapping retaining `read-all`.
- [ ] Verify green with `actionlint .github/workflows/scorecard.yml`.
- [ ] Inspect the exact diff to ensure the effective permissions remain read-only.

### Task 4: Make security and mutation execution bounded and observable

**Files:**

- Modify: `.github/workflows/ci.yml` only at the checkout used by the gitleaks path and the aggregate reference to the existing `dep-review` job.
- Modify: `.github/workflows/mutation.yml`.

- [ ] Identify the gitleaks checkout depth in `ci.yml`; set a documented history depth sufficient for its configured `HEAD^` comparison, avoiding a whole-history fetch unless required.
- [ ] Correct the aggregate reference from nonexistent `needs.dependency-review` to the declared `needs.dep-review`; preserve the same gate semantics.
- [ ] Remove the detector step's invalid self-reference to `steps.detect.outputs`; dependent jobs, not the producing step, consume those outputs.
- [ ] Add a job-level `timeout-minutes` to the mutation job so a runner cannot execute indefinitely.
- [ ] Remove the `|| true` suppression from `cargo mutants`; preserve a bounded command (`-j 2`, `--timeout 120`) so test inadequacy is reported rather than masked.
- [ ] Run `actionlint .github/workflows/ci.yml .github/workflows/mutation.yml`; distinguish pre-existing informational shellcheck findings from syntax/expression errors introduced by this branch.
- [ ] Validate changed YAML with `git diff --check` and inspect the exact command/timeout semantics.

### Task 5: Full local and diff verification

**Files:** no new changes.

- [ ] Run `cargo fmt --all -- --check`.
- [ ] Replace the strict-Clippy test-only `&[h.clone()]` pattern with `std::slice::from_ref(&h)` and rerun the affected test before the workspace Clippy gate.
- [ ] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- [ ] Run `cargo test --workspace`.
- [ ] Run `cargo deny check`.
- [ ] Run `npx --yes prettier@3.6.2 --check audit_scorecard.json`.
- [ ] Run `actionlint` on every changed workflow, recording pre-existing unrelated findings separately.
- [ ] Run `./grade.sh --fast`; capture every skipped gate rather than treating the fast result as release proof.

### Task 6: Governed PR and hosted proof

**Files:** no new changes.

- [ ] Commit the scoped baseline-gate repair with a conventional commit message.
- [ ] Push only the new feature branch and open a PR against `main`; do not merge directly.
- [ ] Obtain the required human approval under Tokn main protection.
- [ ] Verify fresh hosted CI, Trunk Check, Scorecard, Mutation, CodeQL, Traceability, and security checks for the PR.
- [ ] Merge through the protected path, then verify a new `main` SHA has completed required workflows.
- [ ] Keep PR #121 independent; review and merge it only after its own CI/review gates are green.

### Task 7: Current-main integration repair

**Files:**

- Modify: `Cargo.lock`
- Modify: `crates/pareto-rs/Cargo.toml`
- Modify: `crates/pareto-rs/src/error.rs`
- Modify: `crates/pareto-rs/src/event.rs`

The replacement branch merged current `main` and exposed defects in modules
introduced there: `error.rs` used the workspace-pinned `thiserror` crate
without declaring it in the package, and strict Clippy required the event bus
to implement `Default`. The public error-variant names are retained to avoid
an API rename; the local lint exception documents that compatibility choice.

- [x] Add `thiserror.workspace = true`; update the `pareto-rs` lockfile
      dependency list while retaining the existing locked `thiserror` version.
- [x] Add a failing `InMemoryEventBus::default()` subscription test, then the
      minimal `Default` implementation and test-only `Arc` import.
- [x] Verify `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
      `cargo test --workspace --no-fail-fast`, and `cargo deny check`.

## Completion definition

Local completion is not release completion. This repair is complete only when the branch diff is verified, the protected PR has a human approval, all required hosted checks are green on the merge SHA, and a human performs the final acceptance/dogfood decision.
