//! Cross-tenant isolation tests for Tokn token ledger.
//!
//! Verifies:
//! - Tenant ID validation rules (ADR-008)
//! - Tenant resolver isolation (different tenants get different configs)
//! - Tenant-scoped UsageEvent construction and filtering
//! - Cross-tenant spoofing prevention
//!
//! Traces to: ADR-008, L11-L13 audit findings

use chrono::Utc;

use tokenledger::models::*;
use tokenledger::tenant::*;

// =============================================================================
// Tenant ID Validation Tests (ADR-008 §Validation Rules)
// =============================================================================

#[test]
fn test_tenant_id_validation_rejects_empty() {
    let validation = TenantValidation::default();
    let err = validation.validate_tenant_id("").unwrap_err();
    assert!(matches!(err, TenantError::EmptyId));
}

#[test]
fn test_tenant_id_validation_rejects_spaces() {
    let validation = TenantValidation::default();
    let err = validation.validate_tenant_id("my tenant").unwrap_err();
    assert!(matches!(err, TenantError::InvalidCharacters));
}

#[test]
fn test_tenant_id_validation_rejects_special_chars() {
    let validation = TenantValidation::default();
    assert!(validation.validate_tenant_id("tenant-abc_123").is_ok());
    assert!(validation.validate_tenant_id("tenant@abc").is_err());
    assert!(validation.validate_tenant_id("tenant.abc").is_err());
    assert!(validation.validate_tenant_id("tenant/abc").is_err());
}

#[test]
fn test_tenant_id_validation_rejects_reserved_case_insensitive() {
    let validation = TenantValidation::default();
    for reserved in &["system", "admin", "default"] {
        assert!(validation.validate_tenant_id(reserved).is_err());
        assert!(
            validation
                .validate_tenant_id(&reserved.to_uppercase())
                .is_err()
        );
        assert!(
            validation
                .validate_tenant_id(&capitalize(reserved))
                .is_err()
        );
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

#[test]
fn test_tenant_id_validation_accepts_valid_ids() {
    let validation = TenantValidation::default();
    let valid_ids = vec![
        "acme-corp",
        "startup_io",
        "tenant-123",
        "my_tenant-001",
        "a",
        "ok",
        "tenant-id_with_underscores-and-hyphens",
    ];
    for id in valid_ids {
        assert!(
            validation.validate_tenant_id(id).is_ok(),
            "expected valid tenant_id: {id}"
        );
    }
}

// =============================================================================
// Tenant Resolver Isolation Tests
// =============================================================================

fn create_test_resolver() -> TenantResolver {
    let mut resolver = TenantResolver::new();

    // Tenant A: Free tier, no cache
    resolver.register(TenantEntry {
        tenant_id: "tenant-free".to_string(),
        active: true,
        tier: TenantTier::Free,
        key_id: "key-free-001".to_string(),
        rate_limit_config: RateLimitConfig {
            requests_per_minute: 60,
            ..Default::default()
        },
        features: FeatureFlags::default(),
    });

    // Tenant B: Enterprise tier, with caching
    resolver.register(TenantEntry {
        tenant_id: "enterprise-corp".to_string(),
        active: true,
        tier: TenantTier::Enterprise,
        key_id: "key-ent-002".to_string(),
        rate_limit_config: RateLimitConfig {
            requests_per_minute: 10000,
            tokens_per_minute: 50_000_000,
            burst: 500,
        },
        features: FeatureFlags {
            cache_enabled: true,
            audit_logging: true,
            custom: std::collections::HashMap::new(),
        },
    });

    // Tenant C: Pro tier (medium)
    resolver.register(TenantEntry {
        tenant_id: "pro-team".to_string(),
        active: true,
        tier: TenantTier::Pro,
        key_id: "key-pro-003".to_string(),
        rate_limit_config: RateLimitConfig {
            requests_per_minute: 1000,
            tokens_per_minute: 10_000_000,
            burst: 50,
        },
        features: FeatureFlags {
            cache_enabled: true,
            ..Default::default()
        },
    });

    resolver
}

#[test]
fn test_tenant_resolver_three_tenants_isolation() {
    let resolver = create_test_resolver();

    let ctx_free = resolver.resolve_tenant("tenant-free").unwrap();
    let ctx_ent = resolver.resolve_tenant("enterprise-corp").unwrap();
    let ctx_pro = resolver.resolve_tenant("pro-team").unwrap();

    // All three tenants must have different configurations
    assert_ne!(ctx_free.tier, ctx_ent.tier);
    assert_ne!(ctx_free.tier, ctx_pro.tier);
    assert_ne!(ctx_pro.tier, ctx_ent.tier);

    // Rate limits must be isolated
    assert_eq!(ctx_free.rate_limit_config.requests_per_minute, 60);
    assert_eq!(ctx_ent.rate_limit_config.requests_per_minute, 10000);
    assert_eq!(ctx_pro.rate_limit_config.requests_per_minute, 1000);

    // Key IDs must be isolated
    assert_eq!(ctx_free.key_id, "key-free-001");
    assert_eq!(ctx_ent.key_id, "key-ent-002");
    assert_eq!(ctx_pro.key_id, "key-pro-003");
}

#[test]
fn test_tenant_resolver_free_has_no_cache() {
    let resolver = create_test_resolver();
    let ctx = resolver.resolve_tenant("tenant-free").unwrap();
    assert!(!ctx.features.cache_enabled);
    assert!(!ctx.features.audit_logging);
}

#[test]
fn test_tenant_resolver_enterprise_has_cache() {
    let resolver = create_test_resolver();
    let ctx = resolver.resolve_tenant("enterprise-corp").unwrap();
    assert!(ctx.features.cache_enabled);
    assert!(ctx.features.audit_logging);
}

#[test]
fn test_tenant_resolver_rejects_inactive_tenant() {
    let mut resolver = TenantResolver::new();
    resolver.register(TenantEntry {
        tenant_id: "disabled-tenant".to_string(),
        active: false,
        tier: TenantTier::Free,
        key_id: "key-disabled".to_string(),
        rate_limit_config: RateLimitConfig::default(),
        features: FeatureFlags::default(),
    });

    let err = resolver.resolve_tenant("disabled-tenant").unwrap_err();
    assert!(matches!(err, TenantError::TenantInactive(_)));
}

#[test]
fn test_tenant_resolver_rejects_unknown_tenant() {
    let resolver = create_test_resolver();
    let err = resolver.resolve_tenant("nonexistent-tenant").unwrap_err();
    assert!(matches!(err, TenantError::TenantNotFound(_)));
}

// =============================================================================
// Tenant-Scoped UsageEvent Tests
// =============================================================================

#[test]
fn test_usage_event_with_tenant_id() {
    let now = Utc::now();
    let event = UsageEvent {
        provider: "openai".to_string(),
        model: "gpt-4".to_string(),
        session_id: "sess-1".to_string(),
        timestamp: now,
        usage: TokenUsage {
            input_tokens: 100,
            output_tokens: 200,
            cache_write_tokens: 0,
            cache_read_tokens: 0,
            tool_input_tokens: 0,
            tool_output_tokens: 0,
        },
        tenant_id: Some("acme-corp".to_string()),
    };

    assert_eq!(event.tenant_id, Some("acme-corp".to_string()));
    assert_eq!(event.provider, "openai");
}

#[test]
fn test_usage_event_without_tenant_id_defaults_to_none() {
    let now = Utc::now();
    let event = UsageEvent {
        provider: "openai".to_string(),
        model: "gpt-4".to_string(),
        session_id: "sess-1".to_string(),
        timestamp: now,
        usage: TokenUsage {
            input_tokens: 100,
            output_tokens: 200,
            cache_write_tokens: 0,
            cache_read_tokens: 0,
            tool_input_tokens: 0,
            tool_output_tokens: 0,
        },
        tenant_id: None,
    };

    assert!(event.tenant_id.is_none());
}

#[test]
fn test_usage_event_tenant_id_serde_roundtrip() {
    let now = Utc::now();
    let event = UsageEvent {
        provider: "openai".to_string(),
        model: "gpt-4o".to_string(),
        session_id: "sess-tenant".to_string(),
        timestamp: now,
        usage: TokenUsage {
            input_tokens: 500,
            output_tokens: 1000,
            cache_write_tokens: 0,
            cache_read_tokens: 0,
            tool_input_tokens: 0,
            tool_output_tokens: 0,
        },
        tenant_id: Some("enterprise-corp".to_string()),
    };

    // Serialize to JSON and back
    let json = serde_json::to_string(&event).expect("serialize");
    let deserialized: UsageEvent = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(deserialized.tenant_id, Some("enterprise-corp".to_string()));
    assert_eq!(deserialized.provider, "openai");
    assert_eq!(deserialized.model, "gpt-4o");
    assert_eq!(deserialized.session_id, "sess-tenant");
    assert_eq!(deserialized.usage.total(), 1500);
}

#[test]
fn test_usage_event_tenant_id_serde_backward_compat() {
    // Old JSON without tenant_id should deserialize with tenant_id=None
    let old_json = r#"{
        "provider": "anthropic",
        "model": "claude-3",
        "session_id": "legacy-session",
        "timestamp": "2026-01-15T00:00:00Z",
        "usage": {
            "input_tokens": 100,
            "output_tokens": 200,
            "cache_write_tokens": 0,
            "cache_read_tokens": 0,
            "tool_input_tokens": 0,
            "tool_output_tokens": 0
        }
    }"#;

    let event: UsageEvent = serde_json::from_str(old_json).expect("deserialize legacy");
    assert!(event.tenant_id.is_none());
    assert_eq!(event.provider, "anthropic");
    assert_eq!(event.model, "claude-3");
}

#[test]
fn test_tenant_filtered_usage_events() {
    // Simulate tenant-scoped query filtering
    let events = vec![
        UsageEvent {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            session_id: "s1".to_string(),
            timestamp: Utc::now(),
            usage: TokenUsage {
                input_tokens: 100,
                output_tokens: 200,
                cache_write_tokens: 0,
                cache_read_tokens: 0,
                tool_input_tokens: 0,
                tool_output_tokens: 0,
            },
            tenant_id: Some("tenant-a".to_string()),
        },
        UsageEvent {
            provider: "anthropic".to_string(),
            model: "claude-3".to_string(),
            session_id: "s2".to_string(),
            timestamp: Utc::now(),
            usage: TokenUsage {
                input_tokens: 300,
                output_tokens: 400,
                cache_write_tokens: 0,
                cache_read_tokens: 0,
                tool_input_tokens: 0,
                tool_output_tokens: 0,
            },
            tenant_id: Some("tenant-b".to_string()),
        },
        UsageEvent {
            provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            session_id: "s3".to_string(),
            timestamp: Utc::now(),
            usage: TokenUsage {
                input_tokens: 50,
                output_tokens: 100,
                cache_write_tokens: 0,
                cache_read_tokens: 0,
                tool_input_tokens: 0,
                tool_output_tokens: 0,
            },
            tenant_id: Some("tenant-a".to_string()),
        },
    ];

    // Filter for tenant-a only
    let tenant_a_events: Vec<&UsageEvent> = events
        .iter()
        .filter(|e| e.tenant_id.as_deref() == Some("tenant-a"))
        .collect();

    assert_eq!(tenant_a_events.len(), 2);
    for evt in &tenant_a_events {
        assert_eq!(evt.tenant_id.as_deref(), Some("tenant-a"));
    }

    // Filter for tenant-b only
    let tenant_b_events: Vec<&UsageEvent> = events
        .iter()
        .filter(|e| e.tenant_id.as_deref() == Some("tenant-b"))
        .collect();

    assert_eq!(tenant_b_events.len(), 1);

    // Cross-tenant isolation: tenant-b should NOT see tenant-a events
    let tenant_b_ids: Vec<&str> = tenant_b_events
        .iter()
        .map(|e| e.session_id.as_str())
        .collect();
    assert!(!tenant_b_ids.contains(&"s1"));
    assert!(!tenant_b_ids.contains(&"s3"));
}

#[test]
fn test_tenant_context_isolation_boundary() {
    // The TenantContext itself should enforce that one tenant cannot
    // access another tenant's configuration
    let ctx = TenantContext {
        tenant_id: "my-tenant".to_string(),
        tier: TenantTier::Pro,
        key_id: "my-key".to_string(),
        rate_limit_config: RateLimitConfig {
            requests_per_minute: 100,
            ..Default::default()
        },
        features: FeatureFlags::default(),
    };

    // The context only exposes this tenant's data
    assert_eq!(ctx.tenant_id, "my-tenant");
    assert_eq!(ctx.key_id, "my-key");
    assert_eq!(ctx.rate_limit_config.requests_per_minute, 100);
}
