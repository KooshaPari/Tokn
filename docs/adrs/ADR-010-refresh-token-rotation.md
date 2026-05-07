# ADR-010: Refresh Token Rotation Strategy

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

Long-lived refresh tokens require rotation to prevent reuse attacks. We need to implement:
- Token rotation on each use (refresh token reuse detection)
- Family tracking for revoked token families
- Replay attack prevention
- Graceful expiration handling

---

## Decision

We will implement **refresh token rotation with family tracking**.

### Rotation Strategy

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Refresh Token Rotation Flow                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Initial Token Issuance:                                                    │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │   Token Family: fam_abc123                                           │  │
│  │   ┌──────────────────────────────────────────────────────────────┐  │  │
│  │   │  Token ID: tok_001          Generation: 1    Status: Active  │  │  │
│  │   │  Created: T0               Expires: T0 + 30 days            │  │  │
│  │   │  Previous: null                                         │  │  │
│  │   └──────────────────────────────────────────────────────────────┘  │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  First Refresh (T1):                                                         │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │   Token Family: fam_abc123                                           │  │
│  │   ┌──────────────────────────────────────────────────────────────┐  │  │
│  │   │  Token ID: tok_001          Generation: 1    Status: Used    │  │  │
│  │   │  Created: T0               Last Used: T1                    │  │  │
│  │   │  Previous: null                                         │  │  │
│  │   └──────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                         │  │
│  │                              │ Rotate                                  │  │
│  │                              ▼                                         │  │
│  │   ┌──────────────────────────────────────────────────────────────┐  │  │
│  │   │  Token ID: tok_002          Generation: 2    Status: Active │  │  │
│  │   │  Created: T1               Expires: T1 + 30 days            │  │  │
│  │   │  Previous: tok_001                                       │  │  │
│  │   └──────────────────────────────────────────────────────────────┘  │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Second Refresh (T2) - Token Reuse Detection:                                │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │   Attempt to use tok_001 (already used!)                           │  │
│  │                                                                      │  │
│  │   🚨 SECURITY EVENT: Refresh Token Reuse Detected                   │  │
│  │                                                                      │  │
│  │   Action: Revoke entire family (tok_001, tok_002, tok_003)        │  │
│  │   Alert: Security team notified                                     │  │
│  │   Audit: Log full context for investigation                        │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Data Model

```rust
#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub token_id: String,        // Unique token ID
    pub family_id: String,       // Token family for rotation tracking
    pub generation: u32,          // Rotation generation
    pub subject: String,         // User/service ID
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub status: RefreshTokenStatus,
    pub previous_token_id: Option<String>,  // For rotation chain
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RefreshTokenStatus {
    Active,      // Can be used
    Used,        // Already used for rotation
    Revoked,     // Explicitly revoked
    Expired,     // Past expiration
    Compromised, // Reuse detected - family invalidated
}

impl RefreshTokenStore {
    pub async fn rotate_token(
        &self,
        current_token: &RefreshToken,
        new_token: &mut RefreshToken,
    ) -> Result<RotationResult, RotationError> {
        // 1. Validate current token is active
        if current_token.status != RefreshTokenStatus::Active {
            return Err(RotationError::TokenNotActive);
        }
        
        // 2. Mark current token as used
        self.mark_used(current_token.token_id.clone()).await?;
        
        // 3. Create new token in same family
        new_token.family_id = current_token.family_id.clone();
        new_token.generation = current_token.generation + 1;
        new_token.previous_token_id = Some(current_token.token_id.clone());
        
        self.store(new_token).await?;
        
        Ok(RotationResult {
            new_token: new_token.clone(),
            previous_token_expires: current_token.expires_at,
        })
    }
    
    pub async fn detect_reuse(
        &self,
        token_id: &str,
    ) -> Result<ReuseDetectionResult, Error> {
        let token = self.get(token_id).await?;
        
        match token.status {
            RefreshTokenStatus::Used => {
                // Token already used - potential reuse attack!
                let family = self.get_family(&token.family_id).await?;
                
                // Revoke entire family
                self.revoke_family(&token.family_id).await?;
                
                Ok(ReuseDetectionResult {
                    reuse_detected: true,
                    family_revoked: true,
                    tokens_revoked: family.len() as u32,
                    security_alert: true,
                })
            }
            _ => Ok(ReuseDetectionResult {
                reuse_detected: false,
                family_revoked: false,
                tokens_revoked: 0,
                security_alert: false,
            })
        }
    }
}
```

### Rotation Parameters

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| **Token Lifetime** | 30 days | Balance security vs. UX |
| **Rotation Window** | 7 days | Allow refresh before expiry |
| **Max Generation** | 10 | Prevent infinite rotation |
| **Grace Period** | 24 hours | Old token still valid briefly |
| **Family Size Limit** | 20 tokens | Memory protection |

---

## Consequences

### Positive
- Detects stolen refresh tokens immediately
- Prevents replay attacks
- Complete audit trail
- Family-based invalidation
- Industry best practice

### Negative
- Complex state management
- Storage overhead per token
- Reuse detection requires database lookup
- False positives possible (network issues)

### Mitigation
- Implement robust detection logging
- Provide admin override for legitimate reuse
- Monitor for attack patterns
- Document expected behavior clearly

---

## References

- [OAuth 2.0 Token Rotation](https://datatracker.ietf.org/doc/html/rfc6749#section-1.5)
- [Refresh Token Best Practices](https://auth0.com/docs/security/store-tokens)
- [OAuth 2.0 Threat Model](https://datatracker.ietf.org/doc/html/rfc6819)
