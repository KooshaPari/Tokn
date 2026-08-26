# Tokn Migration Guide

## Overview

Tokn uses a forward-only SQL migration system to manage schema changes
across the `pricing_book`, `cost_ledger`, and `_migrations` tracking table.

## Migration Files

| Version | File | Description |
|---------|------|-------------|
| 0001 | `schema/0001_initial.sql` | Core: pricing_book + cost_ledger + _migrations |
| 0002 | `schema/0002_add_budget_alerts.sql` | Budget alerts + spending thresholds |
| 0003 | `schema/0003_add_routing_history.sql` | Routing decision history table |

## Running Migrations

```bash
# Apply all pending migrations
python migrations/runner.py --db pricing_book.db

# Check migration status
python migrations/runner.py --status

# Rollback last migration
python migrations/runner.py --rollback --db pricing_book.db
```

## Writing New Migrations

1. Create a new SQL file: `schema/NNN_description.sql`
2. Follow the convention: `CREATE TABLE IF NOT EXISTS` for idempotency
3. Add a `_migrations` insert at the end
4. Test rollback by reversing your DDL

## Schema Conventions

- All timestamps use ISO 8601 strings
- IDs are `INTEGER PRIMARY KEY AUTOINCREMENT`
- JSON fields stored as `TEXT`
- Foreign keys enforced via `REFERENCES`
- Indexes created with `IF NOT EXISTS`
