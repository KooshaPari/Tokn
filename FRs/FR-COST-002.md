# FR-COST-002: Subscription Cost Allocation

**Requirement:** System SHALL allocate provider subscription cost per event proportionally: `subscription_usd_month * (event_tokens / provider_total_tokens_for_month)`.

**Traces To:** E3.3

**Code Location:** `src/cost.rs::allocate_subscription`

**Repository:** Tokn

**Status:** Active

**Test Traces:**
- `tests/integration_test.rs::test_cost_calculation_workflow`
- `tests/integration_test.rs::test_subscription_allocation_multiple_providers`
