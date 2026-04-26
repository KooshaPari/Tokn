# ADR-007: Rate Limiting via Token Bucket Algorithm

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

Rate limiting protects the token service from abuse and ensures fair resource allocation. We need to limit:
- Token issuance per subject/tenant
- Token validation per client IP
- Revocation requests per actor
- API requests overall

Previous approaches:
- **Fixed window counter** - Burst issues at window boundaries
- **Sliding log** - Memory intensive
- **Token bucket (chosen)** - Allows bursts, memory efficient

---

## Decision

We will use the **Token Bucket algorithm with Redis backing** for distributed rate limiting.

### Algorithm Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Token Bucket Algorithm Visualization                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Bucket State:                                                                │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │     Capacity: 100 tokens          Refill Rate: 10 tokens/second     │  │
│  │         │                                                        │  │
│  │         ▼                                                        │  │
│  │    ┌─────────────────────────────────────────────────────────┐  │  │
│  │    │  ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●  │  │  │
│  │    │  ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●  │  │  │
│  │    │  ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●  │  │  │
│  │    │  ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●  │  │  │
│  │    │  ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●  │  │  │
│  │    │  ● ● ● ● ● ● ● ● ● ●  │  │  │
│  │    │  ● ● ●  │  │  │
│  │    └─────────────────────────────────────────────────────────┘  │  │
│  │         │                                                        │  │
│  │         ▼                                                        │  │
│  │    25 tokens remaining (25/100)                                  │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Token Refill:                                                                │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │    Time: T+0s ────► T+1s ────► T+2s ────► T+3s ────► T+10s        │  │
│  │                  │            │            │            │           │  │
│  │                  ▼            ▼            ▼            ▼           │  │
│  │             +10 tokens    +10 tokens    +10 tokens    +100 tokens   │  │
│  │             (35 total)    (45 total)    (55 total)    (100 cap)    │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Burst Handling:                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │    Request 1: Consume 1 token (99 remaining)                        │  │
│  │    Request 2: Consume 1 token (98 remaining)                        │  │
│  │    ...                                                              │  │
│  │    Request 75: Consume 1 token (25 remaining)                      │  │
│  │    Request 76: REJECTED (only 25 tokens, need 1, allowed)          │  │
│  │    Request 77: REJECTED                                             │  │
│  │    ...                                                              │  │
│  │    Wait 3 seconds: +30 tokens (55 total)                           │  │
│  │    Request 78: Consume 1 token (54 remaining) ✓                    │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Redis Implementation

```rust
#[derive(Debug, Clone)]
pub struct RedisRateLimiter {
    client: redis::aio::MultiplexedConnection,
}

pub struct RateLimitConfig {
    pub bucket_capacity: u64,
    pub refill_rate: u64,  // tokens per second
    pub refill_interval: Duration,
}

impl RedisRateLimiter {
    pub async fn check_rate_limit(
        &self,
        key: &RateLimitKey,
        cost: u64,
    ) -> Result<RateLimitResult, Error> {
        let script = r#"
            local key = KEYS[1]
            local capacity = tonumber(ARGV[1])
            local refill_rate = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])
            local cost = tonumber(ARGV[4])
            
            -- Get current state
            local bucket = redis.call('HMGET', key, 'tokens', 'last_refill')
            local tokens = tonumber(bucket[1]) or capacity
            local last_refill = tonumber(bucket[2]) or now
            
            -- Calculate refill
            local elapsed = now - last_refill
            local refill = math.floor(elapsed * refill_rate / 1000)
            tokens = math.min(capacity, tokens + refill)
            
            -- Check if request allowed
            local allowed = 0
            local remaining = tokens
            if tokens >= cost then
                allowed = 1
                remaining = tokens - cost
            end
            
            -- Update state
            redis.call('HMSET', key, 'tokens', remaining, 'last_refill', now)
            redis.call('EXPIRE', key, 3600)  -- 1 hour TTL
            
            return {allowed, remaining, capacity}
        "#;
        
        let now = Utc::now().timestamp_millis();
        let result: Vec<i64> = redis::Script::new(script)
            .key(key.to_redis_key())
            .arg(self.config.bucket_capacity)
            .arg(self.config.refill_rate)
            .arg(now)
            .arg(cost)
            .invoke_async(&mut self.client.clone())
            .await?;
        
        Ok(RateLimitResult {
            allowed: result[0] == 1,
            remaining: result[1] as u64,
            limit: result[2] as u64,
            retry_after: if result[0] == 0 {
                Some(Duration::from_secs_f64(
                    (cost as f64 - result[1] as f64) / self.config.refill_rate as f64
                ))
            } else {
                None
            },
        })
    }
}
```

### Rate Limit Tiers

| Tier | Token Issuance | Token Validation | Burst Allowance | Use Case |
|------|---------------|-----------------|----------------|----------|
| **Free** | 100/hour | 1000/min | 10x | Development |
| **Standard** | 1000/hour | 5000/min | 10x | Production |
| **Premium** | 10000/hour | 50000/min | 20x | High-volume |
| **Enterprise** | Unlimited | Unlimited | 50x | Custom SLA |

---

## Consequences

### Positive
- Token bucket allows controlled bursting
- Redis backing enables distributed rate limiting
- Memory-efficient (O(1) per key)
- Smooth rate limiting without boundary spikes
- Configurable per tenant/subject/API key

### Negative
- Requires Redis infrastructure
- Lua script complexity for atomicity
- Distributed clock skew (mitigated via Redis time)
- Rate limit state loss on Redis restart

### Mitigation
- Use Redis Cluster for high availability
- Persist rate limit state periodically
- Graceful degradation on Redis failure
- Clear documentation of rate limits

---

## References

- [Token Bucket Algorithm](https://en.wikipedia.org/wiki/Token_bucket)
- [Redis Rate Limiting Patterns](https://redis.io/docs/manual/pubsub/)
- [Cloudflare Rate Limiting](https://developers.cloudflare.com/rate-limiting/)
