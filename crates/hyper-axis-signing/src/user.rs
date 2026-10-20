/// Computes the user-signed digest for a Core USDC transfer.
#[must_use]
pub fn usd_send_signing_digest(
    network: Network,
    destination: &str,
    amount: &str,
    time: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Text(destination),
            UserField::Text(amount),
            UserField::Uint64(time),
        ],
    )
}

/// Computes the user-signed digest for a Core spot transfer.
#[must_use]
pub fn spot_send_signing_digest(
    network: Network,
    destination: &str,
    token: &str,
    amount: &str,
    time: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Text(destination),
            UserField::Text(token),
            UserField::Text(amount),
            UserField::Uint64(time),
        ],
    )
}

/// Computes the user-signed digest for a bridge withdrawal.
#[must_use]
pub fn withdraw_signing_digest(
    network: Network,
    destination: &str,
    amount: &str,
    time: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Text(destination),
            UserField::Text(amount),
            UserField::Uint64(time),
        ],
    )
}

/// Computes the user-signed digest for approving an API wallet.
///
/// `agent_name` must be an empty string for an unnamed agent even though the
/// final exchange action omits the `agentName` field in that case.
#[must_use]
pub fn approve_agent_signing_digest(
    network: Network,
    agent_address: &EvmAddress,
    agent_name: &str,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Address(agent_address),
            UserField::Text(agent_name),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for approving a builder fee rate.
#[must_use]
pub fn approve_builder_fee_signing_digest(
    network: Network,
    max_fee_rate: &str,
    builder: &EvmAddress,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,address builder,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Text(max_fee_rate),
            UserField::Address(builder),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for moving USD between spot and perp classes.
#[must_use]
pub fn usd_class_transfer_signing_digest(
    network: Network,
    amount: &str,
    to_perp: bool,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:UsdClassTransfer(string hyperliquidChain,string amount,bool toPerp,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Text(amount),
            UserField::Bool(to_perp),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for a generalized asset transfer.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn send_asset_signing_digest(
    network: Network,
    destination: &str,
    source_dex: &str,
    destination_dex: &str,
    token: &str,
    amount: &str,
    from_sub_account: &str,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:SendAsset(string hyperliquidChain,string destination,string sourceDex,string destinationDex,string token,string amount,string fromSubAccount,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
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

/// Computes the user-signed digest for depositing native tokens into staking.
#[must_use]
pub fn c_deposit_signing_digest(network: Network, wei: u64, nonce: u64) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:CDeposit(string hyperliquidChain,uint64 wei,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Uint64(wei),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for withdrawing native tokens from staking.
#[must_use]
pub fn c_withdraw_signing_digest(network: Network, wei: u64, nonce: u64) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:CWithdraw(string hyperliquidChain,uint64 wei,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Uint64(wei),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for delegating or undelegating stake.
#[must_use]
pub fn token_delegate_signing_digest(
    network: Network,
    validator: &EvmAddress,
    wei: u64,
    is_undelegate: bool,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:TokenDelegate(string hyperliquidChain,address validator,uint64 wei,bool isUndelegate,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Address(validator),
            UserField::Uint64(wei),
            UserField::Bool(is_undelegate),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for the legacy DEX-abstraction toggle.
#[must_use]
pub fn user_dex_abstraction_signing_digest(
    network: Network,
    user: &EvmAddress,
    enabled: bool,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:UserDexAbstraction(string hyperliquidChain,address user,bool enabled,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Address(user),
            UserField::Bool(enabled),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for changing account abstraction mode.
#[must_use]
pub fn user_set_abstraction_signing_digest(
    network: Network,
    user: &EvmAddress,
    abstraction: &str,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:UserSetAbstraction(string hyperliquidChain,address user,string abstraction,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Address(user),
            UserField::Text(abstraction),
            UserField::Uint64(nonce),
        ],
    )
}

/// Computes the user-signed digest for converting to a multi-signature user.
#[must_use]
pub fn convert_to_multi_sig_user_signing_digest(
    network: Network,
    signers: &str,
    nonce: u64,
) -> [u8; 32] {
    user_action_digest(
        "HyperliquidTransaction:ConvertToMultiSigUser(string hyperliquidChain,string signers,uint64 nonce)",
        &[
            UserField::Text(network.hyperliquid_chain()),
            UserField::Text(signers),
            UserField::Uint64(nonce),
        ],
    )
}
