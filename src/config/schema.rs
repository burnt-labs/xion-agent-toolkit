use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub network: String,
    pub oauth: Option<OAuthConfig>,
    pub treasury: Option<TreasuryConfig>,
    pub networks: NetworkConfigs,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            network: "testnet".to_string(),
            oauth: None,
            treasury: None,
            networks: NetworkConfigs::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub client_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryConfig {
    pub default_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfigs {
    pub local: NetworkConfig,
    pub testnet: NetworkConfig,
    pub mainnet: NetworkConfig,
}

impl Default for NetworkConfigs {
    fn default() -> Self {
        Self {
            local: NetworkConfig {
                oauth_api_url: "http://localhost:8787".to_string(),
                rpc_url: "http://localhost:26657".to_string(),
                chain_id: "xion-local".to_string(),
                treasury_code_id: None,
                treasury_config: None,
            },
            testnet: NetworkConfig {
                oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
                rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
                chain_id: "xion-testnet-2".to_string(),
                treasury_code_id: Some(1260),
                treasury_config: Some(
                    "xion175qd54keur7gkuwtctfupgtucvlvkrxhv0pgq753sfh5xueputvsms6nll".to_string(),
                ),
            },
            mainnet: NetworkConfig {
                oauth_api_url: "https://oauth2.burnt.com".to_string(),
                rpc_url: "https://rpc.xion-mainnet-1.burnt.com:443".to_string(),
                chain_id: "xion-mainnet-1".to_string(),
                treasury_code_id: Some(63),
                treasury_config: Some(
                    "xion1dlsvvgey26ernlj0sq2afjluh3qd4ap0k9eerekfkw5algqrwqkshmn3uq".to_string(),
                ),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub oauth_api_url: String,
    pub rpc_url: String,
    pub chain_id: String,
    pub treasury_code_id: Option<u64>,
    pub treasury_config: Option<String>,
}
