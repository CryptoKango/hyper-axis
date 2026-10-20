/// Input for preparing a single limit order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareLimitOrderRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Exchange asset ID.
    pub asset: AssetId,
    /// Perpetual or spot precision rules.
    pub market_kind: MarketKind,
    /// `szDecimals` read from market metadata.
    pub sz_decimals: u8,
    /// Buy when true, sell when false.
    pub is_buy: bool,
    /// Exact decimal price string.
    pub limit_price: String,
    /// Exact decimal size string.
    pub size: String,
    /// Whether the order may only reduce a position.
    pub reduce_only: bool,
    /// Limit-order time in force.
    pub time_in_force: TimeInForce,
    /// Optional client order ID.
    pub client_order_id: Option<String>,
    /// Batch grouping mode.
    pub grouping: Grouping,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// One limit order inside a batch preparation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderInput {
    /// Exchange asset ID.
    pub asset: AssetId,
    /// Perpetual or spot precision rules.
    pub market_kind: MarketKind,
    /// `szDecimals` read from market metadata.
    pub sz_decimals: u8,
    /// Buy when true, sell when false.
    pub is_buy: bool,
    /// Exact decimal price string.
    pub limit_price: String,
    /// Exact decimal size string.
    pub size: String,
    /// Whether the order may only reduce a position.
    pub reduce_only: bool,
    /// Limit-order time in force.
    pub time_in_force: TimeInForce,
    /// Optional 128-bit client order ID.
    pub client_order_id: Option<String>,
}

/// One trigger order inside a mixed order batch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrderInput {
    /// Exchange asset ID.
    pub asset: AssetId,
    /// Perpetual or spot precision rules.
    pub market_kind: MarketKind,
    /// `szDecimals` read from market metadata.
    pub sz_decimals: u8,
    /// Buy when true, sell when false.
    pub is_buy: bool,
    /// Exact execution limit price. Market triggers still require the wire `p`
    /// value supplied by the application.
    pub limit_price: String,
    /// Exact trigger price.
    pub trigger_price: String,
    /// Exact decimal size string.
    pub size: String,
    /// Whether the order may only reduce a position.
    pub reduce_only: bool,
    /// Whether triggering creates a market order.
    pub is_market: bool,
    /// Take-profit or stop-loss trigger classification.
    pub trigger_kind: TriggerKind,
    /// Optional 128-bit client order ID.
    pub client_order_id: Option<String>,
}

/// Limit or trigger order accepted by the general mixed-batch API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum OrderInput {
    /// Regular limit order.
    Limit(LimitOrderInput),
    /// Take-profit or stop-loss trigger order.
    Trigger(TriggerOrderInput),
}

/// Optional builder fee attribution supplied by an application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderInput {
    /// Builder's EVM address.
    pub address: String,
    /// Fee in tenths of a basis point.
    pub fee: u32,
}

/// Input for preparing one atomic order action containing multiple orders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareBatchOrdersRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Non-empty collection of order inputs.
    pub orders: Vec<LimitOrderInput>,
    /// Batch grouping mode.
    pub grouping: Grouping,
    /// Optional builder fee attribution.
    pub builder: Option<BuilderInput>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for preparing a mixed atomic order batch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareOrdersRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Non-empty collection of limit and/or trigger orders.
    pub orders: Vec<OrderInput>,
    /// Batch grouping mode.
    pub grouping: Grouping,
    /// Optional builder fee attribution.
    pub builder: Option<BuilderInput>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// One cancellation selected by exchange order ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelInput {
    /// Exchange asset ID.
    pub asset: AssetId,
    /// Exchange-assigned order ID.
    #[serde(with = "u64_string")]
    pub oid: u64,
}

/// Input for preparing a cancellation batch by exchange order ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCancelRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Non-empty cancellation collection.
    pub cancels: Vec<CancelInput>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// One cancellation selected by a 128-bit client order ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelByCloidInput {
    /// Exchange asset ID.
    pub asset: AssetId,
    /// Exact 128-bit hexadecimal client order ID.
    pub cloid: String,
}

/// Input for preparing a cancellation batch by Cloid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCancelByCloidRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Non-empty cancellation collection.
    pub cancels: Vec<CancelByCloidInput>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Existing order selector used by an order modification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderSelectorInput {
    /// Exchange-assigned order ID.
    Oid(#[serde(with = "u64_string")] u64),
    /// Exact 128-bit hexadecimal client order ID.
    Cloid(String),
}

/// One complete order replacement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrderInput {
    /// Existing order selected by OID or Cloid.
    pub oid: OrderSelectorInput,
    /// Complete replacement order.
    pub order: LimitOrderInput,
}

/// Input for preparing a batch-modify action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareBatchModifyRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Non-empty collection of order replacements.
    pub modifies: Vec<ModifyOrderInput>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for setting or clearing the cancel-all dead man's switch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareScheduleCancelRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Deadline in Unix milliseconds, or `None` to clear the schedule.
    #[serde(default, with = "option_u64_string")]
    pub scheduled_time: Option<u64>,
    /// Current Unix milliseconds used for the five-second local preflight.
    #[serde(with = "u64_string")]
    pub now_ms: u64,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for preparing one native TWAP order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareTwapOrderRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Exchange asset ID.
    pub asset: AssetId,
    /// Perpetual or spot size precision rules.
    pub market_kind: MarketKind,
    /// `szDecimals` read from market metadata.
    pub sz_decimals: u8,
    /// Buy when true, sell when false.
    pub is_buy: bool,
    /// Exact total size string.
    pub size: String,
    /// Whether execution may only reduce an existing position.
    pub reduce_only: bool,
    /// Duration in minutes.
    pub minutes: u32,
    /// Whether slice timing is randomized.
    pub randomize: bool,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for cancelling one native TWAP order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareTwapCancelRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Exchange asset ID.
    pub asset: AssetId,
    /// Exchange-assigned TWAP ID.
    #[serde(with = "u64_string")]
    pub twap_id: u64,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for changing cross or isolated leverage on one perpetual market.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareUpdateLeverageRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Perpetual asset ID.
    pub asset: AssetId,
    /// Cross margin when true, isolated margin when false.
    pub is_cross: bool,
    /// Requested integer leverage.
    pub leverage: u32,
    /// Current `maxLeverage` from perpetual market metadata.
    pub max_leverage: u32,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for adding or removing USD margin from an isolated position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareUpdateIsolatedMarginRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Perpetual asset ID.
    pub asset: AssetId,
    /// Position side. Reserved by Hyperliquid for future hedge mode.
    pub is_buy: bool,
    /// Signed exact USD delta with at most six fractional digits.
    pub usd_delta: String,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for targeting leverage on an isolated-only market.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareTopUpIsolatedOnlyMarginRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Perpetual asset ID.
    pub asset: AssetId,
    /// Positive exact target leverage string.
    pub leverage: String,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<String>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for creating a named subaccount.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCreateSubAccountRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// User-visible subaccount name.
    pub name: String,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for transferring USD between a master account and its subaccount.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareSubAccountTransferRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Target subaccount address.
    pub sub_account_user: String,
    /// `true` for master to subaccount; `false` for subaccount to master.
    pub is_deposit: bool,
    /// Positive exact USD amount with at most six fractional digits.
    pub amount: String,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for transferring a spot token between a master and subaccount.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareSubAccountSpotTransferRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Target subaccount address.
    pub sub_account_user: String,
    /// `true` for master to subaccount; `false` for subaccount to master.
    pub is_deposit: bool,
    /// Hyperliquid token identifier such as `PURR:0x...`.
    pub token: String,
    /// Positive exact token amount.
    pub amount: String,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for depositing or withdrawing USD to or from a vault.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareVaultTransferRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Target vault address embedded in the action.
    pub vault_address: String,
    /// `true` to deposit; `false` to withdraw.
    pub is_deposit: bool,
    /// Positive exact USD amount with at most six fractional digits.
    pub amount: String,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for registering a referral code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareSetReferrerRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Referral code supplied by the application.
    pub code: String,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for changing the user's EVM large-block preference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareEvmUserModifyRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Whether large EVM blocks are enabled.
    pub using_big_blocks: bool,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for enabling legacy DEX abstraction for an agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareAgentEnableDexAbstractionRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Optional vault or subaccount represented by the signing agent.
    pub vault_address: Option<String>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for selecting an agent account-abstraction mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareAgentSetAbstractionRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Compact agent mode encoded by the official L1 protocol.
    pub abstraction: AgentAbstraction,
    /// Optional vault or subaccount represented by the signing agent.
    pub vault_address: Option<String>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for preparing an empty L1 action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareNoopRequest {
    /// Network controlling the phantom-agent source.
    pub network: Network,
    /// Optional vault or subaccount signing context.
    pub vault_address: Option<String>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for signer-node jail or unjail self-management.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCSignerRequest {
    /// Target network.
    pub network: Network,
    /// Signer-node operation.
    pub variant: CSignerVariant,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for consensus validator registration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCValidatorRegisterRequest {
    /// Target network.
    pub network: Network,
    /// Validator node IPv4 or IPv6 address.
    pub node_ip: String,
    /// Public validator name.
    pub name: String,
    /// Public validator description.
    pub description: String,
    /// Whether third-party delegations are disabled.
    pub delegations_disabled: bool,
    /// Commission in basis points, at most 10,000.
    pub commission_bps: u16,
    /// Consensus signer address.
    pub signer: String,
    /// Whether the validator starts unjailed.
    pub unjailed: bool,
    /// Initial native-token self delegation in wei.
    #[serde(with = "u64_string")]
    pub initial_wei: u64,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for a partial consensus validator profile update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCValidatorChangeProfileRequest {
    /// Target network.
    pub network: Network,
    /// Optional replacement node IP.
    pub node_ip: Option<String>,
    /// Optional replacement public name.
    pub name: Option<String>,
    /// Optional replacement public description.
    pub description: Option<String>,
    /// Whether the validator should be unjailed.
    pub unjailed: bool,
    /// Optional delegation-disable flag.
    pub disable_delegations: Option<bool>,
    /// Optional commission in basis points, at most 10,000.
    pub commission_bps: Option<u16>,
    /// Optional replacement consensus signer.
    pub signer: Option<String>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for consensus validator unregistration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCValidatorUnregisterRequest {
    /// Target network.
    pub network: Network,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for a gossip priority gas bid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareGossipPriorityBidRequest {
    /// Target network.
    pub network: Network,
    /// Target gossip priority slot.
    #[serde(with = "u64_string")]
    pub slot_id: u64,
    /// Target IPv4 or IPv6 address.
    pub ip: String,
    /// Maximum gas bid.
    #[serde(with = "u64_string")]
    pub max_gas: u64,
    /// Optional vault or subaccount signing context.
    pub vault_address: Option<String>,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// One user allocation in Spot token genesis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotUserWeiInput {
    /// Canonical EVM user address.
    pub user: String,
    /// Unsigned integer wei amount.
    pub wei: String,
}

/// One existing-token allocation in Spot token genesis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotExistingTokenWeiInput {
    /// Existing token index.
    pub token: u32,
    /// Unsigned integer wei amount.
    pub wei: String,
}

/// Supported Spot deployer requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum SpotDeployInput {
    /// Registers a new Spot token specification.
    RegisterToken {
        token_name: String,
        sz_decimals: u8,
        wei_decimals: u8,
        #[serde(with = "u64_string")]
        max_gas: u64,
        full_name: String,
    },
    /// Adds initial user and existing-token allocations.
    UserGenesis {
        token: u32,
        user_and_wei: Vec<SpotUserWeiInput>,
        existing_token_and_wei: Vec<SpotExistingTokenWeiInput>,
    },
    /// Enables token-holder freeze privilege.
    EnableFreezePrivilege { token: u32 },
    /// Freezes or unfreezes one token holder.
    FreezeUser {
        token: u32,
        user: String,
        freeze: bool,
    },
    /// Permanently revokes token-holder freeze privilege.
    RevokeFreezePrivilege { token: u32 },
    /// Enables use of the token as a quote asset.
    EnableQuoteToken { token: u32 },
    /// Finalizes token genesis.
    Genesis {
        token: u32,
        max_supply: String,
        no_hyperliquidity: bool,
    },
    /// Registers a base/quote Spot market.
    RegisterSpot { base_token: u32, quote_token: u32 },
    /// Registers Hyperliquidity seeding parameters.
    RegisterHyperliquidity {
        spot: u32,
        start_px: String,
        order_sz: String,
        n_orders: u32,
        n_seeded_levels: Option<u32>,
    },
    /// Sets the deployer's trading-fee share.
    SetDeployerTradingFeeShare { token: u32, share: String },
}

/// Input for any official Spot deployer action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareSpotDeployRequest {
    /// Target network.
    pub network: Network,
    /// Strongly typed deployer operation.
    pub input: SpotDeployInput,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// One named perpetual price entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerpPriceInput {
    /// Asset symbol.
    pub coin: String,
    /// Exact price string.
    pub price: String,
}

/// Optional HIP-3 DEX schema input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDexSchemaInput {
    /// Full DEX display name.
    pub full_name: String,
    /// Collateral token index.
    pub collateral_token: u32,
    /// Optional oracle-updater address.
    pub oracle_updater: Option<String>,
}

/// Supported perpetual deployer requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum PerpDeployInput {
    /// Registers an asset or creates a HIP-3 DEX with its first asset.
    RegisterAsset {
        dex: String,
        #[serde(default, with = "option_u64_string")]
        max_gas: Option<u64>,
        coin: String,
        sz_decimals: u8,
        oracle_px: String,
        margin_table_id: u32,
        only_isolated: bool,
        schema: Option<PerpDexSchemaInput>,
    },
    /// Updates oracle, mark, and external perpetual prices.
    SetOracle {
        dex: String,
        oracle_pxs: Vec<PerpPriceInput>,
        mark_pxs: Vec<Vec<PerpPriceInput>>,
        external_perp_pxs: Vec<PerpPriceInput>,
    },
}

/// Input for any official perpetual deployer action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparePerpDeployRequest {
    /// Target network.
    pub network: Network,
    /// Strongly typed deployer operation.
    pub input: PerpDeployInput,
    /// Millisecond nonce used by the action hash.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
}

/// Input for a Core USDC transfer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareUsdSendRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Destination EVM address.
    pub destination: String,
    /// Positive exact USDC amount.
    pub amount: String,
    /// Millisecond nonce, also used as the action's signed `time`.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for a Core spot asset transfer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareSpotSendRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Destination EVM address.
    pub destination: String,
    /// Hyperliquid token identifier such as `PURR:0x...`.
    pub token: String,
    /// Positive exact token amount.
    pub amount: String,
    /// Millisecond nonce, also used as the action's signed `time`.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for a bridge withdrawal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareWithdrawRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Destination EVM address.
    pub destination: String,
    /// Positive exact withdrawal amount.
    pub amount: String,
    /// Millisecond nonce, also used as the action's signed `time`.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for approving an API wallet (agent wallet).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareApproveAgentRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Existing externally generated API wallet address. `HyperAxis` never creates
    /// or returns its private key.
    pub agent_address: String,
    /// Optional name. `None` creates the single unnamed API wallet slot.
    pub agent_name: Option<String>,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for approving a maximum builder fee rate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareApproveBuilderFeeRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Builder address receiving fee attribution.
    pub builder: String,
    /// Exact percentage string, for example `0.001%`.
    pub max_fee_rate: String,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for transferring USD between spot and perp balance classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareUsdClassTransferRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Positive exact USD amount without a manually appended subaccount suffix.
    pub amount: String,
    /// `true` for spot to perp; `false` for perp to spot.
    pub to_perp: bool,
    /// Optional source subaccount encoded into the signed amount by the Core.
    pub sub_account: Option<String>,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for a generalized transfer across users, subaccounts, and DEX classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareSendAssetRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Destination user or subaccount address.
    pub destination: String,
    /// Source DEX: `""` for default perp, `spot`, or a HIP-3 DEX name.
    pub source_dex: String,
    /// Destination DEX using the same naming rules as `sourceDex`.
    pub destination_dex: String,
    /// Hyperliquid token identifier such as `PURR:0x...`.
    pub token: String,
    /// Positive exact token amount.
    pub amount: String,
    /// Optional source subaccount; `None` is encoded as the required empty string.
    pub from_sub_account: Option<String>,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for moving native tokens from spot balance into staking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCDepositRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Positive amount in the native token's smallest unit. JSON must use a string.
    #[serde(with = "u64_string")]
    pub wei: u64,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for moving native tokens from staking into spot balance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareCWithdrawRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Positive amount in the native token's smallest unit. JSON must use a string.
    #[serde(with = "u64_string")]
    pub wei: u64,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for delegating or undelegating native-token stake.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareTokenDelegateRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Validator address.
    pub validator: String,
    /// Positive amount in the native token's smallest unit. JSON must use a string.
    #[serde(with = "u64_string")]
    pub wei: u64,
    /// `true` to undelegate; `false` to delegate.
    pub is_undelegate: bool,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for the legacy user DEX-abstraction toggle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareUserDexAbstractionRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Master account or owned subaccount whose mode changes.
    pub user: String,
    /// Whether legacy DEX abstraction is enabled.
    pub enabled: bool,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for selecting an account abstraction mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareUserSetAbstractionRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Master account or owned subaccount whose mode changes.
    pub user: String,
    /// Requested official abstraction mode.
    pub abstraction: UserAbstraction,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Input for converting a standard user into a threshold multi-signature user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareConvertToMultiSigUserRequest {
    /// Network embedded in user-action replay protection.
    pub network: Network,
    /// Authorized signer addresses. Rust sorts and deduplicates this set.
    pub authorized_users: Vec<String>,
    /// Number of authorized signatures required by the account.
    pub threshold: u32,
    /// Millisecond nonce, also embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
}

/// Fully validated artifacts needed by a platform signer and HTTP client.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedAction<A> {
    /// Environment used for signing and request routing.
    pub network: Network,
    /// Typed action that was encoded and hashed.
    pub action: A,
    /// Nonce included in the action hash and outer exchange request.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount address.
    pub vault_address: Option<EvmAddress>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
    /// Exchange request action as JSON.
    pub action_json: String,
    /// Canonical named-map `MessagePack` action.
    pub message_pack_hex: String,
    /// Keccak action hash wrapped by the phantom agent.
    pub action_hash_hex: String,
    /// EIP-712 digest that must be signed once with secp256k1.
    pub signing_digest_hex: String,
}

/// Prepared user-signed action, intentionally without vault or expiry fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedUserAction<A> {
    /// Environment embedded in replay protection and request routing.
    pub network: Network,
    /// Typed user action.
    pub action: A,
    /// Outer nonce, equal to the nonce or time embedded in the signed action.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Canonical action JSON used for integrity verification.
    pub action_json: String,
    /// EIP-712 digest the wallet signs exactly once.
    pub signing_digest_hex: String,
}

/// Prepared order action retained for source compatibility.
pub type PreparedOrder = PreparedAction<OrderAction>;
/// Prepared cancellation by exchange order ID.
pub type PreparedCancel = PreparedAction<CancelAction>;
/// Prepared cancellation by client order ID.
pub type PreparedCancelByCloid = PreparedAction<CancelByCloidAction>;
/// Prepared batch modification.
pub type PreparedBatchModify = PreparedAction<BatchModifyAction>;
/// Prepared schedule-cancel action.
pub type PreparedScheduleCancel = PreparedAction<ScheduleCancelAction>;
/// Prepared native TWAP order.
pub type PreparedTwapOrder = PreparedAction<TwapOrderAction>;
/// Prepared native TWAP cancellation.
pub type PreparedTwapCancel = PreparedAction<TwapCancelAction>;
/// Prepared leverage update.
pub type PreparedUpdateLeverage = PreparedAction<UpdateLeverageAction>;
/// Prepared isolated-margin delta.
pub type PreparedUpdateIsolatedMargin = PreparedAction<UpdateIsolatedMarginAction>;
/// Prepared isolated-only margin top-up.
pub type PreparedTopUpIsolatedOnlyMargin = PreparedAction<TopUpIsolatedOnlyMarginAction>;
/// Prepared named-subaccount creation.
pub type PreparedCreateSubAccount = PreparedAction<CreateSubAccountAction>;
/// Prepared USD transfer between a master account and subaccount.
pub type PreparedSubAccountTransfer = PreparedAction<SubAccountTransferAction>;
/// Prepared spot-token transfer between a master account and subaccount.
pub type PreparedSubAccountSpotTransfer = PreparedAction<SubAccountSpotTransferAction>;
/// Prepared USD vault deposit or withdrawal.
pub type PreparedVaultTransfer = PreparedAction<VaultTransferAction>;
/// Prepared referral-code registration.
pub type PreparedSetReferrer = PreparedAction<SetReferrerAction>;
/// Prepared EVM large-block preference update.
pub type PreparedEvmUserModify = PreparedAction<EvmUserModifyAction>;
/// Prepared legacy agent DEX-abstraction enablement.
pub type PreparedAgentEnableDexAbstraction = PreparedAction<AgentEnableDexAbstractionAction>;
/// Prepared agent account-abstraction mode update.
pub type PreparedAgentSetAbstraction = PreparedAction<AgentSetAbstractionAction>;
/// Prepared empty L1 action.
pub type PreparedNoop = PreparedAction<NoopAction>;
/// Prepared consensus signer self-management action.
pub type PreparedCSigner = PreparedAction<CSignerAction>;
/// Prepared consensus validator lifecycle action.
pub type PreparedCValidator = PreparedAction<CValidatorAction>;
/// Prepared gossip priority bid.
pub type PreparedGossipPriorityBid = PreparedAction<GossipPriorityBidAction>;
/// Prepared Spot deployer action.
pub type PreparedSpotDeploy = PreparedAction<SpotDeployAction>;
/// Prepared perpetual deployer action.
pub type PreparedPerpDeploy = PreparedAction<PerpDeployAction>;
/// Prepared Core USDC transfer.
pub type PreparedUsdSend = PreparedUserAction<UsdSendAction>;
/// Prepared Core spot transfer.
pub type PreparedSpotSend = PreparedUserAction<SpotSendAction>;
/// Prepared bridge withdrawal.
pub type PreparedWithdraw = PreparedUserAction<WithdrawAction>;
/// Prepared API wallet authorization.
pub type PreparedApproveAgent = PreparedUserAction<ApproveAgentAction>;
/// Prepared builder fee authorization.
pub type PreparedApproveBuilderFee = PreparedUserAction<ApproveBuilderFeeAction>;
/// Prepared spot/perp USD class transfer.
pub type PreparedUsdClassTransfer = PreparedUserAction<UsdClassTransferAction>;
/// Prepared generalized asset transfer.
pub type PreparedSendAsset = PreparedUserAction<SendAssetAction>;
/// Prepared native-token staking deposit.
pub type PreparedCDeposit = PreparedUserAction<CDepositAction>;
/// Prepared native-token staking withdrawal.
pub type PreparedCWithdraw = PreparedUserAction<CWithdrawAction>;
/// Prepared validator delegation or undelegation.
pub type PreparedTokenDelegate = PreparedUserAction<TokenDelegateAction>;
/// Prepared legacy DEX-abstraction toggle.
pub type PreparedUserDexAbstraction = PreparedUserAction<UserDexAbstractionAction>;
/// Prepared account-abstraction mode update.
pub type PreparedUserSetAbstraction = PreparedUserAction<UserSetAbstractionAction>;
/// Prepared conversion to a threshold multi-signature user.
pub type PreparedConvertToMultiSigUser = PreparedUserAction<ConvertToMultiSigUserAction>;

/// Strongly typed inner actions currently supported by the multi-signature
/// orchestration API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultiSigInnerAction {
    /// L1 order action signed through the multi-signature L1 envelope.
    Order(OrderAction),
    /// L1 cancellation selected by exchange order ID.
    Cancel(CancelAction),
    /// L1 cancellation selected by client order ID.
    CancelByCloid(CancelByCloidAction),
    /// L1 atomic batch modification.
    BatchModify(BatchModifyAction),
    /// L1 dead man's switch update.
    ScheduleCancel(ScheduleCancelAction),
    /// L1 native TWAP order.
    TwapOrder(TwapOrderAction),
    /// L1 native TWAP cancellation.
    TwapCancel(TwapCancelAction),
    /// L1 leverage update.
    UpdateLeverage(UpdateLeverageAction),
    /// L1 isolated-margin delta.
    UpdateIsolatedMargin(UpdateIsolatedMarginAction),
    /// L1 isolated-only margin top-up.
    TopUpIsolatedOnlyMargin(TopUpIsolatedOnlyMarginAction),
    /// L1 named-subaccount creation.
    CreateSubAccount(CreateSubAccountAction),
    /// L1 USD transfer between a master account and subaccount.
    SubAccountTransfer(SubAccountTransferAction),
    /// L1 spot-token transfer between a master account and subaccount.
    SubAccountSpotTransfer(SubAccountSpotTransferAction),
    /// L1 USD vault deposit or withdrawal.
    VaultTransfer(VaultTransferAction),
    /// L1 referral-code registration.
    SetReferrer(SetReferrerAction),
    /// L1 EVM large-block preference update.
    EvmUserModify(EvmUserModifyAction),
    /// L1 legacy DEX-abstraction enablement for an agent.
    AgentEnableDexAbstraction(AgentEnableDexAbstractionAction),
    /// L1 account-abstraction mode update for an agent.
    AgentSetAbstraction(AgentSetAbstractionAction),
    /// Empty L1 action with an explicit signing context.
    Noop(NoopAction),
    /// Consensus signer-node self-management action.
    CSigner(CSignerAction),
    /// Consensus validator lifecycle action.
    CValidator(CValidatorAction),
    /// Gossip priority gas bid.
    GossipPriorityBid(GossipPriorityBidAction),
    /// Spot deployer lifecycle action.
    SpotDeploy(SpotDeployAction),
    /// Perpetual deployer lifecycle action.
    PerpDeploy(PerpDeployAction),
    /// User-signed Core USDC transfer.
    UsdSend(UsdSendAction),
    /// User-signed Core spot transfer.
    SpotSend(SpotSendAction),
    /// User-signed bridge withdrawal.
    Withdraw(WithdrawAction),
    /// User-signed API-wallet authorization.
    ApproveAgent(ApproveAgentAction),
    /// User-signed builder-fee authorization.
    ApproveBuilderFee(ApproveBuilderFeeAction),
    /// User-signed USD class transfer.
    UsdClassTransfer(UsdClassTransferAction),
    /// User-signed generalized asset transfer.
    SendAsset(SendAssetAction),
    /// User-signed native-token staking deposit.
    CDeposit(CDepositAction),
    /// User-signed native-token staking withdrawal.
    CWithdraw(CWithdrawAction),
    /// User-signed validator delegation or undelegation.
    TokenDelegate(TokenDelegateAction),
    /// User-signed legacy DEX-abstraction toggle.
    UserDexAbstraction(UserDexAbstractionAction),
    /// User-signed account-abstraction mode update.
    UserSetAbstraction(UserSetAbstractionAction),
}

/// One collected inner signature and the signer address expected by the host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiSigSignatureInput {
    /// Expected authorized-user address recovered from this signature.
    pub expected_signer: String,
    /// Canonical 32-byte ECDSA `r` scalar.
    pub r: String,
    /// Canonical 32-byte ECDSA `s` scalar.
    pub s: String,
    /// Recovery identifier (`0/1` or `27/28`).
    pub v: u8,
}

/// First-stage multi-signature artifact signed by authorized users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedMultiSigPayload {
    /// Target network.
    pub network: Network,
    /// Original typed inner action. `userSetAbstraction` retains its long value.
    pub inner_action: MultiSigInnerAction,
    /// Multi-signature user whose account executes the action.
    pub multi_sig_user: EvmAddress,
    /// Address that will sign and submit the outer envelope.
    pub outer_signer: EvmAddress,
    /// Shared outer and inner signing nonce.
    #[serde(with = "u64_string")]
    pub nonce: u64,
    /// Optional vault or subaccount signing context.
    pub vault_address: Option<EvmAddress>,
    /// Optional action expiry timestamp.
    #[serde(default, with = "option_u64_string")]
    pub expires_after: Option<u64>,
    /// Canonical original inner action JSON.
    pub inner_action_json: String,
    /// Digest each authorized user signs.
    pub signing_digest_hex: String,
}

/// Second-stage artifact signed by the outer signer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedMultiSigEnvelope {
    /// Verified first-stage payload.
    pub payload: PreparedMultiSigPayload,
    /// Inner signatures sorted by recovered signer address.
    pub signatures: Vec<WireSignature>,
    /// Recovered authorized users in the same order as `signatures`.
    pub recovered_inner_signers: Vec<EvmAddress>,
    /// Canonical complete `multiSig` action JSON.
    pub action_json: String,
    /// `MessagePack` for the outer action with the `type` tag removed.
    pub message_pack_hex: String,
    /// Hash of the outer action without its `type` tag.
    pub action_hash_hex: String,
    /// `SendMultiSig` digest signed by the outer signer.
    pub signing_digest_hex: String,
}

/// Complete request body accepted by the exchange endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRequest<A> {
    /// Signed order action.
    pub action: A,
    /// Nonce used when producing the action hash.
    pub nonce: u64,
    /// Canonical recoverable signature.
    pub signature: WireSignature,
    /// Vault or subaccount address, or `null` for the signer account.
    pub vault_address: Option<EvmAddress>,
    /// Expiry timestamp, or `null` when expiry is disabled.
    pub expires_after: Option<u64>,
}

/// Complete request body for user-signed actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserExchangeRequest<A> {
    /// User-signed action.
    pub action: A,
    /// Nonce equal to the signed action's internal timestamp.
    pub nonce: u64,
    /// Canonical recoverable signature.
    pub signature: WireSignature,
}

/// Final exchange body for an order action.
pub type OrderExchangeRequest = ExchangeRequest<OrderAction>;

/// Final request artifact safe to hand to a platform HTTP transport.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalizedAction {
    /// Official exchange endpoint for the selected network.
    pub endpoint: String,
    /// Canonical JSON request body.
    pub request_json: String,
    /// Recovered signer address, normalized to lowercase.
    pub recovered_signer: EvmAddress,
}

/// Finalized order retained for source compatibility.
pub type FinalizedOrder = FinalizedAction;

/// Failures surfaced through FFI and WebAssembly boundaries.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Protocol value or structural validation failed.
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    /// Message encoding failed.
    #[error(transparent)]
    Codec(#[from] CodecError),
    /// JSON serialization failed.
    #[error("unable to serialize prepared action: {0}")]
    Json(#[from] serde_json::Error),
}

/// Failures while verifying a platform signature and creating the HTTP body.
#[derive(Debug, Error)]
pub enum FinalizeError {
    /// Prepared data no longer matches its typed action and signing context.
    #[error("prepared action integrity check failed for {0}")]
    ArtifactMismatch(&'static str),
    /// Message encoding failed during integrity verification.
    #[error(transparent)]
    Codec(#[from] CodecError),
    /// Signature validation or signer recovery failed.
    #[error(transparent)]
    Signing(#[from] SigningError),
    /// JSON serialization failed.
    #[error("unable to serialize exchange request: {0}")]
    Json(#[from] serde_json::Error),
}

/// Failures while constructing or verifying a multi-signature envelope.
#[derive(Debug, Error)]
pub enum MultiSigError {
    /// A protocol value or collection invariant is invalid.
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    /// A prepared artifact was changed after its digest was produced.
    #[error("prepared multi-signature artifact integrity check failed for {0}")]
    ArtifactMismatch(&'static str),
    /// Message encoding failed.
    #[error(transparent)]
    Codec(#[from] CodecError),
    /// An inner or outer signature is malformed or recovers another signer.
    #[error(transparent)]
    Signing(#[from] SigningError),
    /// JSON serialization failed.
    #[error("unable to serialize multi-signature action: {0}")]
    Json(#[from] serde_json::Error),
}
