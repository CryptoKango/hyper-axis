/// Prepares a validated order, action hash, and platform signing digest.
///
/// # Errors
///
/// Returns a display-safe error when any protocol or serialization rule fails.
pub fn prepare_limit_order(request: PrepareLimitOrderRequest) -> Result<PreparedOrder, String> {
    hyper_axis_core::client::prepare_limit_order(request).map_err(|error| error.to_string())
}

/// Prepares an atomic batch of validated limit orders.
///
/// # Errors
///
/// Returns a display-safe error when any order or signing-context rule fails.
pub fn prepare_batch_orders(request: PrepareBatchOrdersRequest) -> Result<PreparedOrder, String> {
    hyper_axis_core::client::prepare_batch_orders(request).map_err(|error| error.to_string())
}

/// Prepares a cancellation batch by exchange order ID.
///
/// # Errors
///
/// Returns a display-safe error when the batch or signing context is invalid.
pub fn prepare_cancel(request: PrepareCancelRequest) -> Result<PreparedCancel, String> {
    hyper_axis_core::client::prepare_cancel(request).map_err(|error| error.to_string())
}

/// Prepares a cancellation batch by client order ID.
///
/// # Errors
///
/// Returns a display-safe error when a Cloid, batch, or signing context is
/// invalid.
pub fn prepare_cancel_by_cloid(
    request: PrepareCancelByCloidRequest,
) -> Result<PreparedCancelByCloid, String> {
    hyper_axis_core::client::prepare_cancel_by_cloid(request).map_err(|error| error.to_string())
}

/// Prepares a batch of complete order replacements.
///
/// # Errors
///
/// Returns a display-safe error when any selector, replacement order, or
/// signing-context rule fails.
pub fn prepare_batch_modify(
    request: PrepareBatchModifyRequest,
) -> Result<PreparedBatchModify, String> {
    hyper_axis_core::client::prepare_batch_modify(request).map_err(|error| error.to_string())
}

/// Prepares a mixed atomic batch of limit and trigger orders.
///
/// # Errors
///
/// Returns a display-safe error when any order or signing-context rule fails.
pub fn prepare_orders(request: PrepareOrdersRequest) -> Result<PreparedOrder, String> {
    hyper_axis_core::client::prepare_orders(request).map_err(|error| error.to_string())
}

/// Prepares a dead man's switch set or clear action.
///
/// # Errors
///
/// Returns a display-safe error when the deadline or signing context is invalid.
pub fn prepare_schedule_cancel(
    request: PrepareScheduleCancelRequest,
) -> Result<PreparedScheduleCancel, String> {
    hyper_axis_core::client::prepare_schedule_cancel(request).map_err(|error| error.to_string())
}

/// Prepares a native TWAP order.
///
/// # Errors
///
/// Returns a display-safe error when size, duration, or signing context is invalid.
pub fn prepare_twap_order(request: PrepareTwapOrderRequest) -> Result<PreparedTwapOrder, String> {
    hyper_axis_core::client::prepare_twap_order(request).map_err(|error| error.to_string())
}

/// Prepares a native TWAP cancellation.
///
/// # Errors
///
/// Returns a display-safe error when the signing context is invalid.
pub fn prepare_twap_cancel(
    request: PrepareTwapCancelRequest,
) -> Result<PreparedTwapCancel, String> {
    hyper_axis_core::client::prepare_twap_cancel(request).map_err(|error| error.to_string())
}

/// Prepares a validated cross/isolated leverage update.
///
/// # Errors
///
/// Returns a display-safe error for invalid leverage, address, or signing
/// context.
pub fn prepare_update_leverage(
    request: PrepareUpdateLeverageRequest,
) -> Result<PreparedUpdateLeverage, String> {
    hyper_axis_core::client::prepare_update_leverage(request).map_err(|error| error.to_string())
}

/// Prepares an exact isolated-margin USD delta.
///
/// # Errors
///
/// Returns a display-safe error for invalid precision/range, address, or
/// signing context.
pub fn prepare_update_isolated_margin(
    request: PrepareUpdateIsolatedMarginRequest,
) -> Result<PreparedUpdateIsolatedMargin, String> {
    hyper_axis_core::client::prepare_update_isolated_margin(request)
        .map_err(|error| error.to_string())
}

/// Prepares an isolated-only margin top-up targeting leverage.
///
/// # Errors
///
/// Returns a display-safe error for invalid leverage, address, or signing
/// context.
pub fn prepare_top_up_isolated_only_margin(
    request: PrepareTopUpIsolatedOnlyMarginRequest,
) -> Result<PreparedTopUpIsolatedOnlyMargin, String> {
    hyper_axis_core::client::prepare_top_up_isolated_only_margin(request)
        .map_err(|error| error.to_string())
}

/// Prepares named-subaccount creation.
///
/// # Errors
///
/// Returns a display-safe error for invalid names or signing context.
pub fn prepare_create_sub_account(
    request: PrepareCreateSubAccountRequest,
) -> Result<PreparedCreateSubAccount, String> {
    hyper_axis_core::client::prepare_create_sub_account(request).map_err(|error| error.to_string())
}

/// Prepares a USD transfer between a master account and subaccount.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses, amounts, or signing
/// context.
pub fn prepare_sub_account_transfer(
    request: PrepareSubAccountTransferRequest,
) -> Result<PreparedSubAccountTransfer, String> {
    hyper_axis_core::client::prepare_sub_account_transfer(request)
        .map_err(|error| error.to_string())
}

/// Prepares a spot-token transfer between a master account and subaccount.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses, token, amount, or
/// signing context.
pub fn prepare_sub_account_spot_transfer(
    request: PrepareSubAccountSpotTransferRequest,
) -> Result<PreparedSubAccountSpotTransfer, String> {
    hyper_axis_core::client::prepare_sub_account_spot_transfer(request)
        .map_err(|error| error.to_string())
}

/// Prepares a vault USD deposit or withdrawal.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses, amounts, or signing
/// context.
pub fn prepare_vault_transfer(
    request: PrepareVaultTransferRequest,
) -> Result<PreparedVaultTransfer, String> {
    hyper_axis_core::client::prepare_vault_transfer(request).map_err(|error| error.to_string())
}

/// Prepares referral-code registration.
///
/// # Errors
///
/// Returns a display-safe error for invalid codes or signing context.
pub fn prepare_set_referrer(
    request: PrepareSetReferrerRequest,
) -> Result<PreparedSetReferrer, String> {
    hyper_axis_core::client::prepare_set_referrer(request).map_err(|error| error.to_string())
}

/// Prepares an EVM large-block preference update.
///
/// # Errors
///
/// Returns a display-safe error for invalid signing context.
pub fn prepare_evm_user_modify(
    request: PrepareEvmUserModifyRequest,
) -> Result<PreparedEvmUserModify, String> {
    hyper_axis_core::client::prepare_evm_user_modify(request).map_err(|error| error.to_string())
}

/// Prepares legacy DEX-abstraction enablement for an agent.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses or signing context.
pub fn prepare_agent_enable_dex_abstraction(
    request: PrepareAgentEnableDexAbstractionRequest,
) -> Result<PreparedAgentEnableDexAbstraction, String> {
    hyper_axis_core::client::prepare_agent_enable_dex_abstraction(request)
        .map_err(|error| error.to_string())
}

/// Prepares an agent account-abstraction mode update.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses or signing context.
pub fn prepare_agent_set_abstraction(
    request: PrepareAgentSetAbstractionRequest,
) -> Result<PreparedAgentSetAbstraction, String> {
    hyper_axis_core::client::prepare_agent_set_abstraction(request)
        .map_err(|error| error.to_string())
}

/// Prepares an empty L1 action.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses or signing context.
pub fn prepare_noop(request: PrepareNoopRequest) -> Result<PreparedNoop, String> {
    hyper_axis_core::client::prepare_noop(request).map_err(|error| error.to_string())
}

/// Prepares signer-node jail or unjail self-management.
///
/// # Errors
/// Returns a display-safe error for invalid input or signing context.
pub fn prepare_c_signer(request: PrepareCSignerRequest) -> Result<PreparedCSigner, String> {
    hyper_axis_core::client::prepare_c_signer(request).map_err(|error| error.to_string())
}

/// Prepares consensus validator registration.
///
/// # Errors
/// Returns a display-safe error for invalid profile data or signing context.
pub fn prepare_c_validator_register(
    request: PrepareCValidatorRegisterRequest,
) -> Result<PreparedCValidator, String> {
    hyper_axis_core::client::prepare_c_validator_register(request)
        .map_err(|error| error.to_string())
}

/// Prepares a consensus validator profile update.
///
/// # Errors
/// Returns a display-safe error for invalid profile data or signing context.
pub fn prepare_c_validator_change_profile(
    request: PrepareCValidatorChangeProfileRequest,
) -> Result<PreparedCValidator, String> {
    hyper_axis_core::client::prepare_c_validator_change_profile(request)
        .map_err(|error| error.to_string())
}

/// Prepares consensus validator unregistration.
///
/// # Errors
/// Returns a display-safe error for invalid signing context.
pub fn prepare_c_validator_unregister(
    request: PrepareCValidatorUnregisterRequest,
) -> Result<PreparedCValidator, String> {
    hyper_axis_core::client::prepare_c_validator_unregister(request)
        .map_err(|error| error.to_string())
}

/// Prepares a gossip priority gas bid.
///
/// # Errors
/// Returns a display-safe error for invalid bid data or signing context.
pub fn prepare_gossip_priority_bid(
    request: PrepareGossipPriorityBidRequest,
) -> Result<PreparedGossipPriorityBid, String> {
    hyper_axis_core::client::prepare_gossip_priority_bid(request).map_err(|error| error.to_string())
}

/// Prepares any supported Spot deployer lifecycle action.
///
/// # Errors
/// Returns a display-safe error for invalid deployer data or signing context.
pub fn prepare_spot_deploy(
    request: PrepareSpotDeployRequest,
) -> Result<PreparedSpotDeploy, String> {
    hyper_axis_core::client::prepare_spot_deploy(request).map_err(|error| error.to_string())
}

/// Prepares any supported perpetual deployer lifecycle action.
///
/// # Errors
/// Returns a display-safe error for invalid deployer data or signing context.
pub fn prepare_perp_deploy(
    request: PreparePerpDeployRequest,
) -> Result<PreparedPerpDeploy, String> {
    hyper_axis_core::client::prepare_perp_deploy(request).map_err(|error| error.to_string())
}

/// Prepares a Core USDC transfer using the user-action EIP-712 domain.
///
/// # Errors
///
/// Returns a display-safe error for invalid destination, amount, or
/// serialization.
pub fn prepare_usd_send(request: PrepareUsdSendRequest) -> Result<PreparedUsdSend, String> {
    hyper_axis_core::client::prepare_usd_send(request).map_err(|error| error.to_string())
}

/// Prepares a Core spot asset transfer using the user-action EIP-712 domain.
///
/// # Errors
///
/// Returns a display-safe error for invalid destination, token, amount, or
/// serialization.
pub fn prepare_spot_send(request: PrepareSpotSendRequest) -> Result<PreparedSpotSend, String> {
    hyper_axis_core::client::prepare_spot_send(request).map_err(|error| error.to_string())
}

/// Prepares a bridge withdrawal using the user-action EIP-712 domain.
///
/// # Errors
///
/// Returns a display-safe error for invalid destination, amount, or
/// serialization.
pub fn prepare_withdraw(request: PrepareWithdrawRequest) -> Result<PreparedWithdraw, String> {
    hyper_axis_core::client::prepare_withdraw(request).map_err(|error| error.to_string())
}

/// Prepares an API wallet authorization without handling its private key.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses or serialization.
pub fn prepare_approve_agent(
    request: PrepareApproveAgentRequest,
) -> Result<PreparedApproveAgent, String> {
    hyper_axis_core::client::prepare_approve_agent(request).map_err(|error| error.to_string())
}

/// Prepares a builder fee authorization.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses, percentage strings, or
/// serialization.
pub fn prepare_approve_builder_fee(
    request: PrepareApproveBuilderFeeRequest,
) -> Result<PreparedApproveBuilderFee, String> {
    hyper_axis_core::client::prepare_approve_builder_fee(request).map_err(|error| error.to_string())
}

/// Prepares a spot/perp USD class transfer.
///
/// # Errors
///
/// Returns a display-safe error for invalid amounts, subaccounts, or
/// serialization.
pub fn prepare_usd_class_transfer(
    request: PrepareUsdClassTransferRequest,
) -> Result<PreparedUsdClassTransfer, String> {
    hyper_axis_core::client::prepare_usd_class_transfer(request).map_err(|error| error.to_string())
}

/// Prepares a generalized asset transfer.
///
/// # Errors
///
/// Returns a display-safe error for invalid addresses, DEX names, token,
/// amount, or serialization.
pub fn prepare_send_asset(request: PrepareSendAssetRequest) -> Result<PreparedSendAsset, String> {
    hyper_axis_core::client::prepare_send_asset(request).map_err(|error| error.to_string())
}

/// Prepares a native-token staking deposit.
///
/// # Errors
///
/// Returns a display-safe error for zero amounts or serialization.
pub fn prepare_c_deposit(request: PrepareCDepositRequest) -> Result<PreparedCDeposit, String> {
    hyper_axis_core::client::prepare_c_deposit(request).map_err(|error| error.to_string())
}

/// Prepares a native-token staking withdrawal.
///
/// # Errors
///
/// Returns a display-safe error for zero amounts or serialization.
pub fn prepare_c_withdraw(request: PrepareCWithdrawRequest) -> Result<PreparedCWithdraw, String> {
    hyper_axis_core::client::prepare_c_withdraw(request).map_err(|error| error.to_string())
}

/// Prepares a validator delegation or undelegation.
///
/// # Errors
///
/// Returns a display-safe error for invalid validators, zero amounts, or
/// serialization.
pub fn prepare_token_delegate(
    request: PrepareTokenDelegateRequest,
) -> Result<PreparedTokenDelegate, String> {
    hyper_axis_core::client::prepare_token_delegate(request).map_err(|error| error.to_string())
}

/// Prepares the legacy user DEX-abstraction toggle.
///
/// # Errors
///
/// Returns a display-safe error for invalid user addresses or serialization.
pub fn prepare_user_dex_abstraction(
    request: PrepareUserDexAbstractionRequest,
) -> Result<PreparedUserDexAbstraction, String> {
    hyper_axis_core::client::prepare_user_dex_abstraction(request)
        .map_err(|error| error.to_string())
}

/// Prepares an account-abstraction mode update.
///
/// # Errors
///
/// Returns a display-safe error for invalid user addresses or serialization.
pub fn prepare_user_set_abstraction(
    request: PrepareUserSetAbstractionRequest,
) -> Result<PreparedUserSetAbstraction, String> {
    hyper_axis_core::client::prepare_user_set_abstraction(request)
        .map_err(|error| error.to_string())
}

/// Prepares conversion to a threshold multi-signature user.
///
/// # Errors
///
/// Returns a display-safe error for malformed/duplicate addresses, invalid
/// thresholds, or serialization.
pub fn prepare_convert_to_multi_sig_user(
    request: PrepareConvertToMultiSigUserRequest,
) -> Result<PreparedConvertToMultiSigUser, String> {
    hyper_axis_core::client::prepare_convert_to_multi_sig_user(request)
        .map_err(|error| error.to_string())
}
