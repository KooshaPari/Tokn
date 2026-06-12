<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->
![Downloads](https://img.shields.io/github/downloads/KooshaPari/Tokn/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/Tokn?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/Tokn?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.
<!-- AI-DD-META:END -->
> **Work state:** ACTIVE · **Progress:** `███████░░░ 65%`
> Token-ledger / usage accounting (tokenledger) **and the canonical Phenotype Rust ROUTING substrate** — `tokenledger::routing` (hexagonal: pareto_router/ports/adapters). Consumed by OmniRoute per ADR-001. · updated 2026-06-03

![Build Status](https://github.com/KooshaPari/Tokn/actions/workflows/quality-gate.yml/badge.svg)
![Security Audit](https://github.com/KooshaPari/Tokn/actions/workflows/security-guard.yml/badge.svg)
![Policy Compliance](https://github.com/KooshaPari/Tokn/actions/workflows/policy-gate.yml/badge.svg)

# Tokn (tokenledger)

**Status:** alpha

Enterprise-grade token management and pricing governance system for AI coding agents.

This repository works with Claude and other AI agents as autonomous software engineers.

## Workspace

This is a Rust workspace with two main crates:

- **tokenledger** — Enterprise-grade token management and pricing governance system. Provides unified token and cost tracking across multiple AI provider APIs with optimization recommendations.
- **ParetoRs** — Pareto-optimal cost engine for AI coding agents. Delivers cost optimization and resource allocation algorithms for multi-provider agent orchestration.

## Quick Start

```bash
# Development
cargo run

# Testing
cargo test

# Linting
cargo clippy
```

## Environment

```bash
# Required environment variables
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-..."
```

---

## Development Philosophy

### Extend, Never Duplicate

- NEVER create a v2 file. Refactor the original.
- NEVER create a new class if an existing one can be made generic.
- NEVER create custom implementations when an OSS library exists.
- Before writing ANY new code: search the codebase for existing patterns.

### Primitives First

- Build generic building blocks before application logic.
- A provider interface + registry is better than N isolated classes.
- Template strings > hardcoded messages. Config-driven > code-driven.

### Research Before Implementing

- Check crates.io for existing libraries.
- Search GitHub for 80%+ implementations to fork/adapt.

---

## Library Preferences (DO NOT REINVENT)

| Need | Use | NOT |
|------|-----|-----|
| Async runtime | tokio | custom async |
| HTTP client | reqwest | custom wrappers |
| Logging | tracing | print() or log::logger |
| CLI | clap | manual arg parsing |
| Validation | validator | manual if/else |
| Database | sqlx | raw SQL strings |
| Rate limiting | governor | custom rate limiter |

---

## Code Quality Non-Negotiables

- Zero new lint suppressions without inline justification
- All new code must pass: cargo clippy, cargo fmt, tests
- Max function: 40 lines
- No placeholder TODOs in committed code
- All Rust code must be clippy-clean

---

## Verifiable Constraints

| Metric | Threshold | Enforcement |
|--------|-----------|-------------|
| Test coverage | >= 80% | cargo-tarpaulin |
| Security findings | 0 high/critical | cargo-audit |
| Clippy warnings | 0 | CI gate |

---

## Domain-Specific Patterns

### What tokenledger Is

tokenledger is a **token and cost tracking system** for AI coding agents. The core domain is: provide unified token and cost tracking across multiple providers with optimization recommendations.

### Key Interfaces

| Interface | Responsibility |
|-----------|---------------|
| CLI commands | report, costs, optimize |
| Provider traits | Multi-provider abstraction |
| Storage | SQLite, PostgreSQL support |

---

## Integration

### With thegent

```python
# thegent config
llm:
  provider: cliproxy
  base_url: http://localhost:8317/v1
```

### With agentapi

```bash
agentapi --cliproxy http://localhost:8317
```

---

## Governance & Development

**Project Name**: tokenledger (code name: Tokn)  
**AgilePlus Tracking**: All work tracked in `/repos/AgilePlus`. Review `CLAUDE.md` for development policies.

**Quality Standards**:
- **Zero new lint suppressions** without inline justification
- **80% test coverage minimum** (cargo-tarpaulin)
- **Zero high/critical security findings** (cargo-audit)
- **Max 40 lines per function**, zero placeholder TODOs

**Quick Commands**:
```bash
cargo build                 # Development build
cargo test                  # Test suite
cargo clippy               # Lint check
cargo audit                # Security scan
cargo tarpaulin            # Coverage report
```

## Integration & Adoption

**With thegent**: Configure as LLM provider proxy for agent token routing and cost optimization.

**With agentapi**: Use as cost-tracking backend for multi-agent orchestration across provider networks.

**Extensibility**: Implement `Provider` trait to add new token/cost models.

## Related Phenotype Projects

- **[Sidekick](../Sidekick)** — Agent dispatch & routing
- **[cheap-llm-mcp](../cheap-llm-mcp)** — Cost-optimized LLM routing
- **[AgilePlus](../AgilePlus)** — Specification hub
- **[thegent](../thegent)** — Agent execution framework

## License

MIT License - see LICENSE file

**Status**: Active development (agent provider expansion)  
**Maintained by**: Phenotype Org  
**Last Updated**: 2026-04-24