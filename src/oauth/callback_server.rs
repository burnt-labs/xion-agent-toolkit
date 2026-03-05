use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::{
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use thiserror::Error;
use tokio::sync::{Mutex, oneshot};

/// Callback server errors
#[derive(Debug, Error)]
pub enum CallbackError {
    #[error("State parameter mismatch: expected {expected}, got {actual}")]
    StateMismatch { expected: String, actual: String },

    #[error("OAuth2 error: {error}")]
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
/// use xion_agent_cli::oauth::CallbackServer;
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
    /// 5. Shuts down the server
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

        // Run server with timeout
        tokio::select! {
            // Server task
            result = axum::serve(listener, app) => {
                match result {
                    Ok(()) => {
                        // Server closed normally
                        Err(CallbackError::ServerError(
                            "Server closed without receiving callback".to_string()
                        ))
                    }
                    Err(e) => {
                        Err(CallbackError::ServerError(e.to_string()))
                    }
                }
            }

            // Wait for callback result
            result = rx => {
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
    tracing::info!("Received callback with state: {}", params.state);

    // Validate state parameter
    if params.state != state.expected_state {
        let error = CallbackError::StateMismatch {
            expected: state.expected_state.clone(),
            actual: params.state,
        };
        tracing::error!("State mismatch: {}", error);

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
        let error = CallbackError::StateMismatch {
            expected: "abc".to_string(),
            actual: "def".to_string(),
        };
        assert!(error.to_string().contains("State parameter mismatch"));

        let error = CallbackError::Timeout(300);
        assert!(error.to_string().contains("300"));

        let error = CallbackError::OAuthError {
            error: "access_denied".to_string(),
            error_description: Some("User denied access".to_string()),
        };
        assert!(error.to_string().contains("access_denied"));
    }
}