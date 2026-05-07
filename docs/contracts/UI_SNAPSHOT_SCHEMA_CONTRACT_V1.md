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
