# Merged Fragmented Markdown

## Source: contracts/NORMALIZED_EVENT_SCHEMA_CONTRACT_V1.md

# Normalized Event Schema Contract v1

Status: Active  
Contract version: `1`

## Scope

This contract defines the normalized JSONL event shape used by `monthly`, `daily`, `coverage`, `pricing-check`, `pricing-reconcile`, `bench`, and `orchestrate`.

## Artifact form

- Format: JSON Lines (`.jsonl`)
- Encoding: UTF-8
- One JSON object per line

## Required event shape (v1)

```json
{
  "provider": "claude",
  "model": "claude-sonnet-4-5",
  "session_id": "abc",
  "timestamp": "2026-02-19T22:12:00Z",
  "usage": {
    "input_tokens": 1200,
    "output_tokens": 800,
    "cache_write_tokens": 400,
    "cache_read_tokens": 3200,
    "tool_input_tokens": 0,
    "tool_output_tokens": 0
  }
}
```

## Field contract

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `provider` | string | yes | Provider key (alias values are allowed on input; canonicalized against pricing aliases before filtering/costing). |
| `model` | string | yes | Model key (alias values are allowed on input; canonicalized against provider model aliases before filtering/costing). |
| `session_id` | string | yes | Logical session identifier used in session counts and dedupe keys. |
| `timestamp` | RFC3339 datetime string | yes | Must parse as UTC datetime (`DateTime<Utc>`). |
| `usage` | object | yes | Token usage payload (see subfields below). |
| `usage.input_tokens` | non-negative integer | yes | Input prompt tokens. |
| `usage.output_tokens` | non-negative integer | yes | Output/completion tokens. |
| `usage.cache_write_tokens` | non-negative integer | yes | Cache write tokens. |
| `usage.cache_read_tokens` | non-negative integer | yes | Cache read tokens. |
| `usage.tool_input_tokens` | non-negative integer | yes | Tool input tokens. |
| `usage.tool_output_tokens` | non-negative integer | yes | Tool output tokens. |

## Invariants

1. `usage_total_tokens` is defined as:
`input_tokens + output_tokens + cache_write_tokens + cache_read_tokens + tool_input_tokens + tool_output_tokens`.
2. Consumers must treat the payload as append-only for unknown keys (ignore unknown fields).
3. Empty event sets after month/provider/model filters are treated as an execution error for report/snapshot generation.

## Compatibility policy

1. `v1` is additive-forward-compatible:
- adding optional fields is non-breaking,
- adding top-level metadata fields is non-breaking.
2. Breaking changes require `v2`:
- renaming/removing required fields,
- changing numeric semantics,
- changing timestamp format away from RFC3339 UTC.

## Producer/consumer responsibilities

1. Producers (ingest adapters) should emit the exact required keys above.
2. Consumers should not rely on field order.
3. Downstream UI/extensions should consume normalized artifacts, not provider-native raw logs.

---

## Source: contracts/UI_SNAPSHOT_SCHEMA_CONTRACT_V1.md

# UI Snapshot Schema Contract v1

Status: Active  
Contract version: `1`

## Scope

This contract defines the JSON payload written by:

`tokenledger orchestrate --ui-snapshot-path <path>`

It is intended for file-based extension and statusbar integrations (CodexBar/OpenCode style).

## Artifact form

- Format: JSON (`.json`)
- Encoding: UTF-8
- Producer: `orchestrate` command
- Current `schema_version`: `1`

## Required shape (v1)

```json
{
  "schema_version": 1,
  "generated_at": "2026-02-21T02:00:00Z",
  "month": "2026-02",
  "mode": "compact",
  "totals": {
    "cost_usd": 2.0,
    "tokens": 200000,
    "blended_usd_per_mtok": 10.0,
    "session_count": 3,
    "skipped_unpriced_count": 1
  },
  "top_providers": [
    {
      "name": "provider-a",
      "tokens": 120000,
      "total_cost_usd": 1.4,
      "blended_usd_per_mtok": 11.67,
      "session_count": 2
    }
  ],
  "top_models": [
    {
      "name": "model-a",
      "tokens": 150000,
      "total_cost_usd": 1.6,
      "blended_usd_per_mtok": 10.67,
      "session_count": 2
    }
  ],
  "suggestions": [
    "tip"
  ],
  "reconcile_latest_summary_path": "benchmarks/results/reconcile-latest-summary.json"
}
```

## Field contract

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `schema_version` | integer | yes | Compatibility gate for consumers. |
| `generated_at` | RFC3339 datetime string | yes | Snapshot generation timestamp (UTC). |
| `month` | string | yes | Snapshot month in `YYYY-MM`. |
| `mode` | enum | yes | `compact` or `extended`. |
| `totals` | object | yes | Aggregate cost/token/session metrics. |
| `top_providers` | array | yes | Provider-level rows sorted by token volume. |
| `top_models` | array | yes | Model-level rows sorted by token volume. |
| `suggestions` | array of string | yes | Optimization suggestions from report pipeline. |
| `reconcile_latest_summary_path` | string | no | Present when latest reconcile summary pointer exists. |

`totals` fields:
- `cost_usd` (number)
- `tokens` (non-negative integer)
- `blended_usd_per_mtok` (number)
- `session_count` (non-negative integer)
- `skipped_unpriced_count` (non-negative integer)

Row fields in `top_providers[]` and `top_models[]`:
- `name` (string)
- `tokens` (non-negative integer)
- `total_cost_usd` (number)
- `blended_usd_per_mtok` (number)
- `session_count` (non-negative integer)

## Mode semantics

1. `compact`:
- `top_providers` and `top_models` are top-N slices (currently N=5).
2. `extended`:
- `top_providers` and `top_models` include full breakdowns.

## Compatibility policy

1. Consumers must hard-check `schema_version == 1` before strict field assumptions.
2. Additive fields in `v1` are non-breaking; consumers must ignore unknown keys.
3. `mode` additions are non-breaking if existing values and field semantics are preserved.
4. Any of the following requires a major contract bump (`schema_version: 2`):
- required field removal/rename,
- type change for existing fields,
- semantic redefinition of totals or row metrics.

## Consumer guidance

1. Prefer fail-open behavior for missing file/temporary parse errors (show stale/empty UI state).
2. Use atomic read strategy (`readFile` + parse; retry on parse error).
3. If `schema_version` is unsupported, surface a clear integration error and ignore payload contents.

---
