// Caching and aggregation logic for coverage reports and unpriced events

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use anyhow::Result;
use chrono::Datelike;

use crate::cli::IngestProvider;
use crate::models::*;

pub fn build_coverage_report(events: &[UsageEvent], pricing: &PricingBook) -> CoverageReport {
    let mut missing_providers = Vec::new();
    let mut missing_models_by_provider: BTreeMap<String, HashSet<String>> = BTreeMap::new();
    let mut model_counts_by_provider: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut priced_count = 0;
    let mut unpriced_count = 0;

    for event in events {
        let provider_name = resolve_provider_alias(&event.provider, pricing);
        if !pricing.providers.contains_key(&provider_name) {
            if !missing_providers.contains(&provider_name) {
                missing_providers.push(provider_name.clone());
            }
            unpriced_count += 1;
            continue;
        }

        let provider = &pricing.providers[&provider_name];
        let model_name = resolve_model_alias(&provider_name, &event.model, pricing);
        if !provider.models.contains_key(&model_name) {
            missing_models_by_provider
                .entry(provider_name.clone())
                .or_insert_with(HashSet::new)
                .insert(model_name.clone());
            model_counts_by_provider
                .entry(provider_name.clone())
                .or_insert_with(BTreeMap::new)
                .entry(model_name)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            unpriced_count += 1;
        } else {
            priced_count += 1;
        }
    }

    // Convert HashSets to sorted Vecs for missing_models_by_provider
    let missing_models_by_provider_vecs = missing_models_by_provider
        .into_iter()
        .map(|(k, v)| {
            let mut sorted: Vec<String> = v.into_iter().collect();
            sorted.sort();
            (k, sorted)
        })
        .collect();

    // Build suggested provider aliases for missing providers
    let mut suggested_provider_aliases: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for missing_provider in &missing_providers {
        let mut suggestions: Vec<String> = pricing.provider_aliases.keys().cloned().collect();
        if !suggestions.is_empty() {
            suggestions.sort();
            suggested_provider_aliases.insert(missing_provider.clone(), suggestions);
        }
    }

    // Build suggested model aliases by provider
    let suggested_model_aliases_by_provider: BTreeMap<String, Vec<UnknownModelSuggestion>> =
        model_counts_by_provider
            .into_iter()
            .map(|(provider, model_counts)| {
                let mut suggestions: Vec<UnknownModelSuggestion> = model_counts
                    .into_iter()
                    .map(|(model, count)| UnknownModelSuggestion { model, count })
                    .collect();
                suggestions.sort_by_key(|s| std::cmp::Reverse(s.count));
                (provider, suggestions)
            })
            .collect();

    let month = if events.is_empty() {
        "0000-00".to_string()
    } else {
        format!(
            "{:04}-{:02}",
            events[0].timestamp.year(),
            events[0].timestamp.month()
        )
    };

    let mut totals_tokens = 0u64;
    for event in events {
        totals_tokens += event.usage.total();
    }

    CoverageReport {
        month,
        totals: crate::models::CoverageTotals {
            events: events.len(),
            tokens: totals_tokens,
        },
        priced_count,
        unpriced_count,
        missing_providers,
        missing_models_by_provider: missing_models_by_provider_vecs,
        suggested_provider_aliases,
        suggested_model_aliases_by_provider,
    }
}

pub fn collect_unpriced_events(events: &[UsageEvent], pricing: &PricingBook) -> Vec<UsageEvent> {
    events
        .iter()
        .filter(|event| {
            let provider_name = resolve_provider_alias(&event.provider, pricing);
            if !pricing.providers.contains_key(&provider_name) {
                return true;
            }
            let provider = &pricing.providers[&provider_name];
            let model_name = resolve_model_alias(&provider_name, &event.model, pricing);
            !provider.models.contains_key(&model_name)
        })
        .cloned()
        .collect()
}

pub fn maybe_write_unpriced_outputs(
    _events: &[UsageEvent],
    _unpriced: &[UsageEvent],
    _pricing: &PricingBook,
    patch_path: Option<&Path>,
    unpriced_events_path: Option<&Path>,
) -> Result<()> {
    if patch_path.is_some() || unpriced_events_path.is_some() {
        // TODO: implement patch writing and unpriced events output
    }
    Ok(())
}

pub fn resolve_ingest_providers(
    providers: &[IngestProvider],
) -> Vec<IngestProvider> {
    if providers.is_empty() {
        vec![
            IngestProvider::Claude,
            IngestProvider::Codex,
            IngestProvider::Proxyapi,
            IngestProvider::Cursor,
            IngestProvider::Droid,
        ]
    } else {
        providers.to_vec()
    }
}

pub fn resolve_provider_alias(name: &str, pricing: &PricingBook) -> String {
    pricing
        .provider_aliases
        .get(name)
        .cloned()
        .unwrap_or_else(|| name.to_string())
}

pub fn resolve_model_alias(provider: &str, model: &str, pricing: &PricingBook) -> String {
    pricing
        .providers
        .get(provider)
        .and_then(|p| p.model_aliases.get(model).cloned())
        .unwrap_or_else(|| model.to_string())
}
