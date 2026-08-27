# Bounded Mutation Gate Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Tokn mutation testing a bounded, changed-Rust pull-request quality signal whose failures are visible to GitHub.

**Architecture:** Replace the all-branch workflow with one PR job. The job checks out base history, writes a Rust-only zero-context diff, exits cleanly only when that diff is empty, and otherwise invokes `cargo mutants --in-diff` directly. Workflow-level concurrency cancels obsolete PR revisions, while the job deadline prevents a runner from occupying capacity indefinitely.

**Tech Stack:** GitHub Actions YAML, actionlint, Cargo, cargo-mutants 27.1.0.

---

## File structure

- Modify: `.github/workflows/mutation.yml` - the sole mutation policy and execution gate.

### Task 1: Record the pre-change failing policy evidence

**Files:**

- Modify: `.github/workflows/mutation.yml`

- [ ] **Step 1: Verify the current workflow masks mutation failures**

Run: `rg -n 'cargo mutants|\|\| true|2>&1 \| tail' .github/workflows/mutation.yml`

Expected: output contains `cargo mutants ... 2>&1 | tail -100 || true`, proving
the runner cannot expose a mutation failure to GitHub.

### Task 2: Replace the workflow with bounded PR policy

**Files:**

- Modify: `.github/workflows/mutation.yml`

- [ ] **Step 1: Replace the workflow content**

```yaml
name: Mutation

on:
  pull_request:
    paths:
      - "crates/**/*.rs"
      - "Cargo.toml"
      - "Cargo.lock"
      - "mutants.toml"
      - ".github/workflows/mutation.yml"

permissions:
  contents: read

concurrency:
  group: mutation-${{ github.event.pull_request.number }}
  cancel-in-progress: true

jobs:
  mutants:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-mutants --version 27.1.0 --locked
      - id: diff
        name: Build semantic Rust diff
        env:
          BASE_SHA: ${{ github.event.pull_request.base.sha }}
        run: |
          set -euo pipefail
          git diff --unified=0 "$BASE_SHA" HEAD -- ':(glob)crates/**/*.rs' > /tmp/mutation.diff
          if [ ! -s /tmp/mutation.diff ]; then
            echo "No changed Rust source; mutation execution is not applicable."
            echo "has_source_changes=false" >> "$GITHUB_OUTPUT"
            exit 0
          fi
          echo "has_source_changes=true" >> "$GITHUB_OUTPUT"
      - name: Test changed mutants
        if: steps.diff.outputs.has_source_changes == 'true'
        run: |
          set -euo pipefail
          cargo mutants --workspace --in-diff /tmp/mutation.diff --no-shuffle -j 2 --timeout 120 --annotations github
```

- [ ] **Step 2: Run actionlint**

Run: `actionlint .github/workflows/mutation.yml`

Expected: exit 0 with no workflow syntax or expression errors.

### Task 3: Verify guard and failure propagation

**Files:**

- Modify: `.github/workflows/mutation.yml`

- [ ] **Step 1: Confirm the old mask is absent**

Run: `! rg -n '\|\| true|2>&1 \| tail' .github/workflows/mutation.yml`

Expected: exit 0.

- [ ] **Step 2: Smoke-test the empty-diff decision**

Run: `tmp=$(mktemp); if [ ! -s "$tmp" ]; then echo "No changed Rust source; mutation execution is not applicable."; fi; rm "$tmp"`

Expected: the exact no-candidate message and exit 0.

- [ ] **Step 3: Inspect the precise diff**

Run: `git diff --check && git diff -- .github/workflows/mutation.yml`

Expected: only the mutation workflow changes; no whitespace errors.

- [ ] **Step 4: Commit after fresh evidence**

```bash
git add .github/workflows/mutation.yml
git commit -m "fix(ci): bound mutation testing to changed Rust"
```
