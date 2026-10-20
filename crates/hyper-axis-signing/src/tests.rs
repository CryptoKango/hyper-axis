    use hyper_axis_types::{EvmAddress, Network};

    use super::{
        L1_SIGNATURE_CHAIN_ID, USER_SIGNATURE_CHAIN_ID, WireSignature,
        approve_agent_signing_digest, approve_builder_fee_signing_digest, c_deposit_signing_digest,
        c_withdraw_signing_digest, convert_to_multi_sig_user_signing_digest, l1_domain,
        l1_signing_digest, multi_sig_envelope_signing_digest, multi_sig_send_asset_signing_digest,
        multi_sig_user_set_abstraction_signing_digest, phantom_agent, send_asset_signing_digest,
        spot_send_signing_digest, token_delegate_signing_digest, usd_class_transfer_signing_digest,
        usd_send_signing_digest, user_dex_abstraction_signing_digest,
        user_set_abstraction_signing_digest, withdraw_signing_digest,
    };

    #[test]
    fn uses_official_domain_and_network_source_constants() {
        assert_eq!(L1_SIGNATURE_CHAIN_ID, 1337);
        assert_eq!(USER_SIGNATURE_CHAIN_ID, 0x66eee);
        assert_eq!(l1_domain().name, "Exchange");
        assert_eq!(phantom_agent(Network::Mainnet, [0; 32]).source, "a");
        assert_eq!(phantom_agent(Network::Testnet, [0; 32]).source, "b");
    }

    #[test]
    fn signing_digest_depends_on_network_and_action_hash() {
        let mainnet = l1_signing_digest(Network::Mainnet, [1; 32]);
        assert_ne!(mainnet, l1_signing_digest(Network::Testnet, [1; 32]));
        assert_ne!(mainnet, l1_signing_digest(Network::Mainnet, [2; 32]));
    }

    #[test]
    fn matches_official_python_eip712_vector() {
        let action_hash =
            hex_literal("48dc49f678049fbcc913f2499325aaefecaf3a280f90b04236a0e3aeea163ebb");
        assert_eq!(
            hex::encode(l1_signing_digest(Network::Mainnet, action_hash)),
            "329f08729a8f946791ed6e4046ad7d94fb9be574629f1892b48baa6578abc0bc"
        );
    }

    #[test]
    fn validates_and_recovers_official_python_signature_vector() {
        let digest =
            hex_literal("329f08729a8f946791ed6e4046ad7d94fb9be574629f1892b48baa6578abc0bc");
        let signature = WireSignature::new(
            "0xc466abc93e91e15884da406ef9f37e72e924995f0661a286519e0a9e5c7ff919",
            "0x7cd5cef5728c82b9edcc45cf243f701b464d4b292f5c5d15605028902a1a3b20",
            27,
        )
        .unwrap();
        let expected: EvmAddress = "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf"
            .parse()
            .unwrap();

        assert_eq!(signature.verify_signer(digest, expected).unwrap(), expected);
        assert_eq!(signature.v(), 27);
    }

    #[test]
    fn rejects_invalid_recovery_values_and_scalar_lengths() {
        assert!(WireSignature::new("0x01", "0x02", 27).is_err());
        assert!(WireSignature::new("00".repeat(32), "01".repeat(32), 29).is_err());
    }

    #[test]
    fn matches_independent_user_action_eip712_vectors() {
        let destination = "0x000000000000000000000000000000000000dead";
        let time = 1_234_567_890_000;
        assert_eq!(
            hex::encode(usd_send_signing_digest(
                Network::Mainnet,
                destination,
                "12.5",
                time,
            )),
            "e887d1d7857cd5fd0a4ada4afde0bbccab52952c9f00c76fe780f1e274c950b9"
        );
        assert_eq!(
            hex::encode(spot_send_signing_digest(
                Network::Mainnet,
                destination,
                "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "0.01",
                time,
            )),
            "0f21ebb32da085e4342aa4a9287af851aaa2dd22fd947a275c97bc32e661fc0b"
        );
        assert_eq!(
            hex::encode(withdraw_signing_digest(
                Network::Mainnet,
                destination,
                "3.25",
                time,
            )),
            "f0881e5bc8f5b56640c83e30e4a75f21c5a2ef09f932852f7db283c3491cbd48"
        );
        assert_ne!(
            usd_send_signing_digest(Network::Mainnet, destination, "12.5", time),
            usd_send_signing_digest(Network::Testnet, destination, "12.5", time),
        );
    }

    #[test]
    fn matches_independent_user_authorization_eip712_vectors() {
        let nonce = 1_234_567_890_000;
        let agent: EvmAddress = "0x000000000000000000000000000000000000beef"
            .parse()
            .unwrap();
        assert_eq!(
            hex::encode(approve_agent_signing_digest(
                Network::Mainnet,
                &agent,
                "mobile-primary",
                nonce,
            )),
            "faf9083f68b3c0c14c5aad4dc17b71ae18172d67986fd3ce3c0339ccd0197c69"
        );

        let builder: EvmAddress = "0x000000000000000000000000000000000000b0b0"
            .parse()
            .unwrap();
        assert_eq!(
            hex::encode(approve_builder_fee_signing_digest(
                Network::Mainnet,
                "0.001%",
                &builder,
                nonce,
            )),
            "4852876ed63bedbd852da154bc11767067dd453cbe1f26085b4b089a63cb8989"
        );

        assert_ne!(
            approve_agent_signing_digest(Network::Mainnet, &agent, "", nonce),
            approve_agent_signing_digest(Network::Mainnet, &agent, "mobile-primary", nonce),
        );
        assert_eq!(
            hex::encode(approve_agent_signing_digest(
                Network::Mainnet,
                &agent,
                "",
                nonce,
            )),
            "468e00ead5c01b59bbea8fdbdbb3d2b4d269261458883c37ea9542c976eede3b"
        );
    }

    #[test]
    fn matches_independent_account_and_dex_transfer_vectors() {
        let nonce = 1_234_567_890_000;
        assert_eq!(
            hex::encode(usd_class_transfer_signing_digest(
                Network::Mainnet,
                "12.5 subaccount:0x000000000000000000000000000000000000cafe",
                true,
                nonce,
            )),
            "ee2cb87725d2b45f866c8b81a294a87b63c59f74a67998b4ba322fa3e80a0707"
        );
        assert_eq!(
            hex::encode(send_asset_signing_digest(
                Network::Mainnet,
                "0x000000000000000000000000000000000000dead",
                "spot",
                "spot",
                "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "0.01",
                "0x000000000000000000000000000000000000cafe",
                nonce,
            )),
            "ae1c690aeb97bb3af5c21b9c9a1957d99e845b5fc0fe7a1f097e413073e76c64"
        );
    }

    #[test]
    fn matches_independent_staking_action_vectors() {
        let nonce = 1_234_567_890_000;
        let validator: EvmAddress = "0x000000000000000000000000000000000000beef"
            .parse()
            .unwrap();
        assert_eq!(
            hex::encode(c_deposit_signing_digest(
                Network::Mainnet,
                100_000_000,
                nonce,
            )),
            "4c29f3ee7670df095864f567b3af7c0f763b31365ba06406a3187321f98c2b57"
        );
        assert_eq!(
            hex::encode(c_withdraw_signing_digest(
                Network::Mainnet,
                50_000_000,
                nonce,
            )),
            "f04127bd02f74f3a8ee6c5d7c2f536f53ceb6a50a0a93e5cdcc2cf2f63bb1385"
        );
        assert_eq!(
            hex::encode(token_delegate_signing_digest(
                Network::Mainnet,
                &validator,
                25_000_000,
                false,
                nonce,
            )),
            "0562ccd7acc3c69495c25eba2e3c83f82487a20e50a85f5428d2384f91d8d37a"
        );
    }

    #[test]
    fn matches_independent_account_security_vectors() {
        let nonce = 1_234_567_890_000;
        let user: EvmAddress = "0x000000000000000000000000000000000000cafe"
            .parse()
            .unwrap();
        assert_eq!(
            hex::encode(user_dex_abstraction_signing_digest(
                Network::Mainnet,
                &user,
                true,
                nonce,
            )),
            "5aef5cad2e821e3fbe527b9c6d0025a37d65ff36d801907352a81e73383aaf01"
        );
        assert_eq!(
            hex::encode(user_set_abstraction_signing_digest(
                Network::Mainnet,
                &user,
                "portfolioMargin",
                nonce,
            )),
            "30f4e8d850a01e40c99b6527da1d4ef4746f1f75aa1e064fe32c7620b8b829bb"
        );
        let signers = "{\"authorizedUsers\": [\"0x000000000000000000000000000000000000beef\", \"0x000000000000000000000000000000000000cafe\"], \"threshold\": 2}";
        assert_eq!(
            hex::encode(convert_to_multi_sig_user_signing_digest(
                Network::Mainnet,
                signers,
                nonce,
            )),
            "e2b29a94a191a232e0013446c50558538ea47fc0de46bab76b5a177e3bdd949a"
        );
    }

    #[test]
    fn matches_independent_multi_signature_digests() {
        let nonce = 1_234_567_890_000;
        let multi_sig_user: EvmAddress = "0x0000000000000000000000000000000000000005"
            .parse()
            .unwrap();
        let outer_signer: EvmAddress = "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
            .parse()
            .unwrap();
        assert_eq!(
            hex::encode(multi_sig_send_asset_signing_digest(
                Network::Mainnet,
                &multi_sig_user,
                &outer_signer,
                "0x000000000000000000000000000000000000dead",
                "",
                "spot",
                "USDC",
                "12.5",
                "",
                nonce,
            )),
            "a863bcca570bd9745d0c4ad696c8c217ce7f3e519ab3559aa1f6f25f39c9e236"
        );
        assert_eq!(
            hex::encode(multi_sig_user_set_abstraction_signing_digest(
                Network::Mainnet,
                &multi_sig_user,
                &outer_signer,
                &multi_sig_user,
                "portfolioMargin",
                nonce,
            )),
            "15a4962b2f3daeed64ee0efa5f6b9d80cfdaabad919a3462442d8ef2407fe345"
        );
        let action_hash =
            hex_literal("45b45d04f7eef7df611cce3cd52016da8ac2997051bf5456513268b2fab252b2");
        assert_eq!(
            hex::encode(multi_sig_envelope_signing_digest(
                Network::Mainnet,
                action_hash,
                nonce,
            )),
            "18a64efcbfe2d484a61fdf1354bacac56bfc9e5aa7898801783104dffa17a2b7"
        );
    }

    fn hex_literal(value: &str) -> [u8; 32] {
        hex::decode(value).unwrap().try_into().unwrap()
    }
