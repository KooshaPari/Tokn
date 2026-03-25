use chrono::Utc;
use std::collections::HashMap;
use tokenledger::cost::{allocate_subscription, calc_variable_cost};
use tokenledger::models::*;

#[test]
fn test_token_usage_full_workflow() {
    let usage = TokenUsage {
        input_tokens: 1_000_000,
        output_tokens: 2_000_000,
        cache_write_tokens: 500_000,
        cache_read_tokens: 250_000,
        tool_input_tokens: 100_000,
        tool_output_tokens: 150_000,
    };

    let total = usage.total();
    assert_eq!(total, 4_000_000);
}

#[test]
fn test_pricing_book_with_multiple_providers() {
    let mut providers = HashMap::new();

    let gpt4_rate = ModelRate {
        input_usd_per_mtok: 0.5,
        output_usd_per_mtok: 1.0,
        cache_write_usd_per_mtok: Some(0.1),
        cache_read_usd_per_mtok: Some(0.05),
        tool_input_usd_per_mtok: None,
        tool_output_usd_per_mtok: None,
    };

    let claude_rate = ModelRate {
        input_usd_per_mtok: 0.8,
        output_usd_per_mtok: 2.4,
        cache_write_usd_per_mtok: Some(0.24),
        cache_read_usd_per_mtok: Some(0.3),
        tool_input_usd_per_mtok: None,
        tool_output_usd_per_mtok: None,
    };

    let mut openai_models = HashMap::new();
    openai_models.insert("gpt-4".to_string(), gpt4_rate);

    let mut anthropic_models = HashMap::new();
    anthropic_models.insert("claude-3".to_string(), claude_rate);

    providers.insert(
        "openai".to_string(),
        ProviderPricing {
            subscription_usd_month: 20.0,
            models: openai_models,
            model_aliases: HashMap::new(),
        },
    );

    providers.insert(
        "anthropic".to_string(),
        ProviderPricing {
            subscription_usd_month: 0.0,
            models: anthropic_models,
            model_aliases: HashMap::new(),
        },
    );

    let book = PricingBook {
        providers,
        provider_aliases: HashMap::new(),
        meta: Some(PricingMeta {
            updated_at: Some("2025-01-01".to_string()),
            source: Some("official".to_string()),
            version: Some("1.0".to_string()),
        }),
    };

    assert_eq!(book.providers.len(), 2);
    assert!(book.providers.contains_key("openai"));
    assert!(book.providers.contains_key("anthropic"));

    let openai = &book.providers["openai"];
    assert_eq!(openai.subscription_usd_month, 20.0);
    assert_eq!(openai.models.len(), 1);
}

#[test]
fn test_cost_calculation_workflow() {
    let usage = TokenUsage {
        input_tokens: 1_000_000,
        output_tokens: 1_000_000,
        cache_write_tokens: 0,
        cache_read_tokens: 0,
        tool_input_tokens: 0,
        tool_output_tokens: 0,
    };

    let rate = ModelRate {
        input_usd_per_mtok: 0.5,
        output_usd_per_mtok: 1.5,
        cache_write_usd_per_mtok: None,
        cache_read_usd_per_mtok: None,
        tool_input_usd_per_mtok: None,
        tool_output_usd_per_mtok: None,
    };

    let variable_cost = calc_variable_cost(&usage, &rate);
    assert!((variable_cost - 2.0).abs() < 0.0001);

    let subscription = allocate_subscription(2_000_000, 4_000_000, 120.0);
    assert!((subscription - 60.0).abs() < 0.01);

    let total_cost = variable_cost + subscription;
    assert!((total_cost - 62.0).abs() < 0.01);
}

#[test]
fn test_usage_event_with_all_token_types() {
    let now = Utc::now();
    let usage = TokenUsage {
        input_tokens: 100,
        output_tokens: 200,
        cache_write_tokens: 50,
        cache_read_tokens: 25,
        tool_input_tokens: 10,
        tool_output_tokens: 15,
    };

    let event = UsageEvent {
        provider: "openai".to_string(),
        model: "gpt-4".to_string(),
        session_id: "session-xyz".to_string(),
        timestamp: now,
        usage,
    };

    assert_eq!(event.usage.total(), 400);
    assert_eq!(event.provider, "openai");
    assert_eq!(event.model, "gpt-4");
}

#[test]
fn test_pricing_with_cache_tokens() {
    let usage = TokenUsage {
        input_tokens: 1_000_000,
        output_tokens: 500_000,
        cache_write_tokens: 2_000_000,
        cache_read_tokens: 3_000_000,
        tool_input_tokens: 0,
        tool_output_tokens: 0,
    };

    let rate = ModelRate {
        input_usd_per_mtok: 1.0,
        output_usd_per_mtok: 2.0,
        cache_write_usd_per_mtok: Some(0.25),
        cache_read_usd_per_mtok: Some(0.05),
        tool_input_usd_per_mtok: None,
        tool_output_usd_per_mtok: None,
    };

    let cost = calc_variable_cost(&usage, &rate);
    // 1.0 (input) + 1.0 (output) + 0.5 (cache_write) + 0.15 (cache_read) = 2.65
    assert!((cost - 2.65).abs() < 0.0001);
}

#[test]
fn test_multiple_usage_events_aggregation() {
    let events = [
        UsageEvent {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            session_id: "session-1".to_string(),
            timestamp: Utc::now(),
            usage: TokenUsage {
                input_tokens: 1000,
                output_tokens: 2000,
                cache_write_tokens: 0,
                cache_read_tokens: 0,
                tool_input_tokens: 0,
                tool_output_tokens: 0,
            },
        },
        UsageEvent {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            session_id: "session-2".to_string(),
            timestamp: Utc::now(),
            usage: TokenUsage {
                input_tokens: 500,
                output_tokens: 1500,
                cache_write_tokens: 0,
                cache_read_tokens: 0,
                tool_input_tokens: 0,
                tool_output_tokens: 0,
            },
        },
    ];

    let total_input: u64 = events.iter().map(|e| e.usage.input_tokens).sum();
    let total_output: u64 = events.iter().map(|e| e.usage.output_tokens).sum();
    let total_tokens: u64 = events.iter().map(|e| e.usage.total()).sum();

    assert_eq!(total_input, 1500);
    assert_eq!(total_output, 3500);
    assert_eq!(total_tokens, 5000);
}

#[test]
fn test_subscription_allocation_multiple_providers() {
    let provider_tokens = 10_000_000u64;
    let monthly_cost = 100.0;

    let event1_tokens = 2_000_000u64;
    let event2_tokens = 3_000_000u64;
    let event3_tokens = 5_000_000u64;

    let alloc1 = allocate_subscription(event1_tokens, provider_tokens, monthly_cost);
    let alloc2 = allocate_subscription(event2_tokens, provider_tokens, monthly_cost);
    let alloc3 = allocate_subscription(event3_tokens, provider_tokens, monthly_cost);

    let total_allocated = alloc1 + alloc2 + alloc3;

    assert!((alloc1 - 20.0).abs() < 0.01);
    assert!((alloc2 - 30.0).abs() < 0.01);
    assert!((alloc3 - 50.0).abs() < 0.01);
    assert!((total_allocated - 100.0).abs() < 0.01);
}

#[test]
fn test_model_rate_with_optional_fields() {
    let rate_full = ModelRate {
        input_usd_per_mtok: 0.5,
        output_usd_per_mtok: 1.0,
        cache_write_usd_per_mtok: Some(0.1),
        cache_read_usd_per_mtok: Some(0.05),
        tool_input_usd_per_mtok: Some(0.02),
        tool_output_usd_per_mtok: Some(0.03),
    };

    let rate_partial = ModelRate {
        input_usd_per_mtok: 0.5,
        output_usd_per_mtok: 1.0,
        cache_write_usd_per_mtok: None,
        cache_read_usd_per_mtok: None,
        tool_input_usd_per_mtok: None,
        tool_output_usd_per_mtok: None,
    };

    assert!(rate_full.cache_write_usd_per_mtok.is_some());
    assert!(rate_partial.cache_write_usd_per_mtok.is_none());
}

#[test]
fn test_pricing_apply_summary_accumulation() {
    let mut summary = PricingApplySummary::default();
    assert_eq!(summary.providers_added, 0);
    assert_eq!(summary.models_added, 0);
    assert_eq!(summary.aliases_added, 0);
    assert_eq!(summary.models_skipped_existing, 0);

    summary.providers_added = 5;
    summary.models_added = 10;
    summary.aliases_added = 3;
    summary.models_skipped_existing = 2;

    assert_eq!(summary.providers_added, 5);
    assert_eq!(summary.models_added, 10);
    assert_eq!(summary.aliases_added, 3);
    assert_eq!(summary.models_skipped_existing, 2);
}
