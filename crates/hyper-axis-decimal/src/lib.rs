//! Canonical, lossless decimal strings for protocol fields.

use core::{fmt, str::FromStr};

use hyper_axis_types::ProtocolError;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

include!("value.rs");
