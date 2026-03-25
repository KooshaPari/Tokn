# TokenLedger Specification

## Repository Overview

TokenLedger is a Rust-based token accounting and ledger system for tracking usage, billing, and allocation across services.

## Architecture

```
tokenledger/
├── src/                      # Main source code
│   ├── lib.rs               # Library entry point
│   ├── models/              # Domain models
│   │   ├── mod.rs
│   │   ├── token.rs        # Token value object
│   │   ├── account.rs       # Account entity
│   │   ├── transaction.rs   # Transaction entity
│   │   └── ledger.rs       # Ledger aggregate
│   ├── services/            # Application services
│   │   ├── mod.rs
│   │   ├── transfer.rs     # Transfer service
│   │   └── allocation.rs    # Allocation service
│   ├── ports/               # Port interfaces (traits)
│   │   ├── mod.rs
│   │   └── repository.rs    # Repository trait
│   └── adapters/            # Adapter implementations
│       ├── mod.rs
│       ├── postgres.rs       # PostgreSQL adapter
│       └── in_memory.rs      # In-memory for testing
├── tests/                    # Integration tests
├── benches/                  # Benchmarks
├── Cargo.toml               # Workspace manifest
├── Cargo.lock
└── README.md
```

## Domain Model

### Bounded Contexts

1. **Token Accounting** - Core token tracking
2. **Ledger Management** - Transaction ledger
3. **Allocation** - Resource allocation

### Entities & Value Objects

- `Token`: Value object representing token amounts
- `Account`: Entity for user/service accounts
- `Transaction`: Immutable transaction record
- `Ledger`: Aggregate root for ledger operations

## xDD Methodologies Checklist

### TDD (Test-Driven Development)
- [ ] Red-Green-Refactor cycles for all services
- [ ] Unit tests first, then implementation
- [ ] Test coverage > 80%
- [ ] Property-based tests with proptest
- [ ] Mutation coverage with cargo-mutate

### BDD (Behavior-Driven Development)
- [ ] Gherkin feature files for critical flows
- [ ] Scenario outlines for parametrized tests
- [ ] Step definitions in Rust
- [ ] Living documentation generation

### DDD (Domain-Driven Design)
- [ ] Bounded contexts clearly defined
- [ ] Aggregates with clear boundaries
- [ ] Domain events for state changes
- [ ] Value objects for primitives
- [ ] Repository interfaces in ports layer

### ATDD (Acceptance TDD)
- [ ] Acceptance criteria before coding
- [ ] Executable specs
- [ ] Customer-readable documentation

### Clean Architecture
- [ ] Inner layers don't depend on outer
- [ ] Dependencies point inward
- [ ] Ports are abstractions, not implementations

### Hexagonal/Ports & Adapters
- [ ] Driving adapters (CLI, API)
- [ ] Driven adapters (DB, external services)
- [ ] Ports define interface contracts

### SOLID Principles
- [ ] Single Responsibility per module
- [ ] Open/Closed for extension
- [ ] Liskov Substitution everywhere
- [ ] Interface Segregation for ports
- [ ] Dependency Inversion via traits

### GRASP
- [ ] Creator: Factory patterns
- [ ] Information Expert: Logic placement
- [ ] Controller: Use case controllers
- [ ] Polymorphism: Trait objects
- [ ] Protected Variations: Encapsulation

### Event-Driven Architecture
- [ ] Domain events for mutations
- [ ] Event sourcing for ledger
- [ ] Async event handlers
- [ ] Eventual consistency

### CQRS (Command Query Responsibility Segregation)
- [ ] Separate command handlers
- [ ] Separate query handlers
- [ ] Read models optimized
- [ ] Write models for consistency

### Property-Based Testing
- [ ] Arbitrary instances
- [ ] Invariant checks
- [ ] Shrinking on failure

### Mutation Testing
- [ ] Kill mutants
- [ ] Coverage metrics

## Quality Gates

### CI/CD
- [ ] Format check (cargo fmt)
- [ ] Lint check (clippy, rustfmt)
- [ ] Unit tests (cargo test)
- [ ] Integration tests
- [ ] Mutation coverage > 70%
- [ ] Security scan (cargo-audit)
- [ ] Dependency audit

### Performance
- [ ] Benchmark suite (cargo bench)
- [ ] Memory profiling
- [ ] CPU profiling

## File Organization Rules

1. `src/` - All source code
2. `src/models/` - Domain entities and value objects
3. `src/services/` - Application services
4. `src/ports/` - Trait definitions
5. `src/adapters/` - Implementations
6. `tests/` - Integration tests
7. `benches/` - Benchmarks

## Naming Conventions

- Modules: `snake_case`
- Types: `PascalCase`
- Traits: `PascalCase` with `Trait` suffix
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
