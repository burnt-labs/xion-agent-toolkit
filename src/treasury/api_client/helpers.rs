//! Internal helper functions for treasury API client.
//!
//! These functions are used across multiple modules within the api_client module.

use crate::shared::error::{TreasuryError, XionResult};

/// Parse a coin string (e.g., "1000000uxion") into (amount, denom)
pub(crate) fn parse_coin(coin: &str) -> XionResult<(String, String)> {
    // Find where digits end and letters begin
    let split_pos = coin
        .chars()
        .position(|c| !c.is_ascii_digit())
        .ok_or_else(|| TreasuryError::InvalidAddress(format!("Invalid coin format: {}", coin)))?;

    let amount = coin[..split_pos].to_string();
    let denom = coin[split_pos..].to_string();

    if amount.is_empty() || denom.is_empty() {
        return Err(TreasuryError::InvalidAddress(format!("Invalid coin format: {}", coin)).into());
    }

    Ok((amount, denom))
}

/// Extract user address from OAuth2 access token
///
/// Token format: {userId}:{grantId}:{secret}
/// userId is the user's Xion address (starts with "xion1")
pub(crate) fn extract_address_from_token(token: &str) -> XionResult<String> {
    let parts: Vec<&str> = token.split(':').collect();
    if parts.len() != 3 {
        return Err(TreasuryError::InvalidAddress(
            "Invalid access token format: expected 3 parts separated by ':'".to_string(),
        )
        .into());
    }

    let address = parts[0].to_string();
    if !address.starts_with("xion1") {
        return Err(TreasuryError::InvalidAddress(
            "Invalid access token: userId must be a valid Xion address (starts with 'xion1')"
                .to_string(),
        )
        .into());
    }

    Ok(address)
}

/// Convert bytes to JSON number array for OAuth2 API
///
/// The OAuth2 API's JSON object path uses `fromPartial` which expects
/// bytes fields (like `msg` and `salt`) to be array-like objects (number arrays)
/// rather than base64 strings.
pub(crate) fn bytes_to_json_array(bytes: &[u8]) -> serde_json::Value {
    serde_json::Value::Array(
        bytes
            .iter()
            .map(|b| serde_json::Value::Number((*b).into()))
            .collect(),
    )
}

/// Base64 encode a string
pub(crate) fn base64_encode(input: &str) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    STANDARD.encode(input.as_bytes())
}

/// Base64 decode a string
pub(crate) fn base64_decode(input: &str) -> XionResult<String> {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let bytes = STANDARD.decode(input).map_err(|e| {
        TreasuryError::OperationFailed(format!("Failed to decode base64 string: {}", e))
    })?;
    String::from_utf8(bytes).map_err(|e| {
        TreasuryError::OperationFailed(format!("Decoded base64 is not valid UTF-8: {}", e)).into()
    })
}
