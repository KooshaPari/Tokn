#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GATES_FILE="$ROOT_DIR/benchmarks/perf-gates.json"
OUT_FILE="$(mktemp)"
trap 'rm -f "$OUT_FILE"' EXIT

BASELINE_PATH="${PERF_BASELINE:-}"
STRICT_MODE="${PERF_STRICT:-0}"
BENCH_EXTRA_ARGS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --strict)
      STRICT_MODE=1
      shift
      ;;
    --)
      shift
      BENCH_EXTRA_ARGS+=("$@")
      break
      ;;
    -*)
      BENCH_EXTRA_ARGS+=("$1")
      shift
      ;;
    *)
      if [[ -z "$BASELINE_PATH" ]]; then
        BASELINE_PATH="$1"
      else
        BENCH_EXTRA_ARGS+=("$1")
      fi
      shift
      ;;
  esac
done

cd "$ROOT_DIR"
BENCH_CMD=(
  cargo run --quiet -- bench
  --scenario all
  --json-output
  --events ./examples/events.jsonl
  --pricing ./pricing.example.json
)
if [[ -n "$BASELINE_PATH" ]]; then
  BENCH_CMD+=(--baseline "$BASELINE_PATH")
fi
if [[ ${#BENCH_EXTRA_ARGS[@]} -gt 0 ]]; then
  BENCH_CMD+=("${BENCH_EXTRA_ARGS[@]}")
fi
"${BENCH_CMD[@]}" > "$OUT_FILE"

python3 - "$GATES_FILE" "$OUT_FILE" "$BASELINE_PATH" "$STRICT_MODE" <<'PY'
import json
import sys

cfg_path, results_path, baseline_path, strict_mode_raw = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
strict_mode = strict_mode_raw == "1"
with open(cfg_path, "r", encoding="utf-8") as f:
    cfg = json.load(f)
with open(results_path, "r", encoding="utf-8") as f:
    report = json.load(f)

results = {item["scenario"]: item for item in report.get("results", [])}
failures = []

require_baseline = bool(cfg.get("require_baseline_for_regression_checks", False))
has_regression_thresholds = any(
    gate.get("max_elapsed_regression_pct") is not None or gate.get("max_eps_drop_pct") is not None
    for gate in cfg.get("scenarios", {}).values()
)
if strict_mode and (require_baseline or has_regression_thresholds) and not baseline_path:
    failures.append(
        "strict mode requires a baseline path when regression thresholds are configured; set PERF_BASELINE or pass baseline as first arg"
    )

for scenario, gate in cfg.get("scenarios", {}).items():
    result = results.get(scenario)
    if result is None:
        failures.append(f"{scenario}: missing benchmark result")
        continue

    elapsed_ms = float(result.get("elapsed_ms", 0.0))
    events_per_sec = float(result.get("events_per_sec", 0.0))

    max_ms = float(gate["max_ms"])
    min_eps = float(gate["min_events_per_sec"])

    if elapsed_ms > max_ms:
        failures.append(
            f"{scenario}: elapsed_ms {elapsed_ms:.4f} > max_ms {max_ms:.4f}"
        )
    if events_per_sec < min_eps:
        failures.append(
            f"{scenario}: events_per_sec {events_per_sec:.4f} < min_events_per_sec {min_eps:.4f}"
        )

    elapsed_delta = result.get("elapsed_ms_delta")
    eps_delta = result.get("events_per_sec_delta")
    max_elapsed_regression_pct = gate.get("max_elapsed_regression_pct")
    max_eps_drop_pct = gate.get("max_eps_drop_pct")

    if elapsed_delta is not None and max_elapsed_regression_pct is not None:
        elapsed_delta = float(elapsed_delta)
        if elapsed_delta > 0:
            baseline_elapsed = elapsed_ms - elapsed_delta
            if baseline_elapsed > 0:
                regression_pct = (elapsed_delta / baseline_elapsed) * 100.0
                if regression_pct > float(max_elapsed_regression_pct):
                    failures.append(
                        f"{scenario}: elapsed regression {regression_pct:.2f}% > max_elapsed_regression_pct {float(max_elapsed_regression_pct):.2f}%"
                    )

    if eps_delta is not None and max_eps_drop_pct is not None:
        eps_delta = float(eps_delta)
        if eps_delta < 0:
            baseline_eps = events_per_sec - eps_delta
            if baseline_eps > 0:
                drop_pct = ((-eps_delta) / baseline_eps) * 100.0
                if drop_pct > float(max_eps_drop_pct):
                    failures.append(
                        f"{scenario}: events/sec drop {drop_pct:.2f}% > max_eps_drop_pct {float(max_eps_drop_pct):.2f}%"
                    )

if failures:
    print("Performance gate FAILED")
    for failure in failures:
        print(f"- {failure}")
    sys.exit(1)

print("Performance gate passed")
for scenario, gate in cfg.get("scenarios", {}).items():
    result = results[scenario]
    print(
        f"- {scenario}: elapsed_ms={result['elapsed_ms']:.4f} (max={gate['max_ms']}), "
        f"events_per_sec={result['events_per_sec']:.4f} (min={gate['min_events_per_sec']})"
    )
PY
