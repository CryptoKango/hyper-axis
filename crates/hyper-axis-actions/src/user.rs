/// Core USDC transfer signed with the user-action EIP-712 domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdSendAction {
    /// Action discriminator. Always `usdSend`.
    #[serde(rename = "type")]
    action_type: UsdSendActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Canonical destination address.
    pub destination: EvmAddress,
    /// Positive canonical USDC amount.
    pub amount: DecimalValue,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub time: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UsdSendActionType {
    #[serde(rename = "usdSend")]
    UsdSend,
}

/// Core spot asset transfer signed with the user-action EIP-712 domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotSendAction {
    /// Action discriminator. Always `spotSend`.
    #[serde(rename = "type")]
    action_type: SpotSendActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Canonical destination address.
    pub destination: EvmAddress,
    /// Hyperliquid token identifier such as `PURR:0x...`.
    pub token: String,
    /// Positive canonical token amount.
    pub amount: DecimalValue,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub time: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SpotSendActionType {
    #[serde(rename = "spotSend")]
    SpotSend,
}

/// Bridge withdrawal signed with the user-action EIP-712 domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawAction {
    /// Action discriminator. Always `withdraw3`.
    #[serde(rename = "type")]
    action_type: WithdrawActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Canonical destination address.
    pub destination: EvmAddress,
    /// Positive canonical withdrawal amount.
    pub amount: DecimalValue,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub time: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum WithdrawActionType {
    #[serde(rename = "withdraw3")]
    Withdraw,
}

/// API wallet authorization signed with the user-action EIP-712 domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveAgentAction {
    /// Action discriminator. Always `approveAgent`.
    #[serde(rename = "type")]
    action_type: ApproveAgentActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// API wallet address receiving authority.
    pub agent_address: EvmAddress,
    /// Optional API wallet name. When absent, EIP-712 still signs an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ApproveAgentActionType {
    #[serde(rename = "approveAgent")]
    ApproveAgent,
}

/// Exact builder fee percentage without floating-point conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuilderFeeRate(DecimalValue);

impl BuilderFeeRate {
    /// Parses a percent string such as `0.001%` and canonicalizes its decimal part.
    ///
    /// Zero is accepted locally as a valid maximum. Negative values, missing
    /// percent suffixes, whitespace, and exponent syntax are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error when the wire value is not a non-negative percent string.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, ProtocolError> {
        let value = value.as_ref();
        let Some(decimal) = value.strip_suffix('%') else {
            return Err(ProtocolError::InvalidAction(
                "builder fee rate must end with a percent sign",
            ));
        };
        if decimal.is_empty() || decimal.trim() != decimal {
            return Err(ProtocolError::InvalidAction(
                "builder fee rate must be an exact percent string without whitespace",
            ));
        }
        let decimal = DecimalValue::parse(decimal)?;
        if decimal.as_str().starts_with('-') {
            return Err(ProtocolError::InvalidAction(
                "builder fee rate must not be negative",
            ));
        }
        Ok(Self(decimal))
    }

    /// Returns the canonical decimal component without the percent suffix.
    #[must_use]
    pub fn decimal(&self) -> &DecimalValue {
        &self.0
    }

    /// Returns the canonical wire value including the percent suffix.
    #[must_use]
    pub fn to_wire(&self) -> String {
        format!("{}%", self.0.as_str())
    }
}

impl Serialize for BuilderFeeRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_wire())
    }
}

impl<'de> Deserialize<'de> for BuilderFeeRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(de::Error::custom)
    }
}

/// Builder fee authorization signed with the user-action EIP-712 domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFeeAction {
    /// Action discriminator. Always `approveBuilderFee`.
    #[serde(rename = "type")]
    action_type: ApproveBuilderFeeActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Maximum fee rate approved for the builder.
    pub max_fee_rate: BuilderFeeRate,
    /// Builder address receiving the attributed fee.
    pub builder: EvmAddress,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ApproveBuilderFeeActionType {
    #[serde(rename = "approveBuilderFee")]
    ApproveBuilderFee,
}

/// Exact USD class-transfer amount, optionally scoped to a subaccount.
///
/// Hyperliquid encodes the subaccount inside the signed amount string rather
/// than using the outer `vaultAddress` field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsdClassTransferAmount {
    amount: DecimalValue,
    sub_account: Option<EvmAddress>,
}

impl UsdClassTransferAmount {
    /// Creates a positive canonical amount with an optional subaccount suffix.
    ///
    /// # Errors
    ///
    /// Returns an error when `amount` is not positive.
    pub fn new(
        amount: DecimalValue,
        sub_account: Option<EvmAddress>,
    ) -> Result<Self, ProtocolError> {
        require_positive_amount(&amount)?;
        Ok(Self {
            amount,
            sub_account,
        })
    }

    /// Parses the official combined wire representation.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid amounts, addresses, or suffix syntax.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, ProtocolError> {
        let value = value.as_ref();
        if let Some((amount, address)) = value.split_once(" subaccount:") {
            if address.contains(" subaccount:") {
                return Err(ProtocolError::InvalidAction(
                    "USD class transfer amount contains multiple subaccount suffixes",
                ));
            }
            return Self::new(DecimalValue::parse(amount)?, Some(address.parse()?));
        }
        Self::new(DecimalValue::parse(value)?, None)
    }

    /// Returns the positive canonical decimal component.
    #[must_use]
    pub fn amount(&self) -> &DecimalValue {
        &self.amount
    }

    /// Returns the optional source subaccount.
    #[must_use]
    pub const fn sub_account(&self) -> Option<EvmAddress> {
        self.sub_account
    }

    /// Returns the official combined wire representation.
    #[must_use]
    pub fn to_wire(&self) -> String {
        self.sub_account.as_ref().map_or_else(
            || self.amount.to_string(),
            |address| format!("{} subaccount:{address}", self.amount),
        )
    }
}

impl Serialize for UsdClassTransferAmount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_wire())
    }
}

impl<'de> Deserialize<'de> for UsdClassTransferAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(de::Error::custom)
    }
}

/// Source account for a generalized asset transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FromSubAccount(Option<EvmAddress>);

impl FromSubAccount {
    /// Creates the official main-account (empty string) or subaccount source.
    #[must_use]
    pub const fn new(address: Option<EvmAddress>) -> Self {
        Self(address)
    }

    /// Returns the optional subaccount address.
    #[must_use]
    pub const fn address(self) -> Option<EvmAddress> {
        self.0
    }

    /// Returns the official wire string; the main account is represented by `""`.
    #[must_use]
    pub fn to_wire(self) -> String {
        self.0
            .map_or_else(String::new, |address| address.to_string())
    }
}

impl Serialize for FromSubAccount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_wire())
    }
}

impl<'de> Deserialize<'de> for FromSubAccount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.is_empty() {
            return Ok(Self(None));
        }
        value
            .parse()
            .map(|address| Self(Some(address)))
            .map_err(de::Error::custom)
    }
}

/// Spot/perp USD balance transfer signed with the user-action EIP-712 domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdClassTransferAction {
    /// Action discriminator. Always `usdClassTransfer`.
    #[serde(rename = "type")]
    action_type: UsdClassTransferActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Positive amount, optionally containing the official subaccount suffix.
    pub amount: UsdClassTransferAmount,
    /// `true` for spot to perp; `false` for perp to spot.
    pub to_perp: bool,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UsdClassTransferActionType {
    #[serde(rename = "usdClassTransfer")]
    UsdClassTransfer,
}

/// Generalized asset transfer signed with the user-action EIP-712 domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAssetAction {
    /// Action discriminator. Always `sendAsset`.
    #[serde(rename = "type")]
    action_type: SendAssetActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Canonical destination user or subaccount address.
    pub destination: EvmAddress,
    /// Source DEX: `""` for default perp, `spot`, or a HIP-3 DEX name.
    pub source_dex: String,
    /// Destination DEX using the same naming rules as `sourceDex`.
    pub destination_dex: String,
    /// Hyperliquid token identifier such as `PURR:0x...`.
    pub token: String,
    /// Positive canonical transfer amount.
    pub amount: DecimalValue,
    /// Empty for the main account or a canonical source subaccount address.
    pub from_sub_account: FromSubAccount,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SendAssetActionType {
    #[serde(rename = "sendAsset")]
    SendAsset,
}

/// Native-token transfer from spot balance into staking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDepositAction {
    /// Action discriminator. Always `cDeposit`.
    #[serde(rename = "type")]
    action_type: CDepositActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Positive amount in the native token's smallest unit.
    pub wei: u64,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CDepositActionType {
    #[serde(rename = "cDeposit")]
    CDeposit,
}

/// Native-token transfer from staking into spot balance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CWithdrawAction {
    /// Action discriminator. Always `cWithdraw`.
    #[serde(rename = "type")]
    action_type: CWithdrawActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Positive amount in the native token's smallest unit.
    pub wei: u64,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CWithdrawActionType {
    #[serde(rename = "cWithdraw")]
    CWithdraw,
}

/// Delegates or undelegates native-token stake to or from a validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDelegateAction {
    /// Action discriminator. Always `tokenDelegate`.
    #[serde(rename = "type")]
    action_type: TokenDelegateActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Validator receiving or releasing the delegated stake.
    pub validator: EvmAddress,
    /// Positive amount in the native token's smallest unit.
    pub wei: u64,
    /// `true` to undelegate; `false` to delegate.
    pub is_undelegate: bool,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TokenDelegateActionType {
    #[serde(rename = "tokenDelegate")]
    TokenDelegate,
}

/// User account abstraction mode accepted by `userSetAbstraction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserAbstraction {
    /// Disable account abstraction.
    #[serde(rename = "disabled")]
    Disabled,
    /// Use one unified account balance.
    #[serde(rename = "unifiedAccount")]
    UnifiedAccount,
    /// Enable portfolio margin.
    #[serde(rename = "portfolioMargin")]
    PortfolioMargin,
}

impl UserAbstraction {
    /// Official human-readable wire value included in the EIP-712 message.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::UnifiedAccount => "unifiedAccount",
            Self::PortfolioMargin => "portfolioMargin",
        }
    }
}

/// Legacy DEX-abstraction toggle signed by the master user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDexAbstractionAction {
    /// Action discriminator. Always `userDexAbstraction`.
    #[serde(rename = "type")]
    action_type: UserDexAbstractionActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Master account or owned subaccount whose mode changes.
    pub user: EvmAddress,
    /// Whether legacy DEX abstraction is enabled.
    pub enabled: bool,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UserDexAbstractionActionType {
    #[serde(rename = "userDexAbstraction")]
    UserDexAbstraction,
}

/// Sets the current user account abstraction mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSetAbstractionAction {
    /// Action discriminator. Always `userSetAbstraction`.
    #[serde(rename = "type")]
    action_type: UserSetAbstractionActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Master account or owned subaccount whose mode changes.
    pub user: EvmAddress,
    /// Requested human-readable abstraction mode.
    pub abstraction: UserAbstraction,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UserSetAbstractionActionType {
    #[serde(rename = "userSetAbstraction")]
    UserSetAbstraction,
}

/// Canonical authorized-user set embedded as a JSON string by Hyperliquid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSigSigners {
    authorized_users: Vec<EvmAddress>,
    threshold: u32,
}

impl MultiSigSigners {
    /// Creates a sorted, unique signer set with a satisfiable threshold.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty set, duplicate addresses, or a threshold
    /// outside `1..=authorized_users.len()`.
    pub fn new(
        mut authorized_users: Vec<EvmAddress>,
        threshold: u32,
    ) -> Result<Self, ProtocolError> {
        authorized_users.sort_unstable_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        if authorized_users.is_empty() {
            return Err(ProtocolError::InvalidAction(
                "multi-signature authorized users must not be empty",
            ));
        }
        if authorized_users.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(ProtocolError::InvalidAction(
                "multi-signature authorized users must be unique",
            ));
        }
        let user_count = u32::try_from(authorized_users.len()).map_err(|_| {
            ProtocolError::InvalidAction("multi-signature authorized user count exceeds u32")
        })?;
        if threshold == 0 || threshold > user_count {
            return Err(ProtocolError::InvalidAction(
                "multi-signature threshold must be within the authorized user count",
            ));
        }
        Ok(Self {
            authorized_users,
            threshold,
        })
    }

    /// Sorted authorized users.
    #[must_use]
    pub fn authorized_users(&self) -> &[EvmAddress] {
        &self.authorized_users
    }

    /// Required signature count.
    #[must_use]
    pub const fn threshold(&self) -> u32 {
        self.threshold
    }

    /// Official Python-SDK-compatible JSON string representation.
    #[must_use]
    pub fn to_wire(&self) -> String {
        let users = self
            .authorized_users
            .iter()
            .map(|address| format!("\"{address}\""))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{{\"authorizedUsers\": [{users}], \"threshold\": {}}}",
            self.threshold
        )
    }
}

impl Serialize for MultiSigSigners {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_wire())
    }
}

impl<'de> Deserialize<'de> for MultiSigSigners {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct SignersWire {
            authorized_users: Vec<String>,
            threshold: u32,
        }

        let value = String::deserialize(deserializer)?;
        let wire: SignersWire = serde_json::from_str(&value).map_err(de::Error::custom)?;
        let users = wire
            .authorized_users
            .into_iter()
            .map(|address| address.parse())
            .collect::<Result<Vec<EvmAddress>, ProtocolError>>()
            .map_err(de::Error::custom)?;
        Self::new(users, wire.threshold).map_err(de::Error::custom)
    }
}

/// Converts a standard user account into a threshold multi-signature user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToMultiSigUserAction {
    /// Action discriminator. Always `convertToMultiSigUser`.
    #[serde(rename = "type")]
    action_type: ConvertToMultiSigUserActionType,
    /// Replay-protection environment included in the signed message.
    hyperliquid_chain: HyperliquidChain,
    /// Wallet signing chain ID. Always `0x66eee`.
    signature_chain_id: UserSignatureChainId,
    /// Sorted signer configuration encoded as the protocol JSON string.
    pub signers: MultiSigSigners,
    /// Unix milliseconds; must equal the outer exchange nonce.
    pub nonce: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ConvertToMultiSigUserActionType {
    #[serde(rename = "convertToMultiSigUser")]
    ConvertToMultiSigUser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum HyperliquidChain {
    Mainnet,
    Testnet,
}

impl From<hyper_axis_types::Network> for HyperliquidChain {
    fn from(network: hyper_axis_types::Network) -> Self {
        match network {
            hyper_axis_types::Network::Mainnet => Self::Mainnet,
            hyper_axis_types::Network::Testnet => Self::Testnet,
        }
    }
}

impl HyperliquidChain {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet",
            Self::Testnet => "Testnet",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UserSignatureChainId {
    #[serde(rename = "0x66eee")]
    Hyperliquid,
}

impl UsdSendAction {
    /// Creates a canonical positive USDC transfer.
    ///
    /// # Errors
    ///
    /// Returns an error when `amount` is not positive.
    pub fn new(
        network: hyper_axis_types::Network,
        destination: EvmAddress,
        amount: DecimalValue,
        time: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_amount(&amount)?;
        Ok(Self {
            action_type: UsdSendActionType::UsdSend,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            destination,
            amount,
            time,
        })
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl SpotSendAction {
    /// Creates a canonical positive spot transfer.
    ///
    /// # Errors
    ///
    /// Returns an error for non-positive amounts or malformed `name:id` token
    /// identifiers.
    pub fn new(
        network: hyper_axis_types::Network,
        destination: EvmAddress,
        token: String,
        amount: DecimalValue,
        time: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_amount(&amount)?;
        validate_spot_token_identifier(&token)?;
        Ok(Self {
            action_type: SpotSendActionType::SpotSend,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            destination,
            token,
            amount,
            time,
        })
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl WithdrawAction {
    /// Creates a canonical positive bridge withdrawal.
    ///
    /// # Errors
    ///
    /// Returns an error when `amount` is not positive.
    pub fn new(
        network: hyper_axis_types::Network,
        destination: EvmAddress,
        amount: DecimalValue,
        time: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_amount(&amount)?;
        Ok(Self {
            action_type: WithdrawActionType::Withdraw,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            destination,
            amount,
            time,
        })
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl ApproveAgentAction {
    /// Creates an API wallet authorization action.
    #[must_use]
    pub fn new(
        network: hyper_axis_types::Network,
        agent_address: EvmAddress,
        agent_name: Option<String>,
        nonce: u64,
    ) -> Self {
        Self {
            action_type: ApproveAgentActionType::ApproveAgent,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            agent_address,
            agent_name,
            nonce,
        }
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }

    /// Name included in the EIP-712 message, including the official empty-string
    /// representation for an unnamed API wallet.
    #[must_use]
    pub fn signed_agent_name(&self) -> &str {
        self.agent_name.as_deref().unwrap_or("")
    }
}

impl ApproveBuilderFeeAction {
    /// Creates a builder fee authorization action.
    #[must_use]
    pub fn new(
        network: hyper_axis_types::Network,
        max_fee_rate: BuilderFeeRate,
        builder: EvmAddress,
        nonce: u64,
    ) -> Self {
        Self {
            action_type: ApproveBuilderFeeActionType::ApproveBuilderFee,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            max_fee_rate,
            builder,
            nonce,
        }
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl UsdClassTransferAction {
    /// Creates a spot/perp USD class transfer.
    #[must_use]
    pub fn new(
        network: hyper_axis_types::Network,
        amount: UsdClassTransferAmount,
        to_perp: bool,
        nonce: u64,
    ) -> Self {
        Self {
            action_type: UsdClassTransferActionType::UsdClassTransfer,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            amount,
            to_perp,
            nonce,
        }
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl SendAssetAction {
    /// Creates a generalized asset transfer.
    ///
    /// # Errors
    ///
    /// Returns an error for a non-positive amount, malformed token identifier,
    /// or DEX names with surrounding whitespace.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        network: hyper_axis_types::Network,
        destination: EvmAddress,
        source_dex: String,
        destination_dex: String,
        token: String,
        amount: DecimalValue,
        from_sub_account: FromSubAccount,
        nonce: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_amount(&amount)?;
        validate_asset_token(&token)?;
        validate_dex_name(&source_dex)?;
        validate_dex_name(&destination_dex)?;
        Ok(Self {
            action_type: SendAssetActionType::SendAsset,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            destination,
            source_dex,
            destination_dex,
            token,
            amount,
            from_sub_account,
            nonce,
        })
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl CDepositAction {
    /// Creates a staking deposit with a strictly positive amount.
    ///
    /// # Errors
    ///
    /// Returns an error when `wei` is zero.
    pub fn new(
        network: hyper_axis_types::Network,
        wei: u64,
        nonce: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_wei(wei)?;
        Ok(Self {
            action_type: CDepositActionType::CDeposit,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            wei,
            nonce,
        })
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl CWithdrawAction {
    /// Creates a staking withdrawal with a strictly positive amount.
    ///
    /// # Errors
    ///
    /// Returns an error when `wei` is zero.
    pub fn new(
        network: hyper_axis_types::Network,
        wei: u64,
        nonce: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_wei(wei)?;
        Ok(Self {
            action_type: CWithdrawActionType::CWithdraw,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            wei,
            nonce,
        })
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl TokenDelegateAction {
    /// Creates a validator delegation or undelegation action.
    ///
    /// # Errors
    ///
    /// Returns an error when `wei` is zero.
    pub fn new(
        network: hyper_axis_types::Network,
        validator: EvmAddress,
        wei: u64,
        is_undelegate: bool,
        nonce: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_wei(wei)?;
        Ok(Self {
            action_type: TokenDelegateActionType::TokenDelegate,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            validator,
            wei,
            is_undelegate,
            nonce,
        })
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl UserDexAbstractionAction {
    /// Creates a legacy DEX-abstraction toggle.
    #[must_use]
    pub fn new(
        network: hyper_axis_types::Network,
        user: EvmAddress,
        enabled: bool,
        nonce: u64,
    ) -> Self {
        Self {
            action_type: UserDexAbstractionActionType::UserDexAbstraction,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            user,
            enabled,
            nonce,
        }
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl UserSetAbstractionAction {
    /// Creates an account-abstraction mode change.
    #[must_use]
    pub fn new(
        network: hyper_axis_types::Network,
        user: EvmAddress,
        abstraction: UserAbstraction,
        nonce: u64,
    ) -> Self {
        Self {
            action_type: UserSetAbstractionActionType::UserSetAbstraction,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            user,
            abstraction,
            nonce,
        }
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}

impl ConvertToMultiSigUserAction {
    /// Creates a conversion request to a threshold multi-signature user.
    #[must_use]
    pub fn new(network: hyper_axis_types::Network, signers: MultiSigSigners, nonce: u64) -> Self {
        Self {
            action_type: ConvertToMultiSigUserActionType::ConvertToMultiSigUser,
            hyperliquid_chain: network.into(),
            signature_chain_id: UserSignatureChainId::Hyperliquid,
            signers,
            nonce,
        }
    }

    /// Environment string included in the EIP-712 message.
    #[must_use]
    pub const fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain.as_str()
    }
}
