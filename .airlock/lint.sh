#!/usr/bin/env bash
# Lint script for Tokn project
# Lints/checks only files changed between AIRLOCK_BASE_SHA and AIRLOCK_HEAD_SHA

set -euo pipefail

# Compute changed files
CHANGED_FILES=$(git diff --name-only "$AIRLOCK_BASE_SHA" "$AIRLOCK_HEAD_SHA" 2>/dev/null || echo "")

if [[ -z "$CHANGED_FILES" ]]; then
    echo "No changed files detected."
    exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES"
echo ""

# Filter to Rust-relevant files
# .rs files - Rust source
# .toml files - Rust config (but Cargo.lock is auto-generated, skip it for formatting)
RUST_FILES=$(echo "$CHANGED_FILES" | grep -E '\.rs$' || true)
TOML_FILES=$(echo "$CHANGED_FILES" | grep -E '\.toml$' | grep -v 'Cargo.lock' || true)

HAS_ISSUES=0

# Format check for .rs files
if [[ -n "$RUST_FILES" ]]; then
    echo "=== Checking Rust formatting ==="
    echo "Files: $RUST_FILES"
    echo ""

    for file in $RUST_FILES; do
        if [[ -f "$file" ]]; then
            if ! rustfmt --check "$file" 2>/dev/null; then
                echo "FORMAT ISSUE: $file"
                HAS_ISSUES=1
            fi
        fi
    done
else
    echo "No .rs files changed, skipping format check."
fi

echo ""

# Lint check for .rs files using clippy
if [[ -n "$RUST_FILES" ]]; then
    echo "=== Running Clippy on changed files ==="
    echo "Files: $RUST_FILES"
    echo ""

    # Create a temp file with the list of files
    TEMP_FILE=$(mktemp)
    echo "$RUST_FILES" > "$TEMP_FILE"

    # Run clippy on the workspace, filtering to changed files only
    # Using --all-targets=false to skip test targets for faster checks
    if cargo clippy --message-format=short 2>&1 | grep -f "$TEMP_FILE" || true; then
        echo "Clippy issues found"
        HAS_ISSUES=1
    fi

    rm -f "$TEMP_FILE"
else
    echo "No .rs files changed, skipping clippy."
fi

echo ""

# Check for whitespace issues in any text files
TEXT_FILES=$(echo "$CHANGED_FILES" | grep -v -E '\.(lock|bin|jpg|png|gif|ico|woff|woff2|ttf|eot)$' || true)
if [[ -n "$TEXT_FILES" ]]; then
    echo "=== Checking for whitespace issues ==="
    if git diff --cached --check 2>/dev/null || true; then
        echo "Staged whitespace OK"
    else
        echo "Staged whitespace issues found"
        HAS_ISSUES=1
    fi
fi

echo ""

if [[ "$HAS_ISSUES" -eq 0 ]]; then
    echo "✓ All checks passed."
    exit 0
else
    echo "✗ Issues detected."
    exit 1
fi
