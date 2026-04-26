# ADR-002: Tokenization Algorithm Selection for Tokn

**Status:** Proposed  
**Date:** 2026-04-04  
**Deciders:** Tokn Core Team, Research Division  
**Supersedes:** N/A

---

## Context

### Problem Statement

Tokn requires a robust tokenization foundation that balances performance, flexibility, and compatibility. The tokenization algorithm selection fundamentally impacts:

1. **Character coverage**: Must handle all Unicode text without unknown tokens
2. **Sequence length**: Affects model context window utilization
3. **Training capability**: Supporting custom vocabulary creation
4. **Ecosystem compatibility**: Matching outputs of popular tokenizers
5. **Performance**: Critical for high-throughput scenarios

### Current Landscape

The tokenization ecosystem offers several mature algorithms:

| Algorithm | Primary Strength | Limitation |
|-----------|------------------|------------|
| BPE | Fast, simple, widely adopted | Can over-segment rare words |
| WordPiece | Likelihood-optimized | Limited to specific vocabularies |
| Unigram | Optimal segmentation, regularization | Slower, complex training |
| BBPE | Universal coverage (no OOV) | Longer sequences |

### Constraints

1. **Rust implementation**: Tokn is a Rust project requiring native Rust tokenization
2. **Universal coverage**: Must handle any valid UTF-8 text
3. **Performance targets**: 10M+ tokens/second throughput
4. **Training support**: Must allow custom vocabulary training
5. **Compatibility**: Output should match tiktoken for OpenAI models

### Research Findings

Our comprehensive analysis (see `docs/research/TOKENIZATION_ALGORITHMS_SOTA.md`) reveals:

- **BBPE (Byte-level BPE)** is optimal for universal coverage with 0% OOV rate
- **BPE** provides the best balance of speed and quality for single-language use
- **Tiktoken's implementation** achieves 10M+ tokens/second through Rust optimization
- **Vocabulary sizes** between 32K-100K offer optimal compression ratios

---

## Decision

**We will implement Byte-level BPE (BBPE) as the primary tokenization algorithm for Tokn**, with fallback support for standard BPE and extensibility for Unigram.

### Algorithm Stack

```
┌─────────────────────────────────────────────────────────────────┐
│                    Tokn Tokenization Architecture                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  API Layer                                                │   │
│  │  - Tokenizer::encode(text) -> Vec<TokenId>               │   │
│  │  - Tokenizer::decode(tokens) -> String                     │   │
│  │  - Tokenizer::count(text) -> usize                       │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Pre-tokenization (configurable)                         │   │
│  │  - Regex patterns (language-specific)                     │   │
│  │  - Whitespace handling                                   │   │
│  │  - Special token injection                              │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Core Algorithm (Pluggable)                               │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐   │   │
│  │  │  BBPE       │  │  BPE        │  │  Unigram (ext)  │   │   │
│  │  │  (default)  │  │  (option)   │  │  (future)       │   │   │
│  │  └─────────────┘  └─────────────┘  └─────────────────┘   │   │
│  └───────────────────────────────────────────────────────────┘   │
│                              ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │  Post-processing                                          │   │
│  │  - Truncation                                             │   │
│  │  - Padding                                                  │   │
│  │  - Special token handling                                 │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Rationale

### Why BBPE as Default?

1. **Universal Coverage**: BBPE operates on bytes (0-255) rather than characters, ensuring:
   - Zero out-of-vocabulary rate for any valid UTF-8 text
   - Automatic handling of emojis, rare scripts, and new Unicode characters
   - No special handling required for CJK, Arabic, or low-resource languages

2. **Simplicity**: The base vocabulary is fixed at 256 tokens (all bytes), making:
   - Implementation straightforward and auditable
   - Testing comprehensive (all 256 base cases covered)
   - Memory footprint predictable

3. **Proven at Scale**: BBPE powers the world's most deployed LLMs:
   - GPT-2, GPT-3, GPT-4 (OpenAI)
   - RoBERTa, XLM-RoBERTa (Facebook)
   - LLaMA, Mistral (open source)

4. **Training Efficiency**: Starting from 256 base tokens allows:
   - Faster vocabulary training (deterministic initialization)
   - Reproducible results across runs
   - Easy vocabulary expansion

### Why Support Standard BPE?

While BBPE is the default, standard BPE (character-level) offers advantages:

- **Shorter sequences**: ~15% fewer tokens for English text
- **Linguistic alignment**: Tokens often correspond to morphemes
- **Existing vocabularies**: Compatibility with BERT, etc.

### Why Not WordPiece as Default?

WordPiece was evaluated but rejected as primary algorithm because:

- **Complexity**: Likelihood-based training is more complex than frequency-based
- **Speed**: Longest-match decoding slower than greedy merging
- **Less universal**: Primarily used in Google ecosystem (BERT, T5)
- **Unknown tokens**: Requires  or constrained vocabulary

### Why Extensibility for Unigram?

Unigram provides unique capabilities worth supporting:

- **Optimal segmentation**: Viterbi algorithm finds globally optimal solutions
- **Subword regularization**: Multiple valid tokenizations improve robustness
- **Research applications**: Important for tokenization research

---

## Alternatives Rejected

### Alternative 1: Character-Level Tokenization

**Description**: Each Unicode codepoint is a token.

**Pros**:
- Zero OOV rate
- Simplest possible implementation
- No training required

**Cons**:
- Massive vocabulary (100K+ for full Unicode)
- Long sequences (5x tokens vs BBPE for English)
- No subword structure for transfer learning

**Why Rejected**: Poor compression and vocabulary explosion make it impractical for production.

### Alternative 2: Word-Level Tokenization

**Description**: Dictionary-based word tokenization with  for OOV.

**Pros**:
- Best semantic alignment
- Shortest sequences for known words
- Interpretable tokens

**Cons**:
- Massive vocabulary for morphologically rich languages
- High OOV rate for technical/domain text
- Cannot handle rare words or typos

**Why Rejected**: Impossible to achieve reasonable coverage for code and technical content.

### Alternative 3: Neural Tokenization (Learned)

**Description**: Train a neural network to segment text.

**Pros**:
- Potentially optimal for specific corpus
- Can learn task-specific boundaries

**Cons**:
- Computationally expensive at inference
- Requires large training corpus
- Non-deterministic
- Limited research/experimental implementations

**Why Rejected**: Not mature enough for production; no stable reference implementations.

---

## Consequences

### Positive

1. **Universal Compatibility**: BBPE handles any valid UTF-8 without modification
2. **Performance**: Enables 10M+ tokens/second throughput targets
3. **Training Support**: Can train custom vocabularies from any corpus
4. **Ecosystem Alignment**: Matches outputs of tiktoken, GPT models
5. **Maintainability**: Simple, well-understood algorithm

### Negative

1. **Sequence Length**: ~20% longer sequences than character-based BPE
2. **Suboptimal for ASCII**: Wastes vocabulary space on byte representations
3. **Rust Complexity**: Core implementation must be high-performance Rust
4. **Migration Effort**: Existing BERT/WordPiece users need vocabulary retraining

### Neutral

1. **Vocabulary Size**: 256 base + learned merges = 50K-100K typical vocabularies
2. **Pre-tokenization**: Still required for special handling (splitting numbers, etc.)
3. **Multi-language**: Works equally well (or poorly) for all languages

---

## Implementation

### Affected Components

- `crates/tokn-core/src/tokenizer.rs` - Core tokenizer trait
- `crates/tokn-core/src/bbpe.rs` - BBPE implementation
- `crates/tokn-core/src/bpe.rs` - Standard BPE implementation
- `crates/tokn-core/src/vocabulary.rs` - Vocabulary management
- `crates/tokn-core/src/training.rs` - Training pipeline
- `crates/tokn-cli/src/commands/train.rs` - CLI training command

### Migration Strategy

**Phase 1: Core Implementation**
- Implement BBPE core algorithm
- Build training pipeline
- Create vocabulary serialization format

**Phase 2: Compatibility Layer**
- Implement tiktoken-compatible output
- Test against reference implementations
- Achieve 100% match on test corpus

**Phase 3: Extended Algorithms**
- Add standard BPE option
- Add Unigram support (future)

**Phase 4: Optimization**
- SIMD optimizations
- Parallel batch processing
- Memory optimization

### Rollout Plan

- **Phase 1 (Week 1-2)**: Core BBPE implementation and tests
- **Phase 2 (Week 3-4)**: Training pipeline and vocabulary management
- **Phase 3 (Week 5-6)**: Tiktoken compatibility verification
- **Phase 4 (Week 7-8)**: BPE alternative implementation
- **Phase 5 (Week 9+)**: Performance optimization and benchmarking

### Success Criteria

- [ ] BBPE implementation passes 100% of test cases
- [ ] Output matches tiktoken on reference corpus (±0 tokens)
- [ ] Training produces valid vocabularies from any UTF-8 corpus
- [ ] Throughput ≥ 10M tokens/second (single-threaded)
- [ ] Memory usage ≤ 20MB for 50K vocabulary
- [ ] BPE alternative passes 95%+ of same test cases

---

## References

1. [TOKENIZATION_ALGORITHMS_SOTA.md](./TOKENIZATION_ALGORITHMS_SOTA.md) - Comprehensive algorithm analysis
2. [MODERN_TOKENIZERS_SOTA.md](./MODERN_TOKENIZERS_SOTA.md) - Library comparison
3. **Radford, A., et al. (2019).** Language Models are Unsupervised Multitask Learners. *OpenAI Blog*.
4. **Sennrich, R., et al. (2016).** Neural Machine Translation of Rare Words with Subword Units. *ACL*.
5. **OpenAI Tiktoken:** https://github.com/openai/tiktoken

---

**Notes:**
- BBPE is also known as "byte-level BPE" or "Byte Pair Encoding on bytes"
- The 256 base tokens correspond to all possible byte values (0x00-0xFF)
- Special tokens are assigned IDs beyond the base vocabulary range

---

*End of Document - 311 lines*
