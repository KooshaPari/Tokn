#!/usr/bin/env python3
"""Build normalized model benchmark seeds from the raw models pipe-table text.

Input is intentionally treated as text (not CSV), even though the filename is
`models.csv`, because the source is a markdown-style pipe table.
"""

from __future__ import annotations

import argparse
import csv
from dataclasses import dataclass
from pathlib import Path
import re

MISSING_TOKENS = {"", "—", "-", "N/A", "n/a", "na", "NA", "null", "NULL"}
SLASH_SPLIT_RE = re.compile(r"\s*/\s*")


@dataclass(frozen=True)
class NormalizedRow:
    row_id: int
    benchmark: str
    notes_configuration: str
    model: str
    raw_value: str
    value_primary: str
    value_secondary: str
    split_kind: str
    is_missing: int
    source_row_index: int
    source_col_index: int
    source_col_name: str


def parse_pipe_row(line: str) -> list[str]:
    stripped = line.strip()
    if not stripped.startswith("|") or not stripped.endswith("|"):
        return []
    return [cell.strip() for cell in stripped.strip("|").split("|")]


def load_pipe_table(path: Path) -> tuple[list[str], list[list[str]]]:
    rows: list[list[str]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        parsed = parse_pipe_row(line)
        if parsed:
            rows.append(parsed)
    if not rows:
        raise ValueError(f"no pipe-table rows found in {path}")
    header, body = rows[0], rows[1:]
    width = len(header)
    normalized_body = [r[:width] + [""] * max(0, width - len(r)) for r in body]
    return header, normalized_body


def split_value(raw_value: str) -> tuple[str, str, str, int]:
    if raw_value in MISSING_TOKENS:
        return "", "", "none", 1
    parts = SLASH_SPLIT_RE.split(raw_value, maxsplit=1)
    if len(parts) == 2:
        return parts[0], parts[1], "slash", 0
    return raw_value, "", "none", 0


def normalize(header: list[str], body: list[list[str]]) -> list[NormalizedRow]:
    if len(header) < 3:
        raise ValueError("expected at least 3 columns in source pipe-table")
    model_columns = header[2:]
    out: list[NormalizedRow] = []
    next_row_id = 1
    for source_row_index, row in enumerate(body, start=2):
        benchmark = row[0]
        notes_configuration = row[1] if len(row) > 1 else ""
        for offset, model_col_name in enumerate(model_columns, start=3):
            raw_value = row[offset - 1] if offset - 1 < len(row) else ""
            value_primary, value_secondary, split_kind, is_missing = split_value(raw_value)
            out.append(
                NormalizedRow(
                    row_id=next_row_id,
                    benchmark=benchmark,
                    notes_configuration=notes_configuration,
                    model=model_col_name,
                    raw_value=raw_value,
                    value_primary=value_primary,
                    value_secondary=value_secondary,
                    split_kind=split_kind,
                    is_missing=is_missing,
                    source_row_index=source_row_index,
                    source_col_index=offset,
                    source_col_name=model_col_name,
                )
            )
            next_row_id += 1
    return out


def write_csv(path: Path, rows: list[NormalizedRow]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(
            [
                "row_id",
                "benchmark",
                "notes_configuration",
                "model",
                "raw_value",
                "value_primary",
                "value_secondary",
                "split_kind",
                "is_missing",
                "source_row_index",
                "source_col_index",
                "source_col_name",
            ]
        )
        for r in rows:
            writer.writerow(
                [
                    r.row_id,
                    r.benchmark,
                    r.notes_configuration,
                    r.model,
                    r.raw_value,
                    r.value_primary,
                    r.value_secondary,
                    r.split_kind,
                    r.is_missing,
                    r.source_row_index,
                    r.source_col_index,
                    r.source_col_name,
                ]
            )


def sql_quote(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def write_sql(path: Path, rows: list[NormalizedRow]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    lines: list[str] = [
        "-- Generated from tokenledger/models.csv markdown-pipe table.",
        "-- Deterministic normalization rules:",
        "-- 1) Missing tokens are: '', '—', '-', 'N/A', 'n/a', 'na', 'NA', 'null', 'NULL'.",
        "-- 2) Non-missing values are split once on regex /\\s*\\/\\s*/ into value_primary/value_secondary.",
        "-- 3) If no split occurs, value_secondary is NULL and split_kind='none'.",
        "",
        "DROP TABLE IF EXISTS model_benchmark_values;",
        "",
        "CREATE TABLE model_benchmark_values (",
        "  row_id INTEGER PRIMARY KEY,",
        "  benchmark TEXT NOT NULL,",
        "  notes_configuration TEXT NOT NULL,",
        "  model TEXT NOT NULL,",
        "  raw_value TEXT,",
        "  value_primary TEXT,",
        "  value_secondary TEXT,",
        "  split_kind TEXT NOT NULL CHECK (split_kind IN ('none', 'slash')),",
        "  is_missing INTEGER NOT NULL CHECK (is_missing IN (0, 1)),",
        "  source_row_index INTEGER NOT NULL,",
        "  source_col_index INTEGER NOT NULL,",
        "  source_col_name TEXT NOT NULL",
        ");",
        "",
        "INSERT INTO model_benchmark_values (",
        "  row_id,",
        "  benchmark,",
        "  notes_configuration,",
        "  model,",
        "  raw_value,",
        "  value_primary,",
        "  value_secondary,",
        "  split_kind,",
        "  is_missing,",
        "  source_row_index,",
        "  source_col_index,",
        "  source_col_name",
        ") VALUES",
    ]

    tuples: list[str] = []
    for r in rows:
        raw_value = "NULL" if r.raw_value == "" else sql_quote(r.raw_value)
        value_primary = "NULL" if r.value_primary == "" else sql_quote(r.value_primary)
        value_secondary = "NULL" if r.value_secondary == "" else sql_quote(r.value_secondary)
        tuples.append(
            "("
            f"{r.row_id}, "
            f"{sql_quote(r.benchmark)}, "
            f"{sql_quote(r.notes_configuration)}, "
            f"{sql_quote(r.model)}, "
            f"{raw_value}, "
            f"{value_primary}, "
            f"{value_secondary}, "
            f"{sql_quote(r.split_kind)}, "
            f"{r.is_missing}, "
            f"{r.source_row_index}, "
            f"{r.source_col_index}, "
            f"{sql_quote(r.source_col_name)}"
            ")"
        )
    lines.append(",\n".join(tuples) + ";")
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Build normalized CSV + SQL seed from tokenledger models pipe-table text."
    )
    parser.add_argument("--input", type=Path, default=Path("models.csv"))
    parser.add_argument("--csv-out", type=Path, default=Path("models_normalized.csv"))
    parser.add_argument("--sql-out", type=Path, default=Path("models_schema_seed.sql"))
    args = parser.parse_args()

    header, body = load_pipe_table(args.input)
    rows = normalize(header, body)
    write_csv(args.csv_out, rows)
    write_sql(args.sql_out, rows)
    print(
        f"generated {len(rows)} rows -> csv:{args.csv_out} sql:{args.sql_out} from {args.input}"
    )


if __name__ == "__main__":
    main()
