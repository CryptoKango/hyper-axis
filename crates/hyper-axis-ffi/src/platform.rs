fn parse_signature(
    r: String,
    s: String,
    v: u8,
    expected_signer: &str,
) -> Result<(WireSignature, EvmAddress), String> {
    let signature = WireSignature::new(r, s, v).map_err(|error| error.to_string())?;
    let expected_signer = expected_signer
        .parse::<EvmAddress>()
        .map_err(|error| error.to_string())?;
    Ok((signature, expected_signer))
}

/// Configures structured diagnostic capture for the mobile host.
///
/// Supported levels are `error`, `warn`, `info`, `debug`, and `trace`.
/// Logging starts disabled and must be explicitly enabled by the application.
///
/// # Errors
///
/// Returns an error for an unsupported level, invalid capacity, or JSON
/// serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn configure_logging(enabled: bool, level: String, capacity: u32) -> Result<String, String> {
    let level = LogLevel::from_str(&level).map_err(|error| error.to_string())?;
    let config =
        logging::configure(enabled, level, capacity as usize).map_err(|error| error.to_string())?;
    serde_json::to_string(&config).map_err(|error| error.to_string())
}

/// Returns retained diagnostics without removing them.
///
/// # Errors
///
/// Returns an error only if JSON serialization fails.
pub fn snapshot_logs_json() -> Result<String, String> {
    serde_json::to_string(&logging::snapshot()).map_err(|error| error.to_string())
}

/// Returns and removes retained diagnostics.
///
/// # Errors
///
/// Returns an error only if JSON serialization fails.
pub fn drain_logs_json() -> Result<String, String> {
    serde_json::to_string(&logging::drain()).map_err(|error| error.to_string())
}

/// Clears retained diagnostics and the overflow counter.
pub fn clear_logs() {
    logging::clear();
}

/// Builds a runtime-neutral `POST /info` request as JSON.
///
/// The Dart host must send the returned `body` string directly rather than
/// parsing and reserializing protocol integers.
///
/// # Errors
///
/// Returns an error when `body` is not a JSON object or serialization fails.
#[allow(clippy::needless_pass_by_value)]
pub fn build_info_http_request_json(network: Network, body: String) -> Result<String, String> {
    let request = info_http_request(network, &body).map_err(|error| error.to_string())?;
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// Validates a typed Info query and returns its canonical wire body.
///
/// # Errors
///
/// Returns an error for invalid query fields or JSON serialization failure.
pub fn prepare_info_query_json(query: InfoQuery) -> Result<String, String> {
    let prepared = prepare_info_query(query).map_err(|error| error.to_string())?;
    serde_json::to_string(&prepared).map_err(|error| error.to_string())
}

/// Validates and canonicalizes a `perpDexs` response for Dart.
///
/// # Errors
///
/// Returns an error when the response does not match the protocol schema.
#[allow(clippy::needless_pass_by_value)]
pub fn parse_perp_dexs_response_json(body: String) -> Result<String, String> {
    let response = parse_perp_dexs_response(&body).map_err(|error| error.to_string())?;
    serde_json::to_string(&response).map_err(|error| error.to_string())
}

/// Validates and canonicalizes a native or HIP-3 `meta` response for Dart.
///
/// # Errors
///
/// Returns an error when the response does not match the protocol schema.
#[allow(clippy::needless_pass_by_value)]
pub fn parse_perp_meta_response_json(body: String) -> Result<String, String> {
    let response = parse_perp_meta_response(&body).map_err(|error| error.to_string())?;
    serde_json::to_string(&response).map_err(|error| error.to_string())
}

/// Validates and canonicalizes a native or HIP-3 `metaAndAssetCtxs` response for Dart.
///
/// # Errors
///
/// Returns an error when the response is malformed or its market and context
/// counts differ.
#[allow(clippy::needless_pass_by_value)]
pub fn parse_perp_meta_and_asset_ctxs_response_json(body: String) -> Result<String, String> {
    let response =
        parse_perp_meta_and_asset_ctxs_response(&body).map_err(|error| error.to_string())?;
    serde_json::to_string(&response).map_err(|error| error.to_string())
}

/// Builds a runtime-neutral `POST /info` request from a typed query.
///
/// # Errors
///
/// Returns an error for invalid query fields or JSON serialization failure.
pub fn build_typed_info_http_request_json(
    network: Network,
    query: InfoQuery,
) -> Result<String, String> {
    let request = typed_info_http_request(network, query).map_err(|error| error.to_string())?;
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// Builds a runtime-neutral `POST /exchange` request as JSON.
///
/// # Errors
///
/// Returns an error when the finalized artifact is not routed to an official
/// endpoint, its body is malformed, or serialization fails.
#[allow(clippy::needless_pass_by_value)]
pub fn build_exchange_http_request_json(finalized: FinalizedAction) -> Result<String, String> {
    let request = exchange_http_request(&finalized).map_err(|error| error.to_string())?;
    serde_json::to_string(&request).map_err(|error| error.to_string())
}

/// Parses and classifies an HTTP response for Dart as stable JSON.
///
/// This function does not log the body because it may contain private account
/// or order data.
///
/// # Errors
///
/// Returns an error only when the parsed model cannot be serialized.
#[allow(clippy::needless_pass_by_value)]
pub fn parse_http_response_json(
    request_kind: RequestKind,
    status_code: u16,
    body: String,
) -> Result<String, String> {
    let parsed = parse_http_response(request_kind, &HttpResponse { status_code, body });
    serde_json::to_string(&parsed).map_err(|error| error.to_string())
}

/// Classifies a host networking failure without exposing response contents.
///
/// # Errors
///
/// Returns an error only if serialization unexpectedly fails.
pub fn host_failure_retry_advice(
    request_kind: RequestKind,
    failure: HostFailureKind,
) -> Result<String, String> {
    serde_json::to_string(&classify_host_failure(request_kind, failure))
        .map_err(|error| error.to_string())
}

/// Returns the official WebSocket URL for a network.
#[must_use]
pub fn websocket_url(network: Network) -> String {
    network.websocket_url().to_owned()
}

/// Validates a subscription and builds its subscribe command.
///
/// # Errors
///
/// Returns an error for malformed subscription JSON or invalid fields.
#[allow(clippy::needless_pass_by_value)]
pub fn websocket_subscribe_json(subscription_json: String) -> Result<String, String> {
    websocket_subscription_command(&subscription_json, true)
}

/// Validates a subscription and builds its unsubscribe command.
///
/// # Errors
///
/// Returns an error for malformed subscription JSON or invalid fields.
#[allow(clippy::needless_pass_by_value)]
pub fn websocket_unsubscribe_json(subscription_json: String) -> Result<String, String> {
    websocket_subscription_command(&subscription_json, false)
}

/// Returns the canonical WebSocket heartbeat command.
///
/// # Errors
///
/// Returns an error only if serialization unexpectedly fails.
pub fn websocket_ping_json() -> Result<String, String> {
    WsCommand::Ping.to_json().map_err(|error| error.to_string())
}

/// Builds a read-only info post over WebSocket.
///
/// # Errors
///
/// Returns an error when the payload is not a JSON object.
#[allow(clippy::needless_pass_by_value)]
pub fn websocket_info_post_json(id: u32, payload: String) -> Result<String, String> {
    WsCommand::info_post(PostRequestId::new(id), &payload)
        .and_then(|command| command.to_json())
        .map_err(|error| error.to_string())
}

/// Builds a typed read-only Info post over WebSocket.
///
/// # Errors
///
/// Returns an error for invalid query fields or serialization failure.
pub fn websocket_typed_info_post_json(id: u32, query: InfoQuery) -> Result<String, String> {
    WsCommand::typed_info_post(PostRequestId::new(id), query)
        .and_then(|command| command.to_json())
        .map_err(|error| error.to_string())
}

/// Builds a signed-action post over WebSocket.
///
/// # Errors
///
/// Returns an error when the finalized artifact has an invalid endpoint/body.
#[allow(clippy::needless_pass_by_value)]
pub fn websocket_action_post_json(id: u32, finalized: FinalizedAction) -> Result<String, String> {
    WsCommand::action_post(PostRequestId::new(id), &finalized)
        .and_then(|command| command.to_json())
        .map_err(|error| error.to_string())
}

/// Parses one inbound WebSocket message into stable JSON.
///
/// # Errors
///
/// Returns an error for malformed messages or serialization failure.
#[allow(clippy::needless_pass_by_value)]
pub fn parse_websocket_message_json(message: String) -> Result<String, String> {
    let parsed = parse_ws_message(&message).map_err(|error| error.to_string())?;
    serde_json::to_string(&parsed).map_err(|error| error.to_string())
}

fn websocket_subscription_command(
    subscription_json: &str,
    subscribe: bool,
) -> Result<String, String> {
    let subscription: Subscription =
        serde_json::from_str(subscription_json).map_err(|error| error.to_string())?;
    let command = if subscribe {
        WsCommand::Subscribe { subscription }
    } else {
        WsCommand::Unsubscribe { subscription }
    };
    command.to_json().map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests;
