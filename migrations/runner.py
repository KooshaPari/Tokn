"""Migration runner for Tokn schema.

Reads SQL files from schema/ and applies them in order to the configured
SQLite database. Each migration is recorded in the `_migrations` table so
subsequent runs skip already-applied migrations.

Usage:
    python -m migrations.runner [--db PATH] apply
    python -m migrations.runner [--db PATH] status

The default database path is `tokn.db` in the current directory.
"""
from __future__ import annotations

import argparse
import sqlite3
import sys
from dataclasses import dataclass
from pathlib import Path

SCHEMA_DIR = Path(__file__).resolve().parent.parent / "schema"
DEFAULT_DB = Path("tokn.db")


@dataclass(frozen=True)
class Migration:
    id: str
    path: Path

    @property
    def sql(self) -> str:
        return self.path.read_text(encoding="utf-8")


def discover_migrations() -> list[Migration]:
    """Return migrations in lexicographic order."""
    if not SCHEMA_DIR.exists():
        return []
    items: list[Migration] = []
    for sql_file in sorted(SCHEMA_DIR.glob("*.sql")):
        items.append(Migration(id=sql_file.stem, path=sql_file))
    return items


def ensure_tracking_table(conn: sqlite3.Connection) -> None:
    conn.execute(
        """
        CREATE TABLE IF NOT EXISTS _migrations (
            id         TEXT PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )
        """
    )
    conn.commit()


def applied_ids(conn: sqlite3.Connection) -> set[str]:
    rows = conn.execute("SELECT id FROM _migrations").fetchall()
    return {r[0] for r in rows}


def apply_migrations(db_path: Path) -> list[Migration]:
    """Apply pending migrations in order. Returns the list applied."""
    applied: list[Migration] = []
    with sqlite3.connect(db_path) as conn:
        ensure_tracking_table(conn)
        already = applied_ids(conn)
        for mig in discover_migrations():
            if mig.id in already:
                continue
            conn.executescript(mig.sql)
            # executescript may not honor the migration INSERT idempotently;
            # upsert explicitly so repeated runs stay safe.
            conn.execute(
                "INSERT OR REPLACE INTO _migrations (id, applied_at) VALUES (?, strftime('%s','now') * 1000)",
                (mig.id,),
            )
            conn.commit()
            applied.append(mig)
    return applied


def migration_status(db_path: Path) -> tuple[list[Migration], set[str]]:
    """Return (all, applied) for status reporting."""
    with sqlite3.connect(db_path) as conn:
        ensure_tracking_table(conn)
        already = applied_ids(conn)
    return discover_migrations(), already


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="Tokn schema migration runner")
    parser.add_argument("--db", default=str(DEFAULT_DB), help="SQLite database path")
    sub = parser.add_subparsers(dest="cmd", required=True)
    sub.add_parser("apply", help="apply pending migrations")
    sub.add_parser("status", help="show migration status")
    args = parser.parse_args(argv)

    db_path = Path(args.db)
    cmd = args.cmd

    if cmd == "apply":
        applied = apply_migrations(db_path)
        if applied:
            for mig in applied:
                print(f"applied {mig.id}")
        else:
            print("no pending migrations")
        return 0

    if cmd == "status":
        all_migs, already = migration_status(db_path)
        for mig in all_migs:
            status = "applied" if mig.id in already else "pending"
            print(f"{mig.id:24s} {status}")
        return 0

    return 2


if __name__ == "__main__":  # pragma: no cover
    sys.exit(main(sys.argv[1:]))
