#!/usr/bin/env bash
# =============================================================================
# Airlock lint script — runs linters/formatters on changed files
# =============================================================================
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

# Compute changed files
CHANGED_FILES=""
if [[ -n "${AIRLOCK_BASE_SHA:-}" && -n "${AIRLOCK_HEAD_SHA:-}" ]]; then
  CHANGED_FILES=$(git diff --name-only "${AIRLOCK_BASE_SHA}" "${AIRLOCK_HEAD_SHA}")
else
  # Fallback: use git status for uncommitted changes
  CHANGED_FILES=$(git diff --name-only HEAD)
fi

if [[ -z "$CHANGED_FILES" ]]; then
  echo "No changed files detected."
  exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES"
echo ""

# Track exit code
EXIT_CODE=0

# ── Rust files ──────────────────────────────────────────────────────────────
RUST_FILES=$(echo "$CHANGED_FILES" | grep -E '\.rs$' || true)
if [[ -n "$RUST_FILES" ]]; then
  echo "==> Rust: rustfmt (format)"
  echo "$RUST_FILES" | xargs rustfmt || true
  echo ""

  echo "==> Rust: clippy (lint with auto-fix)"
  if ! echo "$RUST_FILES" | xargs clippy --fix --allow-dirty --allow-staged 2>&1; then
    EXIT_CODE=1
  fi
  echo ""

  echo "==> Rust: clippy (check)"
  if ! echo "$RUST_FILES" | xargs clippy 2>&1; then
    EXIT_CODE=1
  fi
fi

# ── Python files ─────────────────────────────────────────────────────────────
PYTHON_FILES=$(echo "$CHANGED_FILES" | grep -E '\.py$' || true)
if [[ -n "$PYTHON_FILES" ]]; then
  echo "==> Python: ruff format"
  if command -v ruff &>/dev/null; then
    echo "$PYTHON_FILES" | xargs ruff format || true
    echo ""

    echo "==> Python: ruff lint (fix)"
    echo "$PYTHON_FILES" | xargs ruff check --fix || true
    echo ""

    echo "==> Python: ruff lint (check)"
    if ! echo "$PYTHON_FILES" | xargs ruff check; then
      EXIT_CODE=1
    fi
  else
    echo "ruff not installed, skipping Python"
  fi
fi

# ── JavaScript/TypeScript files ─────────────────────────────────────────────
JS_FILES=$(echo "$CHANGED_FILES" | grep -E '\.(js|jsx|ts|tsx|mjs|cjs)$' || true)
if [[ -n "$JS_FILES" ]]; then
  echo "==> JS/TS: biome format + lint"
  if command -v biome &>/dev/null; then
    echo "$JS_FILES" | xargs biome check --write || true
    echo ""

    echo "==> JS/TS: biome check"
    if ! echo "$JS_FILES" | xargs biome check; then
      EXIT_CODE=1
    fi
  else
    echo "biome not installed, skipping JS/TS"
  fi
fi

# ── Go files ────────────────────────────────────────────────────────────────
GO_FILES=$(echo "$CHANGED_FILES" | grep -E '\.go$' || true)
if [[ -n "$GO_FILES" ]]; then
  echo "==> Go: golangci-lint"
  if command -v golangci-lint &>/dev/null; then
    echo "$GO_FILES" | xargs golangci-lint run --fix || true
    echo ""

    echo "==> Go: golangci-lint (check)"
    if ! echo "$GO_FILES" | xargs golangci-lint run; then
      EXIT_CODE=1
    fi
  else
    echo "golangci-lint not installed, skipping Go"
  fi
fi

# ── Shell files ──────────────────────────────────────────────────────────────
SHELL_FILES=$(echo "$CHANGED_FILES" | grep -E '\.(sh|bash|zsh)$' || true)
if [[ -n "$SHELL_FILES" ]]; then
  echo "==> Shell: shfmt"
  if command -v shfmt &>/dev/null; then
    echo "$SHELL_FILES" | xargs shfmt -i 2 -ci -w || true
  else
    echo "shfmt not installed, skipping shell"
  fi
  echo ""
fi

# ── YAML files ──────────────────────────────────────────────────────────────
YAML_FILES=$(echo "$CHANGED_FILES" | grep -E '\.(yaml|yml)$' || true)
if [[ -n "$YAML_FILES" ]]; then
  echo "==> YAML: check-yaml"
  echo "$YAML_FILES" | xargs python3 -c "import yaml, sys; [yaml.safe_load(open(f)) for f in sys.stdin.read().split()]" 2>/dev/null || {
    echo "YAML validation failed for some files"
    EXIT_CODE=1
  }
fi

# ── Markdown files ───────────────────────────────────────────────────────────
MD_FILES=$(echo "$CHANGED_FILES" | grep -E '\.md$' || true)
if [[ -n "$MD_FILES" ]]; then
  echo "==> Markdown: trailing-whitespace and end-of-file-fixer"
  if command -v pre-commit &>/dev/null; then
    pre-commit run trailing-whitespace end-of-file-fixer --files $MD_FILES 2>/dev/null || true
  fi
fi

echo ""
if [[ $EXIT_CODE -eq 0 ]]; then
  echo "✅ All lint checks passed."
else
  echo "❌ Some lint checks failed."
fi

exit $EXIT_CODE
