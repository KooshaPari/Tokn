# Usage Audit and Market Scan

Date: 2026-02-20

## Local Findings (kush)

- `usage/` already contains a broad ccusage-derived implementation with Python migration work and hybrid Python bridge support.
- `usage/usage-main/apps/ccusage` includes provider loaders for Claude/Codex/Cursor/Droid and gRPC/subprocess bridges.
- Current pain point is real-time performance in heavy multi-provider scans, especially with large file/database traversals.

## External Tools to Learn From

- `ccusage`: mature usage CLI and ecosystem (`ccusage`, `@ccusage/codex`, `@ccusage/mcp`).
- `OpenCode`: model/provider usage tracking patterns and model management UX.
- `CodexBar`: status-bar-first UX for agent usage visibility.

## Architecture Direction

- Keep provider scraping adapters separate from analytics core.
- Use Rust for streaming ingest + aggregation path.
- Support normalized events so existing local collectors can pipe data into core immediately.
