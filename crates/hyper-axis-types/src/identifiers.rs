/// Integer identifier accepted by Hyperliquid exchange actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssetId(u32);

impl AssetId {
    /// Creates an asset identifier from its wire representation.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the integer placed in the action's `a` field.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Canonical 20-byte EVM address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvmAddress([u8; 20]);

impl EvmAddress {
    /// Creates an address from its exact byte representation.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    /// Returns the exact 20 address bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

impl FromStr for EvmAddress {
    type Err = ProtocolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let unprefixed = value.strip_prefix("0x").unwrap_or(value);
        if unprefixed.len() != 40 {
            return Err(ProtocolError::InvalidAddress(
                "expected exactly 20 hexadecimal bytes",
            ));
        }
        let decoded = hex::decode(unprefixed)
            .map_err(|_| ProtocolError::InvalidAddress("contains non-hexadecimal characters"))?;
        let bytes = decoded
            .try_into()
            .map_err(|_| ProtocolError::InvalidAddress("expected exactly 20 hexadecimal bytes"))?;
        Ok(Self(bytes))
    }
}

impl fmt::Display for EvmAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "0x{}", hex::encode(self.0))
    }
}

impl Serialize for EvmAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EvmAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(de::Error::custom)
    }
}

/// Canonical 128-bit client order identifier (`cloid`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientOrderId([u8; 16]);

impl ClientOrderId {
    /// Creates an identifier from its exact 16-byte representation.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Returns the exact 16 identifier bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl FromStr for ClientOrderId {
    type Err = ProtocolError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let unprefixed = value.strip_prefix("0x").unwrap_or(value);
        if unprefixed.len() != 32 {
            return Err(ProtocolError::InvalidClientOrderId(
                "expected exactly 16 hexadecimal bytes",
            ));
        }
        let decoded = hex::decode(unprefixed).map_err(|_| {
            ProtocolError::InvalidClientOrderId("contains non-hexadecimal characters")
        })?;
        let bytes = decoded.try_into().map_err(|_| {
            ProtocolError::InvalidClientOrderId("expected exactly 16 hexadecimal bytes")
        })?;
        Ok(Self(bytes))
    }
}

impl fmt::Display for ClientOrderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "0x{}", hex::encode(self.0))
    }
}

impl Serialize for ClientOrderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ClientOrderId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(de::Error::custom)
    }
}
