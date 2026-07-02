# Unified Routing Orchestration System - End-to-End Architecture Plan

**Date:** 2026-02-24
**Status:** Draft
**Scope:** Consolidate tokenledger, CLIProxyAPI, thegent, agentapi++ into unified routing orchestration
**Principles:** HEXAGONAL | POLYGLOT | KISS | DRY | SOLID | MICROSERVICE

---

## 1. Executive Summary

This plan consolidates all existing research and implementations across:
- **tokenledger** - Model/provider ledger (56 rows), pricing, Pareto scoring
- **CLIProxyAPI** - API proxy with routing, model definitions
- **thegent** - Quality/speed/cost values, harness model mapping
- **agentapi++** - Agent SDK and message formats

The goal: Build a unified "donut" layer around the harness that handles provider/model optimization without the harness needing to know about it.

---

## 2. Current State Analysis

### 2.1 What's Already Built

| Component | Location | Status |
|-----------|----------|--------|
| Model/Provider Ledger | `tokenledger/ledger/` | ✅ 56 rows, SQL schema |
| Quality Values | `thegent/models/quality_values.py` | ✅ Benchmark-based quality index |
| Speed Values | `thegent/models/speed_values.py` | ✅ Speed index |
| Cost Values | `thegent/models/cost_values.py` | ✅ Cost tracking |
| Harness Model Mapping | `thegent/utils/routing_impl/harness_model_mapping.py` | ✅ Provider/harness resolution |
| Pareto Router | `cliproxy++/pkg/llmproxy/registry/pareto_router.go` | ⚠️ Hardcoded maps (25 models) |
| CLIProxyAPI Feed | `tokenledger/scripts/refresh_ledger.py` | ✅ Snapshot import |
| Polyglot Governance | `thegent/docs/governance/POLYGLOT_RUNTIME_...` | ✅ Language standards |

### 2.2 Data Sources Identified

| Source | Type | Metrics | Status |
|--------|------|---------|--------|
| Artificial Analysis | API | Intelligence, speed, latency, pricing | ✅ Has free API |
| OpenRouter | API | Pricing, context, provider stats | ✅ Has API |
| LMSYS Arena | Scrape | Quality rankings | ⚠️ Scrapers needed |
| Vellum | Scrape | Quality rankings | ⚠️ Scrapers needed |
| Manual Overrides | Config | Any | ❌ Not implemented |

---

## 3. Target Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        UNIFIED ROUTING ORCHESTRATION LAYER                  │
│                           (The "Donut" around Harness)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌─────────────────────────────────────────────────────────────────┐    │
│   │                    TOKENLEDGER CORE (Rust)                      │    │
│   │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────┐  │    │
│   │  │ Model Ledger  │  │Provider Ledger│  │ Benchmark Store   │  │    │
│   │  │ • 56 rows     │  │ • Pricing     │  │ • AA API          │  │    │
│   │  │ • Mappings    │  │ • Credentials │  │ • OpenRouter API  │  │    │
│   │  │ • Provenance  │  │ • Endpoints   │  │ • Manual overrides│  │    │
│   │  └───────────────┘  └───────────────┘  └───────────────────┘  │    │
│   │                                                                    │    │
│   │  ┌─────────────────────────────────────────────────────────────┐  │    │
│   │  │              Provider/Harness/Model Resolver                │  │    │
│   │  │   • resolve_for_backend()  • get_quality()                 │  │    │
│   │  │   • get_cost()  • get_speed()  • get_pareto_score()      │  │    │
│   │  └─────────────────────────────────────────────────────────────┘  │    │
│   └─────────────────────────────────────────────────────────────────┘    │
│                                    │                                        │
│   ┌────────────────────────────────┼────────────────────────────────┐    │
│   │              ADAPTERS           │        (HEXAGONAL PORTS)        │    │
│   ├─────────────────────────────────┼────────────────────────────────┤    │
│   │  ┌──────────┐  ┌──────────┐   │   ┌──────────────────────────┐ │    │
│   │  │CLIProxy  │  │Helios    │   │   │ Provider/Harness/Model  │ │    │
│   │  │Adapter   │  │Harness   │   │   │ Pair & Trio Resolver    │ │    │
│   │  │          │  │Feed      │   │   │                          │ │    │
│   │  └──────────┘  └──────────┘   │   └──────────────────────────┘ │    │
│   │  ┌──────────┐  ┌──────────┐   │   ┌──────────────────────────┐ │    │
│   │  │thegent   │  │agentapi  │   │   │ Routing Engine           │ │    │
│   │  │Adapter   │  │Adapter   │   │   │ • Quality Router        │ │    │
│   │  │          │  │          │   │   │ • Cost Router           │ │    │
│   │  └──────────┘  └──────────┘   │   │ • Speed Router          │ │    │
│   │                               │   │ • Pareto Selector       │ │    │
│   │                               │   └──────────────────────────┘ │    │
│   └────────────────────────────────┴────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           HARNESS LAYER                                    │
│   (Codex, LiteLLM, Claude Code, Gemini CLI, OpenCode, etc.)             │
│                                                                              │
│   The harness is WRAPPED by the routing layer - it doesn't know about    │
│   provider/model optimization, it just receives standardized requests.      │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Core Data Structures

### 4.1 Provider-Harness-Model Trio

```rust
// Core trio: the fundamental routing unit
struct ProviderHarnessModel {
    // Identity
    provider: Provider,      // openai, anthropic, google, etc.
    harness: Harness,         // codex, litellm, claudecode, etc.
    model: Model,             // gpt-4o, claude-3-5-sonnet, etc.
    
    // Metrics from tokenledger
    quality_score: f64,       // 0.0-1.0 from benchmarks
    cost_per_1k: f64,        // USD per 1K tokens
    latency_ms: u32,         // p50 latency
    throughput_tps: f64,     // tokens per second
    
    // Pareto
    pareto_rank: u32,
    pareto_dominated_by: Vec<String>,  // IDs that dominate this
}
```

### 4.2 Provider-Model Pair

```rust
// For pricing/routing without harness context
struct ProviderModelPair {
    provider: Provider,
    model: Model,
    
    // From tokenledger pricing
    input_price_per_1m: f64,
    output_price_per_1m: f64,
    cache_read_price_per_1m: Option<f64>,
    cache_write_price_per_1m: Option<f64>,
    context_window: u64,
    
    // From benchmarks
    intelligence_index: Option<f64>,
    coding_index: Option<f64>,
    agentic_index: Option<f64>,
    speed_tps: Option<f64>,
    latency_ttft_ms: Option<f64>,
}
```

### 4.3 Benchmark Data (Unified)

```rust
struct BenchmarkData {
    model_id: String,
    provider: Option<String>,
    
    // Quality
    intelligence_index: Option<f64>,
    coding_index: Option<f64>,
    agentic_index: Option<f64>,
    
    // Performance
    speed_tps: Option<f64>,
    latency_ttft_ms: Option<f64>,
    latency_e2e_ms: Option<f64>,
    
    // Pricing
    price_input_per_1m: Option<f64>,
    price_output_per_1m: Option<f64>,
    price_cache_read_per_1m: Option<f64>,
    price_cache_write_per_1m: Option<f64>,
    
    // Context
    context_window_tokens: Option<u64>,
    
    // Source tracking
    source: BenchmarkSource,  // artificial_analysis, openrouter, manual, scrape
    confidence: f64,          // 0.0-1.0
    updated_at: DateTime<Utc>,
}

enum BenchmarkSource {
    ArtificialAnalysis,
    OpenRouter,
    ManualOverride,
    WebScrape,
    Fallback,
}
```

---

## 5. Implementation Phases

### Phase 1: Consolidate to tokenledger (Core)

**Goal:** Make tokenledger the single source of truth for all benchmark/pricing data

| Task | From | To | Status |
|------|------|-----|--------|
| Move quality values | `thegent/models/quality_values.py` | `tokenledger/src/benchmarks/quality.rs` | ❌ TODO |
| Move speed values | `thegent/models/speed_values.py` | `tokenledger/src/benchmarks/speed.rs` | ❌ TODO |
| Move cost values | `thegent/models/cost_values.py` | `tokenledger/src/benchmarks/cost.rs` | ❌ TODO |
| Move harness mapping | `thegent/utils/routing_impl/harness_model_mapping.py` | `tokenledger/src/mappings/` | ❌ TODO |
| Add AA API client | NEW | `tokenledger/src/benchmarks/artificial_analysis.rs` | ❌ TODO |
| Add OpenRouter API client | NEW | `tokenledger/src/benchmarks/openrouter.rs` | ❌ TODO |

### Phase 2: Build Hexagonal Adapters

**Goal:** Create clean ports/adapters for each system

| Adapter | Description | Port Interface |
|---------|-------------|----------------|
| CLIProxyAPI | Real-time metrics feed | `trait CLIProxyAdapter` |
| HeliosHarness | Benchmark result ingestion | `trait HarnessAdapter` |
| thegent | Quality/speed/cost queries | `trait RoutingAdapter` |
| agentapi++ | Agent lifecycle events | `trait AgentAdapter` |

```rust
// Example port interface
pub trait BenchmarkPort {
    fn get_benchmark(&self, model_id: &str) -> Option<BenchmarkData>;
    fn get_all_benchmarks(&self) -> Vec<BenchmarkData>;
    fn refresh(&self) -> Result<()>;
}
```

### Phase 3: Routing Engine Enhancement

**Goal:** Replace hardcoded Pareto maps with dynamic tokenledger lookups

| Task | Description |
|------|-------------|
| Remove hardcoded maps | Replace `qualityProxy`, `costPer1kProxy`, `latencyMsProxy` |
| Add dynamic lookup | Query tokenledger for benchmark data |
| Add fallback logic | Use hardcoded values when API unavailable |
| Add real-time updates | Wire CLIProxyAPI telemetry |

### Phase 4: Manual Overrides & Governance

**Goal:** Allow config-driven overrides with highest priority

```yaml
# benchmarks.yaml
sources:
  artificial_analysis:
    enabled: true
    api_key: ${AA_API_KEY}
  
  openrouter:
    enabled: true
    api_key: ${OPENROUTER_API_KEY}

overrides:
  # Highest priority - manual benchmark values
  "gemini-3.1-pro":
    intelligence_index: 57.0
    speed_tps: 65
    latency_ms: 3990
```

---

## 6. Polyglot Standards

Follow existing governance from `thegent/docs/governance/POLYGLOT_RUNTIME_COVERAGE_AND_CONVERSION_MATRIX_2026-02-21.md`:

| Language | Primary Runtime | Required Tests |
|----------|----------------|----------------|
| Rust | stable toolchain | `cargo test`, `clippy -D warnings` |
| Python | `uv` + CPython 3.14 | pytest + PyPy 3.11 |
| Go | latest two minors | `go test ./...`, `go vet` |

---

## 7. Principles

### HEXAGONAL
- Ports and adapters pattern
- Core domain logic isolated from infrastructure
- Adapters are swappable (file-based, API-based, scrape-based)

### POLYGLOT
- Use right tool for job
- Rust for core ledger/performance
- Python for data processing/ML
- Go for CLI tools

### KISS
- Simple first, optimize later
- No premature abstraction

### DRY
- Single source of truth (tokenledger)
- Shared data structures
- No duplication of benchmark data

### SOLID
- Single responsibility: tokenledger owns all benchmark data
- Open/closed: extend via adapters, not modification
- Liskov: adapters must implement full port interface
- Interface segregation: small, focused ports
- Dependency inversion: depends on abstractions, not concretions

### MICROSERVICE
- Independent deployability
- Clear boundaries
- API-first design

---

## 8. Migration Path

### 8.1 Immediate (This Sprint)

1. Add AA + OpenRouter API clients to tokenledger
2. Create benchmarks module structure
3. Define core data structures

### 8.2 Short-term (Next 2 Sprints)

1. Move quality/speed/cost from thegent
2. Build CLIProxyAPI adapter
3. Remove hardcoded maps from pareto_router.go

### 8.3 Medium-term (This Quarter)

1. Add web scrapers for LMSYS/Vellum
2. Build thegent adapter
3. Add agentapi++ adapter

### 8.4 Long-term

1. Full hexagonal architecture
2. Real-time telemetry
3. A/B testing framework

---

## 9. Validation Commands

```bash
# Validate ledger
task ledger:sql:validate

# Run tests
cargo test && pytest

# Lint
cargo clippy && ruff check .

# Integration
task ingest:proxyapi:validate
task orchestrate:proxyapi:validate
```

---

## 10. Open Questions

1. Should tokenledger be a separate binary or a library?
2. How to handle conflicting benchmark data from multiple sources?
3. What's the refresh cadence for benchmark data?
4. How to handle provider API key rotation?

---

## 11. References

- `tokenledger/docs/plans/UNIFIED_MODEL_PROVIDER_LEDGER_PLAN_2026-02-21.md`
- `thegent/docs/governance/POLYGLOT_RUNTIME_COVERAGE_AND_CONVERSION_MATRIX_2026-02-21.md`
- `thegent/docs/research/CODEX_HARNESS_OVERHAUL_DESIGN_V2.md`
- `thegent/docs/reference/api/harness_model_mapping_api.md`
- Artificial Analysis API Documentation
- OpenRouter API Documentation
