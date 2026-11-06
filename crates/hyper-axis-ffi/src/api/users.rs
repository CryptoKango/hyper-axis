//! User-signed authorization bridge operations.

use flutter_rust_bridge::frb;
use hyper_axis_core::client::{PrepareApproveBuilderFeeRequest, PreparedApproveBuilderFee};

use super::json::{decode, encode};

/// Prepares a builder-fee approval and returns its opaque signing artifact.
///
/// # Errors
///
/// Returns a sanitized protocol or JSON error.
#[frb(sync)]
pub fn prepare_approve_builder_fee_json(request_json: String) -> Result<String, String> {
    let request: PrepareApproveBuilderFeeRequest =
        decode(&request_json, "builder-fee approval request")?;
    let prepared = crate::prepare_approve_builder_fee(request)?;
    encode(&prepared, "prepared builder-fee approval")
}

/// Revalidates and finalizes a builder-fee approval.
///
/// # Errors
///
/// Returns a sanitized JSON, signature, signer, or artifact-integrity error.
#[frb(sync)]
pub fn finalize_approve_builder_fee_json(
    prepared_json: String,
    r: String,
    s: String,
    v: u8,
    expected_signer: String,
) -> Result<String, String> {
    let prepared: PreparedApproveBuilderFee =
        decode(&prepared_json, "prepared builder-fee approval")?;
    let finalized = crate::finalize_approve_builder_fee(prepared, r, s, v, expected_signer)?;
    encode(&finalized, "finalized builder-fee approval")
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::prepare_approve_builder_fee_json;

    #[test]
    fn prepares_builder_fee_approval() {
        let prepared = prepare_approve_builder_fee_json(
            r#"{
                "network":"mainnet",
                "builder":"0x000000000000000000000000000000000000b0b0",
                "maxFeeRate":"0.100%",
                "nonce":"1234567890000"
            }"#
            .to_owned(),
        )
        .unwrap();
        let value: Value = serde_json::from_str(&prepared).unwrap();

        assert_eq!(value["action"]["type"], "approveBuilderFee");
        assert_eq!(value["action"]["maxFeeRate"], "0.1%");
    }
}
