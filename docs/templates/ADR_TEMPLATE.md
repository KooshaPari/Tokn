# ADR-{NNN}: {Title}

**Status:** {Draft | Proposed | Accepted | Superseded | Deprecated}
**Date:** {YYYY-MM-DD}
**Deciders:** {Names or roles}
**Supersedes:** {ADR-XXX if applicable, or N/A}

---

## Context

\{Describe the technical, business, or architectural context that necessitates this decision. Include:\}
- {What problem are we solving?}
- {What constraints exist? (technical, timeline, resource, regulatory)}
- {What triggered this decision? (new requirement, technical debt, performance issue)}

## Decision

\{State the decision clearly and concisely. Use active voice. Example: "We will use PostgreSQL as the primary database for the traceability system."\}

## Rationale

\{Explain WHY this decision was made. Include:\}
- {How does this decision solve the problem stated in Context?}
- {What key factors influenced the decision? (performance, cost, maintainability, team expertise)}
- {What trade-offs were considered?}
- {What evidence or data supports this decision? (benchmarks, research, past experience)}

## Alternatives Rejected

### Alternative 1: {Name}
- **Description:** {Brief description}
- **Pros:** {Advantages}
- **Cons:** {Disadvantages}
- **Why Rejected:** {Specific reasons}

### Alternative 2: {Name}
- **Description:** {Brief description}
- **Pros:** {Advantages}
- **Cons:** {Disadvantages}
- **Why Rejected:** {Specific reasons}

\{Add more alternatives as needed\}

## Consequences

### Positive
- {Benefit 1: e.g., "Improved query performance by 40%"}
- {Benefit 2: e.g., "Reduced operational complexity"}
- {Benefit 3: e.g., "Better alignment with team expertise"}

### Negative
- {Trade-off 1: e.g., "Higher initial setup cost"}
- {Trade-off 2: e.g., "Learning curve for team members unfamiliar with X"}
- {Trade-off 3: e.g., "Lock-in to vendor/technology Y"}

### Neutral
- {Neutral consequence 1: e.g., "Requires migration from current system"}
- {Neutral consequence 2: e.g., "Changes monitoring approach"}

## Implementation

### Affected Components
- {Component 1: path/to/component}
- {Component 2: path/to/component}
- {Component 3: path/to/component}

### Migration Strategy
\{If applicable, describe how to migrate from old approach to new. Include:\}
- {Migration steps}
- {Rollback plan}
- {Data migration considerations}
- {Backward compatibility}

### Rollout Plan
- **Phase 1:** {Description and timeline}
- **Phase 2:** {Description and timeline}
- **Phase 3:** {Description and timeline}

### Success Criteria
- [ ] {Measurable criterion 1: e.g., "Query latency &lt; 100ms for 95th percentile"}
- [ ] {Measurable criterion 2: e.g., "Zero data loss during migration"}
- [ ] {Measurable criterion 3: e.g., "All tests passing after implementation"}

## References

- {Link to related ADRs}
- {Link to technical specifications}
- {Link to benchmarks or research}
- {Link to relevant RFCs or standards}
- {Link to implementation PRs or issues}

---

**Notes:**
\{Any additional context, open questions, or follow-up items\}
