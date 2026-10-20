/// Failures emitted by nonce coordination and preflight validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum NonceError {
    /// No larger value can be represented.
    #[error("nonce space exhausted")]
    Exhausted,
    /// Nonce is outside the accepted historical window.
    #[error("nonce is older than the allowed two-day window")]
    TooOld,
    /// Nonce is outside the accepted future window.
    #[error("nonce is more than one day ahead of current time")]
    TooFarInFuture,
}

#[cfg(test)]
mod tests;
