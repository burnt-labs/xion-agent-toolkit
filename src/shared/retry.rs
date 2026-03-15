//! Retry Logic for Transient Failures
//!
//! This module provides automatic retry with exponential backoff for
//! transient network errors. It implements the retry pattern recommended
//! for distributed systems.
//!
//! # Features
//!
//! - Configurable retry attempts (default: 3)
//! - Exponential backoff with jitter
//! - Automatic detection of retryable errors
//! - Async/await support
//!
//! # Example
//!
//! ```rust,ignore
//! use xion_agent_toolkit::shared::retry::{with_retry, RetryConfig};
//!
//! async fn fetch_data() -> Result<Data, NetworkError> {
//!     // ... network operation
//! }
//!
//! let config = RetryConfig::default();
//! let result = with_retry(&config, || fetch_data(), is_retryable_error).await;
//! ```

use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

use super::error::{NetworkError, XionError, XionErrorCode};

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (default: 3)
    pub max_retries: u32,
    /// Initial delay in milliseconds before first retry (default: 100)
    pub initial_delay_ms: u64,
    /// Maximum delay in milliseconds between retries (default: 5000)
    pub max_delay_ms: u64,
    /// Multiplier for exponential backoff (default: 2.0)
    pub multiplier: f64,
    /// Add jitter to prevent thundering herd (default: true)
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration with custom values
    pub fn new(
        max_retries: u32,
        initial_delay_ms: u64,
        max_delay_ms: u64,
        multiplier: f64,
    ) -> Self {
        Self {
            max_retries,
            initial_delay_ms,
            max_delay_ms,
            multiplier,
            jitter: true,
        }
    }

    /// Create a fast retry configuration (for quick operations)
    pub fn fast() -> Self {
        Self {
            max_retries: 2,
            initial_delay_ms: 50,
            max_delay_ms: 1000,
            multiplier: 2.0,
            jitter: true,
        }
    }

    /// Create a patient retry configuration (for slow/degraded services)
    pub fn patient() -> Self {
        Self {
            max_retries: 5,
            initial_delay_ms: 500,
            max_delay_ms: 30000,
            multiplier: 1.5,
            jitter: true,
        }
    }

    /// Disable jitter
    pub fn no_jitter(mut self) -> Self {
        self.jitter = false;
        self
    }

    /// Calculate the delay for a given attempt number (0-indexed)
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let base_delay = self.initial_delay_ms as f64 * self.multiplier.powi(attempt as i32);
        let delay = base_delay.min(self.max_delay_ms as f64) as u64;

        if self.jitter {
            // Add up to 25% jitter
            let jitter_range = delay as f64 * 0.25;
            let jitter = (rand::random::<f64>() * jitter_range) as u64;
            Duration::from_millis(delay + jitter)
        } else {
            Duration::from_millis(delay)
        }
    }
}

/// Result of a retry operation with metadata
#[derive(Debug, Clone)]
pub struct RetryResult<T> {
    /// The final result
    pub result: T,
    /// Number of attempts made (including successful one)
    pub attempts: u32,
    /// Total time spent in retries (including final successful attempt)
    pub total_delay: Duration,
}

/// Execute an operation with automatic retry on failure
///
/// # Arguments
///
/// * `config` - Retry configuration
/// * `operation` - The async operation to execute
/// * `is_retryable` - Function to determine if an error is retryable
///
/// # Returns
///
/// The result of the operation, or the last error if all retries failed
///
/// # Example
///
/// ```rust,ignore
/// use xion_agent_toolkit::shared::retry::{with_retry, RetryConfig};
/// use xion_agent_toolkit::shared::error::{NetworkError, XionError};
///
/// async fn fetch_data() -> Result<String, XionError> {
///     // ... network operation
///     Ok("data".to_string())
/// }
///
/// # async fn example() -> Result<String, XionError> {
/// let config = RetryConfig::default();
/// let result = with_retry(
///     &config,
///     || fetch_data(),
///     |err: &XionError| err.is_retryable()
/// ).await?;
/// # Ok(result)
/// # }
/// ```
pub async fn with_retry<T, E, F, Fut>(
    config: &RetryConfig,
    mut operation: F,
    is_retryable: impl Fn(&E) -> bool,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0u32;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => {
                if attempts > 1 {
                    debug!("Operation succeeded after {} attempts", attempts);
                }
                return Ok(result);
            }
            Err(err) => {
                // Check if we should retry
                if !is_retryable(&err) {
                    debug!("Non-retryable error encountered: {:?}", err);
                    return Err(err);
                }

                // Check if we've exhausted retries
                if attempts > config.max_retries {
                    warn!(
                        "Operation failed after {} attempts (max: {})",
                        attempts, config.max_retries
                    );
                    return Err(err);
                }

                // Calculate and apply delay
                let delay = config.delay_for_attempt(attempts - 1);
                debug!(
                    "Retry attempt {}/{} after {:?}",
                    attempts, config.max_retries, delay
                );
                sleep(delay).await;
            }
        }
    }
}

/// Execute an operation with automatic retry and result metadata
///
/// Similar to `with_retry` but returns metadata about the retry attempts.
pub async fn with_retry_metadata<T, E, F, Fut>(
    config: &RetryConfig,
    mut operation: F,
    is_retryable: impl Fn(&E) -> bool,
) -> Result<RetryResult<T>, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0u32;
    let mut total_delay = Duration::ZERO;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => {
                return Ok(RetryResult {
                    result,
                    attempts,
                    total_delay,
                });
            }
            Err(err) => {
                if !is_retryable(&err) || attempts > config.max_retries {
                    return Err(err);
                }

                let delay = config.delay_for_attempt(attempts - 1);
                total_delay += delay;
                sleep(delay).await;
            }
        }
    }
}

/// Check if an HTTP status code indicates a retryable error
pub fn is_retryable_status(status: u16) -> bool {
    // 429: Too Many Requests
    // 500: Internal Server Error
    // 502: Bad Gateway
    // 503: Service Unavailable
    // 504: Gateway Timeout
    matches!(status, 429 | 500 | 502 | 503 | 504)
}

/// Check if a reqwest error is retryable
pub fn is_retryable_reqwest_error(err: &reqwest::Error) -> bool {
    // Check for specific error kinds that are retryable
    if err.is_timeout() || err.is_connect() {
        return true;
    }

    // Check status code if available
    if let Some(status) = err.status() {
        return is_retryable_status(status.as_u16());
    }

    // Check for body error (network issues)
    if let Some(source) = std::error::Error::source(err) {
        let source_str = source.to_string().to_lowercase();
        // Check for common network errors
        return source_str.contains("timeout")
            || source_str.contains("connection reset")
            || source_str.contains("connection refused")
            || source_str.contains("broken pipe");
    }

    false
}

/// Convert a reqwest error to a XionError
pub fn reqwest_to_xion_error(err: reqwest::Error) -> XionError {
    if err.is_timeout() {
        XionError::from(NetworkError::Timeout(err.to_string()))
    } else if err.is_connect() {
        XionError::from(NetworkError::ConnectionRefused(err.to_string()))
    } else if let Some(status) = err.status() {
        match status.as_u16() {
            429 => XionError::from(NetworkError::RateLimited(err.to_string())),
            503 => XionError::from(NetworkError::ServiceUnavailable(err.to_string())),
            _ => XionError::from(NetworkError::RequestFailed(format!(
                "HTTP {}: {}",
                status, err
            ))),
        }
    } else {
        XionError::from(NetworkError::RequestFailed(err.to_string()))
    }
}

/// Trait for types that can be retried
pub trait Retryable {
    /// Check if this error is retryable
    fn is_retryable(&self) -> bool;
}

impl Retryable for XionError {
    fn is_retryable(&self) -> bool {
        XionError::is_retryable(self)
    }
}

impl Retryable for XionErrorCode {
    fn is_retryable(&self) -> bool {
        XionErrorCode::is_retryable(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay_ms, 100);
        assert_eq!(config.max_delay_ms, 5000);
        assert_eq!(config.multiplier, 2.0);
        assert!(config.jitter);
    }

    #[test]
    fn test_retry_config_fast() {
        let config = RetryConfig::fast();
        assert_eq!(config.max_retries, 2);
        assert_eq!(config.initial_delay_ms, 50);
    }

    #[test]
    fn test_retry_config_patient() {
        let config = RetryConfig::patient();
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.initial_delay_ms, 500);
    }

    #[test]
    fn test_delay_for_attempt() {
        let config = RetryConfig::default().no_jitter();

        // First retry: 100ms * 2^0 = 100ms
        assert_eq!(config.delay_for_attempt(0), Duration::from_millis(100));

        // Second retry: 100ms * 2^1 = 200ms
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(200));

        // Third retry: 100ms * 2^2 = 400ms
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(400));
    }

    #[test]
    fn test_delay_capped_at_max() {
        let config = RetryConfig::new(10, 1000, 5000, 2.0).no_jitter();

        // Third retry would be 4000ms, but fourth would be 8000ms which exceeds max
        assert_eq!(config.delay_for_attempt(3), Duration::from_millis(5000));
    }

    #[test]
    fn test_is_retryable_status() {
        // Retryable status codes
        assert!(is_retryable_status(429));
        assert!(is_retryable_status(500));
        assert!(is_retryable_status(502));
        assert!(is_retryable_status(503));
        assert!(is_retryable_status(504));

        // Non-retryable status codes
        assert!(!is_retryable_status(200));
        assert!(!is_retryable_status(400));
        assert!(!is_retryable_status(401));
        assert!(!is_retryable_status(404));
    }

    #[tokio::test]
    async fn test_with_retry_success_first_try() {
        let config = RetryConfig::default();
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = with_retry(
            &config,
            || {
                let attempts = attempts_clone.clone();
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, String>("success".to_string())
                }
            },
            |_| false,
        )
        .await
        .unwrap();

        assert_eq!(result, "success");
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_with_retry_success_after_failures() {
        let config = RetryConfig::fast();
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = with_retry(
            &config,
            || {
                let attempts = attempts_clone.clone();
                async move {
                    let count = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                    if count < 2 {
                        Err("temporary error".to_string())
                    } else {
                        Ok("success".to_string())
                    }
                }
            },
            |_| true, // All errors are retryable
        )
        .await
        .unwrap();

        assert_eq!(result, "success");
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_with_retry_non_retryable_error() {
        let config = RetryConfig::default();
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result: Result<String, String> = with_retry(
            &config,
            || {
                let attempts = attempts_clone.clone();
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    Err("non-retryable error".to_string())
                }
            },
            |_| false, // No errors are retryable
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 1); // Only tried once
    }

    #[tokio::test]
    async fn test_with_retry_exhausted_retries() {
        let config = RetryConfig::new(2, 10, 100, 1.0).no_jitter();
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result: Result<String, String> = with_retry(
            &config,
            || {
                let attempts = attempts_clone.clone();
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    Err("always fails".to_string())
                }
            },
            |_| true, // All errors are retryable
        )
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 3); // Initial + 2 retries
    }

    #[tokio::test]
    async fn test_with_retry_metadata() {
        let config = RetryConfig::new(3, 10, 100, 1.0).no_jitter();
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = with_retry_metadata(
            &config,
            || {
                let attempts = attempts_clone.clone();
                async move {
                    let count = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                    if count < 2 {
                        Err("temporary".to_string())
                    } else {
                        Ok("success".to_string())
                    }
                }
            },
            |_| true,
        )
        .await
        .unwrap();

        assert_eq!(result.result, "success");
        assert_eq!(result.attempts, 2);
        assert!(result.total_delay > Duration::ZERO);
    }

    #[test]
    fn test_xion_error_retryable() {
        // Network errors are retryable
        let net_err = XionError::from(NetworkError::Timeout("test".to_string()));
        assert!(net_err.is_retryable());

        // Auth errors (except token expired) are not retryable
        let auth_err = XionError::from(super::super::error::AuthError::NotAuthenticated(
            "test".to_string(),
        ));
        assert!(!auth_err.is_retryable());
    }
}
