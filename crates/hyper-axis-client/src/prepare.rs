/// Validates and prepares a limit order for platform signing.
///
/// # Errors
///
/// Returns an error when precision validation, action construction, address
/// parsing, `MessagePack` encoding, or JSON serialization fails.
pub fn prepare_limit_order(
    request: PrepareLimitOrderRequest,
) -> Result<PreparedOrder, ClientError> {
    prepare_batch_orders(PrepareBatchOrdersRequest {
        network: request.network,
        orders: vec![LimitOrderInput {
            asset: request.asset,
            market_kind: request.market_kind,
            sz_decimals: request.sz_decimals,
            is_buy: request.is_buy,
            limit_price: request.limit_price,
            size: request.size,
            reduce_only: request.reduce_only,
            time_in_force: request.time_in_force,
            client_order_id: request.client_order_id,
        }],
        grouping: request.grouping,
        builder: None,
        nonce: request.nonce,
        vault_address: request.vault_address,
        expires_after: request.expires_after,
    })
}

/// Validates and prepares an atomic batch of limit orders.
///
/// # Errors
///
/// Returns an error when the batch is empty or any address, Cloid, precision,
/// action, encoding, or serialization rule fails. The complete batch fails; no
/// partial signing artifact is produced.
pub fn prepare_batch_orders(
    request: PrepareBatchOrdersRequest,
) -> Result<PreparedOrder, ClientError> {
    prepare_orders(PrepareOrdersRequest {
        network: request.network,
        orders: request.orders.into_iter().map(OrderInput::Limit).collect(),
        grouping: request.grouping,
        builder: request.builder,
        nonce: request.nonce,
        vault_address: request.vault_address,
        expires_after: request.expires_after,
    })
}

/// Validates and prepares an atomic mixed batch of limit and trigger orders.
///
/// # Errors
///
/// Returns an error when the batch is empty or any address, Cloid, precision,
/// action, encoding, or serialization rule fails.
pub fn prepare_orders(request: PrepareOrdersRequest) -> Result<PreparedOrder, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let builder = parse_builder(request.builder)?;
    let orders = request
        .orders
        .into_iter()
        .map(build_order)
        .collect::<Result<Vec<_>, _>>()?;
    let action = OrderAction::new(orders, request.grouping, builder)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "order",
    )
}

/// Prepares a non-empty cancellation batch by exchange order ID.
///
/// # Errors
///
/// Returns an error when the batch is empty, the vault address is invalid, or
/// action encoding or serialization fails.
pub fn prepare_cancel(request: PrepareCancelRequest) -> Result<PreparedCancel, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let cancels = request
        .cancels
        .into_iter()
        .map(|cancel| CancelWire {
            asset: cancel.asset,
            oid: cancel.oid,
        })
        .collect();
    let action = CancelAction::new(cancels)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "cancel",
    )
}

/// Prepares a non-empty cancellation batch by client order ID.
///
/// # Errors
///
/// Returns an error when the batch is empty, any Cloid or vault address is
/// invalid, or action encoding or serialization fails.
pub fn prepare_cancel_by_cloid(
    request: PrepareCancelByCloidRequest,
) -> Result<PreparedCancelByCloid, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let cancels = request
        .cancels
        .into_iter()
        .map(|cancel| {
            Ok(CancelByCloidWire {
                asset: cancel.asset,
                cloid: cancel.cloid.parse::<ClientOrderId>()?,
            })
        })
        .collect::<Result<Vec<_>, ProtocolError>>()?;
    let action = CancelByCloidAction::new(cancels)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "cancel_by_cloid",
    )
}

/// Prepares a non-empty batch of complete order replacements.
///
/// # Errors
///
/// Returns an error when the batch is empty or any selector, address, Cloid,
/// precision, action, encoding, or serialization rule fails.
pub fn prepare_batch_modify(
    request: PrepareBatchModifyRequest,
) -> Result<PreparedBatchModify, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let modifies = request
        .modifies
        .into_iter()
        .map(|modify| {
            let oid = match modify.oid {
                OrderSelectorInput::Oid(oid) => OrderId::Oid(oid),
                OrderSelectorInput::Cloid(cloid) => OrderId::Cloid(cloid.parse()?),
            };
            Ok(ModifyWire {
                oid,
                order: build_limit_order(modify.order)?,
            })
        })
        .collect::<Result<Vec<_>, ProtocolError>>()?;
    let action = BatchModifyAction::new(modifies)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "batch_modify",
    )
}

/// Prepares a dead man's switch set or clear action.
///
/// # Errors
///
/// Returns an error when a set deadline is less than five seconds after the
/// supplied current time, or when signing-context encoding fails.
pub fn prepare_schedule_cancel(
    request: PrepareScheduleCancelRequest,
) -> Result<PreparedScheduleCancel, ClientError> {
    if request
        .scheduled_time
        .is_some_and(|scheduled| scheduled < request.now_ms.saturating_add(5_000))
    {
        return Err(ProtocolError::InvalidAction(
            "scheduled cancel time must be at least five seconds in the future",
        )
        .into());
    }
    let vault_address = parse_optional_address(request.vault_address)?;
    prepare_action(
        request.network,
        ScheduleCancelAction::new(request.scheduled_time),
        request.nonce,
        vault_address,
        request.expires_after,
        "schedule_cancel",
    )
}

/// Prepares one native TWAP order.
///
/// # Errors
///
/// Returns an error when size precision, duration, address, encoding, or
/// serialization validation fails.
pub fn prepare_twap_order(
    request: PrepareTwapOrderRequest,
) -> Result<PreparedTwapOrder, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let precision = MarketPrecision::new(request.market_kind, request.sz_decimals)?;
    let size = precision.size(&request.size)?;
    let twap = TwapWire::new(
        request.asset,
        request.is_buy,
        size,
        request.reduce_only,
        request.minutes,
        request.randomize,
    )?;
    prepare_action(
        request.network,
        TwapOrderAction::new(twap),
        request.nonce,
        vault_address,
        request.expires_after,
        "twap_order",
    )
}

/// Prepares one native TWAP cancellation.
///
/// # Errors
///
/// Returns an error when address, encoding, or serialization validation fails.
pub fn prepare_twap_cancel(
    request: PrepareTwapCancelRequest,
) -> Result<PreparedTwapCancel, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    prepare_action(
        request.network,
        TwapCancelAction::new(request.asset, request.twap_id),
        request.nonce,
        vault_address,
        request.expires_after,
        "twap_cancel",
    )
}

/// Prepares a cross/isolated leverage update using current market metadata.
///
/// # Errors
///
/// Returns an error for leverage outside `1..=maxLeverage`, an invalid vault
/// address, or signing-context encoding failure.
pub fn prepare_update_leverage(
    request: PrepareUpdateLeverageRequest,
) -> Result<PreparedUpdateLeverage, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let action = UpdateLeverageAction::new(
        request.asset,
        request.is_cross,
        request.leverage,
        request.max_leverage,
    )?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "update_leverage",
    )
}

/// Prepares an exact six-decimal USD isolated-margin delta.
///
/// # Errors
///
/// Returns an error for malformed, over-precise, zero, or out-of-range USD
/// values, invalid addresses, or signing-context encoding failure.
pub fn prepare_update_isolated_margin(
    request: PrepareUpdateIsolatedMarginRequest,
) -> Result<PreparedUpdateIsolatedMargin, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let usd_delta = DecimalValue::parse(&request.usd_delta)?;
    let ntli = usd_delta.to_scaled_i64("isolated margin delta", 6)?;
    let action = UpdateIsolatedMarginAction::new(request.asset, request.is_buy, ntli)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "update_isolated_margin",
    )
}

/// Prepares an isolated-only margin top-up targeting exact leverage.
///
/// # Errors
///
/// Returns an error for invalid/non-positive leverage, invalid addresses, or
/// signing-context encoding failure.
pub fn prepare_top_up_isolated_only_margin(
    request: PrepareTopUpIsolatedOnlyMarginRequest,
) -> Result<PreparedTopUpIsolatedOnlyMargin, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let leverage = DecimalValue::parse(&request.leverage)?;
    let action = TopUpIsolatedOnlyMarginAction::new(request.asset, leverage)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "top_up_isolated_only_margin",
    )
}

/// Prepares creation of a named subaccount owned by the signer.
///
/// The outer vault field is intentionally fixed to `None`, matching the
/// official L1 action signing path.
///
/// # Errors
///
/// Returns an error for an invalid name, encoding, or serialization failure.
pub fn prepare_create_sub_account(
    request: PrepareCreateSubAccountRequest,
) -> Result<PreparedCreateSubAccount, ClientError> {
    let action = CreateSubAccountAction::new(request.name)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "create_sub_account",
    )
}

/// Prepares a six-decimal USD transfer between master and subaccount.
///
/// # Errors
///
/// Returns an error for an invalid address, non-positive amount, more than six
/// fractional digits, overflow, encoding, or serialization failure.
pub fn prepare_sub_account_transfer(
    request: PrepareSubAccountTransferRequest,
) -> Result<PreparedSubAccountTransfer, ClientError> {
    let sub_account_user = request.sub_account_user.parse::<EvmAddress>()?;
    let usd = DecimalValue::parse(request.amount)?.to_scaled_u64("subaccount USD amount", 6)?;
    let action = SubAccountTransferAction::new(sub_account_user, request.is_deposit, usd)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "sub_account_transfer",
    )
}

/// Prepares a spot-token transfer between master and subaccount.
///
/// # Errors
///
/// Returns an error for an invalid address, token, non-positive amount,
/// encoding, or serialization failure.
pub fn prepare_sub_account_spot_transfer(
    request: PrepareSubAccountSpotTransferRequest,
) -> Result<PreparedSubAccountSpotTransfer, ClientError> {
    let sub_account_user = request.sub_account_user.parse::<EvmAddress>()?;
    let amount = DecimalValue::parse(request.amount)?;
    let action = SubAccountSpotTransferAction::new(
        sub_account_user,
        request.is_deposit,
        request.token,
        amount,
    )?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "sub_account_spot_transfer",
    )
}

/// Prepares a six-decimal USD vault deposit or withdrawal.
///
/// The vault address is part of the action. The outer signing-context vault
/// remains `None`, matching the official SDK.
///
/// # Errors
///
/// Returns an error for an invalid address, non-positive amount, more than six
/// fractional digits, overflow, encoding, or serialization failure.
pub fn prepare_vault_transfer(
    request: PrepareVaultTransferRequest,
) -> Result<PreparedVaultTransfer, ClientError> {
    let vault_address = request.vault_address.parse::<EvmAddress>()?;
    let usd = DecimalValue::parse(request.amount)?.to_scaled_u64("vault USD amount", 6)?;
    let action = VaultTransferAction::new(vault_address, request.is_deposit, usd)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "vault_transfer",
    )
}

/// Prepares referral-code registration for the signer.
///
/// The official SDK signs this action without an outer vault context.
///
/// # Errors
///
/// Returns an error for an invalid code, encoding, or serialization failure.
pub fn prepare_set_referrer(
    request: PrepareSetReferrerRequest,
) -> Result<PreparedSetReferrer, ClientError> {
    let action = SetReferrerAction::new(request.code)?;
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "set_referrer",
    )
}

/// Prepares an EVM large-block preference update.
///
/// The official SDK signs this action without an outer vault context.
///
/// # Errors
///
/// Returns an error for encoding or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_evm_user_modify(
    request: PrepareEvmUserModifyRequest,
) -> Result<PreparedEvmUserModify, ClientError> {
    let action = EvmUserModifyAction::new(request.using_big_blocks);
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "evm_user_modify",
    )
}

/// Prepares legacy DEX-abstraction enablement for an agent.
///
/// # Errors
///
/// Returns an error for invalid vault addresses, encoding, or serialization.
pub fn prepare_agent_enable_dex_abstraction(
    request: PrepareAgentEnableDexAbstractionRequest,
) -> Result<PreparedAgentEnableDexAbstraction, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    prepare_action(
        request.network,
        AgentEnableDexAbstractionAction::new(),
        request.nonce,
        vault_address,
        request.expires_after,
        "agent_enable_dex_abstraction",
    )
}

/// Prepares an agent account-abstraction mode update.
///
/// # Errors
///
/// Returns an error for invalid vault addresses, encoding, or serialization.
pub fn prepare_agent_set_abstraction(
    request: PrepareAgentSetAbstractionRequest,
) -> Result<PreparedAgentSetAbstraction, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    prepare_action(
        request.network,
        AgentSetAbstractionAction::new(request.abstraction),
        request.nonce,
        vault_address,
        request.expires_after,
        "agent_set_abstraction",
    )
}

/// Prepares an empty L1 action with an explicit signing context.
///
/// # Errors
///
/// Returns an error for invalid vault addresses, encoding, or serialization.
pub fn prepare_noop(request: PrepareNoopRequest) -> Result<PreparedNoop, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    prepare_action(
        request.network,
        NoopAction::new(),
        request.nonce,
        vault_address,
        request.expires_after,
        "noop",
    )
}

/// Prepares signer-node jail or unjail self-management.
///
/// The official SDK always signs this action without a vault context.
///
/// # Errors
///
/// Returns an error for encoding or serialization failure.
pub fn prepare_c_signer(request: PrepareCSignerRequest) -> Result<PreparedCSigner, ClientError> {
    prepare_action(
        request.network,
        CSignerAction::new(request.variant),
        request.nonce,
        None,
        request.expires_after,
        "c_signer",
    )
}

/// Prepares consensus validator registration.
///
/// The official SDK always signs this action without a vault context.
///
/// # Errors
///
/// Returns an error for an invalid IP, address, text field, commission, or encoding.
pub fn prepare_c_validator_register(
    request: PrepareCValidatorRegisterRequest,
) -> Result<PreparedCValidator, ClientError> {
    validate_nonempty_display_text(&request.name, "validator name is invalid")?;
    validate_display_text(&request.description, "validator description is invalid")?;
    validate_commission(request.commission_bps)?;
    let profile = ValidatorProfile {
        node_ip: ValidatorNodeIp {
            ip: parse_ip(&request.node_ip)?,
        },
        name: request.name,
        description: request.description,
        delegations_disabled: request.delegations_disabled,
        commission_bps: request.commission_bps,
        signer: request.signer.parse()?,
    };
    let action = CValidatorAction::register(ValidatorRegister {
        profile,
        unjailed: request.unjailed,
        initial_wei: request.initial_wei,
    });
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "c_validator_register",
    )
}

/// Prepares a partial consensus validator profile update.
///
/// The official SDK always signs this action without a vault context.
///
/// # Errors
///
/// Returns an error for an invalid IP, address, text field, commission, or encoding.
pub fn prepare_c_validator_change_profile(
    request: PrepareCValidatorChangeProfileRequest,
) -> Result<PreparedCValidator, ClientError> {
    if let Some(name) = request.name.as_deref() {
        validate_display_text(name, "validator name is invalid")?;
    }
    if let Some(description) = request.description.as_deref() {
        validate_display_text(description, "validator description is invalid")?;
    }
    if let Some(commission_bps) = request.commission_bps {
        validate_commission(commission_bps)?;
    }
    let action = CValidatorAction::change_profile(ValidatorChangeProfile {
        node_ip: request
            .node_ip
            .as_deref()
            .map(parse_ip)
            .transpose()?
            .map(|ip| ValidatorNodeIp { ip }),
        name: request.name,
        description: request.description,
        unjailed: request.unjailed,
        disable_delegations: request.disable_delegations,
        commission_bps: request.commission_bps,
        signer: request
            .signer
            .as_deref()
            .map(str::parse::<EvmAddress>)
            .transpose()?,
    });
    prepare_action(
        request.network,
        action,
        request.nonce,
        None,
        request.expires_after,
        "c_validator_change_profile",
    )
}

/// Prepares consensus validator unregistration.
///
/// # Errors
///
/// Returns an error for encoding or serialization failure.
pub fn prepare_c_validator_unregister(
    request: PrepareCValidatorUnregisterRequest,
) -> Result<PreparedCValidator, ClientError> {
    prepare_action(
        request.network,
        CValidatorAction::unregister(),
        request.nonce,
        None,
        request.expires_after,
        "c_validator_unregister",
    )
}

/// Prepares a gossip priority gas bid.
///
/// # Errors
///
/// Returns an error for an invalid IP, vault address, encoding, or serialization.
pub fn prepare_gossip_priority_bid(
    request: PrepareGossipPriorityBidRequest,
) -> Result<PreparedGossipPriorityBid, ClientError> {
    let vault_address = parse_optional_address(request.vault_address)?;
    let action =
        GossipPriorityBidAction::new(request.slot_id, parse_ip(&request.ip)?, request.max_gas);
    prepare_action(
        request.network,
        action,
        request.nonce,
        vault_address,
        request.expires_after,
        "gossip_priority_bid",
    )
}

/// Prepares any official Spot deployer lifecycle action.
///
/// Spot deployer actions are always signed without a vault context, matching
/// the official SDK.
///
/// # Errors
///
/// Returns an error for malformed names, addresses, amounts, duplicate
/// allocations, invalid precision fields, or serialization failure.
pub fn prepare_spot_deploy(
    request: PrepareSpotDeployRequest,
) -> Result<PreparedSpotDeploy, ClientError> {
    let variant = build_spot_deploy_variant(request.input)?;
    prepare_action(
        request.network,
        SpotDeployAction::new(variant),
        request.nonce,
        None,
        request.expires_after,
        "spot_deploy",
    )
}

/// Prepares any official perpetual deployer lifecycle action.
///
/// # Errors
///
/// Returns an error for malformed names, prices, duplicate oracle entries,
/// invalid addresses, or serialization failure.
pub fn prepare_perp_deploy(
    request: PreparePerpDeployRequest,
) -> Result<PreparedPerpDeploy, ClientError> {
    let variant = match request.input {
        PerpDeployInput::RegisterAsset {
            dex,
            max_gas,
            coin,
            sz_decimals,
            oracle_px,
            margin_table_id,
            only_isolated,
            schema,
        } => {
            validate_display_text(&dex, "perpetual DEX name is invalid")?;
            validate_nonempty_display_text(&coin, "perpetual coin name is invalid")?;
            let schema = schema
                .map(|schema| {
                    validate_nonempty_display_text(
                        &schema.full_name,
                        "perpetual DEX full name is invalid",
                    )?;
                    Ok::<_, ProtocolError>(PerpDexSchema {
                        full_name: schema.full_name,
                        collateral_token: schema.collateral_token,
                        oracle_updater: schema
                            .oracle_updater
                            .as_deref()
                            .map(str::parse::<EvmAddress>)
                            .transpose()?,
                    })
                })
                .transpose()?;
            PerpDeployVariant::RegisterAsset(PerpRegisterAsset {
                max_gas,
                asset_request: PerpAssetRequest {
                    coin,
                    sz_decimals,
                    oracle_px: parse_positive_decimal(&oracle_px, "perpetual oracle price")?,
                    margin_table_id,
                    only_isolated,
                },
                dex,
                schema,
            })
        }
        PerpDeployInput::SetOracle {
            dex,
            oracle_pxs,
            mark_pxs,
            external_perp_pxs,
        } => {
            validate_display_text(&dex, "perpetual DEX name is invalid")?;
            let oracle_pxs = canonical_perp_prices(oracle_pxs)?;
            let mark_pxs = mark_pxs
                .into_iter()
                .map(canonical_perp_prices)
                .collect::<Result<Vec<_>, _>>()?;
            let external_perp_pxs = canonical_perp_prices(external_perp_pxs)?;
            PerpDeployVariant::SetOracle(PerpSetOracle {
                dex,
                oracle_pxs,
                mark_pxs,
                external_perp_pxs,
            })
        }
    };
    prepare_action(
        request.network,
        PerpDeployAction::new(variant),
        request.nonce,
        None,
        request.expires_after,
        "perp_deploy",
    )
}

/// Prepares a user-signed Core USDC transfer.
///
/// # Errors
///
/// Returns an error for invalid destinations, malformed/non-positive amounts,
/// or serialization failure.
pub fn prepare_usd_send(request: PrepareUsdSendRequest) -> Result<PreparedUsdSend, ClientError> {
    let destination = request.destination.parse::<EvmAddress>()?;
    let amount = DecimalValue::parse(request.amount)?;
    let action = UsdSendAction::new(request.network, destination, amount, request.nonce)?;
    prepare_user_action(request.network, action, request.nonce, "usd_send")
}

/// Prepares a user-signed Core spot transfer.
///
/// # Errors
///
/// Returns an error for invalid destinations, token identifiers, amounts, or
/// serialization failure.
pub fn prepare_spot_send(request: PrepareSpotSendRequest) -> Result<PreparedSpotSend, ClientError> {
    let destination = request.destination.parse::<EvmAddress>()?;
    let amount = DecimalValue::parse(request.amount)?;
    let action = SpotSendAction::new(
        request.network,
        destination,
        request.token,
        amount,
        request.nonce,
    )?;
    prepare_user_action(request.network, action, request.nonce, "spot_send")
}

/// Prepares a user-signed bridge withdrawal.
///
/// # Errors
///
/// Returns an error for invalid destinations, malformed/non-positive amounts,
/// or serialization failure.
pub fn prepare_withdraw(request: PrepareWithdrawRequest) -> Result<PreparedWithdraw, ClientError> {
    let destination = request.destination.parse::<EvmAddress>()?;
    let amount = DecimalValue::parse(request.amount)?;
    let action = WithdrawAction::new(request.network, destination, amount, request.nonce)?;
    prepare_user_action(request.network, action, request.nonce, "withdraw")
}

/// Prepares a user-signed API wallet authorization.
///
/// `HyperAxis` accepts an address only and never generates, receives, or returns
/// the API wallet private key.
///
/// # Errors
///
/// Returns an error for invalid addresses or serialization failure.
pub fn prepare_approve_agent(
    request: PrepareApproveAgentRequest,
) -> Result<PreparedApproveAgent, ClientError> {
    let agent_address = request.agent_address.parse::<EvmAddress>()?;
    let action = ApproveAgentAction::new(
        request.network,
        agent_address,
        request.agent_name,
        request.nonce,
    );
    prepare_user_action(request.network, action, request.nonce, "approve_agent")
}

/// Prepares a user-signed builder fee authorization.
///
/// # Errors
///
/// Returns an error for invalid builder addresses, malformed/negative fee
/// percentages, or serialization failure.
pub fn prepare_approve_builder_fee(
    request: PrepareApproveBuilderFeeRequest,
) -> Result<PreparedApproveBuilderFee, ClientError> {
    let builder = request.builder.parse::<EvmAddress>()?;
    let max_fee_rate = BuilderFeeRate::parse(request.max_fee_rate)?;
    let action =
        ApproveBuilderFeeAction::new(request.network, max_fee_rate, builder, request.nonce);
    prepare_user_action(
        request.network,
        action,
        request.nonce,
        "approve_builder_fee",
    )
}

/// Prepares a user-signed transfer between spot and perp USD balances.
///
/// # Errors
///
/// Returns an error for invalid/non-positive amounts, invalid subaccount
/// addresses, or serialization failure.
pub fn prepare_usd_class_transfer(
    request: PrepareUsdClassTransferRequest,
) -> Result<PreparedUsdClassTransfer, ClientError> {
    let amount = DecimalValue::parse(request.amount)?;
    let sub_account = request
        .sub_account
        .map(|address| address.parse::<EvmAddress>())
        .transpose()?;
    let amount = UsdClassTransferAmount::new(amount, sub_account)?;
    let action =
        UsdClassTransferAction::new(request.network, amount, request.to_perp, request.nonce);
    prepare_user_action(request.network, action, request.nonce, "usd_class_transfer")
}

/// Prepares a generalized user-signed asset transfer.
///
/// The server remains authoritative for token collateral compatibility with
/// the selected perp DEXs.
///
/// # Errors
///
/// Returns an error for invalid addresses, token identifiers, DEX names,
/// non-positive amounts, or serialization failure.
pub fn prepare_send_asset(
    request: PrepareSendAssetRequest,
) -> Result<PreparedSendAsset, ClientError> {
    let destination = request.destination.parse::<EvmAddress>()?;
    let amount = DecimalValue::parse(request.amount)?;
    let from_sub_account = request
        .from_sub_account
        .map(|address| address.parse::<EvmAddress>())
        .transpose()?;
    let action = SendAssetAction::new(
        request.network,
        destination,
        request.source_dex,
        request.destination_dex,
        request.token,
        amount,
        FromSubAccount::new(from_sub_account),
        request.nonce,
    )?;
    prepare_user_action(request.network, action, request.nonce, "send_asset")
}

/// Prepares a user-signed native-token deposit into staking.
///
/// # Errors
///
/// Returns an error when `wei` is zero or serialization fails.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_c_deposit(request: PrepareCDepositRequest) -> Result<PreparedCDeposit, ClientError> {
    let action = CDepositAction::new(request.network, request.wei, request.nonce)?;
    prepare_user_action(request.network, action, request.nonce, "c_deposit")
}

/// Prepares a user-signed native-token withdrawal from staking.
///
/// The protocol applies a seven-day unstaking queue after submission.
///
/// # Errors
///
/// Returns an error when `wei` is zero or serialization fails.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_c_withdraw(
    request: PrepareCWithdrawRequest,
) -> Result<PreparedCWithdraw, ClientError> {
    let action = CWithdrawAction::new(request.network, request.wei, request.nonce)?;
    prepare_user_action(request.network, action, request.nonce, "c_withdraw")
}

/// Prepares a user-signed validator delegation or undelegation.
///
/// The protocol applies a one-day lockup to delegations for a validator.
///
/// # Errors
///
/// Returns an error for an invalid validator, zero `wei`, or serialization
/// failure.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_token_delegate(
    request: PrepareTokenDelegateRequest,
) -> Result<PreparedTokenDelegate, ClientError> {
    let validator = request.validator.parse::<EvmAddress>()?;
    let action = TokenDelegateAction::new(
        request.network,
        validator,
        request.wei,
        request.is_undelegate,
        request.nonce,
    )?;
    prepare_user_action(request.network, action, request.nonce, "token_delegate")
}

/// Prepares the legacy user DEX-abstraction toggle.
///
/// # Errors
///
/// Returns an error for an invalid user address or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_user_dex_abstraction(
    request: PrepareUserDexAbstractionRequest,
) -> Result<PreparedUserDexAbstraction, ClientError> {
    let user = request.user.parse::<EvmAddress>()?;
    let action =
        UserDexAbstractionAction::new(request.network, user, request.enabled, request.nonce);
    prepare_user_action(
        request.network,
        action,
        request.nonce,
        "user_dex_abstraction",
    )
}

/// Prepares an account-abstraction mode update.
///
/// # Errors
///
/// Returns an error for an invalid user address or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn prepare_user_set_abstraction(
    request: PrepareUserSetAbstractionRequest,
) -> Result<PreparedUserSetAbstraction, ClientError> {
    let user = request.user.parse::<EvmAddress>()?;
    let action =
        UserSetAbstractionAction::new(request.network, user, request.abstraction, request.nonce);
    prepare_user_action(
        request.network,
        action,
        request.nonce,
        "user_set_abstraction",
    )
}

/// Prepares conversion to a threshold multi-signature user.
///
/// Authorized users are parsed, sorted, checked for duplicates, and encoded
/// into the exact JSON string signed by the official SDK.
///
/// # Errors
///
/// Returns an error for malformed/duplicate addresses, an empty signer set, an
/// unsatisfiable threshold, or serialization failure.
pub fn prepare_convert_to_multi_sig_user(
    request: PrepareConvertToMultiSigUserRequest,
) -> Result<PreparedConvertToMultiSigUser, ClientError> {
    let authorized_users = request
        .authorized_users
        .into_iter()
        .map(|address| address.parse::<EvmAddress>())
        .collect::<Result<Vec<_>, _>>()?;
    let signers = MultiSigSigners::new(authorized_users, request.threshold)?;
    let action = ConvertToMultiSigUserAction::new(request.network, signers, request.nonce);
    prepare_user_action(
        request.network,
        action,
        request.nonce,
        "convert_to_multi_sig_user",
    )
}
