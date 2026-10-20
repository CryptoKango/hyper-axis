/// Errors produced while constructing protocol-safe values.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProtocolError {
    /// A decimal string is syntactically invalid.
    #[error("invalid decimal `{value}`: {reason}")]
    InvalidDecimal {
        /// Original value.
        value: String,
        /// Validation failure.
        reason: &'static str,
    },

    /// A numeric value violates market precision constraints.
    #[error("{field} `{value}` violates precision rule: {reason}")]
    Precision {
        /// Logical field being checked.
        field: &'static str,
        /// Canonical decimal value.
        value: String,
        /// Rule that was violated.
        reason: String,
    },

    /// An asset index cannot be represented by the protocol formula.
    #[error("asset id overflow for {kind} index {index}")]
    AssetIdOverflow {
        /// Formula family.
        kind: &'static str,
        /// Supplied index.
        index: u32,
    },

    /// The requested market precision configuration is impossible.
    #[error("invalid market precision: szDecimals {sz_decimals} exceeds {max}")]
    InvalidMarketPrecision {
        /// Market size precision.
        sz_decimals: u8,
        /// Maximum allowed by the market family.
        max: u8,
    },

    /// An EVM address cannot be decoded safely.
    #[error("invalid EVM address: {0}")]
    InvalidAddress(&'static str),

    /// A client order identifier is not an exact 128-bit hex value.
    #[error("invalid client order ID: {0}")]
    InvalidClientOrderId(&'static str),

    /// An action violates a structural invariant.
    #[error("invalid action: {0}")]
    InvalidAction(&'static str),
}

#[cfg(test)]
mod tests;
