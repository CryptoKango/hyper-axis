//! WebAssembly boundary for browser clients.

use std::str::FromStr;

use hyper_axis_core::decimal::DecimalValue;
use hyper_axis_core::{
    actions::{Grouping, TimeInForce},
    assets::MarketKind,
    client::{FinalizeError, FinalizedAction, PrepareLimitOrderRequest},
    logging::{self, LogLevel},
    nonce::{NonceManager, validate_nonce_window},
    signing::WireSignature,
    transport::{
        HostFailureKind, HttpResponse, PostRequestId, RequestKind, Subscription, WsCommand,
        classify_host_failure, exchange_http_request,
        info::{InfoQuery, prepare_info_query},
        info_http_request, parse_http_response, parse_ws_message, typed_info_http_request,
    },
    types::{AssetId, EvmAddress, Network},
};
use serde::{Serialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;

include!("runtime.rs");
include!("transport.rs");
include!("prepare.rs");
include!("multisig.rs");
include!("finalize.rs");
include!("internal.rs");
