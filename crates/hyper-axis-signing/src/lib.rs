//! Signing metadata shared with host-provided EIP-712 signers.
//!
//! `HyperAxis` does not own or persist private keys. This crate describes the
//! payload a platform signer must sign and keeps protocol constants in one
//! auditable location.

use hyper_axis_types::{EvmAddress, Network};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use serde::{Deserialize, Deserializer, Serialize, de};
use sha3::{Digest, Keccak256};
use thiserror::Error;

/// EIP-712 chain ID used for L1 phantom-agent signatures.
pub const L1_SIGNATURE_CHAIN_ID: u64 = 1337;
/// Chain ID used by user-signed actions (`0x66eee`).
pub const USER_SIGNATURE_CHAIN_ID: u64 = 421_614;
/// Zero address used as the verifying contract in signing domains.
pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

include!("models.rs");
include!("l1.rs");
include!("user.rs");
include!("multisig.rs");
include!("eip712.rs");
