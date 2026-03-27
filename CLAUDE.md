# AGENTS Guide

## Review Bot Governance

- Keep CodeRabbit PR blocking at the lowest level in `.coderabbit.yaml`: `pr_validation.block_on.severity: info`.
- Keep Gemini Code Assist severity at the lowest level in `.gemini/config.yaml`: `code_review.comment_severity_threshold: LOW`.
- Retrigger commands:
  - CodeRabbit: comment `@coderabbitai full review` on the PR.
  - Gemini Code Assist (when enabled in the repo): comment `@gemini-code-assist review` on the PR.
  - If comment-trigger is unavailable, retrigger both bots by pushing a no-op commit to the PR branch.
- Rate-limit discipline:
  - Use a FIFO queue for retriggers (oldest pending PR first).
  - Minimum spacing: one retrigger comment every 120 seconds per repo.
  - On rate-limit response, stop sending new triggers in that repo, wait 15 minutes, then resume queue processing.
  - Do not post duplicate trigger comments while a prior trigger is pending.

## Shared Governance Protocols

These governance blocks are maintained centrally:
- Worktree discipline, reuse protocol, git delivery, stability, CI, child-agent delegation
- Source: `KooshaPari/thegent` -> `templates/claude/governance-blocks/`
- Do not duplicate these blocks here — reference the source instead.

<!-- governance: see thegent/templates/claude/governance-blocks/ for shared protocols -->

## Child Agent Usage
- Use child agents liberally for discovery-heavy, migration-heavy, and high-context work.
- Delegate broad scans, decomposition, and implementation waves to subagents before final parent-agent integration.
- Keep the parent lane focused on deterministic integration and finalization.
- Preserve explicit handoffs and cross-agent context in session notes and audits.


## Worktree Discipline

- Feature work goes in `.worktrees/<topic>/`
- Legacy `PROJECT-wtrees/` and `repo-wtrees/` roots are for migration only and must not receive new work.
- Canonical repository remains on `main` for final integration and verification.
