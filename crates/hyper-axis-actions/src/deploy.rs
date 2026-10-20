/// Token specification nested in `spotDeploy.registerToken2`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotTokenSpec {
    /// Short token symbol.
    pub name: String,
    /// Trading size decimals.
    pub sz_decimals: u8,
    /// On-chain token decimals.
    pub wei_decimals: u8,
}

/// Spot token registration payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotRegisterToken {
    /// Token precision specification.
    pub spec: SpotTokenSpec,
    /// Maximum deployment gas bid.
    pub max_gas: u64,
    /// Full display name.
    pub full_name: String,
}

/// Spot user-genesis distribution payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotUserGenesis {
    /// Token index being distributed.
    pub token: u32,
    /// Canonical user address and wei amount pairs.
    pub user_and_wei: Vec<(EvmAddress, String)>,
    /// Existing token index and wei amount pairs.
    pub existing_token_and_wei: Vec<(u32, String)>,
}

/// Token-only spot deployer payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotTokenReference {
    /// Target token index.
    pub token: u32,
}

/// Freezes or unfreezes one token holder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotFreezeUser {
    /// Target token index.
    pub token: u32,
    /// Target user address.
    pub user: EvmAddress,
    /// `true` to freeze and `false` to unfreeze.
    pub freeze: bool,
}

/// Spot token genesis finalization payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotGenesis {
    /// Target token index.
    pub token: u32,
    /// Maximum token supply as an exact protocol string.
    pub max_supply: String,
    /// Present only when Hyperliquidity is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_hyperliquidity: Option<bool>,
}

/// Spot market registration payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotRegisterSpot {
    /// `[baseToken, quoteToken]`.
    pub tokens: [u32; 2],
}

/// Hyperliquidity seeding configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotRegisterHyperliquidity {
    /// Target spot market index.
    pub spot: u32,
    /// Exact initial price.
    pub start_px: DecimalValue,
    /// Exact order size.
    pub order_sz: DecimalValue,
    /// Number of seeded orders.
    pub n_orders: u32,
    /// Optional number of initially seeded levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_seeded_levels: Option<u32>,
}

/// Deployer trading-fee share configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotDeployerTradingFeeShare {
    /// Target token index.
    pub token: u32,
    /// Exact deployer fee share.
    pub share: DecimalValue,
}

/// Spot deployment action matching the official externally-tagged variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotDeployAction {
    /// Action discriminator. Always `spotDeploy`.
    #[serde(rename = "type")]
    action_type: SpotDeployActionType,
    #[serde(flatten)]
    variant: SpotDeployVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SpotDeployActionType {
    #[serde(rename = "spotDeploy")]
    SpotDeploy,
}

/// Supported official Spot deployer variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SpotDeployVariant {
    RegisterToken2(SpotRegisterToken),
    UserGenesis(SpotUserGenesis),
    EnableFreezePrivilege(SpotTokenReference),
    FreezeUser(SpotFreezeUser),
    RevokeFreezePrivilege(SpotTokenReference),
    EnableQuoteToken(SpotTokenReference),
    Genesis(SpotGenesis),
    RegisterSpot(SpotRegisterSpot),
    RegisterHyperliquidity(SpotRegisterHyperliquidity),
    SetDeployerTradingFeeShare(SpotDeployerTradingFeeShare),
}

impl SpotDeployAction {
    /// Creates a strongly typed Spot deployer action.
    #[must_use]
    pub const fn new(variant: SpotDeployVariant) -> Self {
        Self {
            action_type: SpotDeployActionType::SpotDeploy,
            variant,
        }
    }
}

/// Optional HIP-3 perpetual DEX schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDexSchema {
    /// Full DEX display name.
    pub full_name: String,
    /// Collateral token index.
    pub collateral_token: u32,
    /// Optional canonical oracle-updater address.
    pub oracle_updater: Option<EvmAddress>,
}

/// Perpetual asset specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpAssetRequest {
    /// Asset symbol.
    pub coin: String,
    /// Trading size decimals.
    pub sz_decimals: u8,
    /// Initial exact oracle price.
    pub oracle_px: DecimalValue,
    /// Margin-table identifier.
    pub margin_table_id: u32,
    /// Whether the asset is isolated-only.
    pub only_isolated: bool,
}

/// HIP-3 perpetual asset registration payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpRegisterAsset {
    /// Optional deployment gas bid.
    pub max_gas: Option<u64>,
    /// Asset specification.
    pub asset_request: PerpAssetRequest,
    /// Target perpetual DEX.
    pub dex: String,
    /// Optional schema when registering a DEX and its first asset.
    pub schema: Option<PerpDexSchema>,
}

/// One canonical `[coin, price]` wire tuple.
pub type PerpPriceEntry = (String, DecimalValue);

/// Perpetual oracle update payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpSetOracle {
    /// Target perpetual DEX.
    pub dex: String,
    /// Sorted oracle-price entries.
    pub oracle_pxs: Vec<PerpPriceEntry>,
    /// One sorted mark-price set per oracle source.
    pub mark_pxs: Vec<Vec<PerpPriceEntry>>,
    /// Sorted external perpetual-price entries.
    pub external_perp_pxs: Vec<PerpPriceEntry>,
}

/// Perpetual deployment action matching the official variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpDeployAction {
    /// Action discriminator. Always `perpDeploy`.
    #[serde(rename = "type")]
    action_type: PerpDeployActionType,
    #[serde(flatten)]
    variant: PerpDeployVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum PerpDeployActionType {
    #[serde(rename = "perpDeploy")]
    PerpDeploy,
}

/// Supported official perpetual deployer variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PerpDeployVariant {
    RegisterAsset(PerpRegisterAsset),
    SetOracle(PerpSetOracle),
}

impl PerpDeployAction {
    /// Creates a strongly typed perpetual deployer action.
    #[must_use]
    pub const fn new(variant: PerpDeployVariant) -> Self {
        Self {
            action_type: PerpDeployActionType::PerpDeploy,
            variant,
        }
    }
}
