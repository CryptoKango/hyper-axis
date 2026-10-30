/// Returns the official WebSocket URL for a network.
///
/// # Errors
///
/// Returns a JavaScript error for an unsupported network name.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn websocket_url(network: &str) -> Result<String, JsError> {
    Ok(parse_network(network)?.websocket_url().to_owned())
}

/// Builds a runtime-neutral `POST /info` request as JSON.
///
/// The nested `body` string must be sent directly by the JavaScript host.
///
/// # Errors
///
/// Returns a JavaScript error when the network/body is invalid.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn build_info_http_request_json(network: &str, body: &str) -> Result<String, JsError> {
    let request = info_http_request(parse_network(network)?, body)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&request).map_err(|error| JsError::new(&error.to_string()))
}

/// Validates a typed Info query and returns its canonical wire body.
///
/// Timestamps and OIDs in `query_json` must be decimal strings.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or invalid query fields.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn prepare_info_query_json(query_json: &str) -> Result<String, JsError> {
    let query: InfoQuery = serde_json::from_str(query_json)
        .map_err(|error| JsError::new(&format!("invalid Info query: {error}")))?;
    let prepared = prepare_info_query(query).map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&prepared).map_err(|error| JsError::new(&error.to_string()))
}

/// Validates and canonicalizes a `perpDexs` response.
///
/// Null array entries are preserved because their indexes are part of HIP-3
/// asset ID calculation.
///
/// # Errors
///
/// Returns a JavaScript error when the response does not match the protocol schema.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_perp_dexs_response_json(body: &str) -> Result<String, JsError> {
    let response =
        parse_perp_dexs_response(body).map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&response).map_err(|error| JsError::new(&error.to_string()))
}

/// Validates and canonicalizes a native or HIP-3 `meta` response.
///
/// # Errors
///
/// Returns a JavaScript error when the response does not match the protocol schema.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_perp_meta_response_json(body: &str) -> Result<String, JsError> {
    let response =
        parse_perp_meta_response(body).map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&response).map_err(|error| JsError::new(&error.to_string()))
}

/// Validates and canonicalizes a native or HIP-3 `metaAndAssetCtxs` response.
///
/// # Errors
///
/// Returns a JavaScript error when the response is malformed or its market and
/// context counts differ.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_perp_meta_and_asset_ctxs_response_json(body: &str) -> Result<String, JsError> {
    let response = parse_perp_meta_and_asset_ctxs_response(body)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&response).map_err(|error| JsError::new(&error.to_string()))
}

/// Validates and canonicalizes a perpetual `clearinghouseState` response.
///
/// # Errors
///
/// Returns a JavaScript error when the response does not match the protocol schema.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_perp_clearinghouse_state_response_json(body: &str) -> Result<String, JsError> {
    let response = parse_perp_clearinghouse_state_response(body)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&response).map_err(|error| JsError::new(&error.to_string()))
}

/// Validates and canonicalizes a spot `spotClearinghouseState` response.
///
/// # Errors
///
/// Returns a JavaScript error when the response does not match the protocol schema.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_spot_clearinghouse_state_response_json(body: &str) -> Result<String, JsError> {
    let response = parse_spot_clearinghouse_state_response(body)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&response).map_err(|error| JsError::new(&error.to_string()))
}

/// Builds a runtime-neutral `POST /info` request from a typed query.
///
/// # Errors
///
/// Returns a JavaScript error for an unsupported network, malformed JSON, or
/// invalid query fields.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn build_typed_info_http_request_json(
    network: &str,
    query_json: &str,
) -> Result<String, JsError> {
    let query: InfoQuery = serde_json::from_str(query_json)
        .map_err(|error| JsError::new(&format!("invalid Info query: {error}")))?;
    let request = typed_info_http_request(parse_network(network)?, query)
        .map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&request).map_err(|error| JsError::new(&error.to_string()))
}

/// Builds a runtime-neutral `POST /exchange` request from finalized JSON.
///
/// # Errors
///
/// Returns a JavaScript error for malformed or unsafe finalized artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn build_exchange_http_request_json(finalized_json: &str) -> Result<String, JsError> {
    let finalized = parse_finalized_action(finalized_json)?;
    let request =
        exchange_http_request(&finalized).map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&request).map_err(|error| JsError::new(&error.to_string()))
}

/// Parses and classifies an HTTP response as stable JSON.
///
/// # Errors
///
/// Returns a JavaScript error for an unsupported request kind or serialization
/// failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_http_response_json(
    request_kind: &str,
    status_code: u16,
    body: &str,
) -> Result<String, JsError> {
    let request_kind = parse_request_kind(request_kind)?;
    let parsed = parse_http_response(
        request_kind,
        &HttpResponse {
            status_code,
            body: body.to_owned(),
        },
    );
    serde_json::to_string(&parsed).map_err(|error| JsError::new(&error.to_string()))
}

/// Returns retry guidance for a host networking failure.
///
/// # Errors
///
/// Returns a JavaScript error for unsupported request/failure names or an
/// unexpected serialization failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn host_failure_retry_advice(request_kind: &str, failure: &str) -> Result<String, JsError> {
    let request_kind = parse_request_kind(request_kind)?;
    let failure = match failure {
        "timeout" => HostFailureKind::Timeout,
        "connection" => HostFailureKind::Connection,
        "dns" => HostFailureKind::Dns,
        "tls" => HostFailureKind::Tls,
        "cancelled" => HostFailureKind::Cancelled,
        "other" => HostFailureKind::Other,
        _ => return Err(JsError::new("unsupported host failure kind")),
    };
    serde_json::to_string(&classify_host_failure(request_kind, failure))
        .map_err(|error| JsError::new(&error.to_string()))
}

/// Validates a subscription and builds its subscribe command.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or invalid fields.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn websocket_subscribe_json(subscription_json: &str) -> Result<String, JsError> {
    websocket_subscription_command(subscription_json, true)
}

/// Validates a subscription and builds its unsubscribe command.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or invalid fields.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn websocket_unsubscribe_json(subscription_json: &str) -> Result<String, JsError> {
    websocket_subscription_command(subscription_json, false)
}

/// Returns the canonical WebSocket heartbeat command.
///
/// # Errors
///
/// Returns a JavaScript error only if serialization unexpectedly fails.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn websocket_ping_json() -> Result<String, JsError> {
    WsCommand::Ping
        .to_json()
        .map_err(|error| JsError::new(&error.to_string()))
}

/// Builds a read-only info post over WebSocket.
///
/// # Errors
///
/// Returns a JavaScript error when the payload is not a JSON object.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn websocket_info_post_json(id: u32, payload: &str) -> Result<String, JsError> {
    WsCommand::info_post(PostRequestId::new(id), payload)
        .and_then(|command| command.to_json())
        .map_err(|error| JsError::new(&error.to_string()))
}

/// Builds a typed read-only Info post over WebSocket.
///
/// # Errors
///
/// Returns a JavaScript error for malformed JSON or invalid query fields.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn websocket_typed_info_post_json(id: u32, query_json: &str) -> Result<String, JsError> {
    let query: InfoQuery = serde_json::from_str(query_json)
        .map_err(|error| JsError::new(&format!("invalid Info query: {error}")))?;
    WsCommand::typed_info_post(PostRequestId::new(id), query)
        .and_then(|command| command.to_json())
        .map_err(|error| JsError::new(&error.to_string()))
}

/// Builds a signed-action post over WebSocket.
///
/// # Errors
///
/// Returns a JavaScript error for malformed or unsafe finalized artifacts.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn websocket_action_post_json(id: u32, finalized_json: &str) -> Result<String, JsError> {
    let finalized = parse_finalized_action(finalized_json)?;
    WsCommand::action_post(PostRequestId::new(id), &finalized)
        .and_then(|command| command.to_json())
        .map_err(|error| JsError::new(&error.to_string()))
}

/// Parses one inbound WebSocket message into stable JSON.
///
/// # Errors
///
/// Returns a JavaScript error for malformed messages or serialization failure.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn parse_websocket_message_json(message: &str) -> Result<String, JsError> {
    let parsed = parse_ws_message(message).map_err(|error| JsError::new(&error.to_string()))?;
    serde_json::to_string(&parsed).map_err(|error| JsError::new(&error.to_string()))
}
