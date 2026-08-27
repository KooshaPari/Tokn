# Tokn Architecture

## Workspace

```
Tokn/
  crates/pareto-rs/   Core analytics library
  crates/tokenledger/ Token ledger CLI
  web/                Web dashboard
  schema/             SQL DDL
  migrations/         Migration scripts
  fuzz/               Fuzz targets
  locales/            i18n files
```

## Dependency Graph

```
tokenledger --> pareto-rs (core types + pricing)
pareto-rs    --> chrono, serde, clap
tokenledger  --> anyhow, thiserror
```

## Module Responsibilities

| Module | Crate | Responsibility |
|---|---|---|
| plugin.rs | pareto-rs | Plugin trait + registry |
| pricing.rs | pareto-rs | Pricing book + route decisions |
| cost.rs | pareto-rs | Cost calculation engine |
| event.rs | pareto-rs | Event bus + pub/sub |
| error.rs | pareto-rs | Custom error enum (thiserror) |
| metrics.rs | tokenledger | Prometheus metrics + /health |
| concurrent.rs | tokenledger | RwLock + async spawn |
| models.rs | pareto-rs | Domain types (22 structs) |
