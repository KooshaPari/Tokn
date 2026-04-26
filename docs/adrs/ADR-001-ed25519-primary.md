# ADR-001: Use Ed25519 as Primary Signing Algorithm

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

We need to select a primary signing algorithm for token issuance. The options include RSA-based algorithms (RS256, RS384, RS512), ECDSA algorithms (ES256, ES384, ES512), and Edwards curve algorithms (Ed25519, Ed448).

Modern applications require:
- High performance signing and verification
- Strong security properties
- Small key sizes for efficient storage
- Compatibility with modern cryptographic libraries

---

## Decision

We will use **Ed25519** as the primary signing algorithm for token issuance.

### Justification

| Algorithm | Key Size | Signature Size | Performance | Security Level |
|-----------|----------|---------------|------------|----------------|
| RS256 | 2048 bits | 256 bytes | Medium | 112 bits |
| RS384 | 3072 bits | 384 bytes | Slow | 128 bits |
| RS512 | 4096 bits | 512 bytes | Very Slow | 128 bits |
| ES256 | 256 bits | 64 bytes | Fast | 128 bits |
| ES384 | 384 bits | 96 bytes | Medium | 192 bits |
| ES512 | 521 bits | 132 bytes | Slow | 256 bits |
| **Ed25519** | 256 bits | 64 bytes | **Very Fast** | **256 bits** |

Ed25519 provides:
1. **Superior performance** - 3x faster than RSA, comparable to ECDSA
2. **Small signatures** - 64 bytes, same as ES256 but with higher security
3. **High security** - 256-bit security level, equivalent to AES-256
4. **Side-channel resistance** - Designed to be resistant to timing attacks
5. **Deterministic signatures** - No ECDSA malleability issues
6. **Modern standard** - Widely adopted in modern protocols (PASETO, WireGuard)

### Secondary Algorithm Support

- **RS256** - Supported for legacy system compatibility
- **ES256** - Supported for environments requiring ECDSA
- **PASETO v4** - Full support for platform-agnostic security tokens

---

## Consequences

### Positive
- Faster token operations across all endpoints
- Reduced storage requirements for token data
- Stronger cryptographic guarantees
- Modern, future-proof cryptography

### Negative
- Not compatible with systems requiring RSA certificates
- Some legacy IdPs may not support Ed25519
- Requires additional configuration for RSA-only environments

### Mitigation
- Provide RS256 as fallback for enterprise compatibility
- Document interoperability requirements clearly
- Consider EdDSA certificate chains for future upgrades

---

## References

- [RFC 8032 - EdDSA](https://datatracker.ietf.org/doc/html/rfc8032)
- [RFC 8080 - Edwards-Curve Digital Security Algorithm](https://datatracker.ietf.org/doc/html/rfc8080)
- [Bernstein et al. - Ed25519: High-speed high-security signatures](https://ed25519.cr.yp.to/ed25519-20110701.pdf)
