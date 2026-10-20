/// Produces the authorized-user digest for a prepared multi-signature L1
/// order.
///
/// The prepared order JSON must be passed unchanged from
/// `prepare_limit_order_json` or `prepare_orders_json`.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, modified artifacts,
/// invalid addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_order_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_order_payload,
    )
}

/// Produces the authorized-user digest for a prepared OID cancellation.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_cancel_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_cancel_payload,
    )
}

/// Produces the authorized-user digest for a prepared Cloid cancellation.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_cancel_by_cloid_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_cancel_by_cloid_payload,
    )
}

/// Produces the authorized-user digest for a prepared batch modification.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_batch_modify_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_batch_modify_payload,
    )
}

/// Produces the authorized-user digest for a prepared schedule-cancel action.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_schedule_cancel_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_schedule_cancel_payload,
    )
}

/// Produces the authorized-user digest for a prepared native TWAP order.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_twap_order_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_twap_order_payload,
    )
}

/// Produces the authorized-user digest for a prepared TWAP cancellation.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_twap_cancel_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_twap_cancel_payload,
    )
}

/// Produces the authorized-user digest for a prepared leverage update.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_update_leverage_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_update_leverage_payload,
    )
}

/// Produces the authorized-user digest for a prepared isolated-margin update.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_update_isolated_margin_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_update_isolated_margin_payload,
    )
}

/// Produces the authorized-user digest for a prepared isolated-only margin
/// top-up.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts,
/// addresses, or encoding failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_top_up_isolated_only_margin_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_top_up_isolated_only_margin_payload,
    )
}

/// Produces the authorized-user digest for named-subaccount creation.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_create_sub_account_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_create_sub_account_payload,
    )
}

/// Produces the authorized-user digest for a USD subaccount transfer.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_sub_account_transfer_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_sub_account_transfer_payload,
    )
}

/// Produces the authorized-user digest for a spot subaccount transfer.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_sub_account_spot_transfer_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_sub_account_spot_transfer_payload,
    )
}

/// Produces the authorized-user digest for a vault transfer.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_vault_transfer_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_vault_transfer_payload,
    )
}

/// Produces the authorized-user digest for referral-code registration.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_set_referrer_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_set_referrer_payload,
    )
}

/// Produces the authorized-user digest for an EVM-user update.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_evm_user_modify_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_evm_user_modify_payload,
    )
}

/// Produces the authorized-user digest for legacy agent abstraction.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_agent_enable_dex_abstraction_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_agent_enable_dex_abstraction_payload,
    )
}

/// Produces the authorized-user digest for an agent abstraction update.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_agent_set_abstraction_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_agent_set_abstraction_payload,
    )
}

/// Produces the authorized-user digest for an L1 no-op.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_noop_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_noop_payload,
    )
}

/// Produces the authorized-user digest for signer-node self-management.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_c_signer_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_c_signer_payload,
    )
}

/// Produces the authorized-user digest for a validator lifecycle action.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_c_validator_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_c_validator_payload,
    )
}

/// Produces the authorized-user digest for a gossip priority bid.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_gossip_priority_bid_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_gossip_priority_bid_payload,
    )
}

/// Produces the authorized-user digest for a Spot deployer action.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_spot_deploy_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_spot_deploy_payload,
    )
}

/// Produces the authorized-user digest for a perpetual deployer action.
///
/// # Errors
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_perp_deploy_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_perp_deploy_payload,
    )
}

/// Produces the authorized-user digest for prepared multi-signature `usdSend`.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_usd_send_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_usd_send_payload,
    )
}

/// Produces the authorized-user digest for prepared multi-signature `spotSend`.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_spot_send_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_spot_send_payload,
    )
}

/// Produces the authorized-user digest for a prepared bridge withdrawal.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_withdraw_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_withdraw_payload,
    )
}

/// Produces the authorized-user digest for prepared API-wallet approval.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_approve_agent_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_approve_agent_payload,
    )
}

/// Produces the authorized-user digest for prepared builder-fee approval.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_approve_builder_fee_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_approve_builder_fee_payload,
    )
}

/// Produces the authorized-user digest for a prepared USD class transfer.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_usd_class_transfer_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_usd_class_transfer_payload,
    )
}

/// Produces the authorized-user digest for a prepared multi-signature
/// `sendAsset` action.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, modified artifacts,
/// invalid addresses, or serialization failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_send_asset_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_send_asset_payload,
    )
}

/// Produces the authorized-user digest for a prepared staking deposit.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_c_deposit_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_c_deposit_payload,
    )
}

/// Produces the authorized-user digest for a prepared staking withdrawal.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_c_withdraw_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_c_withdraw_payload,
    )
}

/// Produces the authorized-user digest for prepared validator delegation.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_token_delegate_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_token_delegate_payload,
    )
}

/// Produces the authorized-user digest for a prepared DEX-abstraction toggle.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, modified artifacts, or addresses.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_user_dex_abstraction_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_user_dex_abstraction_payload,
    )
}

/// Produces the authorized-user digest for a prepared multi-signature
/// `userSetAbstraction` action.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, modified artifacts,
/// invalid addresses, or serialization failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_user_set_abstraction_payload_json(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<String, JsError> {
    prepare_multi_sig_payload_json(
        prepared_json,
        multi_sig_user,
        outer_signer,
        hyper_axis_core::client::prepare_multi_sig_user_set_abstraction_payload,
    )
}

/// Verifies authorized-user signatures and produces the outer signer digest.
///
/// `signaturesJson` must be an array of `{expectedSigner,r,s,v}` objects. The
/// returned envelope is opaque and must be passed unchanged to
/// `finalize_multi_sig_json`.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, empty/duplicate signers,
/// signature mismatch, modified artifacts, or serialization failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_multi_sig_envelope_json(
    payload_json: &str,
    signatures_json: &str,
) -> Result<String, JsError> {
    let payload: hyper_axis_core::client::PreparedMultiSigPayload =
        serde_json::from_str(payload_json)
            .map_err(|error| JsError::new(&format!("invalid multi-signature payload: {error}")))?;
    let signatures: Vec<hyper_axis_core::client::MultiSigSignatureInput> =
        serde_json::from_str(signatures_json)
            .map_err(|error| JsError::new(&format!("invalid inner signatures: {error}")))?;
    let prepared = hyper_axis_core::client::prepare_multi_sig_envelope(payload, signatures)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&prepared).map_err(|error| JsError::new(&error.to_string()))
}
