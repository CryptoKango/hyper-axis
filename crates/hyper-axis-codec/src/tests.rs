
use hyper_axis_actions::{Grouping, LimitWire, OrderAction, OrderTypeWire, OrderWire, TimeInForce};
use hyper_axis_decimal::DecimalValue;
use hyper_axis_types::{AssetId, EvmAddress};
use serde::Serialize;

use super::{action_hash, encode_action, prefixed_hex};

#[derive(Serialize)]
struct Action {
    #[serde(rename = "type")]
    action_type: &'static str,
    value: u8,
}

#[test]
fn hash_changes_for_signing_context() {
    let action = Action {
        action_type: "example",
        value: 7,
    };
    let base = action_hash(&action, None, 1, None).unwrap();
    let different_nonce = action_hash(&action, None, 2, None).unwrap();
    let vault_address: EvmAddress = "0x0000000000000000000000000000000000000001"
        .parse()
        .unwrap();
    let vault = action_hash(&action, Some(&vault_address), 1, None).unwrap();
    let expiry = action_hash(&action, None, 1, Some(2)).unwrap();

    assert_ne!(base, different_nonce);
    assert_ne!(base, vault);
    assert_ne!(base, expiry);
    assert_eq!(prefixed_hex(&base).len(), 66);
}

#[test]
fn matches_official_python_messagepack_and_keccak_vector() {
    let order = OrderWire::new(
        AssetId::new(3),
        true,
        DecimalValue::parse("100").unwrap(),
        DecimalValue::parse("0.5").unwrap(),
        false,
        OrderTypeWire::Limit(LimitWire {
            tif: TimeInForce::Gtc,
        }),
        None,
    )
    .unwrap();
    let action = OrderAction::new(vec![order], Grouping::None, None).unwrap();

    assert_eq!(
        hex::encode(encode_action(&action).unwrap()),
        "83a474797065a56f72646572a66f72646572739186a16103a162c3a170a3313030a173a3302e35a172c2a17481a56c696d697481a3746966a3477463a867726f7570696e67a26e61"
    );
    assert_eq!(
        hex::encode(action_hash(&action, None, 1_234_567_890_000, None).unwrap()),
        "48dc49f678049fbcc913f2499325aaefecaf3a280f90b04236a0e3aeea163ebb"
    );
}
