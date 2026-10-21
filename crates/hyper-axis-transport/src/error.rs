/// Protocol construction and parsing failures.
#[derive(Debug, Error)]
pub enum TransportError {
    /// JSON encoding or decoding failed.
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// A request payload was valid JSON but not an object.
    #[error("{0} must be a JSON object")]
    ExpectedObject(&'static str),
    /// A signed action referenced an unexpected destination.
    #[error("unsupported exchange endpoint `{0}`")]
    InvalidExchangeEndpoint(String),
    /// Subscription parameters violate a documented invariant.
    #[error("invalid WebSocket subscription: {0}")]
    InvalidSubscription(&'static str),
    /// A typed Info request violates an address, range, or field invariant.
    #[error("invalid Info query: {0}")]
    InvalidInfoQuery(String),
    /// A typed Info response violates the documented response shape.
    #[error("invalid Info response: {0}")]
    InvalidInfoResponse(String),
    /// A required string was empty or whitespace-only.
    #[error("{0} must not be blank")]
    BlankField(&'static str),
    /// A non-pong message omitted its channel data.
    #[error("WebSocket message is missing channel data")]
    MissingWebSocketData,
}

fn validate_exchange_url(url: &str) -> Result<(), TransportError> {
    if [Network::Mainnet, Network::Testnet]
        .into_iter()
        .any(|network| network.exchange_url() == url)
    {
        Ok(())
    } else {
        Err(TransportError::InvalidExchangeEndpoint(url.to_owned()))
    }
}

fn validate_json_object(context: &'static str, value: &str) -> Result<(), TransportError> {
    parse_json_object(context, value).map(|_| ())
}

fn parse_json_object(context: &'static str, value: &str) -> Result<Value, TransportError> {
    let parsed = serde_json::from_str::<Value>(value)?;
    if parsed.is_object() {
        Ok(parsed)
    } else {
        Err(TransportError::ExpectedObject(context))
    }
}

fn validate_non_blank(field: &'static str, value: &str) -> Result<(), TransportError> {
    if value.trim().is_empty() {
        Err(TransportError::BlankField(field))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
