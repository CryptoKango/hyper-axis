//! Canonical `MessagePack` encoding and exchange action hashing.

use hyper_axis_logging::{LogLevel, emit};
use hyper_axis_types::EvmAddress;
use serde::Serialize;
use sha3::{Digest, Keccak256};
use thiserror::Error;

include!("error.rs");
include!("encoding.rs");
