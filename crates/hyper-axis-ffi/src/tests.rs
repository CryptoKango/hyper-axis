    use super::{
        FfiNonceManager, normalize_decimal, websocket_ping_json, websocket_subscribe_json,
    };

    #[test]
    fn exposes_binding_safe_decimal_normalization() {
        assert_eq!(normalize_decimal("001.2300".to_owned()).unwrap(), "1.23");
    }

    #[test]
    fn exposes_a_signer_local_nonce_handle() {
        let manager = FfiNonceManager::new(None);
        assert_eq!(manager.next(1_000).unwrap(), 1_000);
        assert_eq!(manager.next(1_000).unwrap(), 1_001);
    }

    #[test]
    fn exposes_validated_websocket_commands() {
        assert_eq!(websocket_ping_json().unwrap(), r#"{"method":"ping"}"#);
        assert_eq!(
            websocket_subscribe_json(r#"{"type":"trades","coin":"BTC"}"#.to_owned()).unwrap(),
            r#"{"method":"subscribe","subscription":{"type":"trades","coin":"BTC"}}"#
        );
    }
