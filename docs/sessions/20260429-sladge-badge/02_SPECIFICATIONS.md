# Specifications

## Acceptance Criteria

- README includes exactly one `sladge.net` badge reference.
- The badge appears in the existing top badge block.
- The canonical dirty checkout remains untouched.
- The commit includes the required Codex co-author trailer.

## Assumptions, Risks, Uncertainties

- Assumption: AI-agent token governance is materially LLM-related.
- Risk: Merging into the dirty canonical checkout could mix unrelated work.
- Mitigation: Commit in an isolated worktree and record merge state separately in
  projects-landing.
