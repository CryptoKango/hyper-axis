//! Order preparation and finalization bridge operations.

use flutter_rust_bridge::frb;
use hyper_axis_core::client::{PrepareLimitOrderRequest, PreparedOrder};

use super::json::{decode, encode};

/// Validates a limit-order request and returns the opaque prepared artifact.
///
/// The JSON boundary keeps all exact integer and decimal fields in their Rust
/// wire representation. Signing and hashing are performed only by Rust.
///
/// # Errors
///
/// Returns a sanitized protocol or JSON error.
#[frb(sync)]
pub fn prepare_limit_order_json(request_json: String) -> Result<String, String> {
    let request: PrepareLimitOrderRequest = decode(&request_json, "limit-order request")?;
    let prepared = crate::prepare_limit_order(request)?;
    encode(&prepared, "prepared order")
}

/// Revalidates a prepared order, verifies its signature, and returns the exact
/// exchange request artifact.
///
/// # Errors
///
/// Returns a sanitized JSON, signature, signer, or artifact-integrity error.
#[frb(sync)]
pub fn finalize_order_json(
    prepared_json: String,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<String, String> {
    let prepared: PreparedOrder = decode(&prepared_json, "prepared order")?;
    let finalized = crate::finalize_order(prepared, r, s, v, expected_signer)?;
    encode(&finalized, "finalized order")
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{finalize_order_json, prepare_limit_order_json};

    const REQUEST: &str = r#"{
        "network":"mainnet",
        "asset":3,
        "marketKind":"perpetual",
        "szDecimals":2,
        "isBuy":true,
        "limitPrice":"100.00",
        "size":"0.50",
        "reduceOnly":false,
        "timeInForce":"Gtc",
        "clientOrderId":null,
        "grouping":"na",
        "nonce":"1234567890000",
        "vaultAddress":null,
        "expiresAfter":null
    }"#;

    #[test]
    fn prepares_and_finalizes_the_independent_order_vector() {
        let prepared_json = prepare_limit_order_json(REQUEST.to_owned()).unwrap();
        let prepared: Value = serde_json::from_str(&prepared_json).unwrap();
        assert_eq!(
            prepared["signingDigestHex"],
            "329f08729a8f946791ed6e4046ad7d94fb9be574629f1892b48baa6578abc0bc"
        );
        assert_eq!(prepared["nonce"], "1234567890000");

        let finalized_json = finalize_order_json(
            prepared_json,
            "0xc466abc93e91e15884da406ef9f37e72e924995f0661a286519e0a9e5c7ff919".to_owned(),
            "0x7cd5cef5728c82b9edcc45cf243f701b464d4b292f5c5d15605028902a1a3b20".to_owned(),
            27,
            "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf".to_owned(),
        )
        .unwrap();
        let finalized: Value = serde_json::from_str(&finalized_json).unwrap();
        assert_eq!(
            finalized["endpoint"],
            "https://api.hyperliquid.xyz/exchange"
        );
        assert_eq!(
            finalized["recoveredSigner"],
            "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
        );
    }

    #[test]
    fn rejects_numeric_nonce_at_the_bridge_boundary() {
        let request = REQUEST.replace("\"1234567890000\"", "1234567890000");
        assert!(prepare_limit_order_json(request).is_err());
    }
}
