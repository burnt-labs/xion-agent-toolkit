//! OAuth2 Discovery and Endpoint Management
//!
//! This module handles dynamic discovery of OAuth2 endpoints via
//! `.well-known/oauth-authorization-server` as per RFC 8414.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// OAuth2 Authorization Server Metadata (RFC 8414)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2ServerMetadata {
    /// The authorization server's issuer identifier
    pub issuer: String,

    /// URL of the authorization endpoint
    pub authorization_endpoint: String,

    /// URL of the token endpoint
    pub token_endpoint: String,

    /// JSON array of supported scopes
    #[serde(default)]
    pub scopes_supported: Vec<String>,

    /// JSON array of supported response types
    #[serde(default)]
    pub response_types_supported: Vec<String>,

    /// JSON array of supported response modes
    #[serde(default)]
    pub response_modes_supported: Vec<String>,

    /// JSON array of supported grant types
    #[serde(default)]
    pub grant_types_supported: Vec<String>,

    /// JSON array of supported token endpoint authentication methods
    #[serde(default)]
    pub token_endpoint_auth_methods_supported: Vec<String>,

    /// URL of the revocation endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_endpoint: Option<String>,

    /// JSON array of supported PKCE code challenge methods
    #[serde(default)]
    pub code_challenge_methods_supported: Vec<String>,

    /// Timestamp when this metadata was fetched (for cache validation)
    #[serde(skip)]
    #[allow(dead_code)]
    pub fetched_at: Option<u64>,
}

/// Cached OAuth2 endpoints per network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2EndpointsCache {
    /// Network name
    pub network: String,

    /// OAuth2 server metadata
    pub metadata: OAuth2ServerMetadata,

    /// Cache timestamp (Unix epoch in seconds)
    pub cached_at: u64,

    /// Cache expiry in seconds (default: 24 hours)
    pub cache_ttl: u64,
}

impl OAuth2EndpointsCache {
    const DEFAULT_CACHE_TTL: u64 = 24 * 60 * 60; // 24 hours
    const CACHE_FILENAME: &'static str = "oauth_endpoints.json";

    /// Create a new cache entry
    pub fn new(network: &str, metadata: OAuth2ServerMetadata) -> Self {
        let cached_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            network: network.to_string(),
            metadata,
            cached_at,
            cache_ttl: Self::DEFAULT_CACHE_TTL,
        }
    }

    /// Check if the cache is still valid
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.cached_at < self.cache_ttl
    }

    /// Get the cache file path
    fn get_cache_path() -> Result<PathBuf> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .context("Failed to determine home directory")?;

        let cache_dir = PathBuf::from(home).join(".xion-toolkit");
        fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;

        Ok(cache_dir.join(Self::CACHE_FILENAME))
    }

    /// Load cached endpoints for all networks
    pub fn load_all() -> Result<HashMap<String, Self>> {
        let cache_path = Self::get_cache_path()?;

        if !cache_path.exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&cache_path)
            .context("Failed to read endpoints cache")?;

        let caches: Vec<Self> = serde_json::from_str(&content)
            .context("Failed to parse endpoints cache")?;

        Ok(caches.into_iter().map(|c| (c.network.clone(), c)).collect())
    }

    /// Save all cached endpoints
    pub fn save_all(caches: &HashMap<String, Self>) -> Result<()> {
        let cache_path = Self::get_cache_path()?;

        let caches_vec: Vec<&Self> = caches.values().collect();
        let content = serde_json::to_string_pretty(&caches_vec)
            .context("Failed to serialize endpoints cache")?;

        fs::write(&cache_path, content)
            .context("Failed to write endpoints cache")?;

        Ok(())
    }

    /// Load cached endpoints for a specific network
    pub fn load(network: &str) -> Result<Option<Self>> {
        let all = Self::load_all()?;
        Ok(all.get(network).cloned())
    }

    /// Save this cache entry
    pub fn save(&self) -> Result<()> {
        let mut all = Self::load_all()?;
        all.insert(self.network.clone(), self.clone());
        Self::save_all(&all)
    }
}

/// Fetch OAuth2 server metadata from well-known endpoint
pub async fn fetch_oauth2_metadata(base_url: &str) -> Result<OAuth2ServerMetadata> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .context("Failed to create HTTP client")?;

    let well_known_url = format!(
        "{}/.well-known/oauth-authorization-server",
        base_url.trim_end_matches('/')
    );

    tracing::info!("Fetching OAuth2 metadata from: {}", well_known_url);

    let response = client
        .get(&well_known_url)
        .send()
        .await
        .context("Failed to fetch OAuth2 metadata")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to fetch OAuth2 metadata: HTTP {}",
            response.status()
        );
    }

    let metadata: OAuth2ServerMetadata = response
        .json()
        .await
        .context("Failed to parse OAuth2 metadata")?;

    tracing::info!(
        "Successfully fetched OAuth2 metadata for issuer: {}",
        metadata.issuer
    );

    Ok(metadata)
}

/// Get OAuth2 endpoints for a network (with caching)
///
/// This function will:
/// 1. Check if valid cached endpoints exist
/// 2. If not, fetch from well-known endpoint
/// 3. Cache the result for future use
pub async fn get_oauth2_endpoints(
    network: &str,
    base_url: &str,
) -> Result<OAuth2ServerMetadata> {
    // Try to load from cache
    if let Some(cached) = OAuth2EndpointsCache::load(network)? {
        if cached.is_valid() {
            tracing::info!("Using cached OAuth2 endpoints for network: {}", network);
            return Ok(cached.metadata);
        } else {
            tracing::info!("Cached OAuth2 endpoints expired, fetching fresh data");
        }
    }

    // Fetch from well-known endpoint
    tracing::info!("Fetching OAuth2 endpoints from discovery endpoint");
    let metadata = fetch_oauth2_metadata(base_url).await?;

    // Cache the result
    let cache = OAuth2EndpointsCache::new(network, metadata.clone());
    cache.save()?;

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_deserialization() {
        let json = r#"{
            "issuer": "https://oauth2.testnet.burnt.com",
            "authorization_endpoint": "https://oauth2.testnet.burnt.com/oauth/authorize",
            "token_endpoint": "https://oauth2.testnet.burnt.com/oauth/token",
            "scopes_supported": ["xion:identity:read"],
            "response_types_supported": ["code"],
            "code_challenge_methods_supported": ["S256"]
        }"#;

        let metadata: OAuth2ServerMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.issuer, "https://oauth2.testnet.burnt.com");
        assert_eq!(
            metadata.authorization_endpoint,
            "https://oauth2.testnet.burnt.com/oauth/authorize"
        );
        assert_eq!(
            metadata.token_endpoint,
            "https://oauth2.testnet.burnt.com/oauth/token"
        );
    }

    #[test]
    fn test_cache_validity() {
        let metadata = OAuth2ServerMetadata {
            issuer: "https://example.com".to_string(),
            authorization_endpoint: "https://example.com/auth".to_string(),
            token_endpoint: "https://example.com/token".to_string(),
            scopes_supported: vec![],
            response_types_supported: vec![],
            response_modes_supported: vec![],
            grant_types_supported: vec![],
            token_endpoint_auth_methods_supported: vec![],
            revocation_endpoint: None,
            code_challenge_methods_supported: vec![],
            fetched_at: None,
        };

        let cache = OAuth2EndpointsCache::new("testnet", metadata);
        assert!(cache.is_valid());
    }
}
