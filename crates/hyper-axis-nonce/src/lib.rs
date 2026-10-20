//! Per-signer nonce coordination for Hyperliquid exchange actions.
//!
//! One [`NonceManager`] must be owned by exactly one signing identity (master
//! account or API wallet). Hyperliquid tracks nonces per signer, even when the
//! same signer trades for multiple vaults or subaccounts. This crate therefore
//! deliberately has no process-global nonce singleton.

use std::sync::atomic::{AtomicU64, Ordering};

use hyper_axis_logging::{self as logging, LogLevel};
use thiserror::Error;

/// Maximum accepted age of a nonce relative to server time: two days.
pub const MAX_PAST_AGE_MS: u64 = 2 * 24 * 60 * 60 * 1_000;
/// Maximum accepted future offset of a nonce relative to server time: one day.
pub const MAX_FUTURE_OFFSET_MS: u64 = 24 * 60 * 60 * 1_000;

include!("manager.rs");
include!("error.rs");
