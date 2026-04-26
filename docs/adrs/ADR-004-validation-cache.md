# ADR-004: Validation Cache with Predictive Preloading

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

High-throughput token validation requires caching to avoid repeated cryptographic operations. We need:
- Sub-millisecond validation for cache hits
- High cache hit rate (>95%)
- Minimal memory footprint
- Cache invalidation on revocation

Previous approaches:
- **No caching** - Too slow for high-throughput
- **Simple LRU cache** - No preloading, poor hit rate
- **Write-through cache** - Complex, high write latency

---

## Decision

We will implement a **two-tier cache with predictive preloading**.

### Cache Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Validation Cache Architecture                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                        Cache Hierarchy                                 │  │
│  │                                                                       │  │
│  │   Tier 1: In-Memory LRU                                                │  │
│  │   ┌──────────────────────────────────────────────────────────────┐   │  │
│  │   │  Hot Tokens (Top 20%)                                         │   │  │
│  │   │  Latency: <0.01ms    Size: 100MB max                       │   │  │
│  │   └──────────────────────────────────────────────────────────────┘   │  │
│  │                              │                                         │  │
│  │                              │ Cache Miss                              │  │
│  │                              ▼                                         │  │
│  │   Tier 2: Redis Distributed Cache                                      │  │
│  │   ┌──────────────────────────────────────────────────────────────┐   │  │
│  │   │  All Validated Tokens                                        │   │  │
│  │   │  Latency: <1ms        Size: Unlimited                     │   │  │
│  │   └──────────────────────────────────────────────────────────────┘   │  │
│  │                              │                                         │  │
│  │                              │ Cache Miss                              │  │
│  │                              ▼                                         │  │
│  │   Origin: Database + Cryptographic Verification                       │  │
│  │                                                                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Preloading Strategy:                                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │   On Token Issue:                                                     │  │
│  │   ┌──────────────────────────────────────────────────────────────┐  │  │
│  │   │  1. Issue token to client                                    │  │  │
│  │   │  2. Preload into Tier-2 (Redis) - async                      │  │  │
│  │   │  3. Preload into Tier-1 (local) - async                     │  │  │
│  │   │  4. Return token to client                                   │  │  │
│  │   └──────────────────────────────────────────────────────────────┘  │  │
│  │                                                                      │  │
│  │   Predictive Access:                                                  │  │
│  │   ┌──────────────────────────────────────────────────────────────┐  │  │
│  │   │  • Monitor access patterns                                   │  │  │
│  │   │  • Preload tokens client likely to validate next            │  │  │
│  │   │  • ML-based prediction (future)                             │  │  │
│  │   └──────────────────────────────────────────────────────────────┘  │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Cache Key Design

```rust
pub struct ValidationCache {
    local_cache: Arc<Mutex<LruCache<String, CachedValidation>>>,
    redis: RedisClient,
    config: CacheConfig,
}

impl ValidationCache {
    fn cache_key(&self, token: &str) -> String {
        format!("validate:{}", hash_sha256(token))
    }
}

#[derive(Debug, Clone)]
pub struct CachedValidation {
    pub valid: bool,
    pub claims: Option<Claims>,
    pub jti: Option<String>,
    pub subject: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub scopes: Vec<String>,
    pub cached_at: DateTime<Utc>,
    pub revocation_check_needed: bool,
}
```

### Cache Flow

```rust
impl ValidationService {
    pub async fn validate(
        &self,
        request: ValidationRequest,
    ) -> Result<ValidationResponse, ValidationError> {
        let cache_key = self.cache.cache_key(&request.token);
        
        // 1. Check Tier-1 (local cache)
        if let Some(cached) = self.cache.get_local(&cache_key).await? {
            if !cached.revocation_check_needed || !request.check_revocation {
                return Ok(cached.to_response());
            }
        }
        
        // 2. Check Tier-2 (Redis cache)
        if let Some(cached) = self.cache.get_redis(&cache_key).await? {
            if !cached.revocation_check_needed || !request.check_revocation {
                // Promote to Tier-1
                self.cache.put_local(cache_key.clone(), cached.clone()).await?;
                return Ok(cached.to_response());
            }
        }
        
        // 3. Full validation
        let response = self.validate_full(&request).await?;
        
        // 4. Cache result
        if response.valid {
            let cached = CachedValidation::from_response(&response);
            self.cache.put_redis(cache_key.clone(), &cached).await?;
            self.cache.put_local(cache_key, &cached).await?;
        }
        
        Ok(response)
    }
}
```

### Performance Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Cache Hit Latency | <0.01ms | TBD |
| Cache Miss Latency | <5ms | TBD |
| Hit Rate | >95% | TBD |
| Memory per Entry | ~500 bytes | TBD |
| Max Local Entries | 100,000 | TBD |

---

## Consequences

### Positive
- Sub-millisecond validation for cache hits
- High hit rate with predictive preloading
- Distributed cache for horizontal scaling
- Automatic cache invalidation on revocation
- Memory-efficient LRU eviction

### Negative
- Additional Redis infrastructure required
- Cache coherence challenges in distributed environment
- Preloading adds overhead on token issuance
- Memory usage for local cache

### Mitigation
- Use Redis Cluster for high availability
- Implement cache metrics monitoring
- Tune TTL based on workload characteristics
- Provide cache bypass option for security-critical validations

---

## References

- [Redis LRU Cache](https://redis.io/docs/manual/eviction/)
- [LRU Cache Implementation](https://en.wikipedia.org/wiki/Cache_algorithms)
- [Tokn Cache Design](../design/cache.md)
