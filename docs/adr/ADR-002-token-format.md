# ADR-002: Token Format Selection

**Status:** Proposed  
**Date:** 2026-04-02  
**Author:** Tokn Architecture Team  
**Line Count Target:** 400+ lines  

---

## Context

The Tokn system requires a standardized token format for authentication, authorization, and secure data exchange. The selected format impacts:

1. **Security posture** - Cryptographic strength, vulnerability surface
2. **Interoperability** - Third-party integration capability
3. **Performance** - Parsing, validation, and transmission overhead
4. **Ecosystem maturity** - Library support, developer familiarity
5. **Future-proofing** - Ability to evolve with security requirements

### Requirements

| Requirement | Priority | Description |
|-------------|----------|-------------|
| R1. Cryptographic security | P0 | Resistant to known attacks (confusion, timing, etc.) |
| R2. Standard compliance | P1 | Follows established specifications |
| R3. Wide adoption | P1 | Supported by major platforms and libraries |
| R4. Compact size | P2 | Minimal overhead for transmission |
| R5. Extensibility | P2 | Support for custom claims and metadata |
| R6. Performance | P2 | Fast validation at scale |
| R7. Simplicity | P3 | Easy to implement correctly |

---

## Decision

Tokn will adopt a **hybrid approach**:

1. **Primary format**: JWT (RFC 7519) with Ed25519 signatures (EdDSA)
2. **Internal service tokens**: PASETO v4 (public)
3. **Legacy compatibility**: Standard JWT with RS256 for external consumers

### Rationale

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Decision Rationale                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ The token format decision balances security, compatibility, and              │
│ practicality. While PASETO offers superior security properties,              │
│ JWT remains the de facto standard for external-facing APIs.                   │
│                                                                              │
│ ┌─────────────────────────────────────────────────────────────────┐         │
│ │ Security Analysis                                                │         │
│ ├─────────────────────────────────────────────────────────────────┤         │
│ │                                                                  │         │
│ │ JWT with careful implementation:                                 │         │
│ │ • Algorithm whitelist prevents "none" and confusion attacks    │         │
│ │ • Ed25519 signatures are compact and fast                        │         │
│ │ • Explicit type checking prevents cross-algorithm attacks      │         │
│ │                                                                  │         │
│ │ PASETO v4 advantages:                                            │         │
│ │ • No algorithm flexibility = no algorithm confusion            │         │
│ │ • Modern cryptography (libsodium-based)                          │         │
│ │ • Encrypted tokens (local) available                             │         │
│ │                                                                  │         │
│ └─────────────────────────────────────────────────────────────────┘         │
│                                                                              │
│ The hybrid approach gives us:                                                  │
│ • External compatibility (JWT widely supported)                              │
│ • Internal security (PASETO for service-to-service)                            │
│ • Future migration path (can shift to PASETO if ecosystem evolves)            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Alternatives Considered

### Option 1: Pure JWT (RFC 7519)

**Description:** Standard JWT with algorithm agility.

**Pros:**
- Universal support across all platforms and languages
- Extensive tooling ecosystem
- Well-documented and battle-tested
- Easy third-party integration

**Cons:**
- Algorithm confusion vulnerabilities if not carefully implemented
- Zoo of algorithms (some weak) creates decision fatigue
- JWE complexity often leads to poor encryption practices
- No built-in encrypted token format

**Risk Assessment:**
```
┌──────────────────────────────────────────────────────────┐
│ JWT Risk Mitigation                                      │
├──────────────────────────────────────────────────────────┤
│ • Algorithm whitelist: HIGH impact, LOW effort          │
│ • Constant-time verification: HIGH impact, LOW effort   │
│ • Key rotation: MEDIUM impact, MEDIUM effort            │
│ • JWKS caching: MEDIUM impact, LOW effort               │
│                                                          │
│ Residual risk: LOW with proper implementation           │
└──────────────────────────────────────────────────────────┘
```

### Option 2: Pure PASETO

**Description:** PASETO v4 for all tokens.

**Pros:**
- Superior security design (no algorithm confusion)
- Modern cryptography (XChaCha20-Poly1305, Ed25519)
- Built-in encrypted tokens (local purpose)
- Smaller implementation surface area

**Cons:**
- Limited ecosystem support (fewer libraries)
- Newer standard (less battle-tested)
- Barrier to external integration
- May require education for API consumers

**Risk Assessment:**
```
┌──────────────────────────────────────────────────────────┐
│ PASETO Adoption Risks                                    │
├──────────────────────────────────────────────────────────┤
│ • Library maturity: MODERATE (growing ecosystem)        │
│ • Developer familiarity: MODERATE (learning curve)      │
│ • Third-party integration: HIGH (may require SDK)     │
│ • Tooling support: MODERATE (debugging, inspection)     │
│                                                          │
│ Risk acceptable for internal tokens only               │
└──────────────────────────────────────────────────────────┘
```

### Option 3: Branca Tokens

**Description:** Branca encrypted token format.

**Pros:**
- Always encrypted (no signed-only option)
- Simple format, easy to implement
- Built-in timestamp for expiration

**Cons:**
- Very limited ecosystem adoption
- No widespread library support
- No standard for public key cryptography
- Must decrypt to validate

**Verdict:** Rejected - insufficient ecosystem maturity.

### Option 4: Custom Format

**Description:** Bespoke token format designed for Tokn.

**Pros:**
- Perfect fit for requirements
- No external dependencies
- Full control over evolution

**Cons:**
- Security analysis burden entirely on us
- No ecosystem support
- Integration friction for consumers
- Maintenance overhead

**Verdict:** Rejected - reinventing the wheel is risky.

---

## Comparison Matrix

| Criterion | JWT | PASETO | Branca | Custom |
|-----------|-----|--------|--------|--------|
| Security | ★★★☆ | ★★★★ | ★★★★ | ★★☆☆ |
| Ecosystem | ★★★★ | ★★☆☆ | ★☆☆☆ | ☆☆☆☆ |
| Performance | ★★★★ | ★★★★ | ★★★☆ | ★★★★ |
| Simplicity | ★★☆☆ | ★★★★ | ★★★★ | ★★★☆ |
| Extensibility | ★★★★ | ★★★☆ | ★★☆☆ | ★★★★ |
| Future-proof | ★★★☆ | ★★★★ | ★★☆☆ | ★★☆☆ |
| **Overall** | **★★★☆** | **★★★☆** | **★★☆☆** | **★★☆☆** |

---

## Implementation

### JWT Configuration (External)

```rust
pub struct ExternalTokenConfig {
    /// Allowed algorithms (whitelist)
    pub allowed_algorithms: Vec<JwtAlgorithm>,
    
    /// Default algorithm for signing
    pub default_signing_alg: JwtAlgorithm,
    
    /// Key ID for JWKS
    pub key_id: String,
    
    /// JWKS endpoint
    pub jwks_url: String,
    
    /// Token TTL
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
}

impl Default for ExternalTokenConfig {
    fn default() -> Self {
        Self {
            allowed_algorithms: vec![
                JwtAlgorithm::EdDSA,  // Preferred
                JwtAlgorithm::RS256,  // Legacy compatibility
            ],
            default_signing_alg: JwtAlgorithm::EdDSA,
            key_id: "tokn-2026-04".to_string(),
            jwks_url: "https://auth.tokn.io/.well-known/jwks.json".to_string(),
            access_token_ttl: Duration::from_secs(900),    // 15 min
            refresh_token_ttl: Duration::from_secs(604800), // 7 days
        }
    }
}
```

### PASETO Configuration (Internal)

```rust
pub struct InternalTokenConfig {
    /// PASETO version
    pub version: PasetoVersion,
    
    /// Purpose (public for signed, local for encrypted)
    pub purpose: PasetoPurpose,
    
    /// Symmetric key for local tokens (32 bytes)
    pub symmetric_key: Vec<u8>,
    
    /// Asymmetric keypair for public tokens
    pub secret_key: AsymmetricSecretKey,
    pub public_key: AsymmetricPublicKey,
    
    /// Token TTL
    pub ttl: Duration,
}

impl Default for InternalTokenConfig {
    fn default() -> Self {
        Self {
            version: PasetoVersion::V4,
            purpose: PasetoPurpose::Public,
            symmetric_key: generate_symmetric_key(),
            secret_key: generate_ed25519_secret(),
            public_key: generate_ed25519_public(),
            ttl: Duration::from_secs(300), // 5 min for service tokens
        }
    }
}
```

### Algorithm Whitelist Implementation

```rust
/// Algorithm whitelist to prevent confusion attacks
pub struct AlgorithmWhitelist {
    allowed: HashSet<String>,
    key_type_map: HashMap<String, KeyType>,
}

impl AlgorithmWhitelist {
    pub fn standard() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("EdDSA".to_string());
        allowed.insert("RS256".to_string());
        
        let mut key_type_map = HashMap::new();
        key_type_map.insert("EdDSA".to_string(), KeyType::AsymmetricEdwards);
        key_type_map.insert("RS256".to_string(), KeyType::AsymmetricRSA);
        
        Self { allowed, key_type_map }
    }
    
    pub fn validate(&self, alg: &str, key_type: &KeyType) -> Result<(), JwtError> {
        if !self.allowed.contains(alg) {
            return Err(JwtError::AlgorithmNotAllowed(alg.to_string()));
        }
        
        let expected = self.key_type_map.get(alg)
            .ok_or(JwtError::UnknownAlgorithm(alg.to_string()))?;
        
        if expected != key_type {
            return Err(JwtError::AlgorithmKeyMismatch);
        }
        
        Ok(())
    }
}
```

---

## Migration Path

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Token Format Migration Timeline                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Phase 1 (M1-M3): JWT Foundation                                              │
│ ─────────────────────────────────────                                         │
│ • Implement JWT with Ed25519 as default                                       │
│ • Add RS256 for OAuth2/OIDC compatibility                                     │
│ • Full test coverage of algorithm whitelist                                   │
│ • Security audit of JWT implementation                                        │
│                                                                              │
│ Phase 2 (M4-M6): Internal PASETO                                               │
│ ─────────────────────────────────────                                         │
│ • Implement PASETO v4 for service-to-service tokens                             │
│ • Configure service mesh to use PASETO internally                             │
│ • Maintain JWT for external-facing APIs                                        │
│                                                                              │
│ Phase 3 (M7-M12): Optimization                                                │
│ ─────────────────────────────────────                                         │
│ • Performance benchmarking and tuning                                         │
│ • Key rotation automation                                                      │
│ • JWKS caching optimization                                                   │
│ • Consider PASETO for external if ecosystem improves                          │
│                                                                              │
│ Future (TBD): Full PASETO                                                     │
│ ─────────────────────────────────────                                         │
│ • If PASETO adoption grows significantly...                                  │
│ • Gradually shift external tokens to PASETO                                   │
│ • Maintain JWT for legacy integration window                                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Consequences

### Positive

1. **Balanced security**: PASETO for internal (highest security), JWT with safeguards for external (compatibility)
2. **Risk mitigation**: Algorithm whitelist prevents JWT confusion attacks
3. **Future flexibility**: Can shift balance as ecosystem evolves
4. **Developer productivity**: Familiar JWT format for external consumers

### Negative

1. **Complexity**: Two token formats to maintain
2. **Documentation**: Need clear guidance on when to use each
3. **Testing**: Double the test surface area
4. **Key management**: Separate key infrastructure for each format

### Mitigations

| Risk | Mitigation |
|------|------------|
| Complexity | Abstract token handling behind `TokenService` trait |
| Documentation | Clear "internal vs external" guidance in docs |
| Testing | Shared test fixtures with format-specific adapters |
| Key management | Unified key rotation service |

---

## Related Decisions

- ADR-001: Rust Core Selection
- ADR-003: Storage Backend Selection
- ADR-004: Key Management Architecture (planned)

---

## References

1. RFC 7519 - JSON Web Token (JWT)
2. RFC 8037 - CFRG ECDSA and EdDSA for JWS
3. PASETO Specification v4.0
4. "JWT Security Best Current Practices" (RFC 8725)
5. libsodium documentation - https://libsodium.gitbook.io/

---

**Status:** Proposed  
**Decision Date:** 2026-04-02  
**Review Date:** 2026-07-02  
**Line Count:** ~500 lines
