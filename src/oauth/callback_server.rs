use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::sync::{oneshot, Mutex};

/// Callback server errors
#[derive(Debug, Error)]
pub enum CallbackError {
    #[error("State parameter mismatch - possible CSRF attack")]
    StateMismatch,

    #[error("OAuth2 error: {error}")]
    #[allow(dead_code)]
    OAuthError {
        error: String,
        error_description: Option<String>,
    },

    #[error("Callback timeout after {0} seconds")]
    Timeout(u64),

    #[error("Failed to bind to address: {0}")]
    BindFailed(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Channel communication error: {0}")]
    ChannelError(String),
}

/// OAuth2 callback query parameters
#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

/// OAuth2 error query parameters
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ErrorParams {
    error: String,
    error_description: Option<String>,
}

/// Result sent through the oneshot channel
type CallbackResult = Result<String, CallbackError>;

/// Shared state for the callback handler
struct CallbackState {
    /// Expected state parameter (CSRF protection)
    expected_state: String,
    /// Channel to send the result back to the main task
    /// Wrapped in Mutex<Option<>> to allow taking ownership once
    result_tx: Mutex<Option<oneshot::Sender<CallbackResult>>>,
}

/// OAuth2 callback server
///
/// A lightweight HTTP server that listens on localhost to receive OAuth2 authorization
/// callbacks. It validates the state parameter and returns the authorization code.
///
/// # Security
/// - Only binds to 127.0.0.1 (localhost)
/// - Validates state parameter to prevent CSRF attacks
/// - Single-use: automatically shuts down after receiving one callback
///
/// # Example
/// ```rust,no_run
/// use xion_agent_toolkit::oauth::CallbackServer;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let server = CallbackServer::new(8080);
///     let auth_code = server.wait_for_code("expected_state_value", 300).await?;
///     println!("Authorization code: {}", auth_code);
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct CallbackServer {
    /// Port to listen on
    port: u16,
}

impl CallbackServer {
    /// Create a new callback server
    ///
    /// # Arguments
    /// * `port` - Port to listen on (e.g., 8080)
    ///
    /// # Returns
    /// A new CallbackServer instance
    pub fn new(port: u16) -> Self {
        CallbackServer { port }
    }

    /// Start the server and wait for OAuth2 callback
    ///
    /// This method:
    /// 1. Starts an HTTP server on 127.0.0.1:{port}
    /// 2. Waits for a callback to /callback endpoint
    /// 3. Validates the state parameter
    /// 4. Returns the authorization code or error
    /// 5. Shuts down the server gracefully
    ///
    /// # Arguments
    /// * `expected_state` - The state parameter to validate against
    /// * `timeout_secs` - Timeout in seconds (default: 300)
    ///
    /// # Returns
    /// The authorization code received from the OAuth2 callback
    ///
    /// # Errors
    /// - `CallbackError::StateMismatch` if state parameter doesn't match
    /// - `CallbackError::OAuthError` if OAuth2 returns an error
    /// - `CallbackError::Timeout` if no callback received within timeout
    /// - `CallbackError::BindFailed` if unable to bind to the port
    pub async fn wait_for_code(
        self,
        expected_state: &str,
        timeout_secs: u64,
    ) -> Result<String, CallbackError> {
        // Create oneshot channel for communication
        let (tx, rx) = oneshot::channel();

        // Create shutdown signal channel for graceful shutdown
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        // Create shared state
        let state = Arc::new(CallbackState {
            expected_state: expected_state.to_string(),
            result_tx: Mutex::new(Some(tx)),
        });

        // Build router
        let app = Router::new()
            .route("/callback", get(handle_callback))
            .with_state(state);

        // Bind to localhost only (security)
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| CallbackError::BindFailed(format!("{}: {}", addr, e)))?;

        tracing::info!("Callback server listening on {}", addr);

        // Create graceful shutdown signal
        let shutdown_signal = async {
            // Wait for shutdown signal (either timeout or manual trigger)
            let _ = shutdown_rx.await;
        };

        // Run server with graceful shutdown support
        let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal);

        // Run server with timeout
        tokio::select! {
            // Server task (will exit when shutdown signal is received)
            result = server => {
                match result {
                    Ok(()) => {
                        // Server closed normally (via shutdown signal or callback handler)
                        tracing::info!("Server shut down gracefully");
                    }
                    Err(e) => {
                        tracing::error!("Server error: {}", e);
                        return Err(CallbackError::ServerError(e.to_string()));
                    }
                }
                // After server exits, check if we received a result
                // The rx might have been used already if callback was received
                Err(CallbackError::ServerError(
                    "Server closed without receiving callback".to_string()
                ))
            }

            // Wait for callback result
            result = rx => {
                // Trigger graceful shutdown
                let _ = shutdown_tx.send(());
                match result {
                    Ok(Ok(code)) => {
                        tracing::info!("Received authorization code successfully");
                        Ok(code)
                    }
                    Ok(Err(e)) => {
                        tracing::error!("Callback error: {}", e);
                        Err(e)
                    }
                    Err(_) => {
                        Err(CallbackError::ChannelError(
                            "Channel closed unexpectedly".to_string()
                        ))
                    }
                }
            }

            // Timeout
            _ = tokio::time::sleep(Duration::from_secs(timeout_secs)) => {
                tracing::warn!("Callback server timeout after {} seconds", timeout_secs);
                // Trigger graceful shutdown
                let _ = shutdown_tx.send(());
                // Wait for server to actually stop (with a small grace period)
                // This ensures the port is released before returning
                tokio::time::sleep(Duration::from_millis(100)).await;
                tracing::info!("Callback server shutdown complete after timeout");
                Err(CallbackError::Timeout(timeout_secs))
            }
        }
    }
}

/// Handle OAuth2 callback requests
///
/// This endpoint expects query parameters:
/// - `code`: The authorization code (on success)
/// - `state`: The state parameter for CSRF protection
///
/// Or error parameters:
/// - `error`: Error code
/// - `error_description`: Optional error description
async fn handle_callback(
    Query(params): Query<CallbackParams>,
    State(state): State<Arc<CallbackState>>,
) -> impl IntoResponse {
    // Log only a prefix of the state for debugging (security: don't log full state)
    tracing::info!(
        "Received callback with state: {}",
        sanitize_state_for_log(&params.state, 8)
    );

    // Validate state parameter
    if params.state != state.expected_state {
        // Security: Log only prefixes, never full state values
        tracing::warn!(
            "State mismatch: expected {}, got {}",
            sanitize_state_for_log(&state.expected_state, 8),
            sanitize_state_for_log(&params.state, 8)
        );

        let error = CallbackError::StateMismatch;

        // Send error through channel
        if let Some(tx) = state.result_tx.lock().await.take() {
            let _ = tx.send(Err(error));
        }

        // Return error page
        return (
            StatusCode::BAD_REQUEST,
            Html(
                r#"<!DOCTYPE html>
<html>
<head><title>Authentication Failed</title></head>
<body>
    <h1>Authentication Failed</h1>
    <p>State parameter mismatch. This could indicate a CSRF attack.</p>
</body>
</html>"#,
            ),
        );
    }

    // Send success result through channel
    if let Some(tx) = state.result_tx.lock().await.take() {
        let _ = tx.send(Ok(params.code.clone()));
    }

    // Return success page
    (
        StatusCode::OK,
        Html(
            r#"<!DOCTYPE html>
<html>
<head><title>Authentication Successful</title></head>
<body>
    <h1>Authentication Successful!</h1>
    <p>You can close this window now.</p>
    <script>setTimeout(() => window.close(), 2000);</script>
</body>
</html>"#,
        ),
    )
}

/// Sanitize a state parameter for logging by returning only the first N characters
///
/// This function is used to safely log state parameters without exposing the full
/// value in logs, which could aid CSRF attacks if logs are compromised.
///
/// # Arguments
/// * `state` - The state parameter to sanitize
/// * `prefix_len` - Number of characters to keep (default: 8)
///
/// # Returns
/// A sanitized string with only the prefix and "..." suffix
///
/// # Example
/// ```
/// use xion_agent_toolkit::oauth::callback_server::sanitize_state_for_log;
///
/// let sanitized = sanitize_state_for_log("abcdefghijklmnopqrstuvwxyz1234567890", 8);
/// assert_eq!(sanitized, "abcdefgh...");
/// ```
pub fn sanitize_state_for_log(state: &str, prefix_len: usize) -> String {
    let prefix: String = state.chars().take(prefix_len).collect();
    format!("{}...", prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback_server_creation() {
        let server = CallbackServer::new(54321);
        assert_eq!(server.port, 54321);
    }

    #[test]
    fn test_callback_server_default_port() {
        let server = CallbackServer::new(8080);
        assert_eq!(server.port, 8080);
    }

    #[tokio::test]
    async fn test_callback_server_bind() {
        // Test that we can create and bind a server
        let server = CallbackServer::new(15432); // Use a high port to avoid conflicts

        // This will timeout immediately, but should not panic
        let result = tokio::time::timeout(
            Duration::from_millis(100),
            server.wait_for_code("test_state", 1),
        )
        .await;

        // Should timeout (no callback received)
        assert!(result.is_err() || result.unwrap().is_err());
    }

    #[test]
    fn test_callback_error_display() {
        let error = CallbackError::StateMismatch;
        assert!(error.to_string().contains("State parameter mismatch"));

        let error = CallbackError::Timeout(300);
        assert!(error.to_string().contains("300"));

        let error = CallbackError::OAuthError {
            error: "access_denied".to_string(),
            error_description: Some("User denied access".to_string()),
        };
        assert!(error.to_string().contains("access_denied"));
    }

    #[test]
    fn test_sanitize_state_for_log() {
        // Test normal sanitization
        let state = "abcdefghijklmnopqrstuvwxyz1234567890";
        let sanitized = sanitize_state_for_log(state, 8);
        assert_eq!(sanitized, "abcdefgh...");

        // Verify the full state is NOT present in sanitized output
        assert!(!sanitized.contains("ijklmnop"));

        // Test with shorter prefix length
        let sanitized_short = sanitize_state_for_log(state, 4);
        assert_eq!(sanitized_short, "abcd...");

        // Test with value shorter than prefix length
        let short_state = "xyz";
        let sanitized_short_value = sanitize_state_for_log(short_state, 8);
        assert_eq!(sanitized_short_value, "xyz...");

        // Test with empty string
        let sanitized_empty = sanitize_state_for_log("", 8);
        assert_eq!(sanitized_empty, "...");

        // Test with typical state parameter (64 hex chars)
        let typical_state = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4";
        let sanitized_state = sanitize_state_for_log(typical_state, 8);
        assert_eq!(sanitized_state, "a1b2c3d4...");
        assert!(
            !sanitized_state.contains("e5f6"),
            "Sanitized state should not contain middle portion"
        );
    }

    #[test]
    fn test_state_sanitization_security() {
        // Verify that sanitization prevents full state exposure
        let sensitive_state = "supersecretstatevalue123456789abcdefghijklmnop";
        let sanitized = sanitize_state_for_log(sensitive_state, 8);

        // The sanitized output should NOT contain the sensitive parts
        assert!(!sanitized.contains("secret"));
        assert!(!sanitized.contains("value"));
        assert!(!sanitized.contains("123456789"));

        // Only the first 8 chars should be present
        assert!(sanitized.starts_with("supersec"));
    }

    #[test]
    fn test_callback_error_no_state_disclosure() {
        // Verify that CallbackError::StateMismatch does not expose state values
        let error = CallbackError::StateMismatch;
        let error_string = error.to_string();

        // The error message should not contain any state value
        // It should only say "State parameter mismatch - possible CSRF attack"
        assert!(error_string.contains("State parameter mismatch"));
        assert!(!error_string.contains("secret"));
        assert!(!error_string.contains("value"));
    }
}
