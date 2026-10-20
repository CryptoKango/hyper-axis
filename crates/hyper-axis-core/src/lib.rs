//! Stable public facade for native `HyperAxis` consumers.

pub use hyper_axis_actions as actions;
pub use hyper_axis_assets as assets;
pub use hyper_axis_client as client;
pub use hyper_axis_codec as codec;
pub use hyper_axis_decimal as decimal;
pub use hyper_axis_logging as logging;
pub use hyper_axis_nonce as nonce;
pub use hyper_axis_signing as signing;
pub use hyper_axis_transport as transport;
pub use hyper_axis_types as types;

/// Semantic version of the Rust protocol core.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
