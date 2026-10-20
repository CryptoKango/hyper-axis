/// An exact decimal represented without floating-point conversion.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecimalValue(String);

impl DecimalValue {
    /// Parses and canonicalizes a decimal string.
    ///
    /// Leading integer zeros and trailing fractional zeros are removed. A
    /// negative zero is canonicalized to `0`. Exponents and a leading plus
    /// sign are intentionally rejected because wire values do not use them.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError::InvalidDecimal`] when the input is empty,
    /// contains invalid characters, or uses unsupported exponent syntax.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, ProtocolError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(invalid(value, "value is empty"));
        }

        let (negative, unsigned) = value
            .strip_prefix('-')
            .map_or((false, value), |rest| (true, rest));
        if unsigned.is_empty() {
            return Err(invalid(value, "missing digits"));
        }
        if unsigned.starts_with('+') || unsigned.contains(['e', 'E']) {
            return Err(invalid(
                value,
                "sign prefixes and exponents are unsupported",
            ));
        }

        let mut pieces = unsigned.split('.');
        let integer = pieces.next().unwrap_or_default();
        let fraction = pieces.next();
        if pieces.next().is_some() {
            return Err(invalid(value, "more than one decimal point"));
        }
        if integer.is_empty() || !integer.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(invalid(value, "invalid integer digits"));
        }
        if let Some(fraction) = fraction
            && (fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit()))
        {
            return Err(invalid(value, "invalid fractional digits"));
        }

        let integer = integer.trim_start_matches('0');
        let integer = if integer.is_empty() { "0" } else { integer };
        let fraction = fraction.unwrap_or_default().trim_end_matches('0');
        let is_zero = integer == "0" && fraction.is_empty();

        let mut canonical = String::with_capacity(value.len());
        if negative && !is_zero {
            canonical.push('-');
        }
        canonical.push_str(integer);
        if !fraction.is_empty() {
            canonical.push('.');
            canonical.push_str(fraction);
        }
        Ok(Self(canonical))
    }

    /// Returns the canonical wire value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Number of digits after the decimal point.
    #[must_use]
    pub fn scale(&self) -> usize {
        self.0
            .split_once('.')
            .map_or(0, |(_, fraction)| fraction.len())
    }

    /// Number of significant base-10 digits.
    #[must_use]
    pub fn significant_digits(&self) -> usize {
        let unsigned = self.0.strip_prefix('-').unwrap_or(&self.0);
        let digits = unsigned
            .bytes()
            .filter(u8::is_ascii_digit)
            .skip_while(|byte| *byte == b'0')
            .count();
        digits.max(1)
    }

    /// Whether the value is strictly greater than zero.
    #[must_use]
    pub fn is_positive(&self) -> bool {
        self.0 != "0" && !self.0.starts_with('-')
    }

    /// Converts this decimal to a signed fixed-scale protocol integer without
    /// rounding or truncation.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError::Precision`] when the decimal has more
    /// fractional digits than `scale` or the scaled value exceeds `i64`.
    pub fn to_scaled_i64(&self, field: &'static str, scale: u8) -> Result<i64, ProtocolError> {
        let (negative, unsigned) = self
            .0
            .strip_prefix('-')
            .map_or((false, self.0.as_str()), |rest| (true, rest));
        let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));
        if fraction.len() > usize::from(scale) {
            return Err(ProtocolError::Precision {
                field,
                value: self.0.clone(),
                reason: format!("must have at most {scale} fractional digits"),
            });
        }

        let mut scaled =
            String::with_capacity(usize::from(negative) + integer.len() + usize::from(scale));
        if negative {
            scaled.push('-');
        }
        scaled.push_str(integer);
        scaled.push_str(fraction);
        scaled.extend(std::iter::repeat_n(
            '0',
            usize::from(scale) - fraction.len(),
        ));
        scaled.parse().map_err(|_| ProtocolError::Precision {
            field,
            value: self.0.clone(),
            reason: "scaled integer exceeds signed 64-bit range".to_owned(),
        })
    }

    /// Converts a positive decimal to an unsigned fixed-scale protocol integer
    /// without rounding or truncation.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError::Precision`] when the value is not positive,
    /// has more fractional digits than `scale`, or exceeds `u64`.
    pub fn to_scaled_u64(&self, field: &'static str, scale: u8) -> Result<u64, ProtocolError> {
        if !self.is_positive() {
            return Err(ProtocolError::Precision {
                field,
                value: self.0.clone(),
                reason: "must be positive".to_owned(),
            });
        }
        let (integer, fraction) = self.0.split_once('.').unwrap_or((&self.0, ""));
        if fraction.len() > usize::from(scale) {
            return Err(ProtocolError::Precision {
                field,
                value: self.0.clone(),
                reason: format!("must have at most {scale} fractional digits"),
            });
        }

        let mut scaled = String::with_capacity(integer.len() + usize::from(scale));
        scaled.push_str(integer);
        scaled.push_str(fraction);
        scaled.extend(std::iter::repeat_n(
            '0',
            usize::from(scale) - fraction.len(),
        ));
        scaled.parse().map_err(|_| ProtocolError::Precision {
            field,
            value: self.0.clone(),
            reason: "scaled integer exceeds unsigned 64-bit range".to_owned(),
        })
    }
}

impl fmt::Display for DecimalValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for DecimalValue {
    type Err = ProtocolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Serialize for DecimalValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for DecimalValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(de::Error::custom)
    }
}

fn invalid(value: &str, reason: &'static str) -> ProtocolError {
    ProtocolError::InvalidDecimal {
        value: value.to_owned(),
        reason,
    }
}

#[cfg(test)]
mod tests;
