# PLAN - tokenledger

## Phase 1: Bootstrap Core

- [x] P1.1 Define normalized event schema and pricing schema.
- P1.2 Implement monthly aggregation and blended cost formulas.
- P1.3 Implement table/json output and suggestion engine.

### P1.1 Schema Contract

The normalized event contract is represented by `UsageEvent` in `src/models.rs`.
Every provider adapter must emit:

- `provider`: canonical provider identifier (`claude`, `codex`, `cursor`, or `droid`)
- `model`: canonical model identifier after provider alias resolution
- `session_id`: stable source-session identifier
- `timestamp`: RFC 3339 timestamp normalized to UTC
- `usage`: `TokenUsage` with independent non-negative counts for input, output,
  cache-write, cache-read, tool-input, and tool-output tokens

The pricing contract is represented by `PricingBook` and `ModelRate` in
`src/models.rs` and serialized by `pricing.example.json`:

- provider-level monthly subscription cost and model aliases
- model-level input/output rates in USD per million tokens
- optional cache-write, cache-read, tool-input, and tool-output rates
- missing rates remain explicit and are handled by the configured unpriced-event
  policy rather than silently treated as zero

`models_normalized.csv` and `models_schema_seed.sql` are the deterministic
normalized benchmark/pricing catalog inputs. P1.1 is complete when adapters can
produce the event shape above and pricing lookup can resolve a canonical
`provider:model` pair without changing token-count semantics.

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
