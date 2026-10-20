/// Signer-local monotonic nonce source for browser clients.
///
/// Create one instance per master account or API wallet. A signer may reuse
/// the same instance across its vaults and subaccounts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct WasmNonceManager {
    inner: NonceManager,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl WasmNonceManager {
    /// Creates a manager, optionally restoring the last issued nonce string.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the restored value is not a `u64`.
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(last_issued: Option<String>) -> Result<WasmNonceManager, JsError> {
        let last_issued = last_issued
            .map(|value| parse_u64("lastIssued", &value))
            .transpose()?;
        Ok(Self {
            inner: NonceManager::new(last_issued),
        })
    }

    /// Issues the next nonce as a decimal string for JavaScript integer safety.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error for invalid time or nonce exhaustion.
    pub fn next(&self, now_ms: &str) -> Result<String, JsError> {
        let now_ms = parse_u64("nowMs", now_ms)?;
        self.inner
            .next(now_ms)
            .map(|nonce| nonce.to_string())
            .map_err(|error| JsError::new(&error.to_string()))
    }

    /// Advances the high-water mark without moving it backwards.
    ///
    /// # Errors
    ///
    /// Returns a JavaScript error when the value is not a `u64`.
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fastForward))]
    pub fn fast_forward(&self, last_issued: &str) -> Result<(), JsError> {
        let last_issued = parse_u64("lastIssued", last_issued)?;
        self.inner.fast_forward(last_issued);
        Ok(())
    }

    /// Returns the current high-water mark as a decimal string.
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter, js_name = lastIssued))]
    #[must_use]
    pub fn last_issued(&self) -> Option<String> {
        self.inner.last_issued().map(|nonce| nonce.to_string())
    }
}

/// Checks the documented nonce time window before request submission.
///
/// # Errors
///
/// Returns a JavaScript error for invalid integers or a nonce outside the
/// documented server-time window.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn validate_exchange_nonce(nonce: &str, now_ms: &str) -> Result<(), JsError> {
    let nonce = parse_u64("nonce", nonce)?;
    let now_ms = parse_u64("nowMs", now_ms)?;
    validate_nonce_window(nonce, now_ms).map_err(|error| JsError::new(&error.to_string()))
}

/// Returns the protocol core version.
#[must_use]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn protocol_version() -> String {
    hyper_axis_core::VERSION.to_owned()
}

/// Canonicalizes an exact decimal without JavaScript `Number` conversion.
///
/// # Errors
///
/// Returns a JavaScript error when `value` is not a supported protocol decimal.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn normalize_decimal(value: &str) -> Result<String, JsError> {
    DecimalValue::parse(value)
        .map(|value| value.to_string())
        .map_err(|error| JsError::new(&error.to_string()))
}

/// Configures structured diagnostics for a browser client.
///
/// Supported levels are `error`, `warn`, `info`, `debug`, and `trace`.
/// Logging starts disabled and must be explicitly enabled by the application.
///
/// # Errors
///
/// Returns a JavaScript error for an unsupported level, invalid capacity, or
/// JSON serialization failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn configure_logging(enabled: bool, level: &str, capacity: u32) -> Result<String, JsError> {
    let level = LogLevel::from_str(level).map_err(|error| JsError::new(&error.to_string()))?;
    let config = logging::configure(enabled, level, capacity as usize)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&config).map_err(|error| JsError::new(&error.to_string()))
}

/// Returns retained diagnostics as JSON without removing them.
///
/// # Errors
///
/// Returns a JavaScript error only if JSON serialization fails.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn snapshot_logs_json() -> Result<String, JsError> {
    serde_json::to_string(&logging::snapshot()).map_err(|error| JsError::new(&error.to_string()))
}

/// Returns and removes retained diagnostics as JSON.
///
/// # Errors
///
/// Returns a JavaScript error only if JSON serialization fails.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn drain_logs_json() -> Result<String, JsError> {
    serde_json::to_string(&logging::drain()).map_err(|error| JsError::new(&error.to_string()))
}

/// Clears retained diagnostics and the overflow counter.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn clear_logs() {
    logging::clear();
}
