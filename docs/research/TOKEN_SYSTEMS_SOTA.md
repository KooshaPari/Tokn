# Token Systems State-of-the-Art Research

**Project:** Tokn - Token Management and Modularization System  
**Research Date:** 2026-04-02  
**Classification:** Deep Technical Research  
**Line Count Target:** 1,500+ lines  

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [JWT Deep Dive](#jwt-deep-dive)
3. [OAuth 2.0 and OpenID Connect](#oauth-20-and-openid-connect)
4. [API Key Management Patterns](#api-key-management-patterns)
5. [Token Storage Strategies](#token-storage-strategies)
6. [Revocation Strategies](#revocation-strategies)
7. [Cryptographic Approaches](#cryptographic-approaches)
8. [Token Format Standards Comparison](#token-format-standards-comparison)
9. [Architecture Patterns for Token Services](#architecture-patterns-for-token-services)
10. [Security Best Practices](#security-best-practices)
11. [Performance Considerations](#performance-considerations)
12. [References](#references)

---

## Executive Summary

Token-based authentication has become the cornerstone of modern distributed systems. This research document provides a comprehensive analysis of token systems, covering JWT implementation details, OAuth 2.0/OIDC flows, API key management, storage strategies, revocation mechanisms, cryptographic foundations, and architectural patterns.

### Key Findings

| Aspect | Current SOTA | Recommendation for Tokn |
|--------|--------------|------------------------|
| Token Format | JWT (RFC 7519) with JWS | JWT with configurable algorithms |
| Signing | RS256/ES256 for asymmetric, HS256 for symmetric | Pluggable crypto providers |
| Storage | Redis + PostgreSQL hybrid | Modular storage backend |
| Revocation | Short TTL + blacklist hybrid | Configurable strategy per token type |
| Validation | JWKS endpoint rotation | JWKS with local caching |

---

## JWT Deep Dive

### 1.1 JWT Structure and Encoding

A JSON Web Token consists of three parts separated by dots (`.`):

```
xxxxx.yyyyy.zzzzz
  │      │      │
  │      │      └── Signature (JWS)
  │      └───────── Payload (Claims)
  └──────────────── Header (Metadata)
```

#### Header Structure

```json
{
  "alg": "RS256",
  "typ": "JWT",
  "kid": "2024-01-key-1",
  "cty": "JWT"
}
```

**Header Fields:**

| Field | Description | Required |
|-------|-------------|----------|
| `alg` | Signature algorithm | Yes |
| `typ` | Token type | No (default: JWT) |
| `kid` | Key identifier | Recommended |
| `cty` | Content type | For nested JWTs |
| `jku` | JWK Set URL | For key distribution |
| `x5c` | X.509 Certificate Chain | Alternative to JWK |
| `x5t` | X.509 Certificate SHA-1 Thumbprint | Certificate identification |
| `x5t#S256` | X.509 Certificate SHA-256 Thumbprint | Preferred over x5t |
| `crit` | Critical extensions | For custom fields |

#### Payload Claims

Registered claims per RFC 7519:

```json
{
  "iss": "https://auth.tokn.io",
  "sub": "user_1234567890",
  "aud": "tokn-api",
  "exp": 1735689600,
  "nbf": 1735603200,
  "iat": 1735603200,
  "jti": "token-id-unique-001"
}
```

**Standard Claims Reference:**

| Claim | Full Name | Purpose | Example |
|-------|-----------|---------|---------|
| `iss` | Issuer | Identifies token issuer | `"https://auth.example.com"` |
| `sub` | Subject | Principal identifier | `"user_12345"` |
| `aud` | Audience | Intended recipients | `["api", "mobile"]` |
| `exp` | Expiration Time | Unix timestamp | `1735689600` |
| `nbf` | Not Before | Valid from timestamp | `1735603200` |
| `iat` | Issued At | Creation timestamp | `1735603200` |
| `jti` | JWT ID | Unique token identifier | `"uuid-v4"` |

**Public Claims (IANA Registry):**

| Claim | Description | Use Case |
|-------|-------------|----------|
| `name` | Display name | User profile |
| `email` | Email address | Identity |
| `email_verified` | Email verification status | Trust level |
| `preferred_username` | Username | Login identification |
| `groups` | Group memberships | RBAC |
| `roles` | Role assignments | Authorization |
| `permissions` | Specific permissions | Fine-grained access |
| `scope` | OAuth 2.0 scopes | Delegated access |
| `client_id` | OAuth client identifier | Application identification |
| `tenant_id` | Multi-tenancy identifier | Isolation |

**Custom Claims for Tokn:**

```json
{
  "tokn_ver": "1.0",
  "tokn_module": "auth-service",
  "tokn_tier": "premium",
  "tokn_rate_limit": 10000,
  "tokn_features": ["audit", "analytics", "exports"],
  "tokn_metadata": {
    "region": "us-east-1",
    "cluster": "prod-blue"
  }
}
```

### 1.2 JWT Signing Algorithms

#### Symmetric Algorithms (HMAC)

**HS256 (HMAC with SHA-256)**

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct Hs256Signer {
    secret: Vec<u8>,
}

impl Hs256Signer {
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JwtError> {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .map_err(|_| JwtError::InvalidSecret)?;
        mac.update(message);
        
        let result = mac.finalize();
        Ok(result.into_bytes().to_vec())
    }
    
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, JwtError> {
        let computed = self.sign(message)?;
        Ok(compute_constant_time_eq(&computed, signature))
    }
}

/// Constant-time comparison to prevent timing attacks
fn compute_constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
```

**HS256 Security Considerations:**

```
┌─────────────────────────────────────────────────────────────────┐
│ HS256 Security Analysis                                          │
├─────────────────────────────────────────────────────────────────┤
│ • Key Strength: Minimum 256 bits (32 bytes)                    │
│ • Key Management: Single shared secret                           │
│ • Use Case: Microservices (same trust boundary)                  │
│ • Vulnerabilities:                                               │
│   - None algorithm bypass                                        │
│   - Weak secrets (brute force)                                   │
│   - Key leakage (all tokens compromised)                       │
│ • Best Practice: Rotate keys quarterly                           │
└─────────────────────────────────────────────────────────────────┘
```

#### Asymmetric Algorithms (RSA)

**RS256 (RSA with SHA-256)**

```rust
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use sha2::{Sha256, Digest};

pub struct Rs256Signer {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl Rs256Signer {
    pub fn generate_key_pair(bits: usize) -> Result<(Vec<u8>, Vec<u8>), JwtError> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|_| JwtError::KeyGenerationFailed)?;
        let public_key = RsaPublicKey::from(&private_key);
        
        let private_pem = private_key.to_pkcs8_pem()
            .map_err(|_| JwtError::KeyEncodingFailed)?;
        let public_pem = public_key.to_public_key_pem()
            .map_err(|_| JwtError::KeyEncodingFailed)?;
        
        Ok((private_pem.as_bytes().to_vec(), public_pem.as_bytes().to_vec()))
    }
    
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JwtError> {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
        self.private_key.sign(padding, &hash)
            .map_err(|_| JwtError::SigningFailed)
    }
    
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, JwtError> {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
        self.public_key.verify(padding, &hash, signature)
            .map(|_| true)
            .map_err(|_| JwtError::VerificationFailed)
    }
}
```

**RSA Key Size Recommendations:**

| Security Level | Key Size | Use Case | Performance |
|---------------|----------|----------|-------------|
| Legacy | 2048 bits | Compatibility only | Fast |
| Standard | 3072 bits | General purpose | Moderate |
| High | 4096 bits | Financial, healthcare | Slower |
| Future-proof | 8192 bits | Long-term secrets | Very slow |

#### Elliptic Curve Algorithms (ECDSA)

**ES256 (ECDSA with P-256 and SHA-256)**

```rust
use p256::{ecdsa::{SigningKey, VerifyKey, Signature, signature::Signer}};
use p256::elliptic_curve::rand_core::OsRng;

pub struct Es256Signer {
    signing_key: SigningKey,
    verify_key: VerifyKey,
}

impl Es256Signer {
    pub fn generate_key_pair() -> Result<(Vec<u8>, Vec<u8>), JwtError> {
        let signing_key = SigningKey::random(&mut OsRng);
        let verify_key = VerifyKey::from(&signing_key);
        
        let private_bytes = signing_key.to_bytes().to_vec();
        let public_bytes = verify_key.to_sec1_bytes().to_vec();
        
        Ok((private_bytes, public_bytes))
    }
    
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JwtError> {
        let signature: Signature = self.signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }
    
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, JwtError> {
        let signature = Signature::from_bytes(signature)
            .map_err(|_| JwtError::InvalidSignatureFormat)?;
        
        self.verify_key.verify(message, &signature)
            .map(|_| true)
            .map_err(|_| JwtError::VerificationFailed)
    }
}
```

**ECDSA Curve Comparison:**

| Algorithm | Curve | Key Size | Signature Size | Security Level |
|-----------|-------|----------|----------------|----------------|
| ES256 | P-256 (secp256r1) | 256 bits | 64 bytes | ~128 bits |
| ES256K | secp256k1 | 256 bits | 64 bytes | ~128 bits |
| ES384 | P-384 (secp384r1) | 384 bits | 96 bytes | ~192 bits |
| ES512 | P-521 (secp521r1) | 521 bits | 132 bytes | ~256 bits |
| EdDSA | Ed25519 | 256 bits | 64 bytes | ~128 bits |

**EdDSA (Ed25519) - Modern Preference:**

```rust
use ed25519_dalek::{SigningKey, VerifyKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub struct Ed25519Signer {
    signing_key: SigningKey,
    verify_key: VerifyKey,
}

impl Ed25519Signer {
    pub fn generate_key_pair() -> Result<(Vec<u8>, Vec<u8>), JwtError> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verify_key = VerifyKey::from(&signing_key);
        
        let private_bytes = signing_key.to_bytes().to_vec();
        let public_bytes = verify_key.to_bytes().to_vec();
        
        Ok((private_bytes, public_bytes))
    }
    
    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JwtError> {
        let signature = self.signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }
    
    pub fn verify(&self, message: &[u8], signature_bytes: &[u8]) -> Result<bool, JwtError> {
        let signature = Signature::from_bytes(signature_bytes)
            .map_err(|_| JwtError::InvalidSignatureFormat)?;
        
        self.verify_key.verify(message, &signature)
            .map(|_| true)
            .map_err(|_| JwtError::VerificationFailed)
    }
}
```

**Ed25519 Advantages:**
- Fast signing and verification
- Small signatures (64 bytes)
- Small keys (32 bytes private, 32 bytes public)
- Deterministic signatures (no RNG needed during signing)
- Side-channel resistant
- No nonce-related vulnerabilities

### 1.3 JWT Security Vulnerabilities and Mitigations

#### Algorithm Confusion Attacks

```
┌─────────────────────────────────────────────────────────────────┐
│ Algorithm Confusion Attack                                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Attacker has access to public key (RS256)                      │
│  ↓                                                                │
│  Attacker modifies header: "alg": "HS256"                       │
│  ↓                                                                │
│  Server uses public key as HMAC secret                            │
│  ↓                                                                │
│  Attacker signs with HS256 using public key                      │
│  ↓                                                                │
│  Server accepts forged token                                      │
│                                                                   │
├─────────────────────────────────────────────────────────────────┤
│ MITIGATION:                                                       │
│ • Whitelist allowed algorithms                                    │
│ • Never use asymmetric public key for symmetric verification      │
│ • Separate key types by algorithm                                 │
│ • Validate algorithm matches key type                             │
└─────────────────────────────────────────────────────────────────┘
```

**Rust Implementation - Algorithm Whitelisting:**

```rust
use std::collections::HashSet;

pub struct JwtValidator {
    allowed_algorithms: HashSet<String>,
    algorithm_key_map: HashMap<String, KeyType>,
}

impl JwtValidator {
    pub fn new() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("RS256".to_string());
        allowed.insert("RS384".to_string());
        allowed.insert("ES256".to_string());
        allowed.insert("EdDSA".to_string());
        // Explicitly exclude: "none", "HS256" when using asymmetric
        
        Self {
            allowed_algorithms: allowed,
            algorithm_key_map: Self::build_key_map(),
        }
    }
    
    fn build_key_map() -> HashMap<String, KeyType> {
        let mut map = HashMap::new();
        map.insert("HS256".to_string(), KeyType::Symmetric);
        map.insert("HS384".to_string(), KeyType::Symmetric);
        map.insert("HS512".to_string(), KeyType::Symmetric);
        map.insert("RS256".to_string(), KeyType::AsymmetricRSA);
        map.insert("RS384".to_string(), KeyType::AsymmetricRSA);
        map.insert("RS512".to_string(), KeyType::AsymmetricRSA);
        map.insert("ES256".to_string(), KeyType::AsymmetricEC);
        map.insert("ES384".to_string(), KeyType::AsymmetricEC);
        map.insert("ES512".to_string(), KeyType::AsymmetricEC);
        map.insert("EdDSA".to_string(), KeyType::AsymmetricEdwards);
        map
    }
    
    pub fn validate_algorithm(&self, algorithm: &str, key_type: &KeyType) -> Result<(), JwtError> {
        if !self.allowed_algorithms.contains(algorithm) {
            return Err(JwtError::AlgorithmNotAllowed(algorithm.to_string()));
        }
        
        let expected_key_type = self.algorithm_key_map.get(algorithm)
            .ok_or(JwtError::UnknownAlgorithm(algorithm.to_string()))?;
        
        if expected_key_type != key_type {
            return Err(JwtError::AlgorithmKeyMismatch {
                algorithm: algorithm.to_string(),
                expected: expected_key_type.clone(),
                actual: key_type.clone(),
            });
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    Symmetric,
    AsymmetricRSA,
    AsymmetricEC,
    AsymmetricEdwards,
}
```

#### None Algorithm Attack

```rust
pub struct NoneAlgorithmFilter;

impl NoneAlgorithmFilter {
    /// Rejects tokens with "none" algorithm
    pub fn validate_header(header: &JwtHeader) -> Result<(), JwtError> {
        let alg = header.alg.to_lowercase();
        
        if alg == "none" || alg.is_empty() {
            return Err(JwtError::NoneAlgorithmNotAllowed);
        }
        
        // Also reject case variations that might bypass checks
        let normalized = alg.replace(|c: char| c.is_uppercase(), "none");
        if normalized == "none" {
            return Err(JwtError::NoneAlgorithmNotAllowed);
        }
        
        Ok(())
    }
}
```

#### Key Injection Attacks via JKU/JWK Headers

```rust
use url::Url;

pub struct HeaderSecurityValidator {
    trusted_jku_domains: HashSet<String>,
    allow_jku: bool,
    allow_jwk: bool,
    allow_x5c: bool,
}

impl HeaderSecurityValidator {
    pub fn validate(&self, header: &JwtHeader) -> Result<(), JwtError> {
        // Reject embedded JWK unless explicitly allowed
        if header.jwk.is_some() && !self.allow_jwk {
            return Err(JwtError::EmbeddedJwkNotAllowed);
        }
        
        // Validate JKU URL if present
        if let Some(ref jku) = header.jku {
            if !self.allow_jku {
                return Err(JwtError::JkuHeaderNotAllowed);
            }
            
            self.validate_jku_url(jku)?;
        }
        
        // Validate certificate chain if present
        if header.x5c.is_some() && !self.allow_x5c {
            return Err(JwtError::X5cHeaderNotAllowed);
        }
        
        Ok(())
    }
    
    fn validate_jku_url(&self, jku: &str) -> Result<(), JwtError> {
        let url = Url::parse(jku)
            .map_err(|_| JwtError::InvalidJkuUrl)?;
        
        // Must use HTTPS
        if url.scheme() != "https" {
            return Err(JwtError::JkuMustUseHttps);
        }
        
        // Check trusted domains
        let domain = url.host_str()
            .ok_or(JwtError::JkuMissingHost)?;
        
        if !self.trusted_jku_domains.contains(domain) {
            return Err(JwtError::JkuDomainNotTrusted(domain.to_string()));
        }
        
        // Reject URLs with authentication info
        if !url.username().is_empty() || url.password().is_some() {
            return Err(JwtError::JkuContainsCredentials);
        }
        
        // Reject fragment and query parameters (potential injection)
        if url.fragment().is_some() {
            return Err(JwtError::JkuContainsFragment);
        }
        
        Ok(())
    }
}
```

### 1.4 JWT Best Practices

#### Secure Token Generation

```rust
use uuid::Uuid;
use chrono::{Utc, Duration};

pub struct JwtTokenBuilder {
    claims: HashMap<String, Value>,
    headers: HashMap<String, Value>,
    config: TokenConfig,
}

#[derive(Clone, Debug)]
pub struct TokenConfig {
    pub issuer: String,
    pub default_ttl_seconds: i64,
    pub max_ttl_seconds: i64,
    pub require_jti: bool,
    pub allowed_audiences: Vec<String>,
}

impl JwtTokenBuilder {
    pub fn new(config: TokenConfig) -> Self {
        let mut headers = HashMap::new();
        headers.insert("typ".to_string(), json!("JWT"));
        
        Self {
            claims: HashMap::new(),
            headers,
            config,
        }
    }
    
    pub fn for_subject(mut self, subject: impl Into<String>) -> Self {
        self.claims.insert("sub".to_string(), json!(subject.into()));
        self
    }
    
    pub fn with_audience(mut self, audience: impl Into<String>) -> Self {
        let aud = audience.into();
        if !self.config.allowed_audiences.contains(&aud) {
            panic!("Audience '{}' not in allowed list", aud);
        }
        self.claims.insert("aud".to_string(), json!(aud));
        self
    }
    
    pub fn with_custom_claim(mut self, key: impl Into<String>, value: Value) -> Self {
        self.claims.insert(key.into(), value);
        self
    }
    
    pub fn with_ttl_seconds(mut self, ttl: i64) -> Result<Self, JwtError> {
        if ttl > self.config.max_ttl_seconds {
            return Err(JwtError::TtlTooLong {
                requested: ttl,
                maximum: self.config.max_ttl_seconds,
            });
        }
        
        let now = Utc::now();
        let exp = (now + Duration::seconds(ttl)).timestamp();
        let iat = now.timestamp();
        let nbf = now.timestamp();
        
        self.claims.insert("exp".to_string(), json!(exp));
        self.claims.insert("iat".to_string(), json!(iat));
        self.claims.insert("nbf".to_string(), json!(nbf));
        
        Ok(self)
    }
    
    pub fn build(self, signer: &dyn TokenSigner) -> Result<String, JwtError> {
        // Generate unique token ID
        if self.config.require_jti {
            let jti = Uuid::new_v4().to_string();
            let mut claims = self.claims.clone();
            claims.insert("jti".to_string(), json!(jti));
        }
        
        // Set issuer
        let mut claims = self.claims.clone();
        claims.insert("iss".to_string(), json!(self.config.issuer.clone()));
        
        // Ensure required claims are present
        self.validate_required_claims(&claims)?;
        
        // Encode and sign
        let header_json = serde_json::to_vec(&self.headers)?;
        let claims_json = serde_json::to_vec(&claims)?;
        
        let header_b64 = base64_url_encode(&header_json);
        let claims_b64 = base64_url_encode(&claims_json);
        
        let message = format!("{}.{}", header_b64, claims_b64);
        let signature = signer.sign(message.as_bytes())?;
        let signature_b64 = base64_url_encode(&signature);
        
        Ok(format!("{}.{}.{}", header_b64, claims_b64, signature_b64))
    }
    
    fn validate_required_claims(&self, claims: &HashMap<String, Value>) -> Result<(), JwtError> {
        let required = ["sub", "iss", "aud", "exp", "iat"];
        
        for claim in &required {
            if !claims.contains_key(*claim) {
                return Err(JwtError::MissingRequiredClaim(claim.to_string()));
            }
        }
        
        Ok(())
    }
}

fn base64_url_encode(data: &[u8]) -> String {
    base64::encode_config(data, base64::URL_SAFE_NO_PAD)
}
```

#### Token Validation Pipeline

```rust
pub struct TokenValidator {
    config: ValidationConfig,
    key_resolver: Arc<dyn KeyResolver>,
    revocation_checker: Arc<dyn RevocationChecker>,
}

#[derive(Clone)]
pub struct ValidationConfig {
    pub allowed_issuers: Vec<String>,
    pub allowed_audiences: Vec<String>,
    pub clock_skew_seconds: i64,
    pub require_exp: bool,
    pub require_iat: bool,
    pub max_age_seconds: Option<i64>,
}

impl TokenValidator {
    pub async fn validate(&self, token: &str) -> Result<ValidatedToken, JwtError> {
        // 1. Parse structure
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::InvalidTokenStructure);
        }
        
        // 2. Decode header
        let header_bytes = base64_url_decode(parts[0])?;
        let header: JwtHeader = serde_json::from_slice(&header_bytes)?;
        
        // 3. Security checks on header
        self.validate_header_security(&header)?;
        
        // 4. Decode claims
        let claims_bytes = base64_url_decode(parts[1])?;
        let claims: HashMap<String, Value> = serde_json::from_slice(&claims_bytes)?;
        
        // 5. Resolve signing key
        let key = self.key_resolver.resolve(&header).await?;
        
        // 6. Verify signature
        let message = format!("{}.{}", parts[0], parts[1]);
        let signature = base64_url_decode(parts[2])?;
        
        if !key.verify(message.as_bytes(), &signature, &header.alg)? {
            return Err(JwtError::InvalidSignature);
        }
        
        // 7. Validate time-based claims
        self.validate_time_claims(&claims).await?;
        
        // 8. Validate issuer
        self.validate_issuer(&claims)?;
        
        // 9. Validate audience
        self.validate_audience(&claims)?;
        
        // 10. Check revocation
        if let Some(ref jti) = claims.get("jti").and_then(|v| v.as_str()) {
            if self.revocation_checker.is_revoked(jti).await? {
                return Err(JwtError::TokenRevoked);
            }
        }
        
        // 11. Return validated token
        Ok(ValidatedToken {
            header,
            claims,
            original_token: token.to_string(),
        })
    }
    
    async fn validate_time_claims(&self, claims: &HashMap<String, Value>) -> Result<(), JwtError> {
        let now = Utc::now().timestamp();
        let skew = self.config.clock_skew_seconds;
        
        // Check expiration
        if self.config.require_exp {
            let exp = claims.get("exp")
                .and_then(|v| v.as_i64())
                .ok_or(JwtError::MissingExpiration)?;
            
            if now > exp + skew {
                return Err(JwtError::TokenExpired);
            }
        }
        
        // Check not before
        if let Some(nbf) = claims.get("nbf").and_then(|v| v.as_i64()) {
            if now < nbf - skew {
                return Err(JwtError::TokenNotYetValid);
            }
        }
        
        // Check issued at
        if self.config.require_iat {
            let iat = claims.get("iat")
                .and_then(|v| v.as_i64())
                .ok_or(JwtError::MissingIssuedAt)?;
            
            if iat > now + skew {
                return Err(JwtError::IssuedAtInFuture);
            }
            
            // Check max age
            if let Some(max_age) = self.config.max_age_seconds {
                if now - iat > max_age + skew {
                    return Err(JwtError::TokenTooOld);
                }
            }
        }
        
        Ok(())
    }
}
```

---

## OAuth 2.0 and OpenID Connect

### 2.1 OAuth 2.0 Grant Types

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ OAuth 2.0 Grant Type Selection Flow                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────┐                                                        │
│  │ Client Type?   │                                                        │
│  └────────┬────────┘                                                        │
│           │                                                                  │
│     ┌─────┴─────┬───────────────┐                                            │
│     │           │               │                                            │
│     ▼           ▼               ▼                                            │
│ ┌───────┐ ┌──────────┐ ┌──────────────┐                                    │
│ │Public │ │Confidential│ │ Backend Only  │                                    │
│ │ (SPA) │ │ (Web App) │ │ (Service)    │                                    │
│ └───┬───┘ └────┬─────┘ └───────┬──────┘                                    │
│     │          │               │                                            │
│     ▼          ▼               ▼                                            │
│ ┌──────────┐ ┌──────────┐ ┌──────────────┐                                │
│ │Auth Code │ │Auth Code │ │Client Credentials│                            │
│ │+ PKCE    │ │+ Refresh │ │                  │                            │
│ └──────────┘ └──────────┘ └──────────────┘                                │
│                                                                              │
│  Special Cases:                                                              │
│  • Authorization Code: User delegation to 3rd party                        │
│  • Device Code: Input-constrained devices (TVs, IoT)                         │
│  • Implicit: DEPRECATED - use Auth Code + PKCE                              │
│  • Password: DEPRECATED - use Auth Code + PKCE                              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Authorization Code Flow with PKCE

```rust
pub struct PkceFlow {
    code_verifier: String,
    code_challenge: String,
    code_challenge_method: String,
}

impl PkceFlow {
    /// Generate PKCE parameters for secure public clients
    pub fn generate() -> Self {
        // Generate 128 bytes of random data (96 characters base64)
        let verifier_bytes: Vec<u8> = (0..128)
            .map(|_| rand::random::<u8>())
            .collect();
        
        let code_verifier = base64_url_encode(&verifier_bytes);
        
        // Create SHA256 hash of verifier
        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let challenge_hash = hasher.finalize();
        
        let code_challenge = base64_url_encode(&challenge_hash);
        
        Self {
            code_verifier,
            code_challenge,
            code_challenge_method: "S256".to_string(),
        }
    }
    
    /// Returns parameters for authorization request
    pub fn get_auth_params(&self, client_id: &str, redirect_uri: &str, state: &str) -> AuthParams {
        AuthParams {
            response_type: "code".to_string(),
            client_id: client_id.to_string(),
            redirect_uri: redirect_uri.to_string(),
            code_challenge: self.code_challenge.clone(),
            code_challenge_method: self.code_challenge_method.clone(),
            state: state.to_string(),
            scope: "openid profile".to_string(),
        }
    }
    
    /// Exchange authorization code for tokens
    pub async fn exchange_code(
        &self,
        code: &str,
        token_endpoint: &str,
        client_id: &str,
    ) -> Result<TokenResponse, OAuthError> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", /* redirect_uri */),
            ("client_id", client_id),
            ("code_verifier", &self.code_verifier),
        ];
        
        // POST to token endpoint
        let client = reqwest::Client::new();
        let response = client
            .post(token_endpoint)
            .form(&params)
            .send()
            .await?;
        
        response.json::<TokenResponse>().await
            .map_err(|e| OAuthError::TokenExchangeFailed(e.to_string()))
    }
}

/// OAuth 2.0 Token Response
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>, // OIDC
}
```

#### Client Credentials Flow

```rust
pub struct ClientCredentialsFlow;

impl ClientCredentialsFlow {
    /// Obtain access token for server-to-server authentication
    pub async fn get_token(
        client_id: &str,
        client_secret: &str,
        token_endpoint: &str,
        scope: Option<&str>,
    ) -> Result<TokenResponse, OAuthError> {
        let mut params = vec![
            ("grant_type", "client_credentials"),
        ];
        
        if let Some(s) = scope {
            params.push(("scope", s));
        }
        
        let client = reqwest::Client::new();
        let response = client
            .post(token_endpoint)
            .basic_auth(client_id, Some(client_secret))
            .form(&params)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error = response.json::<OAuthErrorResponse>().await?;
            return Err(OAuthError::from(error));
        }
        
        response.json::<TokenResponse>().await
            .map_err(|e| OAuthError::ParseError(e.to_string()))
    }
}
```

### 2.2 OpenID Connect Flows

#### ID Token Structure

```json
{
  "iss": "https://auth.tokn.io",
  "sub": "auth0|123456789",
  "aud": "tokn-client-id",
  "exp": 1735689600,
  "iat": 1735603200,
  "auth_time": 1735603000,
  "nonce": "random-nonce-value",
  "at_hash": "access_token_hash",
  "c_hash": "authorization_code_hash",
  "amr": ["pwd", "mfa"],
  "acr": "urn:mace:incommon:iap:silver",
  "azp": "authorized-party-client-id",
  "name": "John Doe",
  "email": "john@example.com",
  "email_verified": true,
  "picture": "https://gravatar.com/avatar/...",
  "updated_at": 1735500000
}
```

**OIDC Standard Claims:**

| Category | Claims | Purpose |
|----------|--------|---------|
| Profile | `name`, `family_name`, `given_name`, `middle_name`, `nickname`, `preferred_username`, `profile`, `picture`, `website`, `gender`, `birthdate`, `zoneinfo`, `locale`, `updated_at` | User identity |
| Email | `email`, `email_verified` | Email communication |
| Phone | `phone_number`, `phone_number_verified` | SMS/Call |
| Address | `address` (JSON object) | Physical location |
| Security | `amr`, `acr`, `auth_time`, `nonce`, `at_hash`, `c_hash` | Authentication context |

### 2.3 Token Storage and Refresh

#### Refresh Token Rotation

```rust
pub struct RefreshTokenManager {
    storage: Arc<dyn TokenStorage>,
    rotation_policy: RotationPolicy,
}

#[derive(Clone)]
pub struct RotationPolicy {
    pub rotate_on_every_use: bool,
    pub reuse_detection: bool,
    pub absolute_lifetime_days: i64,
    pub inactivity_lifetime_days: i64,
}

impl RefreshTokenManager {
    /// Rotate refresh token on use (RFC 6819 recommendation)
    pub async fn rotate_token(
        &self,
        old_refresh_token: &str,
    ) -> Result<TokenPair, TokenError> {
        // 1. Validate old token exists and isn't revoked
        let token_data = self.storage.get_refresh_token(old_refresh_token).await?;
        
        if token_data.is_revoked {
            // Potential token reuse attack!
            if self.rotation_policy.reuse_detection {
                self.handle_token_reuse_attack(&token_data).await?;
            }
            return Err(TokenError::TokenRevoked);
        }
        
        // 2. Check if token is expired
        if token_data.expires_at < Utc::now() {
            return Err(TokenError::TokenExpired);
        }
        
        // 3. Generate new token pair
        let new_access_token = self.generate_access_token(&token_data.user_id, &token_data.scopes)?;
        let new_refresh_token = self.generate_refresh_token()?;
        
        // 4. Store new refresh token
        let new_token_data = RefreshTokenData {
            token: new_refresh_token.clone(),
            user_id: token_data.user_id.clone(),
            scopes: token_data.scopes.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(self.rotation_policy.inactivity_lifetime_days),
            is_revoked: false,
            parent_token: Some(old_refresh_token.to_string()),
        };
        
        self.storage.store_refresh_token(&new_token_data).await?;
        
        // 5. Revoke old token (only after successful new token storage)
        if self.rotation_policy.rotate_on_every_use {
            self.storage.revoke_refresh_token(old_refresh_token).await?;
        }
        
        Ok(TokenPair {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in: 3600,
        })
    }
    
    async fn handle_token_reuse_attack(&self, token_data: &RefreshTokenData) -> Result<(), TokenError> {
        // RFC 6819: Revoke entire family of tokens
        tracing::warn!(
            "Potential refresh token reuse detected for user {}",
            token_data.user_id
        );
        
        // Revoke the token that was just presented
        self.storage.revoke_refresh_token(&token_data.token).await?;
        
        // Revoke the entire token family (all descendants)
        if let Some(ref parent) = token_data.parent_token {
            self.revoke_token_family(parent).await?;
        }
        
        // Also revoke all child tokens
        self.revoke_token_descendants(&token_data.token).await?;
        
        // Trigger security event
        self.emit_security_event(SecurityEvent::TokenReuseDetected {
            user_id: token_data.user_id.clone(),
            token_jti: token_data.token.clone(),
            timestamp: Utc::now(),
        }).await?;
        
        Ok(())
    }
}
```

---

## API Key Management Patterns

### 3.1 API Key Formats

```
┌─────────────────────────────────────────────────────────────────┐
│ API Key Format Comparison                                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│ 1. UUID Format (Simple)                                          │
│    ak_live_550e8400-e29b-41d4-a716-446655440000                  │
│    • Pros: Simple, language-agnostic                             │
│    • Cons: No embedded metadata, requires DB lookup              │
│                                                                   │
│ 2. Prefix + Random                                               │
│    tokn_sk_abc123def456ghi789                                    │
│    • Pros: Identifiable prefix, env-specific                      │
│    • Cons: Still requires lookup                                 │
│                                                                   │
│ 3. Encoded Metadata (Self-describing)                            │
│    tokn_eyJhbGciOiJIUzI1NiIs... (JWT-style)                      │
│    • Pros: Contains claims, can verify without DB               │
│    • Cons: Larger, key rotation more complex                     │
│                                                                   │
│ 4. Hashed ID + Signature                                         │
│    tokn_a1b2c3d4.eF5g6h7i8j9 (key_id.signature)                │
│    • Pros: Tamper-evident, embeds key ID                         │
│    • Cons: Complex verification                                   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 API Key Generation

```rust
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;

pub struct ApiKeyGenerator;

impl ApiKeyGenerator {
    /// Generate cryptographically secure API key
    pub fn generate(prefix: &str, length: usize) -> String {
        let rng = SystemRandom::new();
        let mut bytes = vec![0u8; length];
        
        rng.fill(&mut bytes)
            .expect("RNG failure");
        
        let random_part = base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD);
        format!("{}_{}", prefix, random_part)
    }
    
    /// Generate with embedded key ID for fast lookup
    pub fn generate_with_key_id(prefix: &str, key_id: &str) -> String {
        let secret = Self::generate("", 32);
        format!("{}_{}_{}", prefix, key_id, secret)
    }
    
    /// Generate hierarchical/scoped key
    pub fn generate_scoped(
        prefix: &str,
        scope: &str,
        resource: &str,
    ) -> String {
        let secret = Self::generate("", 24);
        let scope_encoded = base64::encode_config(scope.as_bytes(), base64::URL_SAFE_NO_PAD);
        format!("{}_{}_{}_{}", prefix, scope_encoded, resource, secret)
    }
}

// Usage examples:
// tokn_sk_live_a1b2c3...      - Secret key (live environment)
// tokn_pk_test_x9y8z7...      - Publishable key (test environment)
// tokn_ephemeral_temp_...     - Short-lived key
```

### 3.3 API Key Storage and Hashing

```rust
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use argon2::password_hash::rand_core::OsRng;

pub struct ApiKeyStorage {
    db: Arc<dyn Database>,
    hasher: Argon2<'static>,
}

#[derive(Debug)]
pub struct StoredApiKey {
    pub key_id: String,
    pub key_hash: String,        // Argon2 hash of the key
    pub prefix: String,          // First N chars for identification
    pub scopes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub usage_count: i64,
    pub rate_limit: Option<RateLimit>,
    pub metadata: Value,
}

impl ApiKeyStorage {
    pub fn new(db: Arc<dyn Database>) -> Self {
        Self {
            db,
            hasher: Argon2::default(),
        }
    }
    
    /// Store new API key (hash the full key, return the plain key once)
    pub async fn create_key(
        &self,
        scopes: Vec<String>,
        expires_days: Option<i64>,
    ) -> Result<(String, StoredApiKey), StorageError> {
        // Generate key
        let plain_key = ApiKeyGenerator::generate("tokn_sk", 48);
        let key_id = Uuid::new_v4().to_string();
        
        // Hash the key for storage (we never store plain text)
        let salt = SaltString::generate(&mut OsRng);
        let key_hash = self.hasher
            .hash_password(plain_key.as_bytes(), &salt)
            .map_err(|e| StorageError::HashingFailed(e.to_string()))?
            .to_string();
        
        let stored = StoredApiKey {
            key_id: key_id.clone(),
            key_hash,
            prefix: plain_key.chars().take(16).collect(),
            scopes,
            created_at: Utc::now(),
            expires_at: expires_days.map(|d| Utc::now() + Duration::days(d)),
            last_used_at: None,
            usage_count: 0,
            rate_limit: None,
            metadata: json!({}),
        };
        
        self.db.insert_api_key(&stored).await?;
        
        // Return plain key (only time it's available)
        Ok((plain_key, stored))
    }
    
    /// Verify and retrieve key data
    pub async fn verify_key(&self, plain_key: &str) -> Result<StoredApiKey, StorageError> {
        // Extract key ID from key if present
        let key_id = Self::extract_key_id(plain_key);
        
        // Look up by key ID or by prefix
        let candidates = if let Some(id) = key_id {
            vec![self.db.get_api_key(&id).await?]
        } else {
            let prefix = plain_key.chars().take(16).collect();
            self.db.get_api_keys_by_prefix(&prefix).await?
        };
        
        // Verify hash against candidates
        for stored in candidates {
            let parsed_hash = PasswordHash::new(&stored.key_hash)
                .map_err(|e| StorageError::InvalidHash(e.to_string()))?;
            
            if self.hasher.verify_password(plain_key.as_bytes(), &parsed_hash).is_ok() {
                return Ok(stored);
            }
        }
        
        Err(StorageError::KeyNotFound)
    }
    
    fn extract_key_id(key: &str) -> Option<String> {
        // Format: tokn_sk_<key_id>_<secret>
        let parts: Vec<&str> = key.split('_').collect();
        if parts.len() >= 4 {
            Some(parts[2].to_string())
        } else {
            None
        }
    }
}
```

---

## Token Storage Strategies

### 4.1 Storage Architecture Comparison

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Token Storage Architecture Patterns                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ 1. Pure In-Memory                                                             │
│    ┌─────────────┐                                                          │
│    │   Memory    │ ◄── Fastest, no persistence                               │
│    │   (HashMap) │     Use: Testing, single-node                             │
│    └─────────────┘     Limit: Data loss on restart                           │
│                                                                              │
│ 2. Redis / Valkey                                                             │
│    ┌─────────────┐     ┌─────────────┐                                      │
│    │ Application │────►│    Redis    │ ◄── TTL support, pub/sub               │
│    │             │     │  (Cluster)  │     Use: Session cache, revocation     │
│    └─────────────┘     └─────────────┘     Limit: Network latency             │
│                                                                              │
│ 3. PostgreSQL + Cache                                                         │
│    ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                  │
│    │ Application │────►│    Cache    │◄───►│ PostgreSQL  │                  │
│    │             │     │   (Redis)   │     │  (Source)    │                  │
│    └─────────────┘     └─────────────┘     └─────────────┘                  │
│                        Use: Persistent tokens with fast lookup               │
│                                                                              │
│ 4. Distributed Cache (Redis Cluster)                                        │
│    ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                  │
│    │  App Node 1 │◄───►│Redis Cluster│◄───►│  App Node 2 │                  │
│    └─────────────┘     │ (Hash slots)│     └─────────────┘                  │
│                        └─────────────┘                                       │
│                        Use: Multi-region, horizontal scaling                 │
│                                                                              │
│ 5. Event Sourced + CQRS                                                       │
│    ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                  │
│    │   Command   │────►│ Event Store │────►│  Projections │                  │
│    │   Handler   │     │  (Kafka/    │     │  (Read DB)   │                  │
│    └─────────────┘     │   PgStream) │     └─────────────┘                  │
│                        └─────────────┘                                       │
│                        Use: Audit trail, time-travel queries                 │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Redis-Based Token Storage

```rust
use redis::{AsyncCommands, Client, aio::MultiplexedConnection};
use serde_json;

pub struct RedisTokenStorage {
    client: Client,
    prefix: String,
    default_ttl: usize,
}

impl RedisTokenStorage {
    pub async fn new(redis_url: &str, prefix: &str, default_ttl_secs: usize) -> Result<Self, StorageError> {
        let client = Client::open(redis_url)?;
        
        Ok(Self {
            client,
            prefix: prefix.to_string(),
            default_ttl: default_ttl_secs,
        })
    }
    
    async fn get_conn(&self) -> Result<MultiplexedConnection, StorageError> {
        self.client.get_multiplexed_async_connection().await
            .map_err(|e| StorageError::ConnectionFailed(e.to_string()))
    }
    
    fn key(&self, token_id: &str) -> String {
        format!("{}:token:{}", self.prefix, token_id)
    }
    
    fn revocation_key(&self, token_id: &str) -> String {
        format!("{}:revoked:{}", self.prefix, token_id)
    }
    
    /// Store token with TTL
    pub async fn store_token(
        &self,
        token_id: &str,
        data: &TokenData,
        ttl_seconds: Option<usize>,
    ) -> Result<(), StorageError> {
        let mut conn = self.get_conn().await?;
        let key = self.key(token_id);
        
        let ttl = ttl_seconds.unwrap_or(self.default_ttl);
        let json = serde_json::to_string(data)?;
        
        redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl)
            .arg(&json)
            .query_async(&mut conn)
            .await?;
        
        Ok(())
    }
    
    /// Retrieve token
    pub async fn get_token(&self, token_id: &str) -> Result<Option<TokenData>, StorageError> {
        let mut conn = self.get_conn().await?;
        let key = self.key(token_id);
        
        let json: Option<String> = conn.get(&key).await?;
        
        match json {
            Some(data) => {
                let token: TokenData = serde_json::from_str(&data)?;
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }
    
    /// Revoke token (add to blacklist)
    pub async fn revoke_token(
        &self,
        token_id: &str,
        expires_in_seconds: usize,
    ) -> Result<(), StorageError> {
        let mut conn = self.get_conn().await?;
        let key = self.revocation_key(token_id);
        
        // Store revocation marker with same TTL as original token would have
        conn.set_ex(key, "1", expires_in_seconds).await?;
        
        // Also publish revocation event for distributed systems
        let channel = format!("{}:revocations", self.prefix);
        let message = json!({
            "token_id": token_id,
            "revoked_at": Utc::now().to_rfc3339(),
        });
        
        conn.publish(channel, message.to_string()).await?;
        
        Ok(())
    }
    
    /// Check if token is revoked
    pub async fn is_revoked(&self, token_id: &str) -> Result<bool, StorageError> {
        let mut conn = self.get_conn().await?;
        let key = self.revocation_key(token_id);
        
        let exists: bool = conn.exists(&key).await?;
        Ok(exists)
    }
    
    /// Extend token TTL (refresh scenario)
    pub async fn extend_ttl(
        &self,
        token_id: &str,
        additional_seconds: usize,
    ) -> Result<(), StorageError> {
        let mut conn = self.get_conn().await?;
        let key = self.key(token_id);
        
        // Get current TTL
        let current_ttl: i64 = conn.ttl(&key).await?;
        
        if current_ttl > 0 {
            let new_ttl = current_ttl as usize + additional_seconds;
            conn.expire(&key, new_ttl).await?;
        }
        
        Ok(())
    }
    
    /// Bulk operations for performance
    pub async fn get_tokens_batch(
        &self,
        token_ids: &[String],
    ) -> Result<Vec<Option<TokenData>>, StorageError> {
        let mut conn = self.get_conn().await?;
        
        let keys: Vec<String> = token_ids
            .iter()
            .map(|id| self.key(id))
            .collect();
        
        let results: Vec<Option<String>> = conn.mget(&keys).await?;
        
        let tokens: Result<Vec<_>, _> = results
            .into_iter()
            .map(|opt| {
                opt.map(|json| serde_json::from_str(&json))
                    .transpose()
            })
            .collect();
        
        Ok(tokens?)
    }
}

/// Token data stored in Redis
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub jti: String,
    pub sub: String,
    pub aud: Vec<String>,
    pub scopes: Vec<String>,
    pub claims: Value,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub version: i32,
}
```

### 4.3 PostgreSQL Token Storage

```rust
use sqlx::{PgPool, Row};

pub struct PostgresTokenStorage {
    pool: PgPool,
}

#[derive(Debug)]
pub struct TokenRecord {
    pub jti: String,
    pub subject: String,
    pub audience: Vec<String>,
    pub scopes: Vec<String>,
    pub claims: Value,
    pub issued_at: DateTime<Utc>,
    pub not_before: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub refresh_token_hash: Option<String>,
}

impl PostgresTokenStorage {
    pub async fn new(database_url: &str) -> Result<Self, StorageError> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
    
    /// Insert new token
    pub async fn insert_token(&self, record: &TokenRecord) -> Result<(), StorageError> {
        sqlx::query(
            r#"
            INSERT INTO tokens (
                jti, subject, audience, scopes, claims,
                issued_at, not_before, expires_at, revoked_at, refresh_token_hash
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (jti) DO NOTHING
            "#
        )
        .bind(&record.jti)
        .bind(&record.subject)
        .bind(&record.audience)
        .bind(&record.scopes)
        .bind(&record.claims)
        .bind(record.issued_at)
        .bind(record.not_before)
        .bind(record.expires_at)
        .bind(record.revoked_at)
        .bind(&record.refresh_token_hash)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get token by JTI
    pub async fn get_token(&self, jti: &str) -> Result<Option<TokenRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT jti, subject, audience, scopes, claims,
                   issued_at, not_before, expires_at, revoked_at, refresh_token_hash
            FROM tokens
            WHERE jti = $1
            "#
        )
        .bind(jti)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(r) => Ok(Some(self.row_to_record(r))),
            None => Ok(None),
        }
    }
    
    /// Revoke token
    pub async fn revoke_token(
        &self,
        jti: &str,
        reason: Option<&str>,
    ) -> Result<bool, StorageError> {
        let result = sqlx::query(
            r#"
            UPDATE tokens
            SET revoked_at = NOW(),
                revocation_reason = $2
            WHERE jti = $1 AND revoked_at IS NULL
            "#
        )
        .bind(jti)
        .bind(reason)
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }
    
    /// Revoke all tokens for subject
    pub async fn revoke_all_for_subject(
        &self,
        subject: &str,
        reason: Option<&str>,
    ) -> Result<u64, StorageError> {
        let result = sqlx::query(
            r#"
            UPDATE tokens
            SET revoked_at = NOW(),
                revocation_reason = $2
            WHERE subject = $1 AND revoked_at IS NULL
            "#
        )
        .bind(subject)
        .bind(reason)
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected())
    }
    
    /// Get active tokens for subject
    pub async fn get_active_tokens_for_subject(
        &self,
        subject: &str,
    ) -> Result<Vec<TokenRecord>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT jti, subject, audience, scopes, claims,
                   issued_at, not_before, expires_at, revoked_at, refresh_token_hash
            FROM tokens
            WHERE subject = $1
              AND revoked_at IS NULL
              AND expires_at > NOW()
            ORDER BY issued_at DESC
            "#
        )
        .bind(subject)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| self.row_to_record(r)).collect())
    }
    
    /// Cleanup expired tokens
    pub async fn cleanup_expired(&self, batch_size: i64) -> Result<u64, StorageError> {
        let result = sqlx::query(
            r#"
            DELETE FROM tokens
            WHERE expires_at < NOW() - INTERVAL '7 days'
            LIMIT $1
            "#
        )
        .bind(batch_size)
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected())
    }
    
    fn row_to_record(&self, row: sqlx::postgres::PgRow) -> TokenRecord {
        TokenRecord {
            jti: row.get("jti"),
            subject: row.get("subject"),
            audience: row.get("audience"),
            scopes: row.get("scopes"),
            claims: row.get("claims"),
            issued_at: row.get("issued_at"),
            not_before: row.get("not_before"),
            expires_at: row.get("expires_at"),
            revoked_at: row.get("revoked_at"),
            refresh_token_hash: row.get("refresh_token_hash"),
        }
    }
}
```

---

## Revocation Strategies

### 5.1 Revocation Strategy Comparison

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Token Revocation Strategies                                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Strategy 1: Short TTL (No Revocation)                                       │
│ ─────────────────────────────────────                                       │
│ • Tokens have short lifetimes (5-15 minutes)                               │
│ • No revocation mechanism needed                                            │
│ • Refresh tokens handle session extension                                    │
│                                                                              │
│ Tradeoffs:                                                                   │
│ ✓ Simple implementation                                                      │
│ ✓ No storage overhead for revocation                                        │
│ ✓ Immediate security boundary                                                │
│ ✗ Frequent re-authentication                                                 │
│ ✗ Delayed revocation (wait for expiry)                                      │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Strategy 2: Blacklist / Deny List                                             │
│ ─────────────────────────────────────                                         │
│ • Store revoked token IDs until their expiry                                 │
│ • Check blacklist on every validation                                        │
│ • Can revoke immediately                                                     │
│                                                                              │
│ Tradeoffs:                                                                   │
│ ✓ Immediate revocation                                                       │
│ ✓ Granular control (revoke specific tokens)                                 │
│ ✗ Storage grows with number of revocations                                  │
│ ✗ Lookup overhead on every request                                          │
│ ✗ Distributed sync challenges                                               │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Strategy 3: Token Version / Rotation                                        │
│ ─────────────────────────────────────                                         │
│ • Each user has a token_version in database                                  │
│ • Token contains version number in claims                                   │
│ • Increment version to revoke all tokens                                     │
│                                                                              │
│ Tradeoffs:                                                                   │
│ ✓ Single DB write to revoke all tokens                                     │
│ ✓ No storage of individual token IDs                                        │
│ ✓ Fast validation (just compare integers)                                   │
│ ✗ Cannot revoke individual tokens                                           │
│ ✗ Requires database lookup for version                                       │
│ ✗ All tokens revoked simultaneously                                         │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Strategy 4: Bloom Filter (Probabilistic)                                    │
│ ─────────────────────────────────────                                       │
│ • Use bloom filter for space-efficient revocation check                     │
│ • False positives possible (will reject valid tokens)                      │
│ • Background full check for positives                                        │
│                                                                              │
│ Tradeoffs:                                                                   │
│ ✓ Minimal memory footprint                                                   │
│ ✓ Fast O(1) lookup                                                           │
│ ✗ False positives (configurable rate)                                       │
│ ✗ Cannot iterate revoked tokens                                             │
│ ✗ Complex tuning for false positive rate                                    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Hybrid Revocation Implementation

```rust
use bloom::{BloomFilter, ASMS};

pub struct HybridRevocationManager {
    /// Redis for active revocations (fast lookup)
    redis: RedisTokenStorage,
    
    /// PostgreSQL for audit trail
    postgres: PostgresTokenStorage,
    
    /// Bloom filter for probabilistic check (in-memory)
    bloom_filter: RwLock<BloomFilter>,
    
    /// User token version for bulk revocation
    token_versions: Arc<dyn TokenVersionStorage>,
    
    /// Configuration
    config: RevocationConfig,
}

#[derive(Clone)]
pub struct RevocationConfig {
    /// Use bloom filter for first check
    pub enable_bloom_filter: bool,
    
    /// Bloom filter false positive rate
    pub bloom_fpp: f64,
    
    /// Bloom filter expected insertions
    pub bloom_expected_insertions: usize,
    
    /// Token TTL threshold - below this, don't track revocation
    pub short_ttl_threshold_seconds: i64,
    
    /// Enable user token versioning
    pub enable_token_versioning: bool,
}

impl HybridRevocationManager {
    /// Check if token is revoked (multi-layer)
    pub async fn is_revoked(&self, token: &Token) -> Result<bool, RevocationError> {
        // Layer 1: Short TTL optimization
        // If token expires in < threshold, rely on expiry
        let ttl_remaining = token.expires_at - Utc::now();
        if ttl_remaining.num_seconds() < self.config.short_ttl_threshold_seconds {
            return Ok(false); // Not worth tracking
        }
        
        // Layer 2: Bloom filter check (probabilistic, in-memory)
        if self.config.enable_bloom_filter {
            let bloom = self.bloom_filter.read().await;
            if !bloom.contains(&token.jti) {
                // Definitely not revoked
                return Ok(false);
            }
            // Might be revoked, continue to next layer
        }
        
        // Layer 3: Token version check (bulk revocation)
        if self.config.enable_token_versioning {
            let current_version = self.token_versions
                .get_version(&token.subject)
                .await?;
            
            if let Some(token_version) = token.claims.get("tokn_ver") {
                if token_version != &json!(current_version) {
                    return Ok(true); // Version mismatch = revoked
                }
            }
        }
        
        // Layer 4: Redis check (definitive)
        let is_revoked = self.redis.is_revoked(&token.jti).await?;
        
        Ok(is_revoked)
    }
    
    /// Revoke single token
    pub async fn revoke_token(
        &self,
        jti: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), RevocationError> {
        let ttl_seconds = (expires_at - Utc::now()).num_seconds() as usize;
        
        // Add to bloom filter
        if self.config.enable_bloom_filter {
            let mut bloom = self.bloom_filter.write().await;
            bloom.insert(jti);
        }
        
        // Store in Redis
        self.redis.revoke_token(jti, ttl_seconds).await?;
        
        // Log to PostgreSQL for audit
        self.postgres.revoke_token(jti, Some("explicit_revocation")).await?;
        
        // Publish revocation event
        self.publish_revocation_event(jti).await?;
        
        Ok(())
    }
    
    /// Revoke all tokens for user (bulk)
    pub async fn revoke_all_for_user(&self, subject: &str) -> Result<u64, RevocationError> {
        if !self.config.enable_token_versioning {
            return Err(RevocationError::TokenVersioningNotEnabled);
        }
        
        // Increment token version
        let new_version = self.token_versions
            .increment_version(subject)
            .await?;
        
        // Revoke all stored tokens for user
        let revoked_count = self.postgres.revoke_all_for_subject(
            subject,
            Some("bulk_user_revocation")
        ).await?;
        
        // Clear any cached data in Redis
        self.clear_user_cache(subject).await?;
        
        tracing::info!(
            "Revoked {} tokens for user {} (new version: {})",
            revoked_count, subject, new_version
        );
        
        Ok(revoked_count)
    }
    
    /// Graceful degradation: if Redis unavailable, fall back to DB
    pub async fn is_revoked_with_fallback(
        &self,
        token: &Token,
    ) -> Result<bool, RevocationError> {
        // Try Redis first
        match self.redis.is_revoked(&token.jti).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                tracing::warn!("Redis revocation check failed: {}", e);
            }
        }
        
        // Fallback to PostgreSQL
        let record = self.postgres.get_token(&token.jti).await?;
        
        match record {
            Some(r) => Ok(r.revoked_at.is_some()),
            None => {
                // Token not in DB - either:
                // 1. Short-lived token (not stored) - check version
                // 2. Unknown token - reject
                if self.config.enable_token_versioning {
                    let version = self.token_versions.get_version(&token.subject).await?;
                    let token_version = token.claims.get("tokn_ver")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    
                    Ok(token_version != version)
                } else {
                    // Conservative: reject unknown tokens
                    Ok(true)
                }
            }
        }
    }
    
    async fn publish_revocation_event(&self, jti: &str) -> Result<(), RevocationError> {
        // Implementation depends on message broker
        // Redis pub/sub, Kafka, SNS, etc.
        Ok(())
    }
    
    async fn clear_user_cache(&self, subject: &str) -> Result<(), RevocationError> {
        // Clear any cached tokens for user
        Ok(())
    }
}
```

---

## Cryptographic Approaches

### 6.1 Key Management Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Hierarchical Key Management for Tokn                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Level 1: Master Key (HSM Protected)                                          │
│  ┌─────────────────────────┐                                                 │
│  │     Hardware HSM        │                                                 │
│  │   (AWS CloudHSM,        │                                                 │
│  │    Azure Dedicated)     │                                                 │
│  └───────────┬─────────────┘                                                 │
│              │                                                               │
│              ▼                                                               │
│  Level 2: Key Encryption Keys (KEKs)                                         │
│  ┌─────────────────────────┐                                                 │
│  │      KEK Store          │                                                 │
│  │  ┌─────┐ ┌─────┐ ┌────┐│                                                 │
│  │  │KEK-1│ │KEK-2│ │... ││  (Rotate quarterly)                            │
│  │  └─────┘ └─────┘ └────┘│                                                 │
│  └───────────┬─────────────┘                                                 │
│              │                                                               │
│              ▼                                                               │
│  Level 3: Data Encryption Keys (DEKs)                                       │
│  ┌─────────────────────────┐                                                 │
│  │      DEK per Tenant     │                                                 │
│  │  ┌─────┐ ┌─────┐ ┌────┐│                                                 │
│  │  │Tenant│ │Tenant│ │... ││                                                 │
│  │  │  A   │ │  B   │     ││                                                 │
│  │  └─────┘ └─────┘ └────┘│                                                 │
│  └───────────┬─────────────┘                                                 │
│              │                                                               │
│              ▼                                                               │
│  Level 4: Signing Keys (per service/environment)                              │
│  ┌─────────────────────────┐                                                 │
│  │    JWT Signing Keys     │                                                 │
│  │  ┌─────┐ ┌─────┐ ┌────┐│                                                 │
│  │  │Auth │ │API  │ │... ││                                                 │
│  │  │Svc  │ │Svc  │     ││                                                 │
│  │  └─────┘ └─────┘ └────┘│                                                 │
│  └─────────────────────────┘                                                 │
│                                                                              │
│  Key Rotation Schedule:                                                        │
│  • Master Key: Annual (ceremonial)                                           │
│  • KEKs: Quarterly                                                            │
│  • DEKs: On-demand (tenant request)                                          │
│  • Signing Keys: Monthly (with overlap period)                               │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 JWKS (JSON Web Key Set) Implementation

```rust
use serde::{Serialize, Deserialize};

/// JWKS endpoint response
#[derive(Debug, Serialize, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

/// JSON Web Key
#[derive(Debug, Serialize, Deserialize)]
pub struct Jwk {
    /// Key type (RSA, EC, oct)
    pub kty: String,
    
    /// Key ID for key selection
    pub kid: String,
    
    /// Key usage (sig, enc)
    pub r#use: Option<String>,
    
    /// Key operations
    pub key_ops: Option<Vec<String>>,
    
    /// Algorithm
    pub alg: Option<String>,
    
    /// X.509 URL
    pub x5u: Option<String>,
    
    /// X.509 certificate chain
    pub x5c: Option<Vec<String>>,
    
    /// X.509 SHA-1 thumbprint
    pub x5t: Option<String>,
    
    /// X.509 SHA-256 thumbprint
    #[serde(rename = "x5t#S256")]
    pub x5t_s256: Option<String>,
    
    // RSA specific
    pub n: Option<String>,  // Modulus
    pub e: Option<String>,  // Exponent
    pub d: Option<String>,  // Private exponent (never in JWKS!)
    
    // EC specific
    pub crv: Option<String>, // Curve
    pub x: Option<String>,  // X coordinate
    pub y: Option<String>,  // Y coordinate
    
    // Symmetric
    pub k: Option<String>,  // Key value (oct, never in public JWKS!)
}

pub struct JwksManager {
    /// Current JWKS
    jwks: Arc<RwLock<Jwks>>,
    
    /// HTTP client for fetching
    client: reqwest::Client,
    
    /// JWKS URL
    jwks_url: String,
    
    /// Cache TTL
    cache_duration: Duration,
    
    /// Last fetch time
    last_fetch: Arc<RwLock<Instant>>,
}

impl JwksManager {
    pub async fn get_key(&self, kid: &str) -> Result<Jwk, JwksError> {
        // Check if cache needs refresh
        {
            let last_fetch = self.last_fetch.read().await;
            if last_fetch.elapsed() > self.cache_duration {
                drop(last_fetch);
                self.refresh_jwks().await?;
            }
        }
        
        let jwks = self.jwks.read().await;
        
        jwks.keys
            .iter()
            .find(|k| k.kid == kid)
            .cloned()
            .ok_or(JwksError::KeyNotFound(kid.to_string()))
    }
    
    async fn refresh_jwks(&self) -> Result<(), JwksError> {
        let response = self.client
            .get(&self.jwks_url)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(JwksError::FetchFailed(response.status()));
        }
        
        let jwks: Jwks = response.json().await?;
        
        // Validate JWKS (no private keys)
        for key in &jwks.keys {
            if key.d.is_some() || key.k.is_some() {
                return Err(JwksError::PrivateKeyExposed(key.kid.clone()));
            }
        }
        
        // Update cache
        {
            let mut cached = self.jwks.write().await;
            *cached = jwks;
        }
        
        {
            let mut last_fetch = self.last_fetch.write().await;
            *last_fetch = Instant::now();
        }
        
        tracing::info!("JWKS refreshed successfully");
        Ok(())
    }
    
    /// Get all current key IDs
    pub async fn get_key_ids(&self) -> Vec<String> {
        let jwks = self.jwks.read().await;
        jwks.keys.iter().map(|k| k.kid.clone()).collect()
    }
}
```

### 6.3 Secure Envelope Encryption

```rust
/// Envelope encryption for sensitive token data
pub struct EnvelopeEncryption {
    /// KEK provider (HSM or secure storage)
    kek_provider: Arc<dyn KekProvider>,
    
    /// DEK cache
    dek_cache: Arc<RwLock<HashMap<String, DataEncryptionKey>>>,
}

#[derive(Clone)]
pub struct DataEncryptionKey {
    pub id: String,
    pub key: Vec<u8>,
    pub algorithm: String,
    pub created_at: DateTime<Utc>,
}

impl EnvelopeEncryption {
    /// Encrypt data using envelope encryption
    pub async fn encrypt(
        &self,
        tenant_id: &str,
        plaintext: &[u8],
    ) -> Result<EncryptedEnvelope, EncryptionError> {
        // 1. Get or create DEK for tenant
        let dek = self.get_or_create_dek(tenant_id).await?;
        
        // 2. Encrypt data with DEK (AES-256-GCM)
        let encrypted_data = self.encrypt_with_dek(plaintext, &dek)?;
        
        // 3. Encrypt DEK with KEK
        let encrypted_dek = self.kek_provider.encrypt_dek(&dek).await?;
        
        Ok(EncryptedEnvelope {
            dek_id: dek.id,
            encrypted_dek,
            encrypted_data,
            algorithm: "AES-256-GCM".to_string(),
        })
    }
    
    /// Decrypt envelope
    pub async fn decrypt(
        &self,
        envelope: &EncryptedEnvelope,
    ) -> Result<Vec<u8>, EncryptionError> {
        // 1. Decrypt DEK with KEK
        let dek = self.kek_provider.decrypt_dek(&envelope.encrypted_dek).await?;
        
        // 2. Decrypt data with DEK
        let plaintext = self.decrypt_with_dek(&envelope.encrypted_data, &dek)?;
        
        Ok(plaintext)
    }
    
    async fn get_or_create_dek(&self, tenant_id: &str) -> Result<DataEncryptionKey, EncryptionError> {
        // Check cache first
        {
            let cache = self.dek_cache.read().await;
            if let Some(dek) = cache.get(tenant_id) {
                // Check if DEK is expired
                if dek.created_at > Utc::now() - Duration::days(90) {
                    return Ok(dek.clone());
                }
            }
        }
        
        // Create new DEK
        let mut rng = rand::thread_rng();
        let mut key_bytes = vec![0u8; 32];
        rng.fill_bytes(&mut key_bytes);
        
        let dek = DataEncryptionKey {
            id: format!("{}-{}", tenant_id, Uuid::new_v4()),
            key: key_bytes,
            algorithm: "AES-256".to_string(),
            created_at: Utc::now(),
        };
        
        // Store in cache
        {
            let mut cache = self.dek_cache.write().await;
            cache.insert(tenant_id.to_string(), dek.clone());
        }
        
        Ok(dek)
    }
    
    fn encrypt_with_dek(
        &self,
        plaintext: &[u8],
        dek: &DataEncryptionKey,
    ) -> Result<EncryptedData, EncryptionError> {
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use aes_gcm::aead::{Aead, KeyInit};
        
        let key = Key::<Aes256Gcm>::from_slice(&dek.key);
        let cipher = Aes256Gcm::new(key);
        
        // Generate random nonce
        let mut nonce_bytes = vec![0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;
        
        Ok(EncryptedData {
            ciphertext,
            nonce: nonce_bytes,
        })
    }
}

pub struct EncryptedEnvelope {
    pub dek_id: String,
    pub encrypted_dek: Vec<u8>,
    pub encrypted_data: EncryptedData,
    pub algorithm: String,
}

pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}
```

---

## Token Format Standards Comparison

### 7.1 JWT vs PASETO vs Branca

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Token Format Comparison Matrix                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                    │    JWT      │   PASETO    │   Branca    │              │
│ ───────────────────┼─────────────┼─────────────┼─────────────┤              │
│ Standard           │ RFC 7519    │ v3/v4 specs │ libsodium   │              │
│ Maturity           │ L5 (10+ yrs)│ L3 (5 yrs)  │ L3 (5 yrs)  │              │
│ Library Support    │ Excellent   │ Good        │ Moderate    │              │
│ Token Size         │ Variable    │ ~2x JWT     │ ~1.5x JWT   │              │
│ Algorithms         │ Configurable│ Fixed       │ Fixed (XCha│              │
│                    │             │             │ Cha20-Poly) │              │
│ Encryption Support │ JWE (rarely)│ Built-in    │ Built-in    │              │
│                    │ used)       │             │             │              │
│ Key Rotation       │ JWKS        │ v4 supports │ Manual      │              │
│                    │             │             │             │              │
│ Interoperability   │ Universal   │ Modern only │ Limited     │              │
│                  │             │             │             │              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Security Comparison:                                                         │
│                                                                              │
│ 1. JWT Weaknesses:                                                           │
│    • Algorithm confusion (alg=none, alg=HS256 with RSA key)                 │
│    • Complex parser requirements → parser bugs                               │
│    • JWE often skipped (complexity)                                        │
│    • Zoo of algorithms (some weak)                                           │
│                                                                              │
│ 2. PASETO Improvements:                                                      │
│    • Version + purpose prefix prevents confusion                           │
│    • Fixed algorithms per version                                          │
│    • libsodium-based (modern crypto)                                       │
│    • PASETO v3: NIST curves (P-384)                                        │
│    • PASETO v4: Ed25519 + XChaCha20-Poly1305                               │
│    • Built-in encrypted tokens (local)                                     │
│                                                                              │
│ 3. Branca Characteristics:                                                   │
│    • Single format (no versions)                                           │
│    • Always encrypted                                                        │
│    • 220-byte tokens                                                         │
│    • Timestamp included                                                      │
│    • Simple implementation                                                   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 7.2 PASETO Deep Dive

```rust
/// PASETO implementation for Tokn
pub mod paseto {
    use pasetors::version4::{V4, LocalToken, PublicToken};
    use pasetors::claims::Claims;
    use pasetors::keys::{SymmetricKey, AsymmetricSecretKey, AsymmetricPublicKey};
    
    pub struct PasetoV4;
    
    impl PasetoV4 {
        /// Generate local token (encrypted)
        /// Format: v4.local.{encrypted_payload}
        pub fn encrypt_local(
            key: &SymmetricKey,
            claims: &Claims,
        ) -> Result<String, PasetoError> {
            let token = LocalToken::encrypt(key, claims, None, None)
                .map_err(|e| PasetoError::EncryptionFailed(e.to_string()))?;
            Ok(token)
        }
        
        /// Decrypt local token
        pub fn decrypt_local(
            key: &SymmetricKey,
            token: &str,
        ) -> Result<Claims, PasetoError> {
            let claims = LocalToken::decrypt(key, token, None)
                .map_err(|e| PasetoError::DecryptionFailed(e.to_string()))?;
            Ok(claims)
        }
        
        /// Generate public token (signed)
        /// Format: v4.public.{signed_payload}
        pub fn sign_public(
            secret_key: &AsymmetricSecretKey,
            claims: &Claims,
        ) -> Result<String, PasetoError> {
            let token = PublicToken::sign(secret_key, claims, None, None)
                .map_err(|e| PasetoError::SigningFailed(e.to_string()))?;
            Ok(token)
        }
        
        /// Verify public token
        pub fn verify_public(
            public_key: &AsymmetricPublicKey,
            token: &str,
        ) -> Result<Claims, PasetoError> {
            let claims = PublicToken::verify(public_key, token, None, None)
                .map_err(|e| PasetoError::VerificationFailed(e.to_string()))?;
            Ok(claims)
        }
    }
    
    /// PASETO claims builder
    pub struct PasetoClaimsBuilder {
        claims: Claims,
    }
    
    impl PasetoClaimsBuilder {
        pub fn new() -> Self {
            let mut claims = Claims::new().unwrap();
            // PASETO enforces exp claim
            claims.expiration(
                (Utc::now() + Duration::hours(1)).to_rfc3339()
            ).unwrap();
            Self { claims }
        }
        
        pub fn subject(mut self, subject: &str) -> Self {
            self.claims.subject(subject).unwrap();
            self
        }
        
        pub fn audience(mut self, audience: &str) -> Self {
            self.claims.audience(audience).unwrap();
            self
        }
        
        pub fn with_custom_claim(mut self, key: &str, value: impl Into<Value>) -> Self {
            let json = serde_json::to_string(&value.into()).unwrap();
            self.claims.add_additional(key, &json).unwrap();
            self
        }
        
        pub fn build(self) -> Claims {
            self.claims
        }
    }
}
```

### 7.3 Token Format Selection Matrix for Tokn

```rust
/// Token format selector based on requirements
pub struct TokenFormatSelector;

#[derive(Debug, Clone)]
pub enum TokenFormat {
    /// Standard JWT (wide compatibility)
    Jwt { algorithm: JwtAlgorithm },
    
    /// PASETO v4 (modern, secure)
    PasetoV4 { purpose: PasetoPurpose },
    
    /// Branca (encrypted by default)
    Branca,
    
    /// Custom compact format
    Compact,
}

#[derive(Debug, Clone)]
pub enum JwtAlgorithm {
    Ed25519,    // Recommended
    Es256,      // ECDSA
    Rs256,      // RSA
}

#[derive(Debug, Clone)]
pub enum PasetoPurpose {
    Local,      // Encrypted
    Public,     // Signed
}

impl TokenFormatSelector {
    /// Select optimal token format based on requirements
    pub fn select(requirements: TokenRequirements) -> TokenFormat {
        match requirements {
            // Need third-party verification (external services)
            TokenRequirements {
                external_verification: true,
                encryption_needed: false,
                .. 
            } => TokenFormat::Jwt { algorithm: JwtAlgorithm::Ed25519 },
            
            // Internal service-to-service (modern stack)
            TokenRequirements {
                external_verification: false,
                encryption_needed: false,
                modern_stack: true,
                ..
            } => TokenFormat::PasetoV4 { purpose: PasetoPurpose::Public },
            
            // Need encrypted tokens (sensitive claims)
            TokenRequirements {
                encryption_needed: true,
                ..
            } => TokenFormat::PasetoV4 { purpose: PasetoPurpose::Local },
            
            // Default: JWT for maximum compatibility
            _ => TokenFormat::Jwt { algorithm: JwtAlgorithm::Ed25519 },
        }
    }
}

#[derive(Debug)]
pub struct TokenRequirements {
    pub external_verification: bool,
    pub encryption_needed: bool,
    pub modern_stack: bool,
    pub size_constraints: bool,
    pub audit_requirements: bool,
}
```

---

## Architecture Patterns for Token Services

### 8.1 Microservices Token Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Tokn Microservices Architecture                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                        API Gateway                                  │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐    │    │
│  │  │  Rate    │  │  Auth    │  │  Cache   │  │   Logging/       │    │    │
│  │  │  Limit   │──►│  Check   │──►│  Layer   │──►│   Metrics        │    │    │
│  │  │          │  │  (JWKS)  │  │          │  │                  │    │    │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────────────┘    │    │
│  └────────────────────────────────────┬────────────────────────────────┘    │
│                                     │                                        │
│                    ┌────────────────┼────────────────┐                       │
│                    │                │                │                       │
│                    ▼                ▼                ▼                       │
│  ┌──────────────────────┐ ┌──────────────────┐ ┌──────────────────────┐    │
│  │   Auth Service       │ │   Token Service  │ │   User Service       │    │
│  │   ┌──────────────┐   │ │   ┌──────────┐   │ │   ┌──────────────┐   │    │
│  │   │  OAuth 2.0   │   │ │   │  Issue   │   │ │   │  Profile     │   │    │
│  │   │  OIDC Flows  │   │ │   │  Tokens  │   │ │   │  Management  │   │    │
│  │   │  ┌────────┐  │   │ │   │  ┌────┐  │   │ │   │              │   │    │
│  │   │  │ Login  │  │   │ │   │  │JWT │  │   │ │   │              │   │    │
│  │   │  │ MFA    │  │───┼─┼──►│  │PASE│  │◄──┼─┼──│              │   │    │
│  │   │  │ SSO    │  │   │ │   │  │TO  │  │   │ │   │              │   │    │
│  │   │  └────────┘  │   │ │   │  └────┘  │   │ │   │              │   │    │
│  │   └──────────────┘   │ │   └──────────┘   │ │   └──────────────┘   │    │
│  └──────────────────────┘ └──────────────────┘ └──────────────────────┘    │
│           │                                              │                   │
│           ▼                                              ▼                   │
│  ┌──────────────────────┐                    ┌──────────────────────┐        │
│  │   Identity Provider  │                    │   Session Store      │        │
│  │   (IdP) Integration  │                    │   (Redis Cluster)    │        │
│  │   • Auth0            │                    │                      │        │
│  │   • Okta             │                    │   ┌────────┐          │        │
│  │   • Keycloak         │                    │   │ Active │          │        │
│  │   • Azure AD         │                    │   │Tokens  │          │        │
│  │                      │                    │   └────────┘          │        │
│  └──────────────────────┘                    │   ┌────────┐          │        │
│                                              │   │Revoked │          │        │
│                                              │   │Tokens  │          │        │
│                                              │   └────────┘          │        │
│                                              └──────────────────────┘        │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Background Workers                              │   │
│  │   ┌────────────┐   ┌────────────┐   ┌────────────┐                  │   │
│  │   │  Token     │   │  Revocation│   │  Analytics │                  │   │
│  │   │  Cleanup   │   │  Sync      │   │  Export    │                  │   │
│  │   └────────────┘   └────────────┘   └────────────┘                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 8.2 Token Service API Design

```rust
use axum::{
    routing::{post, get, delete},
    Router,
    Json,
    extract::{State, Path, Query},
    http::StatusCode,
};

/// Token Service Router
pub fn token_routes() -> Router<TokenServiceState> {
    Router::new()
        // Token lifecycle
        .route("/tokens", post(issue_token))
        .route("/tokens/:id/validate", post(validate_token))
        .route("/tokens/:id/revoke", post(revoke_token))
        .route("/tokens/:id", get(get_token_info))
        .route("/tokens/:id", delete(delete_token))
        
        // Bulk operations
        .route("/tokens/introspect", post(introspect_tokens))
        .route("/tokens/revoke", post(bulk_revoke))
        
        // JWKS endpoint
        .route("/.well-known/jwks.json", get(jwks_endpoint))
        .route("/.well-known/openid-configuration", get(oidc_config))
        
        // Admin operations
        .route("/admin/tokens", get(list_tokens))
        .route("/admin/tokens/cleanup", post(cleanup_expired))
}

/// Issue new token
async fn issue_token(
    State(state): State<TokenServiceState>,
    Json(request): Json<IssueTokenRequest>,
) -> Result<Json<TokenResponse>, TokenError> {
    let token = state.service.issue_token(request).await?;
    Ok(Json(TokenResponse::from(token)))
}

/// Validate token
async fn validate_token(
    State(state): State<TokenServiceState>,
    Path(token_id): Path<String>,
    Json(request): Json<ValidateRequest>,
) -> Result<Json<ValidationResult>, TokenError> {
    let result = state.service.validate(&request.token).await?;
    Ok(Json(result))
}

/// Revoke token
async fn revoke_token(
    State(state): State<TokenServiceState>,
    Path(token_id): Path<String>,
    Json(request): Json<RevokeRequest>,
) -> Result<StatusCode, TokenError> {
    state.service.revoke(&token_id, request.reason).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// JWKS endpoint
async fn jwks_endpoint(
    State(state): State<TokenServiceState>,
) -> Result<Json<Jwks>, TokenError> {
    let jwks = state.service.get_public_jwks().await?;
    Ok(Json(jwks))
}

/// Request/Response Types
#[derive(Debug, Deserialize)]
pub struct IssueTokenRequest {
    pub subject: String,
    pub audience: Vec<String>,
    pub scopes: Vec<String>,
    pub ttl_seconds: Option<i64>,
    pub custom_claims: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub jti: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub token: String,
    pub expected_audience: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub claims: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RevokeRequest {
    pub reason: Option<String>,
}
```

### 8.3 Event-Driven Token Lifecycle

```rust
use async_trait::async_trait;

/// Token event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenEvent {
    TokenIssued {
        jti: String,
        subject: String,
        issued_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    },
    TokenValidated {
        jti: String,
        subject: String,
        validated_at: DateTime<Utc>,
        success: bool,
    },
    TokenRevoked {
        jti: String,
        subject: String,
        revoked_at: DateTime<Utc>,
        reason: Option<String>,
    },
    TokenExpired {
        jti: String,
        subject: String,
        expired_at: DateTime<Utc>,
    },
    BulkRevocation {
        subject: String,
        revoked_count: u64,
        revoked_at: DateTime<Utc>,
    },
}

/// Event handler trait
#[async_trait]
pub trait TokenEventHandler: Send + Sync {
    async fn handle(&self, event: &TokenEvent) -> Result<(), EventError>;
}

/// Event bus for token events
pub struct TokenEventBus {
    handlers: Vec<Arc<dyn TokenEventHandler>>,
    channel: broadcast::Sender<TokenEvent>,
}

impl TokenEventBus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            handlers: Vec::new(),
            channel: tx,
        }
    }
    
    pub fn register(&mut self, handler: Arc<dyn TokenEventHandler>) {
        self.handlers.push(handler);
    }
    
    pub async fn publish(&self, event: TokenEvent) {
        // Broadcast to channel
        let _ = self.channel.send(event.clone());
        
        // Call all handlers
        for handler in &self.handlers {
            let handler = handler.clone();
            let event = event.clone();
            tokio::spawn(async move {
                if let Err(e) = handler.handle(&event).await {
                    tracing::error!("Event handler failed: {:?}", e);
                }
            });
        }
    }
}

/// Example: Audit log handler
pub struct AuditLogHandler {
    storage: Arc<dyn AuditStorage>,
}

#[async_trait]
impl TokenEventHandler for AuditLogHandler {
    async fn handle(&self, event: &TokenEvent) -> Result<(), EventError> {
        let audit_entry = AuditEntry {
            id: Uuid::new_v4().to_string(),
            event_type: format!("{:?}", event),
            event_data: serde_json::to_value(event)?,
            timestamp: Utc::now(),
        };
        
        self.storage.store(audit_entry).await?;
        Ok(())
    }
}

/// Example: Metrics handler
pub struct MetricsHandler {
    metrics: Arc<MetricsRegistry>,
}

#[async_trait]
impl TokenEventHandler for MetricsHandler {
    async fn handle(&self, event: &TokenEvent) -> Result<(), EventError> {
        match event {
            TokenEvent::TokenIssued { .. } => {
                self.metrics.counter("tokens_issued_total").increment(1);
            }
            TokenEvent::TokenValidated { success, .. } => {
                self.metrics.counter("tokens_validated_total")
                    .with_label("success", &success.to_string())
                    .increment(1);
            }
            TokenEvent::TokenRevoked { .. } => {
                self.metrics.counter("tokens_revoked_total").increment(1);
            }
            _ => {}
        }
        Ok(())
    }
}
```

---

## Security Best Practices

### 9.1 Secure Token Transmission

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Token Transmission Security                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  1. Transport Layer Security                                                   │
│  ─────────────────────────────────────                                         │
│  • Always use HTTPS/TLS 1.3 for token transmission                            │
│  • Certificate pinning for mobile apps                                        │
│  • HSTS headers to prevent downgrade attacks                                  │
│                                                                              │
│  2. Token Placement                                                            │
│  ─────────────────────────────────────                                         │
│  Authorization Header (RECOMMENDED):                                         │
│    Authorization: Bearer <token>                                              │
│                                                                              │
│  Cookie (with security flags):                                               │
│    Set-Cookie: token=<token>; HttpOnly; Secure; SameSite=Strict;             │
│                                                                              │
│  AVOID (Security Risks):                                                     │
│    • URL query parameters (logged in browser history/proxy logs)              │
│    • LocalStorage (XSS vulnerable)                                          │
│    • SessionStorage (XSS vulnerable)                                        │
│                                                                              │
│  3. BFF (Backend-for-Frontend) Pattern                                        │
│  ─────────────────────────────────────                                         │
│    Browser ──► BFF Server ──► API Server                                      │
│       │           │               │                                           │
│       │           │               │                                           │
│    No token    Token in       Token in                                        │
│    storage     HttpOnly         header                                          │
│                cookie                                                         │
│                                                                              │
│  4. Token Binding                                                              │
│  ─────────────────────────────────────                                         │
│  Bind token to:                                                              │
│    • TLS channel ID (prevents export)                                        │
│    • Client certificate (mTLS)                                                │
│    • DPoP (Demonstrating Proof-of-Possession)                                │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 9.2 DPoP (Demonstrating Proof-of-Possession) Implementation

```rust
/// DPoP proof structure
#[derive(Debug, Serialize, Deserialize)]
pub struct DpopProof {
    /// DPoP JWT header
    pub header: DpopHeader,
    
    /// DPoP claims
    pub payload: DpopPayload,
    
    /// Signature
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DpopHeader {
    pub alg: String,  // "ES256"
    pub typ: String,  // "dpop+jwt"
    pub jwk: Jwk,     // Public key for verification
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DpopPayload {
    pub jti: String,  // Unique proof identifier
    pub htm: String,  // HTTP method (GET, POST, etc.)
    pub htu: String,  // HTTP URL
    pub iat: i64,     // Issued at
    pub ath: Option<String>, // Access token hash (for token binding)
}

pub struct DpopValidator;

impl DpopValidator {
    /// Validate DPoP proof for request
    pub fn validate(
        proof: &DpopProof,
        request_method: &str,
        request_url: &str,
        access_token: Option<&str>,
    ) -> Result<(), DpopError> {
        // 1. Validate typ header
        if proof.header.typ != "dpop+jwt" {
            return Err(DpopError::InvalidType);
        }
        
        // 2. Validate htm matches request method
        if proof.payload.htm != request_method {
            return Err(DpopError::MethodMismatch);
        }
        
        // 3. Validate htu matches request URL (normalize)
        let normalized_proof_htm = Self::normalize_url(&proof.payload.htu);
        let normalized_request_url = Self::normalize_url(request_url);
        
        if normalized_proof_htm != normalized_request_url {
            return Err(DpopError::UrlMismatch);
        }
        
        // 4. Validate iat (within 5 minutes)
        let now = Utc::now().timestamp();
        if (now - proof.payload.iat).abs() > 300 {
            return Err(DpopError::ProofExpired);
        }
        
        // 5. Validate signature using embedded JWK
        Self::verify_signature(proof)?;
        
        // 6. If ath present, validate access token binding
        if let Some(ref ath) = proof.payload.ath {
            let token = access_token.ok_or(DpopError::MissingAccessToken)?;
            let expected_ath = Self::hash_access_token(token);
            if ath != &expected_ath {
                return Err(DpopError::AccessTokenHashMismatch);
            }
        }
        
        Ok(())
    }
    
    fn hash_access_token(token: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let hash = hasher.finalize();
        base64::encode_config(&hash[..16], base64::URL_SAFE_NO_PAD)
    }
    
    fn normalize_url(url: &str) -> String {
        // Remove query string, fragment, normalize case
        url.split('?')
            .next()
            .unwrap()
            .split('#')
            .next()
            .unwrap()
            .to_lowercase()
    }
    
    fn verify_signature(proof: &DpopProof) -> Result<(), DpopError> {
        // Implementation depends on algorithm
        // Use embedded JWK to verify signature
        Ok(())
    }
}
```

---

## Performance Considerations

### 10.1 Token Validation Performance

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Token Validation Latency Budget (p99 < 10ms)                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│ Operation                    │ Time (μs) │ Optimization                      │
│ ─────────────────────────────┼───────────┼───────────────────────────────────│
│ Base64 decode (header)       │    5-10   │ SIMD implementations              │
│ JSON parse (header)          │   10-20   │ Small string optimization         │
│ Key lookup (in-memory)       │    1-2    │ HashMap, no allocation            │
│ Base64 decode (payload)      │   10-30   │ SIMD implementations              │
│ JSON parse (payload)         │   20-50   │ Streaming parser                  │
│ Signature verification       │  100-500  │ Pre-computed keys, batch verify   │
│ Claim validation           │    5-10   │ Zero-allocation checks            │
│ ─────────────────────────────┼───────────┼───────────────────────────────────│
│ Total (JWT)                  │ 150-620   │                                   │
│ Total (JWT + cache)          │   20-50   │ Redis/memcached hit               │
│ Total (PASETO)               │  50-200   │ Simpler format, no JWKS           │
│                                                                              │
│ Optimizations:                                                                 │
│ 1. Key caching: Keep JWKS in memory, refresh periodically                   │
│ 2. Validation caching: Cache validation results for short TTL               │
│ 3. Batch validation: Verify multiple tokens together (RSA batch verify)     │
│ 4. Connection pooling: Reuse Redis/DB connections                             │
│ 5. Async/await: Non-blocking I/O for storage lookups                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 10.2 Caching Strategy

```rust
use cached::{Cached, SizedCache, TimedCache};
use cached::proc_macro::cached;

/// Cached token validation results
#[cached(
    name = "VALIDATION_CACHE",
    type = "TimedCache<String, ValidationResult>",
    create = "TimedCache::with_lifespan_and_capacity(60, 10000)",
    key = "String",
    convert = r#"{ format!("{}:{}", token_hash(token), expected_audience.as_deref().unwrap_or("")) }"#,
    result = true
)]
pub async fn validate_token_cached(
    token: String,
    expected_audience: Option<String>,
    state: State<TokenServiceState>,
) -> Result<ValidationResult, TokenError> {
    // This is only called on cache miss
    state.service.validate(&token).await
}

fn token_hash(token: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

/// JWKS cache with background refresh
pub struct CachedJwksManager {
    inner: JwksManager,
    cache: RwLock<Option<(Jwks, Instant)>>,
    cache_ttl: Duration,
}

impl CachedJwksManager {
    pub async fn get_key(&self, kid: &str) -> Result<Jwk, JwksError> {
        // Fast path: check cache
        {
            let cache = self.cache.read().await;
            if let Some((ref jwks, timestamp)) = *cache {
                if timestamp.elapsed() < self.cache_ttl {
                    if let Some(key) = jwks.keys.iter().find(|k| k.kid == kid) {
                        return Ok(key.clone());
                    }
                }
            }
        }
        
        // Slow path: refresh
        self.refresh_and_get(kid).await
    }
    
    async fn refresh_and_get(&self, kid: &str) -> Result<Jwk, JwksError> {
        let mut cache = self.cache.write().await;
        
        // Double-check after acquiring write lock
        if let Some((ref jwks, timestamp)) = *cache {
            if timestamp.elapsed() < self.cache_ttl {
                if let Some(key) = jwks.keys.iter().find(|k| k.kid == kid) {
                    return Ok(key.clone());
                }
            }
        }
        
        // Fetch fresh JWKS
        let jwks = self.inner.fetch_jwks().await?;
        let key = jwks.keys
            .iter()
            .find(|k| k.kid == kid)
            .cloned()
            .ok_or(JwksError::KeyNotFound(kid.to_string()))?;
        
        *cache = Some((jwks, Instant::now()));
        
        Ok(key)
    }
}
```

---

## References

### Standards and RFCs

| Document | Title | Relevance |
|----------|-------|-----------|
| RFC 7519 | JSON Web Token (JWT) | Core JWT standard |
| RFC 7515 | JSON Web Signature (JWS) | Signing algorithms |
| RFC 7516 | JSON Web Encryption (JWE) | Encrypted tokens |
| RFC 7517 | JSON Web Key (JWK) | Key representation |
| RFC 7518 | JSON Web Algorithms (JWA) | Algorithm definitions |
| RFC 6749 | OAuth 2.0 Authorization Framework | OAuth flows |
| RFC 6750 | OAuth 2.0 Bearer Token Usage | Bearer token usage |
| RFC 6819 | OAuth 2.0 Threat Model | Security considerations |
| RFC 7636 | PKCE for OAuth 2.0 | Mobile/SPA security |
| RFC 7009 | OAuth 2.0 Token Revocation | Revocation endpoint |
| RFC 8725 | JWT Best Current Practices | Security best practices |
| RFC 9449 | DPoP | Proof-of-possession |
| OpenID Connect Core 1.0 | OIDC specification | Identity layer |
| PASETO v4 | Platform-Agnostic Security Tokens | Modern alternative |

### Academic Papers

1. **"An Empirical Study of OAuth SSO Systems"** (Wang et al., 2013)
   - Analysis of OAuth vulnerabilities in real systems
   - Application: Security testing methodology

2. **"The Web Never Forgets"** (Acar et al., 2014)
   - Persistent tracking via token mechanisms
   - Application: Privacy considerations

3. **"OAuth 2.0 in the Wild"** (Li et al., 2016)
   - Large-scale OAuth implementation analysis
   - Application: Common pitfalls to avoid

### Implementation References

| Library | Language | Features |
|---------|----------|----------|
| jsonwebtoken | Rust | JWT encode/decode |
| pasetors | Rust | PASETO v3/v4 |
| ring | Rust | Cryptographic primitives |
| rustls | Rust | TLS implementation |
| JOSE-JWT | Java | Enterprise JWT |
| PyJWT | Python | Python JWT standard |
| jose | Go | Go JWT library |

### Security Resources

1. **OWASP JWT Cheat Sheet** - https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html
2. **Auth0 JWT Handbook** - https://auth0.com/resources/ebooks/jwt-handbook
3. **IETF JOSE Working Group** - https://datatracker.ietf.org/wg/jose/documents/
4. **OAuth.net** - https://oauth.net/

---

## Document Metadata

- **Version:** 1.0.0
- **Last Updated:** 2026-04-02
- **Author:** Tokn Research Team
- **Review Status:** Draft
- **Next Review:** 2026-07-02
- **Total Line Count:** ~2,800 lines
- **Sections:** 12 major sections
- **Code Examples:** 25+ Rust implementations

---

*This document represents a comprehensive state-of-the-art analysis of token systems as of April 2026. Technologies and best practices evolve rapidly; always verify current recommendations against the latest standards and security advisories.*
