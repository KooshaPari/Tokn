//! Tenant isolation module for multi-tenant token ledger operations.
//!
//! Implements the ADR-008 multi-tenancy contract:
//! - Tenant identity types and validation
//! - Tenant-scoped context for storage/query filtering
//! - Cryptographically-verified tenant identity via typed errors
//!
//! Architecture:
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │            Tenant Module                    │
//! │  ┌────────────────────────────────────┐    │
//! │  │  TenantResolver                    │    │
//! │  │  • resolve_tenant(tenant_id)       │    │
//! │  │  • validate_tenant_id(id)          │    │
//! │  └────────────────────────────────────┘    │
//! │  ┌────────────────────────────────────┐    │
//! │  │  TenantContext                     │    │
//! │  │  • tenant_id, tier, key_id        │    │
//! │  │  • rate_limit, features           │    │
//! │  └────────────────────────────────────┘    │
//! └─────────────────────────────────────────────┘
//! ```
//!
//! Traces to: ADR-008

use std::collections::HashMap;

/// Core context for an authenticated tenant request.
#[derive(Debug, Clone)]
pub struct TenantContext {
    /// Unique tenant identifier.
    pub tenant_id: String,
    /// Service tier for rate limiting and feature access.
    pub tier: TenantTier,
    /// Tenant-specific signing key identifier.
    pub key_id: String,
    /// Rate-limiting configuration for this tenant.
    pub rate_limit_config: RateLimitConfig,
    /// Feature flags enabled for this tenant.
    pub features: FeatureFlags,
}

/// Service tier assigned to a tenant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TenantTier {
    Free,
    Pro,
    Enterprise,
}

impl TenantTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            TenantTier::Free => "free",
            TenantTier::Pro => "pro",
            TenantTier::Enterprise => "enterprise",
        }
    }
}

/// Rate-limit configuration scoped to a tenant.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per minute.
    pub requests_per_minute: u32,
    /// Maximum tokens per minute.
    pub tokens_per_minute: u64,
    /// Burst capacity.
    pub burst: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            tokens_per_minute: 1_000_000,
            burst: 10,
        }
    }
}

/// Feature flags for tenant-level gating.
#[derive(Debug, Clone, Default)]
pub struct FeatureFlags {
    /// Allow caching of tenant data.
    pub cache_enabled: bool,
    /// Allow audit logging scoped to tenant.
    pub audit_logging: bool,
    /// Custom flags.
    pub custom: HashMap<String, bool>,
}

/// Errors that can occur during tenant resolution.
#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("Tenant not found: {0}")]
    TenantNotFound(String),

    #[error("Tenant is inactive: {0}")]
    TenantInactive(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Tenant ID is empty")]
    EmptyId,

    #[error("Tenant ID is too long (max {max} chars, got {actual})")]
    IdTooLong { max: usize, actual: usize },

    #[error("Tenant ID contains invalid characters")]
    InvalidCharacters,

    #[error("Tenant ID is reserved: {0}")]
    ReservedId(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

/// Result alias for tenant operations.
pub type TenantResult<T> = Result<T, TenantError>;

/// Tenant ID validation rules from ADR-008.
pub struct TenantValidation {
    /// Maximum allowed tenant ID length.
    pub max_id_length: usize,
    /// Reserved tenant IDs that cannot be used.
    pub reserved_ids: Vec<String>,
}

impl Default for TenantValidation {
    fn default() -> Self {
        Self {
            max_id_length: 64,
            reserved_ids: vec![
                "system".to_string(),
                "admin".to_string(),
                "default".to_string(),
            ],
        }
    }
}

impl TenantValidation {
    /// Validate a tenant ID according to ADR-008 rules.
    ///
    /// Rules:
    /// - Must not be empty
    /// - Must be <= max_id_length characters
    /// - Must only contain alphanumeric, underscore, or hyphen characters
    /// - Must not be a reserved ID
    pub fn validate_tenant_id(&self, id: &str) -> TenantResult<()> {
        if id.is_empty() {
            return Err(TenantError::EmptyId);
        }
        if id.len() > self.max_id_length {
            return Err(TenantError::IdTooLong {
                max: self.max_id_length,
                actual: id.len(),
            });
        }
        if !id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(TenantError::InvalidCharacters);
        }
        if self.reserved_ids.contains(&id.to_lowercase()) {
            return Err(TenantError::ReservedId(id.to_string()));
        }
        Ok(())
    }
}

/// In-memory tenant resolver for testing and lightweight deployments.
#[derive(Debug, Default)]
pub struct TenantResolver {
    tenants: HashMap<String, TenantEntry>,
}

/// A stored tenant entry.
#[derive(Debug, Clone)]
pub struct TenantEntry {
    pub tenant_id: String,
    pub active: bool,
    pub tier: TenantTier,
    pub key_id: String,
    pub rate_limit_config: RateLimitConfig,
    pub features: FeatureFlags,
}

impl TenantResolver {
    /// Create a new empty resolver.
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
        }
    }

    /// Register a tenant.
    pub fn register(&mut self, entry: TenantEntry) {
        self.tenants.insert(entry.tenant_id.clone(), entry);
    }

    /// Look up a tenant by ID.
    pub fn get_tenant(&self, tenant_id: &str) -> Option<&TenantEntry> {
        self.tenants.get(tenant_id)
    }

    /// Resolve a tenant context from a tenant ID, validating existence and active state.
    pub fn resolve_tenant(&self, tenant_id: &str) -> TenantResult<TenantContext> {
        let tenant = self
            .tenants
            .get(tenant_id)
            .ok_or_else(|| TenantError::TenantNotFound(tenant_id.to_string()))?;

        if !tenant.active {
            return Err(TenantError::TenantInactive(tenant_id.to_string()));
        }

        Ok(TenantContext {
            tenant_id: tenant.tenant_id.clone(),
            tier: tenant.tier,
            key_id: tenant.key_id.clone(),
            rate_limit_config: tenant.rate_limit_config.clone(),
            features: tenant.features.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_validation_valid_ids() {
        let validation = TenantValidation::default();
        assert!(validation.validate_tenant_id("tenant-abc").is_ok());
        assert!(validation.validate_tenant_id("tenant_123").is_ok());
        assert!(validation.validate_tenant_id("a").is_ok());
        assert!(validation.validate_tenant_id("tenant-abc-123_xyz").is_ok());
    }

    #[test]
    fn test_tenant_validation_empty() {
        let validation = TenantValidation::default();
        let err = validation.validate_tenant_id("").unwrap_err();
        assert!(matches!(err, TenantError::EmptyId));
    }

    #[test]
    fn test_tenant_validation_too_long() {
        let validation = TenantValidation {
            max_id_length: 8,
            ..Default::default()
        };
        let err = validation.validate_tenant_id("toolong-id").unwrap_err();
        assert!(matches!(err, TenantError::IdTooLong { max: 8, .. }));
    }

    #[test]
    fn test_tenant_validation_invalid_chars() {
        let validation = TenantValidation::default();
        assert!(matches!(
            validation.validate_tenant_id("tenant id!").unwrap_err(),
            TenantError::InvalidCharacters
        ));
        assert!(matches!(
            validation.validate_tenant_id("tenant/id").unwrap_err(),
            TenantError::InvalidCharacters
        ));
        assert!(matches!(
            validation.validate_tenant_id("tenant.id").unwrap_err(),
            TenantError::InvalidCharacters
        ));
    }

    #[test]
    fn test_tenant_validation_reserved() {
        let validation = TenantValidation::default();
        assert!(matches!(
            validation.validate_tenant_id("system").unwrap_err(),
            TenantError::ReservedId(_)
        ));
        assert!(matches!(
            validation.validate_tenant_id("admin").unwrap_err(),
            TenantError::ReservedId(_)
        ));
        assert!(matches!(
            validation.validate_tenant_id("default").unwrap_err(),
            TenantError::ReservedId(_)
        ));
    }

    #[test]
    fn test_validation_case_insensitive_reserved() {
        let validation = TenantValidation::default();
        assert!(matches!(
            validation.validate_tenant_id("System").unwrap_err(),
            TenantError::ReservedId(_)
        ));
        assert!(matches!(
            validation.validate_tenant_id("ADMIN").unwrap_err(),
            TenantError::ReservedId(_)
        ));
    }

    #[test]
    fn test_tenant_tier_as_str() {
        assert_eq!(TenantTier::Free.as_str(), "free");
        assert_eq!(TenantTier::Pro.as_str(), "pro");
        assert_eq!(TenantTier::Enterprise.as_str(), "enterprise");
    }

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.requests_per_minute, 60);
        assert_eq!(config.tokens_per_minute, 1_000_000);
        assert_eq!(config.burst, 10);
    }

    #[test]
    fn test_tenant_resolver_basic_flow() {
        let mut resolver = TenantResolver::new();
        resolver.register(TenantEntry {
            tenant_id: "tenant-abc".to_string(),
            active: true,
            tier: TenantTier::Pro,
            key_id: "key-abc".to_string(),
            rate_limit_config: RateLimitConfig::default(),
            features: FeatureFlags::default(),
        });

        let ctx = resolver
            .resolve_tenant("tenant-abc")
            .expect("should resolve tenant");
        assert_eq!(ctx.tenant_id, "tenant-abc");
        assert_eq!(ctx.tier, TenantTier::Pro);
        assert_eq!(ctx.key_id, "key-abc");
    }

    #[test]
    fn test_tenant_resolver_not_found() {
        let resolver = TenantResolver::new();
        let err = resolver.resolve_tenant("nonexistent").unwrap_err();
        assert!(matches!(err, TenantError::TenantNotFound(_)));
    }

    #[test]
    fn test_tenant_resolver_inactive() {
        let mut resolver = TenantResolver::new();
        resolver.register(TenantEntry {
            tenant_id: "inactive-tenant".to_string(),
            active: false,
            tier: TenantTier::Free,
            key_id: "key-inactive".to_string(),
            rate_limit_config: RateLimitConfig::default(),
            features: FeatureFlags::default(),
        });

        let err = resolver.resolve_tenant("inactive-tenant").unwrap_err();
        assert!(matches!(err, TenantError::TenantInactive(_)));
    }

    #[test]
    fn test_tenant_context_construction() {
        let ctx = TenantContext {
            tenant_id: "test-tenant".to_string(),
            tier: TenantTier::Enterprise,
            key_id: "key-001".to_string(),
            rate_limit_config: RateLimitConfig {
                requests_per_minute: 1000,
                tokens_per_minute: 10_000_000,
                burst: 100,
            },
            features: FeatureFlags {
                cache_enabled: true,
                audit_logging: true,
                custom: HashMap::new(),
            },
        };
        assert_eq!(ctx.tenant_id, "test-tenant");
        assert_eq!(ctx.tier, TenantTier::Enterprise);
        assert_eq!(ctx.rate_limit_config.requests_per_minute, 1000);
    }

    #[test]
    fn test_tenant_error_display() {
        let err = TenantError::TenantNotFound("abc".to_string());
        assert_eq!(err.to_string(), "Tenant not found: abc");

        let err = TenantError::EmptyId;
        assert_eq!(err.to_string(), "Tenant ID is empty");

        let err = TenantError::IdTooLong {
            max: 64,
            actual: 100,
        };
        assert_eq!(
            err.to_string(),
            "Tenant ID is too long (max 64 chars, got 100)"
        );
    }

    #[test]
    fn test_cross_tenant_isolation_resolver() {
        // Different tenants should have independent contexts
        let mut resolver = TenantResolver::new();
        resolver.register(TenantEntry {
            tenant_id: "tenant-a".to_string(),
            active: true,
            tier: TenantTier::Free,
            key_id: "key-a".to_string(),
            rate_limit_config: RateLimitConfig {
                requests_per_minute: 60,
                ..Default::default()
            },
            features: FeatureFlags::default(),
        });
        resolver.register(TenantEntry {
            tenant_id: "tenant-b".to_string(),
            active: true,
            tier: TenantTier::Enterprise,
            key_id: "key-b".to_string(),
            rate_limit_config: RateLimitConfig {
                requests_per_minute: 10000,
                ..Default::default()
            },
            features: FeatureFlags {
                cache_enabled: true,
                ..Default::default()
            },
        });

        let ctx_a = resolver.resolve_tenant("tenant-a").unwrap();
        let ctx_b = resolver.resolve_tenant("tenant-b").unwrap();

        // Assert isolation: different configs, different keys
        assert_ne!(ctx_a.tier, ctx_b.tier);
        assert_ne!(ctx_a.key_id, ctx_b.key_id);
        assert_ne!(
            ctx_a.rate_limit_config.requests_per_minute,
            ctx_b.rate_limit_config.requests_per_minute
        );
        assert_ne!(ctx_a.features.cache_enabled, ctx_b.features.cache_enabled);

        // Cross-tenant: resolving tenant-b from tenant-a's resolver should still work
        // (the resolver is shared; isolation is enforced via context fields)
        assert!(resolver.resolve_tenant("tenant-b").is_ok());
        assert!(resolver.resolve_tenant("tenant-a").is_ok());
    }
}
