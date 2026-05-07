# ADR-006: JWKS Endpoint for Public Key Distribution

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

Token validation by external parties requires access to public keys. We need:
- Standardized key distribution mechanism
- Support for key rotation without service disruption
- Multiple algorithm support
- Client-side key caching

---

## Decision

We will implement a **RFC 7517 compliant JWKS endpoint** with automated key rotation support.

### JWKS Endpoint Design

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         JWKS Endpoint Architecture                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Endpoint: GET /.well-known/jwks.json                                        │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  Response Structure                                                   │  │
│  │                                                                       │  │
│  │  {                                                                    │  │
│  │    "keys": [                                                          │  │
│  │      {                                                                │  │
│  │        "kty": "OKP",           // Key Type: Octet Key Pair          │  │
│  │        "crv": "Ed25519",        // Curve: Ed25519                    │  │
│  │        "x": "K6qchxlLy2j7...   // Public Key (base64url)            │  │
│  │        "use": "sig",            // Usage: Signature                  │  │
│  │        "alg": "EdDSA",          // Algorithm: EdDSA                  │  │
│  │        "kid": "2026-04-02-1",   // Key ID: Date-based rotation ID    │  │
│  │        "exp": 1743638465        // Expiration: 2026-04-02           │  │
│  │      },                                                               │  │
│  │      {                                                                │  │
│  │        "kty": "RSA",             // Key Type: RSA                    │  │
│  │        "n": "0vx7agoebGcQ...   // Modulus (base64url)               │  │
│  │        "e": "AQAB",              // Exponent: 65537                  │  │
│  │        "use": "sig",             // Usage: Signature                 │  │
│  │        "alg": "RS256",           // Algorithm: RS256                 │  │
│  │        "kid": "2026-04-02-rsa",  // Key ID                           │  │
│  │        "exp": 1743638465         // Expiration                       │  │
│  │      }                                                                │  │
│  │    ]                                                                  │  │
│  │  }                                                                    │  │
│  │                                                                       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Key Rotation Strategy:                                                      │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  Old Key (N-1)  │  Current Key (N)  │  Next Key (N+1)                │  │
│  │  ┌───────────┐  │  ┌───────────┐    │  ┌───────────┐                │  │
│  │  │ Valid     │  │  │ Valid     │    │  │ Pre-      │                │  │
│  │  │ (90 days) │  │  │ (Active)  │    │  │ Generated │                │  │
│  │  └───────────┘  │  └───────────┘    │  └───────────┘                │  │
│  │       │         │       │           │       │                        │  │
│  │       └─────────┴───────┴───────────┴───────┘                        │  │
│  │                   Timeline                                             │  │
│  │  ─────────────────────────────────────────────────────────────────►  │  │
│  │  Day 0        Day 30        Day 60        Day 90                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Endpoint Implementation

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct JwksResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jwk {
    pub kty: String,        // RSA, EC, OKP
    pub use_: Option<String>,
    pub alg: Option<String>,
    pub kid: String,
    pub exp: Option<i64>,
    
    // RSA-specific
    pub n: Option<String>,  // Modulus
    pub e: Option<String>,  // Exponent
    
    // EC-specific
    pub crv: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
    
    // OKP-specific (Ed25519, X25519)
    pub x: Option<String>,
}

impl JwksEndpoint {
    pub async fn get_jwks(&self) -> Result<JwksResponse, Error> {
        let keys = self.key_service.get_active_keys().await?;
        
        Ok(JwksResponse {
            keys: keys.into_iter().map(|k| k.into_jwk()).collect(),
        })
    }
    
    pub async fn rotate_keys(&self) -> Result<KeyRotationResult, Error> {
        // 1. Generate new key pair
        let new_key = self.key_service.generate_key(KeyType::Ed25519).await?;
        
        // 2. Add new key to active set (old keys still valid)
        self.key_service.add_key(new_key).await?;
        
        // 3. Schedule old key expiration (90 days)
        self.key_service.schedule_expiration(old_key_id, Duration::days(90)).await?;
        
        Ok(KeyRotationResult {
            new_key_id: new_key.kid,
            old_key_expires: expiration_date,
        })
    }
}
```

### RFC 7517 Compliance

| Requirement | Implementation | Status |
|-------------|---------------|--------|
| **kty (Key Type)** | OKP (Ed25519), RSA, EC | ✅ Implemented |
| **use (Public Key Use)** | sig (signature) | ✅ Implemented |
| **alg (Algorithm)** | EdDSA, RS256, ES256 | ✅ Implemented |
| **kid (Key ID)** | Date-based with increment | ✅ Implemented |
| **exp (Expiration)** | Unix timestamp | ✅ Implemented |
| **x, y (Coordinates)** | Base64url encoded | ✅ Implemented |
| **n, e (RSA params)** | Base64url encoded | ✅ Implemented |
| **crv (Curve)** | P-256, P-384, Ed25519 | ✅ Implemented |

---

## Consequences

### Positive
- Standard-compliant key distribution
- Supports multiple algorithms simultaneously
- Key rotation without service disruption
- Client-side caching reduces latency
- Compatible with major JWT libraries

### Negative
- Key rotation requires careful coordination
- Cache invalidation delays can cause validation failures
- Multiple key versions increase complexity
- Additional infrastructure for key storage

### Mitigation
- Overlap period for key rotation (old + new valid)
- Implement key ID caching with TTL
- Provide rotation status endpoint for monitoring
- Use date-based kid for easy identification

---

## References

- [RFC 7517 - JSON Web Key](https://datatracker.ietf.org/doc/html/rfc7517)
- [RFC 7518 - JSON Web Algorithms](https://datatracker.ietf.org/doc/html/rfc7518)
- [RFC 8037 - CFRG Elliptic Curve Diffie-Hellman and Signatures in JWK](https://datatracker.ietf.org/doc/html/rfc8037)
