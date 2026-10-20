//! Runtime-configurable structured diagnostics for native and web clients.
//!
//! Logging is disabled by default. Records are held in a bounded in-memory
//! ring buffer and can be drained by FFI or WebAssembly consumers. Internal
//! events intentionally exclude private keys, signatures, addresses, nonces,
//! prices, sizes, and complete request or response payloads.

use std::{
    collections::VecDeque,
    str::FromStr,
    sync::{
        Mutex, MutexGuard, OnceLock,
        atomic::{AtomicBool, AtomicU8, AtomicU64, AtomicUsize, Ordering},
    },
};

use serde::Serialize;
use thiserror::Error;

/// Default maximum number of retained records.
pub const DEFAULT_CAPACITY: usize = 500;
/// Hard limit protecting clients from unbounded diagnostic memory usage.
pub const MAX_CAPACITY: usize = 10_000;
const MAX_TARGET_BYTES: usize = 128;
const MAX_MESSAGE_BYTES: usize = 4_096;

include!("state.rs");
include!("models.rs");
include!("logger.rs");
