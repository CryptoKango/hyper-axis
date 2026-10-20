//! Typed action models matching the Hyperliquid exchange wire format.

use hyper_axis_decimal::DecimalValue;
use hyper_axis_types::{AssetId, ClientOrderId, EvmAddress, ProtocolError};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

include!("orders.rs");
include!("deploy.rs");
include!("validators.rs");
include!("user.rs");
include!("validation.rs");
