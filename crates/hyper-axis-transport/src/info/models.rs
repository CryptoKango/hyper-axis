//! Strongly typed HIP-3/perpetual Info API response models.

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::TransportError;

/// One builder-deployed perpetual DEX returned by `perpDexs`.
///
/// The response array can contain `null` entries. Callers must retain the
/// array index because it participates in HIP-3 asset ID calculation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpDex {
    /// Protocol DEX identifier used in Info queries and market prefixes.
    pub name: String,
    /// Human-readable venue name.
    pub full_name: String,
    /// Account that deployed the DEX.
    pub deployer: String,
    /// Account authorized to update oracle prices, when separately configured.
    pub oracle_updater: Option<String>,
    /// Account receiving deployer fees, when configured.
    pub fee_recipient: Option<String>,
    /// Market name and streaming open-interest cap pairs.
    #[serde(default)]
    pub asset_to_streaming_oi_cap: Vec<(String, String)>,
    /// Permission name and authorized sub-deployer accounts.
    #[serde(default)]
    pub sub_deployers: Vec<(String, Vec<String>)>,
    /// Market name and funding multiplier pairs.
    #[serde(default)]
    pub asset_to_funding_multiplier: Vec<(String, String)>,
    /// Market name and funding interest-rate pairs.
    #[serde(default)]
    pub asset_to_funding_interest_rate: Vec<(String, String)>,
    /// Market name and funding clamp pairs.
    #[serde(default)]
    pub asset_to_funding_clamp: Vec<(String, String)>,
    /// Forward-compatible fields added by Hyperliquid.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Perpetual metadata returned by `meta` and the first element of
/// `metaAndAssetCtxs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpMeta {
    /// Markets in protocol index order.
    pub universe: Vec<PerpMarketMeta>,
    /// Collateral token index for builder-deployed DEXs.
    #[serde(default)]
    pub collateral_token: Option<u32>,
    /// Margin table ID and definition pairs.
    #[serde(default)]
    pub margin_tables: Vec<(u32, MarginTable)>,
    /// Forward-compatible fields added by Hyperliquid.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Static metadata for one perpetual market.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpMarketMeta {
    /// Canonical market name, such as `BTC` or `xyz:TSLA`.
    pub name: String,
    /// Number of decimal places allowed for order size.
    pub sz_decimals: u32,
    /// Maximum permitted leverage.
    pub max_leverage: u32,
    /// Whether the market only supports isolated margin.
    #[serde(default)]
    pub only_isolated: Option<bool>,
    /// Margin table selected by this market.
    #[serde(default)]
    pub margin_table_id: Option<u32>,
    /// Whether trading has been delisted.
    #[serde(default)]
    pub is_delisted: Option<bool>,
    /// HIP-3 growth mode, when present.
    #[serde(default)]
    pub growth_mode: Option<String>,
    /// Server timestamp for the latest deployer fee scale change.
    #[serde(default)]
    pub last_fee_scale_change_time: Option<String>,
    /// Current deployer fee scale represented as a decimal string.
    #[serde(default)]
    pub deployer_fee_scale: Option<String>,
    /// Forward-compatible fields added by Hyperliquid.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl PerpMarketMeta {
    /// Splits a HIP-3 market name into `(dex, symbol)`.
    ///
    /// Native perpetual names do not contain a DEX prefix and return `None`.
    #[must_use]
    pub fn hip3_parts(&self) -> Option<(&str, &str)> {
        self.name
            .split_once(':')
            .filter(|(dex, symbol)| !dex.is_empty() && !symbol.is_empty())
    }
}

/// One cross-margin tier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTier {
    /// Lower notional bound represented as a decimal string.
    pub lower_bound: String,
    /// Maximum leverage inside the tier.
    pub max_leverage: u32,
    /// Forward-compatible fields added by Hyperliquid.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Margin rules referenced by one or more perpetual markets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTable {
    /// Human-readable table description.
    #[serde(default)]
    pub description: String,
    /// Notional tiers in ascending order.
    #[serde(default)]
    pub margin_tiers: Vec<MarginTier>,
    /// Forward-compatible fields added by Hyperliquid.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Live context for one perpetual market.
///
/// Decimal values remain strings so mobile and web hosts cannot lose
/// precision by passing them through binary floating point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerpAssetContext {
    /// Current funding rate.
    pub funding: String,
    /// Current open interest.
    pub open_interest: String,
    /// Previous daily reference price.
    pub prev_day_px: String,
    /// Rolling daily notional volume.
    pub day_ntl_vlm: String,
    /// Current oracle price.
    pub oracle_px: String,
    /// Current mark price.
    pub mark_px: String,
    /// Current premium, when supplied.
    #[serde(default)]
    pub premium: Option<String>,
    /// Current midpoint, when a two-sided market exists.
    #[serde(default)]
    pub mid_px: Option<String>,
    /// Bid and ask impact prices, when available.
    #[serde(default)]
    pub impact_pxs: Option<Vec<String>>,
    /// Rolling daily base volume, when supplied.
    #[serde(default)]
    pub day_base_vlm: Option<String>,
    /// Forward-compatible fields added by Hyperliquid.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Validated `metaAndAssetCtxs` tuple with aligned market metadata and context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerpMetaAndAssetCtxs {
    /// Static perpetual metadata.
    pub meta: PerpMeta,
    /// Live contexts in the same order as `meta.universe`.
    pub asset_contexts: Vec<PerpAssetContext>,
}

impl Serialize for PerpMetaAndAssetCtxs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.meta, &self.asset_contexts).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PerpMetaAndAssetCtxs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (meta, asset_contexts) =
            <(PerpMeta, Vec<PerpAssetContext>)>::deserialize(deserializer)?;
        Ok(Self {
            meta,
            asset_contexts,
        })
    }
}

impl PerpMetaAndAssetCtxs {
    /// Iterates over metadata and live contexts in protocol index order.
    pub fn markets(&self) -> impl ExactSizeIterator<Item = (&PerpMarketMeta, &PerpAssetContext)> {
        self.meta.universe.iter().zip(&self.asset_contexts)
    }
}

/// Parses the nullable, index-sensitive `perpDexs` response.
///
/// # Errors
///
/// Returns an error when the response does not match the documented schema.
pub fn parse_perp_dexs_response(body: &str) -> Result<Vec<Option<PerpDex>>, TransportError> {
    serde_json::from_str(body).map_err(Into::into)
}

/// Parses one native or HIP-3 `meta` response.
///
/// # Errors
///
/// Returns an error when the response does not match the documented schema.
pub fn parse_perp_meta_response(body: &str) -> Result<PerpMeta, TransportError> {
    serde_json::from_str(body).map_err(Into::into)
}

/// Parses and validates a native or HIP-3 `metaAndAssetCtxs` response.
///
/// # Errors
///
/// Returns an error when the response shape is invalid or metadata and live
/// context counts differ.
pub fn parse_perp_meta_and_asset_ctxs_response(
    body: &str,
) -> Result<PerpMetaAndAssetCtxs, TransportError> {
    let response = serde_json::from_str::<PerpMetaAndAssetCtxs>(body)?;
    let market_count = response.meta.universe.len();
    let context_count = response.asset_contexts.len();
    if market_count != context_count {
        return Err(TransportError::InvalidInfoResponse(format!(
            "meta universe contains {market_count} markets but asset contexts contain {context_count} entries"
        )));
    }
    Ok(response)
}
