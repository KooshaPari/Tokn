# Tokn infrastructure reference.

Tokn ships with first-class container + orchestration support. This document
outlines what's available, how to bring the stack up locally, and how the same
artifacts translate to a real Kubernetes cluster.

## Layout

```
.
├── Dockerfile                       # Multi-stage rust:1.77-bookworm build
├── docker-compose.yml               # Local stack (Tokn + Prometheus + Grafana)
├── .devcontainer/devcontainer.json # GitHub Codespaces / VS Code Remote
├── deploy/
│   ├── k8s/deployment.yaml          # ReplicaSet, Service, ServiceMonitor
│   ├── prometheus.yml               # Scrape config for the Compose stack
│   └── grafana/
│       ├── provisioning/            # Datasource + dashboard auto-import
│       └── dashboards/              # Pre-built dashboards (JSON)
└── docs/INFRASTRUCTURE.md           # You are here.
```

## Local development

```bash
# 1. Bring the whole observability stack up.
docker compose up --build

# 2. Endpoints:
#    - Tokn metrics:        http://localhost:8443/metrics
#    - Tokn health:         http://localhost:8443/health
#    - Prometheus UI:       http://localhost:9090
#    - Grafana UI:          http://localhost:3000  (admin / admin)
#
# Grafana auto-loads the Tokn dashboard on first boot, so the only manual step
# is logging in and selecting the "Tokn" dashboard.
```

## Image build (standalone)

```bash
docker build \
    --build-arg TARGETOS=linux \
    --build-arg TARGETARCH=amd64 \
    -t tokn:dev .
docker run --rm -p 8443:8443 tokn:dev --help
```

The Dockerfile uses two stages: a `rust:1.77-bookworm` builder and a
`debian:bookworm-slim` runtime with a non-root user (uid 1000). The
`docker/dockerfile:1` syntax is implied; no buildkit-only features are required.

## Kubernetes

```bash
# Apply all manifests under deploy/k8s/.
kubectl apply -f deploy/k8s/

# Confirm pods are healthy.
kubectl get pods -l app=tokn
```

The manifests ship:

- **Deployment** — 2 replicas with anti-affinity (topology spread by hostname),
  liveness + readiness probes on `/health`, CPU/memory requests + limits, the
  `--target-os`/`--target-arch` build args threaded through.
- **Service** — ClusterIP exposing the metrics port.
- **ServiceMonitor** — Prometheus Operator scrape target so the kube-prom
  stack picks Tokn up automatically.

## GitHub Codespaces / VS Code Remote

`.devcontainer/devcontainer.json` declares a Rust 1.77 toolchain + the
GitHub CLI. The container post-creates a `cargo build` so the IDE launches
with a fully compiled workspace.

```bash
gh codespace create --repo KooshaPari/Tokn
```

## Observability wiring (Compose)

The bundled `deploy/prometheus.yml` scrapes the Tokn service on
`tokn:8443/metrics`. Grafana provisioning under
`deploy/grafana/provisioning/datasources/` declares Prometheus as a
default source, and `deploy/grafana/dashboards/` contains:

- **Tokn / Ledger** — ingest rate, queue depth, p50/p95/p99 ledger write
  latency, error ratio.

Dashboards are read-only — Grafana stores dashboards under `grafana-data/`
so user edits persist across restarts.

## Production hardening notes

- The base image is `debian:bookworm-slim`; upgrade to `bookworm-security`
  plus a cron job for `apt-get update && apt-get upgrade` for CVE patches.
- Scrape interval is 30s in the ServiceMonitor. For high-cardinality
  workloads, drop it to 15s and bump the retention to 30d.
- The non-root user (uid 1000) is wired so the image is `runAsNonRoot`
  ready. Add `securityContext.runAsNonRoot: true` to the pod spec for
  Pod Security Standard `restricted`.
- For air-gapped clusters, mirror `toki/kobi/tokn:latest` to the internal
  registry and override `image:` in the Deployment.

## See also

- `docs/DATA_MODEL.md` — schema + queries
- `docs/CONCURRENCY.md` — async design notes (added in Wave 4)
