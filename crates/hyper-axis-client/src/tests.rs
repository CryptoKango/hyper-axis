    use hyper_axis_actions::{
        AgentAbstraction, CSignerVariant, Grouping, TimeInForce, TriggerKind, UserAbstraction,
    };
    use hyper_axis_assets::MarketKind;
    use hyper_axis_signing::WireSignature;
    use hyper_axis_types::{AssetId, EvmAddress, Network};
    use serde_json::Value;

    use super::{
        BuilderInput, CancelByCloidInput, CancelInput, FinalizeError, LimitOrderInput,
        ModifyOrderInput, MultiSigSignatureInput, OrderInput, OrderSelectorInput, PerpDeployInput,
        PerpDexSchemaInput, PerpPriceInput, PrepareAgentEnableDexAbstractionRequest,
        PrepareAgentSetAbstractionRequest, PrepareApproveAgentRequest,
        PrepareApproveBuilderFeeRequest, PrepareBatchModifyRequest, PrepareBatchOrdersRequest,
        PrepareCDepositRequest, PrepareCSignerRequest, PrepareCValidatorChangeProfileRequest,
        PrepareCValidatorRegisterRequest, PrepareCValidatorUnregisterRequest,
        PrepareCWithdrawRequest, PrepareCancelByCloidRequest, PrepareCancelRequest,
        PrepareConvertToMultiSigUserRequest, PrepareCreateSubAccountRequest,
        PrepareEvmUserModifyRequest, PrepareGossipPriorityBidRequest, PrepareLimitOrderRequest,
        PrepareNoopRequest, PrepareOrdersRequest, PreparePerpDeployRequest,
        PrepareScheduleCancelRequest, PrepareSendAssetRequest, PrepareSetReferrerRequest,
        PrepareSpotDeployRequest, PrepareSpotSendRequest, PrepareSubAccountSpotTransferRequest,
        PrepareSubAccountTransferRequest, PrepareTokenDelegateRequest,
        PrepareTopUpIsolatedOnlyMarginRequest, PrepareTwapCancelRequest, PrepareTwapOrderRequest,
        PrepareUpdateIsolatedMarginRequest, PrepareUpdateLeverageRequest,
        PrepareUsdClassTransferRequest, PrepareUsdSendRequest, PrepareUserDexAbstractionRequest,
        PrepareUserSetAbstractionRequest, PrepareVaultTransferRequest, PrepareWithdrawRequest,
        PreparedOrder, SpotDeployInput, SpotExistingTokenWeiInput, SpotUserWeiInput,
        TriggerOrderInput, finalize_agent_enable_dex_abstraction, finalize_agent_set_abstraction,
        finalize_approve_agent, finalize_approve_builder_fee, finalize_c_deposit,
        finalize_c_signer, finalize_c_validator, finalize_c_withdraw,
        finalize_convert_to_multi_sig_user, finalize_create_sub_account, finalize_evm_user_modify,
        finalize_gossip_priority_bid, finalize_multi_sig, finalize_noop, finalize_order,
        finalize_perp_deploy, finalize_send_asset, finalize_set_referrer, finalize_spot_deploy,
        finalize_spot_send, finalize_sub_account_spot_transfer, finalize_sub_account_transfer,
        finalize_token_delegate, finalize_usd_class_transfer, finalize_usd_send,
        finalize_user_dex_abstraction, finalize_user_set_abstraction, finalize_vault_transfer,
        finalize_withdraw, prepare_agent_enable_dex_abstraction, prepare_agent_set_abstraction,
        prepare_approve_agent, prepare_approve_builder_fee, prepare_batch_modify,
        prepare_batch_orders, prepare_c_deposit, prepare_c_signer,
        prepare_c_validator_change_profile, prepare_c_validator_register,
        prepare_c_validator_unregister, prepare_c_withdraw, prepare_cancel,
        prepare_cancel_by_cloid, prepare_convert_to_multi_sig_user, prepare_create_sub_account,
        prepare_evm_user_modify, prepare_gossip_priority_bid, prepare_limit_order,
        prepare_multi_sig_agent_enable_dex_abstraction_payload,
        prepare_multi_sig_agent_set_abstraction_payload, prepare_multi_sig_approve_agent_payload,
        prepare_multi_sig_approve_builder_fee_payload, prepare_multi_sig_batch_modify_payload,
        prepare_multi_sig_c_deposit_payload, prepare_multi_sig_c_signer_payload,
        prepare_multi_sig_c_validator_payload, prepare_multi_sig_c_withdraw_payload,
        prepare_multi_sig_cancel_by_cloid_payload, prepare_multi_sig_cancel_payload,
        prepare_multi_sig_create_sub_account_payload, prepare_multi_sig_envelope,
        prepare_multi_sig_evm_user_modify_payload, prepare_multi_sig_gossip_priority_bid_payload,
        prepare_multi_sig_noop_payload, prepare_multi_sig_order_payload,
        prepare_multi_sig_perp_deploy_payload, prepare_multi_sig_schedule_cancel_payload,
        prepare_multi_sig_send_asset_payload, prepare_multi_sig_set_referrer_payload,
        prepare_multi_sig_spot_deploy_payload, prepare_multi_sig_spot_send_payload,
        prepare_multi_sig_sub_account_spot_transfer_payload,
        prepare_multi_sig_sub_account_transfer_payload, prepare_multi_sig_token_delegate_payload,
        prepare_multi_sig_top_up_isolated_only_margin_payload,
        prepare_multi_sig_twap_cancel_payload, prepare_multi_sig_twap_order_payload,
        prepare_multi_sig_update_isolated_margin_payload,
        prepare_multi_sig_update_leverage_payload, prepare_multi_sig_usd_class_transfer_payload,
        prepare_multi_sig_usd_send_payload, prepare_multi_sig_user_dex_abstraction_payload,
        prepare_multi_sig_user_set_abstraction_payload, prepare_multi_sig_vault_transfer_payload,
        prepare_multi_sig_withdraw_payload, prepare_noop, prepare_orders, prepare_perp_deploy,
        prepare_schedule_cancel, prepare_send_asset, prepare_set_referrer, prepare_spot_deploy,
        prepare_spot_send, prepare_sub_account_spot_transfer, prepare_sub_account_transfer,
        prepare_token_delegate, prepare_top_up_isolated_only_margin, prepare_twap_cancel,
        prepare_twap_order, prepare_update_isolated_margin, prepare_update_leverage,
        prepare_usd_class_transfer, prepare_usd_send, prepare_user_dex_abstraction,
        prepare_user_set_abstraction, prepare_vault_transfer, prepare_withdraw,
    };

    fn request() -> PrepareLimitOrderRequest {
        PrepareLimitOrderRequest {
            network: Network::Mainnet,
            asset: AssetId::new(3),
            market_kind: MarketKind::Perpetual,
            sz_decimals: 2,
            is_buy: true,
            limit_price: "100.00".to_owned(),
            size: "0.50".to_owned(),
            reduce_only: false,
            time_in_force: TimeInForce::Gtc,
            client_order_id: None,
            grouping: Grouping::None,
            nonce: 1_234_567_890_000,
            vault_address: None,
            expires_after: None,
        }
    }

    fn order_input(price: &str) -> LimitOrderInput {
        LimitOrderInput {
            asset: AssetId::new(3),
            market_kind: MarketKind::Perpetual,
            sz_decimals: 2,
            is_buy: true,
            limit_price: price.to_owned(),
            size: "0.50".to_owned(),
            reduce_only: false,
            time_in_force: TimeInForce::Gtc,
            client_order_id: None,
        }
    }

    #[test]
    fn prepares_cross_platform_signing_artifacts() {
        let prepared = prepare_limit_order(request()).unwrap();
        assert_eq!(
            prepared.action_hash_hex,
            "48dc49f678049fbcc913f2499325aaefecaf3a280f90b04236a0e3aeea163ebb"
        );
        assert_eq!(
            prepared.signing_digest_hex,
            "329f08729a8f946791ed6e4046ad7d94fb9be574629f1892b48baa6578abc0bc"
        );
        assert!(prepared.action_json.contains("\"p\":\"100\""));
    }

    #[test]
    fn rejects_precision_before_creating_signing_artifacts() {
        let mut invalid = request();
        invalid.size = "0.001".to_owned();
        assert!(prepare_limit_order(invalid).is_err());
    }

    #[test]
    fn verifies_signature_and_builds_exchange_request() {
        let prepared = prepare_limit_order(request()).unwrap();
        let signature = python_vector_signature();
        let vector = python_vector();
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();
        let finalized = finalize_order(prepared, signature, expected).unwrap();
        let body: Value = serde_json::from_str(&finalized.request_json).unwrap();

        assert_eq!(finalized.endpoint, vector["exchangeEndpoint"]);
        assert_eq!(finalized.recovered_signer, expected);
        assert_eq!(body, vector["exchangeRequest"]);
    }

    #[test]
    fn rejects_modified_prepared_artifacts() {
        let mut prepared = prepare_limit_order(request()).unwrap();
        prepared.signing_digest_hex.replace_range(0..2, "00");
        let expected: EvmAddress = "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
            .parse()
            .unwrap();

        assert!(matches!(
            finalize_order(prepared, python_vector_signature(), expected),
            Err(FinalizeError::ArtifactMismatch("signingDigestHex"))
        ));
    }

    #[test]
    fn prepares_atomic_batch_orders_with_validated_builder() {
        let prepared = prepare_batch_orders(PrepareBatchOrdersRequest {
            network: Network::Testnet,
            orders: vec![order_input("100"), order_input("101")],
            grouping: Grouping::None,
            builder: Some(BuilderInput {
                address: "0x0000000000000000000000000000000000000001".to_owned(),
                fee: 10,
            }),
            nonce: 1_000,
            vault_address: None,
            expires_after: Some(2_000),
        })
        .unwrap();
        let action: Value = serde_json::from_str(&prepared.action_json).unwrap();
        assert_eq!(action["orders"].as_array().unwrap().len(), 2);
        assert_eq!(action["builder"]["f"], 10);

        let json = serde_json::to_value(&prepared).unwrap();
        assert_eq!(json["nonce"], "1000");
        assert_eq!(json["expiresAfter"], "2000");
        let round_trip: PreparedOrder = serde_json::from_value(json).unwrap();
        assert_eq!(round_trip, prepared);
    }

    #[test]
    fn prepares_official_cancel_actions_and_rejects_invalid_cloid() {
        let by_oid = prepare_cancel(PrepareCancelRequest {
            network: Network::Mainnet,
            cancels: vec![CancelInput {
                asset: AssetId::new(3),
                oid: 42,
            }],
            nonce: 1_000,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&by_oid.action_json).unwrap(),
            serde_json::json!({"type": "cancel", "cancels": [{"a": 3, "o": 42}]})
        );

        let invalid = prepare_cancel_by_cloid(PrepareCancelByCloidRequest {
            network: Network::Mainnet,
            cancels: vec![CancelByCloidInput {
                asset: AssetId::new(3),
                cloid: "0x01".to_owned(),
            }],
            nonce: 1_001,
            vault_address: None,
            expires_after: None,
        });
        assert!(invalid.is_err());
    }

    #[test]
    fn prepares_risk_actions_with_exact_validation() {
        let leverage = prepare_update_leverage(PrepareUpdateLeverageRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            is_cross: false,
            leverage: 12,
            max_leverage: 50,
            nonce: 1_000,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&leverage.action_json).unwrap(),
            serde_json::json!({
                "type": "updateLeverage",
                "asset": 4,
                "isCross": false,
                "leverage": 12
            })
        );

        let margin = prepare_update_isolated_margin(PrepareUpdateIsolatedMarginRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            is_buy: true,
            usd_delta: "-1.250000".to_owned(),
            nonce: 1_001,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&margin.action_json).unwrap(),
            serde_json::json!({
                "type": "updateIsolatedMargin",
                "asset": 4,
                "isBuy": true,
                "ntli": -1_250_000
            })
        );

        let top_up = prepare_top_up_isolated_only_margin(PrepareTopUpIsolatedOnlyMarginRequest {
            network: Network::Testnet,
            asset: AssetId::new(4),
            leverage: "3.500".to_owned(),
            nonce: 1_002,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&top_up.action_json).unwrap(),
            serde_json::json!({
                "type": "topUpIsolatedOnlyMargin",
                "asset": 4,
                "leverage": "3.5"
            })
        );

        assert!(
            prepare_update_leverage(PrepareUpdateLeverageRequest {
                network: Network::Mainnet,
                asset: AssetId::new(4),
                is_cross: true,
                leverage: 51,
                max_leverage: 50,
                nonce: 2_000,
                vault_address: None,
                expires_after: None,
            })
            .is_err()
        );
        assert!(
            prepare_update_isolated_margin(PrepareUpdateIsolatedMarginRequest {
                network: Network::Mainnet,
                asset: AssetId::new(4),
                is_buy: true,
                usd_delta: "0.0000001".to_owned(),
                nonce: 2_001,
                vault_address: None,
                expires_after: None,
            })
            .is_err()
        );
        assert!(
            prepare_top_up_isolated_only_margin(PrepareTopUpIsolatedOnlyMarginRequest {
                network: Network::Mainnet,
                asset: AssetId::new(4),
                leverage: "0".to_owned(),
                nonce: 2_002,
                vault_address: None,
                expires_after: None,
            })
            .is_err()
        );
    }

    #[test]
    fn prepares_batch_modify_with_oid_and_cloid_selectors() {
        let prepared = prepare_batch_modify(PrepareBatchModifyRequest {
            network: Network::Mainnet,
            modifies: vec![
                ModifyOrderInput {
                    oid: OrderSelectorInput::Oid(42),
                    order: order_input("100"),
                },
                ModifyOrderInput {
                    oid: OrderSelectorInput::Cloid("0x1234567890abcdef1234567890abcdef".to_owned()),
                    order: order_input("101"),
                },
            ],
            nonce: 1_000,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        let action: Value = serde_json::from_str(&prepared.action_json).unwrap();
        assert_eq!(action["type"], "batchModify");
        assert_eq!(action["modifies"][0]["oid"], 42);
        assert_eq!(
            action["modifies"][1]["oid"],
            "0x1234567890abcdef1234567890abcdef"
        );
    }

    #[test]
    fn matches_independent_python_cancel_and_modify_vectors() {
        let nonce = 1_234_567_890_000;
        let vector = l1_action_vector();
        let cancel = prepare_cancel(PrepareCancelRequest {
            network: Network::Mainnet,
            cancels: vec![CancelInput {
                asset: AssetId::new(3),
                oid: 42,
            }],
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&cancel, &vector["cancel"]);

        let by_cloid = prepare_cancel_by_cloid(PrepareCancelByCloidRequest {
            network: Network::Mainnet,
            cancels: vec![CancelByCloidInput {
                asset: AssetId::new(3),
                cloid: "0x1234567890abcdef1234567890abcdef".to_owned(),
            }],
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&by_cloid, &vector["cancelByCloid"]);

        let modify = prepare_batch_modify(PrepareBatchModifyRequest {
            network: Network::Mainnet,
            modifies: vec![ModifyOrderInput {
                oid: OrderSelectorInput::Oid(42),
                order: order_input("100"),
            }],
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&modify, &vector["batchModify"]);
    }

    #[test]
    fn cross_language_json_rejects_numeric_u64_values() {
        let invalid = serde_json::json!({
            "network": "mainnet",
            "cancels": [{"asset": 3, "oid": 42}],
            "nonce": 1000,
            "vaultAddress": null,
            "expiresAfter": null
        });
        assert!(serde_json::from_value::<PrepareCancelRequest>(invalid).is_err());

        let valid = serde_json::json!({
            "network": "mainnet",
            "cancels": [{"asset": 3, "oid": "42"}],
            "nonce": "1000",
            "vaultAddress": null,
            "expiresAfter": null
        });
        assert!(serde_json::from_value::<PrepareCancelRequest>(valid).is_ok());
    }

    #[test]
    fn matches_independent_python_advanced_action_vectors() {
        let nonce = 1_234_567_890_000;
        let vector = advanced_action_vector();
        let trigger = prepare_orders(PrepareOrdersRequest {
            network: Network::Mainnet,
            orders: vec![OrderInput::Trigger(TriggerOrderInput {
                asset: AssetId::new(3),
                market_kind: MarketKind::Perpetual,
                sz_decimals: 2,
                is_buy: false,
                limit_price: "95".to_owned(),
                trigger_price: "96".to_owned(),
                size: "0.5".to_owned(),
                reduce_only: true,
                is_market: true,
                trigger_kind: TriggerKind::Sl,
                client_order_id: None,
            })],
            grouping: Grouping::NormalTpsl,
            builder: None,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&trigger, &vector["triggerOrder"]);

        let scheduled = prepare_schedule_cancel(PrepareScheduleCancelRequest {
            network: Network::Mainnet,
            scheduled_time: Some(nonce + 10_000),
            now_ms: nonce,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&scheduled, &vector["scheduleCancel"]);

        let cleared = prepare_schedule_cancel(PrepareScheduleCancelRequest {
            network: Network::Mainnet,
            scheduled_time: None,
            now_ms: nonce,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&cleared, &vector["scheduleCancelClear"]);

        let twap = prepare_twap_order(PrepareTwapOrderRequest {
            network: Network::Mainnet,
            asset: AssetId::new(3),
            market_kind: MarketKind::Perpetual,
            sz_decimals: 2,
            is_buy: true,
            size: "1.25".to_owned(),
            reduce_only: false,
            minutes: 5,
            randomize: true,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&twap, &vector["twapOrder"]);

        let twap_cancel = prepare_twap_cancel(PrepareTwapCancelRequest {
            network: Network::Mainnet,
            asset: AssetId::new(3),
            twap_id: 77,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&twap_cancel, &vector["twapCancel"]);
    }

    #[test]
    fn matches_independent_python_risk_action_vectors() {
        let nonce = 1_234_567_890_000;
        let vector = risk_action_vector();

        let leverage = prepare_update_leverage(PrepareUpdateLeverageRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            is_cross: false,
            leverage: 12,
            max_leverage: 50,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&leverage, &vector["updateLeverage"]);

        let margin = prepare_update_isolated_margin(PrepareUpdateIsolatedMarginRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            is_buy: true,
            usd_delta: "-1.25".to_owned(),
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&margin, &vector["updateIsolatedMargin"]);

        let top_up = prepare_top_up_isolated_only_margin(PrepareTopUpIsolatedOnlyMarginRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            leverage: "3.5".to_owned(),
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&top_up, &vector["topUpIsolatedOnlyMargin"]);
    }

    #[test]
    fn matches_and_finalizes_independent_user_action_vectors() {
        let vector = user_action_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();

        let usd = prepare_usd_send(PrepareUsdSendRequest {
            network: Network::Mainnet,
            destination: "0x000000000000000000000000000000000000dEaD".to_owned(),
            amount: "12.500".to_owned(),
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&usd, &vector["usdSend"]);
        let usd_finalized = finalize_usd_send(
            usd.clone(),
            user_vector_signature(&vector["usdSend"]),
            expected,
        )
        .unwrap();
        let usd_body: Value = serde_json::from_str(&usd_finalized.request_json).unwrap();
        assert_eq!(usd_body["action"], vector["usdSend"]["action"]);
        assert_eq!(usd_body["nonce"], nonce);
        assert!(usd_body.get("vaultAddress").is_none());
        assert!(usd_body.get("expiresAfter").is_none());

        let spot = prepare_spot_send(PrepareSpotSendRequest {
            network: Network::Mainnet,
            destination: "0x000000000000000000000000000000000000dead".to_owned(),
            token: "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            amount: "0.01".to_owned(),
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&spot, &vector["spotSend"]);
        assert!(
            finalize_spot_send(spot, user_vector_signature(&vector["spotSend"]), expected,).is_ok()
        );

        let withdrawal = prepare_withdraw(PrepareWithdrawRequest {
            network: Network::Mainnet,
            destination: "0x000000000000000000000000000000000000dead".to_owned(),
            amount: "3.25".to_owned(),
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&withdrawal, &vector["withdraw"]);
        assert!(
            finalize_withdraw(
                withdrawal,
                user_vector_signature(&vector["withdraw"]),
                expected,
            )
            .is_ok()
        );

        let mut mismatched = usd;
        mismatched.network = Network::Testnet;
        assert!(matches!(
            finalize_usd_send(
                mismatched,
                user_vector_signature(&vector["usdSend"]),
                expected,
            ),
            Err(FinalizeError::ArtifactMismatch("userActionContext"))
        ));
    }

    #[test]
    fn rejects_invalid_user_action_inputs() {
        assert!(
            prepare_usd_send(PrepareUsdSendRequest {
                network: Network::Mainnet,
                destination: "0x000000000000000000000000000000000000dead".to_owned(),
                amount: "0".to_owned(),
                nonce: 1_000,
            })
            .is_err()
        );
        assert!(
            prepare_spot_send(PrepareSpotSendRequest {
                network: Network::Mainnet,
                destination: "0x000000000000000000000000000000000000dead".to_owned(),
                token: "PURR".to_owned(),
                amount: "1".to_owned(),
                nonce: 1_000,
            })
            .is_err()
        );
    }

    #[test]
    fn matches_and_finalizes_independent_authorization_vectors() {
        let vector = user_authorization_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();

        let agent = prepare_approve_agent(PrepareApproveAgentRequest {
            network: Network::Mainnet,
            agent_address: "0x000000000000000000000000000000000000beef".to_owned(),
            agent_name: Some("mobile-primary".to_owned()),
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&agent, &vector["approveAgent"]);
        let finalized = finalize_approve_agent(
            agent,
            user_vector_signature(&vector["approveAgent"]),
            expected,
        )
        .unwrap();
        let body: Value = serde_json::from_str(&finalized.request_json).unwrap();
        assert_eq!(body["action"], vector["approveAgent"]["action"]);
        assert_eq!(body["nonce"], nonce);

        let unnamed = prepare_approve_agent(PrepareApproveAgentRequest {
            network: Network::Mainnet,
            agent_address: "0x000000000000000000000000000000000000beef".to_owned(),
            agent_name: None,
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&unnamed, &vector["approveUnnamedAgent"]);
        assert!(
            finalize_approve_agent(
                unnamed,
                user_vector_signature(&vector["approveUnnamedAgent"]),
                expected,
            )
            .is_ok()
        );

        let builder = prepare_approve_builder_fee(PrepareApproveBuilderFeeRequest {
            network: Network::Mainnet,
            builder: "0x000000000000000000000000000000000000b0b0".to_owned(),
            max_fee_rate: "0.0010%".to_owned(),
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&builder, &vector["approveBuilderFee"]);
        assert!(
            finalize_approve_builder_fee(
                builder,
                user_vector_signature(&vector["approveBuilderFee"]),
                expected,
            )
            .is_ok()
        );

        assert!(
            prepare_approve_builder_fee(PrepareApproveBuilderFeeRequest {
                network: Network::Mainnet,
                builder: "0x000000000000000000000000000000000000b0b0".to_owned(),
                max_fee_rate: "-0.001%".to_owned(),
                nonce,
            })
            .is_err()
        );
    }

    #[test]
    fn matches_and_finalizes_independent_account_transfer_vectors() {
        let vector = user_account_transfer_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();
        let sub_account = "0x000000000000000000000000000000000000cafe";

        let class_transfer = prepare_usd_class_transfer(PrepareUsdClassTransferRequest {
            network: Network::Mainnet,
            amount: "12.500".to_owned(),
            to_perp: true,
            sub_account: Some(sub_account.to_owned()),
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&class_transfer, &vector["usdClassTransfer"]);
        let class_transfer: super::PreparedUsdClassTransfer =
            serde_json::from_str(&serde_json::to_string(&class_transfer).unwrap()).unwrap();
        assert!(
            finalize_usd_class_transfer(
                class_transfer,
                user_vector_signature(&vector["usdClassTransfer"]),
                expected,
            )
            .is_ok()
        );

        let send = prepare_send_asset(PrepareSendAssetRequest {
            network: Network::Mainnet,
            destination: "0x000000000000000000000000000000000000dead".to_owned(),
            source_dex: "spot".to_owned(),
            destination_dex: "spot".to_owned(),
            token: "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            amount: "0.010".to_owned(),
            from_sub_account: Some(sub_account.to_owned()),
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&send, &vector["sendAsset"]);
        let send: super::PreparedSendAsset =
            serde_json::from_str(&serde_json::to_string(&send).unwrap()).unwrap();
        assert!(
            finalize_send_asset(send, user_vector_signature(&vector["sendAsset"]), expected)
                .is_ok()
        );

        let main_account = prepare_send_asset(PrepareSendAssetRequest {
            network: Network::Testnet,
            destination: "0x000000000000000000000000000000000000dead".to_owned(),
            source_dex: String::new(),
            destination_dex: "spot".to_owned(),
            token: "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            amount: "1".to_owned(),
            from_sub_account: None,
            nonce,
        })
        .unwrap();
        let action: Value = serde_json::from_str(&main_account.action_json).unwrap();
        assert_eq!(action["fromSubAccount"], "");

        assert!(
            prepare_usd_class_transfer(PrepareUsdClassTransferRequest {
                network: Network::Mainnet,
                amount: format!("1 subaccount:{sub_account}"),
                to_perp: true,
                sub_account: None,
                nonce,
            })
            .is_err()
        );
    }

    #[test]
    fn matches_and_finalizes_independent_staking_vectors() {
        let vector = staking_action_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();

        let deposit = prepare_c_deposit(PrepareCDepositRequest {
            network: Network::Mainnet,
            wei: 100_000_000,
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&deposit, &vector["cDeposit"]);
        let deposit: super::PreparedCDeposit =
            serde_json::from_str(&serde_json::to_string(&deposit).unwrap()).unwrap();
        assert!(
            finalize_c_deposit(
                deposit,
                user_vector_signature(&vector["cDeposit"]),
                expected,
            )
            .is_ok()
        );

        let withdrawal = prepare_c_withdraw(PrepareCWithdrawRequest {
            network: Network::Mainnet,
            wei: 50_000_000,
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&withdrawal, &vector["cWithdraw"]);
        assert!(
            finalize_c_withdraw(
                withdrawal,
                user_vector_signature(&vector["cWithdraw"]),
                expected,
            )
            .is_ok()
        );

        let delegation = prepare_token_delegate(PrepareTokenDelegateRequest {
            network: Network::Mainnet,
            validator: "0x000000000000000000000000000000000000beef".to_owned(),
            wei: 25_000_000,
            is_undelegate: false,
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&delegation, &vector["tokenDelegate"]);
        assert!(
            finalize_token_delegate(
                delegation,
                user_vector_signature(&vector["tokenDelegate"]),
                expected,
            )
            .is_ok()
        );

        assert!(
            prepare_c_deposit(PrepareCDepositRequest {
                network: Network::Mainnet,
                wei: 0,
                nonce,
            })
            .is_err()
        );
        assert!(
            prepare_token_delegate(PrepareTokenDelegateRequest {
                network: Network::Mainnet,
                validator: "invalid".to_owned(),
                wei: 1,
                is_undelegate: true,
                nonce,
            })
            .is_err()
        );

        let dto = serde_json::json!({
            "network": "mainnet",
            "wei": "100000000",
            "nonce": "1234567890000"
        });
        assert!(serde_json::from_value::<PrepareCDepositRequest>(dto).is_ok());
        let numeric_dto = serde_json::json!({
            "network": "mainnet",
            "wei": 100_000_000,
            "nonce": "1234567890000"
        });
        assert!(serde_json::from_value::<PrepareCDepositRequest>(numeric_dto).is_err());
    }

    #[test]
    fn matches_and_finalizes_independent_account_l1_vectors() {
        let vector = account_l1_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();

        let create = prepare_create_sub_account(PrepareCreateSubAccountRequest {
            network: Network::Mainnet,
            name: "strategy-one".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&create, &vector["createSubAccount"]);
        assert!(
            finalize_create_sub_account(
                create,
                user_vector_signature(&vector["createSubAccount"]),
                expected,
            )
            .is_ok()
        );

        let usd = prepare_sub_account_transfer(PrepareSubAccountTransferRequest {
            network: Network::Mainnet,
            sub_account_user: "0x000000000000000000000000000000000000cafe".to_owned(),
            is_deposit: true,
            amount: "12.500000".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&usd, &vector["subAccountTransfer"]);
        assert!(usd.vault_address.is_none());
        assert!(
            finalize_sub_account_transfer(
                usd,
                user_vector_signature(&vector["subAccountTransfer"]),
                expected,
            )
            .is_ok()
        );

        let spot = prepare_sub_account_spot_transfer(PrepareSubAccountSpotTransferRequest {
            network: Network::Mainnet,
            sub_account_user: "0x000000000000000000000000000000000000cafe".to_owned(),
            is_deposit: false,
            token: "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            amount: "0.0100".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&spot, &vector["subAccountSpotTransfer"]);
        assert!(
            finalize_sub_account_spot_transfer(
                spot,
                user_vector_signature(&vector["subAccountSpotTransfer"]),
                expected,
            )
            .is_ok()
        );

        let vault = prepare_vault_transfer(PrepareVaultTransferRequest {
            network: Network::Mainnet,
            vault_address: "0x000000000000000000000000000000000000babe".to_owned(),
            is_deposit: false,
            amount: "2".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&vault, &vector["vaultTransfer"]);
        assert!(vault.vault_address.is_none());
        assert!(
            finalize_vault_transfer(
                vault,
                user_vector_signature(&vector["vaultTransfer"]),
                expected,
            )
            .is_ok()
        );

        assert!(
            prepare_sub_account_transfer(PrepareSubAccountTransferRequest {
                network: Network::Mainnet,
                sub_account_user: "0x000000000000000000000000000000000000cafe".to_owned(),
                is_deposit: true,
                amount: "0.0000001".to_owned(),
                nonce,
                expires_after: None,
            })
            .is_err()
        );
        assert!(
            prepare_create_sub_account(PrepareCreateSubAccountRequest {
                network: Network::Mainnet,
                name: String::new(),
                nonce,
                expires_after: None,
            })
            .is_err()
        );
    }

    #[test]
    fn matches_and_finalizes_independent_account_control_vectors() {
        let vector = account_control_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();
        let vault = "0x000000000000000000000000000000000000cafe".to_owned();

        let referrer = prepare_set_referrer(PrepareSetReferrerRequest {
            network: Network::Mainnet,
            code: "HYPERAXIS".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&referrer, &vector["setReferrer"]);
        assert!(referrer.vault_address.is_none());
        assert!(
            finalize_set_referrer(
                referrer,
                user_vector_signature(&vector["setReferrer"]),
                expected,
            )
            .is_ok()
        );

        let evm = prepare_evm_user_modify(PrepareEvmUserModifyRequest {
            network: Network::Mainnet,
            using_big_blocks: true,
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&evm, &vector["evmUserModify"]);
        assert!(
            finalize_evm_user_modify(
                evm,
                user_vector_signature(&vector["evmUserModify"]),
                expected,
            )
            .is_ok()
        );

        let legacy =
            prepare_agent_enable_dex_abstraction(PrepareAgentEnableDexAbstractionRequest {
                network: Network::Mainnet,
                vault_address: Some(vault.clone()),
                nonce,
                expires_after: None,
            })
            .unwrap();
        assert_prepared_matches_vector(&legacy, &vector["agentEnableDexAbstraction"]);
        assert!(
            finalize_agent_enable_dex_abstraction(
                legacy,
                user_vector_signature(&vector["agentEnableDexAbstraction"]),
                expected,
            )
            .is_ok()
        );

        let current = prepare_agent_set_abstraction(PrepareAgentSetAbstractionRequest {
            network: Network::Mainnet,
            abstraction: AgentAbstraction::PortfolioMargin,
            vault_address: Some(vault),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&current, &vector["agentSetAbstraction"]);
        assert!(
            finalize_agent_set_abstraction(
                current,
                user_vector_signature(&vector["agentSetAbstraction"]),
                expected,
            )
            .is_ok()
        );

        let noop = prepare_noop(PrepareNoopRequest {
            network: Network::Mainnet,
            vault_address: None,
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&noop, &vector["noop"]);
        assert!(finalize_noop(noop, user_vector_signature(&vector["noop"]), expected).is_ok());

        assert!(
            prepare_set_referrer(PrepareSetReferrerRequest {
                network: Network::Mainnet,
                code: String::new(),
                nonce,
                expires_after: None,
            })
            .is_err()
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn matches_and_finalizes_independent_validator_vectors() {
        let vector = validator_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();

        for (variant, key) in [
            (CSignerVariant::UnjailSelf, "cSignerUnjailSelf"),
            (CSignerVariant::JailSelf, "cSignerJailSelf"),
        ] {
            let prepared = prepare_c_signer(PrepareCSignerRequest {
                network: Network::Mainnet,
                variant,
                nonce,
                expires_after: None,
            })
            .unwrap();
            assert_prepared_matches_vector(&prepared, &vector[key]);
            assert_eq!(prepared.signing_digest_hex, vector[key]["signingDigestHex"]);
            assert!(
                finalize_c_signer(prepared, user_vector_signature(&vector[key]), expected).is_ok()
            );
        }

        let register = prepare_c_validator_register(PrepareCValidatorRegisterRequest {
            network: Network::Mainnet,
            node_ip: "192.0.2.10".to_owned(),
            name: "hyper-axis-validator".to_owned(),
            description: "production validator".to_owned(),
            delegations_disabled: false,
            commission_bps: 500,
            signer: "0x000000000000000000000000000000000000beef".to_owned(),
            unjailed: true,
            initial_wei: 25_000_000,
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&register, &vector["cValidatorRegister"]);
        assert!(
            finalize_c_validator(
                register,
                user_vector_signature(&vector["cValidatorRegister"]),
                expected,
            )
            .is_ok()
        );

        let change = prepare_c_validator_change_profile(PrepareCValidatorChangeProfileRequest {
            network: Network::Mainnet,
            node_ip: Some("2001:db8::10".to_owned()),
            name: Some("hyper-axis-validator-v2".to_owned()),
            description: None,
            unjailed: true,
            disable_delegations: Some(true),
            commission_bps: Some(450),
            signer: Some("0x000000000000000000000000000000000000cafe".to_owned()),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&change, &vector["cValidatorChangeProfile"]);
        assert!(
            finalize_c_validator(
                change,
                user_vector_signature(&vector["cValidatorChangeProfile"]),
                expected,
            )
            .is_ok()
        );

        let unregister = prepare_c_validator_unregister(PrepareCValidatorUnregisterRequest {
            network: Network::Mainnet,
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_prepared_matches_vector(&unregister, &vector["cValidatorUnregister"]);
        assert!(
            finalize_c_validator(
                unregister,
                user_vector_signature(&vector["cValidatorUnregister"]),
                expected,
            )
            .is_ok()
        );

        let gossip = prepare_gossip_priority_bid(PrepareGossipPriorityBidRequest {
            network: Network::Mainnet,
            slot_id: 7,
            ip: "198.51.100.42".to_owned(),
            max_gas: 900_000,
            vault_address: Some("0x000000000000000000000000000000000000cafe".to_owned()),
            nonce,
            expires_after: Some(nonce + 60_000),
        })
        .unwrap();
        assert_prepared_matches_vector(&gossip, &vector["gossipPriorityBid"]);
        assert_eq!(
            gossip.signing_digest_hex,
            vector["gossipPriorityBid"]["signingDigestHex"]
        );
        assert!(
            finalize_gossip_priority_bid(
                gossip,
                user_vector_signature(&vector["gossipPriorityBid"]),
                expected,
            )
            .is_ok()
        );

        assert!(
            prepare_c_validator_register(PrepareCValidatorRegisterRequest {
                network: Network::Mainnet,
                node_ip: "not-an-ip".to_owned(),
                name: "validator".to_owned(),
                description: String::new(),
                delegations_disabled: false,
                commission_bps: 10_001,
                signer: "0x000000000000000000000000000000000000beef".to_owned(),
                unjailed: true,
                initial_wei: 0,
                nonce,
                expires_after: None,
            })
            .is_err()
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn matches_and_finalizes_independent_deployer_vectors() {
        let vector = deployer_vector();
        let multi_vector = multi_sig_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();
        let multi_sig_user = multi_vector["multiSigUser"].as_str().unwrap();
        let outer_signer = multi_vector["outerSigner"].as_str().unwrap();

        macro_rules! assert_deployer_case {
            ($prepared:expr, $key:literal, $multi_key:literal, $finalize:path, $multi:path) => {{
                let prepared = $prepared.unwrap();
                assert_prepared_matches_vector(&prepared, &vector[$key]);
                assert_eq!(
                    prepared.signing_digest_hex,
                    vector[$key]["signingDigestHex"]
                );
                assert_multi_sig_payload_matches_vector(
                    $multi(prepared.clone(), multi_sig_user, outer_signer).unwrap(),
                    &multi_vector[$multi_key],
                );
                assert!(
                    $finalize(prepared, user_vector_signature(&vector[$key]), expected).is_ok()
                );
            }};
        }

        assert_deployer_case!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::RegisterToken {
                    token_name: "AXIS".to_owned(),
                    sz_decimals: 2,
                    wei_decimals: 8,
                    max_gas: 500_000,
                    full_name: "Hyper Axis".to_owned(),
                },
                nonce,
                expires_after: None,
            }),
            "spotRegisterToken",
            "l1SpotRegisterToken",
            finalize_spot_deploy,
            prepare_multi_sig_spot_deploy_payload
        );
        assert_deployer_case!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::UserGenesis {
                    token: 101,
                    user_and_wei: vec![
                        SpotUserWeiInput {
                            user: "0x000000000000000000000000000000000000beef".to_owned(),
                            wei: "25000000".to_owned(),
                        },
                        SpotUserWeiInput {
                            user: "0x000000000000000000000000000000000000cafe".to_owned(),
                            wei: "75000000".to_owned(),
                        },
                    ],
                    existing_token_and_wei: vec![
                        SpotExistingTokenWeiInput {
                            token: 0,
                            wei: "1000000".to_owned(),
                        },
                        SpotExistingTokenWeiInput {
                            token: 7,
                            wei: "2000000".to_owned(),
                        },
                    ],
                },
                nonce,
                expires_after: None,
            }),
            "spotUserGenesis",
            "l1SpotUserGenesis",
            finalize_spot_deploy,
            prepare_multi_sig_spot_deploy_payload
        );
        for (input, key, multi_key) in [
            (
                SpotDeployInput::EnableFreezePrivilege { token: 101 },
                "spotEnableFreezePrivilege",
                "l1SpotEnableFreezePrivilege",
            ),
            (
                SpotDeployInput::RevokeFreezePrivilege { token: 101 },
                "spotRevokeFreezePrivilege",
                "l1SpotRevokeFreezePrivilege",
            ),
            (
                SpotDeployInput::EnableQuoteToken { token: 101 },
                "spotEnableQuoteToken",
                "l1SpotEnableQuoteToken",
            ),
        ] {
            let prepared = prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input,
                nonce,
                expires_after: None,
            })
            .unwrap();
            assert_prepared_matches_vector(&prepared, &vector[key]);
            assert_multi_sig_payload_matches_vector(
                prepare_multi_sig_spot_deploy_payload(
                    prepared.clone(),
                    multi_sig_user,
                    outer_signer,
                )
                .unwrap(),
                &multi_vector[multi_key],
            );
            assert!(
                finalize_spot_deploy(prepared, user_vector_signature(&vector[key]), expected)
                    .is_ok()
            );
        }
        assert_deployer_case!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::FreezeUser {
                    token: 101,
                    user: "0x000000000000000000000000000000000000beef".to_owned(),
                    freeze: true,
                },
                nonce,
                expires_after: None,
            }),
            "spotFreezeUser",
            "l1SpotFreezeUser",
            finalize_spot_deploy,
            prepare_multi_sig_spot_deploy_payload
        );
        assert_deployer_case!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::Genesis {
                    token: 101,
                    max_supply: "1000000000000000".to_owned(),
                    no_hyperliquidity: true,
                },
                nonce,
                expires_after: None,
            }),
            "spotGenesis",
            "l1SpotGenesis",
            finalize_spot_deploy,
            prepare_multi_sig_spot_deploy_payload
        );
        assert_deployer_case!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::RegisterSpot {
                    base_token: 101,
                    quote_token: 0,
                },
                nonce,
                expires_after: None,
            }),
            "spotRegisterSpot",
            "l1SpotRegisterSpot",
            finalize_spot_deploy,
            prepare_multi_sig_spot_deploy_payload
        );
        assert_deployer_case!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::RegisterHyperliquidity {
                    spot: 12,
                    start_px: "1.2500".to_owned(),
                    order_sz: "10.0".to_owned(),
                    n_orders: 20,
                    n_seeded_levels: Some(5),
                },
                nonce,
                expires_after: None,
            }),
            "spotRegisterHyperliquidity",
            "l1SpotRegisterHyperliquidity",
            finalize_spot_deploy,
            prepare_multi_sig_spot_deploy_payload
        );
        assert_deployer_case!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::SetDeployerTradingFeeShare {
                    token: 101,
                    share: "0.100".to_owned(),
                },
                nonce,
                expires_after: None,
            }),
            "spotSetDeployerTradingFeeShare",
            "l1SpotSetDeployerTradingFeeShare",
            finalize_spot_deploy,
            prepare_multi_sig_spot_deploy_payload
        );
        assert_deployer_case!(
            prepare_perp_deploy(PreparePerpDeployRequest {
                network: Network::Mainnet,
                input: PerpDeployInput::RegisterAsset {
                    dex: "axis".to_owned(),
                    max_gas: Some(700_000),
                    coin: "AXIS".to_owned(),
                    sz_decimals: 3,
                    oracle_px: "12.500".to_owned(),
                    margin_table_id: 7,
                    only_isolated: true,
                    schema: Some(PerpDexSchemaInput {
                        full_name: "Hyper Axis Perps".to_owned(),
                        collateral_token: 0,
                        oracle_updater: Some(
                            "0x000000000000000000000000000000000000beef".to_owned(),
                        ),
                    }),
                },
                nonce,
                expires_after: None,
            }),
            "perpRegisterAsset",
            "l1PerpRegisterAsset",
            finalize_perp_deploy,
            prepare_multi_sig_perp_deploy_payload
        );
        let prices = |axis: &str, beta: &str| {
            vec![
                PerpPriceInput {
                    coin: "BETA".to_owned(),
                    price: beta.to_owned(),
                },
                PerpPriceInput {
                    coin: "AXIS".to_owned(),
                    price: axis.to_owned(),
                },
            ]
        };
        assert_deployer_case!(
            prepare_perp_deploy(PreparePerpDeployRequest {
                network: Network::Mainnet,
                input: PerpDeployInput::SetOracle {
                    dex: "axis".to_owned(),
                    oracle_pxs: prices("12.5", "3.25"),
                    mark_pxs: vec![prices("12.4", "3.2"), prices("12.6", "3.3")],
                    external_perp_pxs: prices("12.45", "3.22"),
                },
                nonce,
                expires_after: None,
            }),
            "perpSetOracle",
            "l1PerpSetOracle",
            finalize_perp_deploy,
            prepare_multi_sig_perp_deploy_payload
        );

        assert!(
            prepare_spot_deploy(PrepareSpotDeployRequest {
                network: Network::Mainnet,
                input: SpotDeployInput::RegisterSpot {
                    base_token: 101,
                    quote_token: 101,
                },
                nonce,
                expires_after: None,
            })
            .is_err()
        );
        assert!(
            prepare_perp_deploy(PreparePerpDeployRequest {
                network: Network::Mainnet,
                input: PerpDeployInput::SetOracle {
                    dex: "axis".to_owned(),
                    oracle_pxs: vec![
                        PerpPriceInput {
                            coin: "AXIS".to_owned(),
                            price: "1".to_owned(),
                        },
                        PerpPriceInput {
                            coin: "AXIS".to_owned(),
                            price: "2".to_owned(),
                        },
                    ],
                    mark_pxs: Vec::new(),
                    external_perp_pxs: Vec::new(),
                },
                nonce,
                expires_after: None,
            })
            .is_err()
        );
    }

    #[test]
    fn matches_and_finalizes_independent_account_security_vectors() {
        let vector = account_security_vector();
        let nonce = 1_234_567_890_000;
        let expected: EvmAddress = vector["expectedSigner"].as_str().unwrap().parse().unwrap();
        let user = "0x000000000000000000000000000000000000cafe";

        let legacy = prepare_user_dex_abstraction(PrepareUserDexAbstractionRequest {
            network: Network::Mainnet,
            user: user.to_owned(),
            enabled: true,
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&legacy, &vector["userDexAbstraction"]);
        assert!(
            finalize_user_dex_abstraction(
                legacy,
                user_vector_signature(&vector["userDexAbstraction"]),
                expected,
            )
            .is_ok()
        );

        let current = prepare_user_set_abstraction(PrepareUserSetAbstractionRequest {
            network: Network::Mainnet,
            user: user.to_owned(),
            abstraction: UserAbstraction::PortfolioMargin,
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&current, &vector["userSetAbstraction"]);
        assert!(
            finalize_user_set_abstraction(
                current,
                user_vector_signature(&vector["userSetAbstraction"]),
                expected,
            )
            .is_ok()
        );

        let multi_sig = prepare_convert_to_multi_sig_user(PrepareConvertToMultiSigUserRequest {
            network: Network::Mainnet,
            authorized_users: vec![
                user.to_owned(),
                "0x000000000000000000000000000000000000beef".to_owned(),
            ],
            threshold: 2,
            nonce,
        })
        .unwrap();
        assert_user_prepared_matches_vector(&multi_sig, &vector["convertToMultiSigUser"]);
        let multi_sig: super::PreparedConvertToMultiSigUser =
            serde_json::from_str(&serde_json::to_string(&multi_sig).unwrap()).unwrap();
        assert!(
            finalize_convert_to_multi_sig_user(
                multi_sig,
                user_vector_signature(&vector["convertToMultiSigUser"]),
                expected,
            )
            .is_ok()
        );

        for (users, threshold) in [
            (Vec::new(), 1),
            (vec![user.to_owned()], 0),
            (vec![user.to_owned()], 2),
            (vec![user.to_owned(), user.to_owned()], 1),
        ] {
            assert!(
                prepare_convert_to_multi_sig_user(PrepareConvertToMultiSigUserRequest {
                    network: Network::Mainnet,
                    authorized_users: users,
                    threshold,
                    nonce,
                })
                .is_err()
            );
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn matches_and_finalizes_independent_multi_signature_vectors() {
        let vector = multi_sig_vector();
        let nonce = 1_234_567_890_000;
        let multi_sig_user = vector["multiSigUser"].as_str().unwrap().to_owned();
        let outer_signer = vector["outerSigner"].as_str().unwrap().to_owned();
        let expected_outer: EvmAddress = outer_signer.parse().unwrap();

        let order = prepare_limit_order(PrepareLimitOrderRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            market_kind: MarketKind::Perpetual,
            sz_decimals: 1,
            is_buy: true,
            limit_price: "1100".to_owned(),
            size: "0.2".to_owned(),
            reduce_only: false,
            time_in_force: TimeInForce::Gtc,
            client_order_id: None,
            grouping: Grouping::None,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        let order_payload =
            prepare_multi_sig_order_payload(order, &multi_sig_user, &outer_signer).unwrap();
        assert_eq!(
            order_payload.signing_digest_hex,
            vector["l1Order"]["innerSigningDigestHexes"][0]
        );
        let order_envelope = prepare_multi_sig_envelope(
            order_payload,
            multi_sig_signature_inputs(&vector["l1Order"]),
        )
        .unwrap();
        assert_multi_sig_envelope_matches_vector(&order_envelope, &vector["l1Order"]);
        let order_envelope: super::PreparedMultiSigEnvelope =
            serde_json::from_str(&serde_json::to_string(&order_envelope).unwrap()).unwrap();
        let finalized = finalize_multi_sig(
            &order_envelope,
            raw_vector_signature(&vector["l1Order"]["outerSignature"]),
            expected_outer,
        )
        .unwrap();
        let request: Value = serde_json::from_str(&finalized.request_json).unwrap();
        assert_eq!(request["action"], vector["l1Order"]["outerAction"]);
        assert!(request["vaultAddress"].is_null());

        let send_asset = prepare_send_asset(PrepareSendAssetRequest {
            network: Network::Mainnet,
            destination: "0x000000000000000000000000000000000000dead".to_owned(),
            source_dex: String::new(),
            destination_dex: "spot".to_owned(),
            token: "USDC".to_owned(),
            amount: "12.5".to_owned(),
            from_sub_account: None,
            nonce,
        })
        .unwrap();
        let send_payload =
            prepare_multi_sig_send_asset_payload(send_asset, &multi_sig_user, &outer_signer)
                .unwrap();
        assert_eq!(
            send_payload.signing_digest_hex,
            vector["sendAsset"]["innerSigningDigestHexes"][0]
        );
        let send_envelope = prepare_multi_sig_envelope(
            send_payload,
            multi_sig_signature_inputs(&vector["sendAsset"]),
        )
        .unwrap();
        assert_multi_sig_envelope_matches_vector(&send_envelope, &vector["sendAsset"]);
        assert!(
            finalize_multi_sig(
                &send_envelope,
                raw_vector_signature(&vector["sendAsset"]["outerSignature"]),
                expected_outer,
            )
            .is_ok()
        );

        let abstraction = prepare_user_set_abstraction(PrepareUserSetAbstractionRequest {
            network: Network::Mainnet,
            user: multi_sig_user.clone(),
            abstraction: UserAbstraction::PortfolioMargin,
            nonce,
        })
        .unwrap();
        let abstraction_payload = prepare_multi_sig_user_set_abstraction_payload(
            abstraction,
            &multi_sig_user,
            &outer_signer,
        )
        .unwrap();
        assert_eq!(
            abstraction_payload.signing_digest_hex,
            vector["userSetAbstraction"]["innerSigningDigestHexes"][0]
        );
        let inputs = multi_sig_signature_inputs(&vector["userSetAbstraction"]);
        let abstraction_envelope =
            prepare_multi_sig_envelope(abstraction_payload.clone(), inputs.clone()).unwrap();
        assert_multi_sig_envelope_matches_vector(
            &abstraction_envelope,
            &vector["userSetAbstraction"],
        );
        assert_eq!(
            serde_json::from_str::<Value>(&abstraction_envelope.action_json).unwrap()["payload"]["action"]
                ["abstraction"],
            "p"
        );
        assert!(
            finalize_multi_sig(
                &abstraction_envelope,
                raw_vector_signature(&vector["userSetAbstraction"]["outerSignature"]),
                expected_outer,
            )
            .is_ok()
        );

        let duplicate = vec![inputs[0].clone(), inputs[0].clone()];
        assert!(prepare_multi_sig_envelope(abstraction_payload, duplicate).is_err());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn matches_expanded_multi_signature_l1_vectors() {
        let vector = multi_sig_vector();
        let nonce = 1_234_567_890_000;
        let multi_sig_user = vector["multiSigUser"].as_str().unwrap();
        let outer_signer = vector["outerSigner"].as_str().unwrap();

        let cancel = prepare_cancel(PrepareCancelRequest {
            network: Network::Mainnet,
            cancels: vec![CancelInput {
                asset: AssetId::new(3),
                oid: 42,
            }],
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_cancel_payload(cancel, multi_sig_user, outer_signer).unwrap(),
            &vector["l1Cancel"],
        );

        let cancel_by_cloid = prepare_cancel_by_cloid(PrepareCancelByCloidRequest {
            network: Network::Mainnet,
            cancels: vec![CancelByCloidInput {
                asset: AssetId::new(3),
                cloid: "0x1234567890abcdef1234567890abcdef".to_owned(),
            }],
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_cancel_by_cloid_payload(
                cancel_by_cloid,
                multi_sig_user,
                outer_signer,
            )
            .unwrap(),
            &vector["l1CancelByCloid"],
        );

        let modify = prepare_batch_modify(PrepareBatchModifyRequest {
            network: Network::Mainnet,
            modifies: vec![ModifyOrderInput {
                oid: OrderSelectorInput::Oid(42),
                order: order_input("100"),
            }],
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_batch_modify_payload(modify, multi_sig_user, outer_signer).unwrap(),
            &vector["l1BatchModify"],
        );

        let scheduled = prepare_schedule_cancel(PrepareScheduleCancelRequest {
            network: Network::Mainnet,
            scheduled_time: Some(nonce + 10_000),
            now_ms: nonce,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_schedule_cancel_payload(scheduled, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1ScheduleCancel"],
        );

        let twap = prepare_twap_order(PrepareTwapOrderRequest {
            network: Network::Mainnet,
            asset: AssetId::new(3),
            market_kind: MarketKind::Perpetual,
            sz_decimals: 2,
            is_buy: true,
            size: "1.25".to_owned(),
            reduce_only: false,
            minutes: 5,
            randomize: true,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_twap_order_payload(twap, multi_sig_user, outer_signer).unwrap(),
            &vector["l1TwapOrder"],
        );

        let twap_cancel = prepare_twap_cancel(PrepareTwapCancelRequest {
            network: Network::Mainnet,
            asset: AssetId::new(3),
            twap_id: 77,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_twap_cancel_payload(twap_cancel, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1TwapCancel"],
        );

        let leverage = prepare_update_leverage(PrepareUpdateLeverageRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            is_cross: false,
            leverage: 12,
            max_leverage: 50,
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_update_leverage_payload(leverage, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1UpdateLeverage"],
        );

        let margin = prepare_update_isolated_margin(PrepareUpdateIsolatedMarginRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            is_buy: true,
            usd_delta: "-1.25".to_owned(),
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_update_isolated_margin_payload(margin, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1UpdateIsolatedMargin"],
        );

        let top_up = prepare_top_up_isolated_only_margin(PrepareTopUpIsolatedOnlyMarginRequest {
            network: Network::Mainnet,
            asset: AssetId::new(4),
            leverage: "3.5".to_owned(),
            nonce,
            vault_address: None,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_top_up_isolated_only_margin_payload(
                top_up,
                multi_sig_user,
                outer_signer,
            )
            .unwrap(),
            &vector["l1TopUpIsolatedOnlyMargin"],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn matches_multi_signature_account_l1_vectors() {
        let vector = multi_sig_vector();
        let nonce = 1_234_567_890_000;
        let multi_sig_user = vector["multiSigUser"].as_str().unwrap();
        let outer_signer = vector["outerSigner"].as_str().unwrap();
        let subaccount = "0x000000000000000000000000000000000000cafe";

        let create = prepare_create_sub_account(PrepareCreateSubAccountRequest {
            network: Network::Mainnet,
            name: "strategy-one".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_create_sub_account_payload(create, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1CreateSubAccount"],
        );

        let usd = prepare_sub_account_transfer(PrepareSubAccountTransferRequest {
            network: Network::Mainnet,
            sub_account_user: subaccount.to_owned(),
            is_deposit: true,
            amount: "12.500000".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_sub_account_transfer_payload(usd, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1SubAccountTransfer"],
        );

        let spot = prepare_sub_account_spot_transfer(PrepareSubAccountSpotTransferRequest {
            network: Network::Mainnet,
            sub_account_user: subaccount.to_owned(),
            is_deposit: false,
            token: "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            amount: "0.0100".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_sub_account_spot_transfer_payload(spot, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1SubAccountSpotTransfer"],
        );

        let vault = prepare_vault_transfer(PrepareVaultTransferRequest {
            network: Network::Mainnet,
            vault_address: "0x000000000000000000000000000000000000babe".to_owned(),
            is_deposit: false,
            amount: "2".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_vault_transfer_payload(vault, multi_sig_user, outer_signer).unwrap(),
            &vector["l1VaultTransfer"],
        );

        let referrer = prepare_set_referrer(PrepareSetReferrerRequest {
            network: Network::Mainnet,
            code: "HYPERAXIS".to_owned(),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_set_referrer_payload(referrer, multi_sig_user, outer_signer).unwrap(),
            &vector["l1SetReferrer"],
        );

        let evm = prepare_evm_user_modify(PrepareEvmUserModifyRequest {
            network: Network::Mainnet,
            using_big_blocks: true,
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_evm_user_modify_payload(evm, multi_sig_user, outer_signer).unwrap(),
            &vector["l1EvmUserModify"],
        );

        let agent_enable =
            prepare_agent_enable_dex_abstraction(PrepareAgentEnableDexAbstractionRequest {
                network: Network::Mainnet,
                vault_address: Some(subaccount.to_owned()),
                nonce,
                expires_after: None,
            })
            .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_agent_enable_dex_abstraction_payload(
                agent_enable,
                multi_sig_user,
                outer_signer,
            )
            .unwrap(),
            &vector["l1AgentEnableDexAbstraction"],
        );

        let agent_set = prepare_agent_set_abstraction(PrepareAgentSetAbstractionRequest {
            network: Network::Mainnet,
            abstraction: AgentAbstraction::PortfolioMargin,
            vault_address: Some(subaccount.to_owned()),
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_agent_set_abstraction_payload(
                agent_set,
                multi_sig_user,
                outer_signer,
            )
            .unwrap(),
            &vector["l1AgentSetAbstraction"],
        );

        let noop = prepare_noop(PrepareNoopRequest {
            network: Network::Mainnet,
            vault_address: Some(subaccount.to_owned()),
            nonce,
            expires_after: Some(nonce + 60_000),
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_noop_payload(noop, multi_sig_user, outer_signer).unwrap(),
            &vector["l1Noop"],
        );

        for (variant, key) in [
            (CSignerVariant::UnjailSelf, "l1CSignerUnjailSelf"),
            (CSignerVariant::JailSelf, "l1CSignerJailSelf"),
        ] {
            let signer_action = prepare_c_signer(PrepareCSignerRequest {
                network: Network::Mainnet,
                variant,
                nonce,
                expires_after: None,
            })
            .unwrap();
            assert_multi_sig_payload_matches_vector(
                prepare_multi_sig_c_signer_payload(signer_action, multi_sig_user, outer_signer)
                    .unwrap(),
                &vector[key],
            );
        }

        let validator_register = prepare_c_validator_register(PrepareCValidatorRegisterRequest {
            network: Network::Mainnet,
            node_ip: "192.0.2.10".to_owned(),
            name: "hyper-axis-validator".to_owned(),
            description: "production validator".to_owned(),
            delegations_disabled: false,
            commission_bps: 500,
            signer: "0x000000000000000000000000000000000000beef".to_owned(),
            unjailed: true,
            initial_wei: 25_000_000,
            nonce,
            expires_after: None,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_c_validator_payload(validator_register, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1CValidatorRegister"],
        );

        let validator_change =
            prepare_c_validator_change_profile(PrepareCValidatorChangeProfileRequest {
                network: Network::Mainnet,
                node_ip: Some("2001:db8::10".to_owned()),
                name: Some("hyper-axis-validator-v2".to_owned()),
                description: None,
                unjailed: true,
                disable_delegations: Some(true),
                commission_bps: Some(450),
                signer: Some(subaccount.to_owned()),
                nonce,
                expires_after: None,
            })
            .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_c_validator_payload(validator_change, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1CValidatorChangeProfile"],
        );

        let validator_unregister =
            prepare_c_validator_unregister(PrepareCValidatorUnregisterRequest {
                network: Network::Mainnet,
                nonce,
                expires_after: None,
            })
            .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_c_validator_payload(
                validator_unregister,
                multi_sig_user,
                outer_signer,
            )
            .unwrap(),
            &vector["l1CValidatorUnregister"],
        );

        let gossip = prepare_gossip_priority_bid(PrepareGossipPriorityBidRequest {
            network: Network::Mainnet,
            slot_id: 7,
            ip: "198.51.100.42".to_owned(),
            max_gas: 900_000,
            vault_address: Some(subaccount.to_owned()),
            nonce,
            expires_after: Some(nonce + 60_000),
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_gossip_priority_bid_payload(gossip, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["l1GossipPriorityBid"],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn matches_expanded_multi_signature_user_signed_vectors() {
        let vector = multi_sig_vector();
        let nonce = 1_234_567_890_000;
        let multi_sig_user = vector["multiSigUser"].as_str().unwrap();
        let outer_signer = vector["outerSigner"].as_str().unwrap();
        let destination = "0x000000000000000000000000000000000000dead";

        let usd = prepare_usd_send(PrepareUsdSendRequest {
            network: Network::Mainnet,
            destination: destination.to_owned(),
            amount: "12.5".to_owned(),
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_usd_send_payload(usd, multi_sig_user, outer_signer).unwrap(),
            &vector["usdSend"],
        );

        let spot = prepare_spot_send(PrepareSpotSendRequest {
            network: Network::Mainnet,
            destination: destination.to_owned(),
            token: "PURR:0xc4bf3f870c0e9465323c0b6ed28096c2".to_owned(),
            amount: "0.01".to_owned(),
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_spot_send_payload(spot, multi_sig_user, outer_signer).unwrap(),
            &vector["spotSend"],
        );

        let withdrawal = prepare_withdraw(PrepareWithdrawRequest {
            network: Network::Mainnet,
            destination: destination.to_owned(),
            amount: "3.25".to_owned(),
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_withdraw_payload(withdrawal, multi_sig_user, outer_signer).unwrap(),
            &vector["withdraw"],
        );

        let agent = prepare_approve_agent(PrepareApproveAgentRequest {
            network: Network::Mainnet,
            agent_address: "0x000000000000000000000000000000000000beef".to_owned(),
            agent_name: Some("mobile-primary".to_owned()),
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_approve_agent_payload(agent, multi_sig_user, outer_signer).unwrap(),
            &vector["approveAgent"],
        );

        let builder = prepare_approve_builder_fee(PrepareApproveBuilderFeeRequest {
            network: Network::Mainnet,
            builder: "0x000000000000000000000000000000000000b0b0".to_owned(),
            max_fee_rate: "0.001%".to_owned(),
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_approve_builder_fee_payload(builder, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["approveBuilderFee"],
        );

        let class_transfer = prepare_usd_class_transfer(PrepareUsdClassTransferRequest {
            network: Network::Mainnet,
            amount: "12.5".to_owned(),
            to_perp: true,
            sub_account: Some("0x000000000000000000000000000000000000cafe".to_owned()),
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_usd_class_transfer_payload(
                class_transfer,
                multi_sig_user,
                outer_signer,
            )
            .unwrap(),
            &vector["usdClassTransfer"],
        );

        let deposit = prepare_c_deposit(PrepareCDepositRequest {
            network: Network::Mainnet,
            wei: 100_000_000,
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_c_deposit_payload(deposit, multi_sig_user, outer_signer).unwrap(),
            &vector["cDeposit"],
        );

        let staking_withdrawal = prepare_c_withdraw(PrepareCWithdrawRequest {
            network: Network::Mainnet,
            wei: 50_000_000,
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_c_withdraw_payload(staking_withdrawal, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["cWithdraw"],
        );

        let delegation = prepare_token_delegate(PrepareTokenDelegateRequest {
            network: Network::Mainnet,
            validator: "0x000000000000000000000000000000000000beef".to_owned(),
            wei: 25_000_000,
            is_undelegate: false,
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_token_delegate_payload(delegation, multi_sig_user, outer_signer)
                .unwrap(),
            &vector["tokenDelegate"],
        );

        let dex_abstraction = prepare_user_dex_abstraction(PrepareUserDexAbstractionRequest {
            network: Network::Mainnet,
            user: "0x000000000000000000000000000000000000cafe".to_owned(),
            enabled: true,
            nonce,
        })
        .unwrap();
        assert_multi_sig_payload_matches_vector(
            prepare_multi_sig_user_dex_abstraction_payload(
                dex_abstraction,
                multi_sig_user,
                outer_signer,
            )
            .unwrap(),
            &vector["userDexAbstraction"],
        );
    }

    #[test]
    fn schedule_cancel_enforces_five_second_preflight() {
        let request = PrepareScheduleCancelRequest {
            network: Network::Mainnet,
            scheduled_time: Some(14_999),
            now_ms: 10_000,
            nonce: 10_000,
            vault_address: None,
            expires_after: None,
        };
        assert!(prepare_schedule_cancel(request).is_err());
    }

    fn python_vector_signature() -> WireSignature {
        let vector = python_vector();
        WireSignature::new(
            vector["signature"]["r"].as_str().unwrap(),
            vector["signature"]["s"].as_str().unwrap(),
            u8::try_from(vector["signature"]["v"].as_u64().unwrap()).unwrap(),
        )
        .unwrap()
    }

    fn python_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/action_hash.json")).unwrap()
    }

    fn l1_action_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/l1_actions.json")).unwrap()
    }

    fn advanced_action_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/advanced_actions.json")).unwrap()
    }

    fn risk_action_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/risk_actions.json")).unwrap()
    }

    fn user_action_vector() -> Value {
        serde_json::from_str(include_str!(
            "../../../test-vectors/user_signed_actions.json"
        ))
        .unwrap()
    }

    fn user_authorization_vector() -> Value {
        serde_json::from_str(include_str!(
            "../../../test-vectors/user_authorizations.json"
        ))
        .unwrap()
    }

    fn user_account_transfer_vector() -> Value {
        serde_json::from_str(include_str!(
            "../../../test-vectors/user_account_transfers.json"
        ))
        .unwrap()
    }

    fn staking_action_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/staking_actions.json")).unwrap()
    }

    fn account_l1_vector() -> Value {
        serde_json::from_str(include_str!(
            "../../../test-vectors/account_l1_actions.json"
        ))
        .unwrap()
    }

    fn account_security_vector() -> Value {
        serde_json::from_str(include_str!(
            "../../../test-vectors/account_security_actions.json"
        ))
        .unwrap()
    }

    fn account_control_vector() -> Value {
        serde_json::from_str(include_str!(
            "../../../test-vectors/account_control_actions.json"
        ))
        .unwrap()
    }

    fn validator_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/validator_actions.json")).unwrap()
    }

    fn deployer_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/deployer_actions.json")).unwrap()
    }

    fn multi_sig_vector() -> Value {
        serde_json::from_str(include_str!("../../../test-vectors/multi_sig_actions.json")).unwrap()
    }

    fn multi_sig_signature_inputs(vector: &Value) -> Vec<MultiSigSignatureInput> {
        vector["innerSigners"]
            .as_array()
            .unwrap()
            .iter()
            .zip(vector["innerSignatures"].as_array().unwrap())
            .map(|(signer, signature)| MultiSigSignatureInput {
                expected_signer: signer.as_str().unwrap().to_owned(),
                r: signature["r"].as_str().unwrap().to_owned(),
                s: signature["s"].as_str().unwrap().to_owned(),
                v: u8::try_from(signature["v"].as_u64().unwrap()).unwrap(),
            })
            .collect()
    }

    fn assert_multi_sig_envelope_matches_vector(
        prepared: &super::PreparedMultiSigEnvelope,
        vector: &Value,
    ) {
        assert_eq!(
            serde_json::from_str::<Value>(&prepared.action_json).unwrap(),
            vector["outerAction"]
        );
        assert_eq!(prepared.message_pack_hex, vector["outerMessagePackHex"]);
        assert_eq!(prepared.action_hash_hex, vector["outerActionHashHex"]);
        assert_eq!(prepared.signing_digest_hex, vector["outerSigningDigestHex"]);
    }

    fn assert_multi_sig_payload_matches_vector(
        payload: super::PreparedMultiSigPayload,
        vector: &Value,
    ) {
        let payload: super::PreparedMultiSigPayload =
            serde_json::from_value(serde_json::to_value(payload).unwrap()).unwrap();
        let expected_vault = vector["vaultAddress"]
            .as_str()
            .map(str::parse::<EvmAddress>)
            .transpose()
            .unwrap();
        let expected_expiry = vector["expiresAfter"]
            .as_str()
            .map(str::parse::<u64>)
            .transpose()
            .unwrap();
        assert_eq!(payload.vault_address, expected_vault);
        assert_eq!(payload.expires_after, expected_expiry);
        assert_eq!(
            serde_json::from_str::<Value>(&payload.inner_action_json).unwrap(),
            vector["innerAction"]
        );
        assert_eq!(
            payload.signing_digest_hex,
            vector["innerSigningDigestHexes"][0]
        );
        let envelope =
            prepare_multi_sig_envelope(payload, multi_sig_signature_inputs(vector)).unwrap();
        assert_multi_sig_envelope_matches_vector(&envelope, vector);
    }

    fn user_vector_signature(vector: &Value) -> WireSignature {
        raw_vector_signature(&vector["signature"])
    }

    fn raw_vector_signature(signature: &Value) -> WireSignature {
        WireSignature::new(
            signature["r"].as_str().unwrap(),
            signature["s"].as_str().unwrap(),
            u8::try_from(signature["v"].as_u64().unwrap()).unwrap(),
        )
        .unwrap()
    }

    fn assert_prepared_matches_vector<A>(prepared: &super::PreparedAction<A>, vector: &Value) {
        assert_eq!(
            serde_json::from_str::<Value>(&prepared.action_json).unwrap(),
            vector["action"]
        );
        assert_eq!(prepared.message_pack_hex, vector["messagePackHex"]);
        assert_eq!(prepared.action_hash_hex, vector["actionHashHex"]);
    }

    fn assert_user_prepared_matches_vector<A>(
        prepared: &super::PreparedUserAction<A>,
        vector: &Value,
    ) {
        assert_eq!(
            serde_json::from_str::<Value>(&prepared.action_json).unwrap(),
            vector["action"]
        );
        assert_eq!(prepared.signing_digest_hex, vector["signingDigestHex"]);
    }
