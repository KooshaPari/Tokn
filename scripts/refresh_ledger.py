#!/usr/bin/env python3
"""Refresh unified ledger artifacts and derive a Pareto scoring view.

This script closes the recurring refresh loop by:
1) Regenerating deterministic unified ledger artifacts.
2) Importing optional CLIProxyAPI management/runtime metrics snapshots.
3) Writing a blended Pareto view derived from ledger + runtime metrics.
"""

from __future__ import annotations

import argparse
import csv
import json
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class RuntimeMetric:
    provider: str
    model: str
    latency_ms: float | None
    quality_score: float | None
    source_path: str


def normalize_text(value: str) -> str:
    out = "".join(ch.lower() if ch.isalnum() else "-" for ch in value.strip())
    while "--" in out:
        out = out.replace("--", "-")
    return out.strip("-")


def parse_float(value: Any) -> float | None:
    if value is None:
        return None
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str):
        text = value.strip().replace(",", "")
        if not text:
            return None
        try:
            return float(text)
        except ValueError:
            return None
    return None


def normalize_quality(value: float | None) -> float | None:
    if value is None:
        return None
    if value < 0:
        return None
    if value <= 1.0:
        return value
    if value <= 100.0:
        return value / 100.0
    return None


def discover_cliproxyapi_snapshots(repo_root: Path) -> list[Path]:
    home = Path.home()
    candidates = [
        repo_root / "benchmarks" / "results" / "cliproxyapi-metrics-snapshot.json",
        home / ".cliproxyapi" / "metrics_snapshot.json",
        home / ".cliproxyapi" / "management" / "metrics_snapshot.json",
        home / ".config" / "cliproxyapi" / "metrics_snapshot.json",
        home / "Library" / "Application Support" / "CLIProxyAPI" / "metrics_snapshot.json",
    ]
    return [path for path in candidates if path.is_file()]


def collect_runtime_candidates(value: Any, source_path: str, out: list[RuntimeMetric]) -> None:
    if isinstance(value, list):
        for item in value:
            collect_runtime_candidates(item, source_path, out)
        return

    if not isinstance(value, dict):
        return

    provider = value.get("provider") or value.get("inferred_provider")
    model = value.get("model") or value.get("pricing_model") or value.get("canonical_model")

    latency = (
        parse_float(value.get("latency_ms"))
        or parse_float(value.get("median_latency_ms"))
        or parse_float(value.get("p50_latency_ms"))
        or parse_float(value.get("p95_latency_ms"))
    )
    quality = (
        parse_float(value.get("quality_score"))
        or parse_float(value.get("success_rate"))
        or parse_float(value.get("win_rate"))
        or parse_float(value.get("accuracy"))
    )
    quality = normalize_quality(quality)

    if isinstance(provider, str) and isinstance(model, str) and (latency is not None or quality is not None):
        out.append(
            RuntimeMetric(
                provider=normalize_text(provider),
                model=normalize_text(model),
                latency_ms=latency,
                quality_score=quality,
                source_path=source_path,
            )
        )

    for child in value.values():
        collect_runtime_candidates(child, source_path, out)


def load_runtime_metrics(paths: list[Path]) -> dict[tuple[str, str], RuntimeMetric]:
    out: dict[tuple[str, str], RuntimeMetric] = {}
    for path in paths:
        payload = json.loads(path.read_text(encoding="utf-8"))
        found: list[RuntimeMetric] = []
        collect_runtime_candidates(payload, str(path), found)
        for metric in found:
            key = (metric.provider, metric.model)
            if key not in out:
                out[key] = metric
    return out


def benchmark_quality(row: dict[str, str]) -> float | None:
    total = parse_float(row.get("benchmark_prior_rows_total"))
    non_missing = parse_float(row.get("benchmark_prior_rows_non_missing"))
    if total is None or non_missing is None or total <= 0:
        return None
    return max(0.0, min(1.0, non_missing / total))


def cost_component(row: dict[str, str]) -> tuple[float, float | None]:
    inp = parse_float(row.get("pricing_input_usd_per_mtok"))
    out = parse_float(row.get("pricing_output_usd_per_mtok"))
    vals = [v for v in [inp, out] if v is not None and v >= 0]
    if not vals:
        return 0.0, None
    avg_cost = sum(vals) / len(vals)
    component = 1.0 / (1.0 + avg_cost)
    return component, avg_cost


def latency_component(latency_ms: float | None) -> float:
    if latency_ms is None or latency_ms < 0:
        return 0.0
    return 1.0 / (1.0 + (latency_ms / 1000.0))


def blended_quality(runtime_quality: float | None, bench_quality: float | None) -> float:
    vals = [v for v in [runtime_quality, bench_quality] if v is not None]
    if not vals:
        return 0.0
    return sum(vals) / len(vals)


def write_runtime_metrics_csv(path: Path, metrics: dict[tuple[str, str], RuntimeMetric]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["provider", "model", "latency_ms", "quality_score", "source_path"])
        for key in sorted(metrics.keys()):
            metric = metrics[key]
            writer.writerow(
                [
                    metric.provider,
                    metric.model,
                    "" if metric.latency_ms is None else f"{metric.latency_ms:.6f}",
                    "" if metric.quality_score is None else f"{metric.quality_score:.6f}",
                    metric.source_path,
                ]
            )


def write_pareto_view_csv(
    path: Path,
    ledger_csv_path: Path,
    runtime_metrics: dict[tuple[str, str], RuntimeMetric],
) -> int:
    with ledger_csv_path.open(newline="", encoding="utf-8") as f:
        ledger_rows = list(csv.DictReader(f))

    scored_rows: list[dict[str, Any]] = []
    for row in ledger_rows:
        provider = normalize_text(row.get("pricing_provider") or row.get("inferred_provider") or "unknown")
        model = normalize_text(row.get("pricing_model") or row.get("canonical_model_guess") or "unknown")
        runtime = runtime_metrics.get((provider, model))

        c_cost, avg_cost = cost_component(row)
        c_latency = latency_component(runtime.latency_ms if runtime else None)
        q_bench = benchmark_quality(row)
        q_runtime = runtime.quality_score if runtime else None
        c_quality = blended_quality(q_runtime, q_bench)

        score = (0.50 * c_quality) + (0.30 * c_cost) + (0.20 * c_latency)
        scored_rows.append(
            {
                "ledger_row_id": row["ledger_row_id"],
                "provider": provider,
                "model": model,
                "pareto_score": round(score * 100.0, 6),
                "quality_component": round(c_quality, 6),
                "cost_component": round(c_cost, 6),
                "latency_component": round(c_latency, 6),
                "blended_cost_usd_per_mtok": "" if avg_cost is None else round(avg_cost, 6),
                "runtime_latency_ms": "" if not runtime or runtime.latency_ms is None else round(runtime.latency_ms, 6),
                "runtime_quality_score": "" if not runtime or runtime.quality_score is None else round(runtime.quality_score, 6),
                "benchmark_quality_score": "" if q_bench is None else round(q_bench, 6),
                "runtime_source_path": "" if not runtime else runtime.source_path,
            }
        )

    scored_rows.sort(key=lambda r: (-float(r["pareto_score"]), r["provider"], r["model"]))
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=[
                "ledger_row_id",
                "provider",
                "model",
                "pareto_score",
                "quality_component",
                "cost_component",
                "latency_component",
                "blended_cost_usd_per_mtok",
                "runtime_latency_ms",
                "runtime_quality_score",
                "benchmark_quality_score",
                "runtime_source_path",
            ],
        )
        writer.writeheader()
        writer.writerows(scored_rows)
    return len(scored_rows)


def write_pareto_sql(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    sql = """-- Runtime metrics + Pareto scoring view generated by scripts/refresh_ledger.py
-- Seed runtime metrics from ledger/cliproxyapi_runtime_metrics_snapshot.csv as needed.

DROP TABLE IF EXISTS cliproxyapi_runtime_metrics_snapshot;
CREATE TABLE cliproxyapi_runtime_metrics_snapshot (
  provider TEXT NOT NULL,
  model TEXT NOT NULL,
  latency_ms NUMERIC,
  quality_score NUMERIC,
  source_path TEXT,
  PRIMARY KEY (provider, model)
);

DROP VIEW IF EXISTS unified_model_provider_pareto_view;
CREATE VIEW unified_model_provider_pareto_view AS
SELECT
  l.ledger_row_id,
  COALESCE(NULLIF(l.pricing_provider, ''), l.inferred_provider) AS provider,
  COALESCE(NULLIF(l.pricing_model, ''), l.canonical_model_guess) AS model,
  l.pricing_input_usd_per_mtok,
  l.pricing_output_usd_per_mtok,
  m.latency_ms AS runtime_latency_ms,
  m.quality_score AS runtime_quality_score,
  CASE
    WHEN l.benchmark_prior_rows_total > 0
      THEN CAST(l.benchmark_prior_rows_non_missing AS NUMERIC) / CAST(l.benchmark_prior_rows_total AS NUMERIC)
    ELSE NULL
  END AS benchmark_quality_score,
  (
    100.0 * (
      0.50 * COALESCE(
        (
          COALESCE(m.quality_score, 0.0) +
          COALESCE(
            CASE
              WHEN l.benchmark_prior_rows_total > 0
                THEN CAST(l.benchmark_prior_rows_non_missing AS NUMERIC) / CAST(l.benchmark_prior_rows_total AS NUMERIC)
              ELSE NULL
            END,
            0.0
          )
        ) /
        CASE
          WHEN m.quality_score IS NOT NULL
               AND l.benchmark_prior_rows_total > 0 THEN 2.0
          WHEN m.quality_score IS NOT NULL
               OR l.benchmark_prior_rows_total > 0 THEN 1.0
          ELSE 1.0
        END,
        0.0
      )
      + 0.30 * (
        CASE
          WHEN COALESCE(l.pricing_input_usd_per_mtok, l.pricing_output_usd_per_mtok) IS NULL THEN 0.0
          ELSE 1.0 / (
            1.0 + (
              COALESCE(l.pricing_input_usd_per_mtok, 0.0) +
              COALESCE(l.pricing_output_usd_per_mtok, 0.0)
            ) /
            CASE
              WHEN l.pricing_input_usd_per_mtok IS NOT NULL
                   AND l.pricing_output_usd_per_mtok IS NOT NULL THEN 2.0
              ELSE 1.0
            END
          )
        END
      )
      + 0.20 * (
        CASE
          WHEN m.latency_ms IS NULL THEN 0.0
          ELSE 1.0 / (1.0 + (m.latency_ms / 1000.0))
        END
      )
    )
  ) AS pareto_score
FROM unified_model_provider_ledger l
LEFT JOIN cliproxyapi_runtime_metrics_snapshot m
  ON m.provider = COALESCE(NULLIF(l.pricing_provider, ''), l.inferred_provider)
 AND m.model = COALESCE(NULLIF(l.pricing_model, ''), l.canonical_model_guess);
"""
    path.write_text(sql, encoding="utf-8")


def run_build_ledger(repo_root: Path, pricing: Path, models_csv: Path, ledger_dir: Path) -> None:
    cmd = [
        sys.executable,
        str(repo_root / "scripts" / "build_unified_ledger.py"),
        "--pricing",
        str(pricing),
        "--models-csv",
        str(models_csv),
        "--ledger-dir",
        str(ledger_dir),
    ]
    subprocess.run(cmd, cwd=repo_root, check=True)


def main() -> None:
    parser = argparse.ArgumentParser(description="Refresh ledger artifacts + Pareto view.")
    parser.add_argument("--repo-root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--pricing", type=Path, default=Path("pricing.example.json"))
    parser.add_argument("--models-csv", type=Path, default=Path("models_normalized.csv"))
    parser.add_argument("--ledger-dir", type=Path, default=Path("ledger"))
    parser.add_argument(
        "--cliproxyapi-snapshot",
        type=Path,
        action="append",
        default=[],
        help="Optional runtime metrics snapshot path (repeatable).",
    )
    parser.add_argument(
        "--allow-missing-snapshot",
        action="store_true",
        help="Do not fail when no runtime snapshot is available.",
    )
    parser.add_argument(
        "--skip-runtime-discovery",
        action="store_true",
        help="Use only explicit --cliproxyapi-snapshot values.",
    )
    args = parser.parse_args()

    repo_root = args.repo_root.resolve()
    pricing = (repo_root / args.pricing).resolve() if not args.pricing.is_absolute() else args.pricing.resolve()
    models_csv = (
        (repo_root / args.models_csv).resolve()
        if not args.models_csv.is_absolute()
        else args.models_csv.resolve()
    )
    ledger_dir = (repo_root / args.ledger_dir).resolve() if not args.ledger_dir.is_absolute() else args.ledger_dir.resolve()

    run_build_ledger(repo_root, pricing, models_csv, ledger_dir)

    runtime_paths: list[Path] = []
    if not args.skip_runtime_discovery:
        runtime_paths.extend(discover_cliproxyapi_snapshots(repo_root))
    runtime_paths.extend(
        (repo_root / p).resolve() if not p.is_absolute() else p.resolve()
        for p in args.cliproxyapi_snapshot
    )
    deduped_paths: list[Path] = []
    seen: set[Path] = set()
    for path in runtime_paths:
        if path in seen:
            continue
        seen.add(path)
        if path.is_file():
            deduped_paths.append(path)

    if not deduped_paths and not args.allow_missing_snapshot:
        raise SystemExit(
            "No CLIProxyAPI runtime snapshot found. "
            "Pass --allow-missing-snapshot or provide --cliproxyapi-snapshot <path>."
        )

    runtime_metrics = load_runtime_metrics(deduped_paths)
    runtime_csv_path = ledger_dir / "cliproxyapi_runtime_metrics_snapshot.csv"
    pareto_csv_path = ledger_dir / "unified_model_provider_pareto.csv"
    pareto_sql_path = ledger_dir / "unified_model_provider_pareto.view.sql"
    ledger_csv_path = ledger_dir / "unified_model_provider_ledger.csv"

    write_runtime_metrics_csv(runtime_csv_path, runtime_metrics)
    pareto_rows = write_pareto_view_csv(pareto_csv_path, ledger_csv_path, runtime_metrics)
    write_pareto_sql(pareto_sql_path)

    print(f"runtime_snapshot_files={len(deduped_paths)}")
    print(f"runtime_metric_rows={len(runtime_metrics)}")
    print(f"pareto_rows={pareto_rows}")
    print(f"runtime_csv={runtime_csv_path}")
    print(f"pareto_csv={pareto_csv_path}")
    print(f"pareto_sql={pareto_sql_path}")


if __name__ == "__main__":
    main()
