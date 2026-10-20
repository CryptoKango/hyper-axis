    use hyper_axis_decimal::DecimalValue;
    use hyper_axis_types::{AssetId, ClientOrderId};
    use serde_json::json;

    use super::{
        AgentAbstraction, AgentEnableDexAbstractionAction, AgentSetAbstractionAction,
        ApproveAgentAction, ApproveBuilderFeeAction, BatchModifyAction, BuilderFeeRate,
        CDepositAction, CWithdrawAction, CancelAction, CancelByCloidAction, CancelByCloidWire,
        CancelWire, ConvertToMultiSigUserAction, CreateSubAccountAction, EvmUserModifyAction,
        FromSubAccount, Grouping, LimitWire, ModifyWire, MultiSigSigners, NoopAction, OrderAction,
        OrderId, OrderTypeWire, OrderWire, ScheduleCancelAction, SendAssetAction,
        SetReferrerAction, SpotSendAction, SubAccountSpotTransferAction, SubAccountTransferAction,
        TimeInForce, TokenDelegateAction, TopUpIsolatedOnlyMarginAction, TwapCancelAction,
        TwapOrderAction, TwapWire, UpdateIsolatedMarginAction, UpdateLeverageAction,
        UsdClassTransferAction, UsdClassTransferAmount, UsdSendAction, UserAbstraction,
        UserDexAbstractionAction, UserSetAbstractionAction, VaultTransferAction, WithdrawAction,
    };

    fn order() -> OrderWire {
        OrderWire::new(
            AssetId::new(3),
            true,
            DecimalValue::parse("100.00").unwrap(),
            DecimalValue::parse("0.50").unwrap(),
            false,
            OrderTypeWire::Limit(LimitWire {
                tif: TimeInForce::Gtc,
            }),
            None,
        )
        .unwrap()
    }

    #[test]
    fn serializes_official_compact_order_shape() {
        let action = OrderAction::new(vec![order()], Grouping::None, None).unwrap();

        assert_eq!(
            serde_json::to_value(action).unwrap(),
            json!({
                "type": "order",
                "orders": [{
                    "a": 3,
                    "b": true,
                    "p": "100",
                    "s": "0.5",
                    "r": false,
                    "t": {"limit": {"tif": "Gtc"}}
                }],
                "grouping": "na"
            })
        );
    }

    #[test]
    fn serializes_official_cancel_shapes() {
        let by_oid = CancelAction::new(vec![CancelWire {
            asset: AssetId::new(3),
            oid: 42,
        }])
        .unwrap();
        assert_eq!(
            serde_json::to_value(by_oid).unwrap(),
            json!({"type": "cancel", "cancels": [{"a": 3, "o": 42}]})
        );

        let cloid: ClientOrderId = "0x1234567890abcdef1234567890abcdef".parse().unwrap();
        let by_cloid = CancelByCloidAction::new(vec![CancelByCloidWire {
            asset: AssetId::new(3),
            cloid,
        }])
        .unwrap();
        assert_eq!(
            serde_json::to_value(by_cloid).unwrap(),
            json!({
                "type": "cancelByCloid",
                "cancels": [{
                    "asset": 3,
                    "cloid": "0x1234567890abcdef1234567890abcdef"
                }]
            })
        );
    }

    #[test]
    fn serializes_leverage_and_margin_shapes() {
        assert_eq!(
            serde_json::to_value(
                UpdateLeverageAction::new(AssetId::new(4), false, 12, 50).unwrap()
            )
            .unwrap(),
            json!({"type": "updateLeverage", "asset": 4, "isCross": false, "leverage": 12})
        );
        assert_eq!(
            serde_json::to_value(
                UpdateIsolatedMarginAction::new(AssetId::new(4), true, -1_250_000).unwrap()
            )
            .unwrap(),
            json!({"type": "updateIsolatedMargin", "asset": 4, "isBuy": true, "ntli": -1_250_000})
        );
        assert_eq!(
            serde_json::to_value(
                TopUpIsolatedOnlyMarginAction::new(
                    AssetId::new(4),
                    DecimalValue::parse("3.500").unwrap(),
                )
                .unwrap()
            )
            .unwrap(),
            json!({"type": "topUpIsolatedOnlyMargin", "asset": 4, "leverage": "3.5"})
        );
    }

    #[test]
    fn serializes_user_signed_transfer_shapes() {
        let destination = "0x0000000000000000000000000000000000000001"
            .parse()
            .unwrap();
        let usd = UsdSendAction::new(
            hyper_axis_types::Network::Mainnet,
            destination,
            DecimalValue::parse("1.2500").unwrap(),
            1_234,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(usd).unwrap(),
            json!({
                "type": "usdSend",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "destination": "0x0000000000000000000000000000000000000001",
                "amount": "1.25",
                "time": 1_234
            })
        );

        let spot = SpotSendAction::new(
            hyper_axis_types::Network::Testnet,
            destination,
            "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            DecimalValue::parse("0.01").unwrap(),
            1_235,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(spot).unwrap(),
            json!({
                "type": "spotSend",
                "hyperliquidChain": "Testnet",
                "signatureChainId": "0x66eee",
                "destination": "0x0000000000000000000000000000000000000001",
                "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "amount": "0.01",
                "time": 1_235
            })
        );

        let withdrawal = WithdrawAction::new(
            hyper_axis_types::Network::Mainnet,
            destination,
            DecimalValue::parse("2").unwrap(),
            1_236,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(withdrawal).unwrap()["type"],
            "withdraw3"
        );
    }

    #[test]
    fn serializes_user_signed_authorization_shapes() {
        let agent = "0x000000000000000000000000000000000000beef"
            .parse()
            .unwrap();
        let named = ApproveAgentAction::new(
            hyper_axis_types::Network::Mainnet,
            agent,
            Some("mobile-primary".to_owned()),
            1_234,
        );
        assert_eq!(
            serde_json::to_value(named).unwrap(),
            json!({
                "type": "approveAgent",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "agentAddress": "0x000000000000000000000000000000000000beef",
                "agentName": "mobile-primary",
                "nonce": 1_234
            })
        );

        let unnamed =
            ApproveAgentAction::new(hyper_axis_types::Network::Testnet, agent, None, 1_235);
        assert_eq!(unnamed.signed_agent_name(), "");
        assert!(
            serde_json::to_value(unnamed)
                .unwrap()
                .get("agentName")
                .is_none()
        );

        let builder = "0x000000000000000000000000000000000000b0b0"
            .parse()
            .unwrap();
        let fee = BuilderFeeRate::parse("0.0010%").unwrap();
        let action =
            ApproveBuilderFeeAction::new(hyper_axis_types::Network::Mainnet, fee, builder, 1_236);
        assert_eq!(
            serde_json::to_value(action).unwrap(),
            json!({
                "type": "approveBuilderFee",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "maxFeeRate": "0.001%",
                "builder": "0x000000000000000000000000000000000000b0b0",
                "nonce": 1_236
            })
        );
        assert!(BuilderFeeRate::parse("-0.1%").is_err());
        assert!(BuilderFeeRate::parse("0.1").is_err());
    }

    #[test]
    fn serializes_user_signed_account_and_dex_transfer_shapes() {
        let sub_account = "0x000000000000000000000000000000000000cafe"
            .parse()
            .unwrap();
        let class_amount =
            UsdClassTransferAmount::new(DecimalValue::parse("12.500").unwrap(), Some(sub_account))
                .unwrap();
        let class_transfer = UsdClassTransferAction::new(
            hyper_axis_types::Network::Mainnet,
            class_amount,
            true,
            1_234,
        );
        assert_eq!(
            serde_json::to_value(class_transfer).unwrap(),
            json!({
                "type": "usdClassTransfer",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "amount": "12.5 subaccount:0x000000000000000000000000000000000000cafe",
                "toPerp": true,
                "nonce": 1_234
            })
        );

        let destination = "0x000000000000000000000000000000000000dead"
            .parse()
            .unwrap();
        let send = SendAssetAction::new(
            hyper_axis_types::Network::Mainnet,
            destination,
            "spot".to_owned(),
            "spot".to_owned(),
            "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            DecimalValue::parse("0.010").unwrap(),
            FromSubAccount::new(Some(sub_account)),
            1_235,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(send).unwrap(),
            json!({
                "type": "sendAsset",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "destination": "0x000000000000000000000000000000000000dead",
                "sourceDex": "spot",
                "destinationDex": "spot",
                "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "amount": "0.01",
                "fromSubAccount": "0x000000000000000000000000000000000000cafe",
                "nonce": 1_235
            })
        );

        assert_eq!(
            serde_json::to_value(FromSubAccount::new(None)).unwrap(),
            json!("")
        );
        assert!(UsdClassTransferAmount::parse("0 subaccount:0x00").is_err());
    }

    #[test]
    fn serializes_user_signed_staking_shapes() {
        let deposit =
            CDepositAction::new(hyper_axis_types::Network::Mainnet, 100_000_000, 1_234).unwrap();
        assert_eq!(
            serde_json::to_value(deposit).unwrap(),
            json!({
                "type": "cDeposit",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "wei": 100_000_000,
                "nonce": 1_234
            })
        );

        let withdrawal =
            CWithdrawAction::new(hyper_axis_types::Network::Testnet, 50_000_000, 1_235).unwrap();
        assert_eq!(
            serde_json::to_value(withdrawal).unwrap(),
            json!({
                "type": "cWithdraw",
                "hyperliquidChain": "Testnet",
                "signatureChainId": "0x66eee",
                "wei": 50_000_000,
                "nonce": 1_235
            })
        );

        let validator = "0x000000000000000000000000000000000000beef"
            .parse()
            .unwrap();
        let delegation = TokenDelegateAction::new(
            hyper_axis_types::Network::Mainnet,
            validator,
            25_000_000,
            false,
            1_236,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(delegation).unwrap(),
            json!({
                "type": "tokenDelegate",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "validator": "0x000000000000000000000000000000000000beef",
                "wei": 25_000_000,
                "isUndelegate": false,
                "nonce": 1_236
            })
        );

        assert!(CDepositAction::new(hyper_axis_types::Network::Mainnet, 0, 1_237).is_err());
        assert!(CWithdrawAction::new(hyper_axis_types::Network::Mainnet, 0, 1_237).is_err());
        assert!(
            TokenDelegateAction::new(
                hyper_axis_types::Network::Mainnet,
                validator,
                0,
                true,
                1_237,
            )
            .is_err()
        );
    }

    #[test]
    fn serializes_account_security_shapes_and_canonicalizes_signers() {
        let user = "0x000000000000000000000000000000000000cafe"
            .parse()
            .unwrap();
        assert_eq!(
            serde_json::to_value(UserDexAbstractionAction::new(
                hyper_axis_types::Network::Mainnet,
                user,
                true,
                1_234,
            ))
            .unwrap(),
            json!({
                "type": "userDexAbstraction",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "user": "0x000000000000000000000000000000000000cafe",
                "enabled": true,
                "nonce": 1_234
            })
        );
        assert_eq!(
            serde_json::to_value(UserSetAbstractionAction::new(
                hyper_axis_types::Network::Mainnet,
                user,
                UserAbstraction::PortfolioMargin,
                1_235,
            ))
            .unwrap(),
            json!({
                "type": "userSetAbstraction",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "user": "0x000000000000000000000000000000000000cafe",
                "abstraction": "portfolioMargin",
                "nonce": 1_235
            })
        );

        let lower = "0x000000000000000000000000000000000000beef"
            .parse()
            .unwrap();
        let signers = MultiSigSigners::new(vec![user, lower], 2).unwrap();
        assert_eq!(
            signers.to_wire(),
            "{\"authorizedUsers\": [\"0x000000000000000000000000000000000000beef\", \"0x000000000000000000000000000000000000cafe\"], \"threshold\": 2}"
        );
        let action =
            ConvertToMultiSigUserAction::new(hyper_axis_types::Network::Mainnet, signers, 1_236);
        let json = serde_json::to_string(&action).unwrap();
        assert_eq!(
            serde_json::from_str::<ConvertToMultiSigUserAction>(&json).unwrap(),
            action
        );
        assert_eq!(
            serde_json::to_value(action).unwrap(),
            json!({
                "type": "convertToMultiSigUser",
                "hyperliquidChain": "Mainnet",
                "signatureChainId": "0x66eee",
                "signers": "{\"authorizedUsers\": [\"0x000000000000000000000000000000000000beef\", \"0x000000000000000000000000000000000000cafe\"], \"threshold\": 2}",
                "nonce": 1_236
            })
        );

        assert!(MultiSigSigners::new(Vec::new(), 1).is_err());
        assert!(MultiSigSigners::new(vec![user], 0).is_err());
        assert!(MultiSigSigners::new(vec![user], 2).is_err());
        assert!(MultiSigSigners::new(vec![user, user], 1).is_err());
    }

    #[test]
    fn serializes_account_and_vault_l1_shapes() {
        let sub_account = "0x000000000000000000000000000000000000cafe"
            .parse()
            .unwrap();
        assert_eq!(
            serde_json::to_value(CreateSubAccountAction::new("strategy-one".to_owned()).unwrap())
                .unwrap(),
            json!({"type": "createSubAccount", "name": "strategy-one"})
        );
        assert_eq!(
            serde_json::to_value(
                SubAccountTransferAction::new(sub_account, true, 12_500_000).unwrap()
            )
            .unwrap(),
            json!({
                "type": "subAccountTransfer",
                "subAccountUser": "0x000000000000000000000000000000000000cafe",
                "isDeposit": true,
                "usd": 12_500_000
            })
        );
        assert_eq!(
            serde_json::to_value(
                SubAccountSpotTransferAction::new(
                    sub_account,
                    false,
                    "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
                    DecimalValue::parse("0.0100").unwrap(),
                )
                .unwrap()
            )
            .unwrap(),
            json!({
                "type": "subAccountSpotTransfer",
                "subAccountUser": "0x000000000000000000000000000000000000cafe",
                "isDeposit": false,
                "token": "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2",
                "amount": "0.01"
            })
        );
        let vault = "0x000000000000000000000000000000000000babe"
            .parse()
            .unwrap();
        assert_eq!(
            serde_json::to_value(VaultTransferAction::new(vault, false, 2_000_000).unwrap())
                .unwrap(),
            json!({
                "type": "vaultTransfer",
                "vaultAddress": "0x000000000000000000000000000000000000babe",
                "isDeposit": false,
                "usd": 2_000_000
            })
        );

        assert!(CreateSubAccountAction::new(String::new()).is_err());
        assert!(SubAccountTransferAction::new(sub_account, true, 0).is_err());
        assert!(VaultTransferAction::new(vault, true, 0).is_err());
    }

    #[test]
    fn serializes_account_control_l1_shapes() {
        assert_eq!(
            serde_json::to_value(SetReferrerAction::new("HYPERAXIS".to_owned()).unwrap()).unwrap(),
            json!({"type": "setReferrer", "code": "HYPERAXIS"})
        );
        assert_eq!(
            serde_json::to_value(EvmUserModifyAction::new(true)).unwrap(),
            json!({"type": "evmUserModify", "usingBigBlocks": true})
        );
        assert_eq!(
            serde_json::to_value(AgentEnableDexAbstractionAction::new()).unwrap(),
            json!({"type": "agentEnableDexAbstraction"})
        );
        assert_eq!(
            serde_json::to_value(AgentSetAbstractionAction::new(
                AgentAbstraction::PortfolioMargin
            ))
            .unwrap(),
            json!({"type": "agentSetAbstraction", "abstraction": "p"})
        );
        assert_eq!(
            serde_json::to_value(NoopAction::new()).unwrap(),
            json!({"type": "noop"})
        );
        assert!(SetReferrerAction::new(String::new()).is_err());
        assert!(SetReferrerAction::new("invalid\ncode".to_owned()).is_err());
    }

    #[test]
    fn serializes_official_batch_modify_shape() {
        let action = BatchModifyAction::new(vec![ModifyWire {
            oid: OrderId::Oid(42),
            order: order(),
        }])
        .unwrap();
        assert_eq!(
            serde_json::to_value(action).unwrap(),
            json!({
                "type": "batchModify",
                "modifies": [{
                    "oid": 42,
                    "order": {
                        "a": 3,
                        "b": true,
                        "p": "100",
                        "s": "0.5",
                        "r": false,
                        "t": {"limit": {"tif": "Gtc"}}
                    }
                }]
            })
        );
        assert!(BatchModifyAction::new(Vec::new()).is_err());
    }

    #[test]
    fn serializes_schedule_and_twap_shapes() {
        assert_eq!(
            serde_json::to_value(ScheduleCancelAction::new(None)).unwrap(),
            json!({"type": "scheduleCancel"})
        );
        assert_eq!(
            serde_json::to_value(ScheduleCancelAction::new(Some(10_000))).unwrap(),
            json!({"type": "scheduleCancel", "time": 10_000})
        );

        let twap = TwapWire::new(
            AssetId::new(3),
            true,
            DecimalValue::parse("1.25").unwrap(),
            false,
            5,
            true,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(TwapOrderAction::new(twap)).unwrap(),
            json!({
                "type": "twapOrder",
                "twap": {"a": 3, "b": true, "s": "1.25", "r": false, "m": 5, "t": true}
            })
        );
        assert_eq!(
            serde_json::to_value(TwapCancelAction::new(AssetId::new(3), 77)).unwrap(),
            json!({"type": "twapCancel", "a": 3, "t": 77})
        );
    }
