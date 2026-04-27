# US-{NNN}: {Title}

**Epic:** {EPIC-XXX}
**Actor:** {End User | Admin | Developer | System | External Service}
**Priority:** {P0-Critical | P1-High | P2-Medium | P3-Low}
**Status:** {Backlog | In Progress | In Review | Blocked | Done}
**Sprint:** {Sprint-XX or N/A}
**Points:** {1, 2, 3, 5, 8, 13, 21}

---

## Traceability

### Implements Functional Requirements
- {FR-CATEGORY-XXX: Brief description}
- {FR-CATEGORY-YYY: Brief description}
- {FR-CATEGORY-ZZZ: Brief description}

### Blocked by
- {US-XXX: Description of blocking story}
- {TECH-DEBT-YYY: Description of technical blocker}
- {N/A if no blockers}

### Blocks
- {US-AAA: Description of dependent story}
- {N/A if no dependents}

---

## User Story

**As a** {actor/user role},
**I want** {goal/desire},
**so that** {benefit/value}.

{Example: "As a project manager, I want to create traceability links between requirements and test cases, so that I can verify test coverage and identify gaps in verification."}

---

## Acceptance Criteria

{Use testable, unambiguous criteria. Format: Given-When-Then or checkbox list}

### Scenario 1: {Happy Path}
- [ ] **Given** {initial context/precondition}
- [ ] **When** {action/event}
- [ ] **Then** {expected outcome}
- [ ] **And** {additional expected outcome}

### Scenario 2: {Alternative Path}
- [ ] **Given** {different context}
- [ ] **When** {action/event}
- [ ] **Then** {expected outcome}

### Scenario 3: {Error Case}
- [ ] **Given** {error-inducing context}
- [ ] **When** {action that triggers error}
- [ ] **Then** {error handling behavior}
- [ ] **And** {user-visible error message or state}

### Scenario 4: {Edge Case}
- [ ] **Given** {edge condition}
- [ ] **When** {action}
- [ ] **Then** {expected behavior}

### Non-Functional Acceptance Criteria
- [ ] {Performance: e.g., "Page loads in &lt; 2 seconds"}
- [ ] {Security: e.g., "Unauthorized users receive 403 Forbidden"}
- [ ] {Usability: e.g., "Error messages are user-friendly and actionable"}
- [ ] {Accessibility: e.g., "All actions keyboard-navigable"}

---

## Technical Implementation

### API Endpoints
| Method | Endpoint | Description | Status |
|--------|----------|-------------|--------|
| {GET} | {/api/v1/resource} | {Get resource} | {☐ Not Started / ☑ Done} |
| {POST} | {/api/v1/resource} | {Create resource} | {☐ Not Started / ☑ Done} |
| {PUT} | {/api/v1/resource/{id}} | {Update resource} | {☐ Not Started / ☑ Done} |
| {DELETE} | {/api/v1/resource/{id}} | {Delete resource} | {☐ Not Started / ☑ Done} |

### Services
- [ ] {ServiceName.method_name (path/to/service.py:LineRange)}
  - **Purpose:** {What this service does}
  - **Implements:** {FR-XXX}

- [ ] {AnotherService.another_method (path/to/another.py:LineRange)}
  - **Purpose:** {What this service does}
  - **Implements:** {FR-YYY}

### Models / Schemas
- [ ] {ModelName (path/to/models.py:LineRange)}
  - **Fields:** {field1, field2, field3}
  - **Constraints:** {unique, nullable, default values}

- [ ] {SchemaName (path/to/schemas.py:LineRange)}
  - **Validation:** {validation rules}
  - **Serialization:** {how data is serialized}

### Database Migrations
- [ ] {Migration: {description}}
  - **File:** {alembic/versions/{hash}_{description}.py}
  - **Changes:** {tables/columns added/modified}

### Frontend Components
- [ ] {ComponentName (path/to/Component.tsx:LineRange)}
  - **Purpose:** {UI component description}
  - **Props:** {prop1, prop2, prop3}

- [ ] {HookName (path/to/useHook.ts:LineRange)}
  - **Purpose:** {Hook description}
  - **Returns:** {return values}

### State Management
- [ ] {Store updates (path/to/store.ts:LineRange)}
  - **State:** {state shape changes}
  - **Actions:** {action creators}

---

## Test Coverage

### Unit Tests
- [ ] {Test file: path/to/test_service.py}
  - [ ] {test_method_happy_path}
  - [ ] {test_method_validation_error}
  - [ ] {test_method_edge_case}

- [ ] {Test file: path/to/component.test.tsx}
  - [ ] {test_renders_correctly}
  - [ ] {test_handles_user_interaction}
  - [ ] {test_error_state}

### Integration Tests
- [ ] {Test file: path/to/test_api_integration.py}
  - [ ] {test_end_to_end_workflow}
  - [ ] {test_authorization_checks}
  - [ ] {test_database_transactions}

### E2E Tests
- [ ] {Test file: path/to/e2e.spec.ts}
  - [ ] {test_user_flow_scenario_1}
  - [ ] {test_user_flow_scenario_2}
  - [ ] {test_error_recovery}

### Manual Testing Checklist
- [ ] {Manual test 1: e.g., "Verify UI renders correctly on mobile"}
- [ ] {Manual test 2: e.g., "Test with screen reader"}
- [ ] {Manual test 3: e.g., "Verify error messages are user-friendly"}

---

## Progress Tracking

### Development Progress
- [ ] {Task 1: Design API contracts}
- [ ] {Task 2: Implement backend services}
- [ ] {Task 3: Create database migrations}
- [ ] {Task 4: Implement frontend components}
- [ ] {Task 5: Wire up state management}
- [ ] {Task 6: Add error handling}
- [ ] {Task 7: Write unit tests}
- [ ] {Task 8: Write integration tests}
- [ ] {Task 9: Write E2E tests}
- [ ] {Task 10: Update documentation}

### Review Checklist
- [ ] {Code review completed}
- [ ] {Tests passing (unit, integration, E2E)}
- [ ] {Quality checks passing (lint, type-check, format)}
- [ ] {Documentation updated (API docs, user docs)}
- [ ] {Acceptance criteria verified}
- [ ] {Security review (if applicable)}
- [ ] {Performance benchmarks met (if applicable)}

### Deployment Checklist
- [ ] {Feature flag configured (if applicable)}
- [ ] {Database migrations tested}
- [ ] {Monitoring/alerts configured}
- [ ] {Rollback plan documented}
- [ ] {Stakeholders notified}

---

## Notes

### Technical Decisions
{Link to relevant ADRs or document technical decisions made during implementation}
- {ADR-XXX: Decision about X}
- {Decision: Chose Y approach because Z}

### Dependencies
{External dependencies, library changes, or infrastructure requirements}
- {Requires library X version Y}
- {Needs Redis for caching}
- {Depends on external service Z}

### Risks / Concerns
{Identified risks, concerns, or areas needing attention}
- {Risk 1: Performance concern with large datasets}
- {Concern 2: Complex authorization logic may need refactor}
- {Attention 3: UI design pending final mockups}

### Open Questions
{Questions that need answering before or during implementation}
- {Q1: Should we support pagination or infinite scroll?}
- {Q2: What's the retention policy for historical data?}
- {Q3: Do we need real-time updates or is polling sufficient?}

### Follow-up Items
{Items to address in future stories or sprints}
- {Follow-up 1: Add bulk operations (US-XXX)}
- {Follow-up 2: Optimize query performance (TECH-DEBT-YYY)}
- {Follow-up 3: Add advanced filtering (US-ZZZ)}

---

## References

- **Epic:** {Link to EPIC-XXX.md}
- **Functional Requirements:** {Links to FR-XXX.md documents}
- **Design Docs:** {Links to design documents or mockups}
- **API Spec:** {Link to OpenAPI spec section}
- **Pull Requests:** {Links to implementation PRs}
- **Related Stories:** {Links to related user stories}
