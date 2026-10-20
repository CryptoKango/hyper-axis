/// Time-in-force behavior for a limit order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Add liquidity or cancel.
    Alo,
    /// Immediate or cancel.
    Ioc,
    /// Good until canceled.
    Gtc,
}

/// Trigger-order classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerKind {
    /// Take-profit trigger.
    Tp,
    /// Stop-loss trigger.
    Sl,
}

/// Limit-order payload nested below the `limit` key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitWire {
    /// Time-in-force value.
    pub tif: TimeInForce,
}

/// Trigger-order payload nested below the `trigger` key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerWire {
    /// Whether execution converts to a market order.
    pub is_market: bool,
    /// Canonical trigger price.
    pub trigger_px: DecimalValue,
    /// Take-profit or stop-loss classification.
    pub tpsl: TriggerKind,
}

/// Order type encoded as either `{ "limit": ... }` or `{ "trigger": ... }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderTypeWire {
    /// Limit order.
    Limit(LimitWire),
    /// Trigger order.
    Trigger(TriggerWire),
}

/// Compact order object accepted by the exchange endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderWire {
    /// Asset ID.
    #[serde(rename = "a")]
    pub asset: AssetId,
    /// Buy when true, sell when false.
    #[serde(rename = "b")]
    pub is_buy: bool,
    /// Limit price.
    #[serde(rename = "p")]
    pub limit_price: DecimalValue,
    /// Size.
    #[serde(rename = "s")]
    pub size: DecimalValue,
    /// Whether execution may only reduce an existing position.
    #[serde(rename = "r")]
    pub reduce_only: bool,
    /// Limit or trigger configuration.
    #[serde(rename = "t")]
    pub order_type: OrderTypeWire,
    /// Optional client order identifier.
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
}

impl OrderWire {
    /// Creates an order from values that have already passed market precision
    /// validation.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError::InvalidAction`] for non-positive prices or
    /// sizes, or an empty client order identifier.
    pub fn new(
        asset: AssetId,
        is_buy: bool,
        limit_price: DecimalValue,
        size: DecimalValue,
        reduce_only: bool,
        order_type: OrderTypeWire,
        client_order_id: Option<ClientOrderId>,
    ) -> Result<Self, ProtocolError> {
        if !limit_price.is_positive() {
            return Err(ProtocolError::InvalidAction("limit price must be positive"));
        }
        if !size.is_positive() {
            return Err(ProtocolError::InvalidAction("order size must be positive"));
        }
        Ok(Self {
            asset,
            is_buy,
            limit_price,
            size,
            reduce_only,
            order_type,
            client_order_id,
        })
    }
}

/// Grouping mode for order actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grouping {
    /// Independent order or orders.
    #[serde(rename = "na")]
    None,
    /// Normal take-profit/stop-loss grouping.
    #[serde(rename = "normalTpsl")]
    NormalTpsl,
    /// Position take-profit/stop-loss grouping.
    #[serde(rename = "positionTpsl")]
    PositionTpsl,
}

/// Optional builder fee attribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuilderWire {
    /// Builder address.
    #[serde(rename = "b")]
    pub address: EvmAddress,
    /// Fee in tenths of a basis point.
    #[serde(rename = "f")]
    pub fee: u32,
}

impl BuilderWire {
    /// Creates validated builder attribution metadata.
    #[must_use]
    pub const fn new(address: EvmAddress, fee: u32) -> Self {
        Self { address, fee }
    }
}

/// Batch order action submitted to the exchange endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderAction {
    /// Action discriminator. Always `order`.
    #[serde(rename = "type")]
    action_type: OrderActionType,
    /// One or more orders.
    pub orders: Vec<OrderWire>,
    /// Grouping behavior.
    pub grouping: Grouping,
    /// Optional builder fee attribution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderWire>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum OrderActionType {
    #[serde(rename = "order")]
    Order,
}

impl OrderAction {
    /// Creates a structurally valid order action.
    ///
    /// # Errors
    ///
    /// Returns [`ProtocolError::InvalidAction`] when the order batch is empty.
    pub fn new(
        orders: Vec<OrderWire>,
        grouping: Grouping,
        builder: Option<BuilderWire>,
    ) -> Result<Self, ProtocolError> {
        if orders.is_empty() {
            return Err(ProtocolError::InvalidAction(
                "order batch must not be empty",
            ));
        }
        Ok(Self {
            action_type: OrderActionType::Order,
            orders,
            grouping,
            builder,
        })
    }
}

/// Exchange-assigned numeric order ID or caller-assigned Cloid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderId {
    /// Exchange-assigned order ID.
    Oid(u64),
    /// Caller-assigned 128-bit client order ID.
    Cloid(ClientOrderId),
}

/// Compact cancellation by exchange order ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelWire {
    /// Asset ID.
    #[serde(rename = "a")]
    pub asset: AssetId,
    /// Exchange-assigned order ID.
    #[serde(rename = "o")]
    pub oid: u64,
}

/// Batch cancellation action by exchange order ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelAction {
    /// Action discriminator. Always `cancel`.
    #[serde(rename = "type")]
    action_type: CancelActionType,
    /// One or more cancellations.
    pub cancels: Vec<CancelWire>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CancelActionType {
    #[serde(rename = "cancel")]
    Cancel,
}

impl CancelAction {
    /// Creates a non-empty cancellation batch.
    ///
    /// # Errors
    ///
    /// Returns an error when no cancellations are provided.
    pub fn new(cancels: Vec<CancelWire>) -> Result<Self, ProtocolError> {
        require_non_empty(&cancels, "cancel batch must not be empty")?;
        Ok(Self {
            action_type: CancelActionType::Cancel,
            cancels,
        })
    }
}

/// Compact cancellation by client order ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelByCloidWire {
    /// Asset ID. The official wire key is intentionally not compacted.
    pub asset: AssetId,
    /// Caller-assigned 128-bit client order ID.
    pub cloid: ClientOrderId,
}

/// Batch cancellation action by client order ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelByCloidAction {
    /// Action discriminator. Always `cancelByCloid`.
    #[serde(rename = "type")]
    action_type: CancelByCloidActionType,
    /// One or more cancellations.
    pub cancels: Vec<CancelByCloidWire>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CancelByCloidActionType {
    #[serde(rename = "cancelByCloid")]
    CancelByCloid,
}

impl CancelByCloidAction {
    /// Creates a non-empty Cloid cancellation batch.
    ///
    /// # Errors
    ///
    /// Returns an error when no cancellations are provided.
    pub fn new(cancels: Vec<CancelByCloidWire>) -> Result<Self, ProtocolError> {
        require_non_empty(&cancels, "cloid cancel batch must not be empty")?;
        Ok(Self {
            action_type: CancelByCloidActionType::CancelByCloid,
            cancels,
        })
    }
}

/// One order replacement in a batch-modify action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifyWire {
    /// Existing order selected by OID or Cloid.
    pub oid: OrderId,
    /// Complete replacement order.
    pub order: OrderWire,
}

/// Batch order replacement action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchModifyAction {
    /// Action discriminator. Always `batchModify`.
    #[serde(rename = "type")]
    action_type: BatchModifyActionType,
    /// One or more order replacements.
    pub modifies: Vec<ModifyWire>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum BatchModifyActionType {
    #[serde(rename = "batchModify")]
    BatchModify,
}

impl BatchModifyAction {
    /// Creates a non-empty order replacement batch.
    ///
    /// # Errors
    ///
    /// Returns an error when no modifications are provided.
    pub fn new(modifies: Vec<ModifyWire>) -> Result<Self, ProtocolError> {
        require_non_empty(&modifies, "modify batch must not be empty")?;
        Ok(Self {
            action_type: BatchModifyActionType::BatchModify,
            modifies,
        })
    }
}

/// Schedules or clears Hyperliquid's cancel-all dead man's switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleCancelAction {
    /// Action discriminator. Always `scheduleCancel`.
    #[serde(rename = "type")]
    action_type: ScheduleCancelActionType,
    /// UTC millisecond deadline. Omitted to clear an existing schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ScheduleCancelActionType {
    #[serde(rename = "scheduleCancel")]
    ScheduleCancel,
}

impl ScheduleCancelAction {
    /// Creates a schedule-set or schedule-clear action.
    #[must_use]
    pub const fn new(time: Option<u64>) -> Self {
        Self {
            action_type: ScheduleCancelActionType::ScheduleCancel,
            time,
        }
    }
}

/// Compact TWAP order payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwapWire {
    /// Asset ID.
    #[serde(rename = "a")]
    pub asset: AssetId,
    /// Buy when true, sell when false.
    #[serde(rename = "b")]
    pub is_buy: bool,
    /// Total TWAP size.
    #[serde(rename = "s")]
    pub size: DecimalValue,
    /// Whether execution may only reduce an existing position.
    #[serde(rename = "r")]
    pub reduce_only: bool,
    /// Duration in minutes.
    #[serde(rename = "m")]
    pub minutes: u32,
    /// Whether slice timing is randomized.
    #[serde(rename = "t")]
    pub randomize: bool,
}

impl TwapWire {
    /// Creates a structurally valid TWAP payload.
    ///
    /// # Errors
    ///
    /// Returns an error for non-positive size or zero duration. Additional
    /// market-specific duration and notional checks remain server-authoritative.
    pub fn new(
        asset: AssetId,
        is_buy: bool,
        size: DecimalValue,
        reduce_only: bool,
        minutes: u32,
        randomize: bool,
    ) -> Result<Self, ProtocolError> {
        if !size.is_positive() {
            return Err(ProtocolError::InvalidAction("TWAP size must be positive"));
        }
        if minutes == 0 {
            return Err(ProtocolError::InvalidAction(
                "TWAP duration must be greater than zero",
            ));
        }
        Ok(Self {
            asset,
            is_buy,
            size,
            reduce_only,
            minutes,
            randomize,
        })
    }
}

/// Places one native TWAP order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwapOrderAction {
    /// Action discriminator. Always `twapOrder`.
    #[serde(rename = "type")]
    action_type: TwapOrderActionType,
    /// TWAP parameters.
    pub twap: TwapWire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TwapOrderActionType {
    #[serde(rename = "twapOrder")]
    TwapOrder,
}

impl TwapOrderAction {
    /// Creates a TWAP order action from a validated payload.
    #[must_use]
    pub const fn new(twap: TwapWire) -> Self {
        Self {
            action_type: TwapOrderActionType::TwapOrder,
            twap,
        }
    }
}

/// Cancels one native TWAP order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwapCancelAction {
    /// Action discriminator. Always `twapCancel`.
    #[serde(rename = "type")]
    action_type: TwapCancelActionType,
    /// Asset ID.
    #[serde(rename = "a")]
    pub asset: AssetId,
    /// Exchange-assigned TWAP ID.
    #[serde(rename = "t")]
    pub twap_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TwapCancelActionType {
    #[serde(rename = "twapCancel")]
    TwapCancel,
}

impl TwapCancelAction {
    /// Creates a TWAP cancellation action.
    #[must_use]
    pub const fn new(asset: AssetId, twap_id: u64) -> Self {
        Self {
            action_type: TwapCancelActionType::TwapCancel,
            asset,
            twap_id,
        }
    }
}

/// Updates cross or isolated leverage for one perpetual asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverageAction {
    /// Action discriminator. Always `updateLeverage`.
    #[serde(rename = "type")]
    action_type: UpdateLeverageActionType,
    /// Perpetual asset ID.
    pub asset: AssetId,
    /// Cross margin when true, isolated margin when false.
    pub is_cross: bool,
    /// Requested integer leverage.
    pub leverage: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UpdateLeverageActionType {
    #[serde(rename = "updateLeverage")]
    UpdateLeverage,
}

impl UpdateLeverageAction {
    /// Creates a leverage update checked against current market metadata.
    ///
    /// # Errors
    ///
    /// Returns an error when `max_leverage` is zero or the requested leverage
    /// is outside `1..=max_leverage`.
    pub fn new(
        asset: AssetId,
        is_cross: bool,
        leverage: u32,
        max_leverage: u32,
    ) -> Result<Self, ProtocolError> {
        if max_leverage == 0 {
            return Err(ProtocolError::InvalidAction(
                "market max leverage must be greater than zero",
            ));
        }
        if leverage == 0 || leverage > max_leverage {
            return Err(ProtocolError::InvalidAction(
                "leverage must be between one and the market maximum",
            ));
        }
        Ok(Self {
            action_type: UpdateLeverageActionType::UpdateLeverage,
            asset,
            is_cross,
            leverage,
        })
    }
}

/// Adds or removes six-decimal USD margin from an isolated position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMarginAction {
    /// Action discriminator. Always `updateIsolatedMargin`.
    #[serde(rename = "type")]
    action_type: UpdateIsolatedMarginActionType,
    /// Perpetual asset ID.
    pub asset: AssetId,
    /// Position side. Currently ignored by the server until hedge mode exists.
    pub is_buy: bool,
    /// Signed USD delta scaled by 10^6.
    pub ntli: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum UpdateIsolatedMarginActionType {
    #[serde(rename = "updateIsolatedMargin")]
    UpdateIsolatedMargin,
}

impl UpdateIsolatedMarginAction {
    /// Creates a non-zero isolated-margin delta.
    ///
    /// # Errors
    ///
    /// Returns an error when `ntli` is zero.
    pub fn new(asset: AssetId, is_buy: bool, ntli: i64) -> Result<Self, ProtocolError> {
        if ntli == 0 {
            return Err(ProtocolError::InvalidAction(
                "isolated margin delta must not be zero",
            ));
        }
        Ok(Self {
            action_type: UpdateIsolatedMarginActionType::UpdateIsolatedMargin,
            asset,
            is_buy,
            ntli,
        })
    }
}

/// Targets a leverage level for an isolated-only perpetual position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopUpIsolatedOnlyMarginAction {
    /// Action discriminator. Always `topUpIsolatedOnlyMargin`.
    #[serde(rename = "type")]
    action_type: TopUpIsolatedOnlyMarginActionType,
    /// Perpetual asset ID.
    pub asset: AssetId,
    /// Positive target leverage represented as an exact decimal string.
    pub leverage: DecimalValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TopUpIsolatedOnlyMarginActionType {
    #[serde(rename = "topUpIsolatedOnlyMargin")]
    TopUpIsolatedOnlyMargin,
}

impl TopUpIsolatedOnlyMarginAction {
    /// Creates an isolated-only margin top-up action.
    ///
    /// # Errors
    ///
    /// Returns an error when target leverage is not positive.
    pub fn new(asset: AssetId, leverage: DecimalValue) -> Result<Self, ProtocolError> {
        if !leverage.is_positive() {
            return Err(ProtocolError::InvalidAction(
                "isolated-only target leverage must be positive",
            ));
        }
        Ok(Self {
            action_type: TopUpIsolatedOnlyMarginActionType::TopUpIsolatedOnlyMargin,
            asset,
            leverage,
        })
    }
}

/// Creates a named subaccount owned by the signing master account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountAction {
    /// Action discriminator. Always `createSubAccount`.
    #[serde(rename = "type")]
    action_type: CreateSubAccountActionType,
    /// User-visible subaccount name.
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CreateSubAccountActionType {
    #[serde(rename = "createSubAccount")]
    CreateSubAccount,
}

impl CreateSubAccountAction {
    /// Creates a structurally valid subaccount action.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty name or control characters. Account
    /// limits and name uniqueness remain server-authoritative.
    pub fn new(name: String) -> Result<Self, ProtocolError> {
        if name.is_empty() || name.chars().any(char::is_control) {
            return Err(ProtocolError::InvalidAction(
                "subaccount name must be non-empty and contain no control characters",
            ));
        }
        Ok(Self {
            action_type: CreateSubAccountActionType::CreateSubAccount,
            name,
        })
    }
}

/// Transfers six-decimal USD between a master account and its subaccount.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferAction {
    /// Action discriminator. Always `subAccountTransfer`.
    #[serde(rename = "type")]
    action_type: SubAccountTransferActionType,
    /// Target subaccount address.
    pub sub_account_user: EvmAddress,
    /// `true` for master to subaccount; `false` for subaccount to master.
    pub is_deposit: bool,
    /// Positive USD amount scaled by 10^6.
    pub usd: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SubAccountTransferActionType {
    #[serde(rename = "subAccountTransfer")]
    SubAccountTransfer,
}

impl SubAccountTransferAction {
    /// Creates a positive USD subaccount transfer.
    ///
    /// # Errors
    ///
    /// Returns an error when `usd` is zero.
    pub fn new(
        sub_account_user: EvmAddress,
        is_deposit: bool,
        usd: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_protocol_integer(usd, "subaccount USD amount must be positive")?;
        Ok(Self {
            action_type: SubAccountTransferActionType::SubAccountTransfer,
            sub_account_user,
            is_deposit,
            usd,
        })
    }
}

/// Transfers a spot token between a master account and its subaccount.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountSpotTransferAction {
    /// Action discriminator. Always `subAccountSpotTransfer`.
    #[serde(rename = "type")]
    action_type: SubAccountSpotTransferActionType,
    /// Target subaccount address.
    pub sub_account_user: EvmAddress,
    /// `true` for master to subaccount; `false` for subaccount to master.
    pub is_deposit: bool,
    /// Hyperliquid token identifier such as `PURR:0x...`.
    pub token: String,
    /// Positive exact token amount.
    pub amount: DecimalValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SubAccountSpotTransferActionType {
    #[serde(rename = "subAccountSpotTransfer")]
    SubAccountSpotTransfer,
}

impl SubAccountSpotTransferAction {
    /// Creates a positive spot-token subaccount transfer.
    ///
    /// # Errors
    ///
    /// Returns an error for a malformed token identifier or non-positive
    /// amount.
    pub fn new(
        sub_account_user: EvmAddress,
        is_deposit: bool,
        token: String,
        amount: DecimalValue,
    ) -> Result<Self, ProtocolError> {
        validate_spot_token_identifier(&token)?;
        require_positive_amount(&amount)?;
        Ok(Self {
            action_type: SubAccountSpotTransferActionType::SubAccountSpotTransfer,
            sub_account_user,
            is_deposit,
            token,
            amount,
        })
    }
}

/// Deposits or withdraws six-decimal USD to or from a vault.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransferAction {
    /// Action discriminator. Always `vaultTransfer`.
    #[serde(rename = "type")]
    action_type: VaultTransferActionType,
    /// Target vault address.
    pub vault_address: EvmAddress,
    /// `true` to deposit; `false` to withdraw.
    pub is_deposit: bool,
    /// Positive USD amount scaled by 10^6.
    pub usd: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum VaultTransferActionType {
    #[serde(rename = "vaultTransfer")]
    VaultTransfer,
}

impl VaultTransferAction {
    /// Creates a positive vault USD transfer.
    ///
    /// # Errors
    ///
    /// Returns an error when `usd` is zero.
    pub fn new(
        vault_address: EvmAddress,
        is_deposit: bool,
        usd: u64,
    ) -> Result<Self, ProtocolError> {
        require_positive_protocol_integer(usd, "vault USD amount must be positive")?;
        Ok(Self {
            action_type: VaultTransferActionType::VaultTransfer,
            vault_address,
            is_deposit,
            usd,
        })
    }
}

/// Registers a referral code for the signing user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrerAction {
    /// Action discriminator. Always `setReferrer`.
    #[serde(rename = "type")]
    action_type: SetReferrerActionType,
    /// Referral code interpreted by the exchange.
    pub code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum SetReferrerActionType {
    #[serde(rename = "setReferrer")]
    SetReferrer,
}

impl SetReferrerAction {
    /// Creates a structurally valid referral action.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty code or control characters. Code
    /// existence and account eligibility remain server-authoritative.
    pub fn new(code: String) -> Result<Self, ProtocolError> {
        if code.is_empty() || code.chars().any(char::is_control) {
            return Err(ProtocolError::InvalidAction(
                "referral code must be non-empty and contain no control characters",
            ));
        }
        Ok(Self {
            action_type: SetReferrerActionType::SetReferrer,
            code,
        })
    }
}

/// Changes whether the user's EVM account consumes large blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmUserModifyAction {
    /// Action discriminator. Always `evmUserModify`.
    #[serde(rename = "type")]
    action_type: EvmUserModifyActionType,
    /// Whether large EVM blocks are enabled.
    pub using_big_blocks: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum EvmUserModifyActionType {
    #[serde(rename = "evmUserModify")]
    EvmUserModify,
}

impl EvmUserModifyAction {
    /// Creates an EVM large-block preference update.
    #[must_use]
    pub const fn new(using_big_blocks: bool) -> Self {
        Self {
            action_type: EvmUserModifyActionType::EvmUserModify,
            using_big_blocks,
        }
    }
}

/// Agent-side account abstraction mode encoded by the L1 protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentAbstraction {
    /// Disable abstraction.
    #[serde(rename = "i")]
    Disabled,
    /// Use unified-account abstraction.
    #[serde(rename = "u")]
    UnifiedAccount,
    /// Use portfolio-margin abstraction.
    #[serde(rename = "p")]
    PortfolioMargin,
}

/// Enables legacy DEX abstraction for an API wallet or agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentEnableDexAbstractionAction {
    /// Action discriminator. Always `agentEnableDexAbstraction`.
    #[serde(rename = "type")]
    action_type: AgentEnableDexAbstractionActionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum AgentEnableDexAbstractionActionType {
    #[serde(rename = "agentEnableDexAbstraction")]
    AgentEnableDexAbstraction,
}

impl AgentEnableDexAbstractionAction {
    /// Creates the legacy agent DEX-abstraction action.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            action_type: AgentEnableDexAbstractionActionType::AgentEnableDexAbstraction,
        }
    }
}

impl Default for AgentEnableDexAbstractionAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Sets an API wallet or agent account abstraction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSetAbstractionAction {
    /// Action discriminator. Always `agentSetAbstraction`.
    #[serde(rename = "type")]
    action_type: AgentSetAbstractionActionType,
    /// Compact mode value required by the L1 wire protocol.
    pub abstraction: AgentAbstraction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum AgentSetAbstractionActionType {
    #[serde(rename = "agentSetAbstraction")]
    AgentSetAbstraction,
}

impl AgentSetAbstractionAction {
    /// Creates an agent account-abstraction update.
    #[must_use]
    pub const fn new(abstraction: AgentAbstraction) -> Self {
        Self {
            action_type: AgentSetAbstractionActionType::AgentSetAbstraction,
            abstraction,
        }
    }
}

/// Empty L1 action useful for explicit nonce consumption and protocol checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoopAction {
    /// Action discriminator. Always `noop`.
    #[serde(rename = "type")]
    action_type: NoopActionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum NoopActionType {
    #[serde(rename = "noop")]
    Noop,
}

impl NoopAction {
    /// Creates an empty L1 action.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            action_type: NoopActionType::Noop,
        }
    }
}

impl Default for NoopAction {
    fn default() -> Self {
        Self::new()
    }
}
