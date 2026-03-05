//! Treasury API Client
//!
//! Client for communicating with Xion's Treasury API Service.
//! Supports listing, querying, and managing treasury contracts.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use super::types::{QueryOptions, TreasuryInfo, TreasuryListItem};

/// Treasury API Client for Xion
///
/// Handles communication with the Treasury API service for:
/// - Listing user's treasury contracts
/// - Querying treasury details
/// - Creating new treasury contracts (future)
/// - Funding treasury contracts (future)
#[derive(Debug, Clone)]
pub struct TreasuryApiClient {
    /// Base URL of the Treasury API service
    base_url: String,
    /// HTTP client for making requests
    http_client: Client,
}

/// Treasury list response from API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreasuryListResponse {
    /// List of treasuries
    treasuries: Vec<TreasuryListItem>,
}

/// Treasury query response from API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreasuryQueryResponse {
    /// Treasury details
    treasury: TreasuryInfo,
}

impl TreasuryApiClient {
    /// Create a new Treasury API client
    ///
    /// # Arguments
    /// * `base_url` - Base URL of the Treasury API service (e.g., "https://oauth2.testnet.burnt.com")
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_cli::api::TreasuryApiClient;
    ///
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
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

    /// List all treasuries for authenticated user
    ///
    /// Retrieves a list of all treasury contracts associated with the authenticated user.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    ///
    /// # Returns
    /// List of treasury items with basic information
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - Network request fails
    /// - API returns an error response
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_cli::api::TreasuryApiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let treasuries = client.list_treasuries("access_token_123").await?;
    /// println!("Found {} treasuries", treasuries.len());
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn list_treasuries(
        &self,
        access_token: &str,
    ) -> Result<Vec<TreasuryListItem>> {
        let url = format!("{}/mgr-api/treasuries", self.base_url);
        debug!("Listing treasuries from: {}", url);

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .context("Failed to send list treasuries request")?;

        let status = response.status();
        debug!("List treasuries response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "List treasuries failed with status {}: {}",
                status,
                error_text
            );
        }

        let result: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse list treasuries response")?;

        let treasuries: Vec<TreasuryListItem> = serde_json::from_value(
            result.get("treasuries").cloned().unwrap_or_default()
        ).context("Failed to parse treasuries list")?;

        debug!("Successfully retrieved {} treasuries", treasuries.len());
        Ok(treasuries)
    }

    /// Query specific treasury details
    ///
    /// Retrieves detailed information about a specific treasury contract.
    ///
    /// # Arguments
    /// * `access_token` - Valid OAuth2 access token
    /// * `address` - Treasury contract address
    /// * `options` - Query options to control what information to include
    ///
    /// # Returns
    /// Complete treasury information including balance, parameters, and configurations
    ///
    /// # Errors
    /// Returns an error if:
    /// - The access token is invalid or expired
    /// - The treasury address is invalid or not found
    /// - Network request fails
    /// - API returns an error response
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_cli::api::{TreasuryApiClient, QueryOptions};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = TreasuryApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let options = QueryOptions::default();
    /// let treasury = client.query_treasury(
    ///     "access_token_123",
    ///     "xion1abc123...",
    ///     options
    /// ).await?;
    /// println!("Treasury balance: {} uxion", treasury.balance);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self, access_token))]
    pub async fn query_treasury(
        &self,
        access_token: &str,
        address: &str,
        options: QueryOptions,
    ) -> Result<TreasuryInfo> {
        let mut url = format!("{}/mgr-api/treasury/{}", self.base_url, address);

        // Add query parameters
        let mut params = Vec::new();
        if options.grants {
            params.push("grants=true");
        }
        if options.fee {
            params.push("fee=true");
        }
        if options.admin {
            params.push("admin=true");
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        debug!("Querying treasury from: {}", url);

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .context("Failed to send query treasury request")?;

        let status = response.status();
        debug!("Query treasury response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!(
                "Query treasury failed with status {}: {}",
                status,
                error_text
            );
        }

        let result: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse query treasury response")?;

        let treasury: TreasuryInfo = serde_json::from_value(
            result.get("treasury").cloned().unwrap_or_default()
        ).context("Failed to parse treasury info")?;

        debug!("Successfully queried treasury: {}", treasury.address);
        Ok(treasury)
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = TreasuryApiClient::new("https://test.com".to_string());
        assert_eq!(client.base_url, "https://test.com");
    }

    #[test]
    fn test_query_options_default() {
        let options = QueryOptions::default();
        assert!(options.grants);
        assert!(options.fee);
        assert!(options.admin);
    }

    #[test]
    fn test_query_options_custom() {
        let options = QueryOptions {
            grants: false,
            fee: true,
            admin: false,
        };
        assert!(!options.grants);
        assert!(options.fee);
        assert!(!options.admin);
    }

    #[test]
    fn test_treasury_list_response_deserialization() {
        let json = r#"{
            "treasuries": [
                {
                    "address": "xion1abc123",
                    "admin": "xion1def456",
                    "balance": "10000000"
                },
                {
                    "address": "xion1xyz789",
                    "balance": "5000000"
                }
            ]
        }"#;

        let response: TreasuryListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.treasuries.len(), 2);
        assert_eq!(response.treasuries[0].address, "xion1abc123");
        assert_eq!(response.treasuries[0].balance, "10000000");
        assert_eq!(response.treasuries[1].address, "xion1xyz789");
        assert_eq!(response.treasuries[1].balance, "5000000");
    }

    #[test]
    fn test_treasury_query_response_deserialization() {
        let json = r#"{
            "treasury": {
                "address": "xion1abc123",
                "admin": "xion1def456",
                "balance": "10000000",
                "params": {
                    "redirect_url": "https://example.com/callback",
                    "icon_url": "https://example.com/icon.png"
                }
            }
        }"#;

        let response: TreasuryQueryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.treasury.address, "xion1abc123");
        assert_eq!(response.treasury.balance, "10000000");
    }
}