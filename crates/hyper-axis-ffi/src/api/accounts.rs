//! Account query, subscription, and balance-transfer bridge operations.

use flutter_rust_bridge::frb;
use hyper_axis_core::{
    client::{FinalizedAction, PrepareUsdClassTransferRequest, PreparedUsdClassTransfer},
    transport::info::InfoQuery,
    types::Network,
};

use super::json::{decode, encode};

/// Builds a canonical HTTP request for one perpetual clearinghouse account.
///
/// # Errors
///
/// Returns an error for an unsupported network, malformed address, or invalid DEX name.
#[frb(sync)]
pub fn build_perp_account_request_json(
    network: String,
    user: String,
    dex: String,
) -> Result<String, String> {
    let network = parse_network(&network)?;
    crate::build_typed_info_http_request_json(network, InfoQuery::ClearinghouseState { user, dex })
}

/// Builds a canonical HTTP request for one spot clearinghouse account.
///
/// # Errors
///
/// Returns an error for an unsupported network or malformed address.
#[frb(sync)]
pub fn build_spot_account_request_json(network: String, user: String) -> Result<String, String> {
    let network = parse_network(&network)?;
    crate::build_typed_info_http_request_json(network, InfoQuery::SpotClearinghouseState { user })
}

/// Validates and canonicalizes a perpetual account response.
///
/// # Errors
///
/// Returns an error when the response does not match the pinned protocol schema.
#[frb(sync)]
pub fn parse_perp_account_state_json(body: String) -> Result<String, String> {
    crate::parse_perp_clearinghouse_state_response_json(body)
}

/// Validates and canonicalizes a spot account response.
///
/// # Errors
///
/// Returns an error when the response does not match the pinned protocol schema.
#[frb(sync)]
pub fn parse_spot_account_state_json(body: String) -> Result<String, String> {
    crate::parse_spot_clearinghouse_state_response_json(body)
}

/// Returns the official WebSocket URL for the requested network.
///
/// # Errors
///
/// Returns an error for an unsupported network.
#[frb(sync)]
pub fn account_websocket_url(network: String) -> Result<String, String> {
    let network = parse_network(&network)?;
    Ok(crate::websocket_url(network))
}

/// Builds a perpetual-account WebSocket subscribe command.
///
/// # Errors
///
/// Returns an error for a malformed address, invalid DEX name, or serialization failure.
#[frb(sync)]
pub fn subscribe_perp_account_json(user: String, dex: String) -> Result<String, String> {
    let subscription = serde_json::json!({
        "type": "clearinghouseState",
        "user": user,
        "dex": dex,
    });
    crate::websocket_subscribe_json(subscription.to_string())
}

/// Builds a spot-account WebSocket subscribe command.
///
/// # Errors
///
/// Returns an error for a malformed address or serialization failure.
#[frb(sync)]
pub fn subscribe_spot_account_json(
    user: String,
    is_portfolio_margin: Option<bool>,
) -> Result<String, String> {
    let mut subscription = serde_json::json!({"type": "spotState", "user": user});
    if let Some(is_portfolio_margin) = is_portfolio_margin {
        subscription["isPortfolioMargin"] = serde_json::Value::Bool(is_portfolio_margin);
    }
    crate::websocket_subscribe_json(subscription.to_string())
}

/// Parses one account WebSocket message into stable JSON.
///
/// # Errors
///
/// Returns an error for malformed protocol messages.
#[frb(sync)]
pub fn parse_account_websocket_message_json(message: String) -> Result<String, String> {
    crate::parse_websocket_message_json(message)
}

/// Prepares a Spot/Perp USDC transfer for main-wallet signing.
///
/// # Errors
///
/// Returns an error for malformed JSON, invalid amounts, addresses, or nonces.
#[frb(sync)]
pub fn prepare_usd_class_transfer_json(request_json: String) -> Result<String, String> {
    let request: PrepareUsdClassTransferRequest =
        decode(&request_json, "USD class-transfer request")?;
    let prepared = crate::prepare_usd_class_transfer(request)?;
    encode(&prepared, "prepared USD class transfer")
}

/// Verifies and finalizes a prepared Spot/Perp USDC transfer.
///
/// # Errors
///
/// Returns an error for modified artifacts, invalid signatures, or signer mismatch.
#[frb(sync)]
pub fn finalize_usd_class_transfer_json(
    prepared_json: String,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<String, String> {
    let prepared: PreparedUsdClassTransfer = decode(&prepared_json, "prepared USD class transfer")?;
    let finalized = crate::finalize_usd_class_transfer(prepared, r, s, v, expected_signer)?;
    encode(&finalized, "finalized USD class transfer")
}

/// Builds a canonical exchange HTTP request from a finalized transfer.
///
/// # Errors
///
/// Returns an error for malformed or unsafe finalized artifacts.
#[frb(sync)]
pub fn build_account_exchange_request_json(finalized_json: String) -> Result<String, String> {
    let finalized: FinalizedAction = decode(&finalized_json, "finalized action")?;
    crate::build_exchange_http_request_json(finalized)
}

fn parse_network(value: &str) -> Result<Network, String> {
    match value {
        "mainnet" => Ok(Network::Mainnet),
        "testnet" => Ok(Network::Testnet),
        _ => Err("unsupported network".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        build_perp_account_request_json, build_spot_account_request_json,
        parse_account_websocket_message_json, parse_perp_account_state_json,
        parse_spot_account_state_json, subscribe_perp_account_json, subscribe_spot_account_json,
    };

    const USER: &str = "0x000000000000000000000000000000000000beef";

    #[test]
    fn builds_account_requests_and_validates_responses() {
        let perp =
            build_perp_account_request_json("mainnet".to_owned(), USER.to_owned(), String::new())
                .unwrap();
        let perp: Value = serde_json::from_str(&perp).unwrap();
        assert!(
            perp["body"]
                .as_str()
                .unwrap()
                .contains("clearinghouseState")
        );

        let spot = build_spot_account_request_json("mainnet".to_owned(), USER.to_owned()).unwrap();
        let spot: Value = serde_json::from_str(&spot).unwrap();
        assert!(
            spot["body"]
                .as_str()
                .unwrap()
                .contains("spotClearinghouseState")
        );

        let perpetual_response = r#"{"assetPositions":[],"crossMaintenanceMarginUsed":"0.0","crossMarginSummary":{"accountValue":"1","totalMarginUsed":"0","totalNtlPos":"0","totalRawUsd":"1"},"marginSummary":{"accountValue":"1","totalMarginUsed":"0","totalNtlPos":"0","totalRawUsd":"1"},"time":1,"withdrawable":"1"}"#;
        assert!(parse_perp_account_state_json(perpetual_response.to_owned()).is_ok());
        assert!(parse_spot_account_state_json(r#"{"balances":[]}"#.to_owned()).is_ok());
    }

    #[test]
    fn builds_and_parses_account_subscriptions() {
        let perp = subscribe_perp_account_json(USER.to_owned(), String::new()).unwrap();
        let perp: Value = serde_json::from_str(&perp).unwrap();
        assert_eq!(
            perp["subscription"],
            serde_json::json!({
                "type": "clearinghouseState",
                "user": USER,
                "dex": ""
            })
        );

        let spot = subscribe_spot_account_json(USER.to_owned(), Some(true)).unwrap();
        let spot: Value = serde_json::from_str(&spot).unwrap();
        assert_eq!(spot["subscription"]["type"], "spotState");
        assert_eq!(spot["subscription"]["isPortfolioMargin"], true);

        let message = r#"{"channel":"spotState","data":{"user":"0x000000000000000000000000000000000000beef","spotState":{"balances":[]}}}"#;
        let parsed = parse_account_websocket_message_json(message.to_owned()).unwrap();
        let parsed: Value = serde_json::from_str(&parsed).unwrap();
        assert_eq!(parsed["channel"], "spotState");
        assert_eq!(
            parsed["data"]["spotState"]["balances"],
            serde_json::json!([])
        );
    }
}
