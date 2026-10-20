/// Converts a prepared L1 order into the digest authorized users sign for a
/// multi-signature account.
///
/// # Errors
///
/// Returns an error when the original prepared order was modified, either
/// account address is invalid, or the inner envelope cannot be encoded.
pub fn prepare_multi_sig_order_payload(
    prepared: PreparedOrder,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::Order,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared cancellation-by-OID into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_cancel_payload(
    prepared: PreparedCancel,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::Cancel,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared cancellation-by-Cloid into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_cancel_by_cloid_payload(
    prepared: PreparedCancelByCloid,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::CancelByCloid,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared batch modification into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_batch_modify_payload(
    prepared: PreparedBatchModify,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::BatchModify,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared schedule-cancel action into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_schedule_cancel_payload(
    prepared: PreparedScheduleCancel,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::ScheduleCancel,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared native TWAP order into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_twap_order_payload(
    prepared: PreparedTwapOrder,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::TwapOrder,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared TWAP cancellation into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_twap_cancel_payload(
    prepared: PreparedTwapCancel,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::TwapCancel,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared leverage update into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_update_leverage_payload(
    prepared: PreparedUpdateLeverage,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::UpdateLeverage,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared isolated-margin delta into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_update_isolated_margin_payload(
    prepared: PreparedUpdateIsolatedMargin,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::UpdateIsolatedMargin,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared isolated-only margin top-up into an authorized-user
/// digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_top_up_isolated_only_margin_payload(
    prepared: PreparedTopUpIsolatedOnlyMargin,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::TopUpIsolatedOnlyMargin,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts prepared named-subaccount creation into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_create_sub_account_payload(
    prepared: PreparedCreateSubAccount,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::CreateSubAccount,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared USD subaccount transfer into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_sub_account_transfer_payload(
    prepared: PreparedSubAccountTransfer,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::SubAccountTransfer,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared spot subaccount transfer into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_sub_account_spot_transfer_payload(
    prepared: PreparedSubAccountSpotTransfer,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::SubAccountSpotTransfer,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared vault transfer into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_vault_transfer_payload(
    prepared: PreparedVaultTransfer,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::VaultTransfer,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts prepared referral-code registration into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_set_referrer_payload(
    prepared: PreparedSetReferrer,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::SetReferrer,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared EVM-user update into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_evm_user_modify_payload(
    prepared: PreparedEvmUserModify,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::EvmUserModify,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts prepared legacy agent abstraction enablement into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_agent_enable_dex_abstraction_payload(
    prepared: PreparedAgentEnableDexAbstraction,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::AgentEnableDexAbstraction,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared agent abstraction update into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_agent_set_abstraction_payload(
    prepared: PreparedAgentSetAbstraction,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::AgentSetAbstraction,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared no-op into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_noop_payload(
    prepared: PreparedNoop,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::Noop,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared signer-node action into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_c_signer_payload(
    prepared: PreparedCSigner,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::CSigner,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared validator lifecycle action into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_c_validator_payload(
    prepared: PreparedCValidator,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::CValidator,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared gossip priority bid into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_gossip_priority_bid_payload(
    prepared: PreparedGossipPriorityBid,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::GossipPriorityBid,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared Spot deployer action into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_spot_deploy_payload(
    prepared: PreparedSpotDeploy,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::SpotDeploy,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared perpetual deployer action into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_perp_deploy_payload(
    prepared: PreparedPerpDeploy,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_l1(
        prepared,
        MultiSigInnerAction::PerpDeploy,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared Core USDC transfer into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_usd_send_payload(
    prepared: PreparedUsdSend,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::UsdSend,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared Core spot transfer into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_spot_send_payload(
    prepared: PreparedSpotSend,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::SpotSend,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared bridge withdrawal into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_withdraw_payload(
    prepared: PreparedWithdraw,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::Withdraw,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared API-wallet authorization into an authorized-user
/// digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_approve_agent_payload(
    prepared: PreparedApproveAgent,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::ApproveAgent,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared builder-fee authorization into an authorized-user
/// digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_approve_builder_fee_payload(
    prepared: PreparedApproveBuilderFee,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::ApproveBuilderFee,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared USD class transfer into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_usd_class_transfer_payload(
    prepared: PreparedUsdClassTransfer,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::UsdClassTransfer,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared user-signed `sendAsset` action into the digest
/// authorized users sign for a multi-signature account.
///
/// # Errors
///
/// Returns an error when the original prepared action was modified, either
/// account address is invalid, or the multi-signature digest cannot be built.
pub fn prepare_multi_sig_send_asset_payload(
    prepared: PreparedSendAsset,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::SendAsset,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared staking deposit into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_c_deposit_payload(
    prepared: PreparedCDeposit,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::CDeposit,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared staking withdrawal into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_c_withdraw_payload(
    prepared: PreparedCWithdraw,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::CWithdraw,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared validator delegation into an authorized-user digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_token_delegate_payload(
    prepared: PreparedTokenDelegate,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::TokenDelegate,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared legacy DEX-abstraction toggle into an authorized-user
/// digest.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid addresses, or encoding.
pub fn prepare_multi_sig_user_dex_abstraction_payload(
    prepared: PreparedUserDexAbstraction,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::UserDexAbstraction,
        multi_sig_user,
        outer_signer,
    )
}

/// Converts a prepared `userSetAbstraction` action into the long-value digest
/// authorized users sign for a multi-signature account.
///
/// The later outer payload is canonicalized to `i`, `u`, or `p`.
///
/// # Errors
///
/// Returns an error when the original prepared action was modified, either
/// account address is invalid, or the multi-signature digest cannot be built.
pub fn prepare_multi_sig_user_set_abstraction_payload(
    prepared: PreparedUserSetAbstraction,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    prepare_multi_sig_from_user(
        prepared,
        MultiSigInnerAction::UserSetAbstraction,
        multi_sig_user,
        outer_signer,
    )
}

/// Verifies, canonicalizes, and aggregates authorized-user signatures into
/// the outer `SendMultiSig` signing artifact.
///
/// Signatures are verified against their declared signer, duplicate signers
/// are rejected, and the resulting array is sorted by recovered address. The
/// server remains authoritative for membership and threshold checks.
///
/// # Errors
///
/// Returns an error for a modified payload, empty/duplicate signer collection,
/// malformed signature, signer mismatch, or serialization failure.
pub fn prepare_multi_sig_envelope(
    payload: PreparedMultiSigPayload,
    signature_inputs: Vec<MultiSigSignatureInput>,
) -> Result<PreparedMultiSigEnvelope, MultiSigError> {
    verify_multi_sig_payload(&payload)?;
    if signature_inputs.is_empty() {
        return Err(ProtocolError::InvalidAction(
            "multi-signature envelope requires at least one inner signature",
        )
        .into());
    }

    let digest = decode_digest(&payload.signing_digest_hex)?;
    let mut verified = signature_inputs
        .into_iter()
        .map(|input| {
            let expected_signer = input.expected_signer.parse::<EvmAddress>()?;
            let signature = WireSignature::new(input.r, input.s, input.v)?;
            let recovered = signature.verify_signer(digest, expected_signer)?;
            Ok((recovered, signature))
        })
        .collect::<Result<Vec<_>, MultiSigError>>()?;
    verified.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    if verified.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(
            ProtocolError::InvalidAction("multi-signature inner signers must be unique").into(),
        );
    }
    let (recovered_inner_signers, signatures): (Vec<_>, Vec<_>) = verified.into_iter().unzip();

    let wire = MultiSigWireContext::new(&payload, &signatures);
    let action_json = serde_json::to_string(&wire.full_action())?;
    let message_pack = encode_action(&wire.without_tag())?;
    let action_hash = action_hash(
        &wire.without_tag(),
        payload.vault_address.as_ref(),
        payload.nonce,
        payload.expires_after,
    )?;
    let signing_digest =
        multi_sig_envelope_signing_digest(payload.network, action_hash, payload.nonce);
    Ok(PreparedMultiSigEnvelope {
        payload,
        signatures,
        recovered_inner_signers,
        action_json,
        message_pack_hex: hex::encode(message_pack),
        action_hash_hex: hex::encode(action_hash),
        signing_digest_hex: hex::encode(signing_digest),
    })
}
