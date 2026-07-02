# FR-PRICE-008: Pricing Apply Merge

**Requirement:** `pricing-apply` SHALL merge a patch JSON into the pricing book without overwriting existing rates by default.

**Traces To:** E2.5

**Code Location:** `src/pricing.rs::run_pricing_apply`

**Repository:** Tokn

**Status:** Active

**Test Traces:** `tests/integration_test.rs::test_pricing_apply_summary_accumulation`
