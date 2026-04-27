//! ParetoRs — pure cost calculation engine.
//!
//! No I/O, no external API calls. Pure functions only.

use crate::models::*;
use crate::pricing::DEFAULT_CALL_COST;
use crate::utils::RawHarnessRecord;

// ─── Core Cost Calculation ────────────────────────────────────────────────────

/// Calculate total cost for a call given token counts and pricing rates.
#[inline]
pub fn calc_total_cost(input_tokens: u64, output_tokens: u64, rate: PricingRate) -> f64 {
    if rate.use_default {
        return DEFAULT_CALL_COST;
    }
    let input_cost = (input_tokens as f64 / 1_000_000.0) * rate.input_per_m;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * rate.output_per_m;
    input_cost + output_cost
}

/// Build a cost snapshot from raw call data and provider harness info.
#[allow(clippy::too_many_arguments)]
pub fn build_snapshot(
    id: String,
    provider: &str,
    model: &str,
    input_tokens: u64,
    output_tokens: u64,
    rate: PricingRate,
    latency_ms: Option<f64>,
    routing_criteria: Option<RoutingCriteria>,
    routing_score: Option<f64>,
    timestamp: chrono::DateTime<chrono::Utc>,
) -> CostSnapshot {
    let total_cost = calc_total_cost(input_tokens, output_tokens, rate);
    CostSnapshot {
        id,
        provider: provider.to_string(),
        model: model.to_string(),
        input_tokens,
        output_tokens,
        input_cost: if rate.use_default {
            0.0
        } else {
            (input_tokens as f64 / 1_000_000.0) * rate.input_per_m
        },
        output_cost: if rate.use_default {
            0.0
        } else {
            (output_tokens as f64 / 1_000_000.0) * rate.output_per_m
        },
        total_cost,
        latency_ms,
        timestamp,
        routing_criteria: routing_criteria.map(|r| r.to_string()),
        routing_score,
    }
}

/// Aggregate costs from a list of snapshots.
pub fn aggregate_costs(snapshots: &[CostSnapshot]) -> CostAggregate {
    if snapshots.is_empty() {
        return CostAggregate {
            total_input_tokens: 0,
            total_output_tokens: 0,
            total_input_cost: 0.0,
            total_output_cost: 0.0,
            total_cost: 0.0,
            call_count: 0,
        };
    }
    let total_input_tokens: u64 = snapshots.iter().map(|s| s.input_tokens).sum();
    let total_output_tokens: u64 = snapshots.iter().map(|s| s.output_tokens).sum();
    let total_input_cost: f64 = snapshots.iter().map(|s| s.input_cost).sum();
    let total_output_cost: f64 = snapshots.iter().map(|s| s.output_cost).sum();
    let total_cost: f64 = snapshots.iter().map(|s| s.total_cost).sum();
    CostAggregate {
        total_input_tokens,
        total_output_tokens,
        total_input_cost,
        total_output_cost,
        total_cost,
        call_count: snapshots.len(),
    }
}

/// Aggregate costs grouped by provider.
pub fn aggregate_by_provider(snapshots: &[CostSnapshot]) -> Vec<ProviderCostAggregate> {
    use std::collections::HashMap;
    let mut map: HashMap<String, Vec<&CostSnapshot>> = HashMap::new();
    for s in snapshots {
        map.entry(s.provider.clone()).or_default().push(s);
    }
    let mut result: Vec<ProviderCostAggregate> = map
        .into_iter()
        .map(|(provider, group)| {
            let total_input_tokens: u64 = group.iter().map(|s| s.input_tokens).sum();
            let total_output_tokens: u64 = group.iter().map(|s| s.output_tokens).sum();
            let total_input_cost: f64 = group.iter().map(|s| s.input_cost).sum();
            let total_output_cost: f64 = group.iter().map(|s| s.output_cost).sum();
            let total_cost: f64 = group.iter().map(|s| s.total_cost).sum();
            let latencies: Vec<f64> = group.iter().filter_map(|s| s.latency_ms).collect();
            let avg_latency_ms = if latencies.is_empty() {
                None
            } else {
                Some(latencies.iter().sum::<f64>() / latencies.len() as f64)
            };
            ProviderCostAggregate {
                provider,
                total_input_tokens,
                total_output_tokens,
                total_input_cost,
                total_output_cost,
                total_cost,
                call_count: group.len(),
                avg_latency_ms,
            }
        })
        .collect();
    result.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap());
    result
}

/// Build pricing audits from raw harness records.
pub fn build_pricing_audits(
    records: &[RawHarnessRecord],
    price_map: &[ModelPricing],
    on_unpriced: OnUnpricedAction,
) -> Vec<PricingAudit> {
    let price_lookup: std::collections::HashMap<(String, String), ModelPricing> = price_map
        .iter()
        .map(|p| ((p.provider.clone(), p.model.clone()), p.clone()))
        .collect();

    let mut audits = Vec::new();
    for record in records {
        let key = (record.provider.clone(), record.model.clone());
        let pricing = price_lookup.get(&key);
        let (total_cost, input_cost, output_cost) = match pricing {
            Some(p) => {
                let ic = (record.input_tokens as f64 / 1_000_000.0) * p.input_per_m;
                let oc = (record.output_tokens as f64 / 1_000_000.0) * p.output_per_m;
                (ic + oc, ic, oc)
            }
            None => match on_unpriced {
                OnUnpricedAction::Error => {
                    continue;
                }
                _ => (0.0, 0.0, 0.0),
            },
        };
        audits.push(PricingAudit {
            provider: record.provider.clone(),
            model: record.model.clone(),
            input_tokens: record.input_tokens,
            output_tokens: record.output_tokens,
            input_cost,
            output_cost,
            total_cost,
            latency_ms: record.latency_ms,
            timestamp: record.timestamp,
            provider_price_per_m: pricing.cloned(),
        });
    }
    audits
}
