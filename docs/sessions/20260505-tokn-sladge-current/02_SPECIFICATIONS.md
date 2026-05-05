# Specifications

## Acceptance Criteria

- Add the Sladge badge to Tokn `README.md`.
- Keep the change README-only plus session governance docs.
- Commit on an isolated feature branch, not canonical `main`.
- Preserve canonical checkout state.

## Risks

- Tokn is locally ahead and behind `origin/main`; remote reconciliation remains
  a separate integration step.
- Broad Rust validation may expose unrelated pre-existing drift outside this
  README/session-doc change.
