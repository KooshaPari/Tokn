# Router Contract (T35)

**Status:** Accepted (2026-06-20) · **Source:** T35 — `helios_router` → `Tokn` consolidation · **Scope:** `tokenledger::routing::*`

---

## Purpose

The `routing` module is the **canonical Rust router substrate** for the Phenotype fleet. It selects the optimal LLM offer (provider + model) for a request given quality, cost, speed, and routing-policy constraints. Tokn owns the substrate; downstream consumers (CLIProxyAPI, HeliosHarness, thegent, agentapi) plug in via the hexagonal **port + adapter** boundary.

## Public Surface

```rust
use tokenledger::routing::{
    // Pareto frontier (T35 port from helios_router/pareto/engine.py)
    ParetoOffer, ParetoObjective, ParetoResult, ParetoCombo,
    pareto_front_mask, compute_pareto, compute_combos,

    // Weighted-sum scorer (pre-existing)
    ParetoRouter,

    // Ports (traits)
    BenchmarkPort, MetricsPort, RoutingPort, ModelMappingPort, TrioPort,

    // Types
    RoutingCriteria, RoutingDecision, RoutingAlternative,
    ModelMapping, ProviderHarnessModel,
    PortError, PortResult,
};
```

## Pareto Frontier Algorithm (T35 addition)

The frontier is computed in **O(N²)** using a generalized non-dominated sort over an arbitrary set of `(attribute_name, Minimize|Maximize)` pairs. An offer A **dominates** B iff A is no worse on every objective AND strictly better on at least one.

| Function | Purpose | Origin |
|----------|---------|--------|
| `pareto_front_mask(offers, objectives)` | Returns `Vec<bool>` parallel to `offers`; `true` = non-dominated. | `helios_router/pareto/engine.py::pareto_front_mask` |
| `compute_pareto(offers, min_cost, min_speed, max_quality)` | Defaults to `(cost_usd ↓, speed_score ↓, quality ↑)`; returns `ParetoResult` with frontier indices. | `helios_router/pareto/engine.py::compute_pareto` |
| `compute_combos(offers, k)` | Returns all k-sized subsets with aggregate (mean quality, sum cost, min speed). | `helios_router/pareto/engine.py::compute_combos` |

**Sentinel handling:** Missing attributes are treated as worst-case (`+∞` for `Minimize`, `-∞` for `Maximize`), so an offer with a missing field never dominates one that has it.

## Adapter Boundary

Each external data source implements the port traits. Tokn ships these adapters out-of-the-box:

| Adapter | Port(s) | Source |
|---------|---------|--------|
| `UnifiedAdapter` | `BenchmarkPort` + `MetricsPort` | Combined stores |
| `CLIProxyAdapter` | `BenchmarkPort` + `MetricsPort` | CLIProxyAPI runtime metrics |
| `HeliosHarnessAdapter` | `BenchmarkPort` | Helios benchmark JSON |
| `ThegentRoutingAdapter` | `RoutingPort` | thegent router signals |
| `AgentAPIAdapter` | `BenchmarkPort` + `ModelMappingPort` | AgentAPI model registry |

## Routing Decision Flow

```
RoutingCriteria ──► RoutingPort::select(criteria)
                          │
                          ├─► fetch benchmarks (BenchmarkPort)
                          ├─► apply Pareto frontier filter  ◄── T35
                          ├─► apply RoutingCriteria filter
                          ├─► rank by weighted Pareto score
                          └─► RoutingDecision { model, provider, confidence, alternatives }
```

## Migration Notes (T35)

- **Migrated:** `pareto/engine.py` (Pareto mask + compute_pareto + compute_combos) → Rust port at `src/routing/pareto_frontier.rs`. Function signatures are 1:1 with the Python source modulo idiomatic Rust.
- **Not migrated (documented gaps for follow-up):**
  - `nats_client.py` — NATS JetStream event bus. Application-layer messaging, not router decision logic. Belongs in `pheno-events` or a future federation substrate per ADR-035B.
  - `db/schema.py` — SQLite schema for offers/providers/roles/benchmarks. Application-layer persistence. Belongs in a downstream dashboard substrate, not the router core.
  - `ui/components.py` — Streamlit dashboard widgets. UI layer; out of scope for the Rust router substrate.

## Testing

Unit tests live in `pareto_frontier::tests` (11 cases covering empty input, no objectives, dominated offers, missing-attribute sentinels, default and quality-only objectives, pairs/trios/empty combos, attribute lookup, and a canonical 5-offer scenario). Run with `cargo test -p tokenledger routing::pareto_frontier`.

## Quality Bar (per ADR-040)

- ✅ Spec (`SPEC_ROUTER.md`, this file — 1 page max)
- ✅ Unit tests (11 cases, all pass)
- ✅ Hexagonal port-adapter boundary (per ADR-038)
- ⏳ Integration test against CLIProxyAPI / HeliosHarness — pending adapter wiring (T35 deliverable is the algorithm; wiring is per-consumer)