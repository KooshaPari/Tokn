# Agent Rules

**This project is managed through AgilePlus.**

## AgilePlus Mandate

All work MUST be tracked in AgilePlus:
- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- CLI: `cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>`

## Branch Discipline

- Feature branches in `repos/worktrees/<project>/<category>/<branch>`
- Canonical repository tracks `main` only
- Return to `main` for merge/integration checkpoints

## Work Requirements

1. **Check for AgilePlus spec before implementing** (`repos/AgilePlus/scripts/list-features.sh` or `repos/AgilePlus/kitty-specs/<slug>/`).
2. **Check Phenotype Registries for patterns/specs**:
   - [PhenoSpecs](https://github.com/KooshaPari/PhenoSpecs) - Specifications & ADRs
   - [PhenoHandbook](https://github.com/KooshaPari/PhenoHandbook) - Patterns & guidelines
   - [HexaKit](https://github.com/KooshaPari/HexaKit) - Templates & scaffolding
   - [Master Index](https://github.com/KooshaPari/phenotype-registry)
3. **Track delivery** with `agileplus validate --feature <slug>`, `plan`, `implement`, `queue list`, and `agileplus cycle list` as appropriate.
4. **No code without corresponding AgilePlus spec**

## Agent Self-Correction & Verification Protocols

All agents MUST follow the verification protocols documented in `CLAUDE.md`:

### Critical: Dead Code Verification
- **NEVER** recommend deletion based on naming patterns alone (`*_old.py`, `*backup*`, etc.)
- **ALWAYS** verify via: import search, git history, replacement confirmation
- **Document findings** with actual evidence before marking for deletion

See `CLAUDE.md:Verification Protocols & Anti-Pattern Self-Correction` for full details.

## Technology Adoption Philosophy

**Aggressive adoption of preview/dev versions is encouraged across ALL languages and frameworks.**

### General Principles

- Do NOT gradually migrate - switch completely to new versions
- Do NOT be scared of non-release versions (dev, preview, rc, beta, nightly)
- Always prefer the latest available version, even if preview
- Rollback is always possible if issues arise
- Prefer native/compiler rewrites over legacy implementations

### Language-Specific Guidelines

| Language | Current Preview/Nightly Targets | Examples |
|----------|--------------------------------|----------|
| **TypeScript** | TypeScript 7 native (Go-based) | `tsgo` primary, `tsc` legacy escape hatch |
| **Rust** | Nightly compiler, latest editions | Edition 2024, latest nightly features |
| **Go** | Go 1.24+ (latest stable), experiment with generics fully | Use latest stable aggressively |
| **Python** | Python 3.14 dev, latest uv, ruff, rye | Preview Python when CI supports |
| **Bun** | Bun 1.2+ (already primary) | Node.js as legacy escape hatch |
| **React/Solid** | SolidJS primary for heliosApp | React as legacy escape hatch |

### Framework-Specific Guidelines

| Framework | Aggressive Target | Legacy Escape Hatch |
|-----------|------------------|---------------------|
| TypeScript Compiler | `tsgo` (TypeScript 7 native) | `tsc` (TypeScript 6.x) |
| Rust Async | `tokio` latest with `async fn` in traits | Manual `Pin<Box<dyn Future>>` |
| Python Package Mgr | `uv` (Rust-based) | `pip` + `venv` |
| Python Linter | `ruff` (Rust-based) | `flake8` + `black` |
| JavaScript Runtime | `bun` | `node` |
| Go Version | `1.24+` (generics, slog, slices) | None - Go versions are additive |

### Examples Applied

- **TypeScript 7 native preview** is primary, not legacy tsc
- **Rust Edition 2024** should be adopted immediately when available
- **Python 3.14** should be primary when CI supports it, not gradual migration
- **uv** should replace pip/venv completely, not side-by-side
- **Bun** is already primary over Node.js
- **SolidJS** is already primary over React in heliosApp

### Rollback Strategy

Always maintain a `:legacy` escape hatch command/script:
- `typecheck:legacy` for TypeScript 6.x
- `run:node` for Node.js execution
- `test:pytest` for pytest-based testing (if migrating to nextest/uv test)

## UTF-8 Encoding

All markdown files must use UTF-8. Avoid smart quotes, em-dashes, and special characters where they break tooling. There is no `agileplus validate-encoding` in the current CLI; rely on editor and pre-commit.

---

# Project Instructions

**Tokn** — Secure token management and identity infrastructure

## Project Summary

This project provides token lifecycle management including generation, validation, rotation, and revocation for secure service-to-service communication.

## Stack

| Layer | Technology | Version |
|-------|------------|---------|
| Language | Go | 1.24+ |
| Package Manager | go mod | Latest |
| Linter | golangci-lint | Latest |
| Test Runner | go test | Latest |

## Key Commands

```bash
# Development
go mod download              # Install dependencies
go test ./...                 # Run tests
golangci-lint run                 # Lint
gofumpt -w .               # Format
go vet ./...                 # Type check

# Build
go build                # Build package
```

## Structure

```
Tokn/
├── core/            # Token generation and validation
├── rotation/        # Automatic rotation
├── revocation/      # Revocation service
├── signing/         # Cryptographic signing
├── storage/         # Token metadata storage
├── sdk/             # Language SDKs
├── cli/             # CLI tools
└── audit/           # Audit logging
```

## Development Rules

- Follow the project charter defined in CHARTER.md
- Maintain code quality standards
- Write tests before implementation
- Document all public APIs

## Quality Gates

- Lint passes with 0 errors
- All tests pass
- Type checking passes
- Coverage meets project threshold

---

## Child-Agent and Delegation Policy

- Use child agents as the default for high-context, multi-file, or parallelizable work.
- Delegate exploration, audits, and long-running analysis to subagents before the parent agent edits.
- Keep parent-agent direct edits narrowly scoped to synthesis, integration, and finalization.

---

# Context Management Strategy

## The Manager Pattern

**CRITICAL**: Operate as a strategic manager, not a worker. Delegate to subagents.

### Keep in Main Context
- User intent and requirements
- Strategic decisions and trade-offs
- Summaries of completed work
- Critical architectural knowledge

### Delegate to Subagents
- File exploration (>3 files)
- Pattern searches across codebase
- Multi-file implementations
- Long command sequences
- Test execution

## Delegation Quick Reference

| Need | Delegate To | Example Prompt |
|------|-------------|----------------|
| Find code patterns | `Explore` | "Find all patterns" |
| Design approach | `Plan` | "Design strategy" |
| Run commands | `Bash` | "Run test suite" |
| Multi-step implementation | `general-purpose` | "Implement feature X" |
| Quick isolated fix | DO NOT delegate | Handle directly |

### Parallel vs Sequential

**Parallel** (no dependencies): Launch 2-3 explore agents simultaneously for independent searches.

**Sequential** (dependent): explore -> receive summary -> plan based on findings -> implement approved plan.

## Subagent Swarm (async orchestration)

**If you have subagent/swarm capabilities:** Use them as an **async swarm**.

- **Call task agents async.** Fire tasks so that as each completes, you are reawoken to re-evaluate, spawn more agents, or do more work yourself.
- **Run a swarm.** Up to **50 concurrent task agents**. Scale up when work is well decomposed and independent.
- **Work in between.** While tasks run async, use your own context for planning, monitoring, or other work.
- **Reawaken on completion.** When idle, you will be reawakened as each agent completes. Use that to spawn more agents, do follow-up work, or consolidate results.

## Anti-Patterns

| Bad | Good |
|-----|------|
| Reading 10 files to "understand" | Delegate exploration, get summary |
| Editing files for multi-file changes | Delegate to `general-purpose` |
| Sequential explorations one-by-one | Batch parallel explores |
| Asking subagent for "all results" | Ask for "summary" or "key files" |
| Committing all dirty worktree changes in one commit | Split into targeted, provenance-based commits |

## Dirty-Tree Commit Discipline (Required)

In dirty worktrees, separate commits by provenance:

- `MODE 1`: user-requested implementation changes.
- `MODE 2`: pre-existing work and WIP from other actors.
- `MODE 3`: generated or temporary artifacts.

Never mix modes in one commit. Prefer multiple small commits over one omnibus commit.

## Context Budget Rule

If task adds >2000 tokens of file content/output, **delegate it**.

---

# Optionality and Failure Behavior

**Require** dependencies where they belong; **require** clear, loud failures -- no silent or "graceful" degradation.

- **Force requirement where it belongs.** Do not make dependencies "optional" just to avoid failure. If a service or config is required for correctness, treat it as required and fail when missing.
- **Fail clearly, not silently.** Use explicit failures -- not reduced functionality, logging-only warnings, or hidden errors. Users must see *what* failed and that the process did not silently degrade.
- **Graceful in other ways.** Retries with visible feedback; error messages that list each failing item; actionable messages and non-obscure stack traces.

---

# Documentation Organization

**CRITICAL**: All project documentation follows a strict organization structure.

### Root-Level Files (Keep in Root)
- `README.md` -- Main project documentation
- `CHANGELOG.md` -- Project changelog
- `AGENTS.md` -- AI agent instructions
- `CLAUDE.md` -- Claude-specific instructions
- `00_START_HERE.md` -- Getting started guide (if applicable)
- Spec docs: `PRD.md`, `ADR.md`, `FUNCTIONAL_REQUIREMENTS.md`, `PLAN.md`, `USER_JOURNEYS.md`

### Documentation Structure

All other `.md` files must be organized in `docs/` subdirectories:

```
docs/
  guides/              # Implementation guides and how-tos
    quick-start/       # Quick start guides
  reports/             # Completion reports, summaries, status reports
  research/            # Research summaries, indexes, analysis
  reference/           # Quick references, API references, trackers
  checklists/          # Implementation checklists, verification lists
  changes/             # Per-change proposal/design/task docs
    archive/           # Completed change docs
```

### AI Agent Instructions

- **NEVER** create `.md` files in the project root (except allowed root-level files above)
- **ALWAYS** place new documentation in the appropriate `docs/` subdirectory
- **VERIFY** file location before creating documentation
- **MOVE** misplaced files to correct subdirectories if found

---

# Project Setup Checklist (Greenfield/Brownfield)

**MUST initialize these for ALL new projects:**

### 1. Docsite (VitePress)
- [ ] Copy `thegent/templates/vitepress-full/` to `docs/.vitepress/`
- [ ] Run `pnpm install && pnpm docs:build`
- [ ] Verify `docs-dist/index.html` opens in browser
- [ ] Add to CLAUDE.md

### 2. Taskfile (NOT Make)
- [ ] Create `Taskfile.yml` with standard tasks:
  - `lint` - Run all linters
  - `test` - Run tests
  - `quality` - Run quality gates
  - `docs:build` - Build docsite

### 3. Linters (Language-Specific)
| Stack | Linter | Formatter | Config Template |
|-------|--------|-----------|---------------|
| Python | ruff | ruff format | `thegent/templates/python/pyproject.template.toml` |
| TypeScript | oxlint | oxfmt/prettier | `thegent/templates/typescript/oxlint.config.json` |
| Go | golangci-lint | gofumpt | `thegent/templates/go/.golangci.yml` |
| Rust | clippy | rustfmt | `thegent/templates/rust/clippy.toml` |

### 4. Pre-commit Hooks
- [ ] Add `.pre-commit-config.yaml`
- [ ] Include: ruff-check, ruff-format, gitleaks, trailing-whitespace
- [ ] Run `pre-commit install`

### 5. Quality Gates
- [ ] Create `hooks/quality-gate.sh` with:
  - Lint check (0 errors)
  - Test check (all pass)
  - Coverage >= 80%
  - Security scan (0 high/critical)

### 6. Full Traceability Setup
- [ ] Create `FUNCTIONAL_REQUIREMENTS.md` with FR-{CAT}-NNN IDs
- [ ] Create `docs/reference/FR_TRACKER.md` to track FR implementation status
- [ ] Create `docs/reference/CODE_ENTITY_MAP.md` mapping code <-> requirements
- [ ] Add FR ID tags to all test functions

---

## Quick Project Initialization Commands

```bash
# NEW PROJECT - Full setup:
cd myproject
mkdir -p docs hooks

# 1. Docsite
cp -r thegent/templates/vitepress-full/* docs/.vitepress/

# 2. Install deps
# (language-specific)

# 3. Build & verify
# (language-specific)

# 4. Pre-commit
pre-commit install

# 5. Verify quality
# (language-specific)
```

**During work:**
- When making significant code changes, note which spec docs would need updating
- When completing a task, mentally check if trackers should be updated
- If you add new functions/modules, note they should be added to CODE_ENTITY_MAP.md

**On session end:**
- If there are unmapped code changes, acknowledge and update trackers if appropriate
- Treat session end as a documentation checkpoint

## Change Documentation (per-change, for significant changes)

For significant changes (new features, major refactors, architecture changes):
- Create `docs/changes/{change-name}/` with:
  - `proposal.md` -- What and why
  - `design.md` -- Technical approach, affected components
  - `tasks.md` -- Implementation checklist
- Archive completed changes to `docs/changes/archive/`
- NOT required for small fixes, typos, or minor adjustments

---

# Generalized Dev Environment Pattern

## Service Management

- **The user runs a dev TUI/dashboard in their own terminal.** **Never** start, stop, or restart the entire dev stack — only the user does that.
- **Use CLI introspection and per-service manipulation commands** to interact with the running stack.
- **Assume services use hot reload** — save files and let watchers pick up changes.
- **Read logs via CLI or log files** — never attach to the user's TUI terminal.

## Package Manager

**Use the project's preferred package manager.** Detect from lockfiles:
- `bun.lockb` or `bun.lock` -> use `bun`
- `pnpm-lock.yaml` -> use `pnpm`
- `yarn.lock` -> use `yarn`
- `package-lock.json` -> use `npm`

## Native Over Docker

**Prefer native services over Docker** for local development.

## OSS and Free First

**Strictly prefer local, OSS, and free tools** over paid SaaS.

---

AgilePlus Governance

- This repo uses AgilePlus for spec-driven development
- Feature specs live in `agileplus-specs/` (AgilePlus native format)
- Spec docs (PRD.md, ADR.md, FUNCTIONAL_REQUIREMENTS.md, PLAN.md) are maintained at repo root
- See the AgilePlus documentation for governance workflows

---

# QA Governance

## Test-First Mandate

- Write tests BEFORE implementation. Test file must exist before source file for new modules.
- For bug fixes, write a failing test that reproduces the bug first, then fix.

## Suppression Policy

- **Zero new suppressions** without inline justification comment.
- Include specific rule code AND a reason.

## Spec Traceability

- All test functions MUST reference an FR ID.
- Orphaned FRs (no test) and orphaned tests (no FR) are reported by the quality gate.

## Quality Gate Awareness

- `quality-gate.sh` runs on every Stop event.
- **Proactively run linters** before finishing work.

## Test Pyramid Targets

- **Unit**: 70% (tolerance: +/-5%)
- **Integration**: 20% (tolerance: +/-5%)
- **E2E**: 10% (tolerance: +/-5%)

## Hook Pipeline Summary (v3)

| Event | Hooks |
|-------|-------|
| SessionStart | spec-preflight, qa-preflight |
| PreToolUse:Write | doc-location-guard, pre-write-validator, suppression-blocker |
| PostToolUse:Edit|Write | change-doc-tracker, post-edit-checker, async-test-runner |
| Stop | quality-gate, stop-reconcile, spec-verifier, complexity-ratchet, security-pipeline, test-maturity |

## Test-First Development (TDD/BDD)

### TDD Mandate
- For NEW modules: test file MUST exist before implementation file
- For BUG FIXES: failing test MUST be written before the fix
- For REFACTORS: existing tests must pass before AND after

## Smart Contract Pattern (Spec Verification)
Specs (PRD/FR) -> Tests (must reference FR IDs) -> Checks (must be green) = Verified
- Every FR-XXX-NNN MUST have >=1 test referencing it
- Every test MUST reference >=1 FR-XXX-NNN (no orphan tests)
- All linters + type checkers + security scanners MUST pass (0 errors)
- Coverage MUST meet threshold

## Complexity Ratchet

Complexity must never increase. The ratchet enforcer:
- Measures cyclomatic complexity, cognitive complexity, maintainability index
- Max function: 40 lines. Max cyclomatic: 10. Max cognitive: 15.

## Security Pipeline

4-layer security scanning on every Stop:
1. Secret detection (gitleaks + regex patterns)
2. SAST (Semgrep, bandit, gosec)
3. Dependency audit (pip-audit, npm audit, govulncheck, cargo audit)
4. Infrastructure (tfsec, hadolint, trivy)

## Test Maturity Model

Projects are assessed on a 5-level scale:
- Level 1 — MVP: tests exist and are runnable
- Level 2 — Production-Ready: coverage >= 60%, integration tests
- Level 3 — Scale: coverage >= 80%, FR traceability >= 50%, security scanning
- Level 4 — High-Reliability: FR traceability >= 80%, architecture enforcement
- Level 5 — Mission-Critical: 100% FR traceability, mutation testing, chaos tests
Target: Level 3 for all projects, Level 4+ for critical systems.

## CI Completeness Policy

- Always evaluate and fix ALL CI check failures on a PR.
- Never dismiss a CI failure as "pre-existing" or "unrelated to our changes".
- After fixing CI failures, verify locally where possible before pushing.

## Phenotype Git and Delivery Workflow Protocol

- Use branch-based delivery with pull requests.
- Prefer stacked PRs for multi-part changes.
- Keep PRs linear and scoped: one concern per PR.
- Enforce CI and required checks strictly.
- Resolve all review threads before merge.
- Rebase or restack to keep branches current.

## Phenotype Org Cross-Project Reuse Protocol

- Treat this repository as part of the broader Phenotype organization.
- Actively identify code that is sharable across repositories.
- Prefer extraction into existing shared modules/projects first.
- Include a `Cross-Project Reuse Opportunities` section in plans.

## Phenotype Long-Term Stability and Non-Destructive Change Protocol

- Optimize for long-term platform value over short-term convenience.
- Prefer `stable_solution` unless an incident requires a temporary fix.
- Prefer targeted edits and forward fixes over deletions/reversions.
- Prefer moving obsolete material into `.archive/` over destructive removal.
- Do not merge any PR while any check is failing.

## Worktree Discipline

- Feature work goes in `.worktrees/<topic>/`
- Legacy `PROJECT-wtrees/` and `repo-wtrees/` roots are for migration only.
- Canonical repository remains on `main` for final integration.
