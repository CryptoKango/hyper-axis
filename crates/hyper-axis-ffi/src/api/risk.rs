//! Risk-setting bridge operations.

use flutter_rust_bridge::frb;
use hyper_axis_core::client::{
    PrepareUpdateIsolatedMarginRequest, PrepareUpdateLeverageRequest, PreparedUpdateIsolatedMargin,
    PreparedUpdateLeverage,
};

use super::json::{decode, encode};

/// Validates a cross/isolated leverage update and returns its opaque artifact.
///
/// # Errors
///
/// Returns a sanitized protocol or JSON error.
#[frb(sync)]
pub fn prepare_update_leverage_json(request_json: String) -> Result<String, String> {
    let request: PrepareUpdateLeverageRequest = decode(&request_json, "update-leverage request")?;
    let prepared = crate::prepare_update_leverage(request)?;
    encode(&prepared, "prepared leverage update")
}

/// Revalidates and finalizes a leverage update after external signing.
///
/// # Errors
///
/// Returns a sanitized JSON, signature, signer, or artifact-integrity error.
#[frb(sync)]
pub fn finalize_update_leverage_json(
    prepared_json: String,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<String, String> {
    let prepared: PreparedUpdateLeverage = decode(&prepared_json, "prepared leverage update")?;
    let finalized = crate::finalize_update_leverage(prepared, r, s, v, expected_signer)?;
    encode(&finalized, "finalized leverage update")
}

/// Validates an exact isolated-margin USD delta and returns its opaque artifact.
#[frb(sync)]
pub fn prepare_update_isolated_margin_json(request_json: String) -> Result<String, String> {
    let request: PrepareUpdateIsolatedMarginRequest =
        decode(&request_json, "update-isolated-margin request")?;
    let prepared = crate::prepare_update_isolated_margin(request)?;
    encode(&prepared, "prepared isolated-margin update")
}

/// Revalidates and finalizes an isolated-margin update after external signing.
#[frb(sync)]
pub fn finalize_update_isolated_margin_json(
    prepared_json: String,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<String, String> {
    let prepared: PreparedUpdateIsolatedMargin =
        decode(&prepared_json, "prepared isolated-margin update")?;
    let finalized = crate::finalize_update_isolated_margin(prepared, r, s, v, expected_signer)?;
    encode(&finalized, "finalized isolated-margin update")
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        finalize_update_leverage_json, prepare_update_isolated_margin_json,
        prepare_update_leverage_json,
    };

    const REQUEST: &str = r#"{
        "network":"mainnet",
        "asset":4,
        "isCross":false,
        "leverage":12,
        "maxLeverage":50,
        "nonce":"1234567890000",
        "vaultAddress":null,
        "expiresAfter":null
    }"#;

    #[test]
    fn prepares_the_independent_leverage_vector() {
        let prepared_json = prepare_update_leverage_json(REQUEST.to_owned()).unwrap();
        let prepared: Value = serde_json::from_str(&prepared_json).unwrap();

        assert_eq!(
            prepared["action"],
            serde_json::json!({
                "type": "updateLeverage",
                "asset": 4,
                "isCross": false,
                "leverage": 12
            })
        );
        assert_eq!(prepared["nonce"], "1234567890000");
        assert_eq!(
            prepared["actionHashHex"],
            "cd0f79c94dacd305420c29f4bc8b140ceadbc27f7fa6336d767a7e31fea410ad"
        );
    }

    #[test]
    fn rejects_out_of_range_leverage_and_invalid_signature() {
        let invalid = REQUEST.replace("\"leverage\":12", "\"leverage\":51");
        assert!(prepare_update_leverage_json(invalid).is_err());

        let prepared = prepare_update_leverage_json(REQUEST.to_owned()).unwrap();
        assert!(
            finalize_update_leverage_json(
                prepared,
                "0x00".to_owned(),
                "0x00".to_owned(),
                27,
                "0x000000000000000000000000000000000000beef".to_owned(),
            )
            .is_err()
        );
    }

    #[test]
    fn prepares_an_exact_isolated_margin_delta() {
        let request = r#"{
            "network":"mainnet","asset":4,"isBuy":true,
            "usdDelta":"12.345678","nonce":"1234567890000",
            "vaultAddress":null,"expiresAfter":null
        }"#;
        let prepared_json = prepare_update_isolated_margin_json(request.to_owned()).unwrap();
        let prepared: Value = serde_json::from_str(&prepared_json).unwrap();
        assert_eq!(prepared["action"]["type"], "updateIsolatedMargin");
        assert_eq!(prepared["action"]["ntli"], 12_345_678);
    }
}
