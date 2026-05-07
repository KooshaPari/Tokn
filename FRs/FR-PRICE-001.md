# FR-PRICE-001: Pricing Book Schema

**Requirement:** System SHALL load a pricing book from a JSON file conforming to the `PricingBook` schema.

**Traces To:** E2.1

**Code Location:** `src/models.rs::PricingBook`, `src/utils.rs::load_pricing`

**Repository:** Tokn

**Status:** Active

**Test Traces:** `tests/integration_test.rs::test_pricing_book_with_multiple_providers`
