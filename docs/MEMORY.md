# Tokn Memory Management

## Usage Patterns
Tokn is designed for high-throughput environments where minimizing heap allocations and memory pressure is critical. The system primarily handles token streams and binary protocol data.

## bytes::Bytes Usage
We utilize `bytes::Bytes` for all zero-copy buffer sharing. This allows incoming HTTP payloads and parsed tokens to be shared across multiple tasks without cloning underlying data.
- **Buffer Pooling**: Tokn maintains an internal pool of `Bytes` buffers to minimize reallocation.
- **Reference Counting**: `Arc<Bytes>` is used for long-lived tokens that are accessed by multiple concurrent consumers.

## Memory Pooling Strategy
To avoid memory fragmentation, we implement a tiered memory allocation strategy:
1. **Small Buffers (< 64KB)**: Managed via a custom allocator optimized for small, frequent allocations.
2. **Large Buffers (> 64KB)**: Allocated directly from the system allocator and returned immediately upon task completion.

## Garbage Collection
While Rust provides ownership-based memory management, Tokn uses `jemalloc` as the default allocator to provide better performance in multi-threaded workloads and more efficient handling of large memory footprints.

## Monitoring
Memory usage can be monitored via the `/metrics` endpoint:
- `tokn_memory_active_bytes`: Current active heap memory.
- `tokn_memory_alloc_total`: Total memory allocated since startup.
- `tokn_pool_hits`: Efficiency metric for the internal buffer pool.
