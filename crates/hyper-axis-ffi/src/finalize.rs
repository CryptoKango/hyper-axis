/// Verifies a platform signature and builds the final exchange request.
///
/// # Errors
///
/// Returns an error when signature components or the expected signer address
/// are invalid, prepared artifacts were modified, signer recovery fails, or
/// request serialization fails.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_order(
    prepared: PreparedOrder,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedOrder, String> {
    let signature = WireSignature::new(r, s, v).map_err(|error| error.to_string())?;
    let expected_signer = expected_signer
        .parse::<EvmAddress>()
        .map_err(|error| error.to_string())?;
    hyper_axis_core::client::finalize_order(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies a signature and finalizes a cancellation-by-OID request.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_cancel(
    prepared: PreparedCancel,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_cancel(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies a signature and finalizes a cancellation-by-Cloid request.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_cancel_by_cloid(
    prepared: PreparedCancelByCloid,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_cancel_by_cloid(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies a signature and finalizes a batch-modify request.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_batch_modify(
    prepared: PreparedBatchModify,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_batch_modify(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a dead man's switch request.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_schedule_cancel(
    prepared: PreparedScheduleCancel,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_schedule_cancel(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a native TWAP order.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_twap_order(
    prepared: PreparedTwapOrder,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_twap_order(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a native TWAP cancellation.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_twap_cancel(
    prepared: PreparedTwapCancel,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_twap_cancel(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a leverage update.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_update_leverage(
    prepared: PreparedUpdateLeverage,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_update_leverage(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes an isolated-margin delta.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_update_isolated_margin(
    prepared: PreparedUpdateIsolatedMargin,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_update_isolated_margin(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes an isolated-only margin top-up.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_top_up_isolated_only_margin(
    prepared: PreparedTopUpIsolatedOnlyMargin,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_top_up_isolated_only_margin(
        prepared,
        signature,
        expected_signer,
    )
    .map_err(|error| error.to_string())
}

/// Verifies and finalizes named-subaccount creation.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_create_sub_account(
    prepared: PreparedCreateSubAccount,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_create_sub_account(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a USD subaccount transfer.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_sub_account_transfer(
    prepared: PreparedSubAccountTransfer,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_sub_account_transfer(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a spot-token subaccount transfer.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_sub_account_spot_transfer(
    prepared: PreparedSubAccountSpotTransfer,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_sub_account_spot_transfer(
        prepared,
        signature,
        expected_signer,
    )
    .map_err(|error| error.to_string())
}

/// Verifies and finalizes a vault USD transfer.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_vault_transfer(
    prepared: PreparedVaultTransfer,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_vault_transfer(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes referral-code registration.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_set_referrer(
    prepared: PreparedSetReferrer,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_set_referrer(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes an EVM large-block preference update.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_evm_user_modify(
    prepared: PreparedEvmUserModify,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_evm_user_modify(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes legacy DEX-abstraction enablement for an agent.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_agent_enable_dex_abstraction(
    prepared: PreparedAgentEnableDexAbstraction,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_agent_enable_dex_abstraction(
        prepared,
        signature,
        expected_signer,
    )
    .map_err(|error| error.to_string())
}

/// Verifies and finalizes an agent account-abstraction mode update.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_agent_set_abstraction(
    prepared: PreparedAgentSetAbstraction,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_agent_set_abstraction(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes an empty L1 action.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_noop(
    prepared: PreparedNoop,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_noop(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes signer-node self-management.
///
/// # Errors
/// Returns an error for invalid signatures, modified artifacts, or serialization.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_c_signer(
    prepared: PreparedCSigner,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_c_signer(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a validator lifecycle action.
///
/// # Errors
/// Returns an error for invalid signatures, modified artifacts, or serialization.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_c_validator(
    prepared: PreparedCValidator,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_c_validator(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a gossip priority bid.
///
/// # Errors
/// Returns an error for invalid signatures, modified artifacts, or serialization.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_gossip_priority_bid(
    prepared: PreparedGossipPriorityBid,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_gossip_priority_bid(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a Spot deployer action.
///
/// # Errors
/// Returns an error for invalid signatures, modified artifacts, or serialization.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_spot_deploy(
    prepared: PreparedSpotDeploy,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_spot_deploy(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a perpetual deployer action.
///
/// # Errors
/// Returns an error for invalid signatures, modified artifacts, or serialization.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_perp_deploy(
    prepared: PreparedPerpDeploy,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_perp_deploy(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a Core USDC transfer.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_usd_send(
    prepared: PreparedUsdSend,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_usd_send(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a Core spot asset transfer.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_spot_send(
    prepared: PreparedSpotSend,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_spot_send(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a bridge withdrawal.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_withdraw(
    prepared: PreparedWithdraw,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_withdraw(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes an API wallet authorization.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_approve_agent(
    prepared: PreparedApproveAgent,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_approve_agent(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a builder fee authorization.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_approve_builder_fee(
    prepared: PreparedApproveBuilderFee,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_approve_builder_fee(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a spot/perp USD class transfer.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_usd_class_transfer(
    prepared: PreparedUsdClassTransfer,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_usd_class_transfer(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a generalized asset transfer.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_send_asset(
    prepared: PreparedSendAsset,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_send_asset(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a native-token staking deposit.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_c_deposit(
    prepared: PreparedCDeposit,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_c_deposit(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a native-token staking withdrawal.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_c_withdraw(
    prepared: PreparedCWithdraw,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_c_withdraw(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes a validator delegation or undelegation.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_token_delegate(
    prepared: PreparedTokenDelegate,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_token_delegate(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes the legacy user DEX-abstraction toggle.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_user_dex_abstraction(
    prepared: PreparedUserDexAbstraction,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_user_dex_abstraction(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes an account-abstraction mode update.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_user_set_abstraction(
    prepared: PreparedUserSetAbstraction,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_user_set_abstraction(prepared, signature, expected_signer)
        .map_err(|error| error.to_string())
}

/// Verifies and finalizes conversion to a threshold multi-signature user.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_convert_to_multi_sig_user(
    prepared: PreparedConvertToMultiSigUser,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_signer) = parse_signature(r, s, v, &expected_signer)?;
    hyper_axis_core::client::finalize_convert_to_multi_sig_user(
        prepared,
        signature,
        expected_signer,
    )
    .map_err(|error| error.to_string())
}

/// Verifies the outer multi-signature envelope and builds the final exchange
/// request.
///
/// # Errors
///
/// Returns an error for invalid signatures, signer/context mismatch, modified
/// artifacts, encoding, or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn finalize_multi_sig(
    prepared: PreparedMultiSigEnvelope,
    r: String,
    s: String,
    v: u8,
    expected_outer_signer: String,
) -> Result<FinalizedAction, String> {
    let (signature, expected_outer_signer) = parse_signature(r, s, v, &expected_outer_signer)?;
    hyper_axis_core::client::finalize_multi_sig(&prepared, signature, expected_outer_signer)
        .map_err(|error| error.to_string())
}
