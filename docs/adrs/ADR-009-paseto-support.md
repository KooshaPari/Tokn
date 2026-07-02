# ADR-009: PASETO v4 as Secondary Token Format

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

We need to support PASETO as a modern alternative to JWT. PASETO eliminates several JWT vulnerabilities:
- No algorithm confusion (fixed by version)
- No cryptographic agility (version locked)
- Authenticated encryption by default

---

## Decision

We will implement **PASETO v4** as the secondary token format with full support.

### PASETO v4 Features

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    PASETO v4 Architecture                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Token Structure:                                                            │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │   v4.public.eyJzdWIiOiJ1c3IxMjMiLCJhdWQiOiJhcHAxIn0.               │  │
│  │   ZmI6xq9nT8KYZ8rQ3mK2vL5pJ1oH6sD4eF0gU7xV2wA9cM3bK8pR1sT5uW2yX7zQ9│  │
│  │                                                                      │  │
│  │   │         │                           │                           │  │
│  │   │         │                           │                           │  │
│  │   │         │                           │                           │  │
│  │   ▼         ▼                           ▼                           │  │
│  │  Ver  Purpose                    Payload                Signature   │  │
│  │                                                                      │  │
│  │  v4.public = Ed25519 signature with SHA-384                         │  │
│  │  v4.local  = AES-256-CTR + HMAC-SHA-384 (encrypted)                 │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Comparison with JWT:                                                         │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  Aspect              │  JWT              │  PASETO v4             │  │
│  ├───────────────────────┼───────────────────┼─────────────────────────┤  │
│  │  Algorithm           │  Flexible (risk)  │  Fixed by version      │  │
│  │  Encryption          │  Optional (JWE)   │  v4.local = default    │  │
│  │  Algorithm None      │  Possible (bug)   │  Impossible            │  │
│  │  Key Confusion        │  Possible         │  Impossible            │  │
│  │  Implementation      │  Complex          │  Simple                │  │
│  │  Interoperability     │  High             │  Growing               │  │
│  │  Library Support      │  Excellent        │  Good                  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Implementation

```rust
pub enum TokenFormat {
    Jwt(Algorithm),
    Paseto(PasetoPurpose),
}

#[derive(Debug, Clone, Copy)]
pub enum PasetoPurpose {
    Public,   // v4.public - signed, not encrypted
    Local,    // v4.local  - signed + encrypted
}

impl PasetoService {
    pub async fn sign_paseto(
        &self,
        claims: &Claims,
        purpose: PasetoPurpose,
        key: &Ed25519SecretKey,
    ) -> Result<String, PasetoError> {
        match purpose {
            PasetoPurpose::Public => {
                // v4.public: EdDSA with SHA-384
                let payload = self.encode_payload(claims);
                let message = format!("v4.public.{}", payload);
                let signature = self.sign(&message, key);
                Ok(format!("{}.{}", message, signature))
            }
            PasetoPurpose::Local => {
                // v4.local: AES-256-CTR + HMAC-SHA-384
                let plaintext = self.encode_payload(claims);
                let nonce = self.generate_nonce();
                let ciphertext = self.encrypt(plaintext, key, &nonce)?;
                let mac = self.compute_mac(&ciphertext, key, &nonce);
                Ok(format!("v4.local.{}.{}.{}", nonce, ciphertext, mac))
            }
        }
    }
    
    pub async fn verify_paseto(
        &self,
        token: &str,
        public_key: &Ed25519PublicKey,
    ) -> Result<Claims, PasetoError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 4 {
            return Err(PasetoError::InvalidFormat);
        }
        
        let (version, purpose, payload, signature) = 
            (parts[0], parts[1], parts[2], parts[3]);
        
        match purpose {
            "public" => {
                self.verify_public(parts, public_key).await
            }
            "local" => {
                self.verify_local(parts, public_key).await
            }
            _ => Err(PasetoError::UnknownPurpose)
        }
    }
}
```

### Use Cases

| Token Type | Use Case | PASETO Version |
|------------|----------|---------------|
| **Internal API** | Service-to-service | PASETO v4.local |
| **External API** | Third-party clients | PASETO v4.public |
| **Legacy Support** | JWT-required systems | JWT RS256 |
| **High Security** | PII-containing tokens | PASETO v4.local |

---

## Consequences

### Positive
- Eliminates JWT algorithm confusion attacks
- Simpler implementation than JWE
- Authenticated encryption in local tokens
- No algorithm negotiation vulnerabilities
- Growing ecosystem support

### Negative
- Less library support than JWT
- Local tokens require key agreement (PAKE)
- Payload visible in public tokens (but not forgeable)
- Newer standard, less battle-tested

### Mitigation
- Provide JWT fallback for legacy systems
- Use PASETO local for sensitive data
- Monitor PASETO library maturity
- Document interoperability requirements

---

## References

- [Paseto Specification](https://github.com/paseto-standard/paseto-spec)
- [Paseto RFC Draft](https://datatracker.ietf.org/doc/html/draft-paragon-paseto)
- [Paragon Initiative Enterprises PASETO](https://github.com/paragonie/paseto)
