# STATUS

## Repository

- **Name**: Tokn
- **Purpose**: LLM token usage pricing, coverage, and reconciliation
  tooling (`pareto-rs` + `tokenledger` crates)
- **Stack**: Rust (edition 2024) workspace, 2 crates
- **Branch**: `main`
- **License**: MIT / Apache-2.0

## Build State

| Dimension | Status |
|---|---|
| Build | PASS — `cargo build --workspace` succeeds |
| Test | PASS — `cargo test --workspace` passes |
| Format | PASS — `cargo fmt --all` clean |
| Lint | PASS — `cargo clippy --workspace --all-targets -- -D warnings` clean |
| Audit | PASS — `cargo-audit` green |
| Docs | PASS — docs build (VitePress) green |

## Quality Gates

| Gate | State | Notes |
|---|---|---|
| CI/CD | PASS | `.github/workflows/ci.yml` (Rust CI + governance validation + docs build) |
| Security | PASS | TruffleHog (default branch scan) |
| Governance | PASS | `LICENSE-MIT`, `LICENSE-APACHE`, `AGENTS.md`, `CODEOWNERS`, `SECURITY.md` |
| Reusable workflows | N/A | Not yet using `phenoShared` reusables |

## Worktrees / Stashes

| Type | Count | State |
|---|---|---|
| Worktrees | 4 | Triage pending per E.4.2 (1 is `Tokn/Tokn/Tokn-wtrees/...` nested — bug per E.4.3) |
| Stashes | 0 | None |

## Branches / PRs

| Branch / PR | Status | Action |
|---|---|---|
| `main` | Default | Current |
| PR #59 (`chore(workflows): hygiene pass`) | OPEN | Human review per E.4.1; classify CRITICAL/NORMAL/SKIP per file |
| 3 active wtrees (`ci/best-practices-2026-06-08` w/ 51 commits, 2 others) | OPEN | Cherry-pick to new `chore/ci-best-practices-2026-06-11` etc. per E.4.2 |
| Nested `Tokn/Tokn/Tokn-wtrees/reproducible-2026-06-08` | BUG | `git worktree remove` + recreate at canonical path per E.4.3 |

## Next Steps

1. Human review of PR #59 (classify each file CRITICAL/NORMAL/SKIP);
   merge or push fix commits.
2. Triage 3 active wtrees: cherry-pick each to a new dated branch and
   open 1 PR per (target: wtree count 4 → 1).
3. Fix nested-path bug at `Tokn/Tokn/Tokn-wtrees/reproducible-2026-06-08`.
4. Add `WORKLOG.md` per `WORKLOG_SCHEMA_2026_06_10.md` with 1 entry per
   E.4.x task (E.4.4).
5. Adopt `pheno-observability` in `pareto-rs` + `tokenledger`; delete
   local `tracing_subscriber::fmt` inits (F.4.2).

## Recent Changes

- 2026-06-11: Adopted `phenotype-otel` org-standard STATUS.md;
  replaced `LICENSE-APACHE` placeholder with full Apache 2.0 text.
- 2026-06-08: CI workflow hygiene pass (PR #59).
- 2026-05-23: Repository bootstrap with 2-crate workspace.
