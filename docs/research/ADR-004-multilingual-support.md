# ADR-004: Multilingual Tokenization Strategy

**Status:** Proposed  
**Date:** 2026-04-04  
**Deciders:** Tokn Core Team, Internationalization Working Group  
**Supersedes:** N/A

---

## Context

### Problem Statement

Multilingual tokenization presents unique challenges that differ significantly from English-only processing:

1. **Script diversity**: 150+ writing systems with varying character counts
2. **Character coverage**: CJK uses 50K+ characters; Arabic has contextual forms
3. **Whitespace**: Not universal (Chinese, Thai, Japanese often omit spaces)
4. **Normalization**: Unicode equivalence, composed vs decomposed forms
5. **Vocabulary allocation**: How to distribute limited vocabulary across languages

### Language Categories

| Category | Languages | Characteristics | Tokenization Challenge |
|----------|-----------|-----------------|------------------------|
| **Latin-based** | English, Spanish, French | Small alphabets, spaces | Baseline case |
| **Cyrillic** | Russian, Ukrainian | Moderate alphabet | Similar to Latin |
| **CJK** | Chinese, Japanese, Korean | Large character sets | Vocabulary pressure |
| **Arabic/Hebrew** | Arabic, Hebrew | RTL, contextual forms | Normalization |
| **Indic** | Hindi, Bengali, Tamil | Complex scripts | Preprocessing |
| **Southeast Asian** | Thai, Khmer, Lao | No spaces | Segmentation |
| **African** | Amharic, Somali | Limited resources | Training data |

### Current State of Multilingual Tokenization

| Approach | Example | Pros | Cons |
|----------|---------|------|------|
| Separate per-language | Early MT systems | Optimal per language | No cross-lingual transfer |
| Shared BPE | XLM-RoBERTa | Transfer learning | Vocabulary competition |
| BBPE universal | GPT-4, XLM-R | No OOV | Longer sequences for some |
| Script-aware | Specialized systems | Cultural alignment | Complex implementation |

### Research Findings

Our analysis shows (see `docs/research/TOKENIZATION_ALGORITHMS_SOTA.md`):

- **BBPE achieves 0% OOV** for all languages with valid UTF-8
- **CJK compression** is 20-30% worse than English with same tokenizer
- **Vocabulary size scaling**: 250K+ needed for good CJK coverage with BPE
- **Character coverage**: BBPE handles all Unicode without special handling

---

## Decision

**Tokn will adopt a universal BBPE approach with language-aware pre-tokenization options.**

### Core Principles

1. **Universal by default**: BBPE handles all languages without configuration
2. **Language hints optional**: Pre-tokenization can be optimized per language family
3. **No language detection required**: Works without knowing input language
4. **Consistent behavior**: Same tokenizer works across all scripts

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                  Multilingual Tokenization                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Input Text (any language)                                        │
│       ↓                                                           │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │ Unicode Normalization (NFKC)                              │   │
│  │ - Compatibility decomposition                           │   │
│  │ - Canonical composition                                 │   │
│  │ - Case folding (optional)                             │   │
│  └───────────────────────────────────────────────────────────┘   │
│       ↓                                                           │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │ Language Detection (optional)                             │   │
│  │ - Heuristic: script-based                                 │   │
│  │ - Confidence threshold                                    │   │
│  │ - Falls back to generic                                 │   │
│  └───────────────────────────────────────────────────────────┘   │
│       ↓                                                           │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │ Pre-tokenization (language-aware)                         │   │
│  │ ┌────────────┬────────────┬─────────────────────────────┐  │   │
│  │ │ Latin      │ Whitespace │ Standard pattern          │  │   │
│  │ │ CJK        │ Character  │ Per-character + bigrams   │  │   │
│  │ │ Thai       │ Dictionary │ Dictionary-based seg      │  │   │
│  │ │ Arabic     │ Whitespace │ Bidi handling             │  │   │
│  │ │ Generic    │ Byte-level │ Universal fallback      │  │   │
│  │ └────────────┴────────────┴─────────────────────────────┘  │   │
│  └───────────────────────────────────────────────────────────┘   │
│       ↓                                                           │
│  ┌───────────────────────────────────────────────────────────┐   │
│  │ BBPE Core (universal)                                     │   │
│  │ - 256 byte base tokens                                    │   │
│  │ - Learned merges (language-agnostic)                    │   │
│  │ - No language-specific logic                              │   │
│  └───────────────────────────────────────────────────────────┘   │
│       ↓                                                           │
│  Token IDs                                                        │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Pre-tokenization Options

```rust
pub enum LanguageHint {
    Auto,           // Detect from input
    Generic,        // No language-specific handling
    Latin,          // English, Spanish, French, etc.
    Cyrillic,       // Russian, Ukrainian, etc.
    Cjk,            // Chinese, Japanese, Korean
    Arabic,         // Arabic, Hebrew (RTL)
    Indic,          // Hindi, Bengali, etc.
    Thai,           // Thai, Lao, Khmer
    Code,           // Programming languages
}

impl LanguageHint {
    pub fn pre_tokenizer(&self) -> Box<dyn PreTokenizer> {
        match self {
            LanguageHint::Cjk => Box::new(CjkPreTokenizer::new()),
            LanguageHint::Thai => Box::new(ThaiPreTokenizer::new()),
            LanguageHint::Code => Box::new(CodePreTokenizer::new()),
            _ => Box::new(WhitespacePreTokenizer::new()),
        }
    }
}
```

---

## Rationale

### Why BBPE for Multilingual?

1. **Universal coverage**: 0% OOV rate for any valid UTF-8 text
2. **No script-specific code**: Same core for all languages
3. **Proven at scale**: GPT-4, XLM-R use BBPE for 100+ languages
4. **Fairness**: No language gets preferential vocabulary allocation
5. **Simplicity**: Single implementation, comprehensive coverage

### Why Language-Aware Pre-tokenization?

While the core BBPE algorithm is universal, pre-tokenization can optimize for specific languages:

| Language | Optimization | Benefit |
|----------|--------------|---------|
| CJK | Character-level seeding | Better bigram coverage |
| Thai | Dictionary segmentation | Handles lack of spaces |
| Arabic | Bidi handling | Correct RTL processing |
| Code | Identifier splitting | Better compression |

### Why Not Language-Specific Tokenizers?

Approaches like separate Chinese-only or English-only tokenizers were rejected:

1. **Deployment complexity**: Manage N tokenizers instead of 1
2. **Code switching**: Mixed-language text is common
3. **Maintenance burden**: Updates needed for each language
4. **Resource overhead**: Multiple vocabulary files

### Vocabulary Size for Multilingual

For multilingual use cases, we recommend the Extended tier (100K):

| Size | CJK Coverage | Latin Coverage | Use Case |
|------|--------------|----------------|----------|
| 32K | Poor | Good | Single language |
| 50K | Fair | Excellent | 2-3 languages |
| 100K | Good | Excellent | Multilingual |
| 250K | Excellent | Excellent | Research (XLM-R) |

---

## Alternatives Rejected

### Alternative 1: Script-Specific Tokenizers

**Description**: Separate tokenizer implementations per script family.

**Pros**:
- Optimal tokenization per script
- Smaller vocabularies possible
- Cultural/linguistic accuracy

**Cons**:
- Requires language detection
- Complex deployment
- No cross-script transfer
- Maintenance burden

**Why Rejected**: Complexity outweighs benefits; BBPE achieves similar results universally.

### Alternative 2: Dynamic Vocabulary Allocation

**Description**: Adjust vocabulary slots per language based on training data distribution.

**Pros**:
- Optimal allocation for training corpus
- Better compression for dominant languages

**Cons**:
- Requires language-tagged training data
- Unfair to low-resource languages
- Hard to predict behavior

**Why Rejected**: Fairness and predictability are more important than marginal compression gains.

### Alternative 3: Character-Level for CJK

**Description**: Use character-level tokenization for Chinese/Japanese.

**Pros**:
- Perfect CJK coverage
- Simple for those languages

**Cons**:
- Poor for other languages
- No subword structure
- Large vocabularies

**Why Rejected**: BBPE naturally learns character + subword patterns for CJK.

---

## Consequences

### Positive

1. **Universal coverage**: Single tokenizer handles all languages
2. **Fairness**: No language receives preferential treatment
3. **Simplicity**: One implementation, comprehensive testing
4. **Transfer learning**: Subword patterns learned across languages
5. **Deployment**: Single vocabulary file per model

### Negative

1. **CJK efficiency**: 20-30% more tokens than specialized CJK tokenizers
2. **Pre-tokenization complexity**: Optional language-specific paths to maintain
3. **Detection overhead**: Auto-detection adds minor latency
4. **Vocabulary pressure**: Multilingual needs larger vocabularies

### Neutral

1. **Training data**: Multilingual training requires diverse corpus
2. **Evaluation**: Must test across language families
3. **Documentation**: Must cover multiple scripts

---

## Implementation

### Affected Components

- `crates/tokn-core/src/multilingual/` - Language-specific modules
- `crates/tokn-core/src/multilingual/cjk.rs` - CJK handling
- `crates/tokn-core/src/multilingual/thai.rs` - Thai segmentation
- `crates/tokn-core/src/multilingual/arabic.rs` - RTL handling
- `crates/tokn-core/src/normalization.rs` - Unicode normalization
- `crates/tokn-core/src/detection.rs` - Language detection
- `crates/tokn-cli/src/commands/bench.rs` - Multilingual benchmarks

### Language Support Matrix

| Language | Script | Pre-tokenization | Status |
|----------|--------|------------------|--------|
| English | Latin | Whitespace | ✅ Supported |
| Spanish | Latin | Whitespace | ✅ Supported |
| French | Latin | Whitespace | ✅ Supported |
| German | Latin | Whitespace | ✅ Supported |
| Russian | Cyrillic | Whitespace | ✅ Supported |
| Chinese | Han | Character + BBPE | ✅ Supported |
| Japanese | Han/Hiragana/Katakana | Character + BBPE | ✅ Supported |
| Korean | Hangul | Jamo/Whitespace | ✅ Supported |
| Arabic | Arabic | Whitespace + Bidi | ✅ Supported |
| Hebrew | Hebrew | Whitespace + Bidi | ✅ Supported |
| Hindi | Devanagari | Whitespace | ✅ Supported |
| Thai | Thai | Dictionary-based | 🚧 Planned |
| Vietnamese | Latin (extended) | Whitespace | ✅ Supported |
| All others | Various | BBPE fallback | ✅ Supported |

### Normalization Pipeline

```rust
pub fn normalize(text: &str, config: &NormalizationConfig) -> String {
    let mut result = text.to_string();
    
    if config.unicode_normalization {
        // NFKC: Compatibility decomposition + canonical composition
        result = result.nfkc().collect();
    }
    
    if config.case_folding {
        result = result.to_lowercase();
    }
    
    if config.strip_accents {
        result = result.chars()
            .filter(|c| !is_combining_mark(*c))
            .collect();
    }
    
    result
}
```

### Training Considerations

For multilingual vocabularies:

1. **Balanced corpus**: Include proportional representation
2. **Script seeding**: Include all Unicode scripts in initial alphabet
3. **Evaluation**: Test on all target languages

```rust
pub struct MultilingualTrainingConfig {
    pub languages: Vec<LanguageConfig>,
    pub balance: bool,  // Equal weight per language vs proportional
    pub min_frequency: usize,
    pub character_coverage: f32,  // 0.995 = cover 99.5% of chars
}

pub struct LanguageConfig {
    pub code: String,      // ISO 639-1
    pub weight: f32,       // Training weight
    pub corpus_path: PathBuf,
    pub script_hint: ScriptHint,
}
```

### Rollout Plan

- **Phase 1 (Week 1-2)**: Universal BBPE implementation
- **Phase 2 (Week 3-4)**: Language detection and pre-tokenization framework
- **Phase 3 (Week 5-6)**: CJK optimization
- **Phase 4 (Week 7-8)**: Arabic/Hebrew RTL support
- **Phase 5 (Week 9-10)**: Thai and Indic support
- **Phase 6 (Week 11+)**: Extended language support

### Success Criteria

- [ ] BBPE tokenizes all Unicode text without errors
- [ ] 0% OOV rate on valid UTF-8 input
- [ ] CJK compression within 30% of English
- [ ] RTL languages render correctly
- [ ] Language detection >90% accuracy
- [ ] Thai segmentation reasonable quality (BLEU > 0.7)

---

## References

1. [TOKENIZATION_ALGORITHMS_SOTA.md](./TOKENIZATION_ALGORITHMS_SOTA.md) - Algorithm analysis
2. [CODE_TOKENIZATION_SOTA.md](./CODE_TOKENIZATION_SOTA.md) - Code-specific handling
3. **Conneau, A., et al. (2020).** Unsupervised Cross-lingual Representation Learning at Scale. *ACL* (XLM-R).
4. **Rust, P., et al. (2023).** How Good is Your Tokenizer? On the Language Specificity of Multilingual Tokenizers. *ACL*.
5. **Unicode Standard Annex #15:** Unicode Normalization Forms. https://unicode.org/reports/tr15/

---

**Notes:**
- BBPE naturally handles all scripts through UTF-8 byte encoding
- Language-specific optimizations are additive, not required
- Unicode normalization is applied before tokenization

---

*End of Document - 328 lines*
