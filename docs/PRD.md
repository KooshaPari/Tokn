# PRD-TOKEN-001: Token Ledger - Enterprise Token Governance

**Status:** Active
**Version:** 1.0.0
**Date:** 2026-02-24
**Last Modified:** 2026-02-24

---

## Epic Hierarchy

```
PRD-TOKEN-001: Token Ledger - Enterprise Token Governance
├── Theme: AI Cost Management
├── Initiative: Autonomous Cost Optimization
└── Parent Epic: N/A
```

---

## Product Overview

### Business Goal

Build an enterprise-grade token and cost tracking system for AI coding agents that provides unified token/cost tracking across multiple providers with optimization recommendations.

### Success Criteria

| Metric | Target | Timeline |
|--------|--------|----------|
| Cost savings | ≥30% vs unmanaged | v1.0 |
| Provider switching | &lt;100ms latency impact | v1.0 |
| Budget alerts | 100% accuracy | v1.0 |
| Report generation | &lt;5s for 30-day report | v1.0 |

### Target Users

| Persona | Need | Use Case |
|---------|------|----------|
| Dev Team Lead | Track team AI spend | "How much did we spend?" |
| Finance | Budget allocation | "Allocate $X to team Y" |
| Platform Eng | Provider optimization | "Switch to cheaper model" |
| CTO | Cost governance | "Enforce $50/user/month limit" |

---

## Market Requirements

### P0 - Critical

| ID | Requirement | Priority |
|----|-------------|----------|
| PRD-TOKEN-001 | Multi-provider token tracking | P0 |
| PRD-TOKEN-002 | Real-time cost aggregation | P0 |
| PRD-TOKEN-003 | Budget alerts and limits | P0 |
| PRD-TOKEN-004 | Provider failover | P0 |

### P1 - High

| ID | Requirement | Priority |
|----|-------------|----------|
| PRD-TOKEN-005 | Cost optimization recommendations | P1 |
| PRD-TOKEN-006 | Usage analytics dashboards | P1 |
| PRD-TOKEN-007 | Team/project attribution | P1 |
| PRD-TOKEN-008 | Rate limiting enforcement | P1 |

### P2 - Medium

| ID | Requirement | Priority |
|----|-------------|----------|
| PRD-TOKEN-009 | Anomaly detection | P2 |
| PRD-TOKEN-010 | Forecasting | P2 |
| PRD-TOKEN-011 | Cost allocation reports | P2 |

---

## Functional Requirements

### FR-001: Multi-Provider Token Tracking

**Description:** Track token usage across multiple LLM providers.

**Providers:**
- Anthropic (Claude)
- OpenAI (GPT-4)
- Google (Gemini)
- AWS (Bedrock)
- Azure (OpenAI)

**Data Captured:**
- Input tokens
- Output tokens
- Cache tokens (if available)
- Latency
- Model version
- Cost (via provider pricing)

---

### FR-002: Real-Time Cost Aggregation

**Description:** Aggregate costs in real-time with configurable windows.

**Features:**
- Per-request cost calculation
- Per-minute/hour/day aggregation
- Team and project attribution
- Custom cost centers

**Pricing Sources:**
- Provider API (real-time)
- Configurable price lists
- Cached pricing with refresh

---

### FR-003: Budget Alerts and Limits

**Description:** Enforce budget limits with alerts.

**Alert Types:**
- Threshold alerts (50%, 75%, 90%)
- Daily/weekly/monthly limits
- Per-user limits
- Per-team limits
- Anomaly detection

**Actions:**
- Email/Slack notification
- Rate limiting
- Provider switching
- Auto-rollback

---

### FR-004: Provider Failover

**Description:** Automatic failover when provider experiences issues.

**Triggers:**
- High latency (>5s)
- Error rate (>5%)
- 429 Too Many Requests
- API down

**Failover Strategy:**
1. Primary fails → Switch to secondary
2. Log the failure
3. Alert on repeated failures
4. Auto-recover when healthy

---

### FR-005: Cost Optimization Recommendations

**Description:** AI-powered recommendations for cost savings.

**Recommendation Types:**

| Type | Description | Potential Savings |
|------|-------------|-------------------|
| Model downgrade | Use cheaper model for simple tasks | 30-50% |
| Caching | Cache similar requests | 20-40% |
| Batching | Batch requests together | 10-20% |
| Provider switch | Switch to cheaper provider | 20-30% |

**Recommendation Engine:**
```python
def generate_recommendations(usage: UsageData) -> List[Recommendation]:
    recommendations = []
    
    # Check for model downgrade opportunities
    if usage.avg_complexity < 0.3:
        recommendations.append(
            Recommendation(
                type="model_downgrade",
                from_model="claude-3-opus",
                to_model="claude-3-sonnet",
                savings_estimate="40%"
            )
        )
    
    # Check cache efficiency
    if usage.cache_hit_rate < 0.5:
        recommendations.append(
            Recommendation(
                type="enable_caching",
                potential_savings="25%"
            )
        )
    
    return recommendations
```

---

## Technical Architecture

### Components

| Component | Technology | Purpose |
|-----------|------------|---------|
| CLI | Rust/Clap | User interface |
| Provider Abstraction | Rust Traits | Multi-provider |
| Storage | SQLite/PostgreSQL | Data persistence |
| Rate Limiting | governor crate | Token bucket |
| Analytics | Custom | Aggregations |

### Provider Interface

```rust
pub trait LLMProvider: Send + Sync {
    fn name(&self) -> &str;
    
    async fn complete(&self, request: Request) -> Result<Response, ProviderError>;
    
    fn calculate_cost(&self, request: &Request, response: &Response) -> Cost;
    
    fn available_models(&self) -> Vec<Model>;
    
    fn health_check(&self) -> impl Future<Output = bool>;
}
```

---

## Data Model

### Usage Record

```rust
struct UsageRecord {
    id: Uuid,
    timestamp: DateTime<Utc>,
    provider: String,
    model: String,
    input_tokens: u32,
    output_tokens: u32,
    cache_tokens: Option<u32>,
    latency_ms: u32,
    cost_usd: Decimal,
    user_id: Option<Uuid>,
    team_id: Option<Uuid>,
    project_id: Option<Uuid>,
    metadata: HashMap<String, String>,
}
```

### Budget

```rust
struct Budget {
    id: Uuid,
    name: String,
    amount_usd: Decimal,
    period: BudgetPeriod, // Daily, Weekly, Monthly
    scope: BudgetScope,   // Org, Team, User
    alerts: Vec<AlertConfig>,
    enforcement: EnforcementAction,
}
```

---

## API Specification

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/v1/usage | Record usage |
| GET | /api/v1/usage | Query usage |
| GET | /api/v1/costs/aggregate | Cost aggregation |
| POST | /api/v1/budgets | Create budget |
| GET | /api/v1/recommendations | Get recommendations |
| POST | /api/v1/alerts/ack | Acknowledge alert |

### Cost Aggregation

```json
GET /api/v1/costs/aggregate?start=2026-01-01&end=2026-01-31&group_by=team

{
    "period": {
        "start": "2026-01-01",
        "end": "2026-01-31"
    },
    "groups": [
        {
            "team": "engineering",
            "total_cost": 1250.00,
            "total_tokens": 1500000,
            "avg_cost_per_request": 0.025
        }
    ]
}
```

---

## Non-Functional Requirements

### Performance

| Metric | Target |
|--------|--------|
| Latency overhead | &lt;10ms |
| Throughput | 1000 req/s |
| Aggregation query | &lt;5s |
| Startup time | &lt;1s |

### Reliability

| Metric | Target |
|--------|--------|
| Uptime | 99.9% |
| Data retention | 1 year |
| Backup frequency | Hourly |

### Security

- All costs logged immutably
- API key encryption
- Role-based access
- Audit trail

---

## Governance Policies

### Default Policies

```yaml
policies:
  - name: "engineering-budget"
    scope: team:engineering
    limit: $500/month
    alerts: [50%, 75%, 90%]
    enforcement: rate_limit
    
  - name: "research-budget"
    scope: team:research
    limit: $1000/month
    alerts: [50%, 75%, 90%]
    enforcement: notify_only
    
  - name: "model-preference"
    scope: team:*
    prefer:
      - claude-3-sonnet
      - gpt-4o-mini
    fallback: claude-3-haiku
```

---

## Integration Points

### With Cliproxy

```
tokenledger <---> cliproxy++
       │
       ├── Token tracking data
       ├── Cost aggregation
       └── Recommendations
```

### With heliosHarness

```
heliosHarness ---> tokenledger
       │
       └── Cost governance for agent execution
```

---

## Milestones

| Milestone | Deliverable | Target |
|-----------|-------------|--------|
| M1 | Core tracking + providers | 2026-03-01 |
| M2 | Budgets + alerts | 2026-03-15 |
| M3 | Recommendations | 2026-04-01 |
| M4 | Analytics dashboard | 2026-04-15 |
| M5 | Production ready | 2026-05-01 |

---

## Success Metrics

- Cost tracking coverage: 100% of API calls
- Budget alert accuracy: >99%
- Cost savings achieved: >30%
- Provider failover success: >99%

---

## Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-------------|
| Pricing API changes | Medium | High | Configurable price lists |
| High volume processing | Medium | Medium | Async aggregation |
| Provider lock-in | Low | Medium | Abstraction layer |

---

## References

- Architecture: Native Rust design
- SPEC: docs/SPEC.md
- Research: docs/research/
