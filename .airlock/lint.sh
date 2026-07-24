#!/usr/bin/env bash
# Airlock lint script for Tokn (Rust + Go)
# Auto-fixes and checks formatting/linting issues in changed files

set -euo pipefail

# Compute changed files between base and head SHAs
CHANGED_FILES=$(git diff --name-only "$AIRLOCK_BASE_SHA" "$AIRLOCK_HEAD_SHA" 2>/dev/null || echo "")

echo "=== Changed files ==="
echo "$CHANGED_FILES"
echo ""

# Categorize changed files
RUST_FILES=$(echo "$CHANGED_FILES" | grep -E '\.rs$' || true)
GO_FILES=$(echo "$CHANGED_FILES" | grep -E '\.go$' || true)
# Lockfiles, markdown, docs, configs — skip these
SKIP_FILES=$(echo "$CHANGED_FILES" | grep -E '\.(lock|md|toml|json|yaml|yml|sql|css|html|svg|png)$' || true)

echo "=== Rust files: ${RUST_FILES:-none} ==="
echo "=== Go files: ${GO_FILES:-none} ==="
echo "=== Files to skip (lock/config/docs): ${SKIP_FILES:-none} ==="
echo ""

# If only non-code files changed, skip linting
if [ -z "$RUST_FILES" ] && [ -z "$GO_FILES" ]; then
    echo "=== No source files changed — skipping lint ==="
    exit 0
fi

# Track overall exit status
EXIT_STATUS=0

# --- Rust: Format first (auto-fix) ---
echo "=== Formatting Rust files ==="
cargo fmt --all 2>/dev/null || true

# --- Rust: Clippy (check mode) ---
echo ""
echo "=== Running Clippy ==="
cargo clippy --all-targets --all-features -- \
    -A clippy::unwrap_or_default \
    -A clippy::too_many_arguments \
    -A clippy::derive_partial_eq_without_eq \
    -A clippy::large_enum_variant \
    -A unused_imports \
    -A unused \
    -D warnings 2>&1 || { EXIT_STATUS=1; echo "Clippy found issues!"; }

# --- Go: Format (auto-fix) ---
if command -v golangci-lint &>/dev/null && [ -n "$GO_FILES" ]; then
    echo ""
    echo "=== Formatting Go files ==="
    gofmt -w $GO_FILES 2>/dev/null || true
fi

# --- Go: Lint (check mode) ---
if command -v golangci-lint &>/dev/null && [ -n "$GO_FILES" ]; then
    echo ""
    echo "=== Running golangci-lint ==="
    golangci-lint run $GO_FILES 2>&1 || { EXIT_STATUS=1; echo "golangci-lint found issues!"; }
fi

# --- Rust: Format check (verify) ---
echo ""
echo "=== Checking Rust formatting ==="
cargo fmt --all -- --check 2>&1 || { EXIT_STATUS=1; echo "Formatting issues found!"; }

echo ""
echo "=== Lint complete ==="
if [ $EXIT_STATUS -eq 0 ]; then
    echo "All checks passed!"
else
    echo "Some checks failed. Please review the output above."
fi

exit $EXIT_STATUS
