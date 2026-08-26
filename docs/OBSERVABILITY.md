# Observability

Tokn / Pareto cost engine ships with built-in Prometheus metrics and a lightweight HTTP server.

## Running the metrics server

```sh
cargo run -p pareto-rs --bin pareto-server
```

This starts an Axum HTTP server on **port 9090** with two endpoints:

| Endpoint   | Description                                   |
| ---------- | --------------------------------------------- |
| `/health`  | Liveness / readiness probe — returns `200 ok` |
| `/metrics` | Prometheus text-format metrics                |

## Exposed metrics

| Metric                              | Type      | Description                             |
| ----------------------------------- | --------- | --------------------------------------- |
| `pareto_ingest_total`               | Counter   | Total cost entries ingested             |
| `pareto_queue_depth`                | Gauge     | Current entries waiting to be processed |
| `pareto_processing_latency_seconds` | Histogram | End-to-end processing latency (seconds) |

### Prometheus scrape config

```yaml
scrape_configs:
  - job_name: "pareto"
    static_configs:
      - targets: ["localhost:9090"]
```

## Grafana dashboard

Import the dashboard JSON from `docs/grafana/` or create a new panel with:

- **Ingest rate** — `rate(pareto_ingest_total[5m])`
- **Queue depth** — `pareto_queue_depth`
- **p50/p95/p99 latency** — `histogram_quantile(0.95, rate(pareto_processing_latency_seconds_bucket[5m]))`

## Using the metrics library directly

```rust
use pareto_rs::metrics::ParetoMetrics;

let m = ParetoMetrics::new();
m.ingest_total.inc();
m.queue_depth.set(42);
m.processing_latency.observe(0.035);

// Emit Prometheus text
println!("{}", m.encode_metrics());
```
