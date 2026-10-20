//! Strict JSON helpers shared by the coarse-grained bridge API.

use serde::{Serialize, de::DeserializeOwned};

pub(crate) fn decode<T: DeserializeOwned>(json: &str, context: &str) -> Result<T, String> {
    serde_json::from_str(json).map_err(|error| format!("invalid {context} JSON: {error}"))
}

pub(crate) fn encode<T: Serialize>(value: &T, context: &str) -> Result<String, String> {
    serde_json::to_string(value)
        .map_err(|error| format!("unable to encode {context} JSON: {error}"))
}
