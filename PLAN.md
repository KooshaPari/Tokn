# PLAN - tokenledger

## Phase 1: Bootstrap Core

- P1.1 Define normalized event schema and pricing schema.
- P1.2 Implement monthly aggregation and blended cost formulas.
- P1.3 Implement table/json output and suggestion engine.

## Phase 2: Provider Adapters

- P2.1 Claude adapter (`~/.claude/projects`) -> normalized events.
- P2.2 Codex adapter (`~/.codex/sessions`) -> normalized events.
- P2.3 Cursor adapter (SQLite + logs) -> normalized events.
- P2.4 Droid adapter (session logs) -> normalized events.

## Phase 3: Real-Time Runtime

- P3.1 Incremental tailing (file watchers / checkpoint offsets).
- P3.2 Sliding window metrics (5m/1h/24h).
- P3.3 Budget guardrails (per-model/provider burn-rate alerts).

## Dependencies (DAG)

- P1.1 -> P1.2 -> P1.3 -> P2.1/P2.2/P2.3/P2.4 -> P3.1 -> P3.2 -> P3.3
