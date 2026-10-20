//! Runtime-independent HTTP and WebSocket protocol models.
//!
//! This crate deliberately performs no network I/O. Native, Dart, and browser
//! hosts own sockets, TLS, cancellation, and scheduling while sharing the same
//! request construction, response parsing, and retry rules.

pub mod info;

use std::collections::BTreeMap;

use hyper_axis_client::FinalizedAction;
use hyper_axis_types::{EvmAddress, Network};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// Media type required by the Hyperliquid HTTP API.
pub const APPLICATION_JSON: &str = "application/json";
/// Heartbeat cadence used by the official SDK.
pub const WS_HEARTBEAT_INTERVAL_MS: u32 = 50_000;
/// Server-side idle timeout documented by Hyperliquid.
pub const WS_SERVER_IDLE_TIMEOUT_MS: u32 = 60_000;
/// Maximum simultaneous WebSocket connections per IP.
pub const WS_MAX_CONNECTIONS: u16 = 10;
/// Maximum new WebSocket connections per minute per IP.
pub const WS_MAX_NEW_CONNECTIONS_PER_MINUTE: u16 = 30;
/// Maximum subscriptions across all WebSocket connections per IP.
pub const WS_MAX_SUBSCRIPTIONS: u16 = 1_000;
/// Maximum distinct users across user-specific subscriptions per IP.
pub const WS_MAX_UNIQUE_USERS: u16 = 10;
/// Maximum client-to-server WebSocket messages per minute per IP.
pub const WS_MAX_MESSAGES_PER_MINUTE: u16 = 2_000;
/// Maximum in-flight WebSocket post requests.
pub const WS_MAX_IN_FLIGHT_POSTS: u16 = 100;

include!("http.rs");
include!("websocket.rs");
include!("error.rs");
