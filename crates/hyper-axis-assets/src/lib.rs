//! Hyperliquid asset identifiers and market precision constraints.

use hyper_axis_decimal::DecimalValue;
use hyper_axis_types::{AssetId, ProtocolError};
use serde::{Deserialize, Serialize};

const SPOT_OFFSET: u32 = 10_000;
const HIP3_OFFSET: u32 = 100_000;
const HIP3_DEX_STRIDE: u32 = 10_000;
const MAX_SIGNIFICANT_PRICE_DIGITS: usize = 5;

include!("asset_id.rs");
include!("precision.rs");
