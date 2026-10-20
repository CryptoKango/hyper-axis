/// Prepares a limit order and returns the action and signing artifacts as JSON.
///
/// Nonce and expiry use strings so JavaScript never converts 64-bit integers
/// through its lossy `Number` representation.
///
/// # Errors
///
/// Returns a JavaScript error for unsupported enum values, invalid integers,
/// precision violations, or serialization failures.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[allow(clippy::too_many_arguments)]
pub fn prepare_limit_order_json(
    network: &str,
    asset: u32,
    market_kind: &str,
    sz_decimals: u8,
    side: &str,
    limit_price: &str,
    order_size: &str,
    reduce_only: bool,
    time_in_force: &str,
    nonce: &str,
    vault_address: Option<String>,
    expires_after: Option<String>,
) -> Result<String, JsError> {
    let network = parse_network(network)?;
    let market_kind = match market_kind {
        "perp" => MarketKind::Perpetual,
        "spot" => MarketKind::Spot,
        _ => return Err(JsError::new("marketKind must be perp or spot")),
    };
    let is_buy = match side {
        "buy" => true,
        "sell" => false,
        _ => return Err(JsError::new("side must be buy or sell")),
    };
    let time_in_force = match time_in_force {
        "Alo" => TimeInForce::Alo,
        "Ioc" => TimeInForce::Ioc,
        "Gtc" => TimeInForce::Gtc,
        _ => return Err(JsError::new("timeInForce must be Alo, Ioc, or Gtc")),
    };
    let nonce = parse_u64("nonce", nonce)?;
    let expires_after = expires_after
        .map(|value| parse_u64("expiresAfter", &value))
        .transpose()?;
    let request = PrepareLimitOrderRequest {
        network,
        asset: AssetId::new(asset),
        market_kind,
        sz_decimals,
        is_buy,
        limit_price: limit_price.to_owned(),
        size: order_size.to_owned(),
        reduce_only,
        time_in_force,
        client_order_id: None,
        grouping: Grouping::None,
        nonce,
        vault_address,
        expires_after,
    };
    let prepared = hyper_axis_core::client::prepare_limit_order(request)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&prepared).map_err(|error| JsError::new(&error.to_string()))
}

/// Prepares an atomic limit-order batch from strict camelCase JSON.
///
/// All `u64` values, including `nonce` and `expiresAfter`, must be decimal
/// strings. OIDs are also strings; asset IDs and builder fees remain numbers.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or any failed protocol rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_batch_orders_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_batch_orders)
}

/// Prepares a mixed atomic batch of limit and trigger orders.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or any failed protocol rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_orders_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_orders)
}

/// Prepares a cancellation batch by exchange OID from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or any failed protocol rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_cancel_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_cancel)
}

/// Prepares a cancellation batch by 128-bit Cloid from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or any failed protocol rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_cancel_by_cloid_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_cancel_by_cloid,
    )
}

/// Prepares a batch of complete order replacements from strict camelCase JSON.
///
/// An `oid` selector is either a decimal string or a 128-bit Cloid string.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or any failed protocol rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_batch_modify_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_batch_modify)
}

/// Prepares a dead man's switch set or clear action.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, an unsafe deadline, or any
/// failed signing-context rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_schedule_cancel_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_schedule_cancel,
    )
}

/// Prepares one native TWAP order.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or any failed protocol rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_twap_order_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_twap_order)
}

/// Prepares one native TWAP cancellation.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or any failed protocol rule.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_twap_cancel_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_twap_cancel)
}

/// Prepares a cross/isolated leverage update from strict camelCase JSON.
///
/// `maxLeverage` must come from current perpetual market metadata. Nonce and
/// expiry values remain decimal strings at the JavaScript boundary.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON, leverage outside the market
/// range, or invalid signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_update_leverage_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_update_leverage,
    )
}

/// Prepares an exact isolated-margin USD delta from strict camelCase JSON.
///
/// `usdDelta` is a signed decimal string with at most six fractional digits.
/// It is never converted through JavaScript `Number`.
///
/// # Errors
///
/// Returns a JavaScript error for malformed, over-precise, zero, or out-of-
/// range values and invalid signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_update_isolated_margin_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_update_isolated_margin,
    )
}

/// Prepares an isolated-only margin top-up from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid target leverage or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_top_up_isolated_only_margin_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_top_up_isolated_only_margin,
    )
}

/// Prepares named-subaccount creation from strict camelCase JSON.
///
/// `nonce` and optional `expiresAfter` must be unsigned decimal strings.
///
/// # Errors
///
/// Returns a JavaScript error for invalid names, integer strings, or signing
/// context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_create_sub_account_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_create_sub_account,
    )
}

/// Prepares a USD master/subaccount transfer from strict camelCase JSON.
///
/// `amount` is an exact decimal string and is converted to six-decimal
/// protocol units without rounding.
///
/// # Errors
///
/// Returns a JavaScript error for invalid addresses, amount precision, integer
/// strings, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_sub_account_transfer_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_sub_account_transfer,
    )
}

/// Prepares a spot-token master/subaccount transfer from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid addresses, token, amount, integer
/// strings, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_sub_account_spot_transfer_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_sub_account_spot_transfer,
    )
}

/// Prepares a vault USD deposit or withdrawal from strict camelCase JSON.
///
/// `amount` is an exact decimal string and is converted to six-decimal
/// protocol units without rounding.
///
/// # Errors
///
/// Returns a JavaScript error for invalid addresses, amount precision, integer
/// strings, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_vault_transfer_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_vault_transfer,
    )
}

/// Prepares referral-code registration from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, code, nonce, or signing
/// context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_set_referrer_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_set_referrer)
}

/// Prepares an EVM large-block preference update from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, nonce, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_evm_user_modify_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_evm_user_modify,
    )
}

/// Prepares legacy DEX-abstraction enablement for an agent.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, address, nonce, or signing
/// context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_agent_enable_dex_abstraction_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_agent_enable_dex_abstraction,
    )
}

/// Prepares an agent account-abstraction mode update.
///
/// Accepted `abstraction` wire values are `i`, `u`, and `p`.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, mode, address, nonce, or
/// signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_agent_set_abstraction_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_agent_set_abstraction,
    )
}

/// Prepares an empty L1 action from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, address, nonce, or signing
/// context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_noop_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_noop)
}

/// Prepares signer-node jail or unjail self-management from JSON.
///
/// # Errors
/// Returns a JavaScript error for invalid JSON or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_c_signer_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_c_signer)
}

/// Prepares consensus validator registration from JSON.
///
/// # Errors
/// Returns a JavaScript error for invalid JSON, profile data, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_c_validator_register_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_c_validator_register,
    )
}

/// Prepares a consensus validator profile update from JSON.
///
/// # Errors
/// Returns a JavaScript error for invalid JSON, profile data, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_c_validator_change_profile_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_c_validator_change_profile,
    )
}

/// Prepares consensus validator unregistration from JSON.
///
/// # Errors
/// Returns a JavaScript error for invalid JSON or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_c_validator_unregister_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_c_validator_unregister,
    )
}

/// Prepares a gossip priority gas bid from JSON.
///
/// # Errors
/// Returns a JavaScript error for invalid JSON, bid data, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_gossip_priority_bid_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_gossip_priority_bid,
    )
}

/// Prepares any supported Spot deployer lifecycle action from JSON.
///
/// # Errors
/// Returns a JavaScript error for invalid JSON, deployer data, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_spot_deploy_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_spot_deploy)
}

/// Prepares any supported perpetual deployer lifecycle action from JSON.
///
/// # Errors
/// Returns a JavaScript error for invalid JSON, deployer data, or signing context.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_perp_deploy_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_perp_deploy)
}

/// Prepares a Core USDC transfer from strict camelCase JSON.
///
/// Nonce is a decimal string and becomes both the signed `time` and outer
/// exchange nonce.
///
/// # Errors
///
/// Returns a JavaScript error for invalid destination, amount, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_usd_send_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_usd_send)
}

/// Prepares a Core spot transfer from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid destination, token, amount, nonce,
/// or serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_spot_send_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_spot_send)
}

/// Prepares a bridge withdrawal from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid destination, amount, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_withdraw_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_withdraw)
}

/// Prepares an API wallet authorization from strict camelCase JSON.
///
/// `HyperAxis` receives only the pre-generated API wallet address and never its
/// private key. Nonce must be a decimal string.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, address, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_approve_agent_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_approve_agent)
}

/// Prepares a builder fee authorization from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, builder address, exact percent
/// string, nonce, or serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_approve_builder_fee_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_approve_builder_fee,
    )
}

/// Prepares a spot/perp USD class transfer from strict camelCase JSON.
///
/// `amount` must not contain a manually appended subaccount suffix; pass the
/// optional `subAccount` field and let the Core construct the signed string.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, amount, subaccount, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_usd_class_transfer_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_usd_class_transfer,
    )
}

/// Prepares a generalized asset transfer from strict camelCase JSON.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, addresses, DEX names, token,
/// amount, nonce, or serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_send_asset_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_send_asset)
}

/// Prepares a native-token staking deposit from strict camelCase JSON.
///
/// `wei` and `nonce` must be unsigned decimal strings. The returned prepared
/// JSON is an opaque integrity token; use `prepared_signing_digest` to read the
/// digest without parsing and rewriting that token in JavaScript.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, zero amounts, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_c_deposit_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_c_deposit)
}

/// Prepares a native-token staking withdrawal from strict camelCase JSON.
///
/// `wei` and `nonce` must be unsigned decimal strings. The returned prepared
/// JSON must be passed unchanged to `finalize_c_withdraw_json`.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, zero amounts, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_c_withdraw_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(request_json, hyper_axis_core::client::prepare_c_withdraw)
}

/// Prepares a validator delegation or undelegation from strict camelCase JSON.
///
/// `wei` and `nonce` must be unsigned decimal strings. The returned prepared
/// JSON must be passed unchanged to `finalize_token_delegate_json`.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, validator, amount, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_token_delegate_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_token_delegate,
    )
}

/// Prepares the legacy user DEX-abstraction toggle from strict camelCase JSON.
///
/// `nonce` must be an unsigned decimal string.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, user address, nonce, or
/// serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_user_dex_abstraction_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_user_dex_abstraction,
    )
}

/// Prepares an account-abstraction mode update from strict camelCase JSON.
///
/// Accepted `abstraction` values are `disabled`, `unifiedAccount`, and
/// `portfolioMargin`.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, user address, abstraction,
/// nonce, or serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_user_set_abstraction_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_user_set_abstraction,
    )
}

/// Prepares conversion to a threshold multi-signature user from strict JSON.
///
/// Rust parses and sorts `authorizedUsers`, rejects duplicates, and validates
/// `threshold` before producing the signing digest.
///
/// # Errors
///
/// Returns a JavaScript error for invalid JSON, addresses, threshold, nonce,
/// or serialization.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_convert_to_multi_sig_user_json(request_json: &str) -> Result<String, JsError> {
    prepare_request_json(
        request_json,
        hyper_axis_core::client::prepare_convert_to_multi_sig_user,
    )
}
