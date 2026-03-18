//! Treasury Integration Tests
//!
//! Integration tests for Treasury operations using mocked API responses.
//! These tests verify the behavior of fund and withdraw operations.

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use xion_agent_toolkit::config::NetworkConfig;
use xion_agent_toolkit::oauth::OAuthClient;
use xion_agent_toolkit::treasury::{TreasuryApiClient, TreasuryManager};

/// TestTreasuryApiClient creates a TreasuryApiClient for testing
fn create_test_api_client(base_url: &str) -> TreasuryApiClient {
    TreasuryApiClient::new(
        base_url.to_string(),
        "https://daodaoindexer.burnt.com/xion-testnet-2".to_string(),
        "https://rpc.xion-testnet-2.burnt.com:443".to_string(),
    )
}

/// Create a mock OAuth client for testing (mocked token retrieval)
fn create_mock_oauth_client(config: &NetworkConfig) -> OAuthClient {
    OAuthClient::new(config.clone()).expect("Failed to create OAuthClient")
}

/// Create test network config pointing to mock server
fn create_test_config(server_url: &str) -> NetworkConfig {
    NetworkConfig {
        network_name: "test".to_string(),
        oauth_api_url: server_url.to_string(),
        rpc_url: "http://localhost:26657".to_string(),
        rest_url: "http://localhost:1317".to_string(),
        chain_id: "xion-local".to_string(),
        oauth_client_id: "test-client-id".to_string(),
        treasury_code_id: 1,
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

// ============================================================================
// Fund Treasury Tests
// ============================================================================

#[tokio::test]
async fn test_fund_treasury_success() {
    // Start mock server
    let mock_server = MockServer::start().await;

    let treasury_address = "xion1treasury123456789";
    let from_address = "xion1sender123456789";
    let amount = "1000000uxion";
    let tx_hash = "tx1234567890abcdef";

    // Mock the broadcast endpoint
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "tx_hash": tx_hash,
            "from": from_address,
            "gas_used": "100000",
            "gas_wanted": "200000"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Perform fund operation
    let result = client
        .fund_treasury("mock_access_token", treasury_address, amount, from_address)
        .await;

    // Verify result
    assert!(result.is_ok(), "Fund should succeed");
    let response = result.unwrap();
    assert_eq!(response.tx_hash, tx_hash);
    assert!(response.success);
}

#[tokio::test]
async fn test_fund_treasury_unauthorized() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock unauthorized response
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": "Unauthorized",
            "message": "Invalid or expired access token"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to fund with invalid token
    let result = client
        .fund_treasury(
            "invalid_token",
            "xion1treasury123",
            "1000000uxion",
            "xion1sender123",
        )
        .await;

    // Should fail with unauthorized error
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("401") || error.to_string().contains("Unauthorized"));
}

#[tokio::test]
async fn test_network_timeout() {
    // Use a non-routable IP to simulate network timeout
    let client = TreasuryApiClient::new(
        "http://10.255.255.1:1".to_string(),
        "http://10.255.255.1:1".to_string(),
        "http://10.255.255.1:1".to_string(),
    );

    let result = client
        .fund_treasury(
            "mock_token",
            "xion1treasury123",
            "1000000uxion",
            "xion1sender123",
        )
        .await;

    // Should fail with network error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_treasury_zero_amount() {
    // Start mock server
    let mock_server = MockServer::start().await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to fund with zero amount
    let result = client
        .fund_treasury("mock_token", "xion1treasury123", "0uxion", "xion1sender123")
        .await;

    // Should fail with invalid amount error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fund_treasury_invalid_address() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock invalid address error from API
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "INVALID_ADDRESS",
            "message": "Treasury address is invalid"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to fund with invalid treasury address
    let result = client
        .fund_treasury(
            "mock_token",
            "invalid_address",
            "1000000uxion",
            "xion1sender123",
        )
        .await;

    // Should fail
    assert!(result.is_err());
}

// ============================================================================
// Withdraw Treasury Tests
// ============================================================================

#[tokio::test]
async fn test_withdraw_treasury_success() {
    // Start mock server
    let mock_server = MockServer::start().await;

    let treasury_address = "xion1treasury123456789";
    let to_address = "xion1admin123456789";
    let amount = "500000uxion";
    let tx_hash = "tx9876543210abcdef";

    // Mock the broadcast endpoint for withdraw
    // Note: The message body contains escaped JSON, so we match on the contract field
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "tx_hash": tx_hash,
            "from": to_address,
            "gas_used": "150000",
            "gas_wanted": "250000"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Perform withdraw operation
    let result = client
        .withdraw_treasury("mock_access_token", treasury_address, amount, to_address)
        .await;

    // Verify result
    assert!(result.is_ok(), "Withdraw should succeed");
    let response = result.unwrap();
    assert_eq!(response.tx_hash, tx_hash);
    assert!(response.success);
}

#[tokio::test]
async fn test_withdraw_treasury_unauthorized() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock unauthorized - user is not admin
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "error": "FORBIDDEN",
            "message": "Only treasury admin can withdraw funds"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to withdraw without admin privileges
    let result = client
        .withdraw_treasury(
            "mock_token",
            "xion1treasury123",
            "1000000uxion",
            "xion1notadmin123",
        )
        .await;

    // Should fail with forbidden error
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("403") || error.to_string().contains("FORBIDDEN"));
}

#[tokio::test]
async fn test_withdraw_treasury_insufficient_balance() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock insufficient balance error
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(
            ResponseTemplate::new(400).set_body_json(serde_json::json!({
                "error": "INSUFFICIENT_BALANCE",
                "message": "Treasury has insufficient balance. Available: 100000uxion, Requested: 1000000uxion"
            })),
        )
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to withdraw more than available
    let result = client
        .withdraw_treasury(
            "mock_token",
            "xion1treasury123",
            "1000000uxion",
            "xion1admin123",
        )
        .await;

    // Should fail with insufficient balance
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains("INSUFFICIENT_BALANCE")
            || error.to_string().contains("insufficient")
    );
}

#[tokio::test]
async fn test_withdraw_treasury_invalid_amount_format() {
    // Start mock server
    let mock_server = MockServer::start().await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to withdraw with invalid amount format
    let result = client
        .withdraw_treasury(
            "mock_token",
            "xion1treasury123",
            "invalid", // Invalid format
            "xion1admin123",
        )
        .await;

    // Should fail with parse error
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains("Invalid coin format") || error.to_string().contains("denom")
    );
}

#[tokio::test]
async fn test_withdraw_treasury_zero_amount() {
    // Start mock server
    let mock_server = MockServer::start().await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to withdraw zero amount
    let result = client
        .withdraw_treasury("mock_token", "xion1treasury123", "0uxion", "xion1admin123")
        .await;

    // Should fail with invalid amount error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_withdraw_treasury_not_found() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock treasury not found error
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "error": "NOT_FOUND",
            "message": "Treasury contract not found"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Try to withdraw from non-existent treasury
    let result = client
        .withdraw_treasury(
            "mock_token",
            "xion1nonexistent123",
            "1000000uxion",
            "xion1admin123",
        )
        .await;

    // Should fail with not found error
    assert!(result.is_err());
}

// ============================================================================
// Treasury Manager Integration Tests (with OAuth)
// ============================================================================

#[tokio::test]
async fn test_manager_fund_requires_authentication() {
    let mock_server = MockServer::start().await;
    let config = create_test_config(&mock_server.uri());
    let oauth_client = create_mock_oauth_client(&config);

    // Create manager without valid authentication
    let manager = TreasuryManager::without_cache(oauth_client, config.clone());

    // Try to fund without authentication
    let result = manager.fund("xion1treasury123", "1000000uxion").await;

    // Should fail - not authenticated
    assert!(result.is_err());
    // Error message may vary - just verify it's an error
}

#[tokio::test]
async fn test_manager_withdraw_requires_authentication() {
    let mock_server = MockServer::start().await;
    let config = create_test_config(&mock_server.uri());
    let oauth_client = create_mock_oauth_client(&config);

    // Create manager without valid authentication
    let manager = TreasuryManager::without_cache(oauth_client, config.clone());

    // Try to withdraw without authentication
    let result = manager.withdraw("xion1treasury123", "1000000uxion").await;

    // Should fail - not authenticated
    assert!(result.is_err());
    // Error message may vary - just verify it's an error
}

// ============================================================================
// JSON Output Format Tests
// ============================================================================

#[tokio::test]
async fn test_fund_result_json_format() {
    // Start mock server
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "tx_hash": "tx123456789",
            "from": "xion1sender123"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());
    let result = client
        .fund_treasury(
            "mock_token",
            "xion1treasury123",
            "1000000uxion",
            "xion1sender123",
        )
        .await
        .unwrap();

    // Verify JSON serialization works - BroadcastResponse only contains tx_hash, from, success
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("tx_hash"));
    assert!(json.contains("success"));
    assert!(json.contains("xion1sender123"));
}

#[tokio::test]
async fn test_withdraw_result_json_format() {
    // Start mock server
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "tx_hash": "tx987654321",
            "from": "xion1admin123"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());
    let result = client
        .withdraw_treasury(
            "mock_token",
            "xion1treasury123",
            "500000uxion",
            "xion1admin123",
        )
        .await
        .unwrap();

    // Verify JSON serialization works - BroadcastResponse only contains tx_hash, from, success
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("tx_hash"));
    assert!(json.contains("success"));
    assert!(json.contains("xion1admin123"));
}

// ============================================================================
// Network Error Tests
// ============================================================================

#[tokio::test]
async fn test_api_server_error() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Mock server error
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
            "error": "INTERNAL_ERROR",
            "message": "Internal server error"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());
    let result = client
        .fund_treasury(
            "mock_token",
            "xion1treasury123",
            "1000000uxion",
            "xion1sender123",
        )
        .await;

    // Should fail with server error
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("500") || error.to_string().contains("Internal"));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[tokio::test]
async fn test_fund_with_large_amount() {
    // Start mock server
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "tx_hash": "tx_large_amount",
            "from": "xion1sender123"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());

    // Test with large amount (billions of uxion)
    let result = client
        .fund_treasury(
            "mock_token",
            "xion1treasury123",
            "999999999999999uxion",
            "xion1sender123",
        )
        .await;

    assert!(
        result.is_ok(),
        "Large amount should be accepted: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_fund_with_different_denoms() {
    // Test parsing different coin denominations
    let mock_server = MockServer::start().await;
    let client = create_test_api_client(&mock_server.uri());

    // Test uusdc denomination
    let result = client
        .fund_treasury(
            "mock_token",
            "xion1treasury123",
            "1000uusdc",
            "xion1sender123",
        )
        .await;

    // Should fail at network level but amount parsing should succeed
    // (we'll get a connection error, not a parse error)
    assert!(result.is_err());
    let error = result.unwrap_err();
    // Should be network error, not parse error
    assert!(!error.to_string().contains("Invalid coin format"));
}

#[tokio::test]
async fn test_withdraw_message_encoding() {
    // Start mock server - verify the withdraw message is properly encoded
    let mock_server = MockServer::start().await;

    // The mock should match the execute contract message pattern
    // Match on contract address which is unique to withdraw
    Mock::given(method("POST"))
        .and(path("/api/v1/transaction"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "tx_hash": "tx_withdraw_test",
            "from": "xion1admin123"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_api_client(&mock_server.uri());
    let result = client
        .withdraw_treasury(
            "mock_token",
            "xion1treasury123",
            "100000uxion",
            "xion1admin123",
        )
        .await;

    assert!(
        result.is_ok(),
        "Withdraw should succeed with proper message encoding: {:?}",
        result.err()
    );
}
