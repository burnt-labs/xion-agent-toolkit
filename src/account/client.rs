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

        let url = format!(
            "{}/contract/{}/xion/account/smart/{}",
            self.indexer_url,
            address
        );

        info!("Requesting from: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request to indexer")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Indexer returned error status {}: {}",
                status,
                body
            ));
        }

        let graphql_response: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse indexer response as JSON")?;

        // Check for GraphQL errors
        if let Some(errors) = graphql_response.get("errors") {
            return Err(anyhow!("GraphQL errors: {:?}", errors));
        }

        // Parse the SmartAccount from response
        let account: SmartAccount = match serde_json::from_value(
            graphql_response
                .get("data")
                .cloned()
                .ok_or_else(|| anyhow!("No data field in GraphQL response"))?
        {
            .context("Failed to parse SmartAccount from response")
        })?;

        query_response
            .smart_account
            .ok_or_else(|| anyhow!("SmartAccount not found for address: {}", address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_url_construction() {
        // Mock a network config for testing
        let config = NetworkConfig {
            network_name: "testnet".to_string(),
            oauth_api_url: "https://test.example.com".to_string(),
            rpc_url: "https://test.example.com".to_string(),
            chain_id: "test-chain".to_string(),
            oauth_client_id: "test-client".to_string(),
            treasury_code_id: 1260,
            callback_port: 54321,
            indexer_url: "https://test-indexer.example.com".to_string(),
        };

        
        let client = AccountClient::new(&config);
        // Just verify the client was be created
        assert!(client.indexer_url.contains("/contract"));
    }
}
