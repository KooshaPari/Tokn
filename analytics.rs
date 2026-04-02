//! Analytics integration for Tokn
//!
//! Traces to: FR-TOKN-ANALYTICS-001
//!
//! Product analytics for token ledger and blockchain operations

use phenotype_analytics::{AnalyticsClient, AnalyticsConfig, EventType};
use std::sync::OnceLock;

static ANALYTICS: OnceLock<AnalyticsClient> = OnceLock::new();

/// Initialize analytics for Tokn
pub fn init() -> anyhow::Result<()> {
    let api_key = std::env::var("PHENOTYPE_ANALYTICS_KEY").ok();
    
    if api_key.is_none() {
        return Ok(());
    }
    
    let config = AnalyticsConfig {
        api_key: api_key.unwrap(),
        environment: std::env::var("PHENOTYPE_ENV")
            .unwrap_or_else(|_| "development".to_string()),
        version: env!("CARGO_PKG_VERSION").to_string(),
        ..Default::default()
    };
    
    let client = AnalyticsClient::new(config)?;
    let _ = ANALYTICS.set(client);
    
    Ok(())
}

/// Track token transfer
pub async fn track_transfer(
    token_id: &str,
    from: &str,
    to: &str,
    amount: u64,
    tx_hash: &str,
) {
    if let Some(client) = ANALYTICS.get() {
        let _ = client.track(
            EventType::FeatureUsed,
            serde_json::json!({
                "feature": "token_transfer",
                "token_id": token_id,
                "from": from,
                "to": to,
                "amount": amount,
                "tx_hash": tx_hash,
            }),
        ).await;
    }
}

/// Track token minting
pub async fn track_mint(token_id: &str, amount: u64, to: &str) {
    if let Some(client) = ANALYTICS.get() {
        let _ = client.track(
            EventType::FeatureUsed,
            serde_json::json!({
                "feature": "token_mint",
                "token_id": token_id,
                "amount": amount,
                "recipient": to,
            }),
        ).await;
    }
}

/// Track ledger operation
pub async fn track_ledger_operation(
    operation: &str,
    aggregate_id: &str,
    duration_ms: u64,
    success: bool,
) {
    if let Some(client) = ANALYTICS.get() {
        let event_type = if success {
            EventType::OperationCompleted
        } else {
            EventType::ErrorOccurred
        };
        
        let _ = client.track(
            event_type,
            serde_json::json!({
                "operation": operation,
                "aggregate_id": aggregate_id,
                "duration_ms": duration_ms,
                "category": "ledger",
            }),
        ).await;
    }
}

/// Track wallet connection
pub async fn track_wallet_connected(wallet_address: &str, wallet_type: &str) {
    if let Some(client) = ANALYTICS.get() {
        let _ = client.identify(
            wallet_address,
            serde_json::json!({
                "wallet_type": wallet_type,
                "connected_at": chrono::Utc::now().to_rfc3339(),
            }),
        ).await;
    }
}

/// Flush analytics before shutdown
pub async fn flush() {
    if let Some(client) = ANALYTICS.get() {
        let _ = client.flush().await;
    }
}
