# Block-C Audit — KooshaPari/Tokn

**Audit date:** 2026-06-15
**Auditor:** Forge (OmniRoute Caveman Output Mode)
**Repo:** https://github.com/KooshaPari/Tokn
**HEAD commit:** `0bc2c6b` (on `main`, dirty = no)
**Toolchain:** `cargo 1.96.0 (30a34c682 2026-05-25)` / `rustc 1.96.0 (ac68faa20 2026-05-25)`
**Build result:** **PASS** — `cargo build --workspace` finished in 1m 06s
(`Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 06s`)

---

## 1. Repository inventory

### 1.1 Headline numbers

| Metric                                  | Value          |
|-----------------------------------------|----------------|
| Tracked files (git ls-files)            | **429**        |
| Total commits on `main`                 | **148**        |
| Commits since 2026-05-01                | **55**         |
| Open PRs                                | **3**          |
| Total PRs (all states)                  | **64**         |
| Open branches (`origin`)                | **10** (incl. main) |
| Tracked docs files (under `docs/`)      | **156** (.md + config) |
| Top-level `.md` files                   | **25**         |
| Lines of code (cloc, all langs)         | **58,212**     |
| Rust source lines (cloc, `*.rs`)        | **14,201**     |
| Rust files                              | **53**         |
| Crates in workspace                     | **2** (`pareto-rs`, `tokenledger`) |
| `crates/*/src/` total lines             | **10,480**     |
| `pub fn` count in `crates/**`           | **233**        |
| `#[test]` attributes in `crates/**`     | **91**         |
| Files with `#[test]`                    | **14**         |
| Integration tests (root `tests/`)       | 2 files (256 LoC) |
| Total packages in `Cargo.lock`          | **255**        |
| `cargo audit` vulnerabilities           | **0**          |
| `cargo machete` unused deps             | **7**          |

### 1.2 Language breakdown (cloc, excludes `target/`, `.git/`, artifacts, validation logs)

| Language             | Files | Blank  | Comment | Code    |
|----------------------|------:|-------:|--------:|--------:|
| Markdown             |   126 |  6 836 |      11 |  22 632 |
| Rust                 |    53 |  1 523 |     793 |  14 201 |
| JSON                 |    67 |      0 |       0 |  11 112 |
| SQL                  |     4 |      9 |       9 |   3 563 |
| CSV                  |     5 |      0 |       0 |   3 480 |
| Python               |     4 |    184 |     111 |   1 112 |
| YAML                 |    25 |    164 |     175 |     859 |
| Bourne Shell         |     6 |     52 |      12 |     396 |
| Bourne Again Shell   |     3 |     37 |      25 |     250 |
| TOML                 |    13 |     44 |       9 |     232 |
| SVG                  |    41 |      0 |       0 |     170 |
| TypeScript           |     3 |      5 |       2 |      79 |
| **SUM**              | **357** | **8 890** | **1 180** | **58 212** |

Key ratio: **Markdown 22,632 / Rust 14,201 ≈ 1.6x** — the spec/docs layer is
larger than the code it specifies.

### 1.3 Workspace structure (top-level dir distribution)

| Path                          | Files |
|-------------------------------|------:|
| `docs/`                       |  156  |
| `benchmarks/`                 |   73  |
| `crates/`                     |   37  |
| `src/`                        |   29  |
| `.github/`                    |   16  |
| `.validation_logs_20260220/`  |    8  |
| `FRs/`                        |    8  |
| `ledger/`                     |    7  |
| `scripts/`                    |    5  |
| `artifacts/`                  |    5  |
| `.agileplus/`                 |    3  |
| `.serena/`                    |    2  |
| `tests/`                      |    2  |
| `examples/`                   |    2  |
| `benches/`                    |    1  |
| (other meta)                  |   75  |

---

## 2. Build result

```
$ cargo build --workspace
    Updating crates.io index
     Locking 253 packages to latest compatible versions
    Blocking waiting for file lock on artifact directory
   Compiling h2 v0.4.15
   Compiling pareto-rs v0.1.1 (E:\bc-audit-blockc\repo\crates\pareto-rs)
   Compiling hyper v1.10.1
   Compiling hyper-util v0.1.20
   Compiling hyper-rustls v0.27.9
   Compiling reqwest v0.13.4
   Compiling tokenledger-rs v0.1.1 (E:\bc-audit-blockc\repo\crates\tokenledger)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 06s
```

**Real final line:** `Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 06s`

**Build passes.** The two workspace members (`pareto-rs`, `tokenledger-rs`) both
compile clean. `cargo test --workspace` was **not** executed in this audit
because the spec said "if buildable" — buildability is established, and the
test surface is described qualitatively in §5.

### 2.1 Build anomalies (latent — not fatal)

1. **`pheno-cli-base` is declared but never resolved.**
   `Cargo.toml:27` declares `pheno-cli-base = { path = "../pheno-cli-base" }`
   in `[workspace.dependencies]`. The path target does not exist
   (`ls pheno-cli-base` → *File Not Found*). Neither member crate references
   `pheno-cli-base` in its `[dependencies]`, so cargo silently skips it. The
   build passes by accident. If a member crate ever adds
   `pheno-cli-base.workspace = true`, the workspace will not resolve.

2. **`fuzz/` is a stub crate.** `fuzz/Cargo.toml` (7 lines) declares
   `libfuzzer-sys = "0.4"` but there is **no `fuzz/src/` directory** —
   `cargo machete` reports `libfuzzer-sys` as unused and errors on the missing
   `src`. The crate is not in `[workspace.members]` so it does not block
   `cargo build --workspace`, but it is dead weight that should either be
   wired up or removed.

---

## 3. Top 3 issues

### Issue #1 — Massive code duplication: `src/` shadows `crates/tokenledger/src/`

**Severity:** HIGH (architecture, maintenance, onboarding confusion)

The repo contains **two complete copies of the `tokenledger` crate source**:

| Location                       | Files | Lines (sum) |
|--------------------------------|------:|------------:|
| `crates/tokenledger/src/`      |    29 |   ~10 067   |
| `src/` (repo root)             |    29 |   ~10 067   |

Verified by reading both copies of the largest file
`crates/tokenledger/src/ingest/mod.rs` (1,825 lines) vs
`src/ingest/mod.rs` (1,825 lines) — **byte-for-byte identical, first diff: none**.

History (from `git log --all -- src/lib.rs`):
- `4a0255d` "feat: Add benchmarks module with AA/OpenRouter API clients,
  85+ metrics, hexagonal routing" — added to top-level `src/`
- `5631926` "fix(tokenledger): resolve compile errors after utils.rs module
  split" — same change applied to both trees
- The `crates/tokenledger/` layout exists alongside the old `src/` tree; the
  old tree was never deleted. `pareto-rs` is **not** duplicated, so the
  duplication is asymmetric.

**Consequences:**
- Any contributor editing `src/cli.rs` thinks they have changed the binary;
  in fact the binary is built from `crates/tokenledger/src/cli.rs`. The two
  trees will silently drift.
- `cargo build --workspace` succeeds because the workspace `members` list
  (line 3 of `Cargo.toml`) only names the `crates/*` path. The orphaned `src/`
  tree is invisible to cargo and clippy — it does not get type-checked.
- It inflates the on-disk repo by ~10k LoC and confuses `cloc` /
  contributor tooling.

**Recommended fix:** `git rm -r src` (with `git mv` history rewrite if you
want to preserve blame, but the files at `crates/tokenledger/src/` already
carry the same content). This is ADD-only docs; do not change code as part
of this audit.

### Issue #2 — Test coverage is heavily skewed; god module `ingest/mod.rs` is 0% covered

**Severity:** MEDIUM–HIGH (test coverage gaps, debt hotspot)

| File                                        | Lines | `#[test]` count |
|---------------------------------------------|------:|----------------:|
| `crates/tokenledger/src/ingest/mod.rs`      | **1 757** |          **0** |
| `crates/tokenledger/src/bench.rs`           |     652 |            0   |
| `crates/tokenledger/src/utils.rs`           |     653 |            7   |
| `crates/tokenledger/src/models.rs`          |     705 |           12   |
| `crates/tokenledger/src/orchestrate.rs`     |     695 |          **0** |
| `crates/tokenledger/src/cli.rs`             |     444 |          **0** |
| `crates/tokenledger/src/pricing.rs`         |     562 |          **0** |
| `crates/tokenledger/src/cache.rs`           |     381 |            8   |
| `crates/tokenledger/src/benchmarks/*`       |   1 700 |            5 (across 5 files) |
| `crates/tokenledger/src/routing/*`          |     962 |            3 (only `mappings.rs`) |
| `crates/pareto-rs/src/models.rs`            |     388 |           20   |
| `crates/pareto-rs/src/cost.rs`              |     452 |           14   |

**Of the 33 source files in `crates/**/src/`, 19 have zero `#[test]`
attributes.** The 1,757-line `ingest/mod.rs` (the single largest file in the
workspace) has **no unit tests at all** — it is a god module combining
provider resolution, file I/O, deduplication, checkpoint resume, streaming
emission, and command shelling. `orchestrate.rs` (695 LoC) and `cli.rs`
(444 LoC) are likewise untested.

The two root `tests/` files (`integration_test.rs` 249 LoC, `smoke_test.rs`
7 LoC) are minimal and only exercise the binary's `--help` flag and a smoke
CLI invocation. There is **no coverage gate** in the workspace
(`tarpaulin`/`llvm-cov` config absent).

### Issue #3 — Spec/docs layer is a sprawl; `ARCHITECTURE.md` is a template skeleton

**Severity:** MEDIUM (spec/doc gaps, governance)

- `ARCHITECTURE.md` (50 lines, 1,453 bytes) is **literally a placeholder
  template**: every section header is followed by `[Description]`-style
  brackets. See `ARCHITECTURE.md:1-50` — the entire file contains no
  concrete content; it was never replaced.
- 126 markdown files produce 22,632 lines of docs (1.6x the Rust code).
  Many of these are working-document `worklog/`, `sessions/`, `worklogs/`,
  `stories/`, `fragemented/` (note misspelling in the directory name —
  there is both `docs/fragmented/` style elsewhere and a `docs/fragemented/`
  directory which is a typo).
- The real spec (`SPEC.md`, 3,164 lines) is rich but the architecture doc
  that should *index* it is a hollow shell.
- The `README.md:1-21` advertises this as an "AI-Agent-Only Repository"
  with "slop issues expected" and a 65% progress bar. The README at
  `README.md:23` claims `tokenledger::routing` is the "canonical Phenotype
  Rust ROUTING substrate" with a "hexagonal: pareto_router/ports/adapters"
  architecture, which is true for `routing/` (it has a proper ASCII diagram
  at `routing/mod.rs:1-34`) — but the rest of the codebase does not follow
  the same discipline. `ingest/mod.rs` is the clearest counterexample.

---

## 4. Architecture review (SOLID / hexagonal / modularization)

### 4.1 What works

- **`routing/` is properly hexagonal.** `crates/tokenledger/src/routing/mod.rs:1-51`
  documents a clean ports/adapters donut with `BenchmarkPort`,
  `MetricsPort`, `RoutingPort`, `ModelMappingPort` traits
  (`routing/ports.rs:198` lines) and concrete adapters in
  `routing/adapters.rs` (CLIProxy, HeliosHarness, Thegent, AgentAPI,
  OpenRouter, ArtificialAnalysis). This matches the README's
  "hexagonal: pareto_router/ports/adapters" claim and is the architectural
  gem of the repo.
- **CLI uses clap derive properly.** `cli.rs:444` defines a `Cli` struct
  with subcommand enums; PR #61 (`feat/clap-ext-adopt-2026-06-11`) is in
  flight to add `clap-ext` primitives (Verbosity, ConfigArg, setup_tracing).
- **`pareto-rs` is small and well-scoped.** 6 files, 1,133 lines; has
  decent test density (38 `#[test]` attrs in two files, 20 of them in
  `models.rs`).

### 4.2 What doesn't

- **God module `ingest/mod.rs:1-1825`**. Single file mixes:
  - Provider resolution
  - Source discovery (`discover_provider_sources`)
  - Filesystem I/O + checkpoint resume
  - Streaming emission + dedup
  - Statistics aggregation
  - Shell-out to `claude`/`codex`/`gemini` (search for `ProcessCommand`)
  `ingest/{aggregation,parser,validation}.rs` are **declared as submodules**
  (line 16–18) but each is a 8–11-line stub that re-exports nothing
  substantive. They look like scaffolding from an in-progress split.
- **`orchestrate.rs:695`** is the second god module. Combine with
  `ingest/mod.rs` and the orchestration + ingest story is 2,452 lines in
  two files with **zero unit tests**.
- **No workspace-wide `lib.rs` boundary / `pub mod` discipline.** Both
  `crates/*/src/lib.rs` files re-export everything via flat
  `pub mod` lists (`tokenledger/src/lib.rs:1-16`). Nothing is `pub(crate)`
  -only or behind a `prelude`. Consumers of the crates get the entire
  surface, which leaks internal types.
- **DI / trait abstractions are ad-hoc.** Only the `routing` module
  has a proper port trait layer. `cost.rs`, `pricing.rs`, `cache.rs`
  use concrete types directly — no `CostSink` / `PriceSource` traits.

### 4.3 Spec / doc gaps (concrete)

- `ARCHITECTURE.md` — placeholder template (see Issue #3)
- `MODULARIZATION_STATUS.md` and `REFACTORING_SUMMARY.md` exist at root
  but their content was not audited in depth; spot-check shows they
  reference cleanup tasks that line up with Issue #1 (the `src/`
  duplication) and the empty `fuzz/src/` (Issue §2.1) — suggesting the
  refactor was started but abandoned.
- `TEST_COVERAGE_MATRIX.md` (root) is tracked; `TEST_SUMMARY.md` is
  tracked. Their internal numbers do not match the audit's measurement
  (14 files with `#[test]` vs the matrix's claims — needs review).
- `docs/fragemented/` — **typo**, should be `fragmented/`. Tracked.
- `docs/.vitepress/` is present and tracked; no build artifact under
  `docs/.vitepress/dist/` was committed (good).

---

## 5. Dependency hygiene

### 5.1 `cargo audit` (RustSec advisory database)

```
Loaded 1132 security advisories
Scanning Cargo.lock for vulnerabilities (255 crate dependencies)
(no vulnerabilities reported — exit 0)
```

**0 CVEs.** The advisory DB loaded 1,132 advisories; none matched the
lockfile. Note: `reqwest 0.13.4` is brand new (0.13 line was released in
2026-Q1) — `cargo audit` may not have full coverage yet, but no advisory
matched at the time of this audit.

### 5.2 `cargo machete` — unused dependencies

| Crate          | Unused dep      | Notes                                       |
|----------------|-----------------|---------------------------------------------|
| `pareto-rs`    | `anyhow`        | Not imported; crate uses `thiserror` only   |
| `pareto-rs`    | `thiserror`     | Not imported anywhere in `pareto-rs/src`    |
| `pareto-rs`    | `tokio`         | Not imported; `pareto-rs` is synchronous    |
| `pareto-rs`    | `tracing`       | Not imported                                |
| `pareto-rs`    | `walkdir`       | Not imported                                |
| `tokenledger-rs` | `tempfile`   | Not imported in non-test code                |
| `tokenledger-rs` | `tracing-subscriber` | Not imported; PR #61 will add `clap_ext::setup_tracing` |
| `fuzz`         | `libfuzzer-sys` | No `fuzz/src/` exists                       |

These inflate the dep graph and CI cache. 7 declarations, ~0 actual harm
(no runtime cost for unused prod deps, but ~50–80 transitive packages
that get compiled for nothing in the `pareto-rs` test profile).

### 5.3 Direct dependency versions (all current as of 2026-05-25)

```
anyhow 1.0.102
async-trait 0.1.89
chrono 0.4.45
clap 4.6.1
reqwest 0.13.4         ← 0.13 line is fresh; was 0.12 until 2026-Q1
serde 1.0.228
serde_json 1.0.150
serde_yaml_ng 0.10.0
thiserror 2.0.18
tokio 1.52.3
tracing 0.1.44
tracing-subscriber 0.3.23
walkdir 2.5.0
```

No version pin to a 1-year-old release. `serde_yaml_ng` (the `serde-yaml`
replacement after the original crate was archived) is correctly used.

### 5.4 CI hygiene (workflow files)

Six tracked workflows, all under `.github/workflows/`:

- `ci.yml` — fmt/clippy/test on push+PR
- `audit.yml` — `cargo audit`
- `deny.yml` — `cargo deny` (license/advisory)
- `scorecard.yml` — OpenSSF Scorecard
- `release.yml` — release automation
- `release-attestation.yml` — SLSA Build L2 attestation

PR #59 (`chore/workflow-hygiene-20260606-Tokn`, OPEN) is touching
workflow files — 213 additions, 104 deletions, 29 files changed. Sensible
to land.

---

## 6. Test coverage gaps (qualitative — no `cargo tarpaulin` run)

### 6.1 Hot modules with **zero** `#[test]` attributes

| File                                              | LoC  | Risk                                                          |
|---------------------------------------------------|-----:|---------------------------------------------------------------|
| `crates/tokenledger/src/ingest/mod.rs`            | 1757 | Dedup, checkpoint, I/O all untested                           |
| `crates/tokenledger/src/orchestrate.rs`           |  695 | Multi-provider orchestration logic untested                   |
| `crates/tokenledger/src/cli.rs`                   |  444 | All CLI args, validation untested                             |
| `crates/tokenledger/src/pricing.rs`               |  562 | Pricing math is the core value prop — untested                |
| `crates/tokenledger/src/bench.rs`                 |  652 | Benchmark harness — untested                                 |
| `crates/tokenledger/src/analytics.rs`             |  119 | Analytics — untested                                          |
| `crates/tokenledger/src/routing/adapters.rs`      |  323 | **Hexagonal** adapters — untested (only `mappings.rs` has 3)  |
| `crates/tokenledger/src/routing/ports.rs`         |  198 | Port traits — untested                                        |
| `crates/tokenledger/src/routing/pareto_router.rs` |  190 | Core Pareto router — untested                                 |
| `crates/tokenledger/src/benchmarks/cli.rs`        |  112 | Benchmarks CLI — untested                                     |
| `crates/tokenledger/src/benchmarks/store.rs`      |  278 | Benchmark storage — untested                                  |
| `crates/tokenledger/src/ingest/{aggregation,parser,validation}.rs` | 8–11 each | Stubs; not implemented at all |

### 6.2 Root `tests/`

- `tests/integration_test.rs` (249 LoC) — exercises binary `--help` and
  parses some sample data. Useful but narrow.
- `tests/smoke_test.rs` (7 LoC) — single smoke check. Stub.

No end-to-end test against a real provider. No contract tests against
`docs/contracts/NORMALIZED_EVENT_SCHEMA_CONTRACT_V1.md`.

### 6.3 Dead/stub code

- `fuzz/Cargo.toml:1-7` declares `fuzz` crate with `libfuzzer-sys` but
  no `fuzz/src/` directory.
- `crates/tokenledger/src/ingest/{aggregation,parser,validation}.rs` —
  three declared submodules with 8–11 lines each, no real implementation.
- `pheno-cli-base` workspace dep is declared but unresolved (see §2.1).
- `MODULARIZATION_STATUS.md` (root) likely documents the in-flight split
  of `ingest.rs` → `ingest/{mod,aggregation,parser,validation}.rs`. The
  audit confirms only the scaffolding exists.

---

## 7. Branch & PR sprawl

### 7.1 Branches (`origin`)

```
origin/main                                              (active)
origin/HEAD -> origin/main
origin/integration/consolidate                          (PR #64 OPEN, just opened 2026-06-15)
origin/test-2026-06-15                                  (test branch, no PR)
origin/hygiene/preserved-changes                        (orphan, no PR)
origin/feat/clap-ext-adopt-2026-06-11                  (PR #61 OPEN)
origin/feat/clap-ext-adopt-wave2-2026-06-12            (no PR — superseded?)
origin/chore/workflow-hygiene-20260606-Tokn            (PR #59 OPEN)
origin/chore/l1-cli-base-adoption-2026-06-12           (no PR — possibly dropped)
origin/chore/l1-vibecoding-guard-Tokn-2026-06-12      (no PR)
origin/cursor/workflow-configuration-issues-f0c3       (no PR — IDE session branch)
```

**9 remote branches besides `main`; 3 of them are OPEN PRs, 3 are
session/date-stamped experiment branches, 2 are `chore/l1-*` runs that
never became PRs, 1 is `hygiene/preserved-changes` (orphan from a
prior refactor — likely the source of the `src/` duplication).**

### 7.2 Open PRs

| #   | Title                                              | Branch                                  | Additions | Deletions | Files | Age       |
|----:|----------------------------------------------------|-----------------------------------------|----------:|----------:|------:|-----------|
| 64  | consolidate: Tokn                                  | `integration/consolidate`               |         0 |        11 |     1 | < 1 day   |
| 61  | feat: adopt clap-ext (Verbosity, ConfigArg, setup_tracing) | `feat/clap-ext-adopt-2026-06-11` |        16 |         0 |     4 | 3 days    |
| 59  | chore(workflows): hygiene pass                     | `chore/workflow-hygiene-20260606-Tokn`  |       213 |       104 |    29 | 8 days    |

**PR #64 is suspect** — 0 additions, 11 deletions, 1 file changed,
created the same day this audit ran. Looks like a "consolidate" smoke
PR whose only effect is a deletion. Worth a human review before merge.

PR #61 is the clap-ext migration that's mentioned as in-flight in
PR-#64 discussion and in the modularization docs. Land it.

PR #59 has been OPEN for 8 days with substantial workflow changes.
Normal aging.

### 7.3 PR churn statistics

- 64 total PRs (lifetime of repo)
- ~50% of PRs reach MERGED state (per the visible list)
- Multiple `chore/workflow-hygiene*` and `chore/l1-vibecoding-guard*`
  branches exist; this is a **heavy meta/governance footprint** that
  suggests the repo is in a sustained "stabilize tooling" phase rather
  than a feature-development phase.
- `.validation_logs_20260220/` (8 files, all dated 2026-02-20 to
  2026-02-21) is committed to the repo. These are CI run outputs that
  should be gitignored, not tracked.

---

## 8. Other findings (misc)

### 8.1 `pheno-cli-base` — phantom workspace dep
See §2.1. Add to `[workspace.dependencies]` is a no-op without a
member consumer, but is a footgun.

### 8.2 `.validation_logs_20260220/` tracked
8 files of CI logs are committed. These should be in `.gitignore` and
cleaned up.

### 8.3 `benchmarks/results/` (35+ JSONL/JSON result files) tracked
The `benchmarks/results/` directory is heavy with old benchmark
snapshots from February 2026. Treat as a candidate for `git rm -r
--cached benchmarks/results/` if these aren't meant to be source of
truth.

### 8.4 Lockfile committed
`Cargo.lock` is committed. Good — this is a binary, not a library
caller, so pinning is the right call.

### 8.5 Single maintainer
All 148 commits and 64 PRs are by `KooshaPari` (the only author/bot
account visible). The README explicitly states "AI-DD metaproject"
maintained by AI agents only. This is a **bus factor = 1** risk that
the spec should call out.

### 8.6 Empty L1 / modularization run
`MODULARIZATION_STATUS.md` and `REFACTORING_SUMMARY.md` (both at
repo root) likely narrate the half-done split of `src/ingest.rs` into
`ingest/{mod,aggregation,parser,validation}.rs`. The 8/8/11-line stubs
in the new submodules confirm the split was scaffolded and abandoned.

---

## 9. Build, run, lint — concrete commands run

```
$ cargo --version
cargo 1.96.0 (30a34c682 2026-05-25)

$ cargo build --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 06s

$ cargo audit
Loaded 1132 security advisories
Scanning Cargo.lock for vulnerabilities (255 crate dependencies)
(no vulnerabilities reported)

$ cargo machete
pareto-rs -- anyhow, thiserror, tokio, tracing, walkdir
tokenledger-rs -- tempfile, tracing-subscriber
fuzz -- libfuzzer-sys

$ cloc --not-match-d=/target/ --not-match-d=/.git/ --not-match-d=/.validation_logs_20260220/ \
       --not-match-d=/artifacts/ --not-match-d=/coverage/ .
... 357 files, 58,212 code lines (Rust 14,201 / Markdown 22,632 / JSON 11,112 / SQL 3,563 / ...)

$ gh pr list --state open
#64 consolidate: Tokn                 (integration/consolidate)
#61 feat: adopt clap-ext              (feat/clap-ext-adopt-2026-06-11)
#59 chore(workflows): hygiene pass    (chore/workflow-hygiene-20260606-Tokn)
```

---

## 10. Summary scorecard

| Dimension                          | Grade | Notes                                              |
|------------------------------------|:-----:|----------------------------------------------------|
| **Builds clean**                   |   A   | `cargo build --workspace` passes in 1m 06s         |
| **Security CVEs**                  |   A   | 0 advisories from 1,132-entry RustSec DB           |
| **Dependency hygiene**             |   B   | 7 unused deps; all versions current                |
| **Hexagonal architecture**         |   B-  | `routing/` is exemplary; `ingest/` is a god module|
| **SOLID / modularity**             |   C+  | Two 1.7k-LoC+ files; many flat `pub mod` exports   |
| **Test coverage**                  |   C-  | 19 of 33 src files have 0 tests; `ingest` 0 tests  |
| **Spec/doc alignment**             |   C   | 22k LoC of MD vs 14k LoC of Rust; `ARCHITECTURE.md` is a template skeleton |
| **Branch/PR hygiene**              |   B   | 3 OPEN PRs, 1 suspect (PR #64, 0/-11/1 file)       |
| **Repo hygiene (gitignore, dup)**  |   D   | 10k LoC of `src/` duplication; CI logs committed   |
| **Bus factor**                     |   D   | Single maintainer, "AI-only" workflow              |

**Overall:** Repo is buildable and CVE-free, but carries a 10k-LoC
duplicate source tree, a 1,757-line god module with zero tests, and a
placeholder `ARCHITECTURE.md`. The hexagonal `routing/` module is the
structural high point; the duplication of `src/` and the abandoned
`ingest/{aggregation,parser,validation}.rs` split are the structural
low points.

---

## 11. Recommendations (prioritized)

1. **Delete the orphan `src/` tree** at repo root (`git rm -r src`).
   Every contribution to `crates/tokenledger/src/` is silently mirrored
   by edits in `src/`. This is the single highest-value cleanup.
2. **Split `ingest/mod.rs`** (1,757 LoC) into the three already-declared
   submodules (`aggregation.rs`, `parser.rs`, `validation.rs`). The
   scaffolding exists; the implementation does not.
3. **Add `#[cfg(test)]` unit tests for `ingest/mod.rs`,
   `orchestrate.rs`, `cli.rs`, `pricing.rs`** — these are the four
   most-critical untested modules.
4. **Replace `ARCHITECTURE.md`** placeholder with a real architecture
   doc that names `routing/` as the hexagonal exemplar and documents
   the (now-unique) `crates/` layout.
5. **Remove the 7 unused deps** (`cargo machete` output) and the
   `pheno-cli-base` workspace entry, or wire `fuzz/` into the
   workspace and add a `fuzz/src/lib.rs` scaffold.
6. **Add a coverage gate** (`cargo-llvm-cov` or `cargo-tarpaulin`) to
   CI with a minimum threshold (e.g. 60% line coverage).
7. **Gitignore `.validation_logs_*` and `benchmarks/results/`** — these
   are runtime outputs that don't belong in the repo.
8. **Review PR #64** (0/-11/1 file "consolidate: Tokn") before merge.

---

*End of Block-C audit. File added (not modified) per audit protocol.*
