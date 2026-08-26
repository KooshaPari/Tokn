# Bounded Mutation Gate Design

## Intent

Replace Tokn's unbounded, failure-masked mutation workflow with an honest,
bounded pull-request signal for Rust source changes.

## Scope

The workflow will run on pull requests only when Rust source, mutation-relevant
Cargo configuration, or its policy file changes. It will checkout merge-base history, derive a
Rust-only unified diff, and run `cargo-mutants` only for mutants in that diff.
It will use fixed parallelism and per-mutant timeout, a job deadline, pinned
`cargo-mutants` version, GitHub annotations, least-privilege permissions, and
PR-scoped cancellation.

## Safety contract

The mutation command will run without a pipe, `|| true`, or any other masked
exit status. A PR with no changed Rust lines will explicitly report that no
source mutation candidates apply, set an explicit false step output, and skip
the mutation step. This is not a claim that the full workspace is
mutation-clean.

## Non-goals

- No full-workspace mutation campaign in this PR: the observed previous run
  exceeded two hours and was terminated. A separately measured scheduled or
  manual campaign remains a future governance decision.
- No changes to product Rust code, dependencies, branch protections, or
  existing baseline PR #122.

## Acceptance criteria

1. Mutation executes on source-relevant PRs and no longer on pushes to main.
2. The job has `contents: read`, fetches complete base history, and cancels
   superseded runs for the same PR.
3. Only changed `crates/**/*.rs` code is passed to `cargo mutants`.
4. The old masked pipeline form cannot remain in the workflow.
5. `actionlint .github/workflows/mutation.yml` succeeds and the empty-diff
   guard exits successfully.
