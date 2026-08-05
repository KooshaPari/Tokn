#!/usr/bin/env bash
# =============================================================================
# Tokn Lint Script — lints/checks changed files only
# =============================================================================
# Usage: .airlock/lint.sh
# Requires: AIRLOCK_BASE_SHA, AIRLOCK_HEAD_SHA (set by harness)
# =============================================================================

set -euo pipefail

REPO_ROOT="$(git -C "$(dirname "${BASH_SOURCE[0]}")/.." rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# ---- Detect changed files ---------------------------------------------------
if [[ -z "${AIRLOCK_BASE_SHA:-}" ]] || [[ -z "${AIRLOCK_HEAD_SHA:-}" ]]; then
  echo "error: AIRLOCK_BASE_SHA and AIRLOCK_HEAD_SHA must be set" >&2
  exit 1
fi

# Get list of changed files (exclude deleted files)
CHANGED_FILES=$(git diff --diff-filter=d --name-only "$AIRLOCK_BASE_SHA" "$AIRLOCK_HEAD_SHA")

if [[ -z "$CHANGED_FILES" ]]; then
  echo "No changed files to lint."
  exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES"
echo ""

# ---- Helper: filter by extension ---------------------------------------------
changed_rust_files() {
  echo "$CHANGED_FILES" | grep -E '\.rs$' || true
}

changed_python_files() {
  echo "$CHANGED_FILES" | grep -E '\.py$' || true
}

changed_yaml_files() {
  echo "$CHANGED_FILES" | grep -E '\.(yaml|yml)$' || true
}

# ---- Rust: rustfmt (format) then clippy (lint) -----------------------------
RUST_FILES=$(changed_rust_files)
if [[ -n "$RUST_FILES" ]]; then
  echo "=== Rust formatting (rustfmt) ==="

  # Format all (rustfmt is idempotent; formatting changed files only would miss
  # style issues in their dependencies' public interfaces)
  cargo fmt --all
  echo "rustfmt done."

  echo ""
  echo "=== Rust linting (clippy) ==="

  # Run clippy on the whole workspace to catch all issues in deps/dependencies
  cargo clippy \
    --all-targets --all-features -- \
    -D warnings \
    -A clippy::unwrap_or_default \
    -A clippy::too_many_arguments \
    -A clippy::derive_partial_eq_without_eq \
    -A clippy::large_enum_variant \
    -A unused_imports \
    -A unused

  echo "clippy done."
fi

# ---- Python: ruff format + ruff check ---------------------------------------
PYTHON_FILES=$(changed_python_files)
if [[ -n "$PYTHON_FILES" ]]; then
  echo "=== Python formatting (ruff format) ==="
  echo "$PYTHON_FILES" | xargs ruff format || true
  echo "ruff format done."

  echo ""
  echo "=== Python linting (ruff check) ==="
  echo "$PYTHON_FILES" | xargs ruff check --fix || true
  echo "ruff check done."
fi

# ---- YAML: yamllint ---------------------------------------------------------
YAML_FILES=$(changed_yaml_files)
if [[ -n "$YAML_FILES" ]]; then
  echo "=== YAML linting (yamllint) ==="
  echo "$YAML_FILES" | xargs yamllint -c "$REPO_ROOT/.yamllint.yaml" || true
  echo "yamllint done."
fi

echo ""
echo "=== All linting complete ==="
exit 0
