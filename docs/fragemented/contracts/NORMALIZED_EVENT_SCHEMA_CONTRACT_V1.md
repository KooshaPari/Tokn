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
