//! Binding-friendly native API.
//!
//! The exported Rust functions are intentionally coarse grained so a binding
//! generator such as `flutter_rust_bridge` can own memory transfer safely.

use std::str::FromStr;

pub use hyper_axis_core::actions::{AgentAbstraction, CSignerVariant, UserAbstraction};
pub use hyper_axis_core::client::{
    BuilderInput, CancelByCloidInput, CancelInput, FinalizedAction, FinalizedOrder,
    LimitOrderInput, ModifyOrderInput, MultiSigError, MultiSigInnerAction, MultiSigSignatureInput,
    OrderInput, OrderSelectorInput, PerpDeployInput, PerpDexSchemaInput, PerpPriceInput,
    PrepareAgentEnableDexAbstractionRequest, PrepareAgentSetAbstractionRequest,
    PrepareApproveAgentRequest, PrepareApproveBuilderFeeRequest, PrepareBatchModifyRequest,
    PrepareBatchOrdersRequest, PrepareCDepositRequest, PrepareCSignerRequest,
    PrepareCValidatorChangeProfileRequest, PrepareCValidatorRegisterRequest,
    PrepareCValidatorUnregisterRequest, PrepareCWithdrawRequest, PrepareCancelByCloidRequest,
    PrepareCancelRequest, PrepareConvertToMultiSigUserRequest, PrepareCreateSubAccountRequest,
    PrepareEvmUserModifyRequest, PrepareGossipPriorityBidRequest, PrepareLimitOrderRequest,
    PrepareNoopRequest, PrepareOrdersRequest, PreparePerpDeployRequest,
    PrepareScheduleCancelRequest, PrepareSendAssetRequest, PrepareSetReferrerRequest,
    PrepareSpotDeployRequest, PrepareSpotSendRequest, PrepareSubAccountSpotTransferRequest,
    PrepareSubAccountTransferRequest, PrepareTokenDelegateRequest,
    PrepareTopUpIsolatedOnlyMarginRequest, PrepareTwapCancelRequest, PrepareTwapOrderRequest,
    PrepareUpdateIsolatedMarginRequest, PrepareUpdateLeverageRequest,
    PrepareUsdClassTransferRequest, PrepareUsdSendRequest, PrepareUserDexAbstractionRequest,
    PrepareUserSetAbstractionRequest, PrepareVaultTransferRequest, PrepareWithdrawRequest,
    PreparedAgentEnableDexAbstraction, PreparedAgentSetAbstraction, PreparedApproveAgent,
    PreparedApproveBuilderFee, PreparedBatchModify, PreparedCDeposit, PreparedCSigner,
    PreparedCValidator, PreparedCWithdraw, PreparedCancel, PreparedCancelByCloid,
    PreparedConvertToMultiSigUser, PreparedCreateSubAccount, PreparedEvmUserModify,
    PreparedGossipPriorityBid, PreparedMultiSigEnvelope, PreparedMultiSigPayload, PreparedNoop,
    PreparedOrder, PreparedPerpDeploy, PreparedScheduleCancel, PreparedSendAsset,
    PreparedSetReferrer, PreparedSpotDeploy, PreparedSpotSend, PreparedSubAccountSpotTransfer,
    PreparedSubAccountTransfer, PreparedTokenDelegate, PreparedTopUpIsolatedOnlyMargin,
    PreparedTwapCancel, PreparedTwapOrder, PreparedUpdateIsolatedMargin, PreparedUpdateLeverage,
    PreparedUsdClassTransfer, PreparedUsdSend, PreparedUserDexAbstraction,
    PreparedUserSetAbstraction, PreparedVaultTransfer, PreparedWithdraw, SpotDeployInput,
    SpotExistingTokenWeiInput, SpotUserWeiInput, TriggerOrderInput,
};
use hyper_axis_core::decimal::DecimalValue;
use hyper_axis_core::logging::{self, LogLevel};
pub use hyper_axis_core::transport::info::{InfoQuery, OrderStatusSelector, PreparedInfoQuery};
use hyper_axis_core::{
    nonce::{NonceManager, validate_nonce_window},
    signing::WireSignature,
    transport::{
        HostFailureKind, HttpResponse, PostRequestId, RequestKind, Subscription, WsCommand,
        classify_host_failure, exchange_http_request, info::prepare_info_query, info_http_request,
        parse_http_response, parse_ws_message, typed_info_http_request,
    },
    types::{EvmAddress, Network},
};

include!("runtime.rs");
include!("prepare.rs");
include!("multisig.rs");
include!("finalize.rs");
include!("platform.rs");
