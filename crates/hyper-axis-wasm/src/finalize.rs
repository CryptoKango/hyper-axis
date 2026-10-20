/// Extracts the signing digest without parsing prepared JSON in JavaScript.
///
/// Prepared action JSON should otherwise be treated as an opaque token and
/// passed unchanged into the matching `finalize_*_json` function.
///
/// # Errors
///
/// Returns a JavaScript error when the token is malformed or lacks the digest.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepared_signing_digest(prepared_json: &str) -> Result<String, JsError> {
    let prepared: serde_json::Value = serde_json::from_str(prepared_json)
        .map_err(|error| JsError::new(&format!("invalid prepared action: {error}")))?;
    prepared
        .get("signingDigestHex")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| JsError::new("prepared action is missing signingDigestHex"))
}

/// Verifies a platform signature and returns a finalized exchange request.
///
/// # Errors
///
/// Returns a JavaScript error when prepared JSON, signature components, or the
/// expected signer are invalid, or when integrity and recovery checks fail.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_order_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_order,
    )
}

/// Verifies and finalizes a cancellation-by-OID request.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_cancel_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_cancel,
    )
}

/// Verifies and finalizes a cancellation-by-Cloid request.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_cancel_by_cloid_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_cancel_by_cloid,
    )
}

/// Verifies and finalizes a batch-modify request.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_batch_modify_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_batch_modify,
    )
}

/// Verifies and finalizes a dead man's switch request.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_schedule_cancel_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_schedule_cancel,
    )
}

/// Verifies and finalizes a native TWAP order.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_twap_order_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_twap_order,
    )
}

/// Verifies and finalizes a native TWAP cancellation.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_twap_cancel_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_twap_cancel,
    )
}

/// Verifies and finalizes a leverage update.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_update_leverage_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_update_leverage,
    )
}

/// Verifies and finalizes an isolated-margin delta.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_update_isolated_margin_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_update_isolated_margin,
    )
}

/// Verifies and finalizes an isolated-only margin top-up.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_top_up_isolated_only_margin_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_top_up_isolated_only_margin,
    )
}

/// Verifies and finalizes named-subaccount creation.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_create_sub_account_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_create_sub_account,
    )
}

/// Verifies and finalizes a USD subaccount transfer.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_sub_account_transfer_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_sub_account_transfer,
    )
}

/// Verifies and finalizes a spot-token subaccount transfer.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_sub_account_spot_transfer_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_sub_account_spot_transfer,
    )
}

/// Verifies and finalizes a vault USD transfer.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_vault_transfer_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_vault_transfer,
    )
}

/// Verifies and finalizes referral-code registration.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_set_referrer_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_set_referrer,
    )
}

/// Verifies and finalizes an EVM large-block preference update.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_evm_user_modify_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_evm_user_modify,
    )
}

/// Verifies and finalizes legacy DEX-abstraction enablement for an agent.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_agent_enable_dex_abstraction_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_agent_enable_dex_abstraction,
    )
}

/// Verifies and finalizes an agent account-abstraction mode update.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_agent_set_abstraction_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_agent_set_abstraction,
    )
}

/// Verifies and finalizes an empty L1 action.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer, or
/// modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_noop_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_noop,
    )
}

/// Verifies and finalizes signer-node self-management.
///
/// # Errors
/// Returns a JavaScript error for invalid prepared JSON, signature, or artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_c_signer_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_c_signer,
    )
}

/// Verifies and finalizes a validator lifecycle action.
///
/// # Errors
/// Returns a JavaScript error for invalid prepared JSON, signature, or artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_c_validator_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_c_validator,
    )
}

/// Verifies and finalizes a gossip priority bid.
///
/// # Errors
/// Returns a JavaScript error for invalid prepared JSON, signature, or artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_gossip_priority_bid_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_gossip_priority_bid,
    )
}

/// Verifies and finalizes a Spot deployer action.
///
/// # Errors
/// Returns a JavaScript error for invalid prepared JSON, signature, or artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_spot_deploy_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_spot_deploy,
    )
}

/// Verifies and finalizes a perpetual deployer action.
///
/// # Errors
/// Returns a JavaScript error for invalid prepared JSON, signature, or artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_perp_deploy_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_perp_deploy,
    )
}

/// Verifies and finalizes a Core USDC transfer.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_usd_send_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_usd_send,
    )
}

/// Verifies and finalizes a Core spot asset transfer.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_spot_send_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_spot_send,
    )
}

/// Verifies and finalizes a bridge withdrawal.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_withdraw_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_withdraw,
    )
}

/// Verifies and finalizes an API wallet authorization.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_approve_agent_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_approve_agent,
    )
}

/// Verifies and finalizes a builder fee authorization.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_approve_builder_fee_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_approve_builder_fee,
    )
}

/// Verifies and finalizes a spot/perp USD class transfer.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_usd_class_transfer_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_usd_class_transfer,
    )
}

/// Verifies and finalizes a generalized asset transfer.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_send_asset_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_send_asset,
    )
}

/// Verifies and finalizes a native-token staking deposit.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_c_deposit_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_c_deposit,
    )
}

/// Verifies and finalizes a native-token staking withdrawal.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_c_withdraw_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_c_withdraw,
    )
}

/// Verifies and finalizes the legacy user DEX-abstraction toggle.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_user_dex_abstraction_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_user_dex_abstraction,
    )
}

/// Verifies and finalizes an account-abstraction mode update.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_user_set_abstraction_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_user_set_abstraction,
    )
}

/// Verifies and finalizes conversion to a threshold multi-signature user.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_convert_to_multi_sig_user_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_convert_to_multi_sig_user,
    )
}

/// Verifies the outer signer and builds the final multi-signature exchange
/// request.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, outer
/// signer mismatch, modified artifacts, encoding, or serialization failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_multi_sig_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_outer_signer: &str,
) -> Result<String, JsError> {
    let prepared: hyper_axis_core::client::PreparedMultiSigEnvelope =
        serde_json::from_str(prepared_json)
            .map_err(|error| JsError::new(&format!("invalid multi-signature envelope: {error}")))?;
    let signature =
        WireSignature::new(r, s, v).map_err(|error| JsError::new(&error.to_string()))?;
    let expected_outer_signer = expected_outer_signer
        .parse::<EvmAddress>()
        .map_err(|error| JsError::new(&error.to_string()))?;
    let finalized =
        hyper_axis_core::client::finalize_multi_sig(&prepared, signature, expected_outer_signer)
            .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&finalized).map_err(|error| JsError::new(&error.to_string()))
}

/// Verifies and finalizes a validator delegation or undelegation.
///
/// # Errors
///
/// Returns a JavaScript error for invalid prepared JSON, signature, signer,
/// context mismatch, or modified artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn finalize_token_delegate_json(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
) -> Result<String, JsError> {
    finalize_request_json(
        prepared_json,
        r,
        s,
        v,
        expected_signer,
        hyper_axis_core::client::finalize_token_delegate,
    )
}
