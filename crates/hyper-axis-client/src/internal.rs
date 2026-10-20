fn prepare_multi_sig_l1_payload(
    network: Network,
    inner_action: MultiSigInnerAction,
    multi_sig_user: EvmAddress,
    outer_signer: EvmAddress,
    nonce: u64,
    vault_address: Option<EvmAddress>,
    expires_after: Option<u64>,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    let inner_action_json = serde_json::to_string(&inner_action)?;
    let envelope = (&multi_sig_user, &outer_signer, &inner_action);
    let inner_hash = action_hash(&envelope, vault_address.as_ref(), nonce, expires_after)?;
    let signing_digest = l1_signing_digest(network, inner_hash);
    Ok(PreparedMultiSigPayload {
        network,
        inner_action,
        multi_sig_user,
        outer_signer,
        nonce,
        vault_address,
        expires_after,
        inner_action_json,
        signing_digest_hex: hex::encode(signing_digest),
    })
}

fn prepare_multi_sig_from_l1<A: Serialize>(
    prepared: PreparedAction<A>,
    wrap: impl FnOnce(A) -> MultiSigInnerAction,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    verify_prepared_l1_integrity(&prepared)?;
    prepare_multi_sig_l1_payload(
        prepared.network,
        wrap(prepared.action),
        multi_sig_user.parse()?,
        outer_signer.parse()?,
        prepared.nonce,
        prepared.vault_address,
        prepared.expires_after,
    )
}

fn prepare_multi_sig_from_user<A: UserSignedAction + Serialize>(
    prepared: PreparedUserAction<A>,
    wrap: impl FnOnce(A) -> MultiSigInnerAction,
    multi_sig_user: &str,
    outer_signer: &str,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    verify_prepared_user_integrity(&prepared)?;
    prepare_multi_sig_user_payload(
        prepared.network,
        wrap(prepared.action),
        multi_sig_user.parse()?,
        outer_signer.parse()?,
        prepared.nonce,
    )
}

fn prepare_multi_sig_user_payload(
    network: Network,
    inner_action: MultiSigInnerAction,
    multi_sig_user: EvmAddress,
    outer_signer: EvmAddress,
    nonce: u64,
) -> Result<PreparedMultiSigPayload, MultiSigError> {
    let inner_action_json = serde_json::to_string(&inner_action)?;
    let mut payload = PreparedMultiSigPayload {
        network,
        inner_action,
        multi_sig_user,
        outer_signer,
        nonce,
        vault_address: None,
        expires_after: None,
        inner_action_json,
        signing_digest_hex: String::new(),
    };
    payload.signing_digest_hex = hex::encode(multi_sig_inner_digest(&payload)?);
    Ok(payload)
}

fn multi_sig_inner_digest(payload: &PreparedMultiSigPayload) -> Result<[u8; 32], MultiSigError> {
    match &payload.inner_action {
        MultiSigInnerAction::Order(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::Cancel(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::CancelByCloid(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::BatchModify(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::ScheduleCancel(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::TwapOrder(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::TwapCancel(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::UpdateLeverage(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::UpdateIsolatedMargin(action) => {
            multi_sig_l1_inner_digest(payload, action)
        }
        MultiSigInnerAction::TopUpIsolatedOnlyMargin(action) => {
            multi_sig_l1_inner_digest(payload, action)
        }
        MultiSigInnerAction::CreateSubAccount(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::SubAccountTransfer(action) => {
            multi_sig_l1_inner_digest(payload, action)
        }
        MultiSigInnerAction::SubAccountSpotTransfer(action) => {
            multi_sig_l1_inner_digest(payload, action)
        }
        MultiSigInnerAction::VaultTransfer(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::SetReferrer(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::EvmUserModify(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::AgentEnableDexAbstraction(action) => {
            multi_sig_l1_inner_digest(payload, action)
        }
        MultiSigInnerAction::AgentSetAbstraction(action) => {
            multi_sig_l1_inner_digest(payload, action)
        }
        MultiSigInnerAction::Noop(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::CSigner(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::CValidator(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::GossipPriorityBid(action) => {
            multi_sig_l1_inner_digest(payload, action)
        }
        MultiSigInnerAction::SpotDeploy(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::PerpDeploy(action) => multi_sig_l1_inner_digest(payload, action),
        MultiSigInnerAction::UsdSend(_)
        | MultiSigInnerAction::SpotSend(_)
        | MultiSigInnerAction::Withdraw(_)
        | MultiSigInnerAction::UsdClassTransfer(_)
        | MultiSigInnerAction::SendAsset(_) => multi_sig_transfer_inner_digest(payload),
        MultiSigInnerAction::ApproveAgent(_) | MultiSigInnerAction::ApproveBuilderFee(_) => {
            multi_sig_authorization_inner_digest(payload)
        }
        MultiSigInnerAction::CDeposit(_)
        | MultiSigInnerAction::CWithdraw(_)
        | MultiSigInnerAction::TokenDelegate(_)
        | MultiSigInnerAction::UserDexAbstraction(_)
        | MultiSigInnerAction::UserSetAbstraction(_) => multi_sig_account_inner_digest(payload),
    }
}

fn multi_sig_transfer_inner_digest(
    payload: &PreparedMultiSigPayload,
) -> Result<[u8; 32], MultiSigError> {
    match &payload.inner_action {
        MultiSigInnerAction::UsdSend(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_usd_send_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.destination.to_string(),
                action.amount.as_str(),
                action.time,
            ))
        }
        MultiSigInnerAction::SpotSend(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_spot_send_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.destination.to_string(),
                &action.token,
                action.amount.as_str(),
                action.time,
            ))
        }
        MultiSigInnerAction::Withdraw(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_withdraw_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.destination.to_string(),
                action.amount.as_str(),
                action.time,
            ))
        }
        MultiSigInnerAction::UsdClassTransfer(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_usd_class_transfer_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.amount.to_wire(),
                action.to_perp,
                action.nonce,
            ))
        }
        MultiSigInnerAction::SendAsset(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_send_asset_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.destination.to_string(),
                &action.source_dex,
                &action.destination_dex,
                &action.token,
                action.amount.as_str(),
                &action.from_sub_account.to_wire(),
                action.nonce,
            ))
        }
        _ => Err(MultiSigError::ArtifactMismatch("innerActionKind")),
    }
}

fn multi_sig_authorization_inner_digest(
    payload: &PreparedMultiSigPayload,
) -> Result<[u8; 32], MultiSigError> {
    match &payload.inner_action {
        MultiSigInnerAction::ApproveAgent(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_approve_agent_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.agent_address,
                action.signed_agent_name(),
                action.nonce,
            ))
        }
        MultiSigInnerAction::ApproveBuilderFee(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_approve_builder_fee_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.max_fee_rate.to_wire(),
                &action.builder,
                action.nonce,
            ))
        }
        _ => Err(MultiSigError::ArtifactMismatch("innerActionKind")),
    }
}

fn multi_sig_account_inner_digest(
    payload: &PreparedMultiSigPayload,
) -> Result<[u8; 32], MultiSigError> {
    match &payload.inner_action {
        MultiSigInnerAction::CDeposit(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_c_deposit_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                action.wei,
                action.nonce,
            ))
        }
        MultiSigInnerAction::CWithdraw(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_c_withdraw_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                action.wei,
                action.nonce,
            ))
        }
        MultiSigInnerAction::TokenDelegate(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_token_delegate_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.validator,
                action.wei,
                action.is_undelegate,
                action.nonce,
            ))
        }
        MultiSigInnerAction::UserDexAbstraction(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_user_dex_abstraction_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.user,
                action.enabled,
                action.nonce,
            ))
        }
        MultiSigInnerAction::UserSetAbstraction(action) => {
            verify_multi_sig_user_context(payload, action)?;
            Ok(multi_sig_user_set_abstraction_signing_digest(
                payload.network,
                &payload.multi_sig_user,
                &payload.outer_signer,
                &action.user,
                action.abstraction.as_str(),
                action.nonce,
            ))
        }
        _ => Err(MultiSigError::ArtifactMismatch("innerActionKind")),
    }
}

fn verify_multi_sig_user_context<A: UserSignedAction>(
    payload: &PreparedMultiSigPayload,
    action: &A,
) -> Result<(), MultiSigError> {
    verify_user_context(payload.network, action, payload.nonce)
        .map_err(|_| MultiSigError::ArtifactMismatch("userActionContext"))
}

fn multi_sig_l1_inner_digest<A: Serialize>(
    payload: &PreparedMultiSigPayload,
    action: &A,
) -> Result<[u8; 32], MultiSigError> {
    let envelope = (&payload.multi_sig_user, &payload.outer_signer, action);
    let inner_hash = action_hash(
        &envelope,
        payload.vault_address.as_ref(),
        payload.nonce,
        payload.expires_after,
    )?;
    Ok(l1_signing_digest(payload.network, inner_hash))
}

fn verify_prepared_l1_integrity<A: Serialize>(
    prepared: &PreparedAction<A>,
) -> Result<(), MultiSigError> {
    let action_json = serde_json::to_string(&prepared.action)?;
    verify_multi_sig_artifact("source.actionJson", &action_json, &prepared.action_json)?;
    let message_pack = encode_action(&prepared.action)?;
    verify_multi_sig_artifact(
        "source.messagePackHex",
        &hex::encode(message_pack),
        &prepared.message_pack_hex,
    )?;
    let hash = action_hash(
        &prepared.action,
        prepared.vault_address.as_ref(),
        prepared.nonce,
        prepared.expires_after,
    )?;
    verify_multi_sig_artifact(
        "source.actionHashHex",
        &hex::encode(hash),
        &prepared.action_hash_hex,
    )?;
    verify_multi_sig_artifact(
        "source.signingDigestHex",
        &hex::encode(l1_signing_digest(prepared.network, hash)),
        &prepared.signing_digest_hex,
    )
}

fn verify_prepared_user_integrity<A: UserSignedAction + Serialize>(
    prepared: &PreparedUserAction<A>,
) -> Result<(), MultiSigError> {
    verify_user_context(prepared.network, &prepared.action, prepared.nonce)
        .map_err(|_| MultiSigError::ArtifactMismatch("source.userActionContext"))?;
    let action_json = serde_json::to_string(&prepared.action)?;
    verify_multi_sig_artifact("source.actionJson", &action_json, &prepared.action_json)?;
    verify_multi_sig_artifact(
        "source.signingDigestHex",
        &hex::encode(prepared.action.signing_digest(prepared.network)),
        &prepared.signing_digest_hex,
    )
}

fn verify_multi_sig_payload(payload: &PreparedMultiSigPayload) -> Result<(), MultiSigError> {
    let action_json = serde_json::to_string(&payload.inner_action)?;
    verify_multi_sig_artifact("innerActionJson", &action_json, &payload.inner_action_json)?;
    let digest = multi_sig_inner_digest(payload)?;
    verify_multi_sig_artifact(
        "innerSigningDigestHex",
        &hex::encode(digest),
        &payload.signing_digest_hex,
    )
}

fn verify_multi_sig_envelope(prepared: &PreparedMultiSigEnvelope) -> Result<(), MultiSigError> {
    verify_multi_sig_payload(&prepared.payload)?;
    if prepared.signatures.is_empty()
        || prepared.signatures.len() != prepared.recovered_inner_signers.len()
    {
        return Err(MultiSigError::ArtifactMismatch("innerSignatures"));
    }
    let inner_digest = decode_digest(&prepared.payload.signing_digest_hex)?;
    for index in 0..prepared.signatures.len() {
        if index > 0
            && prepared.recovered_inner_signers[index - 1].as_bytes()
                >= prepared.recovered_inner_signers[index].as_bytes()
        {
            return Err(MultiSigError::ArtifactMismatch("innerSignerOrder"));
        }
        prepared.signatures[index]
            .verify_signer(inner_digest, prepared.recovered_inner_signers[index])?;
    }

    let wire = MultiSigWireContext::new(&prepared.payload, &prepared.signatures);
    verify_multi_sig_artifact(
        "actionJson",
        &serde_json::to_string(&wire.full_action())?,
        &prepared.action_json,
    )?;
    let message_pack = encode_action(&wire.without_tag())?;
    verify_multi_sig_artifact(
        "messagePackHex",
        &hex::encode(message_pack),
        &prepared.message_pack_hex,
    )?;
    let hash = action_hash(
        &wire.without_tag(),
        prepared.payload.vault_address.as_ref(),
        prepared.payload.nonce,
        prepared.payload.expires_after,
    )?;
    verify_multi_sig_artifact(
        "actionHashHex",
        &hex::encode(hash),
        &prepared.action_hash_hex,
    )?;
    verify_multi_sig_artifact(
        "signingDigestHex",
        &hex::encode(multi_sig_envelope_signing_digest(
            prepared.payload.network,
            hash,
            prepared.payload.nonce,
        )),
        &prepared.signing_digest_hex,
    )
}

fn verify_multi_sig_artifact(
    field: &'static str,
    recomputed: &str,
    prepared: &str,
) -> Result<(), MultiSigError> {
    if recomputed == prepared {
        Ok(())
    } else {
        Err(MultiSigError::ArtifactMismatch(field))
    }
}

fn decode_digest(value: &str) -> Result<[u8; 32], MultiSigError> {
    let bytes =
        hex::decode(value).map_err(|_| MultiSigError::ArtifactMismatch("signingDigestHex"))?;
    bytes
        .try_into()
        .map_err(|_| MultiSigError::ArtifactMismatch("signingDigestHex"))
}

struct MultiSigWireContext<'a> {
    payload: &'a PreparedMultiSigPayload,
    signatures: &'a [WireSignature],
}

impl<'a> MultiSigWireContext<'a> {
    const fn new(payload: &'a PreparedMultiSigPayload, signatures: &'a [WireSignature]) -> Self {
        Self {
            payload,
            signatures,
        }
    }

    const fn payload_wire(&self) -> MultiSigPayloadWire<'a> {
        MultiSigPayloadWire {
            multi_sig_user: &self.payload.multi_sig_user,
            outer_signer: &self.payload.outer_signer,
            action: MultiSigOuterActionRef(&self.payload.inner_action),
        }
    }

    const fn without_tag(&self) -> MultiSigActionWithoutTag<'a> {
        MultiSigActionWithoutTag {
            signature_chain_id: "0x66eee",
            signatures: self.signatures,
            payload: self.payload_wire(),
        }
    }

    const fn full_action(&self) -> MultiSigActionWire<'a> {
        MultiSigActionWire {
            action_type: "multiSig",
            signature_chain_id: "0x66eee",
            signatures: self.signatures,
            payload: self.payload_wire(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MultiSigPayloadWire<'a> {
    multi_sig_user: &'a EvmAddress,
    outer_signer: &'a EvmAddress,
    action: MultiSigOuterActionRef<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MultiSigActionWithoutTag<'a> {
    signature_chain_id: &'static str,
    signatures: &'a [WireSignature],
    payload: MultiSigPayloadWire<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MultiSigActionWire<'a> {
    #[serde(rename = "type")]
    action_type: &'static str,
    signature_chain_id: &'static str,
    signatures: &'a [WireSignature],
    payload: MultiSigPayloadWire<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MultiSigExchangeRequest<'a> {
    action: MultiSigActionWire<'a>,
    nonce: u64,
    signature: WireSignature,
    vault_address: Option<EvmAddress>,
    expires_after: Option<u64>,
}

#[derive(Clone, Copy)]
struct MultiSigOuterActionRef<'a>(&'a MultiSigInnerAction);

impl Serialize for MultiSigOuterActionRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            MultiSigInnerAction::Order(action) => action.serialize(serializer),
            MultiSigInnerAction::Cancel(action) => action.serialize(serializer),
            MultiSigInnerAction::CancelByCloid(action) => action.serialize(serializer),
            MultiSigInnerAction::BatchModify(action) => action.serialize(serializer),
            MultiSigInnerAction::ScheduleCancel(action) => action.serialize(serializer),
            MultiSigInnerAction::TwapOrder(action) => action.serialize(serializer),
            MultiSigInnerAction::TwapCancel(action) => action.serialize(serializer),
            MultiSigInnerAction::UpdateLeverage(action) => action.serialize(serializer),
            MultiSigInnerAction::UpdateIsolatedMargin(action) => action.serialize(serializer),
            MultiSigInnerAction::TopUpIsolatedOnlyMargin(action) => action.serialize(serializer),
            MultiSigInnerAction::CreateSubAccount(action) => action.serialize(serializer),
            MultiSigInnerAction::SubAccountTransfer(action) => action.serialize(serializer),
            MultiSigInnerAction::SubAccountSpotTransfer(action) => action.serialize(serializer),
            MultiSigInnerAction::VaultTransfer(action) => action.serialize(serializer),
            MultiSigInnerAction::SetReferrer(action) => action.serialize(serializer),
            MultiSigInnerAction::EvmUserModify(action) => action.serialize(serializer),
            MultiSigInnerAction::AgentEnableDexAbstraction(action) => action.serialize(serializer),
            MultiSigInnerAction::AgentSetAbstraction(action) => action.serialize(serializer),
            MultiSigInnerAction::Noop(action) => action.serialize(serializer),
            MultiSigInnerAction::CSigner(action) => action.serialize(serializer),
            MultiSigInnerAction::CValidator(action) => action.serialize(serializer),
            MultiSigInnerAction::GossipPriorityBid(action) => action.serialize(serializer),
            MultiSigInnerAction::SpotDeploy(action) => action.serialize(serializer),
            MultiSigInnerAction::PerpDeploy(action) => action.serialize(serializer),
            MultiSigInnerAction::UsdSend(action) => action.serialize(serializer),
            MultiSigInnerAction::SpotSend(action) => action.serialize(serializer),
            MultiSigInnerAction::Withdraw(action) => action.serialize(serializer),
            MultiSigInnerAction::ApproveAgent(action) => action.serialize(serializer),
            MultiSigInnerAction::ApproveBuilderFee(action) => action.serialize(serializer),
            MultiSigInnerAction::UsdClassTransfer(action) => action.serialize(serializer),
            MultiSigInnerAction::SendAsset(action) => action.serialize(serializer),
            MultiSigInnerAction::CDeposit(action) => action.serialize(serializer),
            MultiSigInnerAction::CWithdraw(action) => action.serialize(serializer),
            MultiSigInnerAction::TokenDelegate(action) => action.serialize(serializer),
            MultiSigInnerAction::UserDexAbstraction(action) => action.serialize(serializer),
            MultiSigInnerAction::UserSetAbstraction(action) => {
                let abstraction = match action.abstraction {
                    UserAbstraction::Disabled => "i",
                    UserAbstraction::UnifiedAccount => "u",
                    UserAbstraction::PortfolioMargin => "p",
                };
                let mut state = serializer.serialize_struct("UserSetAbstractionAction", 6)?;
                state.serialize_field("type", "userSetAbstraction")?;
                state.serialize_field("hyperliquidChain", action.hyperliquid_chain())?;
                state.serialize_field("signatureChainId", "0x66eee")?;
                state.serialize_field("user", &action.user)?;
                state.serialize_field("abstraction", abstraction)?;
                state.serialize_field("nonce", &action.nonce)?;
                state.end()
            }
        }
    }
}

fn prepare_action<A: Serialize>(
    network: Network,
    action: A,
    nonce: u64,
    vault_address: Option<EvmAddress>,
    expires_after: Option<u64>,
    operation: &'static str,
) -> Result<PreparedAction<A>, ClientError> {
    emit(
        LogLevel::Debug,
        "hyper_axis.client.prepare_action",
        format!("action preparation started operation={operation}"),
    );
    let result = (|| {
        let message_pack = encode_action(&action)?;
        let action_hash = action_hash(&action, vault_address.as_ref(), nonce, expires_after)?;
        let signing_digest = l1_signing_digest(network, action_hash);
        let action_json = serde_json::to_string(&action)?;
        Ok(PreparedAction {
            network,
            action,
            nonce,
            vault_address,
            expires_after,
            action_json,
            message_pack_hex: hex::encode(message_pack),
            action_hash_hex: hex::encode(action_hash),
            signing_digest_hex: hex::encode(signing_digest),
        })
    })();
    log_prepare_result(&result, operation);
    result
}

trait UserSignedAction {
    fn nonce(&self) -> u64;
    fn hyperliquid_chain(&self) -> &'static str;
    fn signing_digest(&self, network: Network) -> [u8; 32];
}

impl UserSignedAction for UsdSendAction {
    fn nonce(&self) -> u64 {
        self.time
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        usd_send_signing_digest(
            network,
            &self.destination.to_string(),
            self.amount.as_str(),
            self.time,
        )
    }
}

impl UserSignedAction for SpotSendAction {
    fn nonce(&self) -> u64 {
        self.time
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        spot_send_signing_digest(
            network,
            &self.destination.to_string(),
            &self.token,
            self.amount.as_str(),
            self.time,
        )
    }
}

impl UserSignedAction for WithdrawAction {
    fn nonce(&self) -> u64 {
        self.time
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        withdraw_signing_digest(
            network,
            &self.destination.to_string(),
            self.amount.as_str(),
            self.time,
        )
    }
}

impl UserSignedAction for ApproveAgentAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        approve_agent_signing_digest(
            network,
            &self.agent_address,
            self.signed_agent_name(),
            self.nonce,
        )
    }
}

impl UserSignedAction for ApproveBuilderFeeAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        approve_builder_fee_signing_digest(
            network,
            &self.max_fee_rate.to_wire(),
            &self.builder,
            self.nonce,
        )
    }
}

impl UserSignedAction for UsdClassTransferAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        usd_class_transfer_signing_digest(network, &self.amount.to_wire(), self.to_perp, self.nonce)
    }
}

impl UserSignedAction for SendAssetAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        send_asset_signing_digest(
            network,
            &self.destination.to_string(),
            &self.source_dex,
            &self.destination_dex,
            &self.token,
            self.amount.as_str(),
            &self.from_sub_account.to_wire(),
            self.nonce,
        )
    }
}

impl UserSignedAction for CDepositAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        c_deposit_signing_digest(network, self.wei, self.nonce)
    }
}

impl UserSignedAction for CWithdrawAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        c_withdraw_signing_digest(network, self.wei, self.nonce)
    }
}

impl UserSignedAction for TokenDelegateAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        token_delegate_signing_digest(
            network,
            &self.validator,
            self.wei,
            self.is_undelegate,
            self.nonce,
        )
    }
}

impl UserSignedAction for UserDexAbstractionAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        user_dex_abstraction_signing_digest(network, &self.user, self.enabled, self.nonce)
    }
}

impl UserSignedAction for UserSetAbstractionAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        user_set_abstraction_signing_digest(
            network,
            &self.user,
            self.abstraction.as_str(),
            self.nonce,
        )
    }
}

impl UserSignedAction for ConvertToMultiSigUserAction {
    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn hyperliquid_chain(&self) -> &'static str {
        self.hyperliquid_chain()
    }

    fn signing_digest(&self, network: Network) -> [u8; 32] {
        convert_to_multi_sig_user_signing_digest(network, &self.signers.to_wire(), self.nonce)
    }
}

fn prepare_user_action<A: UserSignedAction + Serialize>(
    network: Network,
    action: A,
    nonce: u64,
    operation: &'static str,
) -> Result<PreparedUserAction<A>, ClientError> {
    emit(
        LogLevel::Debug,
        "hyper_axis.client.prepare_user_action",
        format!("user action preparation started operation={operation}"),
    );
    let result = (|| {
        verify_user_context(network, &action, nonce)
            .map_err(|_| ProtocolError::InvalidAction("user action context mismatch"))?;
        let action_json = serde_json::to_string(&action)?;
        let signing_digest = action.signing_digest(network);
        Ok(PreparedUserAction {
            network,
            action,
            nonce,
            action_json,
            signing_digest_hex: hex::encode(signing_digest),
        })
    })();
    log_user_prepare_result(&result, operation);
    result
}

fn finalize_action<A: Serialize>(
    prepared: PreparedAction<A>,
    signature: WireSignature,
    expected_signer: EvmAddress,
    operation: &'static str,
) -> Result<FinalizedAction, FinalizeError> {
    emit(
        LogLevel::Debug,
        "hyper_axis.client.finalize_action",
        format!("action finalization started operation={operation}"),
    );
    let result = (|| {
        let action_json = serde_json::to_string(&prepared.action)?;
        verify_artifact("actionJson", &action_json, &prepared.action_json)?;

        let message_pack = encode_action(&prepared.action)?;
        verify_artifact(
            "messagePackHex",
            &hex::encode(&message_pack),
            &prepared.message_pack_hex,
        )?;

        let action_hash = action_hash(
            &prepared.action,
            prepared.vault_address.as_ref(),
            prepared.nonce,
            prepared.expires_after,
        )?;
        verify_artifact(
            "actionHashHex",
            &hex::encode(action_hash),
            &prepared.action_hash_hex,
        )?;

        let signing_digest = l1_signing_digest(prepared.network, action_hash);
        verify_artifact(
            "signingDigestHex",
            &hex::encode(signing_digest),
            &prepared.signing_digest_hex,
        )?;
        let recovered_signer = signature.verify_signer(signing_digest, expected_signer)?;

        let request = ExchangeRequest {
            action: prepared.action,
            nonce: prepared.nonce,
            signature,
            vault_address: prepared.vault_address,
            expires_after: prepared.expires_after,
        };
        Ok(FinalizedAction {
            endpoint: prepared.network.exchange_url().to_owned(),
            request_json: serde_json::to_string(&request)?,
            recovered_signer,
        })
    })();

    log_finalize_result(&result, operation);
    result
}

fn finalize_user_action<A: UserSignedAction + Serialize>(
    prepared: PreparedUserAction<A>,
    signature: WireSignature,
    expected_signer: EvmAddress,
    operation: &'static str,
) -> Result<FinalizedAction, FinalizeError> {
    emit(
        LogLevel::Debug,
        "hyper_axis.client.finalize_user_action",
        format!("user action finalization started operation={operation}"),
    );
    let result = (|| {
        verify_user_context(prepared.network, &prepared.action, prepared.nonce)?;
        let action_json = serde_json::to_string(&prepared.action)?;
        verify_artifact("actionJson", &action_json, &prepared.action_json)?;

        let signing_digest = prepared.action.signing_digest(prepared.network);
        verify_artifact(
            "signingDigestHex",
            &hex::encode(signing_digest),
            &prepared.signing_digest_hex,
        )?;
        let recovered_signer = signature.verify_signer(signing_digest, expected_signer)?;
        let request = UserExchangeRequest {
            action: prepared.action,
            nonce: prepared.nonce,
            signature,
        };
        Ok(FinalizedAction {
            endpoint: prepared.network.exchange_url().to_owned(),
            request_json: serde_json::to_string(&request)?,
            recovered_signer,
        })
    })();
    log_finalize_result(&result, operation);
    result
}

fn verify_user_context<A: UserSignedAction>(
    network: Network,
    action: &A,
    nonce: u64,
) -> Result<(), FinalizeError> {
    if action.nonce() != nonce || action.hyperliquid_chain() != network.hyperliquid_chain() {
        return Err(FinalizeError::ArtifactMismatch("userActionContext"));
    }
    Ok(())
}

fn build_limit_order(input: LimitOrderInput) -> Result<OrderWire, ProtocolError> {
    let precision = MarketPrecision::new(input.market_kind, input.sz_decimals)?;
    let price = precision.price(&input.limit_price)?;
    let size = precision.size(&input.size)?;
    let client_order_id = input
        .client_order_id
        .map(|value| value.parse())
        .transpose()?;
    OrderWire::new(
        input.asset,
        input.is_buy,
        price,
        size,
        input.reduce_only,
        OrderTypeWire::Limit(LimitWire {
            tif: input.time_in_force,
        }),
        client_order_id,
    )
}

fn build_order(input: OrderInput) -> Result<OrderWire, ProtocolError> {
    match input {
        OrderInput::Limit(input) => build_limit_order(input),
        OrderInput::Trigger(input) => build_trigger_order(input),
    }
}

fn build_trigger_order(input: TriggerOrderInput) -> Result<OrderWire, ProtocolError> {
    let precision = MarketPrecision::new(input.market_kind, input.sz_decimals)?;
    let limit_price = precision.price(&input.limit_price)?;
    let trigger_price = precision.price(&input.trigger_price)?;
    let size = precision.size(&input.size)?;
    let client_order_id = input
        .client_order_id
        .map(|value| value.parse())
        .transpose()?;
    OrderWire::new(
        input.asset,
        input.is_buy,
        limit_price,
        size,
        input.reduce_only,
        OrderTypeWire::Trigger(TriggerWire {
            is_market: input.is_market,
            trigger_px: trigger_price,
            tpsl: input.trigger_kind,
        }),
        client_order_id,
    )
}

fn parse_builder(value: Option<BuilderInput>) -> Result<Option<BuilderWire>, ProtocolError> {
    value
        .map(|input| {
            input
                .address
                .parse::<EvmAddress>()
                .map(|address| BuilderWire::new(address, input.fee))
        })
        .transpose()
}

fn parse_optional_address(value: Option<String>) -> Result<Option<EvmAddress>, ProtocolError> {
    value.map(|address| address.parse()).transpose()
}

fn parse_ip(value: &str) -> Result<String, ProtocolError> {
    value
        .parse::<IpAddr>()
        .map(|ip| ip.to_string())
        .map_err(|_| ProtocolError::InvalidAction("validator or gossip IP address is invalid"))
}

fn build_spot_deploy_variant(input: SpotDeployInput) -> Result<SpotDeployVariant, ProtocolError> {
    match input {
        SpotDeployInput::RegisterToken {
            token_name,
            sz_decimals,
            wei_decimals,
            max_gas,
            full_name,
        } => build_spot_register_token(token_name, sz_decimals, wei_decimals, max_gas, full_name),
        SpotDeployInput::UserGenesis {
            token,
            user_and_wei,
            existing_token_and_wei,
        } => build_spot_user_genesis(token, user_and_wei, existing_token_and_wei),
        SpotDeployInput::EnableFreezePrivilege { token } => Ok(
            SpotDeployVariant::EnableFreezePrivilege(SpotTokenReference { token }),
        ),
        SpotDeployInput::FreezeUser {
            token,
            user,
            freeze,
        } => Ok(SpotDeployVariant::FreezeUser(SpotFreezeUser {
            token,
            user: user.parse()?,
            freeze,
        })),
        SpotDeployInput::RevokeFreezePrivilege { token } => Ok(
            SpotDeployVariant::RevokeFreezePrivilege(SpotTokenReference { token }),
        ),
        SpotDeployInput::EnableQuoteToken { token } => {
            Ok(SpotDeployVariant::EnableQuoteToken(SpotTokenReference {
                token,
            }))
        }
        SpotDeployInput::Genesis {
            token,
            max_supply,
            no_hyperliquidity,
        } => build_spot_genesis(token, max_supply, no_hyperliquidity),
        SpotDeployInput::RegisterSpot {
            base_token,
            quote_token,
        } => build_spot_register_spot(base_token, quote_token),
        SpotDeployInput::RegisterHyperliquidity {
            spot,
            start_px,
            order_sz,
            n_orders,
            n_seeded_levels,
        } => build_spot_hyperliquidity(spot, &start_px, &order_sz, n_orders, n_seeded_levels),
        SpotDeployInput::SetDeployerTradingFeeShare { token, share } => Ok(
            SpotDeployVariant::SetDeployerTradingFeeShare(SpotDeployerTradingFeeShare {
                token,
                share: parse_positive_decimal(&share, "deployer trading fee share")?,
            }),
        ),
    }
}

fn build_spot_register_token(
    token_name: String,
    sz_decimals: u8,
    wei_decimals: u8,
    max_gas: u64,
    full_name: String,
) -> Result<SpotDeployVariant, ProtocolError> {
    validate_nonempty_display_text(&token_name, "spot token name is invalid")?;
    validate_nonempty_display_text(&full_name, "spot token full name is invalid")?;
    if sz_decimals > wei_decimals {
        return Err(ProtocolError::InvalidAction(
            "spot szDecimals must not exceed weiDecimals",
        ));
    }
    Ok(SpotDeployVariant::RegisterToken2(SpotRegisterToken {
        spec: SpotTokenSpec {
            name: token_name,
            sz_decimals,
            wei_decimals,
        },
        max_gas,
        full_name,
    }))
}

fn build_spot_user_genesis(
    token: u32,
    user_and_wei: Vec<SpotUserWeiInput>,
    existing_token_and_wei: Vec<SpotExistingTokenWeiInput>,
) -> Result<SpotDeployVariant, ProtocolError> {
    let users = user_and_wei
        .into_iter()
        .map(|entry| {
            validate_unsigned_integer_string(&entry.wei, "user genesis wei is invalid")?;
            Ok((entry.user.parse::<EvmAddress>()?, entry.wei))
        })
        .collect::<Result<Vec<_>, ProtocolError>>()?;
    if users
        .iter()
        .enumerate()
        .any(|(index, entry)| users[..index].iter().any(|other| other.0 == entry.0))
    {
        return Err(ProtocolError::InvalidAction(
            "user genesis addresses must be unique",
        ));
    }
    let existing = existing_token_and_wei
        .into_iter()
        .map(|entry| {
            validate_unsigned_integer_string(&entry.wei, "existing-token genesis wei is invalid")?;
            Ok((entry.token, entry.wei))
        })
        .collect::<Result<Vec<_>, ProtocolError>>()?;
    if existing
        .iter()
        .enumerate()
        .any(|(index, entry)| existing[..index].iter().any(|other| other.0 == entry.0))
    {
        return Err(ProtocolError::InvalidAction(
            "existing-token genesis entries must be unique",
        ));
    }
    Ok(SpotDeployVariant::UserGenesis(SpotUserGenesis {
        token,
        user_and_wei: users,
        existing_token_and_wei: existing,
    }))
}

fn build_spot_genesis(
    token: u32,
    max_supply: String,
    no_hyperliquidity: bool,
) -> Result<SpotDeployVariant, ProtocolError> {
    validate_positive_integer_string(&max_supply, "spot max supply is invalid")?;
    Ok(SpotDeployVariant::Genesis(SpotGenesis {
        token,
        max_supply,
        no_hyperliquidity: no_hyperliquidity.then_some(true),
    }))
}

fn build_spot_register_spot(
    base_token: u32,
    quote_token: u32,
) -> Result<SpotDeployVariant, ProtocolError> {
    if base_token == quote_token {
        return Err(ProtocolError::InvalidAction(
            "spot base and quote tokens must differ",
        ));
    }
    Ok(SpotDeployVariant::RegisterSpot(SpotRegisterSpot {
        tokens: [base_token, quote_token],
    }))
}

fn build_spot_hyperliquidity(
    spot: u32,
    start_px: &str,
    order_sz: &str,
    n_orders: u32,
    n_seeded_levels: Option<u32>,
) -> Result<SpotDeployVariant, ProtocolError> {
    if n_orders == 0 || n_seeded_levels == Some(0) {
        return Err(ProtocolError::InvalidAction(
            "Hyperliquidity order and seeded-level counts must be positive",
        ));
    }
    Ok(SpotDeployVariant::RegisterHyperliquidity(
        SpotRegisterHyperliquidity {
            spot,
            start_px: parse_positive_decimal(start_px, "Hyperliquidity start price")?,
            order_sz: parse_positive_decimal(order_sz, "Hyperliquidity order size")?,
            n_orders,
            n_seeded_levels,
        },
    ))
}

fn validate_unsigned_integer_string(
    value: &str,
    message: &'static str,
) -> Result<(), ProtocolError> {
    if value.is_empty()
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(ProtocolError::InvalidAction(message));
    }
    Ok(())
}

fn validate_positive_integer_string(
    value: &str,
    message: &'static str,
) -> Result<(), ProtocolError> {
    validate_unsigned_integer_string(value, message)?;
    if value == "0" {
        return Err(ProtocolError::InvalidAction(message));
    }
    Ok(())
}

fn parse_positive_decimal(value: &str, label: &'static str) -> Result<DecimalValue, ProtocolError> {
    let value = DecimalValue::parse(value)?;
    if !value.is_positive() {
        return Err(ProtocolError::InvalidAction(label));
    }
    Ok(value)
}

fn canonical_perp_prices(
    values: Vec<PerpPriceInput>,
) -> Result<Vec<PerpPriceEntry>, ProtocolError> {
    let mut values = values
        .into_iter()
        .map(|entry| {
            validate_nonempty_display_text(&entry.coin, "perpetual price coin is invalid")?;
            Ok((
                entry.coin,
                parse_positive_decimal(&entry.price, "perpetual price must be positive")?,
            ))
        })
        .collect::<Result<Vec<_>, ProtocolError>>()?;
    values.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    if values.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(ProtocolError::InvalidAction(
            "perpetual price coins must be unique",
        ));
    }
    Ok(values)
}

fn validate_display_text(value: &str, message: &'static str) -> Result<(), ProtocolError> {
    if value != value.trim() || value.chars().any(char::is_control) {
        return Err(ProtocolError::InvalidAction(message));
    }
    Ok(())
}

fn validate_nonempty_display_text(value: &str, message: &'static str) -> Result<(), ProtocolError> {
    validate_display_text(value, message)?;
    if value.is_empty() {
        return Err(ProtocolError::InvalidAction(message));
    }
    Ok(())
}

fn validate_commission(commission_bps: u16) -> Result<(), ProtocolError> {
    if commission_bps > 10_000 {
        return Err(ProtocolError::InvalidAction(
            "validator commission must not exceed 10000 basis points",
        ));
    }
    Ok(())
}

fn log_prepare_result<A>(result: &Result<PreparedAction<A>, ClientError>, operation: &str) {
    match result {
        Ok(_) => emit(
            LogLevel::Info,
            "hyper_axis.client.prepare_action",
            format!("action preparation completed operation={operation}"),
        ),
        Err(ClientError::Protocol(_)) => emit(
            LogLevel::Warn,
            "hyper_axis.client.prepare_action",
            format!("action preparation failed operation={operation} category=protocol"),
        ),
        Err(ClientError::Codec(_)) => emit(
            LogLevel::Error,
            "hyper_axis.client.prepare_action",
            format!("action preparation failed operation={operation} category=codec"),
        ),
        Err(ClientError::Json(_)) => emit(
            LogLevel::Error,
            "hyper_axis.client.prepare_action",
            format!("action preparation failed operation={operation} category=json"),
        ),
    }
}

fn log_user_prepare_result<A>(
    result: &Result<PreparedUserAction<A>, ClientError>,
    operation: &str,
) {
    match result {
        Ok(_) => emit(
            LogLevel::Info,
            "hyper_axis.client.prepare_user_action",
            format!("user action preparation completed operation={operation}"),
        ),
        Err(ClientError::Protocol(_)) => emit(
            LogLevel::Warn,
            "hyper_axis.client.prepare_user_action",
            format!("user action preparation failed operation={operation} category=protocol"),
        ),
        Err(ClientError::Codec(_)) => emit(
            LogLevel::Error,
            "hyper_axis.client.prepare_user_action",
            format!("user action preparation failed operation={operation} category=codec"),
        ),
        Err(ClientError::Json(_)) => emit(
            LogLevel::Error,
            "hyper_axis.client.prepare_user_action",
            format!("user action preparation failed operation={operation} category=json"),
        ),
    }
}

fn log_finalize_result(result: &Result<FinalizedAction, FinalizeError>, operation: &str) {
    match result {
        Ok(_) => emit(
            LogLevel::Info,
            "hyper_axis.client.finalize_action",
            format!("action finalization completed operation={operation}"),
        ),
        Err(FinalizeError::ArtifactMismatch(_)) => emit(
            LogLevel::Error,
            "hyper_axis.client.finalize_action",
            format!("action finalization failed operation={operation} category=integrity"),
        ),
        Err(FinalizeError::Codec(_)) => emit(
            LogLevel::Error,
            "hyper_axis.client.finalize_action",
            format!("action finalization failed operation={operation} category=codec"),
        ),
        Err(FinalizeError::Signing(_)) => emit(
            LogLevel::Warn,
            "hyper_axis.client.finalize_action",
            format!("action finalization failed operation={operation} category=signature"),
        ),
        Err(FinalizeError::Json(_)) => emit(
            LogLevel::Error,
            "hyper_axis.client.finalize_action",
            format!("action finalization failed operation={operation} category=json"),
        ),
    }
}

fn verify_artifact(
    field: &'static str,
    recomputed: &str,
    prepared: &str,
) -> Result<(), FinalizeError> {
    if recomputed != prepared {
        return Err(FinalizeError::ArtifactMismatch(field));
    }
    Ok(())
}

mod u64_string {
    use serde::{Deserialize, Deserializer, Serializer, de};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value
            .parse()
            .map_err(|_| de::Error::custom("expected an unsigned 64-bit decimal string"))
    }
}

mod option_u64_string {
    use serde::{Deserialize, Deserializer, Serializer, de};

    #[allow(clippy::ref_option)]
    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_some(&value.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|value| {
                value
                    .parse()
                    .map_err(|_| de::Error::custom("expected an unsigned 64-bit decimal string"))
            })
            .transpose()
    }
}

#[cfg(test)]
mod tests;
