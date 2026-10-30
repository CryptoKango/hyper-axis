/// Supported candle subscription intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CandleInterval {
    /// One minute.
    #[serde(rename = "1m")]
    OneMinute,
    /// Three minutes.
    #[serde(rename = "3m")]
    ThreeMinutes,
    /// Five minutes.
    #[serde(rename = "5m")]
    FiveMinutes,
    /// Fifteen minutes.
    #[serde(rename = "15m")]
    FifteenMinutes,
    /// Thirty minutes.
    #[serde(rename = "30m")]
    ThirtyMinutes,
    /// One hour.
    #[serde(rename = "1h")]
    OneHour,
    /// Two hours.
    #[serde(rename = "2h")]
    TwoHours,
    /// Four hours.
    #[serde(rename = "4h")]
    FourHours,
    /// Eight hours.
    #[serde(rename = "8h")]
    EightHours,
    /// Twelve hours.
    #[serde(rename = "12h")]
    TwelveHours,
    /// One day.
    #[serde(rename = "1d")]
    OneDay,
    /// Three days.
    #[serde(rename = "3d")]
    ThreeDays,
    /// One week.
    #[serde(rename = "1w")]
    OneWeek,
    /// One month.
    #[serde(rename = "1M")]
    OneMonth,
}

/// Every currently documented Hyperliquid WebSocket subscription.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Subscription {
    /// All mid prices, optionally scoped to a perpetual DEX.
    #[serde(rename = "allMids")]
    AllMids {
        /// Perpetual DEX name.
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    /// User notifications.
    #[serde(rename = "notification")]
    Notification {
        /// User address.
        user: EvmAddress,
    },
    /// Aggregate frontend user state.
    #[serde(rename = "webData3")]
    WebData3 {
        /// User address.
        user: EvmAddress,
    },
    /// Native TWAP states.
    #[serde(rename = "twapStates")]
    TwapStates {
        /// User address.
        user: EvmAddress,
        /// Perpetual DEX name.
        dex: String,
    },
    /// Perpetual clearinghouse state.
    #[serde(rename = "clearinghouseState")]
    ClearinghouseState {
        /// User address.
        user: EvmAddress,
        /// Perpetual DEX name.
        dex: String,
    },
    /// Open orders.
    #[serde(rename = "openOrders")]
    OpenOrders {
        /// User address.
        user: EvmAddress,
        /// Perpetual DEX name.
        dex: String,
    },
    /// Candlestick stream.
    #[serde(rename = "candle")]
    Candle {
        /// Market symbol.
        coin: String,
        /// Candle interval.
        interval: CandleInterval,
    },
    /// L2 order book snapshots.
    #[serde(rename = "l2Book")]
    L2Book {
        /// Market symbol.
        coin: String,
        /// Optional significant-figure aggregation, from two through five.
        #[serde(rename = "nSigFigs", skip_serializing_if = "Option::is_none")]
        n_sig_figs: Option<u8>,
        /// Optional mantissa aggregation; only two or five with five figures.
        #[serde(skip_serializing_if = "Option::is_none")]
        mantissa: Option<u8>,
        /// Five levels when true, otherwise twenty.
        #[serde(skip_serializing_if = "Option::is_none")]
        fast: Option<bool>,
    },
    /// Public trades.
    #[serde(rename = "trades")]
    Trades {
        /// Market symbol.
        coin: String,
    },
    /// User order updates.
    #[serde(rename = "orderUpdates")]
    OrderUpdates {
        /// User address.
        user: EvmAddress,
    },
    /// Multiplexed user events.
    #[serde(rename = "userEvents")]
    UserEvents {
        /// User address.
        user: EvmAddress,
    },
    /// User fills.
    #[serde(rename = "userFills")]
    UserFills {
        /// User address.
        user: EvmAddress,
        /// Aggregate partial fills by time.
        #[serde(rename = "aggregateByTime", skip_serializing_if = "Option::is_none")]
        aggregate_by_time: Option<bool>,
    },
    /// User funding payments.
    #[serde(rename = "userFundings")]
    UserFundings {
        /// User address.
        user: EvmAddress,
    },
    /// User ledger updates excluding funding.
    #[serde(rename = "userNonFundingLedgerUpdates")]
    UserNonFundingLedgerUpdates {
        /// User address.
        user: EvmAddress,
    },
    /// Active market context.
    #[serde(rename = "activeAssetCtx")]
    ActiveAssetCtx {
        /// Market symbol.
        coin: String,
    },
    /// User-specific active perpetual market data.
    #[serde(rename = "activeAssetData")]
    ActiveAssetData {
        /// User address.
        user: EvmAddress,
        /// Perpetual market symbol.
        coin: String,
    },
    /// User TWAP slice fills.
    #[serde(rename = "userTwapSliceFills")]
    UserTwapSliceFills {
        /// User address.
        user: EvmAddress,
    },
    /// User TWAP history.
    #[serde(rename = "userTwapHistory")]
    UserTwapHistory {
        /// User address.
        user: EvmAddress,
    },
    /// Best bid and offer.
    #[serde(rename = "bbo")]
    Bbo {
        /// Market symbol.
        coin: String,
    },
    /// Spot account state.
    #[serde(rename = "spotState")]
    SpotState {
        /// User address.
        user: EvmAddress,
        /// Portfolio-margin view toggle.
        #[serde(rename = "isPortfolioMargin", skip_serializing_if = "Option::is_none")]
        is_portfolio_margin: Option<bool>,
    },
    /// Clearinghouse states across all perpetual DEXs.
    #[serde(rename = "allDexsClearinghouseState")]
    AllDexsClearinghouseState {
        /// User address.
        user: EvmAddress,
    },
    /// Asset contexts across all perpetual DEXs.
    #[serde(rename = "allDexsAssetCtxs")]
    AllDexsAssetCtxs,
    /// Outcome metadata changes.
    #[serde(rename = "outcomeMetaUpdates")]
    OutcomeMetaUpdates,
    /// Compressed fast asset context updates.
    #[serde(rename = "fastAssetCtxs")]
    FastAssetCtxs,
}

impl Subscription {
    /// Validates string and aggregation invariants before serialization.
    ///
    /// # Errors
    ///
    /// Returns an error for blank symbols/DEX names or unsupported L2
    /// aggregation combinations.
    pub fn validate(&self) -> Result<(), TransportError> {
        // `ClearinghouseState` intentionally accepts an empty DEX because it
        // is Hyperliquid's documented identifier for the first perpetual DEX.
        match self {
            Self::AllMids { dex: Some(dex) }
            | Self::TwapStates { dex, .. }
            | Self::OpenOrders { dex, .. } => validate_non_blank("dex", dex),
            Self::Candle { coin, .. }
            | Self::Trades { coin }
            | Self::ActiveAssetCtx { coin }
            | Self::ActiveAssetData { coin, .. }
            | Self::Bbo { coin } => validate_non_blank("coin", coin),
            Self::L2Book {
                coin,
                n_sig_figs,
                mantissa,
                ..
            } => {
                validate_non_blank("coin", coin)?;
                if n_sig_figs.is_some_and(|value| !(2..=5).contains(&value)) {
                    return Err(TransportError::InvalidSubscription(
                        "nSigFigs must be between 2 and 5",
                    ));
                }
                if mantissa.is_some_and(|value| value != 2 && value != 5) {
                    return Err(TransportError::InvalidSubscription(
                        "mantissa must be 2 or 5",
                    ));
                }
                if mantissa.is_some() && *n_sig_figs != Some(5) {
                    return Err(TransportError::InvalidSubscription(
                        "mantissa requires nSigFigs=5",
                    ));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Returns a deterministic identity used to deduplicate and unsubscribe.
    ///
    /// # Errors
    ///
    /// Returns an error when validation or serialization fails.
    pub fn key(&self) -> Result<String, TransportError> {
        self.validate()?;
        serde_json::to_string(self).map_err(TransportError::Json)
    }
}

/// Monotonic identifier for one in-flight WebSocket post request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PostRequestId(u32);

impl PostRequestId {
    /// Creates a request identifier safe in every JavaScript runtime.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the wire number.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// WebSocket post payload family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WsPostType {
    /// Read-only info request.
    Info,
    /// Signed exchange action.
    Action,
}

/// Inner WebSocket post request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsPostRequest {
    /// Request family.
    #[serde(rename = "type")]
    pub request_type: WsPostType,
    /// Info or signed-action payload.
    pub payload: Value,
}

/// Outbound WebSocket command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum WsCommand {
    /// Begin a subscription.
    Subscribe {
        /// Exact subscription descriptor.
        subscription: Subscription,
    },
    /// End a subscription using the exact original descriptor.
    Unsubscribe {
        /// Exact subscription descriptor.
        subscription: Subscription,
    },
    /// Application heartbeat.
    Ping,
    /// HTTP-compatible request over WebSocket.
    Post {
        /// Correlation identifier.
        id: PostRequestId,
        /// Wrapped request.
        request: WsPostRequest,
    },
}

impl WsCommand {
    /// Serializes a validated command for the host WebSocket.
    ///
    /// # Errors
    ///
    /// Returns an error when a subscription or post payload is invalid.
    pub fn to_json(&self) -> Result<String, TransportError> {
        match self {
            Self::Subscribe { subscription } | Self::Unsubscribe { subscription } => {
                subscription.validate()?;
            }
            Self::Post { request, .. } => {
                if !request.payload.is_object() {
                    return Err(TransportError::ExpectedObject("WebSocket post payload"));
                }
            }
            Self::Ping => {}
        }
        serde_json::to_string(self).map_err(TransportError::Json)
    }

    /// Creates a WebSocket info post from a raw JSON object.
    ///
    /// # Errors
    ///
    /// Returns an error when `payload` is not a JSON object.
    pub fn info_post(id: PostRequestId, payload: &str) -> Result<Self, TransportError> {
        Ok(Self::Post {
            id,
            request: WsPostRequest {
                request_type: WsPostType::Info,
                payload: parse_json_object("WebSocket info payload", payload)?,
            },
        })
    }

    /// Creates a WebSocket info post from a validated typed query.
    ///
    /// # Errors
    ///
    /// Returns an error when the query is invalid or cannot be encoded.
    pub fn typed_info_post(
        id: PostRequestId,
        query: info::InfoQuery,
    ) -> Result<Self, TransportError> {
        let prepared = info::prepare_info_query(query)?;
        Self::info_post(id, &prepared.body)
    }

    /// Creates a WebSocket action post from a finalized HTTP artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when the finalized endpoint/body is invalid.
    pub fn action_post(
        id: PostRequestId,
        finalized: &FinalizedAction,
    ) -> Result<Self, TransportError> {
        validate_exchange_url(&finalized.endpoint)?;
        Ok(Self::Post {
            id,
            request: WsPostRequest {
                request_type: WsPostType::Action,
                payload: parse_json_object("WebSocket action payload", &finalized.request_json)?,
            },
        })
    }
}

/// Parsed inbound WebSocket message with unknown data preserved.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    /// Channel name.
    pub channel: String,
    /// Channel-specific data, absent for `pong` on some server versions.
    #[serde(default)]
    pub data: Option<Value>,
}

/// Stable classification for dispatching inbound messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WsMessageKind {
    /// Heartbeat acknowledgement.
    Pong,
    /// Subscription acknowledgement.
    SubscriptionResponse,
    /// Response to a WebSocket post request.
    PostResponse,
    /// Subscription data event.
    SubscriptionData,
}

impl WsMessage {
    /// Returns the stable message class without discarding its channel name.
    #[must_use]
    pub fn kind(&self) -> WsMessageKind {
        match self.channel.as_str() {
            "pong" => WsMessageKind::Pong,
            "subscriptionResponse" => WsMessageKind::SubscriptionResponse,
            "post" => WsMessageKind::PostResponse,
            _ => WsMessageKind::SubscriptionData,
        }
    }

    /// Returns `isSnapshot` when the channel payload exposes that marker.
    #[must_use]
    pub fn is_snapshot(&self) -> bool {
        self.data
            .as_ref()
            .and_then(|value| value.get("isSnapshot"))
            .and_then(Value::as_bool)
            .unwrap_or(false)
    }

    /// Decodes a typed WebSocket post response.
    #[must_use]
    pub fn post_response(&self) -> Option<WsPostResponse> {
        (self.kind() == WsMessageKind::PostResponse)
            .then(|| self.data.clone())
            .flatten()
            .and_then(|data| serde_json::from_value(data).ok())
    }
}

/// Correlated response to a WebSocket post request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsPostResponse {
    /// Request correlation identifier.
    pub id: PostRequestId,
    /// Wrapped response.
    pub response: WsPostResponseBody,
}

/// Inner WebSocket post response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsPostResponseBody {
    /// `info`, `action`, or `error`.
    #[serde(rename = "type")]
    pub response_type: WsPostResponseType,
    /// Response body or error string, preserved without narrowing.
    pub payload: Value,
}

/// WebSocket post response family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WsPostResponseType {
    /// Info response.
    Info,
    /// Signed-action response.
    Action,
    /// HTTP-equivalent error.
    Error,
}

/// Parses one inbound WebSocket JSON object.
///
/// # Errors
///
/// Returns an error for malformed JSON, blank channels, or missing data on
/// channels that require it.
pub fn parse_ws_message(message: &str) -> Result<WsMessage, TransportError> {
    let parsed: WsMessage = serde_json::from_str(message).map_err(TransportError::Json)?;
    validate_non_blank("channel", &parsed.channel)?;
    if !matches!(parsed.kind(), WsMessageKind::Pong) && parsed.data.is_none() {
        return Err(TransportError::MissingWebSocketData);
    }
    Ok(parsed)
}
