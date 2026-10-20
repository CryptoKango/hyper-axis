
use super::{ClientOrderId, EvmAddress};

#[test]
fn canonicalizes_evm_addresses() {
    let address: EvmAddress = "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf"
        .parse()
        .unwrap();
    assert_eq!(
        address.to_string(),
        "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
    );
    assert!("0x01".parse::<EvmAddress>().is_err());
}

#[test]
fn validates_and_canonicalizes_client_order_ids() {
    let cloid: ClientOrderId = "1234567890ABCDEF1234567890ABCDEF".parse().unwrap();
    assert_eq!(cloid.to_string(), "0x1234567890abcdef1234567890abcdef");
    assert!("0x01".parse::<ClientOrderId>().is_err());
}
