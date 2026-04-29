# Implementation Strategy

## Approach

Use the narrowest docs-only rollout:

- Add the badge to the existing README badge block.
- Record the work in `docs/sessions/20260429-sladge-badge/`.
- Avoid changing runtime crates, docs catalog metadata, or generated indexes.

## Rationale

The badge communicates governance metadata and should not alter product behavior
or documentation generation beyond the visible README marker.
