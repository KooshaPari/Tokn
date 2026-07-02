# ADR-003: Storage Backend Selection

**Status:** Proposed  
**Date:** 2026-04-02  
**Author:** Tokn Architecture Team  
**Line Count Target:** 400+ lines  

---

## Context

Tokn requires persistent storage for:

1. **Token metadata** - JTI, subject, expiration, revocation status
2. **Audit logs** - Token lifecycle events for compliance
3. **Key material** - Key IDs, rotation metadata (not private keys)
4. **Configuration** - Feature flags, rate limits, policies
5. **Cache** - Validation results, JWKS cache

The storage architecture must support:
- High read throughput (token validation is read-heavy)
- Moderate write throughput (token issuance, revocation)
- Strong consistency for security-critical operations
- Horizontal scalability
- Geographic distribution (multi-region)

---

## Decision

Tokn will adopt a **hybrid storage architecture**:

1. **Primary storage**: PostgreSQL (source of truth)
2. **Caching layer**: Redis Cluster (performance)
3. **Write-through pattern**: All writes go to PostgreSQL, cache invalidated
4. **Read-through pattern**: Reads hit cache first, fallback to PostgreSQL

### Rationale

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Storage Architecture Rationale                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ The hybrid approach combines the strengths of relational and cache            │
│ storage while mitigating their weaknesses.                                    │
│                                                                              │
│ ┌─────────────────────────────────────────────────────────────────┐         │
│ │ Why PostgreSQL?                                                  │         │
│ ├─────────────────────────────────────────────────────────────────┤         │
│ │                                                                  │         │
│ │ • ACID compliance for critical token operations                  │         │
│ │ • Complex queries for audit and analytics                      │         │
│ │ • Mature ecosystem, excellent Rust support (sqlx, tokio-postgres)│         │
│ │ • JSONB support for flexible metadata                          │         │
│ │ • Row-level security for multi-tenancy                           │         │
│ │ • Excellent replication for high availability                    │         │
│ │                                                                  │         │
│ └─────────────────────────────────────────────────────────────────┘         │
│                                                                              │
│ ┌─────────────────────────────────────────────────────────────────┐         │
│ │ Why Redis?                                                       │         │
│ ├─────────────────────────────────────────────────────────────────┤         │
│ │                                                                  │         │
│ │ • Sub-millisecond read latency for hot paths                    │         │
│ │ • TTL support for automatic expiration                          │         │
│ │ • Pub/sub for revocation broadcasting                            │         │
│ │ • Sorted sets for rate limiting windows                          │         │
│ │ • HyperLogLog for cardinality (unique token counts)              │         │
│ │ • Cluster mode for horizontal scaling                            │         │
│ │                                                                  │         │
│ └─────────────────────────────────────────────────────────────────┘         │
│                                                                              │
│ ┌─────────────────────────────────────────────────────────────────┐         │
│ │ Why not pure Redis?                                              │         │
│ ├─────────────────────────────────────────────────────────────────┤         │
│ │                                                                  │         │
│ │ • Persistence guarantees (AOF can lose data on crash)           │         │
│ │ • Complex transactions (Lua scripts are limited)                │         │
│ │ • Audit requirements (need durable log of all changes)         │         │
│ │ • Compliance (some regulations require SQL audit trails)       │         │
│ │                                                                  │         │
│ └─────────────────────────────────────────────────────────────────┘         │
│                                                                              │
│ Why not pure PostgreSQL?                                                     │
│ │                                                                  │         │
│ │ • Validation latency (p99 target < 10ms, DB is 5-20ms)         │         │
│ │ • Revocation broadcast (need pub/sub for distributed invalidation)│         │
│ │ • Rate limiting (Redis is purpose-built for this)                │         │
│ │                                                                  │         │
│ └─────────────────────────────────────────────────────────────────┘         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Alternatives Considered

### Option 1: Pure PostgreSQL

**Description:** All token data stored in PostgreSQL with connection pooling.

**Pros:**
- Single source of truth, no consistency concerns
- ACID transactions for all operations
- Rich query capabilities
- Excellent audit trail

**Cons:**
- Higher read latency (network + query time)
- Connection limits for high throughput
- Scaling challenges for global distribution
- Revocation broadcast requires polling or triggers

**Performance Projection:**
```
┌────────────────────────────────────────────────────────────┐
│ Pure PostgreSQL Performance (Estimated)                  │
├────────────────────────────────────────────────────────────┤
│                                                            │
│ Read latency (p50/p99):     5ms / 20ms                   │
│ Write latency (p50/p99):    10ms / 50ms                  │
│ Throughput (single node):   ~10K ops/sec                 │
│ Throughput (with replicas): ~30K ops/sec                 │
│                                                            │
│ Revocation delay:           0ms (synchronous)            │
│ Revocation broadcast:       Requires polling/notify      │
│                                                            │
│ Verdict: Adequate but not ideal for high-throughput       │
└────────────────────────────────────────────────────────────┘
```

### Option 2: Pure Redis

**Description:** All token data stored in Redis with AOF persistence.

**Pros:**
- Sub-millisecond latency
- High throughput (100K+ ops/sec)
- Built-in TTL for automatic expiration
- Pub/sub for instant revocation broadcast

**Cons:**
- Eventual persistence (AOF fsync intervals)
- Limited query capabilities
- Data loss risk on unclean shutdown
- No complex transactions
- Audit trail requires separate logging

**Risk Assessment:**
```
┌────────────────────────────────────────────────────────────┐
│ Pure Redis Risk Analysis                                   │
├────────────────────────────────────────────────────────────┤
│                                                            │
│ Data loss scenarios:                                       │
│ • Unclean shutdown before AOF fsync: HIGH risk             │
│ • Split brain in cluster mode: MEDIUM risk                 │
│ • Memory pressure eviction: Configurable                   │
│                                                            │
│ Mitigation with AOF everysec:                              │
│ • Maximum data loss: 1 second of writes                    │
│ • For token storage: Acceptable (tokens are ephemeral)   │
│ • For audit: NOT acceptable (need durability)              │
│                                                            │
│ Verdict: Insufficient for audit requirements               │
└────────────────────────────────────────────────────────────┘
```

### Option 3: Cassandra

**Description:** Apache Cassandra for distributed storage.

**Pros:**
- Excellent horizontal scalability
- Tunable consistency
- Multi-datacenter support
- High write throughput

**Cons:**
- Operational complexity
- Read-before-write for updates
- Consistency tuning required
- Smaller Rust ecosystem

**Verdict:** Rejected - operational overhead not justified for current scale.

### Option 4: DynamoDB / Cloud-Native

**Description:** AWS DynamoDB or equivalent cloud-native storage.

**Pros:**
- Fully managed, no operational burden
- Automatic scaling
- Global tables for multi-region
- TTL support

**Cons:**
- Vendor lock-in
- Cost at scale
- Limited query flexibility
- Latency to external service

**Verdict:** Rejected - Tokn aims for deployment flexibility (on-prem, edge).

---

## Hybrid Architecture Detail

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Tokn Hybrid Storage Architecture                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                     ┌──────────────────┐                                    │
│                     │   API Request    │                                    │
│                     └────────┬─────────┘                                    │
│                                │                                            │
│                     ┌──────────▼──────────┐                                  │
│                     │   Token Service     │                                  │
│                     └──────────┬──────────┘                                  │
│                                │                                            │
│              ┌──────────────────┼──────────────────┐                        │
│              │                  │                  │                        │
│              ▼                  ▼                  ▼                        │
│    ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐            │
│    │   Redis Cache   │  │   PostgreSQL    │  │   Redis Pub/Sub │            │
│    │                 │  │   (Primary)     │  │                 │            │
│    │ ┌─────────────┐ │  │                 │  │                 │            │
│    │ │ Token Data  │ │  │ ┌─────────────┐ │  │ ┌─────────────┐ │            │
│    │ │ (hot)       │ │  │ │ Token Table │ │  │ │ Revocation  │ │            │
│    │ │ TTL = exp   │ │  │ │ (all data)  │ │  │ │ Channel     │ │            │
│    │ └─────────────┘ │  │ └─────────────┘ │  │ └─────────────┘ │            │
│    │ ┌─────────────┐ │  │ ┌─────────────┐ │  │                 │            │
│    │ │ Revocation  │ │  │ │ Audit Log   │ │  │                 │            │
│    │ │ (set)       │ │  │ │ (immutable) │ │  │                 │            │
│    │ └─────────────┘ │  │ └─────────────┘ │  │                 │            │
│    └─────────────────┘  └─────────────────┘  └─────────────────┘            │
│           │                     │                     │                     │
│           │                     │                     │                     │
│           ▼                     ▼                     ▼                     │
│    ┌─────────────────────────────────────────────────────────┐            │
│    │                   Write Path                              │            │
│    │  1. Write to PostgreSQL (transaction)                     │            │
│    │  2. On success, invalidate Redis cache                    │            │
│    │  3. Publish revocation event on pub/sub                    │            │
│    └─────────────────────────────────────────────────────────┘            │
│                                                                              │
│    ┌─────────────────────────────────────────────────────────┐            │
│    │                   Read Path                                 │            │
│    │  1. Check Redis (sub-millisecond)                         │            │
│    │  2. If miss, query PostgreSQL                             │            │
│    │  3. Populate Redis cache on hit                           │            │
│    └─────────────────────────────────────────────────────────┘            │
│                                                                              │
│    ┌─────────────────────────────────────────────────────────┐            │
│    │                   Consistency Model                       │            │
│    │  • PostgreSQL: Strong consistency (SERIALIZABLE)          │            │
│    │  • Redis: Eventual consistency (cache)                    │            │
│    │  • Cross-system: Write-through with invalidation          │            │
│    └─────────────────────────────────────────────────────────┘            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Caching Strategy

### Cache-Aside vs Write-Through

```rust
/// Hybrid storage implementation
pub struct HybridStorage {
    postgres: Arc<PostgresStorage>,
    redis: Arc<RedisStorage>,
    cache_config: CacheConfig,
}

#[derive(Clone)]
pub struct CacheConfig {
    /// TTL for token data in cache
    pub token_ttl: Duration,
    /// TTL for revocation status (short - frequent invalidation)
    pub revocation_ttl: Duration,
    /// Enable cache warming
    pub warm_cache: bool,
    /// Write-through vs cache-aside
    pub write_strategy: WriteStrategy,
}

#[derive(Clone)]
pub enum WriteStrategy {
    /// Write to DB, invalidate cache
    WriteThroughInvalidate,
    /// Write to DB and cache (eager)
    WriteThroughPopulate,
    /// Write to DB only, cache on read
    CacheAside,
}

impl HybridStorage {
    /// Store token (write path)
    pub async fn store_token(&self, token: &Token) -> Result<(), StorageError> {
        match self.cache_config.write_strategy {
            WriteStrategy::WriteThroughInvalidate => {
                // 1. Write to PostgreSQL
                self.postgres.store_token(token).await?;
                
                // 2. Invalidate cache (delete, don't update)
                self.redis.delete(&token.jti).await?;
                
                // 3. Invalidate any related cached queries
                self.invalidate_subject_cache(&token.subject).await?;
            }
            WriteStrategy::WriteThroughPopulate => {
                // 1. Write to PostgreSQL
                self.postgres.store_token(token).await?;
                
                // 2. Populate cache
                let ttl = self.calculate_ttl(token);
                self.redis.store_token(token, ttl).await?;
            }
            WriteStrategy::CacheAside => {
                // Only write to PostgreSQL
                self.postgres.store_token(token).await?;
            }
        }
        
        Ok(())
    }
    
    /// Retrieve token (read path with cache-aside)
    pub async fn get_token(&self, jti: &str) -> Result<Option<Token>, StorageError> {
        // 1. Try cache first
        if let Some(cached) = self.redis.get_token(jti).await? {
            tracing::debug!("Token {} cache hit", jti);
            return Ok(Some(cached));
        }
        
        // 2. Cache miss - query PostgreSQL
        tracing::debug!("Token {} cache miss", jti);
        let token = self.postgres.get_token(jti).await?;
        
        // 3. Populate cache if found
        if let Some(ref t) = token {
            let ttl = self.calculate_ttl(t);
            self.redis.store_token(t, ttl).await?;
        }
        
        Ok(token)
    }
    
    /// Revoke token (with immediate broadcast)
    pub async fn revoke_token(
        &self,
        jti: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), StorageError> {
        // 1. Transaction: mark revoked in PostgreSQL
        self.postgres.revoke_token(jti).await?;
        
        // 2. Add to revocation set in Redis (TTL = time until token expires)
        let ttl = (expires_at - Utc::now()).to_std()?;
        self.redis.add_to_revocation_set(jti, ttl).await?;
        
        // 3. Publish revocation event for distributed consumers
        self.redis.publish_revocation(jti).await?;
        
        // 4. Invalidate cache
        self.redis.delete(jti).await?;
        
        Ok(())
    }
    
    fn calculate_ttl(&self, token: &Token) -> Duration {
        let remaining = token.expires_at - Utc::now();
        
        // Cap at configured maximum
        remaining.to_std().unwrap_or(self.cache_config.token_ttl)
    }
    
    async fn invalidate_subject_cache(&self, subject: &str) -> Result<(), StorageError> {
        // Pattern-based invalidation for subject queries
        self.redis.invalidate_pattern(&format!("subject:{}:*", subject)).await?;
        Ok(())
    }
}
```

### Redis Data Structures

```rust
/// Redis data structure design
pub mod redis_structures {
    /// Token data (hash)
    /// Key: token:{jti}
    /// TTL: time until expiration
    pub struct TokenHash {
        pub jti: String,
        pub subject: String,
        pub audience: String, // JSON array
        pub expires_at: String, // Unix timestamp
        pub claims: String, // JSON
    }
    
    /// Revocation set (set for O(1) lookup)
    /// Key: revoked:tokens
    /// TTL: sliding window (max token lifetime)
    pub struct RevocationSet;
    
    /// Subject token index (sorted set by expiration)
    /// Key: subject:{subject}:tokens
    /// Score: expiration timestamp
    /// Member: jti
    pub struct SubjectTokenIndex;
    
    /// Rate limiting (sliding window)
    /// Key: ratelimit:{subject}:{window}
    /// Value: counter
    pub struct RateLimitCounter;
    
    /// JWKS cache (string)
    /// Key: jwks:{kid}
    /// TTL: JWKS refresh interval
    pub struct JwksCache;
}
```

---

## PostgreSQL Schema

```sql
-- Token table (primary storage)
CREATE TABLE tokens (
    jti UUID PRIMARY KEY,
    subject VARCHAR(255) NOT NULL,
    audience TEXT[],
    scopes TEXT[],
    issued_at TIMESTAMPTZ NOT NULL,
    not_before TIMESTAMPTZ,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    revocation_reason VARCHAR(255),
    claims JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for common queries
CREATE INDEX idx_tokens_subject ON tokens(subject) WHERE revoked_at IS NULL;
CREATE INDEX idx_tokens_expires ON tokens(expires_at);
CREATE INDEX idx_tokens_revoked ON tokens(revoked_at) WHERE revoked_at IS NOT NULL;

-- Partial index for active tokens (most queries)
CREATE INDEX idx_tokens_active ON tokens(jti) 
    WHERE revoked_at IS NULL AND expires_at > NOW();

-- Audit log (immutable, partitioned by time)
CREATE TABLE audit_log (
    id BIGSERIAL,
    event_type VARCHAR(50) NOT NULL,
    jti UUID,
    subject VARCHAR(255),
    details JSONB,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_ip INET,
    user_agent TEXT
) PARTITION BY RANGE (timestamp);

-- Monthly partitions
CREATE TABLE audit_log_2026_04 PARTITION OF audit_log
    FOR VALUES FROM ('2026-04-01') TO ('2026-05-01');

-- Retention: drop old partitions

-- Rate limit tracking (sliding window)
CREATE TABLE rate_limits (
    subject VARCHAR(255) NOT NULL,
    window_start TIMESTAMPTZ NOT NULL,
    request_count INT NOT NULL DEFAULT 0,
    PRIMARY KEY (subject, window_start)
);

-- Auto-cleanup old rate limit windows
SELECT cron.schedule('0 * * * *', $$
    DELETE FROM rate_limits WHERE window_start < NOW() - INTERVAL '1 hour';
$$);
```

---

## Consistency and Failure Handling

```rust
/// Consistency handling for hybrid storage
pub struct ConsistencyManager {
    storage: Arc<HybridStorage>,
    retry_policy: RetryPolicy,
}

#[derive(Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl ConsistencyManager {
    /// Write with eventual consistency guarantee
    pub async fn write_with_retry<F, T>(
        &self,
        operation: F,
    ) -> Result<T, StorageError>
    where
        F: Fn() -> futures::future::BoxFuture<'_, Result<T, StorageError>>,
    {
        let mut delay = self.retry_policy.base_delay;
        
        for attempt in 0..self.retry_policy.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < self.retry_policy.max_retries - 1 => {
                    tracing::warn!(
                        "Write attempt {} failed: {:?}, retrying in {:?}",
                        attempt + 1,
                        e,
                        delay
                    );
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(delay * 2, self.retry_policy.max_delay);
                }
                Err(e) => return Err(e),
            }
        }
        
        unreachable!()
    }
    
    /// Handle cache-DB inconsistency
    pub async fn reconcile_cache(&self, jti: &str) -> Result<(), StorageError> {
        // Source of truth is PostgreSQL
        let db_token = self.storage.postgres.get_token(jti).await?;
        
        match db_token {
            Some(token) => {
                // Update cache to match DB
                self.storage.redis.store_token(&token, Duration::from_secs(60)).await?;
            }
            None => {
                // Token doesn't exist in DB, remove from cache
                self.storage.redis.delete(jti).await?;
            }
        }
        
        Ok(())
    }
    
    /// Circuit breaker for cache failures
    pub async fn read_with_circuit_breaker(
        &self,
        jti: &str,
    ) -> Result<Option<Token>, StorageError> {
        // If cache is failing, bypass and go directly to DB
        if self.is_cache_healthy().await {
            self.storage.get_token(jti).await
        } else {
            tracing::warn!("Cache unhealthy, reading from database");
            self.storage.postgres.get_token(jti).await
        }
    }
    
    async fn is_cache_healthy(&self) -> bool {
        // Check recent error rate
        // Implementation: track errors in rolling window
        true
    }
}
```

---

## Scalability Path

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Scaling Strategy                                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Phase 1: Single Node (Current)                                               │
│ ─────────────────────────────────────                                         │
│ • PostgreSQL primary with read replica                                       │
│ • Redis single node (persistence enabled)                                     │
│ • Suitable for: < 10K tokens/sec, < 1M active tokens                          │
│                                                                              │
│ Phase 2: Vertical Scaling (Growth)                                            │
│ ─────────────────────────────────────                                         │
│ • Larger PostgreSQL instance                                                  │
│ • Connection pooling (PgBouncer)                                             │
│ • Redis with persistence (AOF everysec)                                      │
│ • Suitable for: < 50K tokens/sec, < 10M active tokens                           │
│                                                                              │
│ Phase 3: Horizontal Scaling (High Volume)                                     │
│ ─────────────────────────────────────                                         │
│ • PostgreSQL read replicas for validation queries                            │
│ • Redis Cluster (sharded)                                                     │
│ • Regional deployment with geo-distribution                                   │
│ • Suitable for: > 50K tokens/sec, > 10M active tokens                         │
│                                                                              │
│ Phase 4: Global Distribution (Enterprise)                                     │
│ ─────────────────────────────────────                                         │
│ • Multi-region PostgreSQL (Citus or cloud-native)                            │
│ • Global Redis (Redis Enterprise or Dragonfly)                                │
│ • Consistent hashing for data placement                                       │
│ • Conflict-free replicated data types (CRDTs) for rate limiting             │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Monitoring and Alerting

```rust
/// Storage health monitoring
pub struct StorageMonitor {
    metrics: Arc<MetricsRegistry>,
}

impl StorageMonitor {
    pub async fn check_health(&self) -> StorageHealth {
        let postgres_health = self.check_postgres().await;
        let redis_health = self.check_redis().await;
        
        StorageHealth {
            postgres: postgres_health,
            redis: redis_health,
            overall: if postgres_health.healthy && redis_health.healthy {
                HealthStatus::Healthy
            } else if postgres_health.healthy {
                HealthStatus::Degraded // Cache down but functional
            } else {
                HealthStatus::Unhealthy
            },
        }
    }
    
    async fn check_postgres(&self) -> ComponentHealth {
        let start = Instant::now();
        let result = sqlx::query("SELECT 1").fetch_one(&self.pg_pool).await;
        let latency = start.elapsed();
        
        ComponentHealth {
            healthy: result.is_ok(),
            latency_ms: latency.as_millis() as u64,
            error_rate: self.metrics.get_error_rate("postgres"),
        }
    }
    
    async fn check_redis(&self) -> ComponentHealth {
        let start = Instant::now();
        let result = self.redis.ping().await;
        let latency = start.elapsed();
        
        ComponentHealth {
            healthy: result.is_ok(),
            latency_ms: latency.as_millis() as u64,
            error_rate: self.metrics.get_error_rate("redis"),
        }
    }
}

/// Metrics to track
/// - postgres_query_latency (histogram)
/// - redis_operation_latency (histogram)
/// - cache_hit_ratio (gauge)
/// - cache_invalidation_latency (histogram)
/// - storage_reconciliation_count (counter)
```

---

## Consequences

### Positive

1. **Best of both worlds**: ACID durability + cache performance
2. **Gradual degradation**: System works (slower) if cache fails
3. **Audit compliance**: PostgreSQL provides durable audit trail
4. **Operational flexibility**: Can tune cache/DB ratio as needed

### Negative

1. **Complexity**: Two systems to operate and maintain
2. **Consistency challenges**: Need to handle cache-DB divergence
3. **Latency in write path**: Two hops for writes (DB + cache invalidation)
4. **Cost**: Running both PostgreSQL and Redis infrastructure

### Mitigations

| Risk | Mitigation |
|------|------------|
| Complexity | Abstract behind `Storage` trait; swap implementations easily |
| Consistency | Write-through with immediate invalidation; periodic reconciliation |
| Write latency | Async cache invalidation; don't block on cache ops |
| Cost | Can run Redis on same node for small deployments |

---

## Related Decisions

- ADR-001: Rust Core Selection
- ADR-002: Token Format Selection
- ADR-004: Deployment Architecture (planned)

---

**Status:** Proposed  
**Decision Date:** 2026-04-02  
**Review Date:** 2026-07-02  
**Line Count:** ~550 lines
