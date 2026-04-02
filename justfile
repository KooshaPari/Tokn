# Tokn - Rust token management CLI
# Native task runner (just)

# Default recipe
default: help

# Help
help:
  @echo "Tokn - Rust token management CLI"
  @echo ""
  @just --list

# Quality checks
check: fmt clippy test
  @echo "All checks passed!"

# Format code
fmt:
  cargo fmt --all

# Lint
clippy:
  cargo clippy --workspace -- -D warnings

# Run tests
test:
  cargo test --workspace

# Build
build:
  cargo build --release

# Run dev server
dev:
  cargo run --bin toknd --watch

# Run CLI
run *args:
  cargo run --bin tokn {{args}}

# Clean
clean:
  cargo clean

# Documentation
doc:
  cargo doc --no-deps --all
