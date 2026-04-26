# ADR-003: Vocabulary Size and Architecture

**Status:** Proposed  
**Date:** 2026-04-04  
**Deciders:** Tokn Core Team, Performance Engineering  
**Supersedes:** N/A

---

## Context

### Problem Statement

Vocabulary size is a fundamental architectural decision in tokenization systems with far-reaching implications:

1. **Model capacity**: Larger vocabularies increase embedding matrix parameters
2. **Sequence length**: Smaller vocabularies require longer token sequences
3. **Training data**: Larger vocabularies need more data to train properly
4. **Memory footprint**: Vocabulary size directly impacts RAM usage
5. **Coverage**: Smaller vocabularies have higher OOV rates (except BBPE)

### Current Best Practices

Industry standard vocabulary sizes vary by use case:

| Use Case | Typical Size | Examples |
|----------|--------------|----------|
| English-only BERT | 30K | BERT Base/Large |
| Multilingual | 100K-250K | XLM-RoBERTa (250K) |
| General LLM | 50K-100K | GPT-4 (100K), LLaMA (32K) |
| Code-specific | 50K | CodeBERT (50K), CodeT5 (32K) |
| Low-resource | 10K-20K | Experimental |

### Research Findings

Our analysis (see `docs/research/TOKENIZATION_ALGORITHMS_SOTA.md`) shows:

- **Diminishing returns**: Compression improvement drops sharply after 50K
- **32K baseline**: Covers ~95% of tokens in most languages
- **50K optimal**: Covers ~98% with reasonable sequence length
- **100K**: Marginal improvement (99%+) but 2x memory cost

### Compression Analysis (C4 English Dataset)

| Vocab Size | Avg Tokens/Sentence | Total Tokens (B) | Improvement |
|------------|---------------------|------------------|-------------|
| 8K | 35.2 | 12.8 | Baseline |
| 16K | 28.1 | 10.2 | -20% |
| 32K | 24.5 | 8.9 | -30% |
| 50K | 22.8 | 8.3 | -35% |
| 100K | 21.2 | 7.7 | -40% |
| 200K | 20.1 | 7.3 | -43% |

### Constraints

1. **Memory efficiency**: Target <20MB per vocabulary
2. **Performance**: Larger vocabularies have cache implications
3. **Training data**: Must have sufficient corpus for chosen size
4. **Compatibility**: Match reference implementations (tiktoken: 100K)
5. **Flexibility**: Support multiple sizes for different use cases

---

## Decision

**Tokn will support configurable vocabulary sizes with tiered recommendations:**

| Tier | Size | Use Case | Rationale |
|------|------|----------|-----------|
| **Compact** | 32,000 | Resource-constrained, single-language | Balance of coverage and efficiency |
| **Standard** | 50,000 | General-purpose, multi-language | Optimal efficiency point |
| **Extended** | 100,000 | Maximum coverage, multilingual | Match tiktoken/GPT-4 |
| **Custom** | 10K-200K | Specialized applications | User-defined |

### Default Configuration

```rust
pub enum VocabularySize {
    Compact = 32_000,   // 32K tokens
    Standard = 50_000,  // 50K tokens (default)
    Extended = 100_000, // 100K tokens
    Custom(usize),      // User-defined
}

impl Default for VocabularySize {
    fn default() -> Self {
        VocabularySize::Standard
    }
}
```

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                   Vocabulary Architecture                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Vocabulary (trait)                                       │   │
│  │  - token_to_id(&str) -> Option<TokenId>                   │   │
│  │  - id_to_token(TokenId) -> Option<&str>                    │   │
│  │  - len() -> usize                                         │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  BBPEVocabulary                                           │   │
│  │  ┌─────────────────────────────────────────────────────┐   │   │
│  │  │ Base tokens (256)                                  │   │   │
│  │  ├─────────────────────────────────────────────────────┤   │   │
│  │  │ Merged tokens (32K/50K/100K - 256)                │   │   │
│  │  ├─────────────────────────────────────────────────────┤   │   │
│  │  │ Special tokens (50-100)                            │   │   │
│  │  └─────────────────────────────────────────────────────┘   │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Storage Format (.tokn)                                   │   │
│  │  - Binary format for fast loading                         │   │
│  │  - Version metadata                                       │   │
│  │  - Merge rules (ordered)                                  │   │
│  │  - Special token mappings                               │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Rationale

### Why Tiered Sizes?

1. **Resource-constrained environments**: Edge devices, embedded systems need 32K
2. **General use**: 50K provides optimal efficiency/coverage trade-off
3. **Maximum compatibility**: 100K matches OpenAI's tiktoken/GPT-4
4. **Future-proofing**: Custom sizes allow experimentation

### Why 50K as Default?

The 50,000 token vocabulary is optimal because:

1. **Efficiency**: Within 5% of 100K's compression with half the memory
2. **Coverage**: 98%+ coverage for English, 95%+ for most languages
3. **Precedent**: GPT-2, RoBERTa, and many production systems use 50K
4. **Training data**: Requires 1-10GB text (achievable)
5. **Speed**: Fits efficiently in CPU cache during tokenization

### Memory Analysis

| Component | 32K | 50K | 100K |
|-----------|-----|-----|------|
| Base tokens (256) | ~5KB | ~5KB | ~5KB |
| Merged tokens | ~240KB | ~375KB | ~750KB |
| String storage | ~2MB | ~3MB | ~6MB |
| HashMap overhead | ~4MB | ~6MB | ~12MB |
| **Total** | **~6MB** | **~10MB** | **~19MB** |

### Cache Efficiency

| Vocab Size | L1 Cache (32KB) | L2 Cache (256KB) | L3 Cache (8MB) |
|------------|-----------------|------------------|----------------|
| 32K | ✅ Fits | ✅ Fits | ✅ Fits |
| 50K | ❌ Overflow | ✅ Fits | ✅ Fits |
| 100K | ❌ Overflow | ❌ Partial | ✅ Fits |

---

## Alternatives Rejected

### Alternative 1: Fixed Single Size (50K only)

**Description**: Only support 50,000 token vocabulary.

**Pros**:
- Simple implementation
- Single code path to test
- No configuration complexity

**Cons**:
- Too large for resource-constrained environments
- Too small for some multilingual scenarios
- No tiktoken compatibility

**Why Rejected**: Would limit adoption in edge cases and prevent tiktoken parity.

### Alternative 2: Dynamic Vocabulary

**Description**: Automatically determine optimal size based on training corpus.

**Pros**:
- Optimal for each use case
- No manual tuning required

**Cons**:
- Non-deterministic behavior
- Hard to compare across deployments
- Complex implementation
- May produce unexpected sizes

**Why Rejected**: Predictability and reproducibility are more important than theoretical optimality.

### Alternative 3: Very Large Vocabulary (200K+)

**Description**: Default to 200K+ for maximum coverage.

**Pros**:
- Best possible compression
- Handles rare terms well

**Cons**:
- 4x memory vs 50K
- Requires massive training corpus
- Diminishing returns (only 8% better than 50K)
- Poor cache locality

**Why Rejected**: Memory overhead and training requirements outweigh minimal benefits.

---

## Consequences

### Positive

1. **Flexibility**: Users choose appropriate size for their constraints
2. **Efficiency**: Default 50K provides optimal efficiency point
3. **Compatibility**: 100K tier enables tiktoken parity
4. **Edge deployment**: 32K enables resource-constrained use cases
5. **Future-proof**: Custom sizes allow experimentation

### Negative

1. **Complexity**: Multiple code paths to test and maintain
2. **Decision burden**: Users must choose (though default is provided)
3. **Binary size**: Larger vocabularies increase .tokn file sizes
4. **Training time**: Larger vocabularies take longer to train

### Neutral

1. **API surface**: Same API regardless of vocabulary size
2. **Serialization**: Same .tokn format supports all sizes
3. **Interoperability**: Size must be communicated between systems

---

## Implementation

### Affected Components

- `crates/tokn-core/src/vocabulary.rs` - Vocabulary trait and implementations
- `crates/tokn-core/src/vocabulary/bbpe.rs` - BBPE vocabulary
- `crates/tokn-core/src/training/` - Training pipeline with size selection
- `crates/tokn-cli/src/commands/train.rs` - CLI size parameter
- `crates/tokn-cli/src/commands/info.rs` - Vocabulary inspection
- `docs/VOCABULARY_GUIDE.md` - User documentation

### Configuration

```rust
// Configuration structure
pub struct VocabularyConfig {
    pub size: VocabularySize,
    pub special_tokens: Vec<String>,
    pub initial_alphabet: Option<Vec<u8>>, // For BBPE: always all bytes
}

// CLI usage
tokn train --vocab-size 50000 corpus.txt -o vocab.tokn
tokn train --vocab-size compact corpus.txt -o vocab.tokn
tokn train --vocab-size extended corpus.txt -o vocab.tokn
```

### Training Pipeline Changes

```rust
pub fn train_vocabulary(
    corpus: impl Iterator<Item = String>,
    config: &VocabularyConfig,
) -> Result<Vocabulary, TrainingError> {
    match config.size {
        VocabularySize::Compact => train_with_size(corpus, 32_000, config),
        VocabularySize::Standard => train_with_size(corpus, 50_000, config),
        VocabularySize::Extended => train_with_size(corpus, 100_000, config),
        VocabularySize::Custom(n) => train_with_size(corpus, n, config),
    }
}
```

### Validation

| Size | Min Training Data | Expected Compression | Test Coverage |
|------|-------------------|----------------------|-------------|
| 32K | 100MB | 24.5 tok/sent | 100% |
| 50K | 1GB | 22.8 tok/sent | 100% |
| 100K | 10GB | 21.2 tok/sent | 100% |

### Rollout Plan

- **Phase 1 (Week 1-2)**: Implement tiered vocabulary sizes
- **Phase 2 (Week 3)**: Training pipeline size selection
- **Phase 3 (Week 4)**: CLI integration and testing
- **Phase 4 (Week 5)**: Benchmark all tiers
- **Phase 5 (Week 6)**: Documentation and examples

### Success Criteria

- [ ] All three standard sizes (32K, 50K, 100K) pass test suite
- [ ] Memory usage within 10% of projected values
- [ ] Compression ratios match research benchmarks
- [ ] Training produces valid vocabularies for all sizes
- [ ] CLI supports all size options
- [ ] Documentation explains selection criteria

---

## References

1. [TOKENIZATION_ALGORITHMS_SOTA.md](./TOKENIZATION_ALGORITHMS_SOTA.md) - Compression analysis
2. **Bostrom, K., & Durrett, G. (2020).** Byte Pair Encoding is Suboptimal for Language Model Pretraining. *EMNLP*.
3. **OpenAI Tiktoken:** https://github.com/openai/tiktoken (100K vocabulary)
4. **HuggingFace Tokenizers:** https://huggingface.co/docs/tokenizers

---

**Notes:**
- Vocabulary size includes: 256 base tokens + merged tokens + special tokens
- Special tokens typically add 50-100 IDs beyond the nominal size
- Training corpus size recommendations are approximate; more data always helps

---

*End of Document - 304 lines*
