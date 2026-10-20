/// Hyperliquid environment used for API routing and L1 signature source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    /// Production environment.
    Mainnet,
    /// Public test environment.
    Testnet,
}

impl Network {
    /// Source discriminator embedded in the L1 phantom-agent message.
    #[must_use]
    pub const fn l1_source(self) -> &'static str {
        match self {
            Self::Mainnet => "a",
            Self::Testnet => "b",
        }
    }

    /// Human-readable chain name used by user-signed actions.
    #[must_use]
    pub const fn hyperliquid_chain(self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet",
            Self::Testnet => "Testnet",
        }
    }

    /// Base URL of the official HTTP API for this environment.
    #[must_use]
    pub const fn api_url(self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.hyperliquid.xyz",
            Self::Testnet => "https://api.hyperliquid-testnet.xyz",
        }
    }

    /// Exchange endpoint for signed write actions.
    #[must_use]
    pub const fn exchange_url(self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.hyperliquid.xyz/exchange",
            Self::Testnet => "https://api.hyperliquid-testnet.xyz/exchange",
        }
    }

    /// Read-only information endpoint for this environment.
    #[must_use]
    pub const fn info_url(self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.hyperliquid.xyz/info",
            Self::Testnet => "https://api.hyperliquid-testnet.xyz/info",
        }
    }

    /// WebSocket endpoint for this environment.
    #[must_use]
    pub const fn websocket_url(self) -> &'static str {
        match self {
            Self::Mainnet => "wss://api.hyperliquid.xyz/ws",
            Self::Testnet => "wss://api.hyperliquid-testnet.xyz/ws",
        }
    }
}
