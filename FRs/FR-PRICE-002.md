# FR-PRICE-002: ModelRate Fields

**Requirement:** Each `ModelRate` SHALL contain: input_usd_per_mtok, output_usd_per_mtok, and optional cache/tool fields.

**Traces To:** E2.1

**Code Location:** `src/models.rs::ModelRate`

**Repository:** Tokn

**Status:** Active

**Test Traces:** `tests/integration_test.rs::test_model_rate_with_optional_fields`
