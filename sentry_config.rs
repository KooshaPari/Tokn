//! Sentry configuration for Tokn
//!
//! Traces to: FR-TOKN-SENTRY-001
//!
//! Error tracking for token ledger operations

use std::env;

/// Initialize Sentry for blockchain/ledger operations
pub fn init() -> Option<sentry::ClientInitGuard> {
    let dsn = env::var("SENTRY_DSN").ok()?;
    let environment = env::var("SENTRY_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    let release = format!("{}@{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    Some(sentry::init((
        dsn,
        sentry::ClientOptions {
            environment: Some(environment.into()),
            release: Some(release.into()),
            attach_stacktrace: true,
            debug: cfg!(debug_assertions),
            max_breadcrumbs: 50,
            
            // High sample rate for financial operations
            sample_rate: 1.0,
            traces_sample_rate: 0.2,
            
            before_send: Some(std::sync::Arc::new(|mut event| {
                // Scrub sensitive blockchain data
                if let Some(ref mut contexts) = event.contexts {
                    if let Some(ref mut request) = contexts.get_mut("request") {
                        if let Some(ref mut data) = request.data {
                            // Remove private keys if accidentally logged
                            if data.as_str().map(|s| s.contains("private_key")).unwrap_or(false) {
                                request.data = Some("<redacted>".into());
                            }
                        }
                    }
                }
                Some(event)
            })),
            
            ..Default::default()
        },
    )))
}

/// Capture ledger operation error
pub fn capture_ledger_error(
    error: &impl std::error::Error,
    operation: &str,
    token_id: Option<&str>,
) {
    sentry::with_scope(
        |scope| {
            scope.set_tag("ledger.operation", operation);
            scope.set_tag("ledger.token_id", token_id.unwrap_or("unknown"));
            scope.set_level(Some(sentry::Level::Error));
        },
        || sentry::capture_error(error),
    );
}

/// Track transaction for debugging
pub fn track_transaction(tx_hash: &str, operation: &str) {
    sentry::add_breadcrumb(sentry::Breadcrumb {
        message: Some(format!("Transaction {}: {}", operation, tx_hash)),
        category: Some("transaction".into()),
        level: sentry::Level::Info,
        ..Default::default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentry_initialization() {
        let guard = init();
        assert!(guard.is_none()); // No DSN in test
    }
}
