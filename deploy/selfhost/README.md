# Tokenledger Self-Hosted Deployment

Docker Compose stack for self-hosting Tokenledger CLI with data persistence and optional Cloudflare Tunnel support.

## Important: Tokenledger is a CLI Tool

**Tokenledger is not a web service.** It is a command-line tool for usage and cost analytics. This stack:

- Provides a **containerized CLI environment** with persistent data storage
- **Does not expose an HTTP API** by default
- Includes **Caddy** as a placeholder reverse proxy (serves a status page only)
- Supports **data volume mounting** for local CLI access
- Optionally integrates with **Cloudflare Tunnel** for external connectivity

If you need HTTP endpoints, build a separate analytics dashboard service that mounts the tokenledger-data volume and reads from Tokenledger outputs.

## Architecture

```
┌─────────────────────────────────────┐
│ Your CLI / Local Container          │
│ (mounts tokenledger-data volume)    │
│ Runs: tokenledger <command>         │
└─────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────┐
│ Tokenledger CLI Container           │
│ - Data storage at /data/             │
│ - No HTTP server                     │
└─────────────────────────────────────┘
        │
        ▼
  [analytics data]
```

## Quick Start

### 1. Build the Tokenledger Image

```bash
cd /path/to/tokenledger
docker build -t tokenledger:latest .
```

### 2. Create `.env.selfhost`

```bash
cat > .env.selfhost << 'EOF'
HOSTNAME=tokenledger.pheno.studio
CF_TUNNEL_TOKEN=
EOF
```

Add to `.gitignore`:
```
.env.selfhost
.env.*.local
```

### 3. Start the Stack

```bash
docker compose -f deploy/selfhost/docker-compose.selfhost.yml \
  --env-file .env.selfhost up -d
```

### 4. Access the CLI

```bash
# Run CLI commands against the container
docker compose -f deploy/selfhost/docker-compose.selfhost.yml \
  exec tokenledger-cli tokenledger --help

# Run monthly analysis
docker compose exec tokenledger-cli tokenledger monthly

# Run daily report
docker compose exec tokenledger-cli tokenledger daily

# Run pricing check
docker compose exec tokenledger-cli tokenledger pricing-check
```

## Using the Tokenledger CLI

### Available Commands

```bash
# Cost analytics
docker compose exec tokenledger-cli tokenledger monthly
docker compose exec tokenledger-cli tokenledger daily
docker compose exec tokenledger-cli tokenledger coverage

# Pricing operations
docker compose exec tokenledger-cli tokenledger pricing-check
docker compose exec tokenledger-cli tokenledger pricing-apply
docker compose exec tokenledger-cli tokenledger pricing-reconcile
docker compose exec tokenledger-cli tokenledger pricing-lint
docker compose exec tokenledger-cli tokenledger pricing-audit

# Data operations
docker compose exec tokenledger-cli tokenledger ingest
docker compose exec tokenledger-cli tokenledger bench
docker compose exec tokenledger-cli tokenledger benchmarks

# View full help
docker compose exec tokenledger-cli tokenledger --help
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `HOSTNAME` | Domain for Caddy status page (no API) |
| `CF_TUNNEL_TOKEN` | Cloudflare Tunnel token (optional) |

## Managing the Stack

```bash
# Stop
docker compose -f deploy/selfhost/docker-compose.selfhost.yml down

# View logs
docker compose -f deploy/selfhost/docker-compose.selfhost.yml logs -f

# Backup data
docker run --rm -v tokenledger-data:/data -v $(pwd):/backup \
  ubuntu tar czf /backup/tokenledger-backup.tar.gz -C /data .
```

## Building a Dashboard (Optional)

Create a lightweight analytics dashboard that:

1. Mounts the `tokenledger-data` volume
2. Reads Tokenledger output files
3. Exposes HTTP endpoints for visualization

The dashboard could be a React/Vue app served by Caddy that queries the analysis results.

## Troubleshooting

```bash
# Check container status
docker compose ps

# View detailed logs
docker compose logs tokenledger-cli

# Test CLI directly
docker compose exec tokenledger-cli tokenledger --help

# Verify data directory exists
docker compose exec tokenledger-cli ls -la /data/
```

## Security

- Never commit `.env.selfhost` to version control
- Restrict data directory permissions (700 or 750)
- Use Tailscale for private access instead of exposing on public internet
- Rotate API keys regularly if integrating with external pricing APIs

## Updates

```bash
# Update Tokenledger image
docker pull tokenledger:latest
docker compose -f deploy/selfhost/docker-compose.selfhost.yml up -d tokenledger-cli

# Update Caddy
docker compose -f deploy/selfhost/docker-compose.selfhost.yml pull caddy
docker compose -f deploy/selfhost/docker-compose.selfhost.yml up -d caddy
```

## Production Checklist

- [ ] Data backups automated and tested
- [ ] `.env.selfhost` is in `.gitignore`
- [ ] Caddy status page accessible
- [ ] Analytics dashboard built (optional)
- [ ] Cloudflare Tunnel configured (if external access needed)
- [ ] Regular Docker image updates scheduled

---

**Version:** 1.0  
**Status:** CLI-only (no HTTP server)
