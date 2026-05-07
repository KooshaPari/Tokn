# FR-ING-002: Canonical UsageEvent Structure

**Requirement:** System SHALL normalize each raw provider log record into a canonical `UsageEvent`.

**Traces To:** E1.1

**Code Location:** `src/models.rs::UsageEvent`, `src/ingest/parser.rs`

**Repository:** Tokn

**Status:** Active

**Test Traces:** `tests/integration_test.rs::test_usage_event_with_all_token_types`
