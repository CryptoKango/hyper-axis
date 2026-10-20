fn prepare_request_json<R, P>(
    request_json: &str,
    prepare: fn(R) -> Result<P, hyper_axis_core::client::ClientError>,
) -> Result<String, JsError>
where
    R: DeserializeOwned,
    P: Serialize,
{
    let request: R = serde_json::from_str(request_json)
        .map_err(|error| JsError::new(&format!("invalid preparation request: {error}")))?;
    let prepared = prepare(request).map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&prepared).map_err(|error| JsError::new(&error.to_string()))
}

fn prepare_multi_sig_payload_json<P>(
    prepared_json: &str,
    multi_sig_user: &str,
    outer_signer: &str,
    prepare: fn(
        P,
        &str,
        &str,
    ) -> Result<
        hyper_axis_core::client::PreparedMultiSigPayload,
        hyper_axis_core::client::MultiSigError,
    >,
) -> Result<String, JsError>
where
    P: DeserializeOwned,
{
    let prepared: P = serde_json::from_str(prepared_json)
        .map_err(|error| JsError::new(&format!("invalid prepared inner action: {error}")))?;
    let payload = prepare(prepared, multi_sig_user, outer_signer)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&payload).map_err(|error| JsError::new(&error.to_string()))
}

fn finalize_request_json<P>(
    prepared_json: &str,
    r: &str,
    s: &str,
    v: u8,
    expected_signer: &str,
    finalize: fn(P, WireSignature, EvmAddress) -> Result<FinalizedAction, FinalizeError>,
) -> Result<String, JsError>
where
    P: DeserializeOwned,
{
    let prepared: P = serde_json::from_str(prepared_json)
        .map_err(|error| JsError::new(&format!("invalid prepared action: {error}")))?;
    let signature =
        WireSignature::new(r, s, v).map_err(|error| JsError::new(&error.to_string()))?;
    let expected_signer = expected_signer
        .parse::<EvmAddress>()
        .map_err(|error| JsError::new(&error.to_string()))?;
    let finalized = finalize(prepared, signature, expected_signer)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&finalized).map_err(|error| JsError::new(&error.to_string()))
}

fn parse_u64(field: &str, value: &str) -> Result<u64, JsError> {
    value
        .parse()
        .map_err(|_| JsError::new(&format!("{field} must be an unsigned 64-bit integer")))
}

fn parse_network(network: &str) -> Result<Network, JsError> {
    match network {
        "mainnet" => Ok(Network::Mainnet),
        "testnet" => Ok(Network::Testnet),
        _ => Err(JsError::new("network must be mainnet or testnet")),
    }
}

fn parse_request_kind(kind: &str) -> Result<RequestKind, JsError> {
    match kind {
        "info" => Ok(RequestKind::Info),
        "signedAction" => Ok(RequestKind::SignedAction),
        _ => Err(JsError::new("requestKind must be info or signedAction")),
    }
}

fn parse_finalized_action(value: &str) -> Result<FinalizedAction, JsError> {
    serde_json::from_str(value)
        .map_err(|error| JsError::new(&format!("invalid finalized action: {error}")))
}

fn websocket_subscription_command(
    subscription_json: &str,
    subscribe: bool,
) -> Result<String, JsError> {
    let subscription: Subscription = serde_json::from_str(subscription_json)
        .map_err(|error| JsError::new(&format!("invalid subscription: {error}")))?;
    let command = if subscribe {
        WsCommand::Subscribe { subscription }
    } else {
        WsCommand::Unsubscribe { subscription }
    };
    command
        .to_json()
        .map_err(|error| JsError::new(&error.to_string()))
}
