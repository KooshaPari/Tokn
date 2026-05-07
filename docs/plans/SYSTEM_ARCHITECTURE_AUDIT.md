# Unified System Architecture Audit & Optimization Plan

**Date:** 2026-02-24
**Status:** Draft

---

## Executive Summary

This document audits the tokenledger, CLIProxyAPI, thegent, agentapi++, and heliosHarness ecosystems to determine optimal hexagonal/microservice architecture patterns.

### Current State: Massive Overlap

| Component | Routing | Benchmarks | Pricing | Providers | Models |
|-----------|---------|------------|---------|-----------|---------|
| **tokenledger** | ❌ | ✅ (new) | ✅ | ✅ | ✅ |
| **cliproxy++** | ✅ Pareto | ⚠️ hardcoded | ✅ | ✅ | ✅ |
| **thegent** | ✅ Multiple | ✅ quality/speed/cost | ✅ | ✅ | ✅ |
| **agentapi++** | ❌ | ❌ | ❌ | ⚠️ | ❌ |
| **heliosHarness** | ⚠️ | ✅ runners | ❌ | ✅ adapters | ✅ |

---

## 1. Component Audit

### 1.1 tokenledger (Rust)

**Current Scope:**
- Pricing book management
- Cost calculation
- Benchmark store (new - 60+ metrics)
- Model/provider ledger (56 rows)
- CLI commands

**Routing Functions:**
- None currently

**Gaps:**
- No runtime routing logic
- No provider adapters
- No agent/harness integration

### 1.2 CLIProxyAPI (Go)

**Current Scope:**
- Multi-provider proxy (OpenAI, Anthropic, Google, etc.)
- Pareto router (hardcoded maps)
- Model definitions
- Rankings endpoint

**Routing Functions:**
- `pareto_router.go` - hardcoded quality/cost/latency maps
- Auto-routing
- Provider failover

**Gaps:**
- Static benchmark data (25 models)
- No live benchmark integration
- No agentic metrics

### 1.3 thegent (Python/Rust)

**Current Scope:**
- Agent orchestration
- Multiple routing strategies (pareto, cost-aware, hybrid, etc.)
- Quality/speed/cost indices
- Model catalog
- Provider adapters

**Routing Functions:**
- `utils/routing_impl/pareto_router.py`
- `utils/routing_impl/cost_aware_router.py`
- `utils/routing_impl/hybrid_router.py`
- `orchestration/execution/router.py`
- Multiple provider adapters

**Gaps:**
- Scattered routing logic across files
- Duplicated quality/speed/cost with tokenledger
- No unified benchmark source

### 1.4 agentapi++ (Go)

**Current Scope:**
- Agent SDK
- Message formats
- Agent lifecycle

**Routing Functions:**
- None

**Gaps:**
- No routing
- No pricing
- No benchmarks

### 1.5 heliosHarness (Python/Rust)

**Current Scope:**
- Benchmark runners
- Provider adapters
- SLA tracking

**Routing Functions:**
- Some harness-specific routing
- Provider registry

**Gaps:**
- Not integrated with tokenledger
- Duplicates CLIProxyAPI routing

---

## 2. Overlap Analysis

### 2.1 Duplicated Components

| Function | In tokenledger | In thegent | In CLIProxy | In heliosHarness |
|----------|----------------|------------|-------------|-----------------|
| Model definitions | ✅ | ✅ | ✅ | ⚠️ |
| Quality scores | New | ✅ | ❌ | ❌ |
| Speed/latency | New | ✅ | ⚠️ | ❌ |
| Pricing | ✅ | ✅ | ✅ | ❌ |
| Pareto routing | New | ✅ | ✅ | ❌ |
| Provider adapters | ❌ | ✅ | ✅ | ✅ |
| Benchmark runners | ❌ | ❌ | ❌ | ✅ |

### 2.2 Data Flow Issues

```
thegent → quality/speed/cost (Python)
       ↓ (manual sync
tokenledger → benchmark data
       ↓ (missing
CLIProxyAPI → hardcoded maps (25 models)
       ↓ (static
Pareto router → routing decisions
```

**Problem:** No live data flow between systems

---

## 3. Proposed Hexagonal Architecture

### 3.1 Core Domain (tokenledger)

```
┌─────────────────────────────────────────────────────────────┐
│                    CORE DOMAIN (Rust)                      │
│  ┌─────────────────────────────────────────────────┐   │
│  │  BenchmarkStore (canonical source of truth)        │   │
│  │  - 60+ metrics per model                       │   │
│  │  - Priority merge (AA > OR > Manual > Fallback   │   │
│  └─────────────────────────────────────────────────┘   │
│                         ▲                           │
│                         │ data                      │
│  ┌──────────────────────┼───────────────────────┐    │
│  │            PORTS (Interfaces)                 │    │
│  │  • BenchmarkPort                              │    │
│  │  • PricingPort                               │    │
│  │  • ProviderPort                            │    │
│  │  • ModelPort                               │    │
│  └─────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────┘
```

### 3.2 Adapters (Per System)

```
┌─────────────────────────────────────────────────────────────┐
│                      ADAPTERS                            │
├───────────────────────────────────────────────────────┬──┤
│ tokenledger                             │ CLIProxy        │
│  • artificial_analysis.rs               │ • pareto_router.go│
│  • openrouter.rs                      │ • provider_*.go  │
│  • cliproxy_metrics.rs (new)         │                 │
│  • thegent_adapter.rs                │                 │
├─────────────────────────────────────┼────────────────┤
│ thegent                              │ heliosHarness  │
│  • routing_impl/*                    │ • runners/*      │
│  • quality/speed/cost (port)        │ • providers/*   │
│  • catalog.py (port)               │                 │
├─────────────────────────────────────┴────────────────┤
│ agentapi                                                 │
│  • (no routing - uses CLIProxy)                           │
└─────────────────────────────────────────────────────────┘
```

### 3.3 Key Architectural Decisions

| Decision | Rationale |
|----------|------------|
| **Single benchmark source (tokenledger)** | DRY - eliminate quality/speed/cost duplication |
| **Adapters per system** | HEXAGONAL - swap implementations |
| **CLIProxy reads from tokenledger** | INTEGRATION - live data, not static maps |
| **thegent uses tokenledger** | DEPENDENCY INVERSION - port interface |
| **agentapi stays thin** | KISS - no routing logic needed |
| **heliosHarness writes to tokenledger** | SINGLE DIRECTION - benchmarks flow in |

---

## 4. Module Split Plan

### 4.1 New Modules to Create

| Module | Location | Responsibility |
|--------|----------|----------------|
| `tokenledger-benchmarks` | tokenledger/src/benchmarks/ | DONE ✅ |
| `tokenledger-cli` | tokenledger/src/cli.rs | DONE ✅ |
| `cliproxy-tokenledger-client` | cliproxy++/pkg/llmproxy/benchmarks/ | NEW |
| `thegent-tokenledger-adapter` | thegent/adapters/tokenledger.py | NEW |
| `helios-benchmarks-adapter` | heliosHarness/adapters/tokenledger.py | NEW |

### 4.2 Modules to Consolidate

| From | To | Rationale |
|-------|-----|------------|
| thegent/utils/routing_impl/quality_values.py | tokenledger | DRY |
| thegent/models/speed_values.py | tokenledger | DRY |
| cliproxy++/pkg/llmproxy/registry/pareto_router.go | tokenledger integration | DRY |
| CLIProxy hardcoded maps | tokenledger | LIVE DATA |
| heliosHarness benchmarks | tokenledger | SINGLE SOURCE |

### 4.3 Modules to Keep Separate

| Module | Reason |
|--------|---------|
| agentapi++ | Thin SDK, no routing |
| heliosHarness runners | Actual benchmark execution |
| thegent orchestration | Agent logic |
| CLIProxy proxy | API forwarding |

---

## 5. Implementation Phases

### Phase 1: Core Infrastructure ✅ DONE
- [x] tokenledger benchmarks module
- [x] CLI commands
- [x] Integration stub in CLIProxy

### Phase 2: Integration (Current)
- [ ] CLIProxy reads from tokenledger
- [ ] thegent adapter
- [ ] heliosHarness adapter

### Phase 3: Consolidation
- [ ] Move quality/speed/cost to tokenledger
- [ ] Remove duplicates
- [ ] Verify data flow

### Phase 4: Polish
- [ ] Remove hardcoded maps
- [ ] Add tests
- [ ] Documentation

---

## 6. Data Flow After Optimization

```
┌──────────────────────────────────────────────────────────────┐
│                      DATA FLOW                              │
│                                                       │
│  AA API ──► tokenledger ◄── OpenRouter API            │
│      │              │                │                     │
│      │              │                │                     │
│      │              ▼                │                     │
│      │      BenchmarkStore ◄──────┘                     │
│      │              │                                    │
│      │              │ read                               │
│      ▼              ▼                                    │
│ CLIProxy ◄── tokenledger ◄── thegent                  │
│ Pareto           adapter       adapter                     │
│ router                                                    │
│      │              │                                    │
│      │              ▼                                   │
│      │       heliosHarness                            │
│      │       benchmarks                              │
│      ▼                                             │
│ Requests ──► providers                                │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Principles Applied

| Principle | Application |
|-----------|-------------|
| **DRY** | Single benchmark source (tokenledger) |
| **KISS** | Simple adapters per system |
| **SOLID** | Port/adapter pattern |
| **HEXAGONAL** | Clear boundaries |
| **POLYGLOT** | Best language per job |
| **MICROSERVICE** | Independent deployability |

---

## 8. Open Questions

1. Should tokenledger be a library or binary?
2. How to handle real-time vs batch benchmark updates?
3. What's the refresh cadence for benchmarks?
4. How to version benchmark data?
5. Rate limiting strategy for APIs?

---

## 9. References

- tokenledger/docs/plans/UNIFIED_ROUTING_ORCHESTRATION_PLAN.md
- thegent/docs/governance/POLYGLOT_RUNTIME_...md
- CLIProxy++ pkg/llmproxy/registry/pareto_router.go
