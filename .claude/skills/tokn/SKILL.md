---
name: tokn-ops
description: Use for token/session usage analysis, model pricing governance, and cost analytics for AI coding agents via the Tokn (tokenledger) CLI — monthly/daily usage, coverage, pricing check/apply/reconcile/lint/audit, ingest, and benchmarks.
---

# Tokn (tokenledger) operations skill

## When to invoke
Use this skill when the user wants to inspect AI-agent token usage or cost, govern model pricing, or run usage/cost benchmarks — anything involving the `tokenledger` binary.

## Binary
`E:\cargo-target\release\tokenledger.exe` (Rust). Run `tokenledger --help` for the full surface.

## Commands
- `monthly` / `daily` — usage + blended cost over a period.
- `coverage` — usage coverage analysis.
- `pricing-check` — verify current model pricing against the ledger.
- `pricing-apply` — apply pricing updates.
- `pricing-reconcile` — reconcile pricing discrepancies.
- `pricing-lint` — lint the pricing config.
- `pricing-audit` — audit pricing history.
- `ingest` — ingest usage data.
- `bench` / `benchmarks` — run cost/usage benchmarks.
- `orchestrate` — orchestration helpers.

## Typical flow
1. `tokenledger ingest <source>` to load usage.
2. `tokenledger monthly` / `daily` for the spend picture.
3. `tokenledger pricing-check` then `pricing-reconcile`/`pricing-apply` to keep pricing accurate.
4. `tokenledger bench` to surface the few calls driving most spend (pareto).
