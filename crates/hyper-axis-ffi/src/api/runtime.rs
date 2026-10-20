//! Runtime, decimal, and sanitized diagnostic bridge operations.

use flutter_rust_bridge::frb;

/// Initializes utilities required by the generated bridge runtime.
#[frb(init)]
pub fn initialize_bridge() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// Returns the semantic version reported by `hyper-axis-core`.
#[frb(sync)]
#[must_use]
pub fn protocol_version() -> String {
    crate::protocol_version()
}

/// Canonicalizes an exact protocol decimal without floating-point conversion.
///
/// # Errors
///
/// Returns a sanitized validation error for malformed decimal syntax.
#[frb(sync)]
pub fn normalize_decimal(value: String) -> Result<String, String> {
    crate::normalize_decimal(value)
}

/// Applies the bounded, process-local Rust diagnostic configuration.
///
/// # Errors
///
/// Returns a sanitized error for an invalid log level or capacity.
#[frb(sync)]
pub fn configure_logging_json(
    enabled: bool,
    level: String,
    capacity: u32,
) -> Result<String, String> {
    crate::configure_logging(enabled, level, capacity)
}

/// Returns retained sanitized diagnostics as JSON without removing them.
///
/// # Errors
///
/// Returns an error when serialization unexpectedly fails.
#[frb(sync)]
pub fn snapshot_logs_json() -> Result<String, String> {
    crate::snapshot_logs_json()
}

/// Returns and removes retained sanitized diagnostics as JSON.
///
/// # Errors
///
/// Returns an error when serialization unexpectedly fails.
#[frb(sync)]
pub fn drain_logs_json() -> Result<String, String> {
    crate::drain_logs_json()
}

/// Clears retained diagnostics and resets the dropped-record counter.
#[frb(sync)]
pub fn clear_logs() {
    crate::clear_logs();
}
