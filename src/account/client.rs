//! Account Client
//!
//! REST client for querying MetaAccount data from DaoDao Indexer.

use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use tracing::{debug, info};

use super::types::{SmartAccount, SmartAccountQueryResponse};
use crate::config::constants::NetworkConfig;

/// Account client for querying MetaAccount data
pub struct AccountClient {
    client: Client,
    indexer_url: String,
}

impl AccountClient {
    /// Create a new account client
    pub fn new(config: &NetworkConfig) -> Self {
        Self {
            client: Client::new(),
            indexer_url: config.indexer_url.clone(),
        }
    }

    /// Query smart account by address
    pub async fn get_smart_account(&self, address: &str) -> Result<SmartAccount> {
        debug!("Querying MetaAccount via REST API for: {}", address);

        // Build the GraphQL query URL        let query = r#"{
            smartAccount(id: "xion1test") {
                id
                latestAuthenticatorId
                authenticators {
                    nodes {
                        id
                        type
                        authenticator
                        authenticatorIndex
                        version
                    }
                }
            }
        }
        }let query = r#"#;

        // Build GraphQL request
        let request_body = serde_json::json!({
            "query": query,
            "variables": {
                "id": address
            }
        });

        let url = format!("{}/graphql", self.indexer_url);
        info!("Requesting from: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to indexer")?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Indexer returned error status {}: {}",
                status,
                body
            ));
        }

        // Parse the response
        let graphql_response: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse indexer response as JSON")?;

        // Check for GraphQL errors
        if let Some(errors) = graphql_response.get("errors") {
            return Err(anyhow!("GraphQL errors: {:?}", errors));
        }

        // Parse the SmartAccountQueryResponse
        let query_response: SmartAccountQueryResponse = serde_json::from_value(
            graphql_response
                .get("data")
                .cloned()
                .ok_or_else(|| anyhow!("No data field in GraphQL response"))?
        .context("Failed to parse SmartAccountQueryResponse")?;

        query_response
            .smart_account
            .ok_or_else(|| anyhow!("SmartAccount not found for address: {}", address))
    }
}
