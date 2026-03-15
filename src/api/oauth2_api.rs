//! OAuth2 API Client
//!
//! Client for communicating with Xion's OAuth2 API Service.
//! Supports token exchange, refresh, and user info retrieval.

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use crate::shared::error::{AuthError, NetworkError, XionResult};

/// OAuth2 API Client for Xion
///
/// Handles communication with the OAuth2 service for:
/// - Token exchange (authorization code -> access token)
/// - Token refresh
/// - User info retrieval
#[derive(Debug, Clone)]
pub struct OAuth2ApiClient {
    /// Base URL of the OAuth2 API service
    base_url: String,
    /// HTTP client for making requests
    http_client: Client,
}

/// Token request parameters
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TokenRequest {
    /// Grant type (authorization_code or refresh_token)
    pub grant_type: String,
    /// Authorization code (for authorization_code grant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// PKCE code verifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    /// Redirect URI used in authorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    /// Refresh token (for refresh_token grant)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// OAuth2 client ID
    pub client_id: String,
}

/// Token response from OAuth2 service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Access token for API authentication
    pub access_token: String,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// Access token expiration time in seconds
    pub expires_in: i64,
    /// Absolute expiration timestamp for access token (ISO 8601 format)
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Refresh token expiration time in seconds (optional, defaults to 30 days if not provided)
    #[serde(default)]
    pub refresh_token_expires_in: Option<i64>,
    /// Absolute expiration timestamp for refresh token (ISO 8601 format)
    #[serde(default)]
    pub refresh_token_expires_at: Option<String>,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Xion blockchain address associated with the account
    #[serde(default)]
    pub xion_address: Option<String>,
}

impl TokenResponse {
    /// Calculate the absolute expiration time for access token
    ///
    /// Returns the expiration timestamp as an ISO 8601 string
    pub fn calculate_expires_at(&self) -> String {
        let expires_at = Utc::now() + chrono::Duration::seconds(self.expires_in);
        expires_at.to_rfc3339()
    }

    /// Calculate the absolute expiration time for refresh token
    ///
    /// Returns the expiration timestamp as an ISO 8601 string
    /// Uses refresh_token_expires_in if provided, otherwise defaults to 30 days
    pub fn calculate_refresh_token_expires_at(&self) -> String {
        let expires_in = self.refresh_token_expires_in.unwrap_or(30 * 24 * 60 * 60); // Default 30 days
        let expires_at = Utc::now() + chrono::Duration::seconds(expires_in);
        expires_at.to_rfc3339()
    }

    /// Check if the access token is expired
    ///
    /// Returns true if the token has expired or will expire within the next 60 seconds
    #[allow(dead_code)]
    pub fn is_expired(&self) -> bool {
        if let Some(ref expires_at_str) = self.expires_at {
            if let Ok(expires_at) = DateTime::parse_from_rfc3339(expires_at_str) {
                let now = Utc::now() + chrono::Duration::seconds(60);
                return expires_at < now;
            }
        }
        false
    }

    /// Check if the refresh token is expired
    ///
    /// Returns true if the refresh token has expired
    #[allow(dead_code)]
    pub fn is_refresh_token_expired(&self) -> bool {
        if let Some(ref expires_at_str) = self.refresh_token_expires_at {
            if let Ok(expires_at) = DateTime::parse_from_rfc3339(expires_at_str) {
                return expires_at < Utc::now();
            }
        }
        false
    }
}

/// User information from OAuth2 service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// MetaAccount address (same as xion_address)
    pub id: String,
    /// Authenticators associated with the account
    #[serde(default)]
    pub authenticators: Vec<AuthenticatorInfo>,
    /// Account balances
    #[serde(default)]
    pub balances: Option<AccountBalances>,
}

/// Authenticator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorInfo {
    /// Authenticator ID
    pub id: String,
    /// Authenticator type (e.g., "secp256k1")
    #[serde(rename = "type")]
    pub auth_type: String,
    /// Authenticator index
    pub index: u32,
    /// Authenticator data
    pub data: serde_json::Value,
}

/// Account balances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalances {
    /// Xion balance
    pub xion: Balance,
    /// USDC balance
    pub usdc: Balance,
}

/// Balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Human-readable amount
    pub amount: String,
    /// Denomination
    pub denom: String,
    /// Micro amount (smallest unit)
    #[serde(rename = "microAmount")]
    pub micro_amount: String,
}

/// Error response from OAuth2 service
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuth2Error {
    /// Error code
    pub error: String,
    /// Human-readable error description
    #[serde(default)]
    pub error_description: Option<String>,
}

impl OAuth2ApiClient {
    /// Create a new OAuth2 API client
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the OAuth2 API service (e.g., "https://oauth2.testnet.burnt.com")
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::OAuth2ApiClient;
    ///
    /// let client = OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// ```
    pub fn new(base_url: String) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            http_client,
        }
    }

    /// Exchange authorization code for access token
    ///
    /// This is the second step in the OAuth2 authorization flow.
    /// After the user authorizes the application, the callback receives an authorization code.
    /// This code is exchanged for access and refresh tokens.
    ///
    /// # Arguments
    /// * `code` - Authorization code from the OAuth callback
    /// * `code_verifier` - PKCE code verifier used in the authorization request
    /// * `redirect_uri` - The same redirect URI used in the authorization request
    /// * `client_id` - OAuth2 client ID
    ///
    /// # Returns
    /// Token response containing access token, refresh token, and metadata
    ///
    /// # Errors
    /// Returns an error if:
    /// - The authorization code is invalid or expired
    /// - The PKCE verifier doesn't match the challenge
    /// - The redirect URI doesn't match
    /// - Network request fails
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::OAuth2ApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let token = client.exchange_code(
    ///     "auth_code_123",
    ///     "pkce_verifier_123",
    ///     "http://localhost:8080/callback",
    ///     "client_id_123"
    /// ).await?;
    /// println!("Access token: {}", token.access_token);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(dead_code)]
    #[instrument(skip(self, code, code_verifier))]
    pub async fn exchange_code(
        &self,
        code: &str,
        code_verifier: &str,
        redirect_uri: &str,
        client_id: &str,
    ) -> XionResult<TokenResponse> {
        debug!("Exchanging authorization code for token");

        let request = TokenRequest {
            grant_type: "authorization_code".to_string(),
            code: Some(code.to_string()),
            code_verifier: Some(code_verifier.to_string()),
            redirect_uri: Some(redirect_uri.to_string()),
            refresh_token: None,
            client_id: client_id.to_string(),
        };

        self.request_token(&request).await.map_err(|e| {
            AuthError::CallbackFailed(format!(
                "Failed to exchange authorization code for token: {}",
                e
            ))
            .into()
        })
    }

    /// Exchange authorization code for tokens (with custom endpoint)
    ///
    /// Similar to `exchange_code`, but allows specifying a custom token endpoint.
    /// This is useful when endpoints are dynamically discovered via OAuth2 discovery.
    ///
    /// # Arguments
    /// * `code` - Authorization code from the OAuth callback
    /// * `code_verifier` - PKCE code verifier that matches the challenge
    /// * `redirect_uri` - The same redirect URI used in the authorization request
    /// * `client_id` - OAuth2 client ID
    /// * `token_endpoint` - Custom token endpoint URL
    #[instrument(skip(self, code, code_verifier))]
    pub async fn exchange_code_with_endpoint(
        &self,
        code: &str,
        code_verifier: &str,
        redirect_uri: &str,
        client_id: &str,
        token_endpoint: &str,
    ) -> XionResult<TokenResponse> {
        debug!(
            "Exchanging authorization code for token using custom endpoint: {}",
            token_endpoint
        );

        let request = TokenRequest {
            grant_type: "authorization_code".to_string(),
            code: Some(code.to_string()),
            code_verifier: Some(code_verifier.to_string()),
            redirect_uri: Some(redirect_uri.to_string()),
            refresh_token: None,
            client_id: client_id.to_string(),
        };

        self.request_token_with_endpoint(&request, token_endpoint)
            .await
            .map_err(|e| {
                AuthError::CallbackFailed(format!(
                    "Failed to exchange authorization code for token: {}",
                    e
                ))
                .into()
            })
    }

    /// Refresh access token using refresh token
    ///
    /// When an access token expires, use the refresh token to obtain a new access token
    /// without requiring user interaction.
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token obtained from a previous token exchange
    /// * `client_id` - OAuth2 client ID
    ///
    /// # Returns
    /// New token response with fresh access token and possibly new refresh token
    ///
    /// # Errors
    /// Returns an error if:
    /// - The refresh token is invalid or revoked
    /// - Network request fails
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::OAuth2ApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let token = client.refresh_token(
    ///     "refresh_token_123",
    ///     "client_id_123"
    /// ).await?;
    /// println!("New access token: {}", token.access_token);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, refresh_token))]
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
        client_id: &str,
    ) -> XionResult<TokenResponse> {
        debug!("Refreshing access token");

        let request = TokenRequest {
            grant_type: "refresh_token".to_string(),
            code: None,
            code_verifier: None,
            redirect_uri: None,
            refresh_token: Some(refresh_token.to_string()),
            client_id: client_id.to_string(),
        };

        self.request_token(&request)
            .await
            .map_err(|e| AuthError::TokenExpired(format!("Failed to refresh token: {}", e)).into())
    }

    /// Get user information
    ///
    /// Retrieves information about the authenticated user using the access token.
    /// This calls the /api/v1/me endpoint to get the MetaAccount address and other details.
    ///
    /// # Arguments
    /// * `access_token` - Valid access token
    ///
    /// # Returns
    /// User information including MetaAccount address (id) and balances
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - Network request fails
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::api::OAuth2ApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let user_info = client.get_user_info("access_token_123").await?;
    /// println!("MetaAccount address: {}", user_info.id);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn get_user_info(&self, access_token: &str) -> XionResult<UserInfo> {
        debug!("Fetching user info from /api/v1/me");

        let url = format!("{}/api/v1/me", self.base_url);

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| {
                NetworkError::RequestFailed(format!("Failed to send user info request: {}", e))
            })?;

        let status = response.status();
        debug!("User info response status: {}", status);

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(NetworkError::InvalidResponse(format!(
                "Failed to get user info: HTTP {} - {}",
                status, error_text
            ))
            .into());
        }

        let user_info = response.json::<UserInfo>().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse user info response: {}", e))
        })?;

        debug!(
            "Successfully retrieved user info for MetaAccount: {}",
            user_info.id
        );
        Ok(user_info)
    }

    /// Internal method to request tokens from the OAuth2 service
    ///
    /// Makes a POST request to the /oauth/token endpoint
    async fn request_token(&self, request: &TokenRequest) -> XionResult<TokenResponse> {
        let url = format!("{}/oauth/token", self.base_url);

        debug!("Making token request to: {}", url);

        let response = self
            .http_client
            .post(&url)
            .form(request)
            .send()
            .await
            .map_err(|e| {
                NetworkError::RequestFailed(format!("Failed to send token request: {}", e))
            })?;

        let status = response.status();
        debug!("Token response status: {}", status);

        if !status.is_success() {
            // Try to parse error response
            if let Ok(error_text) = response.text().await {
                // Try to parse as OAuth2 error
                if let Ok(oauth_error) = serde_json::from_str::<OAuth2Error>(&error_text) {
                    return Err(AuthError::InvalidCredentials(format!(
                        "OAuth2 error: {} - {}",
                        oauth_error.error,
                        oauth_error
                            .error_description
                            .unwrap_or_else(|| "No description".to_string())
                    ))
                    .into());
                } else {
                    return Err(NetworkError::InvalidResponse(format!(
                        "Token request failed: HTTP {} - {}",
                        status, error_text
                    ))
                    .into());
                }
            } else {
                return Err(NetworkError::InvalidResponse(format!(
                    "Token request failed: HTTP {}",
                    status
                ))
                .into());
            }
        }

        let mut token_response = response.json::<TokenResponse>().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse token response: {}", e))
        })?;

        // Debug: print token prefix
        debug!(
            "Parsed access_token prefix: {}...",
            &token_response
                .access_token
                .chars()
                .take(20)
                .collect::<String>()
        );

        // Calculate and set expires_at if not provided
        if token_response.expires_at.is_none() {
            token_response.expires_at = Some(token_response.calculate_expires_at());
        }

        debug!(
            "Successfully obtained token for address: {:?}",
            token_response.xion_address
        );

        Ok(token_response)
    }

    /// Internal method to request tokens from a custom endpoint
    ///
    /// Similar to `request_token`, but uses a custom token endpoint URL.
    async fn request_token_with_endpoint(
        &self,
        request: &TokenRequest,
        token_endpoint: &str,
    ) -> XionResult<TokenResponse> {
        debug!(
            "Making token request to custom endpoint: {}",
            token_endpoint
        );

        let response = self
            .http_client
            .post(token_endpoint)
            .form(request)
            .send()
            .await
            .map_err(|e| {
                NetworkError::RequestFailed(format!("Failed to send token request: {}", e))
            })?;

        let status = response.status();
        debug!("Token response status: {}", status);

        if !status.is_success() {
            // Try to parse error response
            if let Ok(error_text) = response.text().await {
                // Try to parse as OAuth2 error
                if let Ok(oauth_error) = serde_json::from_str::<OAuth2Error>(&error_text) {
                    return Err(AuthError::InvalidCredentials(format!(
                        "OAuth2 error: {} - {}",
                        oauth_error.error,
                        oauth_error
                            .error_description
                            .unwrap_or_else(|| "No description".to_string())
                    ))
                    .into());
                } else {
                    return Err(NetworkError::InvalidResponse(format!(
                        "Token request failed: HTTP {} - {}",
                        status, error_text
                    ))
                    .into());
                }
            } else {
                return Err(NetworkError::InvalidResponse(format!(
                    "Token request failed: HTTP {}",
                    status
                ))
                .into());
            }
        }

        let mut token_response = response.json::<TokenResponse>().await.map_err(|e| {
            NetworkError::InvalidResponse(format!("Failed to parse token response: {}", e))
        })?;

        // Calculate and set expires_at if not provided
        if token_response.expires_at.is_none() {
            token_response.expires_at = Some(token_response.calculate_expires_at());
        }

        debug!(
            "Successfully obtained token for address: {:?}",
            token_response.xion_address
        );

        Ok(token_response)
    }

    /// Get the base URL
    #[allow(dead_code)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = OAuth2ApiClient::new("https://oauth.test.com".to_string());
        assert_eq!(client.base_url, "https://oauth.test.com");
    }

    #[test]
    fn test_token_response_expires_at() {
        let token = TokenResponse {
            access_token: "test_access".to_string(),
            refresh_token: "test_refresh".to_string(),
            expires_in: 3600,
            expires_at: None,
            refresh_token_expires_in: None,
            refresh_token_expires_at: None,
            token_type: "Bearer".to_string(),
            xion_address: Some("xion1test".to_string()),
        };

        let expires_at = token.calculate_expires_at();
        assert!(expires_at.contains('T')); // ISO 8601 format
    }

    #[test]
    fn test_token_response_is_expired() {
        // Test token that expires in 1 hour
        let mut token = TokenResponse {
            access_token: "test_access".to_string(),
            refresh_token: "test_refresh".to_string(),
            expires_in: 3600,
            expires_at: None,
            refresh_token_expires_in: None,
            refresh_token_expires_at: None,
            token_type: "Bearer".to_string(),
            xion_address: Some("xion1test".to_string()),
        };
        token.expires_at = Some(token.calculate_expires_at());
        assert!(!token.is_expired());

        // Test token that already expired
        let expired_time = Utc::now() - chrono::Duration::seconds(120);
        token.expires_at = Some(expired_time.to_rfc3339());
        assert!(token.is_expired());
    }

    #[test]
    fn test_token_request_serialization() {
        let request = TokenRequest {
            grant_type: "authorization_code".to_string(),
            code: Some("test_code".to_string()),
            code_verifier: Some("test_verifier".to_string()),
            redirect_uri: Some("http://localhost:8080/callback".to_string()),
            refresh_token: None,
            client_id: "test_client".to_string(),
        };

        // Verify serialization doesn't include null fields
        let json = serde_urlencoded::to_string(&request).unwrap();
        assert!(json.contains("grant_type=authorization_code"));
        assert!(json.contains("code=test_code"));
        assert!(json.contains("code_verifier=test_verifier"));
        assert!(!json.contains("refresh_token"));
    }

    #[test]
    fn test_user_info_deserialization() {
        let json = r#"{
            "id": "xion1abc123",
            "authenticators": [
                {
                    "id": "xion1abc123-0",
                    "type": "secp256k1",
                    "index": 0,
                    "data": {}
                }
            ],
            "balances": {
                "xion": {
                    "amount": "100.5",
                    "denom": "uxion",
                    "microAmount": "100500000"
                },
                "usdc": {
                    "amount": "50.0",
                    "denom": "uusdc",
                    "microAmount": "50000000"
                }
            }
        }"#;

        let user_info: UserInfo = serde_json::from_str(json).unwrap();
        assert_eq!(user_info.id, "xion1abc123");
        assert_eq!(user_info.authenticators.len(), 1);
        assert_eq!(user_info.authenticators[0].auth_type, "secp256k1");
        assert!(user_info.balances.is_some());
        let balances = user_info.balances.unwrap();
        assert_eq!(balances.xion.amount, "100.5");
        assert_eq!(balances.usdc.amount, "50.0");
    }

    #[test]
    fn test_user_info_minimal_deserialization() {
        let json = r#"{
            "id": "xion1abc123"
        }"#;

        let user_info: UserInfo = serde_json::from_str(json).unwrap();
        assert_eq!(user_info.id, "xion1abc123");
        assert!(user_info.authenticators.is_empty());
        assert!(user_info.balances.is_none());
    }

    #[test]
    fn test_oauth2_error_deserialization() {
        let json = r#"{
            "error": "invalid_grant",
            "error_description": "The authorization code is invalid"
        }"#;

        let error: OAuth2Error = serde_json::from_str(json).unwrap();
        assert_eq!(error.error, "invalid_grant");
        assert_eq!(
            error.error_description,
            Some("The authorization code is invalid".to_string())
        );
    }

    // Integration tests with mock server using wiremock
    // wiremock is async-friendly and works with tokio runtime
    #[cfg(test)]
    mod integration_tests {
        use super::*;
        use wiremock::matchers::{body_string, header, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        #[tokio::test]
        async fn test_exchange_code_success() {
            // Start mock server
            let mock_server = MockServer::start().await;

            // Mock token endpoint
            let token_response = serde_json::json!({
                "access_token": "test_access_token",
                "refresh_token": "test_refresh_token",
                "expires_in": 3600,
                "token_type": "Bearer",
                "xion_address": "xion1test123"
            });

            Mock::given(method("POST"))
                .and(path("/oauth/token"))
                .and(body_string(
                    "grant_type=authorization_code&code=test_code&code_verifier=test_verifier&redirect_uri=http%3A%2F%2Flocalhost%3A8080%2Fcallback&client_id=test_client"
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(token_response))
                .mount(&mock_server)
                .await;

            // Create client with mock server URL
            let client = OAuth2ApiClient::new(mock_server.uri());

            // Test exchange_code_with_endpoint
            let result = client
                .exchange_code_with_endpoint(
                    "test_code",
                    "test_verifier",
                    "http://localhost:8080/callback",
                    "test_client",
                    &format!("{}/oauth/token", mock_server.uri()),
                )
                .await;

            assert!(result.is_ok());
            let token = result.unwrap();
            assert_eq!(token.access_token, "test_access_token");
            assert_eq!(token.refresh_token, "test_refresh_token");
            assert_eq!(token.expires_in, 3600);
            assert_eq!(token.xion_address, Some("xion1test123".to_string()));
        }

        #[tokio::test]
        async fn test_refresh_token_success() {
            let mock_server = MockServer::start().await;

            let token_response = serde_json::json!({
                "access_token": "new_access_token",
                "refresh_token": "new_refresh_token",
                "expires_in": 3600,
                "token_type": "Bearer",
                "xion_address": "xion1test123"
            });

            Mock::given(method("POST"))
                .and(path("/oauth/token"))
                .and(body_string(
                    "grant_type=refresh_token&refresh_token=old_refresh_token&client_id=test_client"
                ))
                .respond_with(ResponseTemplate::new(200).set_body_json(token_response))
                .mount(&mock_server)
                .await;

            let client = OAuth2ApiClient::new(mock_server.uri());
            let result = client
                .refresh_token("old_refresh_token", "test_client")
                .await;
            assert!(result.is_ok());
            let token = result.unwrap();
            assert_eq!(token.access_token, "new_access_token");
            assert_eq!(token.refresh_token, "new_refresh_token");
            assert_eq!(token.expires_in, 3600);
            assert_eq!(token.xion_address, Some("xion1test123".to_string()));
        }

        #[tokio::test]
        async fn test_get_user_info_success() {
            let mock_server = MockServer::start().await;

            let user_response = serde_json::json!({
                "id": "xion1abc123",
                "authenticators": [
                    {
                        "id": "xion1abc123-0",
                        "type": "secp256k1",
                        "index": 0,
                        "data": {}
                    }
                ],
                "balances": {
                    "xion": {
                        "amount": "100.5",
                        "denom": "uxion",
                        "microAmount": "100500000"
                    },
                    "usdc": {
                        "amount": "50.0",
                        "denom": "uusdc",
                        "microAmount": "50000000"
                    }
                }
            });

            Mock::given(method("GET"))
                .and(path("/api/v1/me"))
                .and(header("Authorization", "Bearer test_access_token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(user_response))
                .mount(&mock_server)
                .await;

            let client = OAuth2ApiClient::new(mock_server.uri());

            let result = client.get_user_info("test_access_token").await;

            assert!(result.is_ok());
            let user_info = result.unwrap();
            assert_eq!(user_info.id, "xion1abc123");
            assert_eq!(user_info.authenticators.len(), 1);
        }

        #[tokio::test]
        async fn test_oauth2_error_response() {
            let mock_server = MockServer::start().await;

            let error_response = serde_json::json!({
                "error": "invalid_grant",
                "error_description": "The authorization code is invalid"
            });

            Mock::given(method("POST"))
                .and(path("/oauth/token"))
                .respond_with(ResponseTemplate::new(400).set_body_json(error_response))
                .mount(&mock_server)
                .await;

            let client = OAuth2ApiClient::new(mock_server.uri());

            let result = client
                .exchange_code_with_endpoint(
                    "invalid_code",
                    "test_verifier",
                    "http://localhost:8080/callback",
                    "test_client",
                    &format!("{}/oauth/token", mock_server.uri()),
                )
                .await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            // Check the error message contains the OAuth2 error
            let error_str = error.to_string();
            assert!(
                error_str.contains("invalid_grant"),
                "Expected 'invalid_grant' in error message, got: {}",
                error_str
            );
        }
    }
}
