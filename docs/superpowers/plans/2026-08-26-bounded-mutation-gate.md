# Bounded Mutation Gate Implementation Plan

**Goal:** Make Tokn mutation testing a bounded, changed-Rust pull-request
quality signal whose failures are visible to GitHub.

**Architecture:** The workflow checks out base history, creates a Rust-only
zero-context textual diff, exits cleanly only for an empty diff, and otherwise
invokes `cargo mutants --in-diff` directly. Concurrency cancels obsolete
revisions; the job deadline prevents a runner from occupying capacity
indefinitely.

## Evidence and tasks

- [x] Record the prior policy: it ran on `main`, installed an unpinned tool,
      piped output through `tail`, suppressed failures with `|| true`, and exceeded
      three hours before GitHub terminated it with exit 143.
- [x] Restore the bounded policy from preserved candidate #123 additively onto
      current-main successor #124; #123 itself remains preserved.
- [x] Run `actionlint .github/workflows/mutation.yml`.
- [x] Prove no old `|| true` or output-masking pipe remains.
- [x] Generate the current base-to-head Rust diff locally and verify the
      empty/non-empty decision.
- [ ] Push only after local workflow validation, then require a fresh hosted
      mutation result for the exact replacement head.
