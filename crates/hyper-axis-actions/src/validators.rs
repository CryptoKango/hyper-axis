/// Signer-node self-management variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CSignerVariant {
    /// Remove the signing node from jail.
    UnjailSelf,
    /// Voluntarily jail the signing node.
    JailSelf,
}

/// Consensus signer self-management action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSignerAction {
    /// Action discriminator. Always `CSignerAction`.
    #[serde(rename = "type")]
    action_type: CSignerActionType,
    #[serde(flatten)]
    variant: CSignerWireVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CSignerActionType {
    #[serde(rename = "CSignerAction")]
    CSignerAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum CSignerWireVariant {
    UnjailSelf(()),
    JailSelf(()),
}

impl CSignerAction {
    /// Creates a signer-node self-management action.
    #[must_use]
    pub const fn new(variant: CSignerVariant) -> Self {
        match variant {
            CSignerVariant::UnjailSelf => Self {
                action_type: CSignerActionType::CSignerAction,
                variant: CSignerWireVariant::UnjailSelf(()),
            },
            CSignerVariant::JailSelf => Self {
                action_type: CSignerActionType::CSignerAction,
                variant: CSignerWireVariant::JailSelf(()),
            },
        }
    }
}

/// Wire wrapper required by validator profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorNodeIp {
    /// Canonical IPv4 or IPv6 address.
    #[serde(rename = "Ip")]
    pub ip: String,
}

/// Complete validator profile used during registration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorProfile {
    /// Validator node IP address.
    pub node_ip: ValidatorNodeIp,
    /// Public validator name.
    pub name: String,
    /// Public validator description.
    pub description: String,
    /// Whether third-party delegations are disabled.
    pub delegations_disabled: bool,
    /// Commission in basis points.
    pub commission_bps: u16,
    /// Consensus signer address.
    pub signer: EvmAddress,
}

/// Validator registration payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorRegister {
    /// Initial public profile.
    pub profile: ValidatorProfile,
    /// Whether the validator starts unjailed.
    pub unjailed: bool,
    /// Initial native-token self delegation in wei.
    pub initial_wei: u64,
}

/// Partial validator profile update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorChangeProfile {
    /// Optional replacement node IP.
    pub node_ip: Option<ValidatorNodeIp>,
    /// Optional replacement public name.
    pub name: Option<String>,
    /// Optional replacement description.
    pub description: Option<String>,
    /// Whether the validator should be unjailed.
    pub unjailed: bool,
    /// Optional delegation-disable flag.
    pub disable_delegations: Option<bool>,
    /// Optional commission in basis points.
    pub commission_bps: Option<u16>,
    /// Optional replacement consensus signer.
    pub signer: Option<EvmAddress>,
}

/// Consensus validator lifecycle action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CValidatorAction {
    /// Action discriminator. Always `CValidatorAction`.
    #[serde(rename = "type")]
    action_type: CValidatorActionType,
    #[serde(flatten)]
    variant: CValidatorVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum CValidatorActionType {
    #[serde(rename = "CValidatorAction")]
    CValidatorAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum CValidatorVariant {
    Register(ValidatorRegister),
    ChangeProfile(ValidatorChangeProfile),
    Unregister(()),
}

impl CValidatorAction {
    /// Creates validator registration.
    #[must_use]
    pub const fn register(register: ValidatorRegister) -> Self {
        Self {
            action_type: CValidatorActionType::CValidatorAction,
            variant: CValidatorVariant::Register(register),
        }
    }

    /// Creates a validator profile update.
    #[must_use]
    pub const fn change_profile(change_profile: ValidatorChangeProfile) -> Self {
        Self {
            action_type: CValidatorActionType::CValidatorAction,
            variant: CValidatorVariant::ChangeProfile(change_profile),
        }
    }

    /// Creates validator unregistration.
    #[must_use]
    pub const fn unregister() -> Self {
        Self {
            action_type: CValidatorActionType::CValidatorAction,
            variant: CValidatorVariant::Unregister(()),
        }
    }
}

/// Bids gas for a gossip priority slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GossipPriorityBidAction {
    /// Action discriminator. Always `gossipPriorityBid`.
    #[serde(rename = "type")]
    action_type: GossipPriorityBidActionType,
    /// Target gossip priority slot.
    pub slot_id: u64,
    /// Canonical target IP address.
    pub ip: String,
    /// Maximum gas bid.
    pub max_gas: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum GossipPriorityBidActionType {
    #[serde(rename = "gossipPriorityBid")]
    GossipPriorityBid,
}

impl GossipPriorityBidAction {
    /// Creates a gossip priority bid.
    #[must_use]
    pub const fn new(slot_id: u64, ip: String, max_gas: u64) -> Self {
        Self {
            action_type: GossipPriorityBidActionType::GossipPriorityBid,
            slot_id,
            ip,
            max_gas,
        }
    }
}
