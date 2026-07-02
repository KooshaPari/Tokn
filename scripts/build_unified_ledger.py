#!/usr/bin/env python3
"""Build deterministic unified model/provider ledger artifacts.

Inputs:
- pricing.example.json
- models_normalized.csv

Outputs (under tokenledger/ledger by default):
- unified_model_provider_ledger.csv
- unified_model_provider_ledger.schema.sql
- unified_model_provider_ledger.seed.sql
"""

from __future__ import annotations

import argparse
import csv
import json
import re
from dataclasses import dataclass
from decimal import Decimal, InvalidOperation
from pathlib import Path
from typing import Any

MISSING_TOKENS = {"", "-", "—", "n/a", "na", "null"}
PRICE_BENCHMARKS = {"Input Price", "Output Price"}

PROVIDER_RULES: list[tuple[str, str, str, int]] = [
    ("claude", "prefix:claude", "claude", 99),
    ("gpt", "prefix:gpt", "codex", 95),
    ("gemini", "prefix:gemini", "google", 95),
    ("grok", "prefix:grok", "xai", 95),
    ("qwen", "prefix:qwen", "qwen", 95),
    ("deepseek", "prefix:deepseek", "deepseek", 95),
    ("minimax", "prefix:minimax", "minimax", 95),
    ("kimi", "prefix:kimi", "kimi", 95),
    ("glm", "prefix:glm", "zhipu", 95),
    ("step", "prefix:step", "stepfun", 95),
    ("devstral", "prefix:devstral", "mistral", 95),
]


@dataclass(frozen=True)
class ModelStats:
    source_model: str
    source_model_slug: str
    benchmark_rows_total: int
    benchmark_rows_non_missing: int
    benchmark_rows_missing: int
    benchmark_prior_rows_total: int
    benchmark_prior_rows_non_missing: int
    benchmark_prior_rows_missing: int
    benchmark_distinct_total: int
    benchmark_distinct_non_missing: int
    benchmark_input_usd_per_mtok: Decimal | None
    benchmark_output_usd_per_mtok: Decimal | None


@dataclass(frozen=True)
class LedgerRow:
    ledger_row_id: int
    source_model: str
    source_model_slug: str
    inferred_provider: str
    provider_mapping_rule: str
    provider_mapping_confidence: int
    canonical_model_guess: str
    model_mapping_rule: str
    model_mapping_confidence: int
    pricing_provider: str
    pricing_model: str
    pricing_subscription_usd_month: Decimal | None
    pricing_input_usd_per_mtok: Decimal | None
    pricing_output_usd_per_mtok: Decimal | None
    pricing_cache_write_usd_per_mtok: Decimal | None
    pricing_cache_read_usd_per_mtok: Decimal | None
    pricing_tool_input_usd_per_mtok: Decimal | None
    pricing_tool_output_usd_per_mtok: Decimal | None
    benchmark_input_usd_per_mtok: Decimal | None
    benchmark_output_usd_per_mtok: Decimal | None
    benchmark_rows_total: int
    benchmark_rows_non_missing: int
    benchmark_rows_missing: int
    benchmark_prior_rows_total: int
    benchmark_prior_rows_non_missing: int
    benchmark_prior_rows_missing: int
    benchmark_distinct_total: int
    benchmark_distinct_non_missing: int
    pricing_vs_benchmark_input_delta: Decimal | None
    pricing_vs_benchmark_output_delta: Decimal | None


def normalize_text(value: str) -> str:
    lower = value.strip().lower()
    lower = re.sub(r"[^a-z0-9]+", "-", lower)
    return re.sub(r"-+", "-", lower).strip("-")


def parse_decimal(value: str) -> Decimal | None:
    raw = value.strip()
    if normalize_text(raw) in MISSING_TOKENS:
        return None
    raw = raw.replace("$", "").replace(",", "").strip()
    raw = re.sub(r"\s*\([^)]*\)", "", raw)
    raw = raw.strip()
    if not raw:
        return None
    try:
        return Decimal(raw)
    except InvalidOperation:
        return None


def decimal_to_str(value: Decimal | None) -> str:
    if value is None:
        return ""
    normalized = value.normalize()
    text = format(normalized, "f")
    if "." in text:
        text = text.rstrip("0").rstrip(".")
    return text or "0"


def sql_quote(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def sql_number(value: Decimal | None) -> str:
    if value is None:
        return "NULL"
    return decimal_to_str(value)


def load_pricing(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def build_pricing_indexes(pricing: dict[str, Any]) -> dict[str, Any]:
    providers = pricing.get("providers", {})
    index: dict[str, tuple[str, str, str]] = {}
    by_provider: dict[str, dict[str, tuple[str, str, str]]] = {}

    for provider_name, provider_cfg in providers.items():
        provider_lookup: dict[str, tuple[str, str, str]] = {}
        models = provider_cfg.get("models", {})
        model_aliases = provider_cfg.get("model_aliases", {})

        for model_name in sorted(models.keys()):
            key = normalize_text(model_name)
            mapped = (provider_name, model_name, "pricing:model_exact")
            provider_lookup[key] = mapped
            index.setdefault(key, mapped)

        for alias, canonical_model in sorted(model_aliases.items()):
            key = normalize_text(alias)
            mapped = (provider_name, canonical_model, "pricing:model_alias")
            provider_lookup[key] = mapped
            index.setdefault(key, mapped)

        by_provider[provider_name] = provider_lookup

    provider_aliases = pricing.get("provider_aliases", {})
    provider_alias_index = {normalize_text(k): v for k, v in provider_aliases.items()}

    return {
        "providers": providers,
        "global_model_index": index,
        "provider_model_index": by_provider,
        "provider_alias_index": provider_alias_index,
    }


def infer_provider(source_model: str, provider_alias_index: dict[str, str]) -> tuple[str, str, int]:
    slug = normalize_text(source_model)

    if slug in provider_alias_index:
        return provider_alias_index[slug], "pricing:provider_alias", 100

    for prefix, rule, canonical, confidence in PROVIDER_RULES:
        if slug.startswith(prefix + "-") or slug == prefix:
            return canonical, rule, confidence

    return "unknown", "heuristic:unknown", 0


def canonical_model_guess(source_model: str) -> str:
    return normalize_text(source_model)


def map_model(
    provider_guess: str,
    model_guess: str,
    pricing_idx: dict[str, Any],
) -> tuple[str, str, str, int]:
    provider_index = pricing_idx["provider_model_index"].get(provider_guess, {})
    global_index = pricing_idx["global_model_index"]

    direct_provider = provider_index.get(model_guess)
    if direct_provider:
        provider, model, rule = direct_provider
        return provider, model, rule, 100

    direct_global = global_index.get(model_guess)
    if direct_global:
        provider, model, rule = direct_global
        return provider, model, rule, 92

    if model_guess.startswith("gpt-5") and "codex" in pricing_idx["providers"]:
        codex_models = pricing_idx["providers"]["codex"].get("models", {})
        if "gpt-5" in codex_models:
            return "codex", "gpt-5", "heuristic:gpt-5-family", 78

    if model_guess.startswith("claude-sonnet-4-5") and "claude" in pricing_idx["providers"]:
        claude_models = pricing_idx["providers"]["claude"].get("models", {})
        if "claude-sonnet-4-5" in claude_models:
            return "claude", "claude-sonnet-4-5", "heuristic:claude-sonnet-4-5", 80

    return "", "", "unmapped", 0


def collect_model_stats(path: Path) -> dict[str, ModelStats]:
    grouped: dict[str, dict[str, Any]] = {}

    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for row in reader:
            model = row["model"].strip()
            benchmark = row["benchmark"].strip()
            is_missing = int(row["is_missing"])
            value_primary = row.get("value_primary", "")

            entry = grouped.setdefault(
                model,
                {
                    "source_model": model,
                    "source_model_slug": normalize_text(model),
                    "benchmark_rows_total": 0,
                    "benchmark_rows_non_missing": 0,
                    "benchmark_rows_missing": 0,
                    "benchmark_prior_rows_total": 0,
                    "benchmark_prior_rows_non_missing": 0,
                    "benchmark_prior_rows_missing": 0,
                    "benchmarks_all": set(),
                    "benchmarks_non_missing": set(),
                    "benchmark_input_usd_per_mtok": None,
                    "benchmark_output_usd_per_mtok": None,
                },
            )

            entry["benchmark_rows_total"] += 1
            entry["benchmarks_all"].add(benchmark)
            if is_missing:
                entry["benchmark_rows_missing"] += 1
            else:
                entry["benchmark_rows_non_missing"] += 1
                entry["benchmarks_non_missing"].add(benchmark)

            if benchmark not in PRICE_BENCHMARKS:
                entry["benchmark_prior_rows_total"] += 1
                if is_missing:
                    entry["benchmark_prior_rows_missing"] += 1
                else:
                    entry["benchmark_prior_rows_non_missing"] += 1

            if benchmark == "Input Price" and not is_missing:
                parsed = parse_decimal(value_primary)
                if parsed is not None:
                    entry["benchmark_input_usd_per_mtok"] = parsed

            if benchmark == "Output Price" and not is_missing:
                parsed = parse_decimal(value_primary)
                if parsed is not None:
                    entry["benchmark_output_usd_per_mtok"] = parsed

    out: dict[str, ModelStats] = {}
    for model, data in grouped.items():
        out[model] = ModelStats(
            source_model=data["source_model"],
            source_model_slug=data["source_model_slug"],
            benchmark_rows_total=data["benchmark_rows_total"],
            benchmark_rows_non_missing=data["benchmark_rows_non_missing"],
            benchmark_rows_missing=data["benchmark_rows_missing"],
            benchmark_prior_rows_total=data["benchmark_prior_rows_total"],
            benchmark_prior_rows_non_missing=data["benchmark_prior_rows_non_missing"],
            benchmark_prior_rows_missing=data["benchmark_prior_rows_missing"],
            benchmark_distinct_total=len(data["benchmarks_all"]),
            benchmark_distinct_non_missing=len(data["benchmarks_non_missing"]),
            benchmark_input_usd_per_mtok=data["benchmark_input_usd_per_mtok"],
            benchmark_output_usd_per_mtok=data["benchmark_output_usd_per_mtok"],
        )
    return out


def build_rows(stats_by_model: dict[str, ModelStats], pricing_idx: dict[str, Any]) -> list[LedgerRow]:
    providers = pricing_idx["providers"]
    rows: list[LedgerRow] = []

    for row_id, model_name in enumerate(sorted(stats_by_model.keys(), key=lambda x: normalize_text(x)), start=1):
        stats = stats_by_model[model_name]
        inferred_provider, provider_rule, provider_conf = infer_provider(
            stats.source_model,
            pricing_idx["provider_alias_index"],
        )

        model_guess = canonical_model_guess(stats.source_model)
        pricing_provider, pricing_model, model_rule, model_conf = map_model(
            inferred_provider,
            model_guess,
            pricing_idx,
        )

        provider_cfg = providers.get(pricing_provider, {}) if pricing_provider else {}
        model_cfg = provider_cfg.get("models", {}).get(pricing_model, {}) if pricing_model else {}

        subscription = provider_cfg.get("subscription_usd_month")
        subscription_dec = Decimal(str(subscription)) if subscription is not None else None

        price_input = model_cfg.get("input_usd_per_mtok")
        price_output = model_cfg.get("output_usd_per_mtok")
        price_cache_write = model_cfg.get("cache_write_usd_per_mtok")
        price_cache_read = model_cfg.get("cache_read_usd_per_mtok")
        price_tool_input = model_cfg.get("tool_input_usd_per_mtok")
        price_tool_output = model_cfg.get("tool_output_usd_per_mtok")

        pricing_input = Decimal(str(price_input)) if price_input is not None else None
        pricing_output = Decimal(str(price_output)) if price_output is not None else None
        pricing_cache_write_dec = Decimal(str(price_cache_write)) if price_cache_write is not None else None
        pricing_cache_read_dec = Decimal(str(price_cache_read)) if price_cache_read is not None else None
        pricing_tool_input_dec = Decimal(str(price_tool_input)) if price_tool_input is not None else None
        pricing_tool_output_dec = Decimal(str(price_tool_output)) if price_tool_output is not None else None

        input_delta = (
            pricing_input - stats.benchmark_input_usd_per_mtok
            if pricing_input is not None and stats.benchmark_input_usd_per_mtok is not None
            else None
        )
        output_delta = (
            pricing_output - stats.benchmark_output_usd_per_mtok
            if pricing_output is not None and stats.benchmark_output_usd_per_mtok is not None
            else None
        )

        rows.append(
            LedgerRow(
                ledger_row_id=row_id,
                source_model=stats.source_model,
                source_model_slug=stats.source_model_slug,
                inferred_provider=inferred_provider,
                provider_mapping_rule=provider_rule,
                provider_mapping_confidence=provider_conf,
                canonical_model_guess=model_guess,
                model_mapping_rule=model_rule,
                model_mapping_confidence=model_conf,
                pricing_provider=pricing_provider,
                pricing_model=pricing_model,
                pricing_subscription_usd_month=subscription_dec,
                pricing_input_usd_per_mtok=pricing_input,
                pricing_output_usd_per_mtok=pricing_output,
                pricing_cache_write_usd_per_mtok=pricing_cache_write_dec,
                pricing_cache_read_usd_per_mtok=pricing_cache_read_dec,
                pricing_tool_input_usd_per_mtok=pricing_tool_input_dec,
                pricing_tool_output_usd_per_mtok=pricing_tool_output_dec,
                benchmark_input_usd_per_mtok=stats.benchmark_input_usd_per_mtok,
                benchmark_output_usd_per_mtok=stats.benchmark_output_usd_per_mtok,
                benchmark_rows_total=stats.benchmark_rows_total,
                benchmark_rows_non_missing=stats.benchmark_rows_non_missing,
                benchmark_rows_missing=stats.benchmark_rows_missing,
                benchmark_prior_rows_total=stats.benchmark_prior_rows_total,
                benchmark_prior_rows_non_missing=stats.benchmark_prior_rows_non_missing,
                benchmark_prior_rows_missing=stats.benchmark_prior_rows_missing,
                benchmark_distinct_total=stats.benchmark_distinct_total,
                benchmark_distinct_non_missing=stats.benchmark_distinct_non_missing,
                pricing_vs_benchmark_input_delta=input_delta,
                pricing_vs_benchmark_output_delta=output_delta,
            )
        )

    return rows


def csv_headers() -> list[str]:
    return [
        "ledger_row_id",
        "source_model",
        "source_model_slug",
        "inferred_provider",
        "provider_mapping_rule",
        "provider_mapping_confidence",
        "canonical_model_guess",
        "model_mapping_rule",
        "model_mapping_confidence",
        "pricing_provider",
        "pricing_model",
        "pricing_subscription_usd_month",
        "pricing_input_usd_per_mtok",
        "pricing_output_usd_per_mtok",
        "pricing_cache_write_usd_per_mtok",
        "pricing_cache_read_usd_per_mtok",
        "pricing_tool_input_usd_per_mtok",
        "pricing_tool_output_usd_per_mtok",
        "benchmark_input_usd_per_mtok",
        "benchmark_output_usd_per_mtok",
        "benchmark_rows_total",
        "benchmark_rows_non_missing",
        "benchmark_rows_missing",
        "benchmark_prior_rows_total",
        "benchmark_prior_rows_non_missing",
        "benchmark_prior_rows_missing",
        "benchmark_distinct_total",
        "benchmark_distinct_non_missing",
        "pricing_vs_benchmark_input_delta",
        "pricing_vs_benchmark_output_delta",
    ]


def write_csv(path: Path, rows: list[LedgerRow]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    headers = csv_headers()
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(headers)
        for r in rows:
            writer.writerow(
                [
                    r.ledger_row_id,
                    r.source_model,
                    r.source_model_slug,
                    r.inferred_provider,
                    r.provider_mapping_rule,
                    r.provider_mapping_confidence,
                    r.canonical_model_guess,
                    r.model_mapping_rule,
                    r.model_mapping_confidence,
                    r.pricing_provider,
                    r.pricing_model,
                    decimal_to_str(r.pricing_subscription_usd_month),
                    decimal_to_str(r.pricing_input_usd_per_mtok),
                    decimal_to_str(r.pricing_output_usd_per_mtok),
                    decimal_to_str(r.pricing_cache_write_usd_per_mtok),
                    decimal_to_str(r.pricing_cache_read_usd_per_mtok),
                    decimal_to_str(r.pricing_tool_input_usd_per_mtok),
                    decimal_to_str(r.pricing_tool_output_usd_per_mtok),
                    decimal_to_str(r.benchmark_input_usd_per_mtok),
                    decimal_to_str(r.benchmark_output_usd_per_mtok),
                    r.benchmark_rows_total,
                    r.benchmark_rows_non_missing,
                    r.benchmark_rows_missing,
                    r.benchmark_prior_rows_total,
                    r.benchmark_prior_rows_non_missing,
                    r.benchmark_prior_rows_missing,
                    r.benchmark_distinct_total,
                    r.benchmark_distinct_non_missing,
                    decimal_to_str(r.pricing_vs_benchmark_input_delta),
                    decimal_to_str(r.pricing_vs_benchmark_output_delta),
                ]
            )


def priors_aggregation(rows: list[LedgerRow]) -> list[dict[str, Any]]:
    grouped: dict[tuple[str, str], dict[str, Any]] = {}
    for r in rows:
        key = (r.inferred_provider, r.pricing_provider or "unmapped")
        g = grouped.setdefault(
            key,
            {
                "inferred_provider": r.inferred_provider,
                "pricing_provider": r.pricing_provider or "unmapped",
                "models_count": 0,
                "mapped_models_count": 0,
                "prior_rows_total": 0,
                "prior_rows_non_missing": 0,
                "prior_rows_missing": 0,
            },
        )
        g["models_count"] += 1
        if r.pricing_model:
            g["mapped_models_count"] += 1
        g["prior_rows_total"] += r.benchmark_prior_rows_total
        g["prior_rows_non_missing"] += r.benchmark_prior_rows_non_missing
        g["prior_rows_missing"] += r.benchmark_prior_rows_missing

    return [grouped[k] for k in sorted(grouped.keys())]


def write_schema_sql(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    schema = [
        "-- Deterministic unified ledger schema generated by scripts/build_unified_ledger.py",
        "DROP TABLE IF EXISTS unified_model_provider_ledger;",
        "",
        "CREATE TABLE unified_model_provider_ledger (",
        "  ledger_row_id INTEGER PRIMARY KEY,",
        "  source_model TEXT NOT NULL,",
        "  source_model_slug TEXT NOT NULL,",
        "  inferred_provider TEXT NOT NULL,",
        "  provider_mapping_rule TEXT NOT NULL,",
        "  provider_mapping_confidence INTEGER NOT NULL,",
        "  canonical_model_guess TEXT NOT NULL,",
        "  model_mapping_rule TEXT NOT NULL,",
        "  model_mapping_confidence INTEGER NOT NULL,",
        "  pricing_provider TEXT,",
        "  pricing_model TEXT,",
        "  pricing_subscription_usd_month NUMERIC,",
        "  pricing_input_usd_per_mtok NUMERIC,",
        "  pricing_output_usd_per_mtok NUMERIC,",
        "  pricing_cache_write_usd_per_mtok NUMERIC,",
        "  pricing_cache_read_usd_per_mtok NUMERIC,",
        "  pricing_tool_input_usd_per_mtok NUMERIC,",
        "  pricing_tool_output_usd_per_mtok NUMERIC,",
        "  benchmark_input_usd_per_mtok NUMERIC,",
        "  benchmark_output_usd_per_mtok NUMERIC,",
        "  benchmark_rows_total INTEGER NOT NULL,",
        "  benchmark_rows_non_missing INTEGER NOT NULL,",
        "  benchmark_rows_missing INTEGER NOT NULL,",
        "  benchmark_prior_rows_total INTEGER NOT NULL,",
        "  benchmark_prior_rows_non_missing INTEGER NOT NULL,",
        "  benchmark_prior_rows_missing INTEGER NOT NULL,",
        "  benchmark_distinct_total INTEGER NOT NULL,",
        "  benchmark_distinct_non_missing INTEGER NOT NULL,",
        "  pricing_vs_benchmark_input_delta NUMERIC,",
        "  pricing_vs_benchmark_output_delta NUMERIC",
        ");",
        "",
        "DROP TABLE IF EXISTS benchmark_priors_aggregation;",
        "",
        "CREATE TABLE benchmark_priors_aggregation (",
        "  aggregation_row_id INTEGER PRIMARY KEY,",
        "  inferred_provider TEXT NOT NULL,",
        "  pricing_provider TEXT NOT NULL,",
        "  models_count INTEGER NOT NULL,",
        "  mapped_models_count INTEGER NOT NULL,",
        "  prior_rows_total INTEGER NOT NULL,",
        "  prior_rows_non_missing INTEGER NOT NULL,",
        "  prior_rows_missing INTEGER NOT NULL",
        ");",
    ]
    path.write_text("\n".join(schema) + "\n", encoding="utf-8")


def write_seed_sql(path: Path, rows: list[LedgerRow]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)

    headers = csv_headers()
    lines = [
        "-- Deterministic unified ledger seed generated by scripts/build_unified_ledger.py",
        "INSERT INTO unified_model_provider_ledger (",
        "  " + ",\n  ".join(headers),
        ") VALUES",
    ]

    tuples: list[str] = []
    for r in rows:
        tuples.append(
            "(" + ", ".join(
                [
                    str(r.ledger_row_id),
                    sql_quote(r.source_model),
                    sql_quote(r.source_model_slug),
                    sql_quote(r.inferred_provider),
                    sql_quote(r.provider_mapping_rule),
                    str(r.provider_mapping_confidence),
                    sql_quote(r.canonical_model_guess),
                    sql_quote(r.model_mapping_rule),
                    str(r.model_mapping_confidence),
                    "NULL" if not r.pricing_provider else sql_quote(r.pricing_provider),
                    "NULL" if not r.pricing_model else sql_quote(r.pricing_model),
                    sql_number(r.pricing_subscription_usd_month),
                    sql_number(r.pricing_input_usd_per_mtok),
                    sql_number(r.pricing_output_usd_per_mtok),
                    sql_number(r.pricing_cache_write_usd_per_mtok),
                    sql_number(r.pricing_cache_read_usd_per_mtok),
                    sql_number(r.pricing_tool_input_usd_per_mtok),
                    sql_number(r.pricing_tool_output_usd_per_mtok),
                    sql_number(r.benchmark_input_usd_per_mtok),
                    sql_number(r.benchmark_output_usd_per_mtok),
                    str(r.benchmark_rows_total),
                    str(r.benchmark_rows_non_missing),
                    str(r.benchmark_rows_missing),
                    str(r.benchmark_prior_rows_total),
                    str(r.benchmark_prior_rows_non_missing),
                    str(r.benchmark_prior_rows_missing),
                    str(r.benchmark_distinct_total),
                    str(r.benchmark_distinct_non_missing),
                    sql_number(r.pricing_vs_benchmark_input_delta),
                    sql_number(r.pricing_vs_benchmark_output_delta),
                ]
            ) + ")"
        )
    lines.append(",\n".join(tuples) + ";")

    priors_rows = priors_aggregation(rows)
    lines.extend(
        [
            "",
            "INSERT INTO benchmark_priors_aggregation (",
            "  aggregation_row_id,",
            "  inferred_provider,",
            "  pricing_provider,",
            "  models_count,",
            "  mapped_models_count,",
            "  prior_rows_total,",
            "  prior_rows_non_missing,",
            "  prior_rows_missing",
            ") VALUES",
        ]
    )

    priors_tuples: list[str] = []
    for idx, row in enumerate(priors_rows, start=1):
        priors_tuples.append(
            "("
            f"{idx}, "
            f"{sql_quote(row['inferred_provider'])}, "
            f"{sql_quote(row['pricing_provider'])}, "
            f"{row['models_count']}, "
            f"{row['mapped_models_count']}, "
            f"{row['prior_rows_total']}, "
            f"{row['prior_rows_non_missing']}, "
            f"{row['prior_rows_missing']}"
            ")"
        )
    lines.append(",\n".join(priors_tuples) + ";")

    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Build deterministic unified model/provider ledger artifacts."
    )
    parser.add_argument("--pricing", type=Path, default=Path("pricing.example.json"))
    parser.add_argument("--models-csv", type=Path, default=Path("models_normalized.csv"))
    parser.add_argument(
        "--ledger-dir",
        type=Path,
        default=Path("ledger"),
        help="Output directory for generated ledger artifacts",
    )
    args = parser.parse_args()

    pricing = load_pricing(args.pricing)
    pricing_idx = build_pricing_indexes(pricing)
    stats_by_model = collect_model_stats(args.models_csv)
    rows = build_rows(stats_by_model, pricing_idx)

    csv_path = args.ledger_dir / "unified_model_provider_ledger.csv"
    schema_path = args.ledger_dir / "unified_model_provider_ledger.schema.sql"
    seed_path = args.ledger_dir / "unified_model_provider_ledger.seed.sql"

    write_csv(csv_path, rows)
    write_schema_sql(schema_path)
    write_seed_sql(seed_path, rows)

    priors_rows = priors_aggregation(rows)
    print(f"ledger_rows={len(rows)}")
    print(f"priors_aggregation_rows={len(priors_rows)}")
    print(f"csv={csv_path}")
    print(f"schema_sql={schema_path}")
    print(f"seed_sql={seed_path}")


if __name__ == "__main__":
    main()
