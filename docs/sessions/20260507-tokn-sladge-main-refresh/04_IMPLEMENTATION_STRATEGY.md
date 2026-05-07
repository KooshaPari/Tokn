# Implementation Strategy

Use a fresh worktree from current local `main`:

- Preserve older `docs/tokn-sladge-current` as stale evidence.
- Reapply the README Sladge badge near the existing badge block.
- Keep canonical Tokn unchanged because direct `main` commits are prohibited.
- Record exact validation blockers for any pre-existing Rust test drift.
