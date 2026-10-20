/// Hyperliquid HTTP endpoint family and its replay semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestKind {
    /// Read-only `POST /info`; safe to retry after transient failure.
    Info,
    /// Signed `POST /exchange`; outcome may be unknown after transport failure.
    SignedAction,
}

/// HTTP method used by protocol request descriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// HTTP POST.
    Post,
}

/// Runtime-neutral HTTP request for execution by a host application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    /// Endpoint family, also used to determine retry safety.
    pub kind: RequestKind,
    /// HTTP method.
    pub method: HttpMethod,
    /// Fully qualified official endpoint URL.
    pub url: String,
    /// Headers to attach exactly once.
    pub headers: BTreeMap<String, String>,
    /// Raw JSON body. The host must send this string without parsing it through
    /// a lossy JavaScript number representation.
    pub body: String,
}

/// Raw HTTP response supplied by a host transport.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    /// HTTP status code.
    pub status_code: u16,
    /// Raw response body.
    pub body: String,
}

/// High-level status of a parsed HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseKind {
    /// A 2xx response accepted by the endpoint.
    Success,
    /// A syntactically valid exchange envelope with `status: "err"`.
    ApiRejected,
    /// A 4xx client error other than rate limiting.
    ClientError,
    /// A 429 response.
    RateLimited,
    /// A 5xx response.
    ServerError,
    /// A nonstandard HTTP status code.
    UnexpectedStatus,
    /// The endpoint returned a body that cannot be decoded as JSON.
    InvalidJson,
    /// A signed-action 2xx body did not contain a valid exchange envelope.
    InvalidEnvelope,
}

/// Retry guidance that accounts for signed-action replay risk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RetryAdvice {
    /// Do not retry the same request.
    Never,
    /// A read-only request may be retried with exponential backoff and jitter.
    SafeAfterBackoff,
    /// The signed action may already have executed. Reconcile account/order
    /// state before deciding whether a new action with a new nonce is needed.
    ReconcileBeforeRetry,
}

/// Parsed HTTP result. Unknown payload fields are retained verbatim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedHttpResponse {
    /// Source endpoint family.
    pub request_kind: RequestKind,
    /// HTTP status code.
    pub status_code: u16,
    /// Response classification.
    pub kind: ResponseKind,
    /// Retry/reconciliation recommendation.
    pub retry: RetryAdvice,
    /// Parsed JSON body when decoding succeeded.
    pub payload: Option<Value>,
    /// Human-readable server or parser message, when available.
    pub message: Option<String>,
}

/// Validated Hyperliquid exchange response envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeResponse {
    /// Server status.
    pub status: ExchangeStatus,
    /// Operation-specific response payload, preserved for forward compatibility.
    pub response: Value,
}

/// Status discriminator returned by `POST /exchange`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExchangeStatus {
    /// Request accepted and processed by the exchange API.
    Ok,
    /// Request explicitly rejected by the exchange API.
    Err,
}

impl ExchangeResponse {
    /// Returns the operation type inside the response payload, when present.
    #[must_use]
    pub fn response_type(&self) -> Option<&str> {
        self.response.get("type").and_then(Value::as_str)
    }

    /// Returns batch statuses without interpreting evolving status variants.
    #[must_use]
    pub fn statuses(&self) -> Option<&[Value]> {
        self.response
            .get("data")
            .and_then(|data| data.get("statuses"))
            .and_then(Value::as_array)
            .map(Vec::as_slice)
    }
}

/// Transport failures reported by the host before an HTTP response exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HostFailureKind {
    /// Request deadline expired.
    Timeout,
    /// Connection could not be established or was reset.
    Connection,
    /// DNS resolution failed.
    Dns,
    /// TLS negotiation or certificate validation failed.
    Tls,
    /// Host cancelled the request intentionally.
    Cancelled,
    /// Other host transport failure.
    Other,
}

/// Builds a read-only `POST /info` request while preserving the exact body.
///
/// # Errors
///
/// Returns an error when `body` is not a JSON object.
pub fn info_http_request(network: Network, body: &str) -> Result<HttpRequest, TransportError> {
    validate_json_object("info request", body)?;
    Ok(json_post_request(
        RequestKind::Info,
        network.info_url(),
        body,
    ))
}

/// Builds a read-only `POST /info` request from a validated typed query.
///
/// # Errors
///
/// Returns an error when the query violates a protocol invariant or cannot be
/// encoded as JSON.
pub fn typed_info_http_request(
    network: Network,
    query: info::InfoQuery,
) -> Result<HttpRequest, TransportError> {
    let prepared = info::prepare_info_query(query)?;
    info_http_request(network, &prepared.body)
}

/// Builds a signed `POST /exchange` request from a finalized action.
///
/// The raw finalized body is never reserialized, preserving `u64` values that
/// would be lossy if a JavaScript host parsed them as `Number` first.
///
/// # Errors
///
/// Returns an error when the finalized body is not a JSON object or the URL is
/// not an official exchange endpoint.
pub fn exchange_http_request(finalized: &FinalizedAction) -> Result<HttpRequest, TransportError> {
    validate_exchange_url(&finalized.endpoint)?;
    validate_json_object("exchange request", &finalized.request_json)?;
    Ok(json_post_request(
        RequestKind::SignedAction,
        &finalized.endpoint,
        &finalized.request_json,
    ))
}

fn json_post_request(kind: RequestKind, url: &str, body: &str) -> HttpRequest {
    HttpRequest {
        kind,
        method: HttpMethod::Post,
        url: url.to_owned(),
        headers: BTreeMap::from([("content-type".to_owned(), APPLICATION_JSON.to_owned())]),
        body: body.to_owned(),
    }
}

/// Parses and classifies a host HTTP response.
#[must_use]
pub fn parse_http_response(kind: RequestKind, response: &HttpResponse) -> ParsedHttpResponse {
    let parsed = serde_json::from_str::<Value>(&response.body);
    if !(200..300).contains(&response.status_code) {
        return classify_non_success(kind, response.status_code, parsed.ok(), &response.body);
    }

    let Ok(payload) = parsed else {
        return ParsedHttpResponse {
            request_kind: kind,
            status_code: response.status_code,
            kind: ResponseKind::InvalidJson,
            retry: ambiguous_retry(kind),
            payload: None,
            message: Some("response body is not valid JSON".to_owned()),
        };
    };

    if kind == RequestKind::Info {
        return ParsedHttpResponse {
            request_kind: kind,
            status_code: response.status_code,
            kind: ResponseKind::Success,
            retry: RetryAdvice::Never,
            payload: Some(payload),
            message: None,
        };
    }

    match serde_json::from_value::<ExchangeResponse>(payload.clone()) {
        Ok(exchange) => {
            let (response_kind, message) = match exchange.status {
                ExchangeStatus::Ok => (ResponseKind::Success, None),
                ExchangeStatus::Err => (
                    ResponseKind::ApiRejected,
                    message_from_value(&exchange.response),
                ),
            };
            ParsedHttpResponse {
                request_kind: kind,
                status_code: response.status_code,
                kind: response_kind,
                retry: RetryAdvice::Never,
                payload: Some(payload),
                message,
            }
        }
        Err(error) => ParsedHttpResponse {
            request_kind: kind,
            status_code: response.status_code,
            kind: ResponseKind::InvalidEnvelope,
            retry: RetryAdvice::ReconcileBeforeRetry,
            payload: Some(payload),
            message: Some(format!("invalid exchange response envelope: {error}")),
        },
    }
}

/// Returns retry guidance for a failure that produced no HTTP response.
#[must_use]
pub const fn classify_host_failure(kind: RequestKind, failure: HostFailureKind) -> RetryAdvice {
    if matches!(failure, HostFailureKind::Cancelled) && matches!(kind, RequestKind::Info) {
        RetryAdvice::Never
    } else {
        ambiguous_retry(kind)
    }
}

const fn ambiguous_retry(kind: RequestKind) -> RetryAdvice {
    match kind {
        RequestKind::Info => RetryAdvice::SafeAfterBackoff,
        RequestKind::SignedAction => RetryAdvice::ReconcileBeforeRetry,
    }
}

fn classify_non_success(
    request_kind: RequestKind,
    status_code: u16,
    payload: Option<Value>,
    raw_body: &str,
) -> ParsedHttpResponse {
    let response_kind = match status_code {
        429 => ResponseKind::RateLimited,
        400..=499 => ResponseKind::ClientError,
        500..=599 => ResponseKind::ServerError,
        _ => ResponseKind::UnexpectedStatus,
    };
    let retry = match response_kind {
        ResponseKind::RateLimited | ResponseKind::ServerError => ambiguous_retry(request_kind),
        ResponseKind::ClientError if status_code == 408 => ambiguous_retry(request_kind),
        _ => RetryAdvice::Never,
    };
    let message = payload
        .as_ref()
        .and_then(server_error_message)
        .or_else(|| (!raw_body.is_empty()).then(|| raw_body.to_owned()));
    ParsedHttpResponse {
        request_kind,
        status_code,
        kind: response_kind,
        retry,
        payload,
        message,
    }
}

fn server_error_message(value: &Value) -> Option<String> {
    value
        .get("msg")
        .or_else(|| value.get("message"))
        .or_else(|| value.get("error"))
        .and_then(message_from_value)
}

fn message_from_value(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(str::to_owned)
        .or_else(|| (!value.is_null()).then(|| value.to_string()))
}
