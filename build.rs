use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Load .env file if it exists
    dotenvy::dotenv().ok();

    // Read OAuth2 client IDs from environment variables
    let testnet_client_id = env::var("XION_TESTNET_OAUTH_CLIENT_ID")
        .unwrap_or_else(|_| "PLACEHOLDER_TESTNET_CLIENT_ID".to_string());

    let mainnet_client_id = env::var("XION_MAINNET_OAUTH_CLIENT_ID")
        .unwrap_or_else(|_| "PLACEHOLDER_MAINNET_CLIENT_ID".to_string());

    // Generate network_config.rs
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("network_config.rs");

    let config_content = format!(
        r#"// Auto-generated network configuration
// This file is generated at compile time from environment variables

use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {{
    pub network_name: String,
    pub oauth_api_url: String,
    pub rpc_url: String,
    pub chain_id: String,
    pub oauth_client_id: String,
    pub treasury_code_id: u64,
    pub callback_port: u16,
    pub indexer_url: String,
    // CW721 Asset Code IDs
    pub cw721_base_code_id: u64,
    pub cw721_metadata_onchain_code_id: u64,
    pub cw721_expiration_code_id: u64,
    pub cw721_fixed_price_code_id: u64,
    pub cw721_non_transferable_code_id: u64,
    pub cw2981_royalties_code_id: u64,
}}

pub fn get_testnet_config() -> NetworkConfig {{
    NetworkConfig {{
        network_name: "testnet".to_string(),
        oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
        rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
        chain_id: "xion-testnet-2".to_string(),
        oauth_client_id: "{}".to_string(),
        treasury_code_id: 1260,
        callback_port: 54321,
        indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
        // CW721 Asset Code IDs (Testnet)
        cw721_base_code_id: 522,
        cw721_metadata_onchain_code_id: 525,
        cw721_expiration_code_id: 523,
        cw721_fixed_price_code_id: 524,
        cw721_non_transferable_code_id: 526,
        cw2981_royalties_code_id: 528,
    }}
}}

pub fn get_mainnet_config() -> NetworkConfig {{
    NetworkConfig {{
        network_name: "mainnet".to_string(),
        oauth_api_url: "https://oauth2.burnt.com".to_string(),
        rpc_url: "https://rpc.xion-mainnet-1.burnt.com:443".to_string(),
        chain_id: "xion-mainnet-1".to_string(),
        oauth_client_id: "{}".to_string(),
        treasury_code_id: 63,
        callback_port: 54321,
        indexer_url: "https://daodaoindexer.burnt.com/xion-mainnet-1".to_string(),
        // CW721 Asset Code IDs (Mainnet - not yet deployed)
        cw721_base_code_id: 0,
        cw721_metadata_onchain_code_id: 0,
        cw721_expiration_code_id: 0,
        cw721_fixed_price_code_id: 0,
        cw721_non_transferable_code_id: 0,
        cw2981_royalties_code_id: 0,
    }}
}}
"#,
        testnet_client_id, mainnet_client_id
    );

    fs::write(&dest_path, config_content).unwrap();

    println!("cargo:rerun-if-env-changed=XION_TESTNET_OAUTH_CLIENT_ID");
    println!("cargo:rerun-if-env-changed=XION_MAINNET_OAUTH_CLIENT_ID");
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-changed=build.rs");
}
