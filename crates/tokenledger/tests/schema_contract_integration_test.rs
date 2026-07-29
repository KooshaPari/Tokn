//! P1.1 contract tests for provider-neutral usage events and pricing.
//!
//! These tests intentionally exercise the JSON boundary used by every ingest
//! adapter.  They protect the schema from silently changing token semantics,
//! dropping optional metadata, or accepting broken alias references.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::json;
use tokenledger::models::{
    ModelRate, PricingBook, ProviderPricing, TokenUsage, UsageEvent,
};
use tokenledger::utils::{resolve_model_alias, resolve_provider_alias, validate_aliases};

#[test]
fn usage_event_round_trip_preserves_all_token_dimensions() {
    let event = UsageEvent {
        provider: "anthropic".into(),
        model: "sonnet".into(),
        session_id: "session-42".into(),
        timestamp: DateTime::parse_from_rfc3339("2026-07-29T01:02:03-07:00")
            .expect("valid RFC3339 timestamp")
            .with_timezone(&Utc),
        usage: TokenUsage {
            input_tokens: 11,
            output_tokens: 22,
            cache_write_tokens: 33,
            cache_read_tokens: 44,
            tool_input_tokens: 55,
            tool_output_tokens: 66,
        },
        tenant_id: Some("tenant-7".into()),
    };

    let encoded = serde_json::to_value(&event).expect("usage event serializes");
    let decoded: UsageEvent = serde_json::from_value(encoded).expect("usage event parses");

    assert_eq!(decoded.provider, "anthropic");
    assert_eq!(decoded.model, "sonnet");
    assert_eq!(decoded.session_id, "session-42");
    assert_eq!(decoded.timestamp, event.timestamp);
    assert_eq!(decoded.tenant_id.as_deref(), Some("tenant-7"));
    assert_eq!(decoded.usage.input_tokens, 11);
    assert_eq!(decoded.usage.output_tokens, 22);
    assert_eq!(decoded.usage.cache_write_tokens, 33);
    assert_eq!(decoded.usage.cache_read_tokens, 44);
    assert_eq!(decoded.usage.tool_input_tokens, 55);
    assert_eq!(decoded.usage.tool_output_tokens, 66);
    assert_eq!(decoded.usage.total(), 231);
}

#[test]
fn usage_event_defaults_missing_optional_token_counters_and_tenant() {
    let event: UsageEvent = serde_json::from_value(json!({
        "provider": "codex",
        "model": "gpt-5",
        "session_id": "session-1",
        "timestamp": "2026-07-29T08:00:00Z",
        "usage": {"input_tokens": 12, "output_tokens": 8}
    }))
    .expect("minimal provider event should parse");

    assert_eq!(event.usage.input_tokens, 12);
    assert_eq!(event.usage.output_tokens, 8);
    assert_eq!(event.usage.cache_write_tokens, 0);
    assert_eq!(event.usage.cache_read_tokens, 0);
    assert_eq!(event.usage.tool_input_tokens, 0);
    assert_eq!(event.usage.tool_output_tokens, 0);
    assert_eq!(event.tenant_id, None);

    let encoded = serde_json::to_value(&event).expect("event serializes");
    assert!(encoded.get("tenant_id").is_none());
}

#[test]
fn pricing_aliases_resolve_to_canonical_provider_and_model() {
    let mut models = HashMap::new();
    models.insert(
        "claude-sonnet-4-5".into(),
        ModelRate {
            input_usd_per_mtok: 3.0,
            output_usd_per_mtok: 15.0,
            cache_write_usd_per_mtok: Some(3.75),
            cache_read_usd_per_mtok: Some(0.30),
            tool_input_usd_per_mtok: Some(3.0),
            tool_output_usd_per_mtok: Some(15.0),
        },
    );

    let mut providers = HashMap::new();
    providers.insert(
        "claude".into(),
        ProviderPricing {
            subscription_usd_month: 30.0,
            models,
            model_aliases: HashMap::from([(String::from("sonnet"), String::from("claude-sonnet-4-5"))]),
        },
    );
    let pricing = PricingBook {
        providers,
        provider_aliases: HashMap::from([(String::from("anthropic"), String::from("claude"))]),
        meta: None,
    };

    validate_aliases(&pricing).expect("all aliases point at declared entries");
    assert_eq!(resolve_provider_alias("anthropic", &pricing), "claude");
    assert_eq!(resolve_model_alias("claude", "sonnet", &pricing), "claude-sonnet-4-5");
}

#[test]
fn pricing_alias_validation_rejects_dangling_references() {
    let pricing = PricingBook {
        providers: HashMap::from([(
            String::from("claude"),
            ProviderPricing {
                subscription_usd_month: 30.0,
                models: HashMap::new(),
                model_aliases: HashMap::from([(String::from("sonnet"), String::from("missing"))]),
            },
        )]),
        provider_aliases: HashMap::new(),
        meta: None,
    };

    let error = validate_aliases(&pricing).expect_err("dangling model alias must fail");
    assert!(error.to_string().contains("unknown model"));
}
