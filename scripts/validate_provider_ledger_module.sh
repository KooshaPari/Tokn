#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

required_paths=(
  "docs/changes/shared-modules/provider-ledger-schema-v1/proposal.md"
  "docs/changes/shared-modules/provider-ledger-schema-v1/tasks.md"
  "docs/contracts/provider-ledger-schema.contract.json"
  "ledger/provider-ledger-schema-v1/unified_model_provider_ledger.schema.sql"
  "scripts/validate_provider_ledger_module.sh"
)

missing=0
for rel in "${required_paths[@]}"; do
  abs="${repo_root}/${rel}"
  if [[ ! -e "${abs}" ]]; then
    echo "ERROR: required artifact missing: ${rel}" >&2
    missing=1
  fi
done

if [[ "${missing}" -ne 0 ]]; then
  echo "Provider ledger module validation FAILED: missing required artifact(s)." >&2
  exit 1
fi

source_schema="${repo_root}/ledger/unified_model_provider_ledger.schema.sql"
module_schema="${repo_root}/ledger/provider-ledger-schema-v1/unified_model_provider_ledger.schema.sql"

if [[ ! -f "${source_schema}" ]]; then
  echo "ERROR: source schema not found: ledger/unified_model_provider_ledger.schema.sql" >&2
  exit 1
fi

if ! cmp -s "${source_schema}" "${module_schema}"; then
  echo "ERROR: schema drift detected between source and module canonical schema." >&2
  echo "       source: ledger/unified_model_provider_ledger.schema.sql" >&2
  echo "       module: ledger/provider-ledger-schema-v1/unified_model_provider_ledger.schema.sql" >&2
  exit 1
fi

echo "Provider ledger module validation PASSED"
echo "- required artifacts: present"
echo "- schema equivalence: source == module"
