/// EIP-712 domain metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningDomain {
    /// Domain name.
    pub name: &'static str,
    /// Domain version.
    pub version: &'static str,
    /// EIP-712 chain identifier.
    pub chain_id: u64,
    /// Verifying contract address.
    pub verifying_contract: &'static str,
}

/// Phantom agent signed for an L1 exchange action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhantomAgent {
    /// `a` on mainnet and `b` on testnet.
    pub source: &'static str,
    /// Keccak action hash.
    pub connection_id: [u8; 32],
}

/// Canonical ECDSA signature accepted by the exchange endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireSignature {
    r: String,
    s: String,
    v: u8,
}

impl WireSignature {
    /// Validates and canonicalizes signature components.
    ///
    /// Recovery IDs `0` and `1` are accepted from platform signers and
    /// normalized to the exchange wire values `27` and `28`.
    ///
    /// # Errors
    ///
    /// Returns an error when a scalar is not exactly 32 hexadecimal bytes, the
    /// signature is not valid secp256k1, `s` is not low, or `v` is unsupported.
    pub fn new(r: impl AsRef<str>, s: impl AsRef<str>, v: u8) -> Result<Self, SigningError> {
        let r = decode_scalar("r", r.as_ref())?;
        let s = decode_scalar("s", s.as_ref())?;
        let signature = signature_from_components(r, s)?;
        if signature.normalize_s() != signature {
            return Err(SigningError::HighS);
        }
        let v = normalize_v(v)?;
        Ok(Self {
            r: format!("0x{}", hex::encode(r)),
            s: format!("0x{}", hex::encode(s)),
            v,
        })
    }

    /// Canonical `r` scalar with `0x` prefix.
    #[must_use]
    pub fn r(&self) -> &str {
        &self.r
    }

    /// Canonical `s` scalar with `0x` prefix.
    #[must_use]
    pub fn s(&self) -> &str {
        &self.s
    }

    /// Recovery value normalized to `27` or `28`.
    #[must_use]
    pub const fn v(&self) -> u8 {
        self.v
    }

    /// Recovers the signer address from an already-computed EIP-712 digest.
    ///
    /// # Errors
    ///
    /// Returns an error when the stored signature cannot recover a valid
    /// secp256k1 public key.
    pub fn recover_address(&self, digest: [u8; 32]) -> Result<EvmAddress, SigningError> {
        let r = decode_scalar("r", &self.r)?;
        let s = decode_scalar("s", &self.s)?;
        let signature = signature_from_components(r, s)?;
        let recovery_id =
            RecoveryId::from_byte(self.v - 27).ok_or(SigningError::InvalidV(self.v))?;
        let verifying_key = VerifyingKey::recover_from_prehash(&digest, &signature, recovery_id)
            .map_err(|_| SigningError::RecoveryFailed)?;
        Ok(address_from_verifying_key(&verifying_key))
    }

    /// Recovers and compares the signer with an expected address.
    ///
    /// # Errors
    ///
    /// Returns an error when recovery fails or the recovered address differs
    /// from `expected`.
    pub fn verify_signer(
        &self,
        digest: [u8; 32],
        expected: EvmAddress,
    ) -> Result<EvmAddress, SigningError> {
        let recovered = self.recover_address(digest)?;
        if recovered != expected {
            return Err(SigningError::SignerMismatch);
        }
        Ok(recovered)
    }
}

impl<'de> Deserialize<'de> for WireSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Components {
            r: String,
            s: String,
            v: u8,
        }

        let components = Components::deserialize(deserializer)?;
        Self::new(components.r, components.s, components.v).map_err(de::Error::custom)
    }
}

/// Signature validation and recovery failures.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SigningError {
    /// Scalar has an invalid hexadecimal representation or length.
    #[error("invalid signature scalar {field}: {reason}")]
    InvalidScalar {
        /// `r` or `s`.
        field: &'static str,
        /// Safe validation message.
        reason: &'static str,
    },
    /// Scalar pair is outside secp256k1's valid range.
    #[error("invalid secp256k1 signature scalars")]
    InvalidSignature,
    /// Non-canonical high-S signature.
    #[error("signature uses non-canonical high-S form")]
    HighS,
    /// Recovery value is unsupported.
    #[error("invalid signature recovery value {0}; expected 0, 1, 27, or 28")]
    InvalidV(u8),
    /// Public-key recovery failed.
    #[error("unable to recover signer from signature")]
    RecoveryFailed,
    /// Recovered signer does not match the expected address.
    #[error("recovered signer does not match expected signer")]
    SignerMismatch,
}

/// Returns the domain used by exchange L1 actions.
#[must_use]
pub const fn l1_domain() -> SigningDomain {
    SigningDomain {
        name: "Exchange",
        version: "1",
        chain_id: L1_SIGNATURE_CHAIN_ID,
        verifying_contract: ZERO_ADDRESS,
    }
}

/// Constructs the typed phantom-agent message around an action hash.
#[must_use]
pub const fn phantom_agent(network: Network, connection_id: [u8; 32]) -> PhantomAgent {
    PhantomAgent {
        source: network.l1_source(),
        connection_id,
    }
}
