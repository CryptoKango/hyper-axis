/// Thread-safe, monotonically increasing nonce source for one signer.
#[derive(Debug)]
pub struct NonceManager {
    last_issued: AtomicU64,
}

impl NonceManager {
    /// Creates a signer-local manager.
    ///
    /// `last_issued` should be restored from durable host storage after an app
    /// restart when available. Passing `None` starts from the next supplied
    /// wall-clock timestamp.
    #[must_use]
    pub fn new(last_issued: Option<u64>) -> Self {
        Self {
            last_issued: AtomicU64::new(last_issued.unwrap_or(0)),
        }
    }

    /// Issues `max(now_ms, previous + 1)` atomically.
    ///
    /// The caller supplies Unix time in milliseconds. Keeping time acquisition
    /// in the host makes this core deterministic and testable on native and Web.
    ///
    /// # Errors
    ///
    /// Returns [`NonceError::Exhausted`] only after `u64::MAX` was issued.
    pub fn next(&self, now_ms: u64) -> Result<u64, NonceError> {
        let issued = self
            .last_issued
            .try_update(Ordering::AcqRel, Ordering::Acquire, |previous| {
                previous
                    .checked_add(1)
                    .map(|incremented| incremented.max(now_ms))
            })
            .map_err(|_| NonceError::Exhausted)?;
        let nonce = issued
            .checked_add(1)
            .map(|incremented| incremented.max(now_ms))
            .ok_or(NonceError::Exhausted)?;
        logging::emit(
            LogLevel::Trace,
            "nonce.issue",
            "issued signer-local monotonic nonce",
        );
        Ok(nonce)
    }

    /// Advances the stored high-water mark without ever moving it backwards.
    ///
    /// This is useful after restoring a persisted value or observing that a
    /// different process has already consumed a higher nonce for the signer.
    pub fn fast_forward(&self, last_issued: u64) {
        self.last_issued.fetch_max(last_issued, Ordering::AcqRel);
        logging::emit(
            LogLevel::Debug,
            "nonce.fast_forward",
            "advanced signer-local nonce high-water mark",
        );
    }

    /// Returns the current high-water mark, or `None` before the first issue.
    #[must_use]
    pub fn last_issued(&self) -> Option<u64> {
        match self.last_issued.load(Ordering::Acquire) {
            0 => None,
            value => Some(value),
        }
    }
}

impl Default for NonceManager {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Validates Hyperliquid's documented server-time acceptance window.
///
/// This is a preflight check only. The server remains authoritative because
/// client and server clocks may differ.
///
/// # Errors
///
/// Returns whether the nonce is older than two days or more than one day in
/// the future relative to `now_ms`.
pub const fn validate_nonce_window(nonce: u64, now_ms: u64) -> Result<(), NonceError> {
    if nonce < now_ms.saturating_sub(MAX_PAST_AGE_MS) {
        return Err(NonceError::TooOld);
    }
    if nonce > now_ms.saturating_add(MAX_FUTURE_OFFSET_MS) {
        return Err(NonceError::TooFarInFuture);
    }
    Ok(())
}
