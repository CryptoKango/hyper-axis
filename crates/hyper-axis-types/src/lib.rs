//! Shared protocol types used by every `HyperAxis` crate.

use core::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

include!("network.rs");
include!("identifiers.rs");
include!("error.rs");
