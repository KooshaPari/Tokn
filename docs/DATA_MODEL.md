# Tokn Data Model

Tokn's persistent layer is a single SQLite database (`tokn.db` by default) with
three core tables plus a migration tracker.

## Schema Overview

```
┌─────────────┐         ┌──────────────────┐         ┌──────────────┐
│   models    │ 1───n   │  pricing_book    │         │ cost_ledger  │
├─────────────┤         ├──────────────────┤         ├──────────────┤
│ id (PK)     │◄────────│ model_id (FK)    │         │ id (PK)      │
│ provider    │         │ effective_at     │         │ model_id (FK)│
│ display_name│         │ input_usd_1k     │         │ input_tokens │
│ created_at  │         │ output_usd_1k    │         │ output_tokens│
│ updated_at  │         │ currency         │         │ cost_usd     │
└─────────────┘         │ (model_id,       │         │ computed_at  │
                        │  effective_at)PK │         │ note         │
                        └──────────────────┘         └──────────────┘
```

Migration tracking is held in `_migrations(id, applied_at)`.

## Table Specifications

### `models`

Canonical model identifier registry.

| Column        | Type    | Notes                                        |
|---------------|---------|----------------------------------------------|
| `id`          | TEXT    | PK — e.g. `claude-opus-4`, `gpt-5`           |
| `provider`    | TEXT    | `anthropic`, `openai`, `google`, …           |
| `display_name`| TEXT    | Human-readable label                          |
| `created_at`  | INTEGER | ms epoch                                     |
| `updated_at`  | INTEGER | ms epoch                                     |

### `pricing_book`

Append-only price history. Each row captures the *effective* price for a model
starting at `effective_at`. To change a price, insert a new row with a later
`effective_at`.

| Column          | Type    | Notes                                    |
|-----------------|---------|------------------------------------------|
| `model_id`      | TEXT    | FK → `models.id`                         |
| `effective_at`  | INTEGER | ms epoch; PK with `model_id`             |
| `input_usd_1k`  | REAL    | USD per 1,000 input tokens (≥ 0)         |
| `output_usd_1k` | REAL    | USD per 1,000 output tokens (≥ 0)        |
| `currency`      | TEXT    | ISO 4217 (default `USD`)                 |

### `cost_ledger`

Append-only cost calculation history. One row per cost call.

| Column          | Type    | Notes                                    |
|-----------------|---------|------------------------------------------|
| `id`            | INTEGER | autoincrement PK                         |
| `model_id`      | TEXT    | FK → `models.id`                         |
| `input_tokens`  | INTEGER | (≥ 0)                                    |
| `output_tokens` | INTEGER | (≥ 0)                                    |
| `cost_usd`      | REAL    | (≥ 0); computed using `pricing_book`     |
| `computed_at`   | INTEGER | ms epoch                                 |
| `note`          | TEXT    | Optional context (request id, etc.)      |

## Migrations

All schema changes go in `schema/NNNN_description.sql`. Apply them with:

```bash
python -m migrations.runner apply
```

The runner is idempotent — re-running is safe. Each migration's `id` is its
filename stem (e.g. `0001_initial.sql` → `0001_initial`).

## Query Helpers

`src/queries.py` exposes three safe, typed helpers over the schema:

- `get_pricing(conn, model_id, as_of=None) -> PricingSnapshot | None`
  Returns the latest pricing snapshot for a model at or before `as_of`.
- `record_cost(conn, *, model_id, input_tokens, output_tokens, cost_usd, note=None) -> int`
  Inserts one ledger row; returns the new id.
- `summarize_spend(conn, *, model_id, since_ms, until_ms=None) -> SpendSummary`
  Aggregates spend over an inclusive time window.

Each helper takes an explicit `sqlite3.Connection` so callers control
transactions and connection lifetime — no module-level globals.

## Adding a New Table

1. Create `schema/0002_add_your_table.sql` with `CREATE TABLE IF NOT EXISTS`.
2. Add a typed helper in `src/queries.py`.
3. Run `python -m migrations.runner apply`.
4. Update this doc.

## Testing

```bash
python -c "
import sqlite3, tempfile
from migrations.runner import apply_migrations
from src.queries import get_pricing, record_cost, summarize_spend

with tempfile.TemporaryDirectory() as d:
    db = f'{d}/t.db'
    apply_migrations(type('P', (), {})())  # placeholder, see tests/
"
```

See `tests/test_queries.py` for a full smoke test that exercises the runner,
helpers, and rollback path.
