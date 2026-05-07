//! ParetoRs — pure data types shared by cost + pricing + format engines.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ─── Core Pricing Models ──────────────────────────────────────────────────────

/// Provider identifier (e.g. "openai/gpt-4o", "anthropic/claude-3-5-sonnet")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderKey(pub String);

/// Raw input/output token counts per model call
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TokenCounts {
    pub input: u64,
    pub output: u64,
}

/// Output token cost per million tokens
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PricingRate {
    pub input_per_m: f64,
    pub output_per_m: f64,
    pub use_default: bool,
}

/// Per-model pricing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub provider: String,
    pub model: String,
    pub input_per_m: f64,
    pub output_per_m: f64,
}

/// Provider-level price list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPrices {
    pub provider: String,
    pub models: Vec<ModelPricing>,
    pub updated_at: DateTime<Utc>,
}

// ─── Routing ─────────────────────────────────────────────────────────────────

/// Provider harness metrics used for routing decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHarness {
    pub provider: String,
    pub model: String,
    pub input_cost: f64,
    pub output_cost: f64,
    pub p50_latency_ms: Option<f64>,
    pub p95_latency_ms: Option<f64>,
    pub success_rate: f64,
}

/// Routing criteria — what matters for a given request
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum RoutingCriteria {
    Cost,
    Latency,
    #[default]
    Balanced,
}

impl std::fmt::Display for RoutingCriteria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoutingCriteria::Cost => write!(f, "cost"),
            RoutingCriteria::Latency => write!(f, "latency"),
            RoutingCriteria::Balanced => write!(f, "balanced"),
        }
    }
}

// ─── Audit ───────────────────────────────────────────────────────────────────

/// How to handle unpriced models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, clap::ValueEnum)]
pub enum OnUnpricedAction {
    Skip,
    Error,
    #[default]
    Warn,
}

/// What to do when a provider returns no cost data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MissingCostStrategy {
    #[default]
    Skip,
    UseProviderDefault,
    BestEffort,
}

/// Pricing audit result for a single call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingAudit {
    pub provider: String,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
    pub latency_ms: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub provider_price_per_m: Option<ModelPricing>,
}

/// Pricing lint finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingLint {
    pub severity: LintSeverity,
    pub provider: String,
    pub model: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LintSeverity {
    Error,
    Warn,
    Info,
}

// ─── Ledger / Cost Snapshot ───────────────────────────────────────────────────

/// Snapshot of costs for one call record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSnapshot {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
    pub latency_ms: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub routing_criteria: Option<String>,
    pub routing_score: Option<f64>,
}

/// Aggregated cost across all calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAggregate {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_input_cost: f64,
    pub total_output_cost: f64,
    pub total_cost: f64,
    pub call_count: usize,
}

/// Per-provider aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCostAggregate {
    pub provider: String,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_input_cost: f64,
    pub total_output_cost: f64,
    pub total_cost: f64,
    pub call_count: usize,
    pub avg_latency_ms: Option<f64>,
}

// ─── Format ─────────────────────────────────────────────────────────────────

/// Output format for pricing commands
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Csv,
    Markdown,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Csv => write!(f, "csv"),
            OutputFormat::Markdown => write!(f, "markdown"),
        }
    }
}
