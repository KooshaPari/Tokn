# Merged Fragmented Markdown

## Source: integrations/EXTENSION_FILE_READ_INTEGRATION_EXAMPLES.md

# Extension Integration Examples (File-Based Reads)

## Purpose

Provide practical integration examples for CodexBar/OpenCode-style consumers that read `tokenledger` artifacts from disk (no service dependency).

## Producer command

```bash
cargo run -- orchestrate \
  --providers codex \
  --limit 500 \
  --events-out ./artifacts/ingested.sample.jsonl \
  --state-file ./benchmarks/results/ingest-state.json \
  --summary-json-path ./benchmarks/results/ingest-summary.json \
  --ingest-cache-path ./benchmarks/results/orchestrate-ingest-cache.json \
  --pipeline-summary-path ./benchmarks/results/orchestrate-summary.json \
  --pricing-reconcile-dry-run \
  --ui-snapshot-path ./benchmarks/results/ui-snapshot.json
```

## Files to read

1. Primary UI payload:
- `./benchmarks/results/ui-snapshot.json`
2. Optional reconcile details pointer:
- `reconcile_latest_summary_path` inside snapshot
3. Optional pipeline status:
- `./benchmarks/results/orchestrate-summary.json`

## Example A: Polling reader (TypeScript/Node)

```ts
import { readFile } from "node:fs/promises";

type UiSnapshotV1 = {
  schema_version: 1;
  generated_at: string;
  month: string;
  mode: "compact" | "extended";
  totals: {
    cost_usd: number;
    tokens: number;
    blended_usd_per_mtok: number;
    session_count: number;
    skipped_unpriced_count: number;
  };
  top_providers: Array<{
    name: string;
    tokens: number;
    total_cost_usd: number;
    blended_usd_per_mtok: number;
    session_count: number;
  }>;
  top_models: Array<{
    name: string;
    tokens: number;
    total_cost_usd: number;
    blended_usd_per_mtok: number;
    session_count: number;
  }>;
  suggestions: string[];
  reconcile_latest_summary_path?: string;
};

async function loadSnapshot(path: string): Promise<UiSnapshotV1 | null> {
  try {
    const raw = await readFile(path, "utf8");
    const parsed = JSON.parse(raw);
    if (parsed?.schema_version !== 1) return null;
    return parsed as UiSnapshotV1;
  } catch {
    return null;
  }
}

setInterval(async () => {
  const snapshot = await loadSnapshot("./benchmarks/results/ui-snapshot.json");
  if (!snapshot) return;
  console.log(snapshot.month, snapshot.totals.cost_usd, snapshot.top_models[0]?.name);
}, 2000);
```

## Example B: fs-watch reader with debounce (TypeScript/Node)

```ts
import { watch } from "node:fs";
import { readFile } from "node:fs/promises";

const path = "./benchmarks/results/ui-snapshot.json";
let timer: NodeJS.Timeout | undefined;

async function refresh() {
  try {
    const payload = JSON.parse(await readFile(path, "utf8"));
    if (payload?.schema_version !== 1) return;
    // render/update extension UI state
  } catch {
    // ignore transient partial-write/parse errors
  }
}

watch(path, () => {
  if (timer) clearTimeout(timer);
  timer = setTimeout(() => void refresh(), 120);
});
```

## Integration checklist

1. Validate `schema_version` before consuming fields.
2. Ignore unknown keys (forward compatibility).
3. Handle missing file and parse errors without crashing extension process.
4. Keep reads local-only and low frequency (1-5s polling) unless fs-watch is available.
5. Optionally read `reconcile_latest_summary_path` when present for detail views.

---
