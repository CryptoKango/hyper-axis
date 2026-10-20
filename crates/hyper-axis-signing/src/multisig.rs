/// Computes an authorized user's digest for a multi-signature Core USDC
/// transfer.
#[must_use]
pub fn multi_sig_usd_send_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    destination: &str,
    amount: &str,
    time: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Text(destination),
            UserField::Text(amount),
            UserField::Uint64(time),
        ],
    )
}

/// Computes an authorized user's digest for a multi-signature spot transfer.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn multi_sig_spot_send_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    destination: &str,
    token: &str,
    amount: &str,
    time: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Text(destination),
            UserField::Text(token),
            UserField::Text(amount),
            UserField::Uint64(time),
        ],
    )
}

/// Computes an authorized user's digest for a multi-signature bridge
/// withdrawal.
#[must_use]
pub fn multi_sig_withdraw_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    destination: &str,
    amount: &str,
    time: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Text(destination),
            UserField::Text(amount),
            UserField::Uint64(time),
        ],
    )
}

/// Computes an authorized user's digest for multi-signature API-wallet
/// approval.
#[must_use]
pub fn multi_sig_approve_agent_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    agent_address: &EvmAddress,
    agent_name: &str,
    nonce: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Address(agent_address),
            UserField::Text(agent_name),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes an authorized user's digest for multi-signature builder-fee
/// approval.
#[must_use]
pub fn multi_sig_approve_builder_fee_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    max_fee_rate: &str,
    builder: &EvmAddress,
    nonce: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,address builder,uint64 nonce)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Text(max_fee_rate),
            UserField::Address(builder),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes an authorized user's digest for a multi-signature USD class
/// transfer.
#[must_use]
pub fn multi_sig_usd_class_transfer_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    amount: &str,
    to_perp: bool,
    nonce: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:UsdClassTransfer(string hyperliquidChain,string amount,bool toPerp,uint64 nonce)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Text(amount),
            UserField::Bool(to_perp),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes an authorized user's digest for a multi-signature staking
/// deposit.
#[must_use]
pub fn multi_sig_c_deposit_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    wei: u64,
    nonce: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:CDeposit(string hyperliquidChain,uint64 wei,uint64 nonce)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[UserField::Uint64(wei), UserField::Uint64(nonce)],
    )
}

/// Computes an authorized user's digest for a multi-signature staking
/// withdrawal.
#[must_use]
pub fn multi_sig_c_withdraw_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    wei: u64,
    nonce: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:CWithdraw(string hyperliquidChain,uint64 wei,uint64 nonce)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[UserField::Uint64(wei), UserField::Uint64(nonce)],
    )
}

/// Computes an authorized user's digest for multi-signature validator
/// delegation.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn multi_sig_token_delegate_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    validator: &EvmAddress,
    wei: u64,
    is_undelegate: bool,
    nonce: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:TokenDelegate(string hyperliquidChain,address validator,uint64 wei,bool isUndelegate,uint64 nonce)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Address(validator),
            UserField::Uint64(wei),
            UserField::Bool(is_undelegate),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes an authorized user's digest for the multi-signature legacy DEX
/// abstraction toggle.
#[must_use]
pub fn multi_sig_user_dex_abstraction_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    user: &EvmAddress,
    enabled: bool,
    nonce: u64,
) -> [u8; 32] {
    multi_sig_user_action_digest(
        "HyperliquidTransaction:UserDexAbstraction(string hyperliquidChain,address user,bool enabled,uint64 nonce)",
        network,
        payload_multi_sig_user,
        outer_signer,
        &[
            UserField::Address(user),
            UserField::Bool(enabled),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes an authorized user's digest for a multi-signature `sendAsset`
/// inner payload.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn multi_sig_send_asset_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    destination: &str,
    source_dex: &str,
    destination_dex: &str,
    token: &str,
    amount: &str,
    from_sub_account: &str,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:SendAsset(string hyperliquidChain,address payloadMultiSigUser,address outerSigner,string destination,string sourceDex,string destinationDex,string token,string amount,string fromSubAccount,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Address(payload_multi_sig_user),
            UserField::Address(outer_signer),
            UserField::Text(destination),
            UserField::Text(source_dex),
            UserField::Text(destination_dex),
            UserField::Text(token),
            UserField::Text(amount),
            UserField::Text(from_sub_account),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes an authorized user's digest for a multi-signature
/// `userSetAbstraction` inner payload.
#[must_use]
pub fn multi_sig_user_set_abstraction_signing_digest(
    network: Network,
    payload_multi_sig_user: &EvmAddress,
    outer_signer: &EvmAddress,
    user: &EvmAddress,
    abstraction: &str,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:UserSetAbstraction(string hyperliquidChain,address payloadMultiSigUser,address outerSigner,address user,string abstraction,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Address(payload_multi_sig_user),
            UserField::Address(outer_signer),
            UserField::Address(user),
            UserField::Text(abstraction),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the outer signer's digest for a complete multi-signature action.
#[must_use]
pub fn multi_sig_envelope_signing_digest(
    network: Network,
    multi_sig_action_hash: [u8; 32],
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:SendMultiSig(string hyperliquidChain,bytes32 multiSigActionHash,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Bytes32(&multi_sig_action_hash),
            UserField::Uint64(nonce),
        ],
    )
}
