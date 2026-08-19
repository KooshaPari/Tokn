// Cost calculation utilities

use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

use anyhow::{anyhow, Result};

use crate::cli::OnUnpricedAction;
use crate::format::{round2, round4};
use crate::models::*;

pub const MTOK: f64 = 1_000_000.0;

pub fn compute_costs(
    events: &[UsageEvent],
    pricing: &PricingBook,
    on_unpriced: OnUnpricedAction,
) -> Result<CostBreakdown> {
    let mut global = Acc::default();
    let mut by_provider: BTreeMap<String, Acc> = BTreeMap::new();
    let mut by_model: BTreeMap<String, Acc> = BTreeMap::new();
    let mut provider_token_totals: HashMap<String, u64> = HashMap::new();
    let mut missing: BTreeMap<String, usize> = BTreeMap::new();

    for evt in events {
        if event_pricing(evt, pricing).is_some() {
            *provider_token_totals
                .entry(evt.provider.clone())
                .or_default() += evt.usage.total();
        } else {
            *missing
                .entry(format!("{}:{}", evt.provider, evt.model))
                .or_default() += 1;
        }
    }

    if on_unpriced == OnUnpricedAction::Error && !missing.is_empty() {
        let details = missing
            .iter()
            .map(|(key, count)| format!("{key} (events={count})"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(anyhow!(
            "unpriced events found: {}. Re-run with --on-unpriced skip to ignore them",
            details
        ));
    }

    for evt in events {
        let Some((provider, rate)) = event_pricing(evt, pricing) else {
            continue;
        };

        let variable_cost = calc_variable_cost(&evt.usage, rate);
        let provider_total_tokens = *provider_token_totals
            .get(&evt.provider)
            .ok_or_else(|| anyhow!("missing token totals for provider {}", evt.provider))?;
        let event_sub_alloc = allocate_subscription(
            evt.usage.total(),
            provider_total_tokens,
            provider.subscription_usd_month,
        );

        merge_acc(
            &mut global,
            evt,
            variable_cost,
            event_sub_alloc,
        );
        merge_acc(
            by_provider.entry(evt.provider.clone()).or_default(),
            evt,
            variable_cost,
            event_sub_alloc,
        );
        merge_acc(
            by_model.entry(evt.model.clone()).or_default(),
            evt,
            variable_cost,
            event_sub_alloc,
        );
    }

    let total_subscription = global.subscription_allocated_usd;
    let provider_breakdown = build_breakdown(&by_provider);
    let model_breakdown = build_breakdown(&by_model);

    let sub_alloc = global.subscription_allocated_usd;
    let monthly_total = global.variable_cost_usd + sub_alloc;
    let mtok = global.tokens as f64 / MTOK;
    let blended = if mtok > 0.0 {
        monthly_total / mtok
    } else {
        0.0
    };

    Ok(CostBreakdown {
        variable_cost_usd: round2(global.variable_cost_usd),
        subscription_allocated_usd: round2(sub_alloc),
        monthly_total_usd: round2(monthly_total),
        blended_usd_per_mtok: round4(blended),
        total_tokens: global.tokens,
        total_mtok: round4(mtok),
        input_tokens: global.input_tokens,
        output_tokens: global.output_tokens,
        cache_write_tokens: global.cache_write_tokens,
        cache_read_tokens: global.cache_read_tokens,
        tool_input_tokens: global.tool_input_tokens,
        tool_output_tokens: global.tool_output_tokens,
        session_count: global.sessions.len(),
        skipped_unpriced_count: missing.values().copied().sum(),
        provider_breakdown,
        model_breakdown,
        suggestions: make_suggestions(&global, total_subscription),
    })
}

pub fn build_breakdown(items: &BTreeMap<String, Acc>) -> Vec<NamedMetric> {
    items
        .iter()
        .map(|(name, acc)| {
            let sub = acc.subscription_allocated_usd;
            let total = acc.variable_cost_usd + sub;
            let mtok = acc.tokens as f64 / MTOK;
            let tool_tokens = acc.tool_input_tokens + acc.tool_output_tokens;
            let tool_share = if acc.tokens == 0 {
                0.0
            } else {
                tool_tokens as f64 / acc.tokens as f64
            };
            NamedMetric {
                name: name.clone(),
                tokens: acc.tokens,
                mtok: round4(mtok),
                variable_cost_usd: round2(acc.variable_cost_usd),
                subscription_allocated_usd: round2(sub),
                total_cost_usd: round2(total),
                blended_usd_per_mtok: round4(if mtok > 0.0 { total / mtok } else { 0.0 }),
                session_count: acc.sessions.len(),
                tool_share: round4(tool_share),
            }
        })
        .collect()
}

pub fn merge_acc(acc: &mut Acc, evt: &UsageEvent, variable_cost: f64, sub_alloc: f64) {
    acc.tokens += evt.usage.total();
    acc.input_tokens += evt.usage.input_tokens;
    acc.output_tokens += evt.usage.output_tokens;
    acc.cache_write_tokens += evt.usage.cache_write_tokens;
    acc.cache_read_tokens += evt.usage.cache_read_tokens;
    acc.tool_input_tokens += evt.usage.tool_input_tokens;
    acc.tool_output_tokens += evt.usage.tool_output_tokens;
    acc.variable_cost_usd += variable_cost;
    acc.subscription_allocated_usd += sub_alloc;
    acc.sessions
        .insert(session_hash(&evt.provider, &evt.session_id));
}

pub fn session_hash(provider: &str, session_id: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    provider.hash(&mut hasher);
    session_id.hash(&mut hasher);
    hasher.finish()
}

pub fn calc_variable_cost(usage: &TokenUsage, rate: &ModelRate) -> f64 {
    let cache_write = rate
        .cache_write_usd_per_mtok
        .unwrap_or(rate.input_usd_per_mtok);
    let cache_read = rate
        .cache_read_usd_per_mtok
        .unwrap_or(rate.input_usd_per_mtok * 0.1);
    let tool_in = rate
        .tool_input_usd_per_mtok
        .unwrap_or(rate.input_usd_per_mtok);
    let tool_out = rate
        .tool_output_usd_per_mtok
        .unwrap_or(rate.output_usd_per_mtok);

    (usage.input_tokens as f64 / MTOK) * rate.input_usd_per_mtok
        + (usage.output_tokens as f64 / MTOK) * rate.output_usd_per_mtok
        + (usage.cache_write_tokens as f64 / MTOK) * cache_write
        + (usage.cache_read_tokens as f64 / MTOK) * cache_read
        + (usage.tool_input_tokens as f64 / MTOK) * tool_in
        + (usage.tool_output_tokens as f64 / MTOK) * tool_out
}

pub fn allocate_subscription(item_tokens: u64, total_tokens: u64, subscription: f64) -> f64 {
    if total_tokens == 0 {
        0.0
    } else {
        subscription * (item_tokens as f64 / total_tokens as f64)
    }
}

pub fn make_suggestions(global: &Acc, total_subscription: f64) -> Vec<String> {
    let mut tips = Vec::new();
    let total_tokens = global.tokens as f64;
    if total_tokens > 0.0 {
        let tool_share =
            (global.tool_input_tokens + global.tool_output_tokens) as f64 / total_tokens;
        if tool_share > 0.35 {
            tips.push("Tool-token share is high (>35%): add per-tool budgets and short-circuit low-value tool calls.".to_string());
        }
        let cache_share = global.cache_read_tokens as f64 / total_tokens;
        if cache_share < 0.10 {
            tips.push("Cache-read share is low (<10%): improve prompt prefix reuse and session stickiness for Claude-style caching.".to_string());
        }
        let var_per_mtok = global.variable_cost_usd / (total_tokens / MTOK);
        if var_per_mtok > 12.0 {
            tips.push("Blended variable $/MTok is high: route low-complexity jobs to cheaper models/providers via policy rules.".to_string());
        }
    }
    let total_monthly = global.variable_cost_usd + total_subscription;
    if total_monthly > 0.0 && total_subscription / total_monthly > 0.7 {
        tips.push("Subscriptions dominate monthly cost (>70%): consolidate seats/plans or increase utilization with shared routing.".to_string());
    }
    if tips.is_empty() {
        tips.push("No obvious anomalies detected; keep collecting session-level data and compare 4-week trend deltas.".to_string());
    }
    tips
}

// Helper function to get event pricing - uses utils functions
pub fn event_pricing<'a>(evt: &UsageEvent, pricing: &'a PricingBook) -> Option<(&'a ProviderPricing, &'a ModelRate)> {
    let provider_name = crate::utils::resolve_provider_alias(&evt.provider, pricing);
    let provider = pricing.providers.get(&provider_name)?;
    let model_name = crate::utils::resolve_model_alias(&provider_name, &evt.model, pricing);
    let rate = provider.models.get(&model_name)?;
    Some((provider, rate))
}
