//! Runtime-neutral Info and WebSocket bridge operations.

use flutter_rust_bridge::frb;
use hyper_axis_core::{
    transport::{RequestKind, info::InfoQuery},
    types::Network,
};

use super::json::decode;

/// Builds a canonical HTTP request from a typed Info query.
#[frb(sync)]
pub fn build_typed_info_request_json(
    network: String,
    query_json: String,
) -> Result<String, String> {
    let network = parse_network(&network)?;
    let query: InfoQuery = decode(&query_json, "Info query")?;
    crate::build_typed_info_http_request_json(network, query)
}

/// Validates and canonicalizes a typed Info query.
#[frb(sync)]
pub fn prepare_info_query_json(query_json: String) -> Result<String, String> {
    let query: InfoQuery = decode(&query_json, "Info query")?;
    crate::prepare_info_query_json(query)
}

/// Parses and classifies an HTTP response with shared protocol rules.
#[frb(sync)]
pub fn parse_http_response_json(
    request_kind: String,
    status_code: u16,
    body: String,
) -> Result<String, String> {
    crate::parse_http_response_json(parse_request_kind(&request_kind)?, status_code, body)
}

/// Validates and canonicalizes a `perpDexs` response.
#[frb(sync)]
pub fn parse_perp_dexs_response_json(body: String) -> Result<String, String> {
    crate::parse_perp_dexs_response_json(body)
}

/// Validates and canonicalizes a `metaAndAssetCtxs` response.
#[frb(sync)]
pub fn parse_perp_meta_and_asset_contexts_response_json(body: String) -> Result<String, String> {
    crate::parse_perp_meta_and_asset_ctxs_response_json(body)
}

/// Returns the official WebSocket URL for a network.
#[frb(sync)]
pub fn websocket_url(network: String) -> Result<String, String> {
    Ok(crate::websocket_url(parse_network(&network)?))
}

/// Validates a subscription and builds its subscribe command.
#[frb(sync)]
pub fn websocket_subscribe_json(subscription_json: String) -> Result<String, String> {
    crate::websocket_subscribe_json(subscription_json)
}

/// Validates a subscription and builds its unsubscribe command.
#[frb(sync)]
pub fn websocket_unsubscribe_json(subscription_json: String) -> Result<String, String> {
    crate::websocket_unsubscribe_json(subscription_json)
}

/// Returns the canonical WebSocket heartbeat command.
#[frb(sync)]
pub fn websocket_ping_json() -> Result<String, String> {
    crate::websocket_ping_json()
}

/// Parses one inbound WebSocket message into stable JSON.
#[frb(sync)]
pub fn parse_websocket_message_json(message: String) -> Result<String, String> {
    crate::parse_websocket_message_json(message)
}

fn parse_network(value: &str) -> Result<Network, String> {
    match value {
        "mainnet" => Ok(Network::Mainnet),
        "testnet" => Ok(Network::Testnet),
        _ => Err("unsupported network".to_owned()),
    }
}

fn parse_request_kind(value: &str) -> Result<RequestKind, String> {
    match value {
        "info" => Ok(RequestKind::Info),
        "signedAction" => Ok(RequestKind::SignedAction),
        _ => Err("unsupported request kind".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        build_typed_info_request_json, parse_http_response_json, websocket_subscribe_json,
    };

    const USER: &str = "0x000000000000000000000000000000000000beef";

    #[test]
    fn bridges_info_http_and_websocket_protocols() {
        let request = build_typed_info_request_json(
            "mainnet".to_owned(),
            serde_json::json!({"type": "openOrders", "user": USER, "dex": ""}).to_string(),
        )
        .unwrap();
        let request: Value = serde_json::from_str(&request).unwrap();
        assert_eq!(request["kind"], "info");
        assert!(request["body"].as_str().unwrap().contains("openOrders"));

        let parsed = parse_http_response_json("info".to_owned(), 200, "[]".to_owned()).unwrap();
        let parsed: Value = serde_json::from_str(&parsed).unwrap();
        assert_eq!(parsed["kind"], "success");

        let frame = websocket_subscribe_json(
            serde_json::json!({"type": "l2Book", "coin": "BTC"}).to_string(),
        )
        .unwrap();
        assert!(frame.contains("subscribe"));
    }
}
