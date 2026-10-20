/// Signer-local nonce handle intended to be owned by one Dart account session.
///
/// Do not share one handle between different master accounts or API wallets.
#[derive(Debug)]
pub struct FfiNonceManager {
    inner: NonceManager,
}

impl FfiNonceManager {
    /// Creates a manager, optionally restoring its last issued nonce.
    #[must_use]
    pub fn new(last_issued: Option<u64>) -> Self {
        Self {
            inner: NonceManager::new(last_issued),
        }
    }

    /// Issues the next monotonic nonce for this signer.
    ///
    /// # Errors
    ///
    /// Returns an error if the unsigned 64-bit nonce space is exhausted.
    pub fn next(&self, now_ms: u64) -> Result<u64, String> {
        self.inner.next(now_ms).map_err(|error| error.to_string())
    }

    /// Advances the persisted high-water mark without moving it backwards.
    pub fn fast_forward(&self, last_issued: u64) {
        self.inner.fast_forward(last_issued);
    }

    /// Returns the current high-water mark.
    #[must_use]
    pub fn last_issued(&self) -> Option<u64> {
        self.inner.last_issued()
    }
}

/// Checks the documented nonce time window before request submission.
///
/// # Errors
///
/// Returns an error when `nonce` is too old or too far in the future relative
/// to `now_ms`. The server remains authoritative.
pub fn validate_exchange_nonce(nonce: u64, now_ms: u64) -> Result<(), String> {
    validate_nonce_window(nonce, now_ms).map_err(|error| error.to_string())
}

/// Returns the protocol core version.
#[must_use]
pub fn protocol_version() -> String {
    hyper_axis_core::VERSION.to_owned()
}

/// Canonicalizes an exact decimal without passing through a floating point
/// representation in Dart or native code.
///
/// # Errors
///
/// Returns a display-safe validation message when `value` is not a supported
/// protocol decimal.
pub fn normalize_decimal(value: String) -> Result<String, String> {
    DecimalValue::parse(value)
        .map(|value| value.to_string())
        .map_err(|error| error.to_string())
}
