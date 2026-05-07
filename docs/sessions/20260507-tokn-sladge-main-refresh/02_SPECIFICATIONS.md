# Specifications

## Acceptance Criteria

- The README shows the Sladge badge near the existing badge block.
- The prepared branch is based on current local `main`.
- Validation records whitespace, badge presence, and available Rust gates.
- Canonical Tokn remains untouched.

## ARUs

- Assumption: This is a documentation/governance disclosure only.
- Risk: Integrating directly into `main` would violate repo-local instructions.
- Uncertainty: Existing `pareto-rs` test drift may still block full test
  validation.
