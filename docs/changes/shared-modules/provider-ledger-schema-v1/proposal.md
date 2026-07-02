# Proposal: Provider Ledger Schema Module v1

## Summary
Create a reusable, versioned module for the provider ledger SQL schema so downstream tooling can depend on a stable path and contract.

## Motivation
- The unified schema exists today but is not packaged as an explicit shared module.
- Consumers need a single canonical module path with version ownership and artifact guarantees.

## Scope
- Add rollout docs for this module under `docs/changes/shared-modules/provider-ledger-schema-v1/`.
- Add a machine-readable contract artifact defining ownership, semantic versioning, and required artifacts.
- Introduce module directory `ledger/provider-ledger-schema-v1/` with canonical schema SQL.
- Add a validation script that fails loudly on missing or drifted artifacts.

## Non-Goals
- No schema content redesign in this rollout.
- No migration logic, fallback logic, or compatibility shims.

## Deliverables
- `docs/changes/shared-modules/provider-ledger-schema-v1/proposal.md`
- `docs/changes/shared-modules/provider-ledger-schema-v1/tasks.md`
- `docs/contracts/provider-ledger-schema.contract.json`
- `ledger/provider-ledger-schema-v1/unified_model_provider_ledger.schema.sql`
- `scripts/validate_provider_ledger_module.sh`

## Acceptance Criteria
- Validation script exits `0` only when all required artifacts exist.
- Validation script exits non-zero and prints explicit error(s) when artifacts are missing or schema content diverges.
- Canonical module schema remains equivalent to `ledger/unified_model_provider_ledger.schema.sql`.
