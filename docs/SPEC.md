# SPEC-TOKEN-001: Token Ledger Technical Specification

**Status:** Active
**Version:** 1.0.0
**Date:** 2026-02-24
**Last Modified:** 2026-02-24

---

## 1. System Architecture

### 1.1 High-Level Design

```
┌─────────────────────────────────────────────────────────────────┐
│                        CLI / API Client                          │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Command Layer (Clap)                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐│
│  │ report      │  │ costs       │  │ optimize               ││
│  │ command     │  │ command     │  │ command                ││
│  └─────────────┘  └─────────────┘  └─────────────────────────┘│
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Service Layer                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐│
│  │ UsageService│  │ CostService │  │ BudgetService          ││
│  └─────────────┘  └─────────────┘  └─────────────────────────┘│
└──────────────────────────┬──────────────────────────────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
┌─────────────────┐ ┌─────────────┐ ┌─────────────────┐
│ Provider        │ │ Storage     │ │ Rate Limiter    │
│ Registry        │ │ (SQLite/PG) │ │ (governor)     │
└─────────────────┘ └─────────────┘ └─────────────────┘
```

---

## 2. Technology Stack

### 2.1 Core

| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 1.75+ | Core language |
| tokio | 1.x | Async runtime |
| clap | 4.x | CLI parsing |
| serde | 1.x | Serialization |
| thiserror | 1.x | Error handling |

### 2.2 Database

| Technology | Version | Purpose |
|------------|---------|---------|
| SQLx | 0.7 | Database access |
| SQLite | 3.x | Local storage |
| PostgreSQL | 15+ | Production |
| rusqlite | 0.31 | SQLite wrapper |

### 2.3 Rate Limiting

| Technology | Purpose |
|------------|---------|
| governor | Token bucket rate limiting |

---

## 3. Provider Abstraction

### 3.1 Trait Definition

```rust
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Provider name (e.g., "anthropic", "openai")
    fn name(&self) -> &str;
    
    /// Available models from this provider
    fn available_models(&self) -> Vec<Model>;
    
    /// Execute a completion request
    async fn complete(&self, request: Request) -> Result<Response, ProviderError>;
    
    /// Calculate cost for a request/response pair
    fn calculate_cost(&self, request: &Request, response: &Response) -> Cost;
    
    /// Health check for provider
    async fn health_check(&self) -> bool;
    
    /// Get current pricing (may be cached)
    fn get_pricing(&self) -> Pricing;
}
```

### 3.2 Provider Implementations

```rust
// anthropic.rs
pub struct AnthropicProvider {
    client: reqwest::Client,
    api_key: String,
    pricing: Pricing,
}

impl LLMProvider for AnthropicProvider {
    fn name(&self) -> &str { "anthropic" }
    
    async fn complete(&self, request: Request) -> Result<Response, ProviderError> {
        // Implementation
    }
}

// openai.rs  
pub struct OpenAIProvider {
    client: reqwest::Client,
    api_key: String,
    organization: Option<String>,
}
```

### 3.3 Provider Registry

```rust
pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn LLMProvider>>,
    default_provider: String,
}

impl ProviderRegistry {
    pub fn register(&mut self, provider: Box<dyn LLMProvider>) {
        self.providers.insert(provider.name().to_string(), provider);
    }
    
    pub fn get(&self, name: &str) -> Option<&dyn LLMProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }
    
    pub fn get_default(&self) -> &dyn LLMProvider {
        self.get(&self.default_provider).unwrap()
    }
}
```

---

## 4. Data Models

### 4.1 Usage Record

```rust
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub provider: String,
    pub model: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_creation_tokens: Option<u32>,
    pub cache_read_tokens: Option<u32>,
    pub latency_ms: u32,
    pub cost_usd: Decimal,
    pub user_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub request_id: Uuid,
    pub metadata: HashMap<String, String>,
}
```

### 4.2 Cost Aggregation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAggregation {
    pub period: DateRange,
    pub group_by: GroupBy,
    pub total_cost: Decimal,
    pub total_tokens: u64,
    pub request_count: u64,
    pub avg_cost_per_request: Decimal,
    pub avg_latency_ms: f64,
    pub breakdown: Vec<CostBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub provider: String,
    pub model: String,
    pub cost: Decimal,
    pub tokens: u64,
    pub percentage: f64,
}
```

### 4.3 Budget

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub amount_usd: Decimal,
    pub period: BudgetPeriod,
    pub scope: BudgetScope,
    pub current_spent: Decimal,
    pub alerts: Vec<AlertConfig>,
    pub enforcement: EnforcementAction,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BudgetPeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementAction {
    NotifyOnly,
    RateLimit,
    BlockRequests,
    SwitchProvider(String),
}
```

---

## 5. CLI Commands

### 5.1 Report Command

```bash
# Generate usage report
tokenledger report --start 2026-01-01 --end 2026-01-31

# Group by team
tokenledger report --group-by team

# Group by user
tokenledger report --group-by user

# Export to CSV
tokenledger report --format csv --output report.csv
```

### 5.2 Costs Command

```bash
# Show current month costs
tokenledger costs

# Show costs for specific team
tokenledger costs --team engineering

# Show daily breakdown
tokenledger costs --breakdown daily
```

### 5.3 Optimize Command

```bash
# Get optimization recommendations
tokenledger optimize

# Apply recommended provider switch
tokenledger optimize --apply provider-switch

# Show potential savings
tokenledger optimize --simulate
```

---

## 6. Rate Limiting

### 6.1 Token Bucket Implementation

```rust
use governor::{GovernorBuilder, clock::DefaultClock, state::keyed::DefaultKeyedStateStore};
use std::num::NonZeroU32;

pub struct RateLimiter {
    limiter: GovernorDefault,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        let limiter = GovernorBuilder::new()
            .keyed(DefaultKeyedStateStore::default())
            .period(Duration::from_secs(60))
            .burst_size(NonZeroU32::new(requests_per_minute).unwrap())
            .build()
            .unwrap();
            
        Self { limiter }
    }
    
    pub fn check(&self, key: &str) -> Result<(), RateLimitExceeded> {
        self.limiter.check_key(key).map_err(|_| RateLimitExceeded)
    }
}
```

### 6.2 Budget Enforcement

```rust
impl Budget {
    pub fn check_limit(&self, current_spent: Decimal) -> EnforcementAction {
        let percentage = (current_spent / self.amount_usd) * Decimal::from(100);
        
        for alert in &self.alerts {
            if percentage >= alert.threshold {
                return self.enforcement.clone();
            }
        }
        
        EnforcementAction::NotifyOnly
    }
}
```

---

## 7. Database Schema

### 7.1 SQLite Schema

```sql
CREATE TABLE usage_records (
    id TEXT PRIMARY KEY,
    timestamp TEXT NOT NULL,
    provider TEXT NOT NULL,
    model TEXT NOT NULL,
    input_tokens INTEGER NOT NULL,
    output_tokens INTEGER NOT NULL,
    cache_creation_tokens INTEGER,
    cache_read_tokens INTEGER,
    latency_ms INTEGER NOT NULL,
    cost_usd REAL NOT NULL,
    user_id TEXT,
    team_id TEXT,
    project_id TEXT,
    request_id TEXT NOT NULL,
    metadata TEXT
);

CREATE INDEX idx_usage_timestamp ON usage_records(timestamp);
CREATE INDEX idx_usage_provider ON usage_records(provider);
CREATE INDEX idx_usage_team ON usage_records(team_id);

CREATE TABLE budgets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    amount_usd REAL NOT NULL,
    period TEXT NOT NULL,
    scope TEXT NOT NULL,
    current_spent REAL NOT NULL DEFAULT 0,
    enforcement TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE alerts (
    id TEXT PRIMARY KEY,
    budget_id TEXT NOT NULL,
    threshold REAL NOT NULL,
    triggered_at TEXT,
    acknowledged BOOLEAN DEFAULT FALSE
);
```

---

## 8. Pricing Configuration

### 8.1 Default Pricing

```yaml
# config/pricing.yaml
providers:
  anthropic:
    claude-3-opus:
      input: 0.015   # per 1K tokens
      output: 0.075
      cache_creation: 0.01875
      cache_read: 0.0
    claude-3-sonnet:
      input: 0.003
      output: 0.015
    claude-3-haiku:
      input: 0.00025
      output: 0.00125
      
  openai:
    gpt-4o:
      input: 0.005
      output: 0.015
    gpt-4o-mini:
      input: 0.00015
      output: 0.0006
```

### 8.2 Pricing Lookup

```rust
impl Pricing {
    pub fn calculate(&self, provider: &str, model: &str, tokens: &TokenCount) -> Decimal {
        let model_pricing = self.providers
            .get(provider)
            .and_then(|p| p.get(model));
            
        // Calculate cost
    }
}
```

---

## 9. Optimization Engine

### 9.1 Recommendation Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recommendation {
    ModelDowngrade {
        from: String,
        to: String,
        savings_percent: f64,
    },
    EnableCaching {
        potential_savings: f64,
    },
    ProviderSwitch {
        from: String,
        to: String,
        savings_percent: f64,
    },
    BatchRequests {
        potential_savings: f64,
    },
}
```

### 9.2 Analysis Logic

```rust
pub fn analyze_optimization(usage: &[UsageRecord]) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();
    
    // 1. Check for model downgrade opportunities
    let simple_requests = usage.iter()
        .filter(|u| u.complexity_score() < 0.3)
        .count();
        
    if simple_requests > usage.len() / 2 {
        recommendations.push(Recommendation::ModelDowngrade {
            from: "claude-3-opus".to_string(),
            to: "claude-3-haiku".to_string(),
            savings_percent: 40.0,
        });
    }
    
    // 2. Check cache efficiency
    let cache_hit_rate = usage.iter()
        .filter(|u| u.cache_read_tokens.is_some())
        .count() as f64 / usage.len() as f64;
        
    if cache_hit_rate < 0.3 {
        recommendations.push(Recommendation::EnableCaching {
            potential_savings: 25.0,
        });
    }
    
    recommendations
}
```

---

## 10. Error Handling

### 10.1 Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenLedgerError {
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    
    #[error("Budget exceeded: {0}")]
    BudgetExceeded(String),
    
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}
```

---

## 11. Configuration

### 11.1 Config File

```yaml
# config/tokenledger.yaml
database:
  url: "sqlite:///data/tokenledger.db"
  # For production:
  # url: "postgresql://user:pass@localhost/tokenledger"

providers:
  anthropic:
    api_key: "${ANTHROPIC_API_KEY}"
    default_model: "claude-3-sonnet"
  openai:
    api_key: "${OPENAI_API_KEY}"
    organization: "${OPENAI_ORG}"

rate_limits:
  default: 1000  # requests per minute
  by_team:
    engineering: 2000
    research: 3000

budgets:
  - name: "engineering-monthly"
    team: "engineering"
    amount: 500
    period: monthly
    alerts: [50, 75, 90]
    enforcement: rate_limit
```

---

## References

- PRD: docs/PRD.md
- Source: src/
- Tests: tests/
