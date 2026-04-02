# FR-COST-001: Variable Token Cost Computation

**Requirement:** System SHALL compute variable token cost per event as: `sum_over_token_types(token_count * rate_per_mtok / 1_000_000)`.

**Traces To:** E3.1

**Code Location:** `src/cost.rs::calc_variable_cost`

**Repository:** Tokn

**Status:** Active

**Test Traces:** 
- `tests/integration_test.rs::test_cost_calculation_workflow`
- `tests/integration_test.rs::test_pricing_with_cache_tokens`
