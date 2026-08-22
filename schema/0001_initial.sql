-- Tokn L14 Data Layer — Initial Schema
-- Pricing book + cost ledger + model metadata
-- Migration: 0001_initial.sql

-- Models registry: canonical model identifiers and metadata.
CREATE TABLE IF NOT EXISTS models (
    id           TEXT PRIMARY KEY,
    provider     TEXT NOT NULL,
    display_name TEXT NOT NULL,
    created_at   INTEGER NOT NULL,
    updated_at   INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_models_provider ON models(provider);

-- Pricing book: per-model input/output token prices (USD per 1k tokens).
CREATE TABLE IF NOT EXISTS pricing_book (
    model_id      TEXT NOT NULL,
    effective_at  INTEGER NOT NULL,
    input_usd_1k  REAL NOT NULL CHECK (input_usd_1k >= 0),
    output_usd_1k REAL NOT NULL CHECK (output_usd_1k >= 0),
    currency      TEXT NOT NULL DEFAULT 'USD',
    PRIMARY KEY (model_id, effective_at),
    FOREIGN KEY (model_id) REFERENCES models(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_pricing_model_effective
    ON pricing_book(model_id, effective_at DESC);

-- Cost ledger: append-only record of every cost calculation.
CREATE TABLE IF NOT EXISTS cost_ledger (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    model_id      TEXT NOT NULL,
    input_tokens  INTEGER NOT NULL CHECK (input_tokens >= 0),
    output_tokens INTEGER NOT NULL CHECK (output_tokens >= 0),
    cost_usd      REAL NOT NULL CHECK (cost_usd >= 0),
    computed_at   INTEGER NOT NULL,
    note          TEXT,
    FOREIGN KEY (model_id) REFERENCES models(id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_model_time
    ON cost_ledger(model_id, computed_at DESC);

CREATE INDEX IF NOT EXISTS idx_ledger_computed_at
    ON cost_ledger(computed_at DESC);

-- Migration ledger: track applied migrations.
CREATE TABLE IF NOT EXISTS _migrations (
    id          TEXT PRIMARY KEY,
    applied_at  INTEGER NOT NULL
);

-- Seed: ensure migration tracking is bootstrapped.
INSERT OR IGNORE INTO _migrations (id, applied_at)
    VALUES ('0001_initial', strftime('%s','now') * 1000);
