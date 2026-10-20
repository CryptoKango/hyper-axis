/// Revalidates prepared artifacts, verifies the platform signature, and builds
/// the exact JSON body for `POST /exchange`.
///
/// All hashes are recomputed from the typed action. This prevents a modified
/// FFI or WebAssembly payload from being finalized under a stale signature.
///
/// # Errors
///
/// Returns an error when an artifact was modified, encoding fails, signature
/// recovery fails, the recovered signer differs from `expected_signer`, or the
/// final request cannot be serialized.
pub fn finalize_order(
    prepared: PreparedOrder,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedOrder, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "order")
}

/// Verifies and finalizes a cancellation-by-OID request.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_cancel(
    prepared: PreparedCancel,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "cancel")
}

/// Verifies and finalizes a cancellation-by-Cloid request.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_cancel_by_cloid(
    prepared: PreparedCancelByCloid,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "cancel_by_cloid")
}

/// Verifies and finalizes a batch-modify request.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_batch_modify(
    prepared: PreparedBatchModify,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "batch_modify")
}

/// Verifies and finalizes a schedule-cancel request.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_schedule_cancel(
    prepared: PreparedScheduleCancel,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "schedule_cancel")
}

/// Verifies and finalizes a native TWAP order.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_twap_order(
    prepared: PreparedTwapOrder,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "twap_order")
}

/// Verifies and finalizes a native TWAP cancellation.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_twap_cancel(
    prepared: PreparedTwapCancel,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "twap_cancel")
}

/// Verifies and finalizes a leverage update.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_update_leverage(
    prepared: PreparedUpdateLeverage,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "update_leverage")
}

/// Verifies and finalizes an isolated-margin delta.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_update_isolated_margin(
    prepared: PreparedUpdateIsolatedMargin,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(
        prepared,
        signature,
        expected_signer,
        "update_isolated_margin",
    )
}

/// Verifies and finalizes an isolated-only margin top-up.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, encoding failure, or final serialization failure.
pub fn finalize_top_up_isolated_only_margin(
    prepared: PreparedTopUpIsolatedOnlyMargin,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(
        prepared,
        signature,
        expected_signer,
        "top_up_isolated_only_margin",
    )
}

/// Verifies and finalizes named-subaccount creation.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_create_sub_account(
    prepared: PreparedCreateSubAccount,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "create_sub_account")
}

/// Verifies and finalizes a USD subaccount transfer.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_sub_account_transfer(
    prepared: PreparedSubAccountTransfer,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "sub_account_transfer")
}

/// Verifies and finalizes a spot-token subaccount transfer.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_sub_account_spot_transfer(
    prepared: PreparedSubAccountSpotTransfer,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(
        prepared,
        signature,
        expected_signer,
        "sub_account_spot_transfer",
    )
}

/// Verifies and finalizes a vault USD transfer.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_vault_transfer(
    prepared: PreparedVaultTransfer,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "vault_transfer")
}

/// Verifies and finalizes referral-code registration.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_set_referrer(
    prepared: PreparedSetReferrer,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "set_referrer")
}

/// Verifies and finalizes an EVM large-block preference update.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_evm_user_modify(
    prepared: PreparedEvmUserModify,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "evm_user_modify")
}

/// Verifies and finalizes legacy agent DEX-abstraction enablement.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_agent_enable_dex_abstraction(
    prepared: PreparedAgentEnableDexAbstraction,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(
        prepared,
        signature,
        expected_signer,
        "agent_enable_dex_abstraction",
    )
}

/// Verifies and finalizes an agent account-abstraction mode update.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_agent_set_abstraction(
    prepared: PreparedAgentSetAbstraction,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(
        prepared,
        signature,
        expected_signer,
        "agent_set_abstraction",
    )
}

/// Verifies and finalizes an empty L1 action.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_noop(
    prepared: PreparedNoop,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "noop")
}

/// Verifies and finalizes signer-node self-management.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_c_signer(
    prepared: PreparedCSigner,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "c_signer")
}

/// Verifies and finalizes a validator lifecycle action.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_c_validator(
    prepared: PreparedCValidator,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "c_validator")
}

/// Verifies and finalizes a gossip priority bid.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_gossip_priority_bid(
    prepared: PreparedGossipPriorityBid,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "gossip_priority_bid")
}

/// Verifies and finalizes a Spot deployer action.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_spot_deploy(
    prepared: PreparedSpotDeploy,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "spot_deploy")
}

/// Verifies and finalizes a perpetual deployer action.
///
/// # Errors
///
/// Returns an error on artifact modification, signature failure, signer
/// mismatch, or final serialization failure.
pub fn finalize_perp_deploy(
    prepared: PreparedPerpDeploy,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_action(prepared, signature, expected_signer, "perp_deploy")
}

/// Verifies and finalizes a user-signed Core USDC transfer.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_usd_send(
    prepared: PreparedUsdSend,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "usd_send")
}

/// Verifies and finalizes a user-signed Core spot transfer.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_spot_send(
    prepared: PreparedSpotSend,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "spot_send")
}

/// Verifies and finalizes a user-signed bridge withdrawal.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_withdraw(
    prepared: PreparedWithdraw,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "withdraw")
}

/// Verifies and finalizes an API wallet authorization.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_approve_agent(
    prepared: PreparedApproveAgent,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "approve_agent")
}

/// Verifies and finalizes a builder fee authorization.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_approve_builder_fee(
    prepared: PreparedApproveBuilderFee,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "approve_builder_fee")
}

/// Verifies and finalizes a spot/perp USD class transfer.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_usd_class_transfer(
    prepared: PreparedUsdClassTransfer,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "usd_class_transfer")
}

/// Verifies and finalizes a generalized asset transfer.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_send_asset(
    prepared: PreparedSendAsset,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "send_asset")
}

/// Verifies and finalizes a native-token staking deposit.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_c_deposit(
    prepared: PreparedCDeposit,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "c_deposit")
}

/// Verifies and finalizes a native-token staking withdrawal.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_c_withdraw(
    prepared: PreparedCWithdraw,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "c_withdraw")
}

/// Verifies and finalizes a validator delegation or undelegation.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_token_delegate(
    prepared: PreparedTokenDelegate,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "token_delegate")
}

/// Verifies and finalizes the legacy user DEX-abstraction toggle.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_user_dex_abstraction(
    prepared: PreparedUserDexAbstraction,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "user_dex_abstraction")
}

/// Verifies and finalizes an account-abstraction mode update.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_user_set_abstraction(
    prepared: PreparedUserSetAbstraction,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(prepared, signature, expected_signer, "user_set_abstraction")
}

/// Verifies and finalizes conversion to a threshold multi-signature user.
///
/// # Errors
///
/// Returns an error on artifact/context modification, signature failure,
/// signer mismatch, or final serialization failure.
pub fn finalize_convert_to_multi_sig_user(
    prepared: PreparedConvertToMultiSigUser,
    signature: WireSignature,
    expected_signer: EvmAddress,
) -> Result<FinalizedAction, FinalizeError> {
    finalize_user_action(
        prepared,
        signature,
        expected_signer,
        "convert_to_multi_sig_user",
    )
}

/// Verifies the outer signer and builds the final `/exchange` multi-signature
/// request body.
///
/// Every inner and outer artifact is recomputed before the request is emitted.
///
/// # Errors
///
/// Returns an error for modified artifacts, signer-context mismatch, malformed
/// signatures, encoding failure, or final serialization failure.
pub fn finalize_multi_sig(
    prepared: &PreparedMultiSigEnvelope,
    signature: WireSignature,
    expected_outer_signer: EvmAddress,
) -> Result<FinalizedAction, MultiSigError> {
    verify_multi_sig_envelope(prepared)?;
    if expected_outer_signer != prepared.payload.outer_signer {
        return Err(MultiSigError::ArtifactMismatch("outerSigner"));
    }
    let digest = decode_digest(&prepared.signing_digest_hex)?;
    let recovered_signer = signature.verify_signer(digest, expected_outer_signer)?;
    let wire = MultiSigWireContext::new(&prepared.payload, &prepared.signatures);
    let request = MultiSigExchangeRequest {
        action: wire.full_action(),
        nonce: prepared.payload.nonce,
        signature,
        vault_address: prepared.payload.vault_address,
        expires_after: prepared.payload.expires_after,
    };
    Ok(FinalizedAction {
        endpoint: prepared.payload.network.exchange_url().to_owned(),
        request_json: serde_json::to_string(&request)?,
        recovered_signer,
    })
}
