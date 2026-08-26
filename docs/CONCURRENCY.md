# Concurrency Architecture — pareto-rs

## Overview

`pareto-rs` (part of the Tokn ecosystem) provides high-performance pricing and routing logic. To support multi-threaded and async workloads, it includes a concurrency module (`concurrent.rs`) that wraps shared state with `tokio::sync::RwLock` and provides async hot-path functions using `tokio::spawn`.

## PricingBook

The `PricingBook` is the central store for model pricing data. It is implemented as a `HashMap<(String, String), ModelPricing>` and is wrapped in a `tokio::sync::RwLock` via the `SharedPricingBook` type alias:

```rust
pub type SharedPricingBook = Arc<RwLock<PricingBook>>;
```

### Key Features

- **Concurrent Readers**: Multiple tasks can read pricing data simultaneously without blocking each other.
- **Exclusive Writers**: Only one task can modify the pricing book at a time, ensuring data consistency.
- **Async/Await**: All access is async, allowing seamless integration with other Tokio tasks.

## Hot-Path Functions

The module provides several `tokio::spawn`-based functions to parallelize expensive operations:

### `concurrent_cost_audit`

Calculates costs for a batch of harness records by spawning a separate task for each record. This is ideal for large-scale audit pipelines where records are independent.

### `concurrent_batch_lookup`

Performs multiple price lookups in parallel. Useful for pre-fetching pricing data for a set of pending requests.

### `concurrent_batch_upsert`

Updates multiple pricing entries concurrently. This is used for periodic price feed updates where bulk writes are common.

## Usage Patterns

### Creating a Shared Book

```rust
let book = PricingBook::shared_from_prices(initial_prices);
```

### Concurrent Access

```rust
// Read
let guard = book.read().await;
let price = guard.get_price("openai", "gpt-4o");

// Write
let mut guard = book.write().await;
guard.upsert(new_pricing);
```

### Spawning Audit Tasks

```rust
let agg = concurrent_cost_audit(records, book, OnUnpricedAction::Warn).await;
```

## Best Practices

1. **Clone the `Arc`**: Always clone the `SharedPricingBook` when passing it to spawned tasks to avoid lifetime issues.
2. **Short Critical Sections**: Keep the duration of `write().await` as short as possible to minimize contention.
3. **Use `tokio::spawn`**: For CPU-bound or independent I/O-bound work within the critical sections, prefer spawning new tasks.
4. **Read-Heavy Workloads**: Since `RwLock` allows multiple readers, this architecture is ideal for high-throughput read-heavy systems like live pricing lookups.

## Testing

The module includes both unit tests (in `concurrent.rs`) and a comprehensive suite of 10 integration tests (`tests/concurrent_integration.rs`) that validate concurrency safety under heavy load, including mixed read/write contention scenarios.
