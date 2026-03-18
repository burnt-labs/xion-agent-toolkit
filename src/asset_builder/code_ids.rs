//! Asset Builder Code IDs
//!
//! Code ID management for CW721 NFT contracts.
//!
//! ## Testnet Code IDs
//!
//! | Asset Type | Code ID |
//! |------------|---------|
//! | cw721-base | 522 |
//! | cw721-metadata-onchain | 525 |
//! | cw721-expiration | 523 |
//! | cw721-non-transferable | 526 |
//! | cw2981-royalties | 528 |

use crate::config::NetworkConfig;

use super::types::{AssetBuilderError, AssetType};

/// Get the code ID for an asset type on a specific network
///
/// # Arguments
/// * `asset_type` - The asset type to get code ID for
/// * `config` - Network configuration
///
/// # Returns
/// Code ID for the asset type on the specified network
///
/// # Errors
/// Returns an error if the code ID is not configured for the asset type
pub fn get_code_id(
    asset_type: AssetType,
    config: &NetworkConfig,
) -> Result<u64, AssetBuilderError> {
    let code_id = match asset_type {
        AssetType::Cw721Base => config.cw721_base_code_id,
        AssetType::Cw721MetadataOnchain => config.cw721_metadata_onchain_code_id,
        AssetType::Cw721Expiration => config.cw721_expiration_code_id,
        AssetType::Cw721NonTransferable => config.cw721_non_transferable_code_id,
        AssetType::Cw2981Royalties => config.cw2981_royalties_code_id,
    };

    if code_id == 0 {
        return Err(AssetBuilderError::CodeIdNotFound(
            asset_type.to_string(),
            config.network_name.clone(),
        ));
    }

    Ok(code_id)
}

/// Get checksum for instantiate2 (Phase 3 - optional)
///
/// This is reserved for Phase 3 when we implement address prediction.
/// The checksum is required for computing predictable addresses.
#[allow(dead_code)]
pub fn get_checksum(asset_type: AssetType, _config: &NetworkConfig) -> Option<String> {
    // Phase 3: Return actual checksums for each code ID
    // For now, return None as this is optional functionality
    match asset_type {
        AssetType::Cw721Base => None, // TODO: Add actual checksum
        AssetType::Cw721MetadataOnchain => None,
        AssetType::Cw721Expiration => None,
        AssetType::Cw721NonTransferable => None,
        AssetType::Cw2981Royalties => None,
    }
}

/// Get asset type info for display
///
/// # Returns
/// List of asset type information
pub fn get_asset_types_info() -> Vec<super::types::AssetTypeInfo> {
    use super::types::AssetTypeInfo;

    let types = AssetType::all();
    types
        .iter()
        .map(|&t| AssetTypeInfo {
            asset_type: t.as_str().to_string(),
            display_name: t.display_name().to_string(),
            testnet_code_id: get_testnet_code_id(t),
            mainnet_code_id: get_mainnet_code_id(t),
            description: get_description(t),
        })
        .collect()
}

/// Get testnet code ID for asset type
fn get_testnet_code_id(asset_type: AssetType) -> u64 {
    match asset_type {
        AssetType::Cw721Base => 522,
        AssetType::Cw721MetadataOnchain => 525,
        AssetType::Cw721Expiration => 523,
        AssetType::Cw721NonTransferable => 526,
        AssetType::Cw2981Royalties => 528,
    }
}

/// Get mainnet code ID for asset type
fn get_mainnet_code_id(asset_type: AssetType) -> u64 {
    // Mainnet code IDs are not yet deployed/configured
    // Set to 0 to indicate "not available"
    match asset_type {
        AssetType::Cw721Base => 0,
        AssetType::Cw721MetadataOnchain => 0,
        AssetType::Cw721Expiration => 0,
        AssetType::Cw721NonTransferable => 0,
        AssetType::Cw2981Royalties => 0,
    }
}

/// Get description for asset type
fn get_description(asset_type: AssetType) -> String {
    match asset_type {
        AssetType::Cw721Base => {
            "Standard CW721 NFT contract with basic mint and transfer functionality".to_string()
        }
        AssetType::Cw721MetadataOnchain => {
            "NFT contract with on-chain metadata storage".to_string()
        }
        AssetType::Cw721Expiration => "NFT contract with time-based token expiration".to_string(),
        AssetType::Cw721NonTransferable => "Non-transferable (soulbound) NFT contract".to_string(),
        AssetType::Cw2981Royalties => {
            "NFT contract with CW2981 royalty standard support".to_string()
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_testnet_config() -> NetworkConfig {
        NetworkConfig {
            network_name: "testnet".to_string(),
            oauth_api_url: "https://oauth2.testnet.burnt.com".to_string(),
            rpc_url: "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
            rest_url: "https://api.xion-testnet-2.burnt.com".to_string(),
            chain_id: "xion-testnet-2".to_string(),
            oauth_client_id: "test-client-id".to_string(),
            treasury_code_id: 1260,
            callback_port: 54321,
            indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
            cw721_base_code_id: 522,
            cw721_metadata_onchain_code_id: 525,
            cw721_expiration_code_id: 523,
            cw721_fixed_price_code_id: 524,
            cw721_non_transferable_code_id: 526,
            cw2981_royalties_code_id: 528,
        }
    }

    #[test]
    fn test_get_code_id_cw721_base() {
        let config = create_testnet_config();
        let code_id = get_code_id(AssetType::Cw721Base, &config).unwrap();
        assert_eq!(code_id, 522);
    }

    #[test]
    fn test_get_code_id_cw2981_royalties() {
        let config = create_testnet_config();
        let code_id = get_code_id(AssetType::Cw2981Royalties, &config).unwrap();
        assert_eq!(code_id, 528);
    }

    #[test]
    fn test_get_code_id_zero_returns_error() {
        let mut config = create_testnet_config();
        config.cw721_base_code_id = 0;

        let result = get_code_id(AssetType::Cw721Base, &config);
        assert!(result.is_err());

        match result {
            Err(AssetBuilderError::CodeIdNotFound(asset_type, network)) => {
                assert_eq!(asset_type, "cw721-base");
                assert_eq!(network, "testnet");
            }
            _ => panic!("Expected CodeIdNotFound error"),
        }
    }

    #[test]
    fn test_get_testnet_code_id() {
        assert_eq!(get_testnet_code_id(AssetType::Cw721Base), 522);
        assert_eq!(get_testnet_code_id(AssetType::Cw721MetadataOnchain), 525);
        assert_eq!(get_testnet_code_id(AssetType::Cw721Expiration), 523);
        assert_eq!(get_testnet_code_id(AssetType::Cw721NonTransferable), 526);
        assert_eq!(get_testnet_code_id(AssetType::Cw2981Royalties), 528);
    }

    #[test]
    fn test_get_mainnet_code_id() {
        // All mainnet code IDs should be 0 (not configured)
        assert_eq!(get_mainnet_code_id(AssetType::Cw721Base), 0);
        assert_eq!(get_mainnet_code_id(AssetType::Cw2981Royalties), 0);
    }

    #[test]
    fn test_get_asset_types_info() {
        let info = get_asset_types_info();

        assert_eq!(info.len(), 5);

        // Check first entry (cw721-base)
        let base_info = &info[0];
        assert_eq!(base_info.asset_type, "cw721-base");
        assert_eq!(base_info.testnet_code_id, 522);
        assert_eq!(base_info.mainnet_code_id, 0);
        assert!(!base_info.description.is_empty());
    }

    #[test]
    fn test_get_description() {
        let desc = get_description(AssetType::Cw721Base);
        assert!(desc.contains("Standard"));
        assert!(desc.contains("CW721"));

        let desc = get_description(AssetType::Cw721NonTransferable);
        assert!(desc.contains("soulbound"));
    }
}
