# Contributing to tokenledger

Thank you for your interest in contributing to tokenledger.

## Development Setup

```bash
# Clone the repository
git clone https://github.com/Phenotype-Enterprise/tokenledger
cd Tokn

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build

# Test
cargo test

# Lint
cargo clippy
cargo fmt --check
```

## Architecture

tokenledger follows hexagonal architecture:

```
src/
├── domain/           # Pure domain (no external deps)
├── application/      # Use cases
├── adapters/         # Port implementations
│   ├── primary/     # Driving adapters
│   └── secondary/   # Driven adapters
└── infrastructure/   # Cross-cutting concerns
```

## Code Quality Standards

| Metric | Threshold | Enforcement |
|--------|-----------|-------------|
| Test coverage | >= 80% | cargo-tarpaulin |
| Security findings | 0 high/critical | cargo-audit |
| Clippy warnings | 0 | CI gate |

## Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes
4. Add tests (targeting 80% coverage)
5. Ensure all checks pass: `cargo clippy -- -D warnings`
6. Commit using conventional commits
7. Push and create PR

## Development Philosophy

- **Extend, Never Duplicate**: Refactor the original, never create v2
- **Primitives First**: Build generic building blocks before application logic
- **Research Before Implementing**: Check crates.io for existing libraries

## Code Style

- Max function length: 40 lines
- No placeholder TODOs in committed code
- Zero lint suppressions without inline justification
- All Rust code must be clippy-clean
