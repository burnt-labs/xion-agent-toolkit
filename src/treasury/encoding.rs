//! Treasury Encoding Module
//!
//! This module provides functions to encode treasury configurations (fee grants and authz grants)
//! into protobuf-encoded base64 strings that can be sent to the blockchain.
//!
//! The encoding follows Cosmos SDK protobuf specifications and matches the Developer Portal implementation.

use base64::Engine;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Supporting Types
// ============================================================================

/// Parsed coin (amount + denom)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    pub amount: String,
    pub denom: String,
}

/// IBC transfer allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbcAllocation {
    pub source_port: String,
    pub source_channel: String,
    pub spend_limit: Vec<Coin>,
    pub allow_list: Option<Vec<String>>,
}

/// Contract execution grant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGrant {
    pub address: String,
    pub max_calls: Option<u64>,
    pub max_funds: Option<Vec<Coin>>,
    pub filter_type: String, // "allow_all" or "accepted_keys"
    pub keys: Option<Vec<String>>,
}

/// Encoding error
#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("Invalid coin format: {0}")]
    InvalidCoinFormat(String),

    #[error("Protobuf encoding failed: {0}")]
    ProtobufError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Base64 encoding failed: {0}")]
    Base64Error(String),
}

// ============================================================================
// Helper Utilities
// ============================================================================

/// Parse coin string like "1000000uxion" or "1000uxion,500uatom"
///
/// # Arguments
/// * `input` - Coin string in format "amountdenom" or multiple comma-separated coins
///
/// # Returns
/// Vector of parsed coins sorted alphabetically by denom
///
/// # Examples
/// ```
/// use xion_agent_toolkit::treasury::encoding::{parse_coin_string, Coin};
///
/// let coins = parse_coin_string("1000000uxion").unwrap();
/// assert_eq!(coins.len(), 1);
/// assert_eq!(coins[0].amount, "1000000");
/// assert_eq!(coins[0].denom, "uxion");
/// ```
pub fn parse_coin_string(input: &str) -> Result<Vec<Coin>, EncodingError> {
    if input.is_empty() {
        return Err(EncodingError::InvalidCoinFormat("empty input".to_string()));
    }

    // Compile regex once outside the loop
    let re = regex::Regex::new(r"^(\d+)([-a-zA-Z0-9/]+)$")
        .map_err(|e| EncodingError::ProtobufError(e.to_string()))?;

    let mut coins = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        // Normalize: remove spaces between digits and denom
        let normalized = part.replace(" ", "");

        // Match: digits followed by denom (allowing hyphens, slashes, alphanumeric)
        if let Some(caps) = re.captures(&normalized) {
            coins.push(Coin {
                amount: caps[1].to_string(),
                denom: caps[2].to_string(),
            });
        } else {
            return Err(EncodingError::InvalidCoinFormat(part.to_string()));
        }
    }

    // Sort by denom alphabetically
    coins.sort_by(|a, b| a.denom.cmp(&b.denom));

    Ok(coins)
}

/// Parse single coin like "1000000uxion"
///
/// # Arguments
/// * `input` - Single coin string in format "amountdenom"
///
/// # Returns
/// Single parsed coin
pub fn parse_single_denom(input: &str) -> Result<Coin, EncodingError> {
    let coins = parse_coin_string(input)?;
    if coins.len() != 1 {
        return Err(EncodingError::InvalidCoinFormat(
            "expected single denomination".to_string(),
        ));
    }
    Ok(coins.into_iter().next().unwrap())
}

/// Encode bytes to base64
///
/// # Arguments
/// * `bytes` - Bytes to encode
///
/// # Returns
/// Base64-encoded string
pub fn encode_to_base64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

// ============================================================================
// Protobuf Encoding Helpers
// ============================================================================

/// Encode a varint (variable-length integer)
fn encode_varint(mut value: u64) -> Vec<u8> {
    let mut result = Vec::new();
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        result.push(byte);
        if value == 0 {
            break;
        }
    }
    result
}

/// Encode a length-delimited field (string, bytes, message)
fn encode_length_delimited(field_number: u32, data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    // Field tag: (field_number << 3) | wire_type (2 for length-delimited)
    let tag = ((field_number << 3) | 2) as u64;
    result.extend(encode_varint(tag));
    result.extend(encode_varint(data.len() as u64));
    result.extend(data);
    result
}

/// Encode a string field
fn encode_string_field(field_number: u32, value: &str) -> Vec<u8> {
    encode_length_delimited(field_number, value.as_bytes())
}

/// Encode a protobuf Coin message
///
/// Cosmos SDK Coin protobuf definition:
/// ```protobuf
/// message Coin {
///   string denom = 1;
///   string amount = 2;
/// }
/// ```
fn encode_coin(coin: &Coin) -> Vec<u8> {
    let mut result = Vec::new();
    // Field 1: denom (string)
    result.extend(encode_string_field(1, &coin.denom));
    // Field 2: amount (string)
    result.extend(encode_string_field(2, &coin.amount));
    result
}

/// Encode repeated coin fields
fn encode_repeated_coins(field_number: u32, coins: &[Coin]) -> Vec<u8> {
    let mut result = Vec::new();
    for coin in coins {
        let coin_bytes = encode_coin(coin);
        result.extend(encode_length_delimited(field_number, &coin_bytes));
    }
    result
}

/// Encode a Duration message (for periodic allowance)
fn encode_duration(seconds: u64) -> Vec<u8> {
    let mut result = Vec::new();
    // Field 1: seconds (int64, varint)
    let tag = (1 << 3) as u64; // wire type 0 for varint
    result.extend(encode_varint(tag));
    result.extend(encode_varint(seconds));
    result
}

/// Encode an Any message (for nested allowances)
fn encode_any(type_url: &str, value: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    // Field 1: type_url (string)
    result.extend(encode_string_field(1, type_url));
    // Field 2: value (bytes)
    result.extend(encode_length_delimited(2, value));
    result
}

// ============================================================================
// Fee Allowance Encoding
// ============================================================================

/// Encode BasicAllowance to protobuf base64
///
/// # Arguments
/// * `spend_limit` - Vector of coins representing the spend limit
///
/// # Returns
/// Base64-encoded protobuf BasicAllowance message
///
/// # Example
/// ```
/// use xion_agent_toolkit::treasury::encoding::{encode_basic_allowance, Coin};
///
/// let coins = vec![Coin {
///     denom: "uxion".into(),
///     amount: "1000000".into(),
/// }];
/// let encoded = encode_basic_allowance(coins).unwrap();
/// assert!(!encoded.is_empty());
/// ```
pub fn encode_basic_allowance(spend_limit: Vec<Coin>) -> Result<String, EncodingError> {
    if spend_limit.is_empty() {
        return Err(EncodingError::InvalidInput(
            "BasicAllowance requires at least one spend_limit coin".into(),
        ));
    }

    let mut bytes = Vec::new();
    // Field 1: spend_limit (repeated Coin)
    bytes.extend(encode_repeated_coins(1, &spend_limit));
    // Field 2: expiration is not set (optional)

    Ok(encode_to_base64(&bytes))
}

/// Encode PeriodicAllowance to protobuf base64
///
/// # Arguments
/// * `basic_spend_limit` - Optional basic spend limit (overall cap)
/// * `period_seconds` - Duration of each period in seconds
/// * `period_spend_limit` - Maximum spend per period
///
/// # Returns
/// Base64-encoded protobuf PeriodicAllowance message
pub fn encode_periodic_allowance(
    basic_spend_limit: Option<Vec<Coin>>,
    period_seconds: u64,
    period_spend_limit: Vec<Coin>,
) -> Result<String, EncodingError> {
    if period_spend_limit.is_empty() {
        return Err(EncodingError::InvalidInput(
            "PeriodicAllowance requires period_spend_limit".into(),
        ));
    }

    let mut bytes = Vec::new();

    // Field 1: basic (optional BasicAllowance)
    if let Some(limit) = basic_spend_limit {
        if !limit.is_empty() {
            let basic_bytes = encode_basic_allowance(limit)?;
            let basic_decoded = base64::engine::general_purpose::STANDARD
                .decode(&basic_bytes)
                .map_err(|e| EncodingError::Base64Error(e.to_string()))?;
            bytes.extend(encode_length_delimited(1, &basic_decoded));
        }
    }

    // Field 2: period (Duration)
    let duration_bytes = encode_duration(period_seconds);
    bytes.extend(encode_length_delimited(2, &duration_bytes));

    // Field 3: period_spend_limit (repeated Coin)
    bytes.extend(encode_repeated_coins(3, &period_spend_limit));

    // Fields 4-6 are not set (period_can_roll_over, remaining_spend_limit, last_period_reset)

    Ok(encode_to_base64(&bytes))
}

/// Encode AllowedMsgAllowance to protobuf base64
///
/// # Arguments
/// * `allowed_messages` - List of allowed message type URLs
/// * `nested_allowance_type_url` - Type URL of the nested allowance
/// * `nested_allowance_value` - Binary value of the nested allowance (will be converted to base64)
///
/// # Returns
/// Base64-encoded protobuf AllowedMsgAllowance message
pub fn encode_allowed_msg_allowance(
    allowed_messages: Vec<String>,
    nested_allowance_type_url: &str,
    nested_allowance_value_base64: &str,
) -> Result<String, EncodingError> {
    if allowed_messages.is_empty() {
        return Err(EncodingError::InvalidInput(
            "AllowedMsgAllowance requires at least one allowed message".into(),
        ));
    }

    // Decode nested allowance from base64
    let nested_bytes = base64::engine::general_purpose::STANDARD
        .decode(nested_allowance_value_base64)
        .map_err(|e| EncodingError::Base64Error(e.to_string()))?;

    let mut bytes = Vec::new();

    // Field 1: allowance (Any)
    let any_bytes = encode_any(nested_allowance_type_url, &nested_bytes);
    bytes.extend(encode_length_delimited(1, &any_bytes));

    // Field 2: allowed_messages (repeated string)
    for msg in &allowed_messages {
        bytes.extend(encode_string_field(2, msg));
    }

    Ok(encode_to_base64(&bytes))
}

// ============================================================================
// Grant Authorization Encoding
// ============================================================================

/// Encode GenericAuthorization to protobuf base64
///
/// # Arguments
/// * `msg_type_url` - Message type URL (e.g., "/cosmos.bank.v1beta1.MsgSend")
///
/// # Returns
/// Base64-encoded protobuf GenericAuthorization message
pub fn encode_generic_authorization(msg_type_url: &str) -> Result<String, EncodingError> {
    if msg_type_url.is_empty() {
        return Err(EncodingError::InvalidInput(
            "GenericAuthorization requires msg_type_url".into(),
        ));
    }

    let mut bytes = Vec::new();
    // Field 1: msg (string)
    bytes.extend(encode_string_field(1, msg_type_url));

    Ok(encode_to_base64(&bytes))
}

/// Encode SendAuthorization to protobuf base64
///
/// # Arguments
/// * `spend_limit` - Maximum coins that can be sent
/// * `allow_list` - Optional list of allowed recipient addresses
///
/// # Returns
/// Base64-encoded protobuf SendAuthorization message
pub fn encode_send_authorization(
    spend_limit: Vec<Coin>,
    allow_list: Option<Vec<String>>,
) -> Result<String, EncodingError> {
    if spend_limit.is_empty() {
        return Err(EncodingError::InvalidInput(
            "SendAuthorization requires spend_limit".into(),
        ));
    }

    let mut bytes = Vec::new();
    // Field 1: spend_limit (repeated Coin)
    bytes.extend(encode_repeated_coins(1, &spend_limit));

    // Field 2: allow_list (repeated string)
    if let Some(addrs) = allow_list {
        for addr in &addrs {
            bytes.extend(encode_string_field(2, addr));
        }
    }

    Ok(encode_to_base64(&bytes))
}

/// Encode StakeAuthorization to protobuf base64
///
/// # Arguments
/// * `max_tokens` - Maximum tokens for staking
/// * `allow_list` - Optional list of allowed validator addresses
/// * `deny_list` - Optional list of denied validator addresses
/// * `authorization_type` - 1=DELEGATE, 2=UNDELEGATE, 3=REDELEGATE
///
/// # Returns
/// Base64-encoded protobuf StakeAuthorization message
pub fn encode_stake_authorization(
    max_tokens: Coin,
    allow_list: Option<Vec<String>>,
    deny_list: Option<Vec<String>>,
    authorization_type: i32,
) -> Result<String, EncodingError> {
    if !(1..=3).contains(&authorization_type) {
        return Err(EncodingError::InvalidInput(
            "authorization_type must be 1 (DELEGATE), 2 (UNDELEGATE), or 3 (REDELEGATE)".into(),
        ));
    }

    let mut bytes = Vec::new();

    // Field 1: max_tokens (Coin)
    let coin_bytes = encode_coin(&max_tokens);
    bytes.extend(encode_length_delimited(1, &coin_bytes));

    // Field 2: allow_list (StakeAuthorizationValidators) - oneof
    if let Some(ref validators) = allow_list {
        if !validators.is_empty() {
            let mut validators_bytes = Vec::new();
            for addr in validators {
                validators_bytes.extend(encode_string_field(1, addr));
            }
            bytes.extend(encode_length_delimited(2, &validators_bytes));
        }
    }

    // Field 3: deny_list (StakeAuthorizationValidators) - oneof
    if let Some(ref validators) = deny_list {
        if !validators.is_empty() {
            let mut validators_bytes = Vec::new();
            for addr in validators {
                validators_bytes.extend(encode_string_field(1, addr));
            }
            bytes.extend(encode_length_delimited(3, &validators_bytes));
        }
    }

    // Field 4: authorization_type (enum, varint)
    let tag = (4 << 3) as u64; // wire type 0 for varint
    bytes.extend(encode_varint(tag));
    bytes.extend(encode_varint(authorization_type as u64));

    Ok(encode_to_base64(&bytes))
}

/// Encode TransferAuthorization to protobuf base64 (for IBC transfers)
///
/// # Arguments
/// * `allocations` - List of IBC allocation configurations
///
/// # Returns
/// Base64-encoded protobuf TransferAuthorization message
pub fn encode_ibc_transfer_authorization(
    allocations: Vec<IbcAllocation>,
) -> Result<String, EncodingError> {
    if allocations.is_empty() {
        return Err(EncodingError::InvalidInput(
            "TransferAuthorization requires at least one allocation".into(),
        ));
    }

    let mut bytes = Vec::new();

    // Field 1: allocations (repeated Allocation)
    for alloc in &allocations {
        let mut alloc_bytes = Vec::new();

        // Field 1: source_port (string)
        alloc_bytes.extend(encode_string_field(1, &alloc.source_port));

        // Field 2: source_channel (string)
        alloc_bytes.extend(encode_string_field(2, &alloc.source_channel));

        // Field 3: spend_limit (repeated Coin)
        alloc_bytes.extend(encode_repeated_coins(3, &alloc.spend_limit));

        // Field 4: allow_list (repeated string)
        if let Some(ref addrs) = alloc.allow_list {
            for addr in addrs {
                alloc_bytes.extend(encode_string_field(4, addr));
            }
        }

        bytes.extend(encode_length_delimited(1, &alloc_bytes));
    }

    Ok(encode_to_base64(&bytes))
}

/// Encode ContractExecutionAuthorization to protobuf base64
///
/// # Arguments
/// * `grants` - List of contract execution grants
///
/// # Returns
/// Base64-encoded protobuf ContractExecutionAuthorization message
pub fn encode_contract_execution_authorization(
    grants: Vec<ContractGrant>,
) -> Result<String, EncodingError> {
    if grants.is_empty() {
        return Err(EncodingError::InvalidInput(
            "ContractExecutionAuthorization requires at least one grant".into(),
        ));
    }

    let mut bytes = Vec::new();

    // Field 1: grants (repeated ContractGrant)
    for grant in &grants {
        let mut grant_bytes = Vec::new();

        // Field 1: contract (string)
        grant_bytes.extend(encode_string_field(1, &grant.address));

        // Field 2: limit (Any) - encode based on max_calls and max_funds
        let limit_bytes = match (&grant.max_calls, &grant.max_funds) {
            (Some(calls), Some(funds)) => {
                // CombinedLimit
                let mut combined_bytes = Vec::new();
                // Field 1: calls_remaining (uint64, varint)
                let tag = (1 << 3) as u64;
                combined_bytes.extend(encode_varint(tag));
                combined_bytes.extend(encode_varint(*calls));
                // Field 2: amounts (repeated Coin)
                combined_bytes.extend(encode_repeated_coins(2, funds));

                encode_any("/cosmwasm.wasm.v1.CombinedLimit", &combined_bytes)
            }
            (Some(calls), None) => {
                // MaxCallsLimit
                let mut max_calls_bytes = Vec::new();
                // Field 1: remaining (uint64, varint)
                let tag = (1 << 3) as u64;
                max_calls_bytes.extend(encode_varint(tag));
                max_calls_bytes.extend(encode_varint(*calls));

                encode_any("/cosmwasm.wasm.v1.MaxCallsLimit", &max_calls_bytes)
            }
            (None, Some(funds)) => {
                // MaxFundsLimit
                let mut max_funds_bytes = Vec::new();
                // Field 1: amounts (repeated Coin)
                max_funds_bytes.extend(encode_repeated_coins(1, funds));

                encode_any("/cosmwasm.wasm.v1.MaxFundsLimit", &max_funds_bytes)
            }
            (None, None) => {
                return Err(EncodingError::InvalidInput(
                    "ContractGrant requires max_calls or max_funds or both".into(),
                ));
            }
        };
        grant_bytes.extend(encode_length_delimited(2, &limit_bytes));

        // Field 3: filter (Any)
        let filter_bytes = match grant.filter_type.as_str() {
            "allow_all" => {
                // AllowAllMessagesFilter - empty message
                encode_any("/cosmwasm.wasm.v1.AllowAllMessagesFilter", &[])
            }
            "accepted_keys" => {
                // AcceptedMessageKeysFilter
                let keys = grant.keys.as_ref().ok_or_else(|| {
                    EncodingError::InvalidInput("accepted_keys filter requires keys field".into())
                })?;

                let mut keys_bytes = Vec::new();
                // Field 1: keys (repeated string)
                for key in keys {
                    keys_bytes.extend(encode_string_field(1, key));
                }

                encode_any("/cosmwasm.wasm.v1.AcceptedMessageKeysFilter", &keys_bytes)
            }
            _ => {
                return Err(EncodingError::InvalidInput(format!(
                    "Invalid filter_type: {}",
                    grant.filter_type
                )));
            }
        };
        grant_bytes.extend(encode_length_delimited(3, &filter_bytes));

        bytes.extend(encode_length_delimited(1, &grant_bytes));
    }

    Ok(encode_to_base64(&bytes))
}

// ============================================================================
// AUTHORIZATION INPUT ENCODING (for CLI commands)
// ============================================================================

/// Encode AuthorizationInput to (type_url, base64_value)
///
/// # Arguments
/// * `auth_input` - Authorization input from CLI
/// * `msg_type_url` - The message type URL being authorized (for GenericAuthorization)
///
/// # Returns
/// Tuple of (type_url, base64_encoded_value)
pub fn encode_authorization_input(
    auth_input: &super::types::AuthorizationInput,
    msg_type_url: &str,
) -> Result<(String, String), EncodingError> {
    match auth_input {
        super::types::AuthorizationInput::Generic => {
            // GenericAuthorization needs the msg field (the message type being authorized)
            let encoded = encode_generic_authorization(msg_type_url)?;
            Ok((
                "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
                encoded,
            ))
        }
        super::types::AuthorizationInput::Send {
            spend_limit,
            allow_list,
        } => {
            let coins = parse_coin_string(spend_limit)?;
            let encoded = encode_send_authorization(coins, allow_list.clone())?;
            Ok((
                "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
                encoded,
            ))
        }
        super::types::AuthorizationInput::Stake {
            max_tokens,
            validators,
            deny_validators,
            authorization_type,
        } => {
            let coin = parse_single_denom(max_tokens)?;
            let encoded = encode_stake_authorization(
                coin,
                validators.clone(),
                deny_validators.clone(),
                *authorization_type,
            )?;
            Ok((
                "/cosmos.staking.v1beta1.StakeAuthorization".to_string(),
                encoded,
            ))
        }
        super::types::AuthorizationInput::IbcTransfer { allocations } => {
            let ibc_allocations: Vec<IbcAllocation> = allocations
                .iter()
                .map(|a| {
                    let coins = parse_coin_string(&a.spend_limit).map_err(|e| {
                        EncodingError::InvalidInput(format!("Invalid spend_limit: {}", e))
                    })?;
                    Ok(IbcAllocation {
                        source_port: a.source_port.clone(),
                        source_channel: a.source_channel.clone(),
                        spend_limit: coins,
                        allow_list: a.allow_list.clone(),
                    })
                })
                .collect::<Result<Vec<_>, EncodingError>>()?;

            let encoded = encode_ibc_transfer_authorization(ibc_allocations)?;
            Ok((
                "/ibc.applications.transfer.v1.TransferAuthorization".to_string(),
                encoded,
            ))
        }
        super::types::AuthorizationInput::ContractExecution { grants } => {
            let contract_grants: Vec<ContractGrant> = grants
                .iter()
                .map(|g| {
                    let max_funds = if let Some(ref funds) = g.max_funds {
                        Some(parse_coin_string(funds)?)
                    } else {
                        None
                    };

                    Ok(ContractGrant {
                        address: g.address.clone(),
                        max_calls: g.max_calls,
                        max_funds,
                        filter_type: g.filter_type.clone(),
                        keys: g.keys.clone(),
                    })
                })
                .collect::<Result<Vec<_>, EncodingError>>()?;

            let encoded = encode_contract_execution_authorization(contract_grants)?;
            Ok((
                "/cosmwasm.wasm.v1.ContractExecutionAuthorization".to_string(),
                encoded,
            ))
        }
    }
}

/// Encode FeeConfigInput to (allowance_type_url, base64_value)
///
/// # Arguments
/// * `fee_config` - Fee config input from CLI
///
/// # Returns
/// Tuple of (allowance_type_url, base64_encoded_value)
pub fn encode_fee_config_input(
    fee_config: &super::types::FeeConfigInput,
) -> Result<(String, String), EncodingError> {
    match fee_config {
        super::types::FeeConfigInput::Basic { spend_limit, .. } => {
            let coins = parse_coin_string(spend_limit)?;
            let encoded = encode_basic_allowance(coins)?;
            Ok((
                "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
                encoded,
            ))
        }
        super::types::FeeConfigInput::Periodic {
            basic_spend_limit,
            period_seconds,
            period_spend_limit,
            ..
        } => {
            let basic_limit = if let Some(ref limit) = basic_spend_limit {
                Some(parse_coin_string(limit)?)
            } else {
                None
            };
            let period_limit = parse_coin_string(period_spend_limit)?;
            let encoded = encode_periodic_allowance(basic_limit, *period_seconds, period_limit)?;
            Ok((
                "/cosmos.feegrant.v1beta1.PeriodicAllowance".to_string(),
                encoded,
            ))
        }
        super::types::FeeConfigInput::AllowedMsg {
            allowed_messages,
            nested_allowance,
            ..
        } => {
            // Recursively encode nested allowance
            let (nested_type_url, nested_value) = encode_fee_config_input(nested_allowance)?;
            let encoded = encode_allowed_msg_allowance(
                allowed_messages.clone(),
                &nested_type_url,
                &nested_value, // Pass base64 string directly
            )?;
            Ok((
                "/cosmos.feegrant.v1beta1.AllowedMsgAllowance".to_string(),
                encoded,
            ))
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coin_string_single() {
        let coins = parse_coin_string("1000000uxion").unwrap();
        assert_eq!(coins.len(), 1);
        assert_eq!(coins[0].amount, "1000000");
        assert_eq!(coins[0].denom, "uxion");
    }

    #[test]
    fn test_parse_coin_string_multiple() {
        let coins = parse_coin_string("1000000uxion,500uatom").unwrap();
        assert_eq!(coins.len(), 2);
        // Should be sorted by denom
        assert_eq!(coins[0].denom, "uatom");
        assert_eq!(coins[0].amount, "500");
        assert_eq!(coins[1].denom, "uxion");
        assert_eq!(coins[1].amount, "1000000");
    }

    #[test]
    fn test_parse_coin_string_ibc() {
        let coins = parse_coin_string("1000ibc/xion123/fight-channel").unwrap();
        assert_eq!(coins.len(), 1);
        assert_eq!(coins[0].amount, "1000");
        assert_eq!(coins[0].denom, "ibc/xion123/fight-channel");
    }

    #[test]
    fn test_parse_coin_string_invalid() {
        assert!(parse_coin_string("").is_err());
        assert!(parse_coin_string("invalid").is_err());
        assert!(parse_coin_string("uxion").is_err());
    }

    #[test]
    fn test_parse_single_denom() {
        let coin = parse_single_denom("1000000uxion").unwrap();
        assert_eq!(coin.amount, "1000000");
        assert_eq!(coin.denom, "uxion");
    }

    #[test]
    fn test_parse_single_denom_multiple_coins() {
        assert!(parse_single_denom("100uxion,200uatom").is_err());
    }

    #[test]
    fn test_encode_basic_allowance() {
        let coins = vec![Coin {
            denom: "uxion".into(),
            amount: "1000000".into(),
        }];
        let encoded = encode_basic_allowance(coins).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());

        // Verify the encoded structure contains expected data
        // BasicAllowance with spend_limit field
        assert!(decoded.starts_with(&[0x0a])); // field 1, wire type 2
    }

    #[test]
    fn test_encode_basic_allowance_multiple_coins() {
        let coins = vec![
            Coin {
                denom: "uatom".into(),
                amount: "500".into(),
            },
            Coin {
                denom: "uxion".into(),
                amount: "1000000".into(),
            },
        ];
        let encoded = encode_basic_allowance(coins).unwrap();

        // Should encode successfully
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_basic_allowance_empty() {
        assert!(encode_basic_allowance(vec![]).is_err());
    }

    #[test]
    fn test_encode_periodic_allowance() {
        let period_limit = vec![Coin {
            denom: "uxion".into(),
            amount: "1000000".into(),
        }];
        let encoded = encode_periodic_allowance(None, 86400, period_limit).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_periodic_allowance_with_basic() {
        let basic_limit = vec![Coin {
            denom: "uxion".into(),
            amount: "10000000".into(),
        }];
        let period_limit = vec![Coin {
            denom: "uxion".into(),
            amount: "1000000".into(),
        }];
        let encoded = encode_periodic_allowance(Some(basic_limit), 3600, period_limit).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_periodic_allowance_empty() {
        assert!(encode_periodic_allowance(None, 86400, vec![]).is_err());
    }

    #[test]
    fn test_encode_allowed_msg_allowance() {
        let basic_encoded = encode_basic_allowance(vec![Coin {
            denom: "uxion".into(),
            amount: "1000000".into(),
        }])
        .unwrap();

        let encoded = encode_allowed_msg_allowance(
            vec!["/cosmos.bank.v1beta1.MsgSend".to_string()],
            "/cosmos.feegrant.v1beta1.BasicAllowance",
            &basic_encoded, // Pass base64 string directly
        )
        .unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_allowed_msg_allowance_empty_messages() {
        assert!(encode_allowed_msg_allowance(
            vec![],
            "/cosmos.feegrant.v1beta1.BasicAllowance",
            "dummy" // Dummy base64 string
        )
        .is_err());
    }

    #[test]
    fn test_encode_generic_authorization() {
        let encoded = encode_generic_authorization("/cosmos.bank.v1beta1.MsgSend").unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());

        // Should contain the message type URL
        let decoded_str = String::from_utf8_lossy(&decoded);
        assert!(decoded_str.contains("MsgSend"));
    }

    #[test]
    fn test_encode_generic_authorization_empty() {
        assert!(encode_generic_authorization("").is_err());
    }

    #[test]
    fn test_encode_send_authorization() {
        let coins = vec![Coin {
            denom: "uxion".into(),
            amount: "1000000".into(),
        }];
        let encoded = encode_send_authorization(coins, None).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_send_authorization_with_allow_list() {
        let coins = vec![Coin {
            denom: "uxion".into(),
            amount: "1000000".into(),
        }];
        let allow_list = vec!["xion1abc...".to_string(), "xion1def...".to_string()];
        let encoded = encode_send_authorization(coins, Some(allow_list)).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_send_authorization_empty() {
        assert!(encode_send_authorization(vec![], None).is_err());
    }

    #[test]
    fn test_encode_stake_authorization_delegate() {
        let coin = Coin {
            denom: "uxion".into(),
            amount: "10000000".into(),
        };
        let encoded = encode_stake_authorization(coin, None, None, 1).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_stake_authorization_with_validators() {
        let coin = Coin {
            denom: "uxion".into(),
            amount: "10000000".into(),
        };
        let validators = vec!["xionvaloper1abc...".to_string()];
        let encoded = encode_stake_authorization(coin, Some(validators), None, 1).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_stake_authorization_invalid_type() {
        let coin = Coin {
            denom: "uxion".into(),
            amount: "10000000".into(),
        };
        assert!(encode_stake_authorization(coin.clone(), None, None, 0).is_err());
        assert!(encode_stake_authorization(coin, None, None, 4).is_err());
    }

    #[test]
    fn test_encode_ibc_transfer_authorization() {
        let allocations = vec![IbcAllocation {
            source_port: "transfer".to_string(),
            source_channel: "channel-1".to_string(),
            spend_limit: vec![Coin {
                denom: "uxion".into(),
                amount: "1000000".into(),
            }],
            allow_list: None,
        }];
        let encoded = encode_ibc_transfer_authorization(allocations).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_ibc_transfer_authorization_with_allow_list() {
        let allocations = vec![IbcAllocation {
            source_port: "transfer".to_string(),
            source_channel: "channel-1".to_string(),
            spend_limit: vec![Coin {
                denom: "uxion".into(),
                amount: "1000000".into(),
            }],
            allow_list: Some(vec!["xion1recipient...".to_string()]),
        }];
        let encoded = encode_ibc_transfer_authorization(allocations).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_ibc_transfer_authorization_empty() {
        assert!(encode_ibc_transfer_authorization(vec![]).is_err());
    }

    #[test]
    fn test_encode_contract_execution_authorization_max_calls() {
        let grants = vec![ContractGrant {
            address: "xion1contract...".to_string(),
            max_calls: Some(100),
            max_funds: None,
            filter_type: "allow_all".to_string(),
            keys: None,
        }];
        let encoded = encode_contract_execution_authorization(grants).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_contract_execution_authorization_max_funds() {
        let grants = vec![ContractGrant {
            address: "xion1contract...".to_string(),
            max_calls: None,
            max_funds: Some(vec![Coin {
                denom: "uxion".into(),
                amount: "5000000".into(),
            }]),
            filter_type: "allow_all".to_string(),
            keys: None,
        }];
        let encoded = encode_contract_execution_authorization(grants).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_contract_execution_authorization_combined() {
        let grants = vec![ContractGrant {
            address: "xion1contract...".to_string(),
            max_calls: Some(100),
            max_funds: Some(vec![Coin {
                denom: "uxion".into(),
                amount: "5000000".into(),
            }]),
            filter_type: "allow_all".to_string(),
            keys: None,
        }];
        let encoded = encode_contract_execution_authorization(grants).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_contract_execution_authorization_accepted_keys() {
        let grants = vec![ContractGrant {
            address: "xion1contract...".to_string(),
            max_calls: Some(100),
            max_funds: None,
            filter_type: "accepted_keys".to_string(),
            keys: Some(vec!["transfer".to_string(), "mint".to_string()]),
        }];
        let encoded = encode_contract_execution_authorization(grants).unwrap();

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_encode_contract_execution_authorization_no_limits() {
        let grants = vec![ContractGrant {
            address: "xion1contract...".to_string(),
            max_calls: None,
            max_funds: None,
            filter_type: "allow_all".to_string(),
            keys: None,
        }];
        assert!(encode_contract_execution_authorization(grants).is_err());
    }

    #[test]
    fn test_encode_contract_execution_authorization_empty_keys() {
        let grants = vec![ContractGrant {
            address: "xion1contract...".to_string(),
            max_calls: Some(100),
            max_funds: None,
            filter_type: "accepted_keys".to_string(),
            keys: None,
        }];
        assert!(encode_contract_execution_authorization(grants).is_err());
    }

    #[test]
    fn test_encode_contract_execution_authorization_empty() {
        assert!(encode_contract_execution_authorization(vec![]).is_err());
    }

    #[test]
    fn test_encode_to_base64() {
        let bytes = vec![0x0a, 0x03, 0x66, 0x6f, 0x6f];
        let encoded = encode_to_base64(&bytes);
        assert!(!encoded.is_empty());

        // Should be valid base64
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert_eq!(decoded, bytes);
    }
}
