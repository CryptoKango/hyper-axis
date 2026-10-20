/// Market family controlling price decimal precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarketKind {
    /// Regular or HIP-3 perpetual market.
    Perpetual,
    /// Spot market.
    Spot,
}

impl MarketKind {
    const fn max_decimal_places(self) -> u8 {
        match self {
            Self::Perpetual => 6,
            Self::Spot => 8,
        }
    }
}

/// Tick and lot precision obtained from exchange metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketPrecision {
    kind: MarketKind,
    sz_decimals: u8,
}

impl MarketPrecision {
    /// Creates a validated precision configuration.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError::InvalidMarketPrecision`] when `sz_decimals`
    /// exceeds the market family's decimal capacity.
    pub fn new(kind: MarketKind, sz_decimals: u8) -> Result<Self, ProtocolError> {
        let max = kind.max_decimal_places();
        if sz_decimals > max {
            return Err(ProtocolError::InvalidMarketPrecision { sz_decimals, max });
        }
        Ok(Self { kind, sz_decimals })
    }

    /// Validates a price and returns its canonical wire value.
    ///
    /// # Errors
    ///
    /// Returns a decimal or precision error when the price is non-positive,
    /// malformed, or exceeds Hyperliquid tick constraints.
    pub fn price(self, value: impl AsRef<str>) -> Result<DecimalValue, ProtocolError> {
        let value = DecimalValue::parse(value)?;
        if !value.is_positive() {
            return Err(precision_error(
                "price",
                &value,
                "must be greater than zero",
            ));
        }
        if value.significant_digits() > MAX_SIGNIFICANT_PRICE_DIGITS {
            return Err(precision_error(
                "price",
                &value,
                "must contain at most 5 significant figures",
            ));
        }
        let max_scale = usize::from(self.kind.max_decimal_places() - self.sz_decimals);
        if value.scale() > max_scale {
            return Err(precision_error(
                "price",
                &value,
                &format!("must contain at most {max_scale} decimal places"),
            ));
        }
        Ok(value)
    }

    /// Validates an order size and returns its canonical wire value.
    ///
    /// # Errors
    ///
    /// Returns a decimal or precision error when the size is non-positive,
    /// malformed, or has more fractional digits than `sz_decimals`.
    pub fn size(self, value: impl AsRef<str>) -> Result<DecimalValue, ProtocolError> {
        let value = DecimalValue::parse(value)?;
        if !value.is_positive() {
            return Err(precision_error("size", &value, "must be greater than zero"));
        }
        let max_scale = usize::from(self.sz_decimals);
        if value.scale() > max_scale {
            return Err(precision_error(
                "size",
                &value,
                &format!("must contain at most {max_scale} decimal places"),
            ));
        }
        Ok(value)
    }
}

fn precision_error(field: &'static str, value: &DecimalValue, reason: &str) -> ProtocolError {
    ProtocolError::Precision {
        field,
        value: value.to_string(),
        reason: reason.to_owned(),
    }
}

#[cfg(test)]
mod tests;
