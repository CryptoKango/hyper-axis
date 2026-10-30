//! Strongly typed request models for the read-only Hyperliquid Info API.

mod models;

pub use models::{
    MarginTable, MarginTier, PerpAssetContext, PerpAssetPosition, PerpClearinghouseState,
    PerpCumulativeFunding, PerpDex, PerpLeverage, PerpMarginSummary, PerpMarketMeta, PerpMeta,
    PerpMetaAndAssetCtxs, PerpPosition, SpotBalance, SpotClearinghouseState,
    parse_perp_clearinghouse_state_response, parse_perp_dexs_response,
    parse_perp_meta_and_asset_ctxs_response, parse_perp_meta_response,
    parse_spot_clearinghouse_state_response,
};

use hyper_axis_types::{ClientOrderId, EvmAddress};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{CandleInterval, TransportError, validate_non_blank};

/// Order identifier accepted by the `orderStatus` query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum OrderStatusSelector {
    /// Exchange-assigned numeric order identifier.
    Oid {
        /// Unsigned 64-bit OID represented as a string at cross-language boundaries.
        #[serde(with = "u64_string")]
        oid: u64,
    },
    /// Client-assigned 128-bit hexadecimal order identifier.
    Cloid {
        /// Canonical `0x`-prefixed Cloid.
        cloid: String,
    },
}

/// Complete request surface exposed by the official Python `Info` client.
///
/// Millisecond timestamps and OIDs are strings in serialized input so web
/// clients cannot lose integer precision. [`prepare_info_query`] emits the
/// numeric wire representation required by `/info`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum InfoQuery {
    /// Perpetual clearinghouse state.
    ClearinghouseState { user: String, dex: String },
    /// Spot clearinghouse state.
    SpotClearinghouseState { user: String },
    /// Basic open orders.
    OpenOrders { user: String, dex: String },
    /// Open orders enriched for frontend display.
    FrontendOpenOrders { user: String, dex: String },
    /// Mid prices for active markets.
    AllMids { dex: String },
    /// Recent user fills.
    UserFills { user: String },
    /// User fills within an inclusive time range.
    UserFillsByTime {
        user: String,
        #[serde(with = "u64_string")]
        start_time: u64,
        #[serde(default, with = "option_u64_string")]
        end_time: Option<u64>,
        aggregate_by_time: bool,
    },
    /// Perpetual metadata for one DEX.
    Meta { dex: String },
    /// Perpetual metadata and live asset contexts, optionally scoped to a HIP-3 DEX.
    MetaAndAssetCtxs {
        /// HIP-3 DEX name. Omit for the native perpetual DEX.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    /// Registered perpetual DEXs.
    PerpDexs,
    /// Spot metadata.
    SpotMeta,
    /// Spot metadata and live asset contexts.
    SpotMetaAndAssetCtxs,
    /// Public funding history.
    FundingHistory {
        coin: String,
        #[serde(with = "u64_string")]
        start_time: u64,
        #[serde(default, with = "option_u64_string")]
        end_time: Option<u64>,
    },
    /// User funding history.
    UserFunding {
        user: String,
        #[serde(with = "u64_string")]
        start_time: u64,
        #[serde(default, with = "option_u64_string")]
        end_time: Option<u64>,
    },
    /// L2 order-book snapshot.
    L2Book {
        coin: String,
        n_sig_figs: Option<u8>,
        mantissa: Option<u8>,
    },
    /// Historical candles.
    CandleSnapshot {
        coin: String,
        interval: CandleInterval,
        #[serde(with = "u64_string")]
        start_time: u64,
        #[serde(with = "u64_string")]
        end_time: u64,
    },
    /// User fee schedule and volume.
    UserFees { user: String },
    /// Delegator summary.
    DelegatorSummary { user: String },
    /// Active delegations.
    Delegations { user: String },
    /// Delegator rewards.
    DelegatorRewards { user: String },
    /// Delegation and undelegation history.
    DelegatorHistory { user: String },
    /// Status of an order selected by OID or Cloid.
    OrderStatus {
        user: String,
        selector: OrderStatusSelector,
    },
    /// Referral state.
    Referral { user: String },
    /// Subaccounts owned by a user.
    SubAccounts { user: String },
    /// Multi-signature signers for a user.
    UserToMultiSigSigners { user: String },
    /// Perpetual deploy auction status.
    PerpDeployAuctionStatus,
    /// Legacy DEX-abstraction state.
    UserDexAbstraction { user: String },
    /// Account-abstraction state.
    UserAbstraction { user: String },
    /// Recent historical orders.
    HistoricalOrders { user: String },
    /// Non-funding ledger updates within a time range.
    UserNonFundingLedgerUpdates {
        user: String,
        #[serde(with = "u64_string")]
        start_time: u64,
        #[serde(default, with = "option_u64_string")]
        end_time: Option<u64>,
    },
    /// Portfolio performance history.
    Portfolio { user: String },
    /// TWAP slice fills.
    UserTwapSliceFills { user: String },
    /// User equity across vaults.
    UserVaultEquities { user: String },
    /// User role and account type.
    UserRole { user: String },
    /// User API rate-limit state.
    UserRateLimit { user: String },
    /// Spot deploy auction state.
    SpotDeployState { user: String },
    /// Additional API agents.
    ExtraAgents { user: String },
}

/// Canonical Info query body ready for HTTP or WebSocket transport.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedInfoQuery {
    /// Wire request discriminator.
    pub query_type: String,
    /// Exact JSON object sent as the Info payload.
    pub body: String,
}

/// Validates and canonicalizes a typed Info query.
///
/// # Errors
///
/// Returns an error for malformed addresses/Cloids, invalid time ranges,
/// blank market symbols, unsupported L2 aggregation, or JSON encoding.
#[allow(clippy::too_many_lines)]
pub fn prepare_info_query(query: InfoQuery) -> Result<PreparedInfoQuery, TransportError> {
    let query_type = query.query_type().to_owned();
    let body = match query {
        InfoQuery::ClearinghouseState { user, dex }
        | InfoQuery::OpenOrders { user, dex }
        | InfoQuery::FrontendOpenOrders { user, dex } => {
            json!({"type": query_type, "user": address(&user)?, "dex": dex})
        }
        InfoQuery::SpotClearinghouseState { user }
        | InfoQuery::UserFills { user }
        | InfoQuery::UserFees { user }
        | InfoQuery::DelegatorSummary { user }
        | InfoQuery::Delegations { user }
        | InfoQuery::DelegatorRewards { user }
        | InfoQuery::DelegatorHistory { user }
        | InfoQuery::Referral { user }
        | InfoQuery::SubAccounts { user }
        | InfoQuery::UserToMultiSigSigners { user }
        | InfoQuery::UserDexAbstraction { user }
        | InfoQuery::UserAbstraction { user }
        | InfoQuery::HistoricalOrders { user }
        | InfoQuery::Portfolio { user }
        | InfoQuery::UserTwapSliceFills { user }
        | InfoQuery::UserVaultEquities { user }
        | InfoQuery::UserRole { user }
        | InfoQuery::UserRateLimit { user }
        | InfoQuery::SpotDeployState { user }
        | InfoQuery::ExtraAgents { user } => {
            json!({"type": query_type, "user": address(&user)?})
        }
        InfoQuery::AllMids { dex } | InfoQuery::Meta { dex } => {
            json!({"type": query_type, "dex": dex})
        }
        InfoQuery::UserFillsByTime {
            user,
            start_time,
            end_time,
            aggregate_by_time,
        } => {
            validate_time_range(start_time, end_time)?;
            json!({
                "type": query_type,
                "user": address(&user)?,
                "startTime": start_time,
                "endTime": end_time,
                "aggregateByTime": aggregate_by_time,
            })
        }
        InfoQuery::MetaAndAssetCtxs { dex } => {
            if let Some(dex) = dex {
                validate_non_blank("dex", &dex)?;
                json!({"type": query_type, "dex": dex})
            } else {
                json!({"type": query_type})
            }
        }
        InfoQuery::PerpDexs
        | InfoQuery::SpotMeta
        | InfoQuery::SpotMetaAndAssetCtxs
        | InfoQuery::PerpDeployAuctionStatus => json!({"type": query_type}),
        InfoQuery::FundingHistory {
            coin,
            start_time,
            end_time,
        } => time_range_query(&query_type, None, Some(coin), start_time, end_time)?,
        InfoQuery::UserFunding {
            user,
            start_time,
            end_time,
        } => time_range_query(&query_type, Some(user), None, start_time, end_time)?,
        InfoQuery::L2Book {
            coin,
            n_sig_figs,
            mantissa,
        } => l2_book_query(&query_type, &coin, n_sig_figs, mantissa)?,
        InfoQuery::CandleSnapshot {
            coin,
            interval,
            start_time,
            end_time,
        } => {
            validate_non_blank("coin", &coin)?;
            validate_time_range(start_time, Some(end_time))?;
            json!({
                "type": query_type,
                "req": {
                    "coin": coin,
                    "interval": interval,
                    "startTime": start_time,
                    "endTime": end_time,
                },
            })
        }
        InfoQuery::OrderStatus { user, selector } => {
            let oid = match selector {
                OrderStatusSelector::Oid { oid } => Value::from(oid),
                OrderStatusSelector::Cloid { cloid } => Value::from(
                    cloid
                        .parse::<ClientOrderId>()
                        .map_err(|error| TransportError::InvalidInfoQuery(error.to_string()))?
                        .to_string(),
                ),
            };
            json!({"type": query_type, "user": address(&user)?, "oid": oid})
        }
        InfoQuery::UserNonFundingLedgerUpdates {
            user,
            start_time,
            end_time,
        } => {
            validate_time_range(start_time, end_time)?;
            json!({
                "type": query_type,
                "user": address(&user)?,
                "startTime": start_time,
                "endTime": end_time,
            })
        }
    };
    Ok(PreparedInfoQuery {
        query_type,
        body: serde_json::to_string(&body)?,
    })
}

impl InfoQuery {
    /// Returns the exact Info API discriminator.
    #[must_use]
    pub const fn query_type(&self) -> &'static str {
        match self {
            Self::ClearinghouseState { .. } => "clearinghouseState",
            Self::SpotClearinghouseState { .. } => "spotClearinghouseState",
            Self::OpenOrders { .. } => "openOrders",
            Self::FrontendOpenOrders { .. } => "frontendOpenOrders",
            Self::AllMids { .. } => "allMids",
            Self::UserFills { .. } => "userFills",
            Self::UserFillsByTime { .. } => "userFillsByTime",
            Self::Meta { .. } => "meta",
            Self::MetaAndAssetCtxs { .. } => "metaAndAssetCtxs",
            Self::PerpDexs => "perpDexs",
            Self::SpotMeta => "spotMeta",
            Self::SpotMetaAndAssetCtxs => "spotMetaAndAssetCtxs",
            Self::FundingHistory { .. } => "fundingHistory",
            Self::UserFunding { .. } => "userFunding",
            Self::L2Book { .. } => "l2Book",
            Self::CandleSnapshot { .. } => "candleSnapshot",
            Self::UserFees { .. } => "userFees",
            Self::DelegatorSummary { .. } => "delegatorSummary",
            Self::Delegations { .. } => "delegations",
            Self::DelegatorRewards { .. } => "delegatorRewards",
            Self::DelegatorHistory { .. } => "delegatorHistory",
            Self::OrderStatus { .. } => "orderStatus",
            Self::Referral { .. } => "referral",
            Self::SubAccounts { .. } => "subAccounts",
            Self::UserToMultiSigSigners { .. } => "userToMultiSigSigners",
            Self::PerpDeployAuctionStatus => "perpDeployAuctionStatus",
            Self::UserDexAbstraction { .. } => "userDexAbstraction",
            Self::UserAbstraction { .. } => "userAbstraction",
            Self::HistoricalOrders { .. } => "historicalOrders",
            Self::UserNonFundingLedgerUpdates { .. } => "userNonFundingLedgerUpdates",
            Self::Portfolio { .. } => "portfolio",
            Self::UserTwapSliceFills { .. } => "userTwapSliceFills",
            Self::UserVaultEquities { .. } => "userVaultEquities",
            Self::UserRole { .. } => "userRole",
            Self::UserRateLimit { .. } => "userRateLimit",
            Self::SpotDeployState { .. } => "spotDeployState",
            Self::ExtraAgents { .. } => "extraAgents",
        }
    }
}

fn address(value: &str) -> Result<String, TransportError> {
    value
        .parse::<EvmAddress>()
        .map(|address| address.to_string())
        .map_err(|error| TransportError::InvalidInfoQuery(error.to_string()))
}

fn validate_time_range(start_time: u64, end_time: Option<u64>) -> Result<(), TransportError> {
    if end_time.is_some_and(|end_time| end_time < start_time) {
        return Err(TransportError::InvalidInfoQuery(
            "endTime must not precede startTime".to_owned(),
        ));
    }
    Ok(())
}

fn time_range_query(
    query_type: &str,
    user: Option<String>,
    coin: Option<String>,
    start_time: u64,
    end_time: Option<u64>,
) -> Result<Value, TransportError> {
    validate_time_range(start_time, end_time)?;
    let mut value = json!({"type": query_type, "startTime": start_time});
    let object = value.as_object_mut().expect("JSON object literal");
    if let Some(user) = user {
        object.insert("user".to_owned(), Value::from(address(&user)?));
    }
    if let Some(coin) = coin {
        validate_non_blank("coin", &coin)?;
        object.insert("coin".to_owned(), Value::from(coin));
    }
    if let Some(end_time) = end_time {
        object.insert("endTime".to_owned(), Value::from(end_time));
    }
    Ok(value)
}

fn l2_book_query(
    query_type: &str,
    coin: &str,
    n_sig_figs: Option<u8>,
    mantissa: Option<u8>,
) -> Result<Value, TransportError> {
    validate_non_blank("coin", coin)?;
    if n_sig_figs.is_some_and(|value| !(2..=5).contains(&value)) {
        return Err(TransportError::InvalidInfoQuery(
            "nSigFigs must be between 2 and 5".to_owned(),
        ));
    }
    if mantissa.is_some_and(|value| value != 2 && value != 5) {
        return Err(TransportError::InvalidInfoQuery(
            "mantissa must be 2 or 5".to_owned(),
        ));
    }
    if mantissa.is_some() && n_sig_figs != Some(5) {
        return Err(TransportError::InvalidInfoQuery(
            "mantissa requires nSigFigs=5".to_owned(),
        ));
    }
    let mut value = json!({"type": query_type, "coin": coin});
    let object = value.as_object_mut().expect("JSON object literal");
    if let Some(n_sig_figs) = n_sig_figs {
        object.insert("nSigFigs".to_owned(), Value::from(n_sig_figs));
    }
    if let Some(mantissa) = mantissa {
        object.insert("mantissa".to_owned(), Value::from(mantissa));
    }
    Ok(value)
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
#[path = "info_tests.rs"]
mod tests;
