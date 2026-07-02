# ADR-005: Performance Optimization Strategy

**Status:** Proposed  
**Date:** 2026-04-04  
**Deciders:** Tokn Core Team, Performance Engineering  
**Supersedes:** N/A

---

## Context

### Problem Statement

Tokenization performance is critical for Tokn's success in production environments. The library must achieve competitive throughput while maintaining correctness and compatibility with reference implementations.

### Performance Requirements

Based on market analysis and use cases:

| Use Case | Throughput Target | Latency Target | Scale |
|----------|-------------------|----------------|-------|
| API services | 10M+ tok/s | &lt;1ms p99 | High concurrent |
| Batch processing | 50M+ tok/s | N/A | Single-node throughput |
| CLI operations | 5M+ tok/s | Interactive | Single-threaded |
| Training | 1M+ tok/s | N/A | Background processing |

### Current Baselines

Benchmarks from research (see `docs/research/MODERN_TOKENIZERS_SOTA.md`):

| Library | Throughput | Latency p99 | Language |
|---------|------------|-------------|----------|
| tiktoken | 10M tok/s | 0.15ms | Rust |
| HF Tokenizers | 3M tok/s | 0.50ms | Rust + Python |
| SentencePiece | 2M tok/s | 0.80ms | C++ |
| Pure Python BPE | 0.5M tok/s | 2.0ms | Python |

### Performance Bottlenecks in Tokenization

1. **Regex matching**: Pre-tokenization pattern matching
2. **Hash lookups**: Vocabulary token-to-ID resolution
3. **Memory allocation**: Dynamic token vector growth
4. **String manipulation**: Substring operations
5. **Unicode handling**: Multi-byte character processing

### Constraints

1. **Rust implementation**: Leverage Rust's zero-cost abstractions
2. **Correctness first**: Optimizations must not affect output
3. **Memory safety**: No unsafe code except where necessary
4. **Portability**: Optimizations work across platforms
5. **Compatibility**: Match tiktoken output exactly

---

## Decision

**Tokn will implement a multi-layered performance strategy targeting 15M+ tokens/second throughput.**

### Performance Targets

| Metric | Target | Stretch | Measurement |
|--------|--------|---------|-------------|
| Single-thread throughput | 15M tok/s | 20M tok/s | 1MB English text |
| Latency p50 | 0.05ms | 0.03ms | 1K character input |
| Latency p99 | 0.20ms | 0.10ms | 1K character input |
| Memory/vocab | 15MB | 10MB | 50K vocabulary |
| Batch efficiency | 90% | 95% | 1000x batch size |
| Multi-thread scaling | 80% | 90% | 8 cores |

### Optimization Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                  Performance Architecture                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Layer 1: Algorithmic Optimizations                       │   │
│  │  ┌─────────────────────────────────────────────────────┐ │   │
│  │  │ Pre-compiled regex (lazy_static/once_cell)          │ │   │
│  │  │ Greedy BPE (O(n) merges)                           │ │   │
│  │  │ Binary vocabulary format (zero-copy loading)      │ │   │
│  │  │ Cache-friendly data structures                  │ │   │
│  │  └─────────────────────────────────────────────────────┘ │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Layer 2: Data Structure Optimizations                    │   │
│  │  ┌─────────────────────────────────────────────────────┐ │   │
│  │  │ FNV-1a hash for vocabulary lookups (fast, good)    │ │   │
│  │  │ Vec with capacity hints (pre-allocate)             │ │   │
│  │  │ Stack allocation for small inputs                │ │   │
│  │  │ Arena allocators for batch processing            │ │   │
│  │  └─────────────────────────────────────────────────────┘ │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Layer 3: SIMD and Parallelization                        │   │
│  │  ┌─────────────────────────────────────────────────────┐ │   │
│  │  │ SIMD for byte-level operations (where applicable) │ │   │
│  │  │ Parallel batch processing (rayon)                  │ │   │
│  │  │ Lock-free concurrent vocabularies                  │ │   │
│  │  │ Memory-mapped file I/O for large corpora          │ │   │
│  │  └─────────────────────────────────────────────────────┘ │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Layer 4: System Optimizations                              │   │
│  │  ┌─────────────────────────────────────────────────────┐ │   │
│  │  │ CPU affinity for dedicated workers                │ │   │
│  │  │ NUMA-aware memory allocation                      │ │   │
│  │  │ Profile-guided optimization (PGO)                 │ │   │
│  │  │ Link-time optimization (LTO)                      │ │   │
│  │  └─────────────────────────────────────────────────────┘ │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Rationale

### Why Target 15M tok/s?

Positioning between existing solutions:

- **Faster than HF Tokenizers** (3M/s): 5x improvement demonstrates Rust advantage
- **Faster than tiktoken** (10M/s): Competitive differentiation
- **Achievable**: Within theoretical bounds for optimized Rust

### Why Multi-Layer Optimization?

Different layers address different bottlenecks:

| Layer | Bottleneck Addressed | Expected Gain |
|-------|---------------------|---------------|
| Algorithmic | Regex, lookup complexity | 5-10x |
| Data structures | Memory access patterns | 2-3x |
| SIMD/Parallel | CPU utilization | 2-4x |
| System | Resource efficiency | 1.2-1.5x |

### Why Rust?

Rust enables:
- **Zero-cost abstractions**: High-level code compiles to efficient machine code
- **Memory safety**: No GC pauses or memory corruption
- **SIMD support**: Stable SIMD intrinsics
- **Concurrency**: Fearless parallelism
- **C ABI**: Easy FFI if needed

---

## Alternatives Rejected

### Alternative 1: GPU Tokenization

**Description**: Implement tokenization on GPU for massive parallelism.

**Pros**:
- Theoretical 100x+ speedup
- Good for massive batches

**Cons**:
- Overhead for small inputs
- Memory transfer costs
- Limited by GPU memory
- Complex implementation
- Not needed for most use cases

**Why Rejected**: CPU tokenization is already fast enough; GPU adds complexity for marginal gains in typical workloads.

### Alternative 2: Pure Single-Threaded

**Description**: Optimize only single-threaded performance, ignore parallelism.

**Pros**:
- Simpler implementation
- Deterministic performance
- No synchronization overhead

**Cons**:
- Underutilizes modern CPUs
- Poor batch performance
- Not competitive at scale

**Why Rejected**: Modern servers have many cores; ignoring parallelism wastes resources.

### Alternative 3: Cache-Heavy Design

**Description**: Aggressive caching of all tokenization results.

**Pros**:
- Instant repeated tokenization
- Reduced CPU usage

**Cons**:
- Memory explosion
- Cache invalidation complexity
- Diminishing returns (texts rarely repeat)

**Why Rejected**: Text tokenization has low temporal locality; caching doesn't help typical workloads.

---

## Consequences

### Positive

1. **Competitive advantage**: 15M+ tok/s exceeds most alternatives
2. **API server viability**: Sub-millisecond latency enables real-time use
3. **Batch efficiency**: High throughput for data processing
4. **Energy efficiency**: Fast processing reduces power consumption
5. **Scalability**: Efficient resource use enables scaling

### Negative

1. **Implementation complexity**: Multiple optimization layers
2. **Maintenance burden**: Optimized code is harder to modify
3. **Platform differences**: SIMD varies across architectures
4. **Testing complexity**: Must verify optimizations don't break correctness

### Neutral

1. **API unchanged**: Optimizations are internal
2. **Compatibility maintained**: Output identical to reference
3. **Incremental deployable**: Optimizations can be added over time

---

## Implementation

### Affected Components

- `crates/tokn-core/src/optimized/` - Optimized implementations
- `crates/tokn-core/src/optimized/regex.rs` - Pre-compiled regex
- `crates/tokn-core/src/optimized/lookup.rs` - Fast hash maps
- `crates/tokn-core/src/optimized/batch.rs` - Parallel batch processing
- `crates/tokn-core/src/optimized/simd.rs` - SIMD operations
- `crates/tokn-bench/` - Comprehensive benchmarks
- `.github/workflows/bench.yml` - CI performance tracking

### Algorithmic Optimizations

#### Pre-compiled Regex

```rust
use once_cell::sync::Lazy;
use regex::Regex;

// Compile once at program start
static PRETOKENIZE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+").unwrap()
});

pub fn pretokenize(text: &str) -> Vec<&str> {
    PRETOKENIZE_REGEX.find_iter(text)
        .map(|m| m.as_str())
        .collect()
}
```

#### Fast Vocabulary Lookup

```rust
use rustc_hash::FxHashMap; // Faster than std HashMap for integer keys

pub struct Vocabulary {
    // FNV-1a hash for fast string hashing
    token_to_id: FxHashMap<String, TokenId>,
    id_to_token: Vec<String>,
}

impl Vocabulary {
    pub fn get_id(&self, token: &str) -> Option<TokenId> {
        // Single hash lookup, O(1)
        self.token_to_id.get(token).copied()
    }
}
```

#### Greedy BPE with Early Termination

```rust
pub fn encode(&self, text: &str) -> Vec<TokenId> {
    let mut tokens: Vec<u8> = text.bytes().collect();
    
    // Apply merges greedily
    for &(first, second) in &self.merges {
        let mut i = 0;
        while i < tokens.len() - 1 {
            if tokens[i] == first && tokens[i + 1] == second {
                // Found merge, apply it
                tokens[i] = self.merged_id(first, second);
                tokens.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
    
    tokens
}
```

### Data Structure Optimizations

#### Pre-allocated Vectors

```rust
pub fn encode_batch(&self, texts: &[&str]) -> Vec<Vec<TokenId>> {
    texts.par_iter().map(|text| {
        // Pre-allocate with estimated capacity
        let estimated_tokens = text.len() / 3; // ~3 bytes per token avg
        let mut tokens = Vec::with_capacity(estimated_tokens);
        
        self.encode_into(text, &mut tokens);
        tokens
    }).collect()
}
```

#### Memory-Mapped Vocabularies

```rust
use memmap2::Mmap;

pub struct MmapVocabulary {
    mmap: Mmap,
    token_offsets: Vec<(usize, usize)>, // Offset and length
}

impl MmapVocabulary {
    pub fn get_token(&self, id: TokenId) -> &str {
        let (offset, len) = self.token_offsets[id as usize];
        // Zero-copy string from mmap
        std::str::from_utf8(&self.mmap[offset..offset + len]).unwrap()
    }
}
```

### Parallelization

#### Rayon for Parallel Batches

```rust
use rayon::prelude::*;

pub fn encode_parallel(&self, texts: &[&str]) -> Vec<Vec<TokenId>> {
    texts.par_iter()
        .map(|text| self.encode(text))
        .collect()
}
```

#### Lock-Free Vocabulary Sharing

```rust
use arc_swap::ArcSwap;

pub struct SharedVocabulary {
    // ArcSwap allows lock-free reads, atomic updates
    vocab: ArcSwap<Vocabulary>,
}

impl SharedVocabulary {
    pub fn encode(&self, text: &str) -> Vec<TokenId> {
        // Lock-free read
        self.vocab.load().encode(text)
    }
}
```

### SIMD Optimizations

#### Byte-Level Operations

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn count_tokens_fast(text: &[u8]) -> usize {
    // Use AVX2 for parallel byte scanning if available
    if is_x86_feature_detected!("avx2") {
        unsafe { count_tokens_avx2(text) }
    } else {
        count_tokens_scalar(text)
    }
}

#[target_feature(enable = "avx2")]
unsafe fn count_tokens_avx2(text: &[u8]) -> usize {
    // SIMD implementation
    // ...
}
```

### Benchmarking Infrastructure

```rust
// benches/throughput.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn benchmark_encode(c: &mut Criterion) {
    let tokenizer = load_test_tokenizer();
    let text = load_test_corpus();
    
    let mut group = c.benchmark_group("encode");
    group.throughput(Throughput::Bytes(text.len() as u64));
    
    group.bench_function("single_thread", |b| {
        b.iter(|| tokenizer.encode(black_box(&text)))
    });
    
    group.bench_function("parallel", |b| {
        let texts: Vec<&str> = vec![&text; 100];
        b.iter(|| tokenizer.encode_batch(black_box(&texts)))
    });
}

criterion_group!(benches, benchmark_encode);
criterion_main!(benches);
```

### CI Performance Tracking

```yaml
# .github/workflows/perf.yml
name: Performance
on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run benchmarks
        run: cargo bench -- --save-baseline main
      - name: Compare with main
        run: cargo bench -- --baseline main
      - name: Upload results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion/report.json
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
```

### Rollout Plan

- **Phase 1 (Week 1-2)**: Algorithmic optimizations (regex, lookup)
- **Phase 2 (Week 3-4)**: Data structure optimization (pre-allocation, fast hash)
- **Phase 3 (Week 5-6)**: Parallelization with rayon
- **Phase 4 (Week 7-8)**: SIMD optimizations (where applicable)
- **Phase 5 (Week 9-10)**: System-level optimizations (PGO, LTO)
- **Phase 6 (Ongoing)**: CI performance tracking and regression detection

### Success Criteria

- [ ] Single-thread throughput ≥ 15M tok/s on reference hardware
- [ ] Latency p99 ≤ 0.20ms for 1K character input
- [ ] Memory usage ≤ 15MB for 50K vocabulary
- [ ] Multi-thread scaling ≥ 80% efficiency (8 cores)
- [ ] Output identical to tiktoken on all test inputs
- [ ] No unsafe code except SIMD intrinsics
- [ ] CI performance regression detection operational

---

## References

1. [MODERN_TOKENIZERS_SOTA.md](./MODERN_TOKENIZERS_SOTA.md) - Benchmark data
2. [TOKENIZATION_ALGORITHMS_SOTA.md](./TOKENIZATION_ALGORITHMS_SOTA.md) - Algorithm efficiency
3. **Rust Performance Book:** https://nnethercote.github.io/perf-book/
4. **Tiktoken source:** https://github.com/openai/tiktoken (optimization reference)
5. **Rayon documentation:** https://docs.rs/rayon/latest/rayon/
6. **Criterion.rs:** https://bheisler.github.io/criterion.rs/book/

---

**Notes:**
- All optimizations must maintain bitwise compatibility with tiktoken output
- Performance targets measured on Intel Core i9-12900K or equivalent
- ARM optimizations follow x86 implementation with NEON intrinsics
- Profile-guided optimization requires representative workload for training

---

*End of Document - 362 lines*
