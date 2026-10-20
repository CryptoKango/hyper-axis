//! Coarse-grained orchestration APIs shared by native and web bindings.

use std::net::IpAddr;

use hyper_axis_actions::{
    AgentAbstraction, AgentEnableDexAbstractionAction, AgentSetAbstractionAction,
    ApproveAgentAction, ApproveBuilderFeeAction, BatchModifyAction, BuilderFeeRate, BuilderWire,
    CDepositAction, CSignerAction, CSignerVariant, CValidatorAction, CWithdrawAction, CancelAction,
    CancelByCloidAction, CancelByCloidWire, CancelWire, ConvertToMultiSigUserAction,
    CreateSubAccountAction, EvmUserModifyAction, FromSubAccount, GossipPriorityBidAction, Grouping,
    LimitWire, ModifyWire, MultiSigSigners, NoopAction, OrderAction, OrderId, OrderTypeWire,
    OrderWire, PerpAssetRequest, PerpDeployAction, PerpDeployVariant, PerpDexSchema,
    PerpPriceEntry, PerpRegisterAsset, PerpSetOracle, ScheduleCancelAction, SendAssetAction,
    SetReferrerAction, SpotDeployAction, SpotDeployVariant, SpotDeployerTradingFeeShare,
    SpotFreezeUser, SpotGenesis, SpotRegisterHyperliquidity, SpotRegisterSpot, SpotRegisterToken,
    SpotSendAction, SpotTokenReference, SpotTokenSpec, SpotUserGenesis,
    SubAccountSpotTransferAction, SubAccountTransferAction, TimeInForce, TokenDelegateAction,
    TopUpIsolatedOnlyMarginAction, TriggerKind, TriggerWire, TwapCancelAction, TwapOrderAction,
    TwapWire, UpdateIsolatedMarginAction, UpdateLeverageAction, UsdClassTransferAction,
    UsdClassTransferAmount, UsdSendAction, UserAbstraction, UserDexAbstractionAction,
    UserSetAbstractionAction, ValidatorChangeProfile, ValidatorNodeIp, ValidatorProfile,
    ValidatorRegister, VaultTransferAction, WithdrawAction,
};
use hyper_axis_assets::{MarketKind, MarketPrecision};
use hyper_axis_codec::{CodecError, action_hash, encode_action};
use hyper_axis_decimal::DecimalValue;
use hyper_axis_logging::{LogLevel, emit};
use hyper_axis_signing::{
    SigningError, WireSignature, approve_agent_signing_digest, approve_builder_fee_signing_digest,
    c_deposit_signing_digest, c_withdraw_signing_digest, convert_to_multi_sig_user_signing_digest,
    l1_signing_digest, multi_sig_approve_agent_signing_digest,
    multi_sig_approve_builder_fee_signing_digest, multi_sig_c_deposit_signing_digest,
    multi_sig_c_withdraw_signing_digest, multi_sig_envelope_signing_digest,
    multi_sig_send_asset_signing_digest, multi_sig_spot_send_signing_digest,
    multi_sig_token_delegate_signing_digest, multi_sig_usd_class_transfer_signing_digest,
    multi_sig_usd_send_signing_digest, multi_sig_user_dex_abstraction_signing_digest,
    multi_sig_user_set_abstraction_signing_digest, multi_sig_withdraw_signing_digest,
    send_asset_signing_digest, spot_send_signing_digest, token_delegate_signing_digest,
    usd_class_transfer_signing_digest, usd_send_signing_digest,
    user_dex_abstraction_signing_digest, user_set_abstraction_signing_digest,
    withdraw_signing_digest,
};
use hyper_axis_types::{AssetId, ClientOrderId, EvmAddress, Network, ProtocolError};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use thiserror::Error;

include!("models.rs");
include!("prepare.rs");
include!("multisig.rs");
include!("finalize.rs");
include!("internal.rs");
