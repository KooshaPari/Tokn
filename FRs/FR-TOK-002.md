# FR-TOK-002: Total Token Computation

**Requirement:** System SHALL compute total_tokens as the sum of all six token categories per event.

**Traces To:** E3.1

**Code Location:** `src/models.rs::TokenUsage::total`

**Repository:** Tokn

**Status:** Active

**Test Traces:** `tests/integration_test.rs::test_token_usage_full_workflow`
