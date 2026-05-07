# ADR-008: Multi-Tenant Isolation via Tenant ID Header

**Status:** Accepted  
**Date:** 2026-04-02  
**Deciders:** Architecture Team  

---

## Context

We need to support multiple tenants on a single Tokn deployment with strict isolation. Each tenant must have:
- Isolated token namespaces
- Separate cryptographic keys
- Independent rate limits
- Isolated audit logs

Previous approaches:
- **Database schema isolation** - Complex migration, resource inefficiency
- **Separate deployments** - Operational overhead, high cost
- **Tenant ID in JWT** - Cannot trust unverified claims

---

## Decision

We will use **tenant ID from request header with cryptographic isolation**.

### Isolation Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Multi-Tenant Isolation Architecture                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Request Flow:                                                               │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │   Client Request                                                    │  │
│  │   ┌─────────────────────────────────────────────────────────────┐  │  │
│  │   │ X-Tokn-Tenant-ID: tenant_abc123                              │  │  │
│  │   │ Authorization: Bearer eyJhbGci...                           │  │  │
│  │   └─────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                       │  │
│  │                              ▼                                       │  │
│  │   Tenant Resolver Middleware                                         │  │
│  │   ┌─────────────────────────────────────────────────────────────┐  │  │
│  │   │  1. Extract tenant ID from header                          │  │  │
│  │   │  2. Validate tenant exists and is active                   │  │  │
│  │   │  3. Load tenant-specific configuration                      │  │  │
│  │   │  4. Inject tenant context into request                      │  │  │
│  │   └─────────────────────────────────────────────────────────────┘  │  │
│  │                              │                                       │  │
│  │                              ▼                                       │  │
│  │   Tenant-Aware Services                                             │  │
│  │   ┌─────────────────────────────────────────────────────────────┐  │  │
│  │   │                                                             │  │  │
│  │   │  Storage: WHERE tenant_id = $tenant_id                     │  │  │
│  │   │  Keys:     key_service.get_tenant_key($tenant_id)         │  │  │
│  │   │  Limits:   rate_limiter.check($tenant_id, ...)             │  │  │
│  │   │  Audit:    audit.log(..., tenant_id = $tenant_id)          │  │  │
│  │   │                                                             │  │  │
│  │   └─────────────────────────────────────────────────────────────┘  │  │
│  │                                                                      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Tenant Isolation Matrix:                                                    │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  Resource        │  Isolation Method    │  Verification             │  │
│  ├──────────────────┼──────────────────────┼────────────────────────────┤  │
│  │  Tokens          │  tenant_id claim    │  Signature verification    │  │
│  │  Signing Keys    │  Per-tenant key ID  │  Key resolution           │  │
│  │  Rate Limits     │  Tenant key prefix  │  Redis namespace          │  │
│  │  Audit Logs      │  tenant_id index    │  Query filtering          │  │
│  │  Configuration    │  Tenant config map │  Config resolution        │  │
│  │  Revocations     │  Tenant scope       │  DB tenant isolation      │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Implementation

```rust
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: String,
    pub tier: TenantTier,
    pub key_id: String,
    pub rate_limit_config: RateLimitConfig,
    pub features: FeatureFlags,
}

pub struct TenantMiddleware {
    tenant_resolver: Arc<TenantResolver>,
    default_tenant: String,
}

impl TenantMiddleware {
    pub async fn resolve_tenant(
        &self,
        request: &mut Request,
    ) -> Result<TenantContext, TenantError> {
        // Extract tenant ID from header
        let tenant_id = request
            .headers()
            .get("X-Tokn-Tenant-ID")
            .and_then(|v| v.to_str().ok())
            .map(String::from)
            .unwrap_or_else(|| self.default_tenant.clone());
        
        // Validate tenant
        let tenant = self.tenant_resolver
            .get_tenant(&tenant_id)
            .await?
            .ok_or(TenantError::TenantNotFound(tenant_id.clone()))?;
        
        if !tenant.active {
            return Err(TenantError::TenantInactive(tenant_id));
        }
        
        // Build tenant context
        Ok(TenantContext {
            tenant_id: tenant.id,
            tier: tenant.tier,
            key_id: tenant.key_id,
            rate_limit_config: tenant.rate_limit_config,
            features: tenant.features,
        })
    }
}

// Per-tenant key resolution
impl KeyService {
    pub async fn get_tenant_signing_key(
        &self,
        tenant_id: &str,
        alg: Algorithm,
    ) -> Result<Arc<dyn SigningKey>, KeyError> {
        let tenant = self.tenant_resolver
            .get_tenant(tenant_id)
            .await?
            .ok_or(KeyError::TenantNotFound)?;
        
        self.get_signing_key(&tenant.key_id, alg).await
    }
}
```

### Header Specification

| Header | Required | Format | Description |
|--------|----------|--------|-------------|
| **X-Tokn-Tenant-ID** | Yes | String (max 64 chars) | Tenant identifier |
| **X-Tokn-Tenant-Key** | Conditional | String | Tenant API key for authentication |
| **X-Tokn-Request-ID** | No | UUID | Request correlation ID |

### Validation Rules

```rust
pub struct TenantValidation {
    pub max_id_length: usize = 64,
    pub allowed_chars: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap(),
    pub reserved_ids: Vec<String> = vec!["system", "admin", "default"],
}

impl TenantValidation {
    pub fn validate_tenant_id(&self, id: &str) -> Result<(), TenantValidationError> {
        if id.is_empty() {
            return Err(TenantValidationError::EmptyId);
        }
        if id.len() > self.max_id_length {
            return Err(TenantValidationError::IdTooLong);
        }
        if !self.allowed_chars.is_match(id) {
            return Err(TenantValidationError::InvalidCharacters);
        }
        if self.reserved_ids.contains(&id.to_lowercase()) {
            return Err(TenantValidationError::ReservedId);
        }
        Ok(())
    }
}
```

---

## Consequences

### Positive
- Simple header-based tenant identification
- Strong isolation via per-tenant resources
- No database schema complexity
- Cost-effective multi-tenancy
- Independent tenant configuration

### Negative
- Header spoofing risk (mitigated by validation)
- Cross-tenant requests require explicit trust
- Tenant isolation relies on application-level checks
- Need for tenant-aware debugging tools

### Mitigation
- Validate tenant ID against API key or JWT claim
- Implement tenant isolation tests
- Add tenant ID to all log entries
- Monitor cross-tenant access attempts

---

## References

- [Multi-Tenant Patterns](https://docs.microsoft.com/en-us/azure/azure-sql/database/designing-multi-tenant-saas-apps)
- [OWASP Tenant Isolation](https://owasp.org/www-project-web-security-testing/)
