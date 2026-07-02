//! ParetoRs — pure utility functions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Raw harness record from a run log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawHarnessRecord {
    pub provider: String,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub latency_ms: Option<f64>,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
}

/// Parse a CSV row of harness records (provider,model,itokens,otokens,latency,success,timestamp).
pub fn parse_harness_csv_line(line: &str) -> Option<RawHarnessRecord> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 6 {
        return None;
    }
    let provider = parts[0].trim().to_string();
    let model = parts[1].trim().to_string();
    let input_tokens = parts[2].trim().parse().ok()?;
    let output_tokens = parts[3].trim().parse().ok()?;
    let latency_ms = parts[4].trim().parse().ok();
    let success = parts[5].trim().parse().ok()?;
    let timestamp = if parts.len() > 6 {
        DateTime::parse_from_rfc3339(parts[6].trim())
            .map(|dt| dt.with_timezone(&Utc))
            .ok()
    } else {
        Some(Utc::now())
    }?;
    Some(RawHarnessRecord {
        provider,
        model,
        input_tokens,
        output_tokens,
        latency_ms,
        success,
        timestamp,
    })
}

/// Format cost as a dollar string.
pub fn format_cost(cost: f64) -> String {
    if cost < 0.001 {
        format!("${:.6}", cost)
    } else if cost < 1.0 {
        format!("${:.4}", cost)
    } else {
        format!("${:.2}", cost)
    }
}

/// Format a percentage (0-100).
pub fn format_pct(pct: f64) -> String {
    format!("{:.2}%", pct)
}
