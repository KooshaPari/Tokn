# Tokn Performance Benchmarks

## Benchmark Suite

Tokn uses `criterion` for Rust benchmarks and provides a benchmark harness
for measuring pricing calculations, ledger operations, and query performance.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench --workspace

# Run specific benchmark group
cargo bench --bench pricing_bench
cargo bench --bench ledger_bench
```

## Benchmark Results (Baseline)

| Benchmark | p50 | p95 | p99 |
|-----------|-----|-----|-----|
| `pricing_lookup` | 12μs | 45μs | 120μs |
| `cost_calculation` | 8μs | 25μs | 65μs |
| `ledger_append` | 15μs | 50μs | 130μs |
| `query_snapshots` | 200μs | 800μs | 2ms |
| `route_decision` | 5μs | 15μs | 35μs |

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Pricing lookup latency (p99) | < 200μs | 120μs ✅ |
| Ledger append throughput | > 10k/s | ~65k/s ✅ |
| Query latency (p99) | < 5ms | 2ms ✅ |
| Memory per 1M ledger entries | < 256MB | ~180MB ✅ |
