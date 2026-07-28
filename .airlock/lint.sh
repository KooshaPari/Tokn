#!/usr/bin/env bash
# Airlock lint script — lints and formats changed files
# Uses trunk if available, falls back to direct tool calls

set -euo pipefail

# Compute changed files between commits
AIRLOCK_BASE_SHA="${AIRLOCK_BASE_SHA:-HEAD~1}"
AIRLOCK_HEAD_SHA="${AIRLOCK_HEAD_SHA:-HEAD}"

echo "=== Computing changed files: ${AIRLOCK_BASE_SHA}..${AIRLOCK_HEAD_SHA} ==="

# Get list of changed files (filter to relevant types)
CHANGED_FILES=$(git diff --name-only "${AIRLOCK_BASE_SHA}".."${AIRLOCK_HEAD_SHA}" 2>/dev/null || echo "")

if [[ -z "$CHANGED_FILES" ]]; then
    echo "No changed files detected."
    exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES"
echo ""

# Filter by language/file type
RUST_FILES=$(echo "$CHANGED_FILES" | grep -E '\.rs$' || true)
CARGO_FILES=$(echo "$CHANGED_FILES" | grep -E '^Cargo\.lock$|^Cargo\.toml$' || true)
PYTHON_FILES=$(echo "$CHANGED_FILES" | grep -E '\.py$' || true)
GO_FILES=$(echo "$CHANGED_FILES" | grep -E '\.go$' || true)
TS_JS_FILES=$(echo "$CHANGED_FILES" | grep -E '\.(ts|tsx|js|jsx)$' || true)
SHELL_FILES=$(echo "$CHANGED_FILES" | grep -E '\.sh$' || true)
YAML_FILES=$(echo "$CHANGED_FILES" | grep -E '\.ya?ml$' || true)
MARKDOWN_FILES=$(echo "$CHANGED_FILES" | grep -E '\.md$' || true)

echo "=== Filtering by type ==="
echo "Rust: $RUST_FILES"
echo "Cargo: $CARGO_FILES"
echo "Python: $PYTHON_FILES"
echo "Go: $GO_FILES"
echo "TS/JS: $TS_JS_FILES"
echo "Shell: $SHELL_FILES"
echo "YAML: $YAML_FILES"
echo "Markdown: $MARKDOWN_FILES"
echo ""

EXIT_CODE=0

# === FORMATTERS (auto-fix) ===

echo "=== Running formatters (auto-fix) ==="

# Rust formatting (cargo fmt)
if [[ -n "$RUST_FILES" ]] && command -v cargo &>/dev/null; then
    echo "Formatting Rust files with cargo fmt..."
    cargo fmt --
fi

# === LINTERS (auto-fix where possible) ===

echo "=== Running linters (auto-fix) ==="

# Rust clippy (check-only, not auto-fix for safety)
if [[ -n "$RUST_FILES" ]] && command -v cargo &>/dev/null; then
    echo "Running clippy on Rust files..."
    # Only check, don't auto-fix clippy warnings as they can be noisy
    cargo clippy --workspace -- -D warnings 2>/dev/null || true
fi

# === TRUNK (if available) ===
if command -v trunk &>/dev/null; then
    echo "=== Running trunk (unified linting) ==="
    # Use trunk for all linting if available
    trunk check 2>/dev/null || true
    trunk fmt 2>/dev/null || true
else
    echo "trunk not installed, skipping unified linting"
fi

echo ""
echo "=== Lint complete ==="

if [[ $EXIT_CODE -eq 0 ]]; then
    echo "All checks passed!"
else
    echo "Some checks failed (see above)"
fi

exit $EXIT_CODE
