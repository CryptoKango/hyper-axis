/// Prepares the authorized-user digest for a multi-signature L1 order.
///
/// # Errors
///
/// Returns a display-safe error for modified source artifacts, invalid account
/// addresses, or encoding failure.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_order_payload(
    prepared: PreparedOrder,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_order_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature OID cancel.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_cancel_payload(
    prepared: PreparedCancel,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_cancel_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature Cloid cancel.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_cancel_by_cloid_payload(
    prepared: PreparedCancelByCloid,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_cancel_by_cloid_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature batch modify.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_batch_modify_payload(
    prepared: PreparedBatchModify,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_batch_modify_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature schedule cancel.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_schedule_cancel_payload(
    prepared: PreparedScheduleCancel,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_schedule_cancel_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature TWAP order.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_twap_order_payload(
    prepared: PreparedTwapOrder,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_twap_order_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature TWAP cancel.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_twap_cancel_payload(
    prepared: PreparedTwapCancel,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_twap_cancel_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature leverage update.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_update_leverage_payload(
    prepared: PreparedUpdateLeverage,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_update_leverage_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature isolated-margin
/// update.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_update_isolated_margin_payload(
    prepared: PreparedUpdateIsolatedMargin,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_update_isolated_margin_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature isolated-only
/// margin top-up.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_top_up_isolated_only_margin_payload(
    prepared: PreparedTopUpIsolatedOnlyMargin,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_top_up_isolated_only_margin_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for named-subaccount creation.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_create_sub_account_payload(
    prepared: PreparedCreateSubAccount,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_create_sub_account_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a USD subaccount transfer.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_sub_account_transfer_payload(
    prepared: PreparedSubAccountTransfer,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_sub_account_transfer_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a spot subaccount transfer.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_sub_account_spot_transfer_payload(
    prepared: PreparedSubAccountSpotTransfer,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_sub_account_spot_transfer_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a vault transfer.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_vault_transfer_payload(
    prepared: PreparedVaultTransfer,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_vault_transfer_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for referral-code registration.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_set_referrer_payload(
    prepared: PreparedSetReferrer,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_set_referrer_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for an EVM-user update.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_evm_user_modify_payload(
    prepared: PreparedEvmUserModify,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_evm_user_modify_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for legacy agent abstraction.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_agent_enable_dex_abstraction_payload(
    prepared: PreparedAgentEnableDexAbstraction,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_agent_enable_dex_abstraction_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for an agent abstraction update.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_agent_set_abstraction_payload(
    prepared: PreparedAgentSetAbstraction,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_agent_set_abstraction_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for an L1 no-op.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_noop_payload(
    prepared: PreparedNoop,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_noop_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for signer-node self-management.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_c_signer_payload(
    prepared: PreparedCSigner,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_c_signer_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a validator lifecycle action.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_c_validator_payload(
    prepared: PreparedCValidator,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_c_validator_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a gossip priority bid.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_gossip_priority_bid_payload(
    prepared: PreparedGossipPriorityBid,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_gossip_priority_bid_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a Spot deployer action.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_spot_deploy_payload(
    prepared: PreparedSpotDeploy,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_spot_deploy_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a perpetual deployer action.
///
/// # Errors
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_perp_deploy_payload(
    prepared: PreparedPerpDeploy,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_perp_deploy_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for multi-signature `usdSend`.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_usd_send_payload(
    prepared: PreparedUsdSend,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_usd_send_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for multi-signature `spotSend`.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_spot_send_payload(
    prepared: PreparedSpotSend,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_spot_send_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature withdrawal.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_withdraw_payload(
    prepared: PreparedWithdraw,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_withdraw_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for multi-signature API-wallet approval.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_approve_agent_payload(
    prepared: PreparedApproveAgent,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_approve_agent_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for multi-signature builder approval.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_approve_builder_fee_payload(
    prepared: PreparedApproveBuilderFee,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_approve_builder_fee_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature USD class transfer.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_usd_class_transfer_payload(
    prepared: PreparedUsdClassTransfer,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_usd_class_transfer_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for multi-signature `sendAsset`.
///
/// # Errors
///
/// Returns a display-safe error for modified source artifacts, invalid account
/// addresses, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_send_asset_payload(
    prepared: PreparedSendAsset,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_send_asset_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature staking deposit.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_c_deposit_payload(
    prepared: PreparedCDeposit,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_c_deposit_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature staking withdrawal.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_c_withdraw_payload(
    prepared: PreparedCWithdraw,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_c_withdraw_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for multi-signature delegation.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_token_delegate_payload(
    prepared: PreparedTokenDelegate,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_token_delegate_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for a multi-signature DEX toggle.
///
/// # Errors
///
/// Returns a display-safe error for modified artifacts, addresses, or encoding.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_user_dex_abstraction_payload(
    prepared: PreparedUserDexAbstraction,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_user_dex_abstraction_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Prepares the authorized-user digest for multi-signature
/// `userSetAbstraction`.
///
/// # Errors
///
/// Returns a display-safe error for modified source artifacts, invalid account
/// addresses, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_multi_sig_user_set_abstraction_payload(
    prepared: PreparedUserSetAbstraction,
    multi_sig_user: String,
    outer_signer: String,
) -> Result<PreparedMultiSigPayload, String> {
    hyper_axis_core::client::prepare_multi_sig_user_set_abstraction_payload(
        prepared,
        &multi_sig_user,
        &outer_signer,
    )
    .map_err(|error| error.to_string())
}

/// Verifies and aggregates authorized-user signatures for the outer signer.
///
/// # Errors
///
/// Returns a display-safe error for empty/duplicate signers, invalid
/// signatures, modified artifacts, or serialization failure.
pub fn prepare_multi_sig_envelope(
    payload: PreparedMultiSigPayload,
    signatures: Vec<MultiSigSignatureInput>,
) -> Result<PreparedMultiSigEnvelope, String> {
    hyper_axis_core::client::prepare_multi_sig_envelope(payload, signatures)
        .map_err(|error| error.to_string())
}
