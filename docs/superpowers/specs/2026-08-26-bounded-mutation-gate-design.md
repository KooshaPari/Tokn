# Bounded Mutation Gate Design

## Intent

Replace Tokn's unbounded, failure-masked mutation workflow with an honest,
bounded pull-request signal for Rust source changes.

## Scope

The workflow runs on pull requests only when Rust source, mutation-relevant
Cargo configuration, or its policy file changes. It checks out merge-base
history, derives a Rust-only zero-context textual diff, and runs
`cargo-mutants` only for mutants in that diff. It uses fixed parallelism and
per-mutant timeout, a job deadline, pinned `cargo-mutants` version, GitHub
annotations, least-privilege permissions, and PR-scoped cancellation.

## Safety contract

The mutation command runs without a pipe, `|| true`, or any other masked exit
status. A PR with no changed Rust lines explicitly reports that no source
mutation candidates apply, sets an explicit false step output, and skips the
mutation step. This is not a claim that the full workspace is mutation-clean.
Reformatted Rust may still appear in the textual diff; semantic normalization
is intentionally outside this repair's scope.

The exact `pareto-server` `main -> ()` mutant is excluded by name. Its `main`
function is a long-lived process entry point with no controlled shutdown or
process harness, so that replacement cannot be observed by this unit-test
runner. This exclusion does not exclude the file or its HTTP handlers, does
not establish server startup coverage, and leaves a separately scoped
process-lifecycle smoke test required before treating that entry point as
covered.

## Non-goals

- No full-workspace mutation campaign in this PR: the previous run exceeded
  three hours and was terminated. A separately measured scheduled or manual
  campaign remains a future governance decision.
- No product Rust, dependency, branch-protection, or baseline-PR changes.

## Acceptance criteria

1. Mutation executes on source-relevant PRs and no longer on pushes to main.
2. The job has `contents: read`, fetches complete base history, and cancels
   superseded runs for the same PR.
3. Changed `crates/**/*.rs` code is passed to `cargo mutants`, except for the
   explicitly documented `pareto-server` `main -> ()` mutant.
4. The old masked pipeline form cannot remain in the workflow.
5. `actionlint .github/workflows/mutation.yml` succeeds and the empty-diff
   guard exits successfully.
