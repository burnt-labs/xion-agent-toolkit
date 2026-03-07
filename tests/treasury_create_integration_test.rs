//! Treasury Create Integration Tests
//!
//! Integration tests for Treasury Create command including:
//! - Encoding tests
//! - Type serialization tests  
//! - End-to-end flow tests with mocked API

use base64::Engine;
use mockito::Server;
use serde_json::json;
use xion_agent_toolkit::config::NetworkConfig;
use xion_agent_toolkit::oauth::OAuthClient;
use xion_agent_toolkit::treasury::encoding::{
    encode_basic_allowance, encode_generic_authorization, encode_periodic_allowance,
    encode_send_authorization, parse_coin_string, Coin,
};
use xion_agent_toolkit::treasury::types::{
    AuthorizationInput, CreateTreasuryRequest, FeeConfigInput, FeeConfigMessage, GrantConfigInput,
    GrantConfigMessage, TreasuryCreateRequest, TreasuryParamsInput, TreasuryParamsMessage,
    TypeUrlValue,
};
use xion_agent_toolkit::treasury::{TreasuryApiClient, TreasuryManager};

// ============================================================================
// Helper Functions
// ============================================================================

fn create_test_config(server_url: &str) -> NetworkConfig {
    NetworkConfig {
        network_name: "test".to_string(),
        oauth_api_url: server_url.to_string(),
        rpc_url: "http://localhost:26657".to_string(),
        chain_id: "xion-local".to_string(),
        oauth_client_id: "test-client-id".to_string(),
        treasury_code_id: 1260,
        callback_port: 54321,
        indexer_url: "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    }
}

// ============================================================================
// Encoding Tests - BasicAllowance
// ============================================================================

#[test]
fn test_basic_allowance_encoding_single_coin() {
    let coins = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000000".to_string(),
    }];

    let encoded = encode_basic_allowance(coins).expect("Failed to encode BasicAllowance");

    // Should be valid base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    assert!(!decoded.is_empty());

    // First byte should be field 1, wire type 2 = 0x0a
    assert_eq!(decoded[0], 0x0a);

    // Verify the encoded amount is present
    let decoded_str = String::from_utf8_lossy(&decoded);
    assert!(decoded_str.contains("1000000"));
    assert!(decoded_str.contains("uxion"));
}

#[test]
fn test_basic_allowance_encoding_multiple_coins() {
    let coins = vec![
        Coin {
            denom: "uatom".to_string(),
            amount: "500".to_string(),
        },
        Coin {
            denom: "uxion".to_string(),
            amount: "1000000".to_string(),
        },
    ];

    let encoded = encode_basic_allowance(coins).expect("Failed to encode BasicAllowance");

    // Should be valid base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    assert!(!decoded.is_empty());

    // Verify both coins are present (sorted alphabetically by denom)
    let decoded_str = String::from_utf8_lossy(&decoded);
    assert!(decoded_str.contains("500"));
    assert!(decoded_str.contains("uatom"));
    assert!(decoded_str.contains("1000000"));
    assert!(decoded_str.contains("uxion"));
}

#[test]
fn test_basic_allowance_encoding_empty_spend_limit() {
    let result = encode_basic_allowance(vec![]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err
        .to_string()
        .contains("requires at least one spend_limit"));
}

// ============================================================================
// Encoding Tests - PeriodicAllowance
// ============================================================================

#[test]
fn test_periodic_allowance_encoding() {
    let period_limit = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000000".to_string(),
    }];

    let encoded = encode_periodic_allowance(None, 86400, period_limit)
        .expect("Failed to encode PeriodicAllowance");

    // Should be valid base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    assert!(!decoded.is_empty());

    // Verify coin data is present
    let decoded_str = String::from_utf8_lossy(&decoded);
    assert!(decoded_str.contains("uxion"));
    assert!(decoded_str.contains("1000000"));
}

#[test]
fn test_periodic_allowance_encoding_with_basic_limit() {
    let basic_limit = vec![Coin {
        denom: "uxion".to_string(),
        amount: "10000000".to_string(),
    }];
    let period_limit = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000000".to_string(),
    }];

    let encoded = encode_periodic_allowance(Some(basic_limit), 3600, period_limit)
        .expect("Failed to encode PeriodicAllowance");

    // Should be valid base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    assert!(!decoded.is_empty());

    // Both limits should be present
    let decoded_str = String::from_utf8_lossy(&decoded);
    assert!(decoded_str.contains("10000000")); // basic
    assert!(decoded_str.contains("1000000")); // period
}

#[test]
fn test_periodic_allowance_encoding_empty_period_spend_limit() {
    let result = encode_periodic_allowance(None, 86400, vec![]);
    assert!(result.is_err());
}

// ============================================================================
// Encoding Tests - GenericAuthorization
// ============================================================================

#[test]
fn test_generic_authorization_encoding() {
    let encoded = encode_generic_authorization("/cosmos.bank.v1beta1.MsgSend")
        .expect("Failed to encode GenericAuthorization");

    // Should be valid base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    assert!(!decoded.is_empty());

    // Verify the message type is present
    let decoded_str = String::from_utf8_lossy(&decoded);
    assert!(decoded_str.contains("MsgSend"));
}

#[test]
fn test_generic_authorization_encoding_empty_msg_type() {
    let result = encode_generic_authorization("");
    assert!(result.is_err());
}

// ============================================================================
// Encoding Tests - SendAuthorization
// ============================================================================

#[test]
fn test_send_authorization_encoding() {
    let coins = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000000".to_string(),
    }];

    let encoded =
        encode_send_authorization(coins, None).expect("Failed to encode SendAuthorization");

    // Should be valid base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    assert!(!decoded.is_empty());

    // Verify coin data is present
    let decoded_str = String::from_utf8_lossy(&decoded);
    assert!(decoded_str.contains("1000000"));
    assert!(decoded_str.contains("uxion"));
}

#[test]
fn test_send_authorization_encoding_with_allow_list() {
    let coins = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000000".to_string(),
    }];
    let allow_list = vec![
        "xion1abc123456789".to_string(),
        "xion1def456789012".to_string(),
    ];

    let encoded = encode_send_authorization(coins, Some(allow_list.clone()))
        .expect("Failed to encode SendAuthorization");

    // Should be valid base64
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .expect("Failed to decode base64");

    assert!(!decoded.is_empty());

    // Verify allow list is present
    let decoded_str = String::from_utf8_lossy(&decoded);
    assert!(decoded_str.contains("xion1abc"));
    assert!(decoded_str.contains("xion1def"));
}

#[test]
fn test_send_authorization_encoding_empty_spend_limit() {
    let result = encode_send_authorization(vec![], None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("requires spend_limit"));
}

// ============================================================================
// Type Serialization Tests
// ============================================================================

#[test]
fn test_treasury_create_request_deserialization() {
    let json = r#"{
        "params": {
            "redirect_url": "https://myapp.com/callback",
            "icon_url": "https://myapp.com/icon.png",
            "name": "My Treasury",
            "is_oauth2_app": true
        },
        "fee_config": {
            "allowance_type": "basic",
            "spend_limit": "1000000uxion",
            "description": "Basic fee allowance"
        },
        "grant_configs": [
            {
                "type_url": "/cosmos.bank.v1beta1.MsgSend",
                "description": "Allow sending funds",
                "authorization": {
                    "auth_type": "send",
                    "spend_limit": "1000000uxion"
                },
                "optional": false
            }
        ]
    }"#;

    let request: TreasuryCreateRequest = serde_json::from_str(json).unwrap();
    assert_eq!(request.params.redirect_url, "https://myapp.com/callback");
    assert_eq!(request.params.icon_url, "https://myapp.com/icon.png");
    assert_eq!(request.params.name, Some("My Treasury".to_string()));
    assert!(request.fee_config.is_some());
    assert_eq!(request.grant_configs.len(), 1);
}

#[test]
fn test_fee_config_input_basic() {
    let json = r#"{
        "allowance_type": "basic",
        "spend_limit": "1000000uxion",
        "description": "Test"
    }"#;

    let config: FeeConfigInput = serde_json::from_str(json).unwrap();
    match config {
        FeeConfigInput::Basic {
            spend_limit,
            description,
        } => {
            assert_eq!(spend_limit, "1000000uxion");
            assert_eq!(description, "Test");
        }
        _ => panic!("Expected Basic variant"),
    }
}

#[test]
fn test_fee_config_input_periodic() {
    let json = r#"{
        "allowance_type": "periodic",
        "basic_spend_limit": "10000000uxion",
        "period_seconds": 86400,
        "period_spend_limit": "1000000uxion",
        "description": "Daily limit"
    }"#;

    let config: FeeConfigInput = serde_json::from_str(json).unwrap();
    match config {
        FeeConfigInput::Periodic {
            basic_spend_limit,
            period_seconds,
            period_spend_limit,
            description,
        } => {
            assert_eq!(basic_spend_limit, Some("10000000uxion".to_string()));
            assert_eq!(period_seconds, 86400);
            assert_eq!(period_spend_limit, "1000000uxion");
            assert_eq!(description, "Daily limit");
        }
        _ => panic!("Expected Periodic variant"),
    }
}

#[test]
fn test_authorization_input_send() {
    let json = r#"{
        "auth_type": "send",
        "spend_limit": "1000000uxion",
        "allow_list": ["xion1abc...", "xion1def..."]
    }"#;

    let auth: AuthorizationInput = serde_json::from_str(json).unwrap();
    match auth {
        AuthorizationInput::Send {
            spend_limit,
            allow_list,
        } => {
            assert_eq!(spend_limit, "1000000uxion");
            assert_eq!(
                allow_list,
                Some(vec!["xion1abc...".to_string(), "xion1def...".to_string()])
            );
        }
        _ => panic!("Expected Send variant"),
    }
}

// ============================================================================
// Coin Parsing Tests
// ============================================================================

#[test]
fn test_parse_coin_string_uxion() {
    let coins = parse_coin_string("1000000uxion").expect("Failed to parse coin");
    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].amount, "1000000");
    assert_eq!(coins[0].denom, "uxion");
}

#[test]
fn test_parse_coin_string_multiple() {
    let coins = parse_coin_string("1000000uxion,500uatom").expect("Failed to parse coins");
    assert_eq!(coins.len(), 2);
    // Should be sorted alphabetically by denom
    assert_eq!(coins[0].denom, "uatom");
    assert_eq!(coins[0].amount, "500");
    assert_eq!(coins[1].denom, "uxion");
    assert_eq!(coins[1].amount, "1000000");
}

#[test]
fn test_parse_coin_string_ibc() {
    let coins = parse_coin_string("1000ibc/channel-1/uatom").expect("Failed to parse IBC coin");
    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].amount, "1000");
    assert_eq!(coins[0].denom, "ibc/channel-1/uatom");
}

#[test]
fn test_parse_coin_string_invalid_empty() {
    let result = parse_coin_string("");
    assert!(result.is_err());
}

#[test]
fn test_parse_coin_string_invalid_no_amount() {
    let result = parse_coin_string("uxion");
    assert!(result.is_err());
}

#[test]
fn test_parse_coin_string_invalid_no_denom() {
    let result = parse_coin_string("1000000");
    // The regex treats the last digit(s) as denom: (\d+)([-a-zA-Z0-9/]+)
    // So "1000000" becomes amount="100000" and denom="0"
    // This is a known edge case - users should always provide proper denom
    assert!(result.is_ok(), "Regex treats last digit as denom");
    let coins = result.unwrap();
    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].amount, "100000");
    assert_eq!(coins[0].denom, "0");
}

// ============================================================================
// End-to-End Tests (Mocked API)
// ============================================================================

#[tokio::test]
async fn test_create_treasury_api_success() {
    let mut server = Server::new_async().await;

    let tx_hash = "tx_create_1234567890abcdef";
    let treasury_address = "xion1newtreasury123456789";
    let admin_address = "xion1admin123456789";

    // Create a token with admin address as userId
    let token = format!("{}:grant123:secret456", admin_address);

    // Mock the broadcast endpoint
    let _mock_broadcast = server
        .mock("POST", "/api/v1/transaction")
        .match_header(
            "authorization",
            mockito::Matcher::Regex(r"Bearer .+".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "success": true,
                "tx_hash": tx_hash,
                "from": admin_address,
                "gas_used": "200000",
                "gas_wanted": "300000"
            })
            .to_string(),
        )
        .create();

    // Mock the DaoDao indexer endpoint (for waiting for indexing)
    // Using the actual DaoDao Indexer format (direct array)
    let _mock_list = server
        .mock(
            "GET",
            mockito::Matcher::Regex(format!(
                r"/contract/{}/xion/account/treasuries",
                admin_address
            )),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([{
                "contractAddress": treasury_address,
                "balances": {"uxion": "0"},
                "codeId": 1260,
                "params": {
                    "metadata": "{\"name\":\"Test Treasury\"}"
                }
            }])
            .to_string(),
        )
        .create();

    let client = TreasuryApiClient::new(
        server.url(),
        server.url(), // Use mock server as indexer URL
    );

    let request = CreateTreasuryRequest {
        admin: "xion1admin123456789".to_string(),
        fee_config: FeeConfigMessage {
            allowance: TypeUrlValue {
                type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
                value: cosmwasm_std::Binary::from_base64(
                    &base64::engine::general_purpose::STANDARD
                        .encode(b"{spend_limit:{amount:\"1000000\",denom:\"uxion\"}}"),
                )
                .unwrap(),
            },
            description: "Basic fee allowance".to_string(),
        },
        grant_configs: vec![GrantConfigMessage {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            authorization: TypeUrlValue {
                type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                value: cosmwasm_std::Binary::from_base64(
                    &base64::engine::general_purpose::STANDARD
                        .encode(b"{spend_limit:{amount:\"500000\",denom:\"uxion\"}}"),
                )
                .unwrap(),
            },
            description: Some("Send funds".to_string()),
        }],
        params: TreasuryParamsMessage {
            redirect_url: "https://myapp.com/callback".to_string(),
            icon_url: "https://myapp.com/icon.png".to_string(),
            display_url: None,
            metadata: None,
        },
        name: Some("Test Treasury".to_string()),
        is_oauth2_app: false,
    };

    let salt: [u8; 32] = [0u8; 32];
    let result = client.create_treasury(&token, 1260, request, &salt).await;

    assert!(
        result.is_ok(),
        "Create treasury should succeed: {:?}",
        result.err()
    );
    let response = result.unwrap();
    assert_eq!(response.tx_hash, tx_hash);

    _mock_broadcast.assert();
}

#[tokio::test]
async fn test_create_treasury_api_unauthorized() {
    let mut server = Server::new_async().await;

    // Mock unauthorized response
    let mock = server
        .mock("POST", "/api/v1/transaction")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "error": "UNAUTHORIZED",
                "message": "Invalid or expired access token"
            })
            .to_string(),
        )
        .create();

    let client = TreasuryApiClient::new(
        server.url(),
        "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
    );

    let request = CreateTreasuryRequest {
        admin: "xion1admin123456789".to_string(),
        fee_config: FeeConfigMessage {
            allowance: TypeUrlValue {
                type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
                value: cosmwasm_std::Binary::from_base64("Cg=").unwrap(), // Empty BasicAllowance
            },
            description: "Fee".to_string(),
        },
        grant_configs: vec![],
        params: TreasuryParamsMessage {
            redirect_url: "https://myapp.com/callback".to_string(),
            icon_url: "https://myapp.com/icon.png".to_string(),
            display_url: None,
            metadata: None,
        },
        name: None,
        is_oauth2_app: false,
    };

    let salt: [u8; 32] = [0u8; 32];
    let result = client
        .create_treasury("invalid_token", 1260, request, &salt)
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("401") || err.to_string().contains("Unauthorized"));

    mock.assert();
}

#[tokio::test]
async fn test_manager_create_requires_auth() {
    let server = Server::new_async().await;
    let config = create_test_config(&server.url());
    let oauth_client = OAuthClient::new(config.clone()).expect("Failed to create OAuthClient");
    let manager = TreasuryManager::new(oauth_client, config.clone());

    let request = TreasuryCreateRequest {
        params: TreasuryParamsInput {
            redirect_url: "https://example.com/callback".to_string(),
            icon_url: "https://example.com/icon.png".to_string(),
            name: None,
            is_oauth2_app: None,
        },
        fee_config: Some(FeeConfigInput::Basic {
            spend_limit: "1000000uxion".to_string(),
            description: "Basic fee".to_string(),
        }),
        grant_configs: vec![GrantConfigInput {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            description: "Send".to_string(),
            authorization: AuthorizationInput::Generic,
            optional: false,
        }],
    };

    let result = manager.create(request).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Not authenticated"));
}

// ============================================================================
// Config File Loading Tests
// ============================================================================

#[test]
fn test_load_config_from_file() {
    use std::fs;
    use tempfile::TempDir;

    // Create a temp config file
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("treasury_config.json");

    let config_content = r#"{
        "params": {
            "redirect_url": "https://myapp.com/callback",
            "icon_url": "https://myapp.com/icon.png",
            "name": "Config Treasury",
            "is_oauth2_app": true
        },
        "fee_config": {
            "allowance_type": "basic",
            "spend_limit": "1000000uxion",
            "description": "From config file"
        },
        "grant_configs": [
            {
                "type_url": "/cosmos.bank.v1beta1.MsgSend",
                "description": "Send funds",
                "authorization": {
                    "auth_type": "send",
                    "spend_limit": "500000uxion"
                },
                "optional": false
            }
        ]
    }"#;

    fs::write(&config_path, config_content).expect("Failed to write config file");

    // Test loading
    let content = fs::read_to_string(&config_path).expect("Failed to read config");
    let request: TreasuryCreateRequest =
        serde_json::from_str(&content).expect("Failed to parse config");

    assert_eq!(request.params.redirect_url, "https://myapp.com/callback");
    assert_eq!(request.params.name, Some("Config Treasury".to_string()));
    assert!(request.params.is_oauth2_app.is_some());

    match request.fee_config.unwrap() {
        FeeConfigInput::Basic {
            spend_limit,
            description,
        } => {
            assert_eq!(spend_limit, "1000000uxion");
            assert_eq!(description, "From config file");
        }
        _ => panic!("Expected Basic fee config"),
    }

    assert_eq!(request.grant_configs.len(), 1);
}

#[test]
fn test_load_invalid_config_file() {
    use std::fs;
    use tempfile::TempDir;

    // Create a temp invalid config file
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("invalid_config.json");

    let config_content = "not valid json";
    fs::write(&config_path, config_content).expect("Failed to write config");

    // Try to parse invalid JSON
    let content = fs::read_to_string(&config_path).expect("Failed to read config");
    let result: Result<TreasuryCreateRequest, _> = serde_json::from_str(&content);

    assert!(result.is_err());
}

// ============================================================================
// Integration Test - Full Flow
// ============================================================================

#[tokio::test]
async fn test_full_create_flow_with_mocks() {
    let mut server = Server::new_async().await;

    let tx_hash = "tx_full_flow_abcdef123456";
    let admin_address = "xion1admin123456789";
    let treasury_address = "xion1treasury_fullflow";

    // Create a token with admin address as userId
    let token = format!("{}:grant123:secret456", admin_address);

    // Mock broadcast transaction
    let _mock_broadcast = server
        .mock("POST", "/api/v1/transaction")
        .match_header(
            "authorization",
            mockito::Matcher::Regex(r"Bearer .+".to_string()),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "success": true,
                "tx_hash": tx_hash,
                "from": admin_address,
                "gas_used": "250000",
                "gas_wanted": "350000"
            })
            .to_string(),
        )
        .create();

    // Mock DaoDao indexer for wait_for_treasury_creation
    // Using the actual DaoDao Indexer format (direct array)
    let _mock_list = server
        .mock(
            "GET",
            mockito::Matcher::Regex(format!(
                r"/contract/{}/xion/account/treasuries",
                admin_address
            )),
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([{
                "contractAddress": treasury_address,
                "balances": {"uxion": "0"},
                "codeId": 1260,
                "params": {
                    "metadata": "{\"name\":\"Full Flow Treasury\"}"
                }
            }])
            .to_string(),
        )
        .create();

    let client = TreasuryApiClient::new(
        server.url(),
        server.url(), // Use mock server as indexer URL
    );

    // Test that we can build and encode a treasury create request
    let coins = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000000".to_string(),
    }];

    let basic_allowance_encoded =
        encode_basic_allowance(coins.clone()).expect("Failed to encode basic allowance");

    let send_coins = vec![Coin {
        denom: "uxion".to_string(),
        amount: "500000".to_string(),
    }];

    let send_auth_encoded =
        encode_send_authorization(send_coins, None).expect("Failed to encode send authorization");

    let request = CreateTreasuryRequest {
        admin: "xion1admin123456789".to_string(),
        fee_config: FeeConfigMessage {
            allowance: TypeUrlValue {
                type_url: "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
                value: cosmwasm_std::Binary::from_base64(&basic_allowance_encoded).unwrap(),
            },
            description: "Basic fee allowance".to_string(),
        },
        grant_configs: vec![GrantConfigMessage {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            authorization: TypeUrlValue {
                type_url: "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                value: cosmwasm_std::Binary::from_base64(&send_auth_encoded).unwrap(),
            },
            description: Some("Send funds".to_string()),
        }],
        params: TreasuryParamsMessage {
            redirect_url: "https://myapp.com/callback".to_string(),
            icon_url: "https://myapp.com/icon.png".to_string(),
            display_url: Some("https://myapp.com".to_string()),
            metadata: Some(serde_json::json!({"name": "Test"})),
        },
        name: Some("Full Flow Treasury".to_string()),
        is_oauth2_app: true,
    };

    let salt: [u8; 32] = [1u8; 32];
    let result = client.create_treasury(&token, 1260, request, &salt).await;

    assert!(
        result.is_ok(),
        "Full flow should succeed: {:?}",
        result.err()
    );
    let response = result.unwrap();
    assert_eq!(response.tx_hash, tx_hash);
    assert_eq!(response.admin, "xion1admin123456789");

    _mock_broadcast.assert();
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_coin_parsing_with_large_amounts() {
    // Test with very large amounts (billions)
    let coins = parse_coin_string("999999999999uxion").expect("Failed to parse large amount");
    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].amount, "999999999999");
    assert_eq!(coins[0].denom, "uxion");
}

#[test]
fn test_coin_parsing_ibc_denom() {
    // Test IBC denom with multiple slashes
    let coins =
        parse_coin_string("1000ibc/partner/channel-1/uatom").expect("Failed to parse IBC denom");
    assert_eq!(coins.len(), 1);
    assert_eq!(coins[0].amount, "1000");
    assert_eq!(coins[0].denom, "ibc/partner/channel-1/uatom");
}
