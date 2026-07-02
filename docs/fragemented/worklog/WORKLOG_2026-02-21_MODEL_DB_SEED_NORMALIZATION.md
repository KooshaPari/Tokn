# Worklog 2026-02-21: Model DB Seed Normalization

## Context

`tokenledger/models.csv` was user-pasted baseline data but not valid CSV. It is a markdown-style pipe table and must be treated as text input.

## Completed

1. Added reproducible generator:
- `scripts/build_model_seed.py`

2. Generated review/query artifacts from `models.csv`:
- `models_normalized.csv` (valid long-format CSV)
- `models_schema_seed.sql` (DDL + inserts for SQL workflows)

3. Added README instructions under:
- `Model Database Seed (CSV + SQL)`

## Normalization rules

1. Missing tokens: `''`, `—`, `-`, `N/A`, `n/a`, `na`, `NA`, `null`, `NULL`.
2. Split mixed values once on `/` into `value_primary` + `value_secondary`.
3. Preserve raw source token in `raw_value`.
4. Track provenance columns: `source_row_index`, `source_col_index`, `source_col_name`.

## Validation

1. SQL seed loads in SQLite memory DB.
2. Inserted rows match normalized CSV row count.
