//! OAuth2 Token Manager
//!
//! Manages OAuth2 token lifecycle including expiration checks, automatic refresh,
//! and credential persistence.

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, instrument};

use crate::api::oauth2_api::OAuth2ApiClient;
use crate::config::{CredentialsManager, UserCredentials, DEFAULT_REFRESH_TOKEN_LIFETIME_SECS};

/// Token expiry buffer in seconds (5 minutes)
const EXPIRY_BUFFER_SECS: i64 = 300;

/// Refresh token expiry buffer in seconds (1 minute)
/// Shorter than access token buffer since refresh tokens are long-lived
const REFRESH_TOKEN_EXPIRY_BUFFER_SECS: i64 = 60;

/// Manages OAuth2 token lifecycle
///
/// Responsible for:
/// - Checking token expiration status
/// - Automatically refreshing expired tokens
/// - Updating stored credentials after refresh
/// - Providing valid access tokens to the application
#[derive(Debug)]
pub struct TokenManager {
    /// Credentials manager for token persistence
    credentials_manager: CredentialsManager,
    /// OAuth2 API client for token refresh operations
    oauth2_api: OAuth2ApiClient,
    /// OAuth2 client ID for refresh requests
    client_id: String,
    /// Lock to prevent concurrent token refresh operations
    refresh_lock: Arc<Mutex<()>>,
}

impl TokenManager {
    /// Create a new TokenManager
    ///
    /// # Arguments
    /// * `credentials_manager` - Manager for credential storage
    /// * `oauth2_api` - OAuth2 API client for token operations
    /// * `client_id` - OAuth2 client identifier
    ///
    /// # Example
    /// ```no_run
    /// use xion_agent_toolkit::config::CredentialsManager;
    /// use xion_agent_toolkit::api::OAuth2ApiClient;
    /// use xion_agent_toolkit::oauth::TokenManager;
    ///
    /// let creds_mgr = CredentialsManager::new("testnet")?;
    /// let api_client = OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string());
    /// let token_mgr = TokenManager::new(creds_mgr, api_client, "client_id".to_string());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn new(
        credentials_manager: CredentialsManager,
        oauth2_api: OAuth2ApiClient,
        client_id: String,
    ) -> Self {
        Self {
            credentials_manager,
            oauth2_api,
            client_id,
            refresh_lock: Arc::new(Mutex::new(())),
        }
    }

    /// Get a valid access token (refresh if needed)
    ///
    /// This is the primary method to obtain an access token for API calls.
    /// It automatically handles token refresh if the current token is expired
    /// or will expire soon.
    ///
    /// # Returns
    /// Valid access token ready for use in API calls
    ///
    /// # Errors
    /// Returns an error if:
    /// - Credentials are not found
    /// - Token refresh fails
    /// - Token storage fails
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::config::CredentialsManager;
    /// # use xion_agent_toolkit::api::OAuth2ApiClient;
    /// # use xion_agent_toolkit::oauth::TokenManager;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let token_mgr = TokenManager::new(
    ///     CredentialsManager::new("testnet")?,
    ///     OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string()),
    ///     "client_id".to_string()
    /// );
    /// let token = token_mgr.get_valid_token().await?;
    /// println!("Access token: {}", token);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_valid_token(&self) -> Result<String> {
        debug!("Getting valid access token");

        // Load current credentials
        let credentials = self
            .credentials_manager
            .load_credentials()
            .context("Failed to load credentials")?;

        // Debug: show minimal token prefix (security: log only first 8 chars)
        debug!(
            "Loaded token: {}",
            sanitize_for_log(&credentials.access_token, 8)
        );
        debug!(
            "Loaded token length: {} chars",
            credentials.access_token.len()
        );
        debug!("Token expires at: {}", credentials.expires_at);

        // Check if token will expire soon (within buffer)
        if self.will_expire_soon(EXPIRY_BUFFER_SECS)? {
            info!(
                "Token will expire within {} seconds, refreshing",
                EXPIRY_BUFFER_SECS
            );

            // Acquire lock to prevent concurrent refresh
            let _guard = self.refresh_lock.lock().await;

            // Double-check: another task may have refreshed while we waited
            if !self.will_expire_soon(EXPIRY_BUFFER_SECS)? {
                debug!("Token was refreshed by another task, using existing token");
                let fresh_credentials = self
                    .credentials_manager
                    .load_credentials()
                    .context("Failed to load fresh credentials")?;
                return Ok(fresh_credentials.access_token);
            }

            // Refresh token
            let new_credentials = self
                .refresh_access_token()
                .await
                .context("Failed to refresh token")?;

            debug!(
                "Refreshed token: {}",
                sanitize_for_log(&new_credentials.access_token, 8)
            );

            return Ok(new_credentials.access_token);
        }

        debug!("Current token is still valid");
        Ok(credentials.access_token)
    }

    /// Check if the current token is expired
    ///
    /// # Returns
    /// `true` if the token has expired, `false` otherwise
    ///
    /// # Errors
    /// Returns an error if credentials cannot be loaded or parsed
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::TokenManager;
    /// # use xion_agent_toolkit::config::CredentialsManager;
    /// # use xion_agent_toolkit::api::OAuth2ApiClient;
    /// # let token_mgr = TokenManager::new(
    /// #     CredentialsManager::new("testnet")?,
    /// #     OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string()),
    /// #     "client_id".to_string()
    /// # );
    /// if token_mgr.is_token_expired()? {
    ///     println!("Token has expired!");
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    #[allow(dead_code)]
    #[instrument(skip(self))]
    pub fn is_token_expired(&self) -> Result<bool> {
        let credentials = self
            .credentials_manager
            .load_credentials()
            .context("Failed to load credentials")?;

        let expires_at =
            parse_expiry_time(&credentials.expires_at).context("Failed to parse expiry time")?;

        let now = Utc::now();

        Ok(expires_at <= now)
    }

    /// Check if the token will expire soon
    ///
    /// This is used to proactively refresh tokens before they expire,
    /// avoiding failed API calls due to expired tokens.
    ///
    /// # Arguments
    /// * `buffer_secs` - Number of seconds before expiry to consider as "expiring soon"
    ///
    /// # Returns
    /// `true` if the token will expire within the buffer period, `false` otherwise
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::TokenManager;
    /// # use xion_agent_toolkit::config::CredentialsManager;
    /// # use xion_agent_toolkit::api::OAuth2ApiClient;
    /// # let token_mgr = TokenManager::new(
    /// #     CredentialsManager::new("testnet")?,
    /// #     OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string()),
    /// #     "client_id".to_string()
    /// # );
    /// // Check if token will expire in the next 5 minutes
    /// if token_mgr.will_expire_soon(300)? {
    ///     println!("Token will expire soon, consider refreshing");
    /// }
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    #[instrument(skip(self))]
    pub fn will_expire_soon(&self, buffer_secs: i64) -> Result<bool> {
        let credentials = self
            .credentials_manager
            .load_credentials()
            .context("Failed to load credentials")?;

        let expires_at =
            parse_expiry_time(&credentials.expires_at).context("Failed to parse expiry time")?;

        let now = Utc::now();
        let time_until_expiry = expires_at.signed_duration_since(now);

        Ok(time_until_expiry.num_seconds() < buffer_secs)
    }

    /// Refresh the access token using the refresh token
    ///
    /// Calls the OAuth2 API to exchange the refresh token for a new access token.
    /// Updates the stored credentials with the new tokens.
    ///
    /// # Returns
    /// New credentials with fresh access token
    ///
    /// # Errors
    /// Returns an error if:
    /// - Current credentials cannot be loaded
    /// - Refresh token is invalid or revoked
    /// - New credentials cannot be saved
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::TokenManager;
    /// # use xion_agent_toolkit::config::CredentialsManager;
    /// # use xion_agent_toolkit::api::OAuth2ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let token_mgr = TokenManager::new(
    /// #     CredentialsManager::new("testnet")?,
    /// #     OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string()),
    /// #     "client_id".to_string()
    /// # );
    /// let new_creds = token_mgr.refresh_access_token().await?;
    /// println!("New token expires at: {}", new_creds.expires_at);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn refresh_access_token(&self) -> Result<UserCredentials> {
        info!("Refreshing access token");

        // Load current credentials to get refresh token
        let credentials = self
            .credentials_manager
            .load_credentials()
            .context("Failed to load current credentials")?;

        // Check if refresh token is expired (with buffer for safety)
        if let Some(ref refresh_token_expires_at) = credentials.refresh_token_expires_at {
            let expires_at = parse_expiry_time(refresh_token_expires_at)
                .context("Failed to parse refresh token expiry time")?;
            let expiry_with_buffer =
                Utc::now() + Duration::seconds(REFRESH_TOKEN_EXPIRY_BUFFER_SECS);
            if expires_at <= expiry_with_buffer {
                anyhow::bail!(
                    "Refresh token has expired at {}. Please login again.",
                    refresh_token_expires_at
                );
            }
        }

        // Call OAuth2 API to refresh token
        let token_response = self
            .oauth2_api
            .refresh_token(&credentials.refresh_token, &self.client_id)
            .await
            .context("Token refresh request failed")?;

        // Build new credentials
        let expires_at = token_response
            .expires_at
            .clone()
            .unwrap_or_else(|| calculate_expiry_time(token_response.expires_in));

        // Calculate refresh token expiration
        // Use new value from response if provided, otherwise keep the old one or calculate new
        let refresh_token_expires_at = token_response
            .refresh_token_expires_at
            .clone()
            .or_else(|| {
                token_response
                    .refresh_token_expires_in
                    .map(calculate_expiry_time)
            })
            .or(credentials.refresh_token_expires_at)
            .or_else(|| Some(calculate_expiry_time(DEFAULT_REFRESH_TOKEN_LIFETIME_SECS)));

        let new_credentials = UserCredentials {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            refresh_token_expires_at,
            xion_address: token_response.xion_address.or(credentials.xion_address),
        };

        // Save updated credentials
        self.credentials_manager
            .save_credentials(&new_credentials)
            .context("Failed to save refreshed credentials")?;

        info!("Access token refreshed successfully");
        Ok(new_credentials)
    }

    /// Validate token with OAuth2 API
    ///
    /// Makes a request to the OAuth2 userinfo endpoint to verify that
    /// the access token is still valid.
    ///
    /// # Arguments
    /// * `access_token` - Access token to validate
    ///
    /// # Returns
    /// `true` if the token is valid, `false` otherwise
    ///
    /// # Example
    /// ```no_run
    /// # use xion_agent_toolkit::oauth::TokenManager;
    /// # use xion_agent_toolkit::config::CredentialsManager;
    /// # use xion_agent_toolkit::api::OAuth2ApiClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let token_mgr = TokenManager::new(
    /// #     CredentialsManager::new("testnet")?,
    /// #     OAuth2ApiClient::new("https://oauth2.testnet.burnt.com".to_string()),
    /// #     "client_id".to_string()
    /// # );
    /// let is_valid = token_mgr.validate_token("some_token").await?;
    /// if is_valid {
    ///     println!("Token is valid");
    /// } else {
    ///     println!("Token is invalid or expired");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[allow(dead_code)]
    #[instrument(skip(self, access_token))]
    pub async fn validate_token(&self, access_token: &str) -> Result<bool> {
        debug!("Validating access token");

        match self.oauth2_api.get_user_info(access_token).await {
            Ok(_) => {
                debug!("Token is valid");
                Ok(true)
            }
            Err(e) => {
                debug!("Token validation failed: {}", e);
                Ok(false)
            }
        }
    }
}

/// Calculate expiry time from expires_in seconds
///
/// Takes the relative expiration time in seconds and calculates
/// the absolute expiration timestamp.
///
/// # Arguments
/// * `expires_in` - Number of seconds until expiration
///
/// # Returns
/// ISO 8601 formatted expiration timestamp
///
/// # Example
/// ```
/// use xion_agent_toolkit::oauth::token_manager::calculate_expiry_time;
///
/// let expires_at = calculate_expiry_time(3600);
/// println!("Expires at: {}", expires_at);
/// ```
pub fn calculate_expiry_time(expires_in: i64) -> String {
    let now = Utc::now();
    let expires_at = now + Duration::seconds(expires_in);
    expires_at.to_rfc3339()
}

/// Parse expiry time string into DateTime
///
/// Converts an ISO 8601 formatted timestamp into a DateTime object.
///
/// # Arguments
/// * `expires_at` - ISO 8601 formatted expiration timestamp
///
/// # Returns
/// Parsed DateTime in UTC timezone
///
/// # Errors
/// Returns an error if the string cannot be parsed as ISO 8601
///
/// # Example
/// ```
/// use xion_agent_toolkit::oauth::token_manager::parse_expiry_time;
///
/// let dt = parse_expiry_time("2024-01-01T00:00:00Z")?;
/// println!("Parsed: {}", dt);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn parse_expiry_time(expires_at: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(expires_at)
        .map(|dt| dt.with_timezone(&Utc))
        .context(format!("Failed to parse expiry time: {}", expires_at))
}

/// Sanitize a sensitive value for logging by returning only the first N characters
///
/// This function is used to safely log tokens, state parameters, and other sensitive
/// values without exposing the full value in logs.
///
/// # Arguments
/// * `value` - The sensitive value to sanitize
/// * `prefix_len` - Number of characters to keep (default: 8)
///
/// # Returns
/// A sanitized string with only the prefix and "..." suffix
///
/// # Example
/// ```
/// use xion_agent_toolkit::oauth::token_manager::sanitize_for_log;
///
/// let sanitized = sanitize_for_log("secret_token_value_12345", 8);
/// assert_eq!(sanitized, "secret_t...");
/// ```
pub fn sanitize_for_log(value: &str, prefix_len: usize) -> String {
    let prefix: String = value.chars().take(prefix_len).collect();
    format!("{}...", prefix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_calculate_expiry_time() {
        let now = Utc::now();
        let expires_in = 3600; // 1 hour
        let expires_at = calculate_expiry_time(expires_in);

        let parsed = parse_expiry_time(&expires_at).unwrap();
        let diff = parsed.signed_duration_since(now);

        // Should be approximately 1 hour (allow 1 second tolerance)
        assert!(diff.num_seconds() >= 3599);
        assert!(diff.num_seconds() <= 3601);
    }

    #[test]
    fn test_parse_expiry_time() {
        let time_str = "2024-01-01T00:00:00Z";
        let parsed = parse_expiry_time(time_str).unwrap();
        // Verify the year and month are correct
        assert_eq!(parsed.year(), 2024);
        assert_eq!(parsed.month(), 1);
        assert_eq!(parsed.day(), 1);
    }

    #[test]
    fn test_parse_expiry_time_invalid() {
        let result = parse_expiry_time("invalid-time");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_expiry_time_with_timezone() {
        // Test with timezone offset
        let time_str = "2024-01-01T00:00:00+00:00";
        let parsed = parse_expiry_time(time_str).unwrap();
        // Should convert to UTC
        assert_eq!(parsed.to_rfc3339(), "2024-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_calculate_expiry_time_zero() {
        let now = Utc::now();
        let expires_at = calculate_expiry_time(0);

        let parsed = parse_expiry_time(&expires_at).unwrap();
        let diff = parsed.signed_duration_since(now);

        // Should be approximately now (allow 1 second tolerance)
        assert!(diff.num_seconds().abs() <= 1);
    }

    #[test]
    fn test_calculate_expiry_time_negative() {
        // Negative expires_in should work (token already expired)
        let expires_at = calculate_expiry_time(-3600);
        let parsed = parse_expiry_time(&expires_at).unwrap();
        let now = Utc::now();

        assert!(parsed < now);
    }

    #[test]
    fn test_expiry_buffer_constant() {
        // Ensure the buffer is reasonable (5 minutes)
        assert_eq!(EXPIRY_BUFFER_SECS, 300);
    }

    #[test]
    fn test_refresh_token_expiry_buffer_constant() {
        // Ensure the refresh token buffer is reasonable (1 minute)
        assert_eq!(REFRESH_TOKEN_EXPIRY_BUFFER_SECS, 60);
    }

    #[test]
    fn test_refresh_token_expiry_buffer_logic() {
        // Test that refresh token expiry buffer is correctly applied
        let now = Utc::now();

        // Test case 1: Token expires in 30 seconds (within buffer)
        // Should be considered expired with the 60-second buffer
        let expires_in_30_secs = now + Duration::seconds(30);
        let buffer_time = now + Duration::seconds(REFRESH_TOKEN_EXPIRY_BUFFER_SECS);
        assert!(
            expires_in_30_secs <= buffer_time,
            "Token expiring in 30 seconds should be within buffer"
        );

        // Test case 2: Token expires in 2 minutes (outside buffer)
        // Should NOT be considered expired with the 60-second buffer
        let expires_in_2_mins = now + Duration::seconds(120);
        assert!(
            expires_in_2_mins > buffer_time,
            "Token expiring in 2 minutes should be outside buffer"
        );

        // Test case 3: Token expires exactly at buffer boundary
        // Should be considered expired (expires_at <= buffer)
        let expires_at_boundary = now + Duration::seconds(REFRESH_TOKEN_EXPIRY_BUFFER_SECS);
        assert!(
            expires_at_boundary <= buffer_time,
            "Token at buffer boundary should be considered expiring soon"
        );

        // Test case 4: Token expires 1 second after buffer
        // Should NOT be considered expired
        let expires_after_buffer = now + Duration::seconds(REFRESH_TOKEN_EXPIRY_BUFFER_SECS + 1);
        assert!(
            expires_after_buffer > buffer_time,
            "Token 1 second after buffer should NOT be considered expiring soon"
        );
    }

    #[test]
    fn test_sanitize_for_log() {
        // Test normal sanitization
        let sanitized = sanitize_for_log("secret_token_value_12345", 8);
        assert_eq!(sanitized, "secret_t...");

        // Test with shorter prefix length
        let sanitized_short = sanitize_for_log("secret_token_value_12345", 4);
        assert_eq!(sanitized_short, "secr...");

        // Test with value shorter than prefix length
        let sanitized_short_value = sanitize_for_log("abc", 8);
        assert_eq!(sanitized_short_value, "abc...");

        // Test with empty string
        let sanitized_empty = sanitize_for_log("", 8);
        assert_eq!(sanitized_empty, "...");

        // Test with typical token length
        let typical_token = "xion1abc123def456ghi789jkl012mno345pqr678stu901vwx234yz";
        let sanitized_token = sanitize_for_log(typical_token, 8);
        assert_eq!(sanitized_token, "xion1abc...");
        assert!(
            !sanitized_token.contains("def456"),
            "Sanitized value should not contain middle portion of token"
        );
    }

    #[test]
    fn test_access_token_buffer_vs_refresh_token_buffer() {
        // Verify that access token buffer (5 min) is larger than refresh token buffer (1 min)
        // This is intentional: refresh tokens are long-lived, so we use a smaller buffer
        // Use const assertion to verify at compile time
        const { assert!(EXPIRY_BUFFER_SECS > REFRESH_TOKEN_EXPIRY_BUFFER_SECS) };

        // Access token buffer should be 5 minutes
        assert_eq!(EXPIRY_BUFFER_SECS, 300);

        // Refresh token buffer should be 1 minute
        assert_eq!(REFRESH_TOKEN_EXPIRY_BUFFER_SECS, 60);
    }
}
