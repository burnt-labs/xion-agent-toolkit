//! MGR API Client
//!
//! Client for communicating with Xion's MGR API endpoints.
//! Provides OAuth2 client lifecycle management (CRUD), extension management,
//! manager management, ownership transfer, and treasury queries.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use crate::shared::error::{OAuthClientError, XionError, XionResult};
use crate::shared::retry::{is_retryable_status, with_retry, RetryConfig};

// ============================================================================
// Request Types
// ============================================================================

/// Request body for creating a new OAuth client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClientRequest {
    /// OAuth redirect URIs (required, min 1)
    pub redirect_uris: Vec<String>,
    /// Human-readable client name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// Client homepage URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    /// Client logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    /// Privacy policy URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_uri: Option<String>,
    /// Terms of service URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_uri: Option<String>,
    /// JWKS endpoint URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
    /// Contact email addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,
    /// Token endpoint auth method (none, client_secret_basic, client_secret_post)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<String>,
    /// Treasury contract address to bind (required)
    pub binded_treasury: String,
    /// Owner user ID (defaults to authenticated user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Manager user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
}

/// Request body for updating an existing OAuth client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClientRequest {
    /// OAuth redirect URIs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    /// Human-readable client name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// Client homepage URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    /// Client logo URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    /// Privacy policy URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_uri: Option<String>,
    /// Terms of service URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_uri: Option<String>,
    /// JWKS endpoint URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
    /// Contact email addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,
}

/// Request body for updating a client extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExtensionRequest {
    /// Manager user IDs
    pub managers: Vec<String>,
}

/// Request body for adding a manager
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddManagerRequest {
    /// User ID of the manager to add
    pub manager_user_id: String,
}

/// Request body for transferring ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferOwnershipRequest {
    /// User ID of the new owner
    pub new_owner: String,
}

// ============================================================================
// Response Types
// ============================================================================

/// Client extension data (treasury binding, owner, managers)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientExtension {
    /// Treasury contract address bound to this client
    pub binded_treasury: String,
    /// Owner user ID
    pub owner: String,
    /// Manager user IDs
    #[serde(default)]
    pub managers: Vec<String>,
}

/// OAuth client information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    /// Unique client identifier
    pub client_id: String,
    /// OAuth redirect URIs
    #[serde(default)]
    pub redirect_uris: Vec<String>,
    /// Human-readable client name
    #[serde(default)]
    pub client_name: Option<String>,
    /// Client homepage URL
    #[serde(default)]
    pub client_uri: Option<String>,
    /// Client logo URL
    #[serde(default)]
    pub logo_uri: Option<String>,
    /// Privacy policy URL
    #[serde(default)]
    pub policy_uri: Option<String>,
    /// Terms of service URL
    #[serde(default)]
    pub tos_uri: Option<String>,
    /// JWKS endpoint URL
    #[serde(default)]
    pub jwks_uri: Option<String>,
    /// Contact email addresses
    #[serde(default)]
    pub contacts: Option<Vec<String>>,
    /// Grant types
    #[serde(default)]
    pub grant_types: Option<Vec<String>>,
    /// Response types
    #[serde(default)]
    pub response_types: Option<Vec<String>>,
    /// Token endpoint auth method
    #[serde(default)]
    pub token_endpoint_auth_method: Option<String>,
    /// Client extension data
    #[serde(default)]
    pub extension: Option<ClientExtension>,
    /// User's role for this client (e.g., "owner")
    #[serde(default)]
    pub role: Option<String>,
}

/// Response for list clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientListResponse {
    /// Whether the request was successful
    pub success: bool,
    /// List of client info objects
    pub items: Vec<ClientInfo>,
    /// Pagination cursor for next page
    #[serde(default)]
    pub cursor: Option<String>,
    /// Number of items returned
    #[serde(default)]
    pub count: usize,
}

/// Response containing a single client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Client information
    pub client: ClientInfo,
}

/// Response for client creation (includes secret)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClientResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Client information
    pub client: ClientInfo,
    /// Client secret (only returned on creation)
    pub client_secret: Option<String>,
}

/// Response for extension data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Extension data
    pub extension: ClientExtension,
}

/// Simple message response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Response message
    pub message: String,
}

/// User info from /mgr-api/me
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeUserInfo {
    /// User ID
    pub user_id: String,
    /// Xion blockchain address
    #[serde(default)]
    pub xion_address: Option<String>,
    /// Authenticator type (undefined in OAuth mode)
    #[serde(default)]
    pub authenticator_type: Option<String>,
    /// Authenticator index (undefined in OAuth mode)
    #[serde(default)]
    pub authenticator_index: Option<u32>,
    /// Network name
    #[serde(default)]
    pub network: Option<String>,
}

/// Response for /mgr-api/me
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    /// Whether the request was successful
    pub success: bool,
    /// User ID
    pub user_id: String,
    /// Detailed user information
    pub user: MeUserInfo,
}

/// Treasury info (simplified for MGR API responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasuryInfo {
    /// Treasury contract address
    pub address: String,
    /// Admin address
    #[serde(default)]
    pub admin: Option<String>,
    /// Treasury parameters
    #[serde(default)]
    pub params: serde_json::Value,
    /// Treasury balance
    #[serde(default)]
    pub balance: Option<String>,
    /// Pending admin address
    #[serde(default)]
    pub pending_admin: Option<String>,
    /// Fee configuration
    #[serde(default)]
    pub fee_config: Option<serde_json::Value>,
    /// Grant configurations
    #[serde(default)]
    pub grant_configs: Option<Vec<serde_json::Value>>,
}

/// Response for list treasuries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryListResponse {
    /// Whether the request was successful
    pub success: bool,
    /// List of treasury info objects
    #[serde(default)]
    pub treasuries: Vec<TreasuryInfo>,
    /// Number of items returned
    #[serde(default)]
    pub count: usize,
}

/// Response for single treasury query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryResponse {
    /// Whether the request was successful
    pub success: bool,
    /// Treasury information
    pub treasury: TreasuryInfo,
}

/// Backend error response body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendError {
    /// Error message
    pub error: String,
    /// Backend error code
    #[serde(default)]
    pub code: Option<String>,
    /// HTTP status code
    #[serde(default)]
    pub status_code: Option<u16>,
}

// ============================================================================
// MGR API Client
// ============================================================================

/// MGR API Client for Xion
///
/// Handles communication with the MGR API endpoints for:
/// - OAuth client CRUD operations
/// - Client extension management
/// - Manager management
/// - Ownership transfer
/// - Treasury queries
#[derive(Debug, Clone)]
pub struct MgrApiClient {
    /// Base URL of the OAuth2 API service (MGR endpoints under this)
    base_url: String,
    /// HTTP client for making requests
    http_client: Client,
}

impl MgrApiClient {
    /// Create a new MGR API client
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the OAuth2 API service (e.g., "https://oauth2.testnet.burnt.com")
    pub fn new(base_url: String) -> Result<Self, XionError> {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| {
                XionError::from(OAuthClientError::NetworkError {
                    message: format!("Failed to create HTTP client: {}", e),
                })
            })?;

        Ok(Self {
            base_url,
            http_client,
        })
    }

    // ========================================================================
    // Core CRUD
    // ========================================================================

    /// List OAuth clients for the authenticated user
    #[instrument(skip(self, access_token))]
    pub async fn list_clients(
        &self,
        access_token: &str,
        limit: Option<u32>,
        cursor: Option<&str>,
    ) -> XionResult<ClientListResponse> {
        debug!(
            "Listing OAuth clients (limit={:?}, cursor={:?})",
            limit, cursor
        );

        let mut query_params = Vec::new();
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        if let Some(c) = cursor {
            query_params.push(format!("cursor={}", c));
        }

        let path = if query_params.is_empty() {
            "/mgr-api/clients".to_string()
        } else {
            format!("/mgr-api/clients?{}", query_params.join("&"))
        };

        self.request("GET", &path, access_token, None).await
    }

    /// Create a new OAuth client
    #[instrument(skip(self, access_token, request))]
    pub async fn create_client(
        &self,
        access_token: &str,
        request: CreateClientRequest,
    ) -> XionResult<CreateClientResponse> {
        debug!("Creating OAuth client: {:?}", request.client_name);

        let body = serde_json::to_value(&request).map_err(|e| {
            XionError::from(OAuthClientError::InvalidResponse {
                message: e.to_string(),
            })
        })?;

        self.request("POST", "/mgr-api/clients", access_token, Some(&body))
            .await
    }

    /// Get a specific OAuth client by ID
    #[instrument(skip(self, access_token))]
    pub async fn get_client(
        &self,
        access_token: &str,
        client_id: &str,
    ) -> XionResult<ClientResponse> {
        debug!("Getting OAuth client: {}", client_id);

        let path = format!("/mgr-api/clients/{}", client_id);
        self.request("GET", &path, access_token, None).await
    }

    /// Update an existing OAuth client
    #[instrument(skip(self, access_token, request))]
    pub async fn update_client(
        &self,
        access_token: &str,
        client_id: &str,
        request: UpdateClientRequest,
    ) -> XionResult<ClientResponse> {
        debug!("Updating OAuth client: {}", client_id);

        let body = serde_json::to_value(&request).map_err(|e| {
            XionError::from(OAuthClientError::InvalidResponse {
                message: e.to_string(),
            })
        })?;

        let path = format!("/mgr-api/clients/{}", client_id);
        self.request("PUT", &path, access_token, Some(&body)).await
    }

    /// Delete an OAuth client
    #[instrument(skip(self, access_token))]
    pub async fn delete_client(
        &self,
        access_token: &str,
        client_id: &str,
    ) -> XionResult<MessageResponse> {
        debug!("Deleting OAuth client: {}", client_id);

        let path = format!("/mgr-api/clients/{}", client_id);
        self.request("DELETE", &path, access_token, None).await
    }

    // ========================================================================
    // Extension
    // ========================================================================

    /// Get the extension data for a client
    #[instrument(skip(self, access_token))]
    pub async fn get_extension(
        &self,
        access_token: &str,
        client_id: &str,
    ) -> XionResult<ExtensionResponse> {
        debug!("Getting extension for client: {}", client_id);

        let path = format!("/mgr-api/clients/{}/extension", client_id);
        self.request("GET", &path, access_token, None).await
    }

    /// Update the extension data for a client
    #[instrument(skip(self, access_token, request))]
    pub async fn update_extension(
        &self,
        access_token: &str,
        client_id: &str,
        request: UpdateExtensionRequest,
    ) -> XionResult<ClientResponse> {
        debug!("Updating extension for client: {}", client_id);

        let body = serde_json::to_value(&request).map_err(|e| {
            XionError::from(OAuthClientError::InvalidResponse {
                message: e.to_string(),
            })
        })?;

        let path = format!("/mgr-api/clients/{}/extension", client_id);
        self.request("PATCH", &path, access_token, Some(&body))
            .await
    }

    // ========================================================================
    // Managers
    // ========================================================================

    /// Add a manager to a client
    #[instrument(skip(self, access_token))]
    pub async fn add_manager(
        &self,
        access_token: &str,
        client_id: &str,
        manager_user_id: &str,
    ) -> XionResult<MessageResponse> {
        debug!(
            "Adding manager {} to client: {}",
            manager_user_id, client_id
        );

        let body = serde_json::to_value(AddManagerRequest {
            manager_user_id: manager_user_id.to_string(),
        })
        .map_err(|e| {
            XionError::from(OAuthClientError::InvalidResponse {
                message: e.to_string(),
            })
        })?;

        let path = format!("/mgr-api/clients/{}/managers", client_id);
        self.request("POST", &path, access_token, Some(&body)).await
    }

    /// Remove a manager from a client
    #[instrument(skip(self, access_token))]
    pub async fn remove_manager(
        &self,
        access_token: &str,
        client_id: &str,
        manager_user_id: &str,
    ) -> XionResult<MessageResponse> {
        debug!(
            "Removing manager {} from client: {}",
            manager_user_id, client_id
        );

        let path = format!(
            "/mgr-api/clients/{}/managers/{}",
            client_id, manager_user_id
        );
        self.request("DELETE", &path, access_token, None).await
    }

    // ========================================================================
    // Ownership
    // ========================================================================

    /// Transfer ownership of a client
    #[instrument(skip(self, access_token))]
    pub async fn transfer_ownership(
        &self,
        access_token: &str,
        client_id: &str,
        new_owner: &str,
    ) -> XionResult<MessageResponse> {
        debug!("Transferring ownership of {} to {}", client_id, new_owner);

        let body = serde_json::to_value(TransferOwnershipRequest {
            new_owner: new_owner.to_string(),
        })
        .map_err(|e| {
            XionError::from(OAuthClientError::InvalidResponse {
                message: e.to_string(),
            })
        })?;

        let path = format!("/mgr-api/clients/{}/transfer-ownership", client_id);
        self.request("POST", &path, access_token, Some(&body)).await
    }

    // ========================================================================
    // User Info
    // ========================================================================

    /// Get the current authenticated user info
    #[instrument(skip(self, access_token))]
    pub async fn get_me(&self, access_token: &str) -> XionResult<MeResponse> {
        debug!("Getting current user info from /mgr-api/me");

        self.request("GET", "/mgr-api/me", access_token, None).await
    }

    // ========================================================================
    // Treasury Queries (extra endpoints beyond the 11 core methods)
    // ========================================================================

    /// List treasuries for the authenticated user
    #[instrument(skip(self, access_token))]
    pub async fn list_treasuries(&self, access_token: &str) -> XionResult<TreasuryListResponse> {
        debug!("Listing treasuries from /mgr-api/treasuries");

        self.request("GET", "/mgr-api/treasuries", access_token, None)
            .await
    }

    /// Query a specific treasury by address
    #[instrument(skip(self, access_token))]
    pub async fn query_treasury(
        &self,
        access_token: &str,
        address: &str,
        grants: bool,
        fee: bool,
        admin: bool,
    ) -> XionResult<TreasuryResponse> {
        debug!("Querying treasury: {}", address);

        let mut params = Vec::new();
        if grants {
            params.push("grants=true".to_string());
        }
        if fee {
            params.push("fee=true".to_string());
        }
        if admin {
            params.push("admin=true".to_string());
        }

        let path = if params.is_empty() {
            format!("/mgr-api/utils/treasury/{}", address)
        } else {
            format!("/mgr-api/utils/treasury/{}?{}", address, params.join("&"))
        };

        self.request("GET", &path, access_token, None).await
    }

    // ========================================================================
    // Private Helpers
    // ========================================================================

    /// Make an HTTP request to the MGR API
    ///
    /// Handles authentication, retry logic, and error mapping.
    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        access_token: &str,
        body: Option<&serde_json::Value>,
    ) -> XionResult<T> {
        let url = format!("{}{}", self.base_url, path);
        let config = RetryConfig::default();

        with_retry(
            &config,
            || {
                let url = url.clone();
                let token = access_token.to_string();
                let body = body.cloned();

                async move {
                    let mut req = match method {
                        "GET" => self.http_client.get(&url),
                        "POST" => self.http_client.post(&url),
                        "PUT" => self.http_client.put(&url),
                        "PATCH" => self.http_client.patch(&url),
                        "DELETE" => self.http_client.delete(&url),
                        _ => {
                            return Err(XionError::from(OAuthClientError::InvalidResponse {
                                message: format!("Unsupported HTTP method: {}", method),
                            }));
                        }
                    };

                    req = req
                        .header("Authorization", format!("Bearer {}", token))
                        .header("Content-Type", "application/json");

                    if let Some(b) = &body {
                        req = req.json(b);
                    }

                    let response = req.send().await.map_err(|e| {
                        if e.is_timeout() {
                            XionError::from(OAuthClientError::NetworkError {
                                message: format!("Request timed out: {}", e),
                            })
                        } else if e.is_connect() {
                            XionError::from(OAuthClientError::NetworkError {
                                message: format!("Connection failed: {}", e),
                            })
                        } else {
                            XionError::from(OAuthClientError::NetworkError {
                                message: format!("Request failed: {}", e),
                            })
                        }
                    })?;

                    let status = response.status().as_u16();
                    debug!("MGR API response: {} {}", method, status);

                    if !response.status().is_success() {
                        // Try to parse error body
                        let error_body = response.text().await.unwrap_or_default();
                        let error_value: serde_json::Value = serde_json::from_str(&error_body)
                            .unwrap_or(serde_json::json!({
                                "error": error_body,
                                "code": null,
                                "statusCode": status
                            }));

                        let mapped = Self::map_error(status, &error_value);

                        // Only retry on transient errors
                        if is_retryable_status(status) {
                            // Return the error; retry will happen if the status is retryable
                            return Err(mapped);
                        }

                        return Err(mapped);
                    }

                    // Parse successful response
                    response.json::<T>().await.map_err(|e| {
                        XionError::from(OAuthClientError::InvalidResponse {
                            message: format!("Failed to parse response: {}", e),
                        })
                    })
                }
            },
            |err: &XionError| {
                matches!(
                    err,
                    XionError::OAuthClient(OAuthClientError::NetworkError { .. })
                        | XionError::OAuthClient(OAuthClientError::ServerError { .. })
                )
            },
        )
        .await
    }

    /// Map a backend error response to an `OAuthClientError`
    ///
    /// Parses the backend error code and HTTP status to produce
    /// the appropriate toolkit error variant.
    pub fn map_error(status: u16, body: &serde_json::Value) -> XionError {
        let error_msg = body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error")
            .to_string();

        let backend_code = body
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        match (status, backend_code.as_str()) {
            // 400 errors
            (400, "BAD_REQUEST") => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),
            (400, "CLIENT_ID_REQUIRED") => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),
            (400, "REDIRECT_URIS_REQUIRED") => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),
            (400, "BINDED_TREASURY_REQUIRED") => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),
            (400, "OWNER_REQUIRED") => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),
            (400, "INVALID_GRANT_TYPE") => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),
            (400, "MANAGER_USER_ID_REQUIRED") => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),

            // 401 errors
            (401, "AUTHENTICATION_REQUIRED") => {
                XionError::from(OAuthClientError::AuthenticationRequired { message: error_msg })
            }
            (401, "USER_NOT_FOUND") => {
                XionError::from(OAuthClientError::UserNotFound { message: error_msg })
            }

            // 403 errors
            (403, "INSUFFICIENT_SCOPE") => {
                XionError::from(OAuthClientError::InsufficientScope { message: error_msg })
            }
            (403, "ONLY_OWNER_ALLOWED") => {
                XionError::from(OAuthClientError::OnlyOwnerAllowed { message: error_msg })
            }

            // 404 errors
            (404, "CLIENT_NOT_FOUND") => XionError::from(OAuthClientError::ClientNotFound {
                client_id: error_msg,
            }),
            (404, "CLIENT_EXTENSION_NOT_FOUND") => {
                XionError::from(OAuthClientError::ClientExtensionNotFound {
                    client_id: error_msg,
                })
            }
            (404, "TREASURY_NOT_FOUND") => {
                XionError::from(OAuthClientError::TreasuryNotFound { address: error_msg })
            }

            // 500 errors
            (500, "INTERNAL_SERVER_ERROR") => XionError::from(OAuthClientError::ServerError {
                code: backend_code,
                message: error_msg,
            }),
            (500, "TREASURY_FETCH_ERROR") => XionError::from(OAuthClientError::ServerError {
                code: backend_code,
                message: error_msg,
            }),
            (500, "TREASURY_QUERY_ERROR") => XionError::from(OAuthClientError::ServerError {
                code: backend_code,
                message: error_msg,
            }),
            (500, "UNKNOWN_NETWORK") => XionError::from(OAuthClientError::ServerError {
                code: backend_code,
                message: error_msg,
            }),

            // Fallback: map by HTTP status class
            (401, _) => {
                XionError::from(OAuthClientError::AuthenticationRequired { message: error_msg })
            }
            (403, _) => XionError::from(OAuthClientError::InsufficientScope { message: error_msg }),
            (404, _) => XionError::from(OAuthClientError::InvalidResponse {
                message: format!("Resource not found: {}", error_msg),
            }),
            (status, _) if is_retryable_status(status) => {
                XionError::from(OAuthClientError::ServerError {
                    code: backend_code,
                    message: error_msg,
                })
            }
            _ => XionError::from(OAuthClientError::BadRequest {
                code: backend_code,
                message: error_msg,
            }),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // map_error() tests — covering all 18 backend error codes
    // =========================================================================

    #[test]
    fn test_map_error_bad_request() {
        let body = serde_json::json!({
            "error": "Bad request",
            "code": "BAD_REQUEST",
            "statusCode": 400
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Bad request"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_client_id_required() {
        let body = serde_json::json!({
            "error": "Client ID is required",
            "code": "CLIENT_ID_REQUIRED",
            "statusCode": 400
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Client ID is required"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_redirect_uris_required() {
        let body = serde_json::json!({
            "error": "Redirect URIs are required",
            "code": "REDIRECT_URIS_REQUIRED",
            "statusCode": 400
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Redirect URIs are required"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_binded_treasury_required() {
        let body = serde_json::json!({
            "error": "Binded treasury is required",
            "code": "BINDED_TREASURY_REQUIRED",
            "statusCode": 400
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Binded treasury is required"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_owner_required() {
        let body = serde_json::json!({
            "error": "Owner is required",
            "code": "OWNER_REQUIRED",
            "statusCode": 400
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Owner is required"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_invalid_grant_type() {
        let body = serde_json::json!({
            "error": "Invalid grant type",
            "code": "INVALID_GRANT_TYPE",
            "statusCode": 400
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid grant type"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_manager_user_id_required() {
        let body = serde_json::json!({
            "error": "Manager user ID is required",
            "code": "MANAGER_USER_ID_REQUIRED",
            "statusCode": 400
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Manager user ID is required"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_authentication_required() {
        let body = serde_json::json!({
            "error": "Authentication required",
            "code": "AUTHENTICATION_REQUIRED",
            "statusCode": 401
        });
        let err = MgrApiClient::map_error(401, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Authentication required"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_user_not_found() {
        let body = serde_json::json!({
            "error": "User not found",
            "code": "USER_NOT_FOUND",
            "statusCode": 401
        });
        let err = MgrApiClient::map_error(401, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("User not found"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_insufficient_scope() {
        let body = serde_json::json!({
            "error": "Insufficient scope",
            "code": "INSUFFICIENT_SCOPE",
            "statusCode": 403
        });
        let err = MgrApiClient::map_error(403, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Insufficient scope"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_only_owner_allowed() {
        let body = serde_json::json!({
            "error": "Only owner allowed",
            "code": "ONLY_OWNER_ALLOWED",
            "statusCode": 403
        });
        let err = MgrApiClient::map_error(403, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Only owner allowed"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_client_not_found() {
        let body = serde_json::json!({
            "error": "Client not found: client_abc",
            "code": "CLIENT_NOT_FOUND",
            "statusCode": 404
        });
        let err = MgrApiClient::map_error(404, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Client not found"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_client_extension_not_found() {
        let body = serde_json::json!({
            "error": "Client extension not found: client_abc",
            "code": "CLIENT_EXTENSION_NOT_FOUND",
            "statusCode": 404
        });
        let err = MgrApiClient::map_error(404, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Client extension not found"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_treasury_not_found() {
        let body = serde_json::json!({
            "error": "Treasury not found: xion1abc",
            "code": "TREASURY_NOT_FOUND",
            "statusCode": 404
        });
        let err = MgrApiClient::map_error(404, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Treasury not found"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_internal_server_error() {
        let body = serde_json::json!({
            "error": "Internal server error",
            "code": "INTERNAL_SERVER_ERROR",
            "statusCode": 500
        });
        let err = MgrApiClient::map_error(500, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Internal server error"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_treasury_fetch_error() {
        let body = serde_json::json!({
            "error": "Failed to fetch treasury",
            "code": "TREASURY_FETCH_ERROR",
            "statusCode": 500
        });
        let err = MgrApiClient::map_error(500, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Failed to fetch treasury"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_treasury_query_error() {
        let body = serde_json::json!({
            "error": "Failed to query treasury",
            "code": "TREASURY_QUERY_ERROR",
            "statusCode": 500
        });
        let err = MgrApiClient::map_error(500, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Failed to query treasury"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_unknown_network() {
        let body = serde_json::json!({
            "error": "Unknown network",
            "code": "UNKNOWN_NETWORK",
            "statusCode": 500
        });
        let err = MgrApiClient::map_error(500, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Unknown network"), "got: {}", msg);
    }

    // =========================================================================
    // Fallback error mapping tests
    // =========================================================================

    #[test]
    fn test_map_error_401_fallback() {
        let body = serde_json::json!({
            "error": "Unauthorized",
            "code": "CUSTOM_AUTH_ERROR",
            "statusCode": 401
        });
        let err = MgrApiClient::map_error(401, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Unauthorized"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_403_fallback() {
        let body = serde_json::json!({
            "error": "Forbidden",
            "code": "CUSTOM_FORBIDDEN",
            "statusCode": 403
        });
        let err = MgrApiClient::map_error(403, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Forbidden"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_429_retryable() {
        let body = serde_json::json!({
            "error": "Too many requests",
            "code": "RATE_LIMITED",
            "statusCode": 429
        });
        let err = MgrApiClient::map_error(429, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("Too many requests"), "got: {}", msg);
    }

    #[test]
    fn test_map_error_unknown_body() {
        let body = serde_json::json!({
            "error": "something went wrong"
        });
        let err = MgrApiClient::map_error(400, &body);
        let msg = format!("{}", err);
        assert!(msg.contains("something went wrong"), "got: {}", msg);
    }

    // =========================================================================
    // Type serialization tests
    // =========================================================================

    #[test]
    fn test_client_info_serialization() {
        let client = ClientInfo {
            client_id: "client_abc123".to_string(),
            redirect_uris: vec!["https://example.com/callback".to_string()],
            client_name: Some("My App".to_string()),
            client_uri: Some("https://myapp.com".to_string()),
            logo_uri: None,
            policy_uri: None,
            tos_uri: None,
            jwks_uri: None,
            contacts: Some(vec!["admin@myapp.com".to_string()]),
            grant_types: Some(vec!["authorization_code".to_string()]),
            response_types: Some(vec!["code".to_string()]),
            token_endpoint_auth_method: Some("none".to_string()),
            extension: Some(ClientExtension {
                binded_treasury: "xion1abc123".to_string(),
                owner: "user_123".to_string(),
                managers: vec!["user_456".to_string()],
            }),
            role: Some("owner".to_string()),
        };

        let json = serde_json::to_value(&client).unwrap();
        assert_eq!(json["clientId"], "client_abc123");
        assert_eq!(json["redirectUris"][0], "https://example.com/callback");
        assert_eq!(json["clientName"], "My App");
        assert_eq!(json["extension"]["bindedTreasury"], "xion1abc123");
        assert_eq!(json["extension"]["managers"][0], "user_456");
    }

    #[test]
    fn test_client_info_deserialization() {
        let json = r#"{
            "clientId": "client_abc123",
            "redirectUris": ["https://example.com/callback"],
            "clientName": "My App",
            "extension": {
                "bindedTreasury": "xion1abc123",
                "owner": "user_123",
                "managers": ["user_456", "user_789"]
            },
            "role": "owner"
        }"#;

        let client: ClientInfo = serde_json::from_str(json).unwrap();
        assert_eq!(client.client_id, "client_abc123");
        assert_eq!(client.redirect_uris.len(), 1);
        assert_eq!(client.client_name.as_deref(), Some("My App"));
        assert!(client.extension.is_some());
        let ext = client.extension.unwrap();
        assert_eq!(ext.binded_treasury, "xion1abc123");
        assert_eq!(ext.managers.len(), 2);
    }

    #[test]
    fn test_client_info_minimal_deserialization() {
        let json = r#"{
            "clientId": "client_minimal"
        }"#;

        let client: ClientInfo = serde_json::from_str(json).unwrap();
        assert_eq!(client.client_id, "client_minimal");
        assert!(client.redirect_uris.is_empty());
        assert!(client.client_name.is_none());
        assert!(client.extension.is_none());
        assert!(client.role.is_none());
    }

    #[test]
    fn test_create_client_request_serialization() {
        let request = CreateClientRequest {
            redirect_uris: vec!["https://example.com/callback".to_string()],
            client_name: Some("My App".to_string()),
            client_uri: None,
            logo_uri: None,
            policy_uri: None,
            tos_uri: None,
            jwks_uri: None,
            contacts: None,
            token_endpoint_auth_method: Some("none".to_string()),
            binded_treasury: "xion1treasury".to_string(),
            owner: None,
            managers: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["redirectUris"][0], "https://example.com/callback");
        assert_eq!(json["bindedTreasury"], "xion1treasury");
        assert_eq!(json["tokenEndpointAuthMethod"], "none");
        assert!(json.get("clientUri").is_none());
        assert!(json.get("owner").is_none());
    }

    #[test]
    fn test_update_client_request_serialization() {
        let request = UpdateClientRequest {
            redirect_uris: Some(vec!["https://new.example.com/callback".to_string()]),
            client_name: Some("Updated App".to_string()),
            client_uri: None,
            logo_uri: None,
            policy_uri: None,
            tos_uri: None,
            jwks_uri: None,
            contacts: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["redirectUris"][0], "https://new.example.com/callback");
        assert_eq!(json["clientName"], "Updated App");
        // None fields should not be serialized
        assert!(json.get("logoUri").is_none());
    }

    #[test]
    fn test_create_client_response_deserialization() {
        let json = r#"{
            "success": true,
            "client": {
                "clientId": "client_new",
                "redirectUris": ["https://example.com/callback"],
                "clientName": "My App"
            },
            "clientSecret": "secret_xyz789"
        }"#;

        let response: CreateClientResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert_eq!(response.client.client_id, "client_new");
        assert_eq!(response.client_secret, Some("secret_xyz789".to_string()));
    }

    #[test]
    fn test_me_response_deserialization() {
        let json = r#"{
            "success": true,
            "userId": "xion1abc123",
            "user": {
                "userId": "xion1abc123",
                "xionAddress": "xion1abc123",
                "authenticatorType": "EthWallet",
                "authenticatorIndex": 0,
                "network": "testnet"
            }
        }"#;

        let response: MeResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert_eq!(response.user_id, "xion1abc123");
        assert_eq!(response.user.xion_address.as_deref(), Some("xion1abc123"));
        assert_eq!(
            response.user.authenticator_type.as_deref(),
            Some("EthWallet")
        );
    }

    #[test]
    fn test_me_response_minimal_deserialization() {
        let json = r#"{
            "success": true,
            "userId": "xion1abc123",
            "user": {
                "userId": "xion1abc123"
            }
        }"#;

        let response: MeResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert!(response.user.authenticator_type.is_none());
        assert!(response.user.network.is_none());
    }

    #[test]
    fn test_client_list_response_deserialization() {
        let json = r#"{
            "success": true,
            "items": [
                {
                    "clientId": "client_1",
                    "redirectUris": ["https://example.com/callback"],
                    "clientName": "App 1",
                    "role": "owner"
                }
            ],
            "cursor": "next_cursor",
            "count": 1
        }"#;

        let response: ClientListResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.cursor.as_deref(), Some("next_cursor"));
        assert_eq!(response.count, 1);
    }

    #[test]
    fn test_message_response_deserialization() {
        let json = r#"{
            "success": true,
            "message": "Client deleted successfully"
        }"#;

        let response: MessageResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert_eq!(response.message, "Client deleted successfully");
    }

    #[test]
    fn test_extension_response_deserialization() {
        let json = r#"{
            "success": true,
            "extension": {
                "bindedTreasury": "xion1treasury",
                "owner": "user_123",
                "managers": ["user_456"]
            }
        }"#;

        let response: ExtensionResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert_eq!(response.extension.binded_treasury, "xion1treasury");
        assert_eq!(response.extension.managers.len(), 1);
    }

    #[test]
    fn test_treasury_info_deserialization() {
        let json = r#"{
            "success": true,
            "treasury": {
                "address": "xion1treasury",
                "admin": "xion1admin",
                "balance": "1000000",
                "params": {
                    "display_url": "https://example.com"
                },
                "feeConfig": {},
                "grantConfigs": []
            }
        }"#;

        let response: TreasuryResponse = serde_json::from_str(json).unwrap();
        assert!(response.success);
        assert_eq!(response.treasury.address, "xion1treasury");
        assert_eq!(response.treasury.balance.as_deref(), Some("1000000"));
    }

    // =========================================================================
    // Client creation test
    // =========================================================================

    #[test]
    fn test_mgr_api_client_creation() {
        let client = MgrApiClient::new("https://oauth2.testnet.burnt.com".to_string()).unwrap();
        assert_eq!(client.base_url, "https://oauth2.testnet.burnt.com");
    }

    // =========================================================================
    // Integration tests with wiremock
    // =========================================================================

    mod wiremock_tests {
        use super::*;
        use wiremock::matchers::{header, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        #[tokio::test]
        async fn test_get_me_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "userId": "xion1abc123",
                "user": {
                    "userId": "xion1abc123",
                    "xionAddress": "xion1abc123",
                    "network": "testnet"
                }
            });

            Mock::given(method("GET"))
                .and(path("/mgr-api/me"))
                .and(header("Authorization", "Bearer test_token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client.get_me("test_token").await;

            assert!(result.is_ok());
            let me = result.unwrap();
            assert_eq!(me.user_id, "xion1abc123");
        }

        #[tokio::test]
        async fn test_list_clients_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "items": [
                    {
                        "clientId": "client_1",
                        "clientName": "App 1",
                        "role": "owner"
                    }
                ],
                "cursor": "next_cursor",
                "count": 1
            });

            Mock::given(method("GET"))
                .and(path("/mgr-api/clients"))
                .and(header("Authorization", "Bearer test_token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client.list_clients("test_token", Some(10), None).await;

            assert!(result.is_ok());
            let list = result.unwrap();
            assert_eq!(list.items.len(), 1);
            assert_eq!(list.items[0].client_id, "client_1");
        }

        #[tokio::test]
        async fn test_create_client_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "client": {
                    "clientId": "client_new",
                    "redirectUris": ["https://example.com/callback"],
                    "clientName": "My App",
                    "extension": {
                        "bindedTreasury": "xion1treasury",
                        "owner": "user_123",
                        "managers": []
                    },
                    "role": "owner"
                },
                "clientSecret": "secret_xyz"
            });

            Mock::given(method("POST"))
                .and(path("/mgr-api/clients"))
                .and(header("Authorization", "Bearer test_token"))
                .respond_with(ResponseTemplate::new(201).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let request = CreateClientRequest {
                redirect_uris: vec!["https://example.com/callback".to_string()],
                binded_treasury: "xion1treasury".to_string(),
                client_name: Some("My App".to_string()),
                client_uri: None,
                logo_uri: None,
                policy_uri: None,
                tos_uri: None,
                jwks_uri: None,
                contacts: None,
                token_endpoint_auth_method: None,
                owner: None,
                managers: None,
            };

            let result = client.create_client("test_token", request).await;
            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.client.client_id, "client_new");
            assert_eq!(resp.client_secret, Some("secret_xyz".to_string()));
        }

        #[tokio::test]
        async fn test_get_client_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "client": {
                    "clientId": "client_abc",
                    "clientName": "My App"
                }
            });

            Mock::given(method("GET"))
                .and(path("/mgr-api/clients/client_abc"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client.get_client("test_token", "client_abc").await;

            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.client.client_id, "client_abc");
        }

        #[tokio::test]
        async fn test_update_client_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "client": {
                    "clientId": "client_abc",
                    "clientName": "Updated App"
                }
            });

            Mock::given(method("PUT"))
                .and(path("/mgr-api/clients/client_abc"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let request = UpdateClientRequest {
                client_name: Some("Updated App".to_string()),
                redirect_uris: None,
                client_uri: None,
                logo_uri: None,
                policy_uri: None,
                tos_uri: None,
                jwks_uri: None,
                contacts: None,
            };

            let result = client
                .update_client("test_token", "client_abc", request)
                .await;

            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.client.client_name.as_deref(), Some("Updated App"));
        }

        #[tokio::test]
        async fn test_delete_client_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "message": "Client deleted successfully"
            });

            Mock::given(method("DELETE"))
                .and(path("/mgr-api/clients/client_abc"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client.delete_client("test_token", "client_abc").await;

            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.message, "Client deleted successfully");
        }

        #[tokio::test]
        async fn test_get_extension_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "extension": {
                    "bindedTreasury": "xion1treasury",
                    "owner": "user_123",
                    "managers": ["user_456"]
                }
            });

            Mock::given(method("GET"))
                .and(path("/mgr-api/clients/client_abc/extension"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client.get_extension("test_token", "client_abc").await;

            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.extension.binded_treasury, "xion1treasury");
        }

        #[tokio::test]
        async fn test_add_manager_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "message": "Manager added successfully"
            });

            Mock::given(method("POST"))
                .and(path("/mgr-api/clients/client_abc/managers"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client
                .add_manager("test_token", "client_abc", "user_456")
                .await;

            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.message, "Manager added successfully");
        }

        #[tokio::test]
        async fn test_remove_manager_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "message": "Manager removed successfully"
            });

            Mock::given(method("DELETE"))
                .and(path("/mgr-api/clients/client_abc/managers/user_456"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client
                .remove_manager("test_token", "client_abc", "user_456")
                .await;

            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.message, "Manager removed successfully");
        }

        #[tokio::test]
        async fn test_transfer_ownership_success() {
            let mock_server = MockServer::start().await;

            let response = serde_json::json!({
                "success": true,
                "message": "Ownership transferred successfully"
            });

            Mock::given(method("POST"))
                .and(path("/mgr-api/clients/client_abc/transfer-ownership"))
                .respond_with(ResponseTemplate::new(200).set_body_json(response))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client
                .transfer_ownership("test_token", "client_abc", "user_789")
                .await;

            assert!(result.is_ok());
            let resp = result.unwrap();
            assert_eq!(resp.message, "Ownership transferred successfully");
        }

        #[tokio::test]
        async fn test_auth_error_response() {
            let mock_server = MockServer::start().await;

            let error = serde_json::json!({
                "error": "Authentication required",
                "code": "AUTHENTICATION_REQUIRED",
                "statusCode": 401
            });

            Mock::given(method("GET"))
                .and(path("/mgr-api/me"))
                .respond_with(ResponseTemplate::new(401).set_body_json(error))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client.get_me("invalid_token").await;

            assert!(result.is_err());
            let err = result.unwrap_err();
            let msg = format!("{}", err);
            assert!(msg.contains("Authentication required"), "got: {}", msg);
        }

        #[tokio::test]
        async fn test_client_not_found_error() {
            let mock_server = MockServer::start().await;

            let error = serde_json::json!({
                "error": "Client not found: client_missing",
                "code": "CLIENT_NOT_FOUND",
                "statusCode": 404
            });

            Mock::given(method("GET"))
                .and(path("/mgr-api/clients/client_missing"))
                .respond_with(ResponseTemplate::new(404).set_body_json(error))
                .mount(&mock_server)
                .await;

            let client = MgrApiClient::new(mock_server.uri()).unwrap();
            let result = client.get_client("test_token", "client_missing").await;

            assert!(result.is_err());
            let err = result.unwrap_err();
            let msg = format!("{}", err);
            assert!(msg.contains("Client not found"), "got: {}", msg);
        }
    }
}
