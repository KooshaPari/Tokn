# ADR-003: Token Revocation via Redis Set with TTL

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

Token revocation must be immediate and highly available. We need to support:
- Single token revocation (by JTI)
- Subject-wide revocation (all tokens for a user)
- Bulk revocation (by timestamp, scope, etc.)

Previous approaches considered:
- **Database-only revocation lists** - Too slow for high-throughput validation
- **Blocklists in memory** - No horizontal scaling
- **Distributed cache with invalidation** - Complex consistency

---

## Decision

We will use a **Redis SET with per-token TTL** for revocation tracking.

### Data Structures

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Revocation Storage Architecture                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Redis Key Structure:                                                        │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Key Pattern                          │ Value    │ TTL               │   │
│  ├─────────────────────────────────────────────────────────────────────┤   │
│  │ revoke:jti:{jti}                     │ 1        │ token.expires_at  │   │
│  │ revoke:sub:{subject}                 │ SET{jti} │ token.expires_at  │   │
│  │ revoke:scope:{scope_pattern}         │ SET{jti} │ token.expires_at  │   │
│  │ revoke:iat:{timestamp}               │ SET{jti} │ max_token_ttl     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  JTI Lookup (O(1)):                                                          │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  GET revoke:jti:550e8400-e29b-41d4-a716-446655440000              │   │
│  │  → 1 (if revoked) or nil (if valid)                               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  Subject Lookup (O(m) where m = token count):                               │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  SMEMBERS revoke:sub:user_123                                       │   │
│  │  → {"jti1", "jti2", "jti3"}                                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Implementation Details

```rust
pub struct RedisRevocationStore {
    client: redis::aio::MultiplexedConnection,
}

impl RedisRevocationStore {
    pub async fn revoke_single(&self, jti: &str, expires_at: DateTime<Utc>) -> Result<()> {
        let ttl = (expires_at - Utc::now()).to_std()?;
        
        // Single JTI revocation
        let _: () = self.client
            .set_ex(format!("revoke:jti:{}", jti), "1", ttl)
            .await?;
        
        Ok(())
    }
    
    pub async fn revoke_by_subject(&self, subject: &str, tokens: Vec<TokenRecord>) -> Result<()> {
        let mut pipe = redis::pipe();
        
        for token in &tokens {
            let jti = &token.jti;
            let ttl = (token.expires_at - Utc::now()).to_std()?;
            
            // Add to subject set
            pipe.sadd(format!("revoke:sub:{}", subject), jti);
            pipe.expire(format!("revoke:sub:{}", subject), ttl);
            
            // Individual JTI for fast lookup
            pipe.set_ex(format!("revoke:jti:{}", jti), "1", ttl);
        }
        
        pipe.query_async(&mut self.client).await?;
        Ok(())
    }
    
    pub async fn is_revoked(&self, jti: &str) -> Result<bool> {
        let exists: bool = self.client
            .exists(format!("revoke:jti:{}", jti))
            .await?;
        Ok(exists)
    }
}
```

### Rationale

| Approach | Lookup Speed | Storage | Scaling | Consistency |
|----------|-------------|---------|---------|-------------|
| **Redis SET (chosen)** | O(1) JTI | O(n) revoked | Horizontal | Eventual |
| Database table | O(log n) | O(n) | Vertical | Strong |
| In-memory set | O(1) | O(n) | None | Strong |
| Bloom filter | O(k) | O(1) | Complex | Probabilistic |

### Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Revocation write | <1ms | p99 |
| Single JTI check | <0.1ms | p99 |
| Subject revocation | <10ms | per 100 tokens |
| Cache memory | <1GB | per 1M tokens |

---

## Consequences

### Positive
- O(1) lookup for JTI-based revocation
- Automatic TTL cleanup of expired revocations
- Horizontal scaling via Redis Cluster
- Simple implementation
- Efficient memory usage

### Negative
- Requires Redis infrastructure
- Subject-based lookup requires full set scan
- Eventual consistency in cluster mode
- Memory pressure with many active revocations

### Mitigation
- Use Redis Cluster for horizontal scaling
- Set max TTL to longest possible token lifetime
- Monitor memory and scale Redis accordingly
- Consider subject-level caching for frequently revoked subjects

---

## References

- [Redis SETEX](https://redis.io/commands/setex/)
- [Redis SADD with EXPIRE](https://redis.io/commands/sadd/)
- [Tokn Token Revocation Design](../design/revocation.md)
