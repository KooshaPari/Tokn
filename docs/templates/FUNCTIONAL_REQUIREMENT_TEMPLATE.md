# FR-{CATEGORY}-{NNN}: {Title}

**Status:** {Draft | Active | Implemented | Verified | Deprecated}
**Version:** {X.Y.Z}
**Date:** {YYYY-MM-DD}
**Last Modified:** {YYYY-MM-DD}

---

## Traceability

### Traces to
- **Epic:** {EPIC-XXX}
- **User Stories:** {US-XXX, US-YYY}
- **Parent FR:** {FR-CATEGORY-XXX if hierarchical, or N/A}
- **Specification Source:** {OpenAPI endpoint, design doc, etc.}

### Implemented in
- **File(s):** {path/to/implementation.py:LineRange}
- **Functions/Classes:** {ClassName.method_name, function_name}
- **API Endpoints:** {GET /api/v1/resource, POST /api/v1/action}

### Tested in
- **Unit Tests:** {path/to/test_file.py::test_function_name}
- **Integration Tests:** {path/to/integration_test.py::test_scenario}
- **E2E Tests:** {path/to/e2e_test.spec.ts::test_name}

---

## Requirement Description

{Use RFC 2119 keywords: SHALL, MUST, SHOULD, MAY, MUST NOT, SHALL NOT}

The system **SHALL** {detailed requirement description}.

{Example: "The system SHALL provide a REST API endpoint that accepts traceability item creation requests with required fields: title, type, and description. The endpoint SHALL validate input according to OpenAPI schema TraceItemCreate and SHALL return HTTP 201 with the created item on success or HTTP 422 with validation errors on failure."}

---

## Input

### Parameters
| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| {param1} | {string} | {Yes} | {min: 1, max: 200} | {Description} |
| {param2} | {integer} | {Yes} | {≥ 0} | {Description} |
| {param3} | {object} | {No} | {See schema} | {Description} |

### Input Example
```json
{
  "param1": "example_value",
  "param2": 42,
  "param3": {
    "nested": "data"
  }
}
```

---

## Output

### Return Values
| Field | Type | Always Present | Description |
|-------|------|----------------|-------------|
| {field1} | {string} | {Yes} | {Description} |
| {field2} | {integer} | {Yes} | {Description} |
| {field3} | {array} | {No} | {Description} |

### Output Example
```json
{
  "field1": "result_value",
  "field2": 123,
  "field3": [
    {"item": "data"}
  ]
}
```

### Status Codes
| Code | Meaning | Condition |
|------|---------|-----------|
| {200} | {Success} | {Normal operation} |
| {400} | {Bad Request} | {Invalid input format} |
| {404} | {Not Found} | {Resource does not exist} |
| {422} | {Validation Error} | {Input fails schema validation} |
| {500} | {Server Error} | {Unexpected internal error} |

---

## Constraints

### Functional Constraints
- {Constraint 1: e.g., "Title MUST be unique within project scope"}
- {Constraint 2: e.g., "Type MUST be one of: requirement, test, story, bug, feature"}
- {Constraint 3: e.g., "User MUST be authenticated to access endpoint"}

### Performance Constraints
- {Performance 1: e.g., "Response time SHALL be &lt; 200ms for 95th percentile"}
- {Performance 2: e.g., "System SHALL handle 1000 concurrent requests"}
- {Performance 3: e.g., "Batch operations SHALL process ≥ 100 items/second"}

### Security Constraints
- {Security 1: e.g., "All requests MUST include valid JWT token"}
- {Security 2: e.g., "Sensitive data MUST NOT appear in logs"}
- {Security 3: e.g., "Rate limiting: 100 requests/minute per user"}

### Data Constraints
- {Data 1: e.g., "Maximum payload size: 10 MB"}
- {Data 2: e.g., "Text fields MUST use UTF-8 encoding"}
- {Data 3: e.g., "Timestamps MUST follow ISO 8601 format"}

---

## Algorithm (if applicable)

{Describe the algorithm or business logic in pseudocode or structured steps}

```
1. Validate input against schema
2. Check user authorization
3. IF creating new item:
     a. Generate unique ID
     b. Set timestamps
     c. Assign default values
4. ELSE IF updating existing item:
     a. Verify item exists
     b. Check user has edit permission
     c. Preserve immutable fields
5. Persist to database within transaction
6. Trigger async events (notifications, indexing)
7. Return response
```

{OR if simple: "See implementation in {path/to/file.py:LineRange}"}

---

## Schema

### Database Schema (if applicable)
```sql
CREATE TABLE {table_name} (
  {column1} {TYPE} {CONSTRAINTS},
  {column2} {TYPE} {CONSTRAINTS},
  PRIMARY KEY ({column1}),
  FOREIGN KEY ({column2}) REFERENCES {other_table}({column})
);
```

### API Schema (OpenAPI/JSON Schema)
```yaml
{OpenAPI schema definition or JSON Schema}
```

{OR reference: "See OpenAPI spec: openapi/{service}.yaml#{SchemaName}"}

---

## Progress Tracking

### Implementation Status
- [ ] {Subtask 1: e.g., "Define Pydantic model"}
- [ ] {Subtask 2: e.g., "Implement service layer logic"}
- [ ] {Subtask 3: e.g., "Create API endpoint"}
- [ ] {Subtask 4: e.g., "Add database migration"}
- [ ] {Subtask 5: e.g., "Update OpenAPI schema"}

### Testing Status
- [ ] {Test 1: e.g., "Unit tests for validation logic"}
- [ ] {Test 2: e.g., "Integration test for happy path"}
- [ ] {Test 3: e.g., "Error case tests (400, 404, 422, 500)"}
- [ ] {Test 4: e.g., "Performance test (response time &lt; 200ms)"}
- [ ] {Test 5: e.g., "Security test (auth, rate limiting)"}

### Documentation Status
- [ ] {Doc 1: e.g., "API documentation in OpenAPI spec"}
- [ ] {Doc 2: e.g., "Code comments and docstrings"}
- [ ] {Doc 3: e.g., "User-facing documentation"}
- [ ] {Doc 4: e.g., "ADR if architectural decision made"}

---

## Notes

{Additional context, open questions, dependencies, or follow-up items}

### Dependencies
- {Dependency 1: e.g., "Requires FR-AUTH-001 (JWT authentication)"}
- {Dependency 2: e.g., "Blocks FR-QUERY-005 (advanced filtering)"}

### Open Questions
- {Question 1: e.g., "Should we support bulk delete?"}
- {Question 2: e.g., "What's the retention policy for deleted items?"}

### References
- {Link to related FRs}
- {Link to user stories}
- {Link to design documents}
- {Link to implementation PRs}
