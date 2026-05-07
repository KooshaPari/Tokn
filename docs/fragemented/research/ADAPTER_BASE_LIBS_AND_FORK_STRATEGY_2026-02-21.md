# Adapter Base Libs and Fork Strategy (2026-02-21)

## Decision

Do not fork a single monolithic Rust "ccusage-equivalent" that attempts to own Claude + Codex + Cursor + Droid end-to-end.

Adopt a split strategy:

1. Keep `tokenledger` as the canonical core (normalized schema, pricing, reconciliation, formulas, benchmark gates, UI snapshot artifacts).
2. Use provider-specific adapters as thin import/fork units per source.
3. Prefer upstream library import first; if missing, fork only the parser/adapter slice (not UI/CLI monoliths).

## Why this is the right architecture

1. Provider surfaces differ materially (auth/session, log shape, local storage format, CLI semantics).
2. A monolith fork couples unrelated churn and slows update velocity.
3. Per-provider adapters preserve blast radius and make provider updates incremental.
4. `tokenledger` remains the stable integration point for cross-provider blended cost analytics.

## Web market scan summary (practical starting points)

### Aggregators / UX references

1. OpenUsage (plugin-style multi-provider tracker): `https://www.openusage.ai/`
2. CodexBar (menu bar + multi-provider integrations): `https://github.com/steipete/CodexBar`
3. opencode (UI/UX reference for low-overhead coding workflows): `https://github.com/sst/opencode`

### Provider-oriented usage parsers / trackers

1. ccusage (existing baseline behavior and UX to match/improve): `https://github.com/ryoppippi/ccusage`
2. ccstat (Rust Claude usage/cost parsing): `https://crates.io/crates/ccstat`
3. cxusage (Go Codex usage parser/reporting): `https://github.com/johanneserhardt/cxusage`
4. tokscale (Rust multi-provider usage/cost tracker): `https://github.com/junhoyeo/tokscale`

Inference: useful pieces exist, but there is no single mature Rust monolith that robustly covers Claude + Codex + Cursor + Droid together with high-quality adapter depth and stable governance. Split adapters are still the lower-risk path.

## Adapter contract (required)

Each provider adapter must emit normalized JSONL records directly consumable by `tokenledger`:

1. `event_ts`
2. `provider`
3. `model_raw`
4. `model_canonical`
5. `session_id`
6. `request_id` (if available)
7. `input_tokens`
8. `output_tokens`
9. `cache_write_tokens` / `cache_read_tokens` (if provider exposes)
10. `tool_tokens` / tool-usage metadata (if provider exposes)
11. provenance (`source_path`, `offset`, parser version)

## Fork/import policy per provider

1. Import upstream parser/lib when:
- data contract is stable,
- dependency/license is acceptable,
- integration surface is small.

2. Fork adapter slice when:
- parser quality is good but tightly coupled to its own CLI/UI,
- only a narrow extraction layer is needed.

3. Re-implement in-house when:
- upstream churn is high or project is abandonware,
- licensing blocks intended distribution,
- parser logic is smaller than integration overhead.

## Governance for imported/forked adapter slices

1. Pin every external import/fork to explicit commit/tag.
2. Keep one adapter directory per provider with:
- source attribution,
- patch log,
- update script/checklist,
- fixture corpus.
3. Run adapter conformance tests into normalized golden JSONL.
4. Enforce bench gates on:
- cold backfill,
- warm tail,
- burst ingest.
5. Treat model-provider mapping and pricing mapping as separate audited layers.

## Performance-first implementation guidance

### SQLite / local state

1. `rusqlite` as default embedded checkpoint/index DB.
2. WAL mode + deterministic migrations.
3. Optional read-heavy pooling wrapper where license/governance permits.

### Disk cache

1. Add optional disk-backed cache layer for:
- normalized-event dedupe indexes,
- monthly aggregate materializations,
- provider model metadata snapshots.

2. Keep cache optional and bounded; no mandatory overhead for minimal runs.

### Tail/parse pipeline

1. Provider adapters should support both backfill and incremental tail.
2. Use append-safe checkpoints (`source file + inode/hash + offset`).
3. Ensure deterministic resume after restarts.

## Pricing methodology alignment (subscription + token-based providers)

1. Token-priced providers:
- direct `USD / Mtok` using configured input/output/cache/tool rates.

2. Subscription/session-priced providers:
- derive effective monthly budget:
  - either explicit user-entered monthly USD value, or
  - programmatic derivation from plan price and billing metadata.
- allocate monthly budget proportionally over measured token usage.
- compute effective blended `USD / Mtok` per model and provider.

3. Keep both in one comparable surface:
- `effective_cost_usd`
- `effective_usd_per_mtok`
- `blended_provider_usd_per_mtok`
- `blended_global_usd_per_mtok`

4. Mark derivation method in output metadata:
- `pricing_mode = token|subscription_derived|subscription_manual`

## Concrete next steps (execution order)

1. Build real adapters:
- `~/.claude`
- `~/.codex`
- Cursor DB/logs
- Droid logs

2. Emit normalized JSONL directly to `tokenledger ingest`.
3. Add fixture-backed conformance tests for each adapter.
4. Add benchmark harness perf gates and enforce in CI:
- cold backfill,
- warm tail,
- burst.

5. Add low-overhead UI integration path (statusbar/extension) using `--ui-snapshot-path`.
6. Add optional SQLite + disk-cache acceleration behind feature flags/config.

## Explicit conclusion

Yes, separate provider-focused imports/forks are preferred.

No, do not fork a full monolith.

Keep `tokenledger` as the core, and treat adapters as replaceable, benchmarked, governed modules.
