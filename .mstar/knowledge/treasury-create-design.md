# Treasury Create Command Enhancement - Design Document

## Overview

This document specifies the CLI interface, data structures, and encoding module design for enhancing the `xion-toolkit treasury create` command to match Developer Portal capabilities.

## 1. CLI Interface Specification

### 1.1 Command Structure

The command will support two approaches:
1. **Flag-based**: Simple to moderate configurations using command-line flags
2. **Config-file-based**: Complex configurations using a JSON file

```bash
xion-toolkit treasury create [OPTIONS]
xion-toolkit treasury create --config <FILE>
```

### 1.2 Complete Flag List

#### Treasury Parameters (Required)

| Flag | Short | Type | Required | Default | Description |
|------|-------|------|----------|---------|-------------|
| `--redirect-url` | `-r` | String | Yes | - | OAuth callback URL |
| `--icon-url` | `-i` | String | Yes | - | Treasury icon URL |
| `--name` | `-n` | String | No | "" | Treasury display name |
| `--is-oauth2-app` | | Flag | No | false | Mark as OAuth2 application |

#### Fee Grant Configuration

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--fee-allowance-type` | String | Yes* | Allowance type: `basic`, `periodic`, `allowed-msg` |
| `--fee-spend-limit` | String | Conditional | Spend limit (e.g., "1000000uxion") |
| `--fee-period-seconds` | u64 | Conditional | Period duration in seconds (periodic only) |
| `--fee-period-spend-limit` | String | Conditional | Period spend limit (periodic only) |
| `--fee-allowed-messages` | String | Conditional | Comma-separated message type URLs (allowed-msg only) |
| `--fee-sub-allowance-type` | String | Conditional | Nested allowance type (allowed-msg only) |
| `--fee-description` | String | Yes | Fee grant description |

#### Grant Configuration (Repeatable)

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--grant-type-url` | String | Yes | Message type URL (e.g., "/cosmos.bank.v1beta1.MsgSend") |
| `--grant-auth-type` | String | Yes | Authorization type (see below) |
| `--grant-description` | String | Yes | Permission description |
| `--grant-optional` | Flag | No | Mark as optional permission |

#### Authorization-Specific Flags

**For Generic Authorization**:
- No additional flags required

**For Send Authorization** (`--grant-auth-type send`):
- `--grant-spend-limit`: Spend limit (required)
- `--grant-allow-list`: Comma-separated addresses (optional)

**For Stake Authorization** (`--grant-auth-type stake`):
- `--grant-max-tokens`: Max tokens to stake (required)
- `--grant-validators`: Comma-separated validator addresses (optional)
- `--grant-deny-validators`: Comma-separated denied validators (optional)
- `--grant-auth-mode`: `delegate`, `undelegate`, or `redelegate` (default: delegate)

**For IBC Transfer Authorization** (`--grant-auth-type ibc-transfer`):
- `--grant-ibc-allocations`: JSON array or repeated flag for allocations
  - Format: `port:channel:limit[:allow_list]`
  - Example: `transfer:channel-1:1000000uxion:addr1,addr2`

**For Contract Execution Authorization** (`--grant-auth-type contract-execution`):
- `--grant-contract-address`: Contract address (required)
- `--grant-max-calls`: Max execution calls (optional)
- `--grant-max-funds`: Max funds limit (optional)
- `--grant-filter-type`: `allow-all` or `accepted-keys` (default: allow-all)
- `--grant-accepted-keys`: Comma-separated message keys (if filter is accepted-keys)

#### Alternative Input

| Flag | Type | Description |
|------|------|-------------|
| `--config` | Path | Path to JSON configuration file |

### 1.3 CLI Usage Examples

#### Example 1: Minimal Treasury Creation

```bash
xion-toolkit treasury create \
  --redirect-url "https://myapp.com/callback" \
  --icon-url "https://myapp.com/icon.png" \
  --name "My Treasury" \
  --fee-allowance-type basic \
  --fee-spend-limit "1000000uxion" \
  --fee-description "Basic fee allowance" \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending funds"
```

**Output:**
```json
{
  "success": true,
  "treasury_address": "xion1...",
  "tx_hash": "ABC123...",
  "admin": "xion1...",
  "created_at": "2026-03-06T12:00:00Z"
}
```

#### Example 2: Treasury with Basic Fee Grant (Simplest)

```bash
xion-toolkit treasury create \
  --redirect-url "https://myapp.com/callback" \
  --icon-url "https://myapp.com/icon.png" \
  --fee-allowance-type basic \
  --fee-spend-limit "5000000uxion" \
  --fee-description "Gas fees for user transactions" \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type generic \
  --grant-description "Generic send permission"
```

#### Example 3: Treasury with Periodic Fee Grant

```bash
xion-toolkit treasury create \
  --redirect-url "https://myapp.com/callback" \
  --icon-url "https://myapp.com/icon.png" \
  --fee-allowance-type periodic \
  --fee-spend-limit "10000000uxion" \
  --fee-period-seconds 86400 \
  --fee-period-spend-limit "1000000uxion" \
  --fee-description "Daily fee allowance limit" \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending up to 1M uxion"
```

#### Example 4: Treasury with Single Permission (Generic)

```bash
xion-toolkit treasury create \
  --redirect-url "https://myapp.com/callback" \
  --icon-url "https://myapp.com/icon.png" \
  --fee-allowance-type basic \
  --fee-spend-limit "1000000uxion" \
  --fee-description "Fee grant" \
  --grant-type-url "/cosmwasm.wasm.v1.MsgExecuteContract" \
  --grant-auth-type generic \
  --grant-description "Execute any contract"
```

#### Example 5: Complex Treasury with Multiple Permissions

Using flag approach (for moderate complexity):

```bash
xion-toolkit treasury create \
  --redirect-url "https://myapp.com/callback" \
  --icon-url "https://myapp.com/icon.png" \
  --name "Production Treasury" \
  --is-oauth2-app \
  --fee-allowance-type periodic \
  --fee-spend-limit "10000000uxion" \
  --fee-period-seconds 86400 \
  --fee-period-spend-limit "2000000uxion" \
  --fee-description "Daily operational fees" \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "5000000uxion" \
  --grant-allow-list "xion1abc...,xion1def..." \
  --grant-description "Send to whitelisted addresses" \
  --grant-type-url "/cosmos.staking.v1beta1.MsgDelegate" \
  --grant-auth-type stake \
  --grant-max-tokens "10000000uxion" \
  --grant-validators "xionvaloper1abc..." \
  --grant-auth-mode delegate \
  --grant-description "Delegate to specific validators"
```

#### Example 6: Complex Treasury with Config File

For very complex configurations, use a config file:

```bash
xion-toolkit treasury create --config treasury-config.json
```

**treasury-config.json:**
```json
{
  "params": {
    "redirect_url": "https://myapp.com/callback",
    "icon_url": "https://myapp.com/icon.png",
    "name": "Production Treasury",
    "is_oauth2_app": true
  },
  "fee_config": {
    "description": "Advanced fee configuration with message restrictions",
    "allowance_type": "allowed-msg",
    "spend_limit": "10000000uxion",
    "allowed_messages": [
      "/cosmos.bank.v1beta1.MsgSend",
      "/cosmwasm.wasm.v1.MsgExecuteContract"
    ],
    "sub_allowance": {
      "type": "periodic",
      "period_seconds": 3600,
      "period_spend_limit": "1000000uxion"
    }
  },
  "grant_configs": [
    {
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "description": "Send funds to whitelisted addresses",
      "authorization": {
        "type": "send",
        "spend_limit": "10000000uxion",
        "allow_list": [
          "xion1abc123...",
          "xion1def456..."
        ]
      },
      "optional": false
    },
    {
      "type_url": "/cosmos.staking.v1beta1.MsgDelegate",
      "description": "Delegate to trusted validators",
      "authorization": {
        "type": "stake",
        "max_tokens": "50000000uxion",
        "validators": ["xionvaloper1abc..."],
        "authorization_type": "delegate"
      },
      "optional": false
    },
    {
      "type_url": "/cosmwasm.wasm.v1.MsgExecuteContract",
      "description": "Execute specific contracts with limits",
      "authorization": {
        "type": "contract-execution",
        "contracts": [
          {
            "address": "xion1contract1...",
            "max_calls": 1000,
            "max_funds": "5000000uxion",
            "filter": {
              "type": "accepted-keys",
              "keys": ["transfer", "mint", "burn"]
            }
          }
        ]
      },
      "optional": true
    },
    {
      "type_url": "/ibc.applications.transfer.v1.MsgTransfer",
      "description": "IBC transfers with channel restrictions",
      "authorization": {
        "type": "ibc-transfer",
        "allocations": [
          {
            "source_port": "transfer",
            "source_channel": "channel-1",
            "spend_limit": "10000000uxion",
            "allow_list": []
          },
          {
            "source_port": "transfer",
            "source_channel": "channel-5",
            "spend_limit": "5000000uxion",
            "allow_list": ["xion1recipient..."]
          }
        ]
      },
      "optional": false
    }
  ]
}
```

### 1.4 Config File Schema (JSON Schema)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["params", "fee_config", "grant_configs"],
  "properties": {
    "params": {
      "type": "object",
      "required": ["redirect_url", "icon_url"],
      "properties": {
        "redirect_url": { "type": "string", "format": "uri" },
        "icon_url": { "type": "string", "format": "uri" },
        "name": { "type": "string" },
        "is_oauth2_app": { "type": "boolean", "default": false }
      }
    },
    "fee_config": {
      "type": "object",
      "required": ["description", "allowance_type"],
      "properties": {
        "description": { "type": "string" },
        "allowance_type": { 
          "type": "string", 
          "enum": ["basic", "periodic", "allowed-msg"] 
        },
        "spend_limit": { "type": "string" },
        "period_seconds": { "type": "integer" },
        "period_spend_limit": { "type": "string" },
        "allowed_messages": { 
          "type": "array", 
          "items": { "type": "string" } 
        },
        "sub_allowance": {
          "type": "object",
          "properties": {
            "type": { "type": "string", "enum": ["basic", "periodic"] },
            "spend_limit": { "type": "string" },
            "period_seconds": { "type": "integer" },
            "period_spend_limit": { "type": "string" }
          }
        }
      }
    },
    "grant_configs": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "required": ["type_url", "description", "authorization"],
        "properties": {
          "type_url": { "type": "string" },
          "description": { "type": "string" },
          "optional": { "type": "boolean", "default": false },
          "authorization": {
            "type": "object",
            "oneOf": [
              { "$ref": "#/definitions/GenericAuthorization" },
              { "$ref": "#/definitions/SendAuthorization" },
              { "$ref": "#/definitions/StakeAuthorization" },
              { "$ref": "#/definitions/IbcTransferAuthorization" },
              { "$ref": "#/definitions/ContractExecutionAuthorization" }
            ]
          }
        }
      }
    }
  },
  "definitions": {
    "GenericAuthorization": {
      "type": "object",
      "required": ["type"],
      "properties": {
        "type": { "const": "generic" }
      }
    },
    "SendAuthorization": {
      "type": "object",
      "required": ["type", "spend_limit"],
      "properties": {
        "type": { "const": "send" },
        "spend_limit": { "type": "string" },
        "allow_list": { "type": "array", "items": { "type": "string" } }
      }
    },
    "StakeAuthorization": {
      "type": "object",
      "required": ["type", "max_tokens", "authorization_type"],
      "properties": {
        "type": { "const": "stake" },
        "max_tokens": { "type": "string" },
        "validators": { "type": "array", "items": { "type": "string" } },
        "deny_validators": { "type": "array", "items": { "type": "string" } },
        "authorization_type": { 
          "type": "string", 
          "enum": ["delegate", "undelegate", "redelegate"] 
        }
      }
    },
    "IbcTransferAuthorization": {
      "type": "object",
      "required": ["type", "allocations"],
      "properties": {
        "type": { "const": "ibc-transfer" },
        "allocations": {
          "type": "array",
          "items": {
            "type": "object",
            "required": ["source_port", "source_channel", "spend_limit"],
            "properties": {
              "source_port": { "type": "string" },
              "source_channel": { "type": "string" },
              "spend_limit": { "type": "string" },
              "allow_list": { "type": "array", "items": { "type": "string" } }
            }
          }
        }
      }
    },
    "ContractExecutionAuthorization": {
      "type": "object",
      "required": ["type", "contracts"],
      "properties": {
        "type": { "const": "contract-execution" },
        "contracts": {
          "type": "array",
          "items": {
            "type": "object",
            "required": ["address", "filter"],
            "properties": {
              "address": { "type": "string" },
              "max_calls": { "type": "integer" },
              "max_funds": { "type": "string" },
              "filter": {
                "type": "object",
                "required": ["type"],
                "properties": {
                  "type": { 
                    "type": "string", 
                    "enum": ["allow-all", "accepted-keys"] 
                  },
                  "keys": { "type": "array", "items": { "type": "string" } }
                }
              }
            }
          }
        }
      }
    }
  }
}
```

## 2. Data Structures

### 2.1 Core Types

```rust
// src/treasury/types.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Treasury creation request from CLI or config file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryCreateRequest {
    /// Treasury parameters
    pub params: TreasuryParamsInput,
    /// Fee grant configuration
    pub fee_config: FeeConfigInput,
    /// Grant configurations (at least one required)
    pub grant_configs: Vec<GrantConfigInput>,
}

/// Treasury parameters input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParamsInput {
    /// OAuth callback URL (required)
    pub redirect_url: String,
    /// Treasury icon URL (required)
    pub icon_url: String,
    /// Treasury display name (optional)
    #[serde(default)]
    pub name: String,
    /// Mark as OAuth2 application (optional)
    #[serde(default)]
    pub is_oauth2_app: bool,
}

impl TreasuryParamsInput {
    /// Convert to chain format (metadata as JSON string)
    pub fn to_chain_format(&self) -> TreasuryParamsChain {
        let metadata = TreasuryMetadata {
            name: self.name.clone(),
            archived: false,
            is_oauth2_app: self.is_oauth2_app,
        };
        
        TreasuryParamsChain {
            redirect_url: self.redirect_url.clone(),
            icon_url: self.icon_url.clone(),
            metadata: serde_json::to_string(&metadata).unwrap_or_else(|_| "{}".to_string()),
        }
    }
    
    /// Validate parameters
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.redirect_url.is_empty() {
            return Err(ValidationError::MissingField("redirect_url"));
        }
        if self.icon_url.is_empty() {
            return Err(ValidationError::MissingField("icon_url"));
        }
        
        // Validate URL format
        url::Url::parse(&self.redirect_url)
            .map_err(|_| ValidationError::InvalidUrl("redirect_url"))?;
        url::Url::parse(&self.icon_url)
            .map_err(|_| ValidationError::InvalidUrl("icon_url"))?;
        
        Ok(())
    }
}

/// Treasury parameters in chain format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryParamsChain {
    pub redirect_url: String,
    pub icon_url: String,
    pub metadata: String, // JSON string
}

/// Treasury metadata (embedded in params)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryMetadata {
    pub name: String,
    pub archived: bool,
    pub is_oauth2_app: bool,
}

// ============================================================================
// Fee Configuration
// ============================================================================

/// Fee configuration input (from CLI or config file)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "allowance_type", rename_all = "kebab-case")]
pub enum FeeConfigInput {
    Basic(BasicAllowanceInput),
    Periodic(PeriodicAllowanceInput),
    AllowedMsg(AllowedMsgAllowanceInput),
}

/// Basic allowance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAllowanceInput {
    pub description: String,
    pub spend_limit: String, // e.g., "1000000uxion"
}

/// Periodic allowance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodicAllowanceInput {
    pub description: String,
    #[serde(default)]
    pub spend_limit: Option<String>, // Optional basic spend limit
    pub period_seconds: u64,
    pub period_spend_limit: String,
}

/// Allowed message allowance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedMsgAllowanceInput {
    pub description: String,
    pub allowed_messages: Vec<String>, // Type URLs
    pub sub_allowance: SubAllowanceInput,
}

/// Sub-allowance for AllowedMsgAllowance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SubAllowanceInput {
    Basic {
        spend_limit: String,
    },
    Periodic {
        #[serde(default)]
        spend_limit: Option<String>,
        period_seconds: u64,
        period_spend_limit: String,
    },
}

impl FeeConfigInput {
    /// Get description
    pub fn description(&self) -> &str {
        match self {
            FeeConfigInput::Basic(b) => &b.description,
            FeeConfigInput::Periodic(p) => &p.description,
            FeeConfigInput::AllowedMsg(a) => &a.description,
        }
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            FeeConfigInput::Basic(b) => {
                if b.spend_limit.is_empty() {
                    return Err(ValidationError::MissingField("spend_limit"));
                }
                parse_coin_string(&b.spend_limit)?;
            }
            FeeConfigInput::Periodic(p) => {
                if p.period_seconds == 0 {
                    return Err(ValidationError::InvalidValue("period_seconds must be > 0"));
                }
                if p.period_spend_limit.is_empty() {
                    return Err(ValidationError::MissingField("period_spend_limit"));
                }
                parse_coin_string(&p.period_spend_limit)?;
                if let Some(ref limit) = p.spend_limit {
                    parse_coin_string(limit)?;
                }
            }
            FeeConfigInput::AllowedMsg(a) => {
                if a.allowed_messages.is_empty() {
                    return Err(ValidationError::MissingField("allowed_messages"));
                }
                a.sub_allowance.validate()?;
            }
        }
        
        if self.description().is_empty() {
            return Err(ValidationError::MissingField("description"));
        }
        
        Ok(())
    }
}

impl SubAllowanceInput {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            SubAllowanceInput::Basic { spend_limit } => {
                if spend_limit.is_empty() {
                    return Err(ValidationError::MissingField("spend_limit"));
                }
                parse_coin_string(spend_limit)?;
            }
            SubAllowanceInput::Periodic { spend_limit, period_seconds, period_spend_limit } => {
                if *period_seconds == 0 {
                    return Err(ValidationError::InvalidValue("period_seconds must be > 0"));
                }
                if period_spend_limit.is_empty() {
                    return Err(ValidationError::MissingField("period_spend_limit"));
                }
                parse_coin_string(period_spend_limit)?;
                if let Some(ref limit) = spend_limit {
                    parse_coin_string(limit)?;
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Grant Configuration
// ============================================================================

/// Grant configuration input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigInput {
    /// Message type URL (e.g., "/cosmos.bank.v1beta1.MsgSend")
    pub type_url: String,
    /// Human-readable description
    pub description: String,
    /// Authorization configuration
    pub authorization: AuthorizationInput,
    /// Whether this grant is optional
    #[serde(default)]
    pub optional: bool,
}

/// Authorization input variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AuthorizationInput {
    Generic,
    Send {
        spend_limit: String,
        #[serde(default)]
        allow_list: Vec<String>,
    },
    Stake {
        max_tokens: String,
        #[serde(default)]
        validators: Vec<String>,
        #[serde(default)]
        deny_validators: Vec<String>,
        authorization_type: StakeAuthorizationType,
    },
    IbcTransfer {
        allocations: Vec<IbcAllocationInput>,
    },
    ContractExecution {
        contracts: Vec<ContractGrantInput>,
    },
}

/// Stake authorization type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum StakeAuthorizationType {
    Delegate,
    Undelegate,
    Redelegate,
}

/// IBC transfer allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IbcAllocationInput {
    pub source_port: String,
    pub source_channel: String,
    pub spend_limit: String,
    #[serde(default)]
    pub allow_list: Vec<String>,
}

/// Contract execution grant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGrantInput {
    pub address: String,
    #[serde(default)]
    pub max_calls: Option<u64>,
    #[serde(default)]
    pub max_funds: Option<String>,
    pub filter: ContractFilterInput,
}

/// Contract execution filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ContractFilterInput {
    AllowAll,
    AcceptedKeys {
        keys: Vec<String>,
    },
}

impl GrantConfigInput {
    /// Validate grant configuration
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.type_url.is_empty() {
            return Err(ValidationError::MissingField("type_url"));
        }
        if self.description.is_empty() {
            return Err(ValidationError::MissingField("description"));
        }
        
        // Validate authorization-specific fields
        match &self.authorization {
            AuthorizationInput::Generic => {
                // No additional validation needed
            }
            AuthorizationInput::Send { spend_limit, .. } => {
                if spend_limit.is_empty() {
                    return Err(ValidationError::MissingField("spend_limit"));
                }
                parse_coin_string(spend_limit)?;
            }
            AuthorizationInput::Stake { max_tokens, .. } => {
                if max_tokens.is_empty() {
                    return Err(ValidationError::MissingField("max_tokens"));
                }
                parse_coin_string(max_tokens)?;
            }
            AuthorizationInput::IbcTransfer { allocations } => {
                if allocations.is_empty() {
                    return Err(ValidationError::MissingField("allocations"));
                }
                for alloc in allocations {
                    if alloc.source_channel.is_empty() {
                        return Err(ValidationError::MissingField("source_channel"));
                    }
                    if !alloc.source_channel.starts_with("channel-") {
                        return Err(ValidationError::InvalidValue(
                            "source_channel must be in format 'channel-N'"
                        ));
                    }
                    parse_coin_string(&alloc.spend_limit)?;
                }
            }
            AuthorizationInput::ContractExecution { contracts } => {
                if contracts.is_empty() {
                    return Err(ValidationError::MissingField("contracts"));
                }
                for contract in contracts {
                    if contract.address.is_empty() {
                        return Err(ValidationError::MissingField("contract address"));
                    }
                    if contract.max_calls.is_none() && contract.max_funds.is_none() {
                        return Err(ValidationError::InvalidValue(
                            "contract must have max_calls or max_funds or both"
                        ));
                    }
                    if let Some(ref funds) = contract.max_funds {
                        parse_coin_string(funds)?;
                    }
                }
            }
        }
        
        Ok(())
    }
}

// ============================================================================
// Output Types (for Treasury instantiation message)
// ============================================================================

/// Treasury instantiation message (sent to chain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryInstantiateMessage {
    pub type_urls: Vec<String>,
    pub grant_configs: Vec<GrantConfigEncoded>,
    pub fee_config: FeeConfigEncoded,
    pub admin: String,
    pub params: TreasuryParamsChain,
}

/// Encoded fee configuration (ready for chain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeConfigEncoded {
    pub description: String,
    pub allowance: ProtobufAny,
}

/// Encoded grant configuration (ready for chain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConfigEncoded {
    pub description: String,
    pub authorization: ProtobufAny,
    #[serde(default)]
    pub optional: bool,
}

/// Protobuf Any type (type_url + base64 value)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufAny {
    #[serde(rename = "type_url")]
    pub type_url: String,
    pub value: String, // Base64-encoded protobuf
}

// ============================================================================
// Helper Types
// ============================================================================

/// Parsed coin (amount + denom)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    pub amount: String,
    pub denom: String,
}

/// Validation error
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
    
    #[error("Invalid value: {0}")]
    InvalidValue(&'static str),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(&'static str),
    
    #[error("Invalid coin format: {0}")]
    InvalidCoinFormat(String),
}

/// Parse coin string (e.g., "1000000uxion" or "1000000uxion,5000000uatom")
pub fn parse_coin_string(input: &str) -> Result<Vec<Coin>, ValidationError> {
    if input.is_empty() {
        return Err(ValidationError::InvalidCoinFormat("empty input".to_string()));
    }
    
    let mut coins = Vec::new();
    
    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        // Normalize: remove spaces between digits and denom
        let normalized = part.replace(
            regex::Regex::new(r"(\d+)\s+([a-zA-Z0-9/-]+)").unwrap(),
            "$1$2"
        );
        
        // Match: digits followed by denom (allowing hyphens, slashes, alphanumeric)
        let re = regex::Regex::new(r"^(\d+)([-a-zA-Z0-9/]+)$").unwrap();
        
        if let Some(caps) = re.captures(&normalized) {
            coins.push(Coin {
                amount: caps[1].to_string(),
                denom: caps[2].to_string(),
            });
        } else {
            return Err(ValidationError::InvalidCoinFormat(part.to_string()));
        }
    }
    
    // Sort by denom alphabetically
    coins.sort_by(|a, b| a.denom.cmp(&b.denom));
    
    Ok(coins)
}

/// Parse single denom coin string (e.g., "1000000uxion")
pub fn parse_single_denom(input: &str) -> Result<Coin, ValidationError> {
    let coins = parse_coin_string(input)?;
    if coins.len() != 1 {
        return Err(ValidationError::InvalidCoinFormat(
            "expected single denomination".to_string()
        ));
    }
    Ok(coins.into_iter().next().unwrap())
}
```

### 2.2 Builder Pattern Support

```rust
// src/treasury/builder.rs

use super::types::*;

/// Builder for treasury creation request
pub struct TreasuryCreateBuilder {
    params: Option<TreasuryParamsInput>,
    fee_config: Option<FeeConfigInput>,
    grant_configs: Vec<GrantConfigInput>,
}

impl TreasuryCreateBuilder {
    pub fn new() -> Self {
        Self {
            params: None,
            fee_config: None,
            grant_configs: Vec::new(),
        }
    }
    
    pub fn params(mut self, params: TreasuryParamsInput) -> Self {
        self.params = Some(params);
        self
    }
    
    pub fn redirect_url(mut self, url: impl Into<String>) -> Self {
        let params = self.params.get_or_insert(TreasuryParamsInput {
            redirect_url: String::new(),
            icon_url: String::new(),
            name: String::new(),
            is_oauth2_app: false,
        });
        params.redirect_url = url.into();
        self
    }
    
    pub fn icon_url(mut self, url: impl Into<String>) -> Self {
        let params = self.params.get_or_insert(TreasuryParamsInput {
            redirect_url: String::new(),
            icon_url: String::new(),
            name: String::new(),
            is_oauth2_app: false,
        });
        params.icon_url = url.into();
        self
    }
    
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let params = self.params.get_or_insert(TreasuryParamsInput {
            redirect_url: String::new(),
            icon_url: String::new(),
            name: String::new(),
            is_oauth2_app: false,
        });
        params.name = name.into();
        self
    }
    
    pub fn is_oauth2_app(mut self, is_oauth2: bool) -> Self {
        let params = self.params.get_or_insert(TreasuryParamsInput {
            redirect_url: String::new(),
            icon_url: String::new(),
            name: String::new(),
            is_oauth2_app: false,
        });
        params.is_oauth2_app = is_oauth2;
        self
    }
    
    pub fn fee_config(mut self, config: FeeConfigInput) -> Self {
        self.fee_config = Some(config);
        self
    }
    
    pub fn basic_fee(mut self, description: impl Into<String>, spend_limit: impl Into<String>) -> Self {
        self.fee_config = Some(FeeConfigInput::Basic(BasicAllowanceInput {
            description: description.into(),
            spend_limit: spend_limit.into(),
        }));
        self
    }
    
    pub fn periodic_fee(
        mut self,
        description: impl Into<String>,
        spend_limit: Option<String>,
        period_seconds: u64,
        period_spend_limit: impl Into<String>,
    ) -> Self {
        self.fee_config = Some(FeeConfigInput::Periodic(PeriodicAllowanceInput {
            description: description.into(),
            spend_limit,
            period_seconds,
            period_spend_limit: period_spend_limit.into(),
        }));
        self
    }
    
    pub fn add_grant(mut self, config: GrantConfigInput) -> Self {
        self.grant_configs.push(config);
        self
    }
    
    pub fn add_generic_grant(
        mut self,
        type_url: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.grant_configs.push(GrantConfigInput {
            type_url: type_url.into(),
            description: description.into(),
            authorization: AuthorizationInput::Generic,
            optional: false,
        });
        self
    }
    
    pub fn add_send_grant(
        mut self,
        type_url: impl Into<String>,
        description: impl Into<String>,
        spend_limit: impl Into<String>,
        allow_list: Vec<String>,
    ) -> Self {
        self.grant_configs.push(GrantConfigInput {
            type_url: type_url.into(),
            description: description.into(),
            authorization: AuthorizationInput::Send {
                spend_limit: spend_limit.into(),
                allow_list,
            },
            optional: false,
        });
        self
    }
    
    pub fn build(self) -> Result<TreasuryCreateRequest, ValidationError> {
        let params = self.params.ok_or(ValidationError::MissingField("params"))?;
        let fee_config = self.fee_config.ok_or(ValidationError::MissingField("fee_config"))?;
        
        if self.grant_configs.is_empty() {
            return Err(ValidationError::MissingField("grant_configs"));
        }
        
        // Validate all components
        params.validate()?;
        fee_config.validate()?;
        for grant in &self.grant_configs {
            grant.validate()?;
        }
        
        Ok(TreasuryCreateRequest {
            params,
            fee_config,
            grant_configs: self.grant_configs,
        })
    }
}

impl Default for TreasuryCreateBuilder {
    fn default() -> Self {
        Self::new()
    }
}
```

## 3. Encoding Module Design

### 3.1 Module Structure

```
src/treasury/
├── mod.rs
├── types.rs              # (existing, enhanced)
├── builder.rs            # New: Builder pattern
├── encoding.rs           # New: Protobuf encoding
│   ├── mod.rs
│   ├── fee.rs            # Fee allowance encoding
│   ├── grant.rs          # Grant authorization encoding
│   ├── proto.rs          # Protobuf definitions
│   └── utils.rs          # Helper utilities
└── manager.rs            # (existing, enhanced)
```

### 3.2 Encoding Module Interface

```rust
// src/treasury/encoding/mod.rs

mod fee;
mod grant;
mod proto;
mod utils;

pub use fee::*;
pub use grant::*;
pub use utils::*;
pub use proto::*;

use super::types::*;
use thiserror::Error;

/// Encoding error
#[derive(Debug, Error)]
pub enum EncodingError {
    #[error("Protobuf encoding failed: {0}")]
    ProtobufEncode(#[from] prost::EncodeError),
    
    #[error("Invalid coin format: {0}")]
    InvalidCoin(String),
    
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
    
    #[error("Invalid allowance type: {0}")]
    InvalidAllowanceType(String),
    
    #[error("Invalid authorization type: {0}")]
    InvalidAuthorizationType(String),
    
    #[error("Base64 encoding failed: {0}")]
    Base64(#[from] base64::EncodeError),
}

/// Encode treasury creation request to chain-ready format
pub fn encode_treasury_create(
    request: TreasuryCreateRequest,
    admin: String,
) -> Result<TreasuryInstantiateMessage, EncodingError> {
    // Encode fee config
    let fee_config = encode_fee_config(&request.fee_config)?;
    
    // Encode grant configs
    let mut type_urls = Vec::new();
    let mut grant_configs_encoded = Vec::new();
    
    for grant in &request.grant_configs {
        type_urls.push(grant.type_url.clone());
        grant_configs_encoded.push(encode_grant_config(grant)?);
    }
    
    // Convert params to chain format
    let params = request.params.to_chain_format();
    
    Ok(TreasuryInstantiateMessage {
        type_urls,
        grant_configs: grant_configs_encoded,
        fee_config,
        admin,
        params,
    })
}

/// Encode fee configuration
pub fn encode_fee_config(input: &FeeConfigInput) -> Result<FeeConfigEncoded, EncodingError> {
    let (type_url, value) = match input {
        FeeConfigInput::Basic(basic) => {
            encode_basic_allowance(&basic.spend_limit)?
        }
        FeeConfigInput::Periodic(periodic) => {
            encode_periodic_allowance(
                periodic.spend_limit.as_deref(),
                periodic.period_seconds,
                &periodic.period_spend_limit,
            )?
        }
        FeeConfigInput::AllowedMsg(allowed) => {
            encode_allowed_msg_allowance(
                &allowed.allowed_messages,
                &allowed.sub_allowance,
            )?
        }
    };
    
    Ok(FeeConfigEncoded {
        description: input.description().to_string(),
        allowance: ProtobufAny { type_url, value },
    })
}

/// Encode grant configuration
pub fn encode_grant_config(input: &GrantConfigInput) -> Result<GrantConfigEncoded, EncodingError> {
    let (type_url, value) = match &input.authorization {
        AuthorizationInput::Generic => {
            encode_generic_authorization(&input.type_url)?
        }
        AuthorizationInput::Send { spend_limit, allow_list } => {
            encode_send_authorization(spend_limit, allow_list)?
        }
        AuthorizationInput::Stake {
            max_tokens,
            validators,
            deny_validators,
            authorization_type,
        } => {
            encode_stake_authorization(
                max_tokens,
                validators,
                deny_validators,
                *authorization_type,
            )?
        }
        AuthorizationInput::IbcTransfer { allocations } => {
            encode_ibc_transfer_authorization(allocations)?
        }
        AuthorizationInput::ContractExecution { contracts } => {
            encode_contract_execution_authorization(contracts)?
        }
    };
    
    Ok(GrantConfigEncoded {
        description: input.description.clone(),
        authorization: ProtobufAny { type_url, value },
        optional: input.optional,
    })
}
```

### 3.3 Fee Allowance Encoding

```rust
// src/treasury/encoding/fee.rs

use super::*;
use prost::Message;

/// Encode BasicAllowance
pub fn encode_basic_allowance(
    spend_limit: &str,
) -> Result<(String, String), EncodingError> {
    let coins = parse_coin_string(spend_limit)
        .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
    
    let allowance = proto::BasicAllowance {
        spend_limit: coins.into_iter().map(|c| proto::Coin::from(c)).collect(),
        expiration: None,
    };
    
    let mut buf = Vec::new();
    allowance.encode(&mut buf)?;
    
    Ok((
        "/cosmos.feegrant.v1beta1.BasicAllowance".to_string(),
        base64::encode(&buf),
    ))
}

/// Encode PeriodicAllowance
pub fn encode_periodic_allowance(
    basic_spend_limit: Option<&str>,
    period_seconds: u64,
    period_spend_limit: &str,
) -> Result<(String, String), EncodingError> {
    let basic = if let Some(limit) = basic_spend_limit {
        let coins = parse_coin_string(limit)
            .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
        Some(proto::BasicAllowance {
            spend_limit: coins.into_iter().map(|c| proto::Coin::from(c)).collect(),
            expiration: None,
        })
    } else {
        None
    };
    
    let period_coins = parse_coin_string(period_spend_limit)
        .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
    
    let allowance = proto::PeriodicAllowance {
        basic,
        period: Some(proto::Duration { seconds: period_seconds as i64, nanos: 0 }),
        period_spend_limit: period_coins.into_iter().map(|c| proto::Coin::from(c)).collect(),
        period_can_roll_over: false,
        remaining_spend_limit: vec![],
        last_period_reset: None,
    };
    
    let mut buf = Vec::new();
    allowance.encode(&mut buf)?;
    
    Ok((
        "/cosmos.feegrant.v1beta1.PeriodicAllowance".to_string(),
        base64::encode(&buf),
    ))
}

/// Encode AllowedMsgAllowance
pub fn encode_allowed_msg_allowance(
    allowed_messages: &[String],
    sub_allowance: &SubAllowanceInput,
) -> Result<(String, String), EncodingError> {
    // Encode nested allowance first
    let (nested_type_url, nested_value) = match sub_allowance {
        SubAllowanceInput::Basic { spend_limit } => {
            encode_basic_allowance(spend_limit)?
        }
        SubAllowanceInput::Periodic { spend_limit, period_seconds, period_spend_limit } => {
            encode_periodic_allowance(spend_limit.as_deref(), *period_seconds, period_spend_limit)?
        }
    };
    
    // Decode nested value from base64
    let nested_bytes = base64::decode(&nested_value)?;
    
    let allowance = proto::AllowedMsgAllowance {
        allowance: Some(proto::Any {
            type_url: nested_type_url,
            value: nested_bytes,
        }),
        allowed_messages: allowed_messages.to_vec(),
    };
    
    let mut buf = Vec::new();
    allowance.encode(&mut buf)?;
    
    Ok((
        "/cosmos.feegrant.v1beta1.AllowedMsgAllowance".to_string(),
        base64::encode(&buf),
    ))
}
```

### 3.4 Grant Authorization Encoding

```rust
// src/treasury/encoding/grant.rs

use super::*;

/// Encode GenericAuthorization
pub fn encode_generic_authorization(
    msg_type_url: &str,
) -> Result<(String, String), EncodingError> {
    let auth = proto::GenericAuthorization {
        msg: msg_type_url.to_string(),
    };
    
    let mut buf = Vec::new();
    prost::Message::encode(&auth, &mut buf)?;
    
    Ok((
        "/cosmos.authz.v1beta1.GenericAuthorization".to_string(),
        base64::encode(&buf),
    ))
}

/// Encode SendAuthorization
pub fn encode_send_authorization(
    spend_limit: &str,
    allow_list: &[String],
) -> Result<(String, String), EncodingError> {
    let coins = parse_coin_string(spend_limit)
        .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
    
    let auth = proto::SendAuthorization {
        spend_limit: coins.into_iter().map(|c| proto::Coin::from(c)).collect(),
        allow_list: allow_list.to_vec(),
    };
    
    let mut buf = Vec::new();
    prost::Message::encode(&auth, &mut buf)?;
    
    Ok((
        "/cosmos.bank.v1beta1.SendAuthorization".to_string(),
        base64::encode(&buf),
    ))
}

/// Encode StakeAuthorization
pub fn encode_stake_authorization(
    max_tokens: &str,
    validators: &[String],
    deny_validators: &[String],
    authorization_type: StakeAuthorizationType,
) -> Result<(String, String), EncodingError> {
    let coin = parse_single_denom(max_tokens)
        .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
    
    let auth_type = match authorization_type {
        StakeAuthorizationType::Delegate => 1,
        StakeAuthorizationType::Undelegate => 2,
        StakeAuthorizationType::Redelegate => 3,
    };
    
    let auth = proto::StakeAuthorization {
        max_tokens: Some(proto::Coin::from(coin)),
        allow_list: if !validators.is_empty() {
            Some(proto::StakeAuthorizationValidators {
                address: validators.to_vec(),
            })
        } else {
            None
        },
        deny_list: if !deny_validators.is_empty() {
            Some(proto::StakeAuthorizationValidators {
                address: deny_validators.to_vec(),
            })
        } else {
            None
        },
        authorization_type: auth_type,
    };
    
    let mut buf = Vec::new();
    prost::Message::encode(&auth, &mut buf)?;
    
    Ok((
        "/cosmos.staking.v1beta1.StakeAuthorization".to_string(),
        base64::encode(&buf),
    ))
}

/// Encode TransferAuthorization (IBC)
pub fn encode_ibc_transfer_authorization(
    allocations: &[IbcAllocationInput],
) -> Result<(String, String), EncodingError> {
    let proto_allocations: Result<Vec<_>, _> = allocations
        .iter()
        .map(|alloc| {
            let coins = parse_coin_string(&alloc.spend_limit)
                .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
            
            Ok(proto::TransferAuthorizationAllocation {
                source_port: alloc.source_port.clone(),
                source_channel: alloc.source_channel.clone(),
                spend_limit: coins.into_iter().map(|c| proto::Coin::from(c)).collect(),
                allow_list: alloc.allow_list.clone(),
            })
        })
        .collect();
    
    let auth = proto::TransferAuthorization {
        allocations: proto_allocations?,
    };
    
    let mut buf = Vec::new();
    prost::Message::encode(&auth, &mut buf)?;
    
    Ok((
        "/ibc.applications.transfer.v1.TransferAuthorization".to_string(),
        base64::encode(&buf),
    ))
}

/// Encode ContractExecutionAuthorization
pub fn encode_contract_execution_authorization(
    contracts: &[ContractGrantInput],
) -> Result<(String, String), EncodingError> {
    let grants: Result<Vec<_>, _> = contracts
        .iter()
        .map(|contract| {
            // Build limit
            let limit = match (&contract.max_calls, &contract.max_funds) {
                (Some(calls), Some(funds)) => {
                    let coins = parse_coin_string(funds)
                        .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
                    
                    let combined = proto::CombinedLimit {
                        calls_remaining: *calls,
                        amounts: coins.into_iter().map(|c| proto::Coin::from(c)).collect(),
                    };
                    let mut buf = Vec::new();
                    prost::Message::encode(&combined, &mut buf)?;
                    
                    proto::Any {
                        type_url: "/cosmwasm.wasm.v1.CombinedLimit".to_string(),
                        value: buf,
                    }
                }
                (Some(calls), None) => {
                    let max_calls = proto::MaxCallsLimit {
                        remaining: *calls,
                    };
                    let mut buf = Vec::new();
                    prost::Message::encode(&max_calls, &mut buf)?;
                    
                    proto::Any {
                        type_url: "/cosmwasm.wasm.v1.MaxCallsLimit".to_string(),
                        value: buf,
                    }
                }
                (None, Some(funds)) => {
                    let coins = parse_coin_string(funds)
                        .map_err(|e| EncodingError::InvalidCoin(e.to_string()))?;
                    
                    let max_funds = proto::MaxFundsLimit {
                        amounts: coins.into_iter().map(|c| proto::Coin::from(c)).collect(),
                    };
                    let mut buf = Vec::new();
                    prost::Message::encode(&max_funds, &mut buf)?;
                    
                    proto::Any {
                        type_url: "/cosmwasm.wasm.v1.MaxFundsLimit".to_string(),
                        value: buf,
                    }
                }
                (None, None) => {
                    return Err(EncodingError::MissingField("max_calls or max_funds"));
                }
            };
            
            // Build filter
            let filter = match &contract.filter {
                ContractFilterInput::AllowAll => proto::Any {
                    type_url: "/cosmwasm.wasm.v1.AllowAllMessagesFilter".to_string(),
                    value: vec![],
                },
                ContractFilterInput::AcceptedKeys { keys } => {
                    let accepted_keys = proto::AcceptedMessageKeysFilter {
                        keys: keys.clone(),
                    };
                    let mut buf = Vec::new();
                    prost::Message::encode(&accepted_keys, &mut buf)?;
                    
                    proto::Any {
                        type_url: "/cosmwasm.wasm.v1.AcceptedMessageKeysFilter".to_string(),
                        value: buf,
                    }
                }
            };
            
            Ok(proto::ContractGrant {
                contract: contract.address.clone(),
                limit: Some(limit),
                filter: Some(filter),
            })
        })
        .collect();
    
    let auth = proto::ContractExecutionAuthorization {
        grants: grants?,
    };
    
    let mut buf = Vec::new();
    prost::Message::encode(&auth, &mut buf)?;
    
    Ok((
        "/cosmwasm.wasm.v1.ContractExecutionAuthorization".to_string(),
        base64::encode(&buf),
    ))
}
```

### 3.5 Protobuf Definitions

```rust
// src/treasury/encoding/proto.rs

use prost::Message;

// ============================================================================
// Cosmos SDK Protobuf Types
// ============================================================================

#[derive(Clone, PartialEq, Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub amount: String,
    #[prost(string, tag = "2")]
    pub denom: String,
}

impl From<super::Coin> for Coin {
    fn from(coin: super::Coin) -> Self {
        Self {
            amount: coin.amount,
            denom: coin.denom,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Duration {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct Any {
    #[prost(string, tag = "1")]
    pub type_url: String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: Vec<u8>,
}

// ============================================================================
// Fee Grant Types
// ============================================================================

#[derive(Clone, PartialEq, Message)]
pub struct BasicAllowance {
    #[prost(message, repeated, tag = "1")]
    pub spend_limit: Vec<Coin>,
    #[prost(message, optional, tag = "2")]
    pub expiration: Option<Timestamp>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PeriodicAllowance {
    #[prost(message, optional, tag = "1")]
    pub basic: Option<BasicAllowance>,
    #[prost(message, optional, tag = "2")]
    pub period: Option<Duration>,
    #[prost(message, repeated, tag = "3")]
    pub period_spend_limit: Vec<Coin>,
    #[prost(bool, tag = "4")]
    pub period_can_roll_over: bool,
    #[prost(message, repeated, tag = "5")]
    pub remaining_spend_limit: Vec<Coin>,
    #[prost(message, optional, tag = "6")]
    pub last_period_reset: Option<Timestamp>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AllowedMsgAllowance {
    #[prost(message, optional, tag = "1")]
    pub allowance: Option<Any>,
    #[prost(string, repeated, tag = "2")]
    pub allowed_messages: Vec<String>,
}

// ============================================================================
// Authorization Types
// ============================================================================

#[derive(Clone, PartialEq, Message)]
pub struct GenericAuthorization {
    #[prost(string, tag = "1")]
    pub msg: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct SendAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub spend_limit: Vec<Coin>,
    #[prost(string, repeated, tag = "2")]
    pub allow_list: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct StakeAuthorization {
    #[prost(message, optional, tag = "1")]
    pub max_tokens: Option<Coin>,
    #[prost(message, optional, tag = "2")]
    pub allow_list: Option<StakeAuthorizationValidators>,
    #[prost(message, optional, tag = "3")]
    pub deny_list: Option<StakeAuthorizationValidators>,
    #[prost(int32, tag = "4")]
    pub authorization_type: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct StakeAuthorizationValidators {
    #[prost(string, repeated, tag = "1")]
    pub address: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TransferAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub allocations: Vec<TransferAuthorizationAllocation>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TransferAuthorizationAllocation {
    #[prost(string, tag = "1")]
    pub source_port: String,
    #[prost(string, tag = "2")]
    pub source_channel: String,
    #[prost(message, repeated, tag = "3")]
    pub spend_limit: Vec<Coin>,
    #[prost(string, repeated, tag = "4")]
    pub allow_list: Vec<String>,
}

// ============================================================================
// Contract Execution Authorization Types
// ============================================================================

#[derive(Clone, PartialEq, Message)]
pub struct ContractExecutionAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub grants: Vec<ContractGrant>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ContractGrant {
    #[prost(string, tag = "1")]
    pub contract: String,
    #[prost(message, optional, tag = "2")]
    pub limit: Option<Any>,
    #[prost(message, optional, tag = "3")]
    pub filter: Option<Any>,
}

#[derive(Clone, PartialEq, Message)]
pub struct MaxCallsLimit {
    #[prost(uint64, tag = "1")]
    pub remaining: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct MaxFundsLimit {
    #[prost(message, repeated, tag = "1")]
    pub amounts: Vec<Coin>,
}

#[derive(Clone, PartialEq, Message)]
pub struct CombinedLimit {
    #[prost(uint64, tag = "1")]
    pub calls_remaining: u64,
    #[prost(message, repeated, tag = "2")]
    pub amounts: Vec<Coin>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AllowAllMessagesFilter {}

#[derive(Clone, PartialEq, Message)]
pub struct AcceptedMessageKeysFilter {
    #[prost(string, repeated, tag = "1")]
    pub keys: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AcceptedMessagesFilter {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub messages: Vec<Vec<u8>>,
}

// Timestamp placeholder
#[derive(Clone, PartialEq, Message)]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
```

### 3.6 Utility Functions

```rust
// src/treasury/encoding/utils.rs

use super::*;

/// Validate that a type URL is well-formed
pub fn validate_type_url(type_url: &str) -> Result<(), EncodingError> {
    if !type_url.starts_with('/') {
        return Err(EncodingError::InvalidAuthorizationType(
            "type URL must start with '/'".to_string()
        ));
    }
    Ok(())
}

/// Check if authorization type is allowed for a given message type
pub fn is_valid_auth_type_for_message(
    message_type_url: &str,
    auth_type: &str,
) -> bool {
    match message_type_url {
        "/cosmos.bank.v1beta1.MsgSend" => {
            matches!(auth_type, "/cosmos.bank.v1beta1.SendAuthorization")
        }
        "/ibc.applications.transfer.v1.MsgTransfer" => {
            matches!(auth_type, "/ibc.applications.transfer.v1.TransferAuthorization")
        }
        "/cosmos.staking.v1beta1.MsgDelegate" 
        | "/cosmos.staking.v1beta1.MsgUndelegate"
        | "/cosmos.staking.v1beta1.MsgBeginRedelegate" => {
            matches!(
                auth_type,
                "/cosmos.authz.v1beta1.GenericAuthorization" 
                    | "/cosmos.staking.v1beta1.StakeAuthorization"
            )
        }
        "/cosmwasm.wasm.v1.MsgExecuteContract" => {
            matches!(
                auth_type,
                "/cosmos.authz.v1beta1.GenericAuthorization"
                    | "/cosmwasm.wasm.v1.ContractExecutionAuthorization"
            )
        }
        "/osmosis.tokenfactory.v1beta1.MsgMint"
        | "/osmosis.tokenfactory.v1beta1.MsgBurn" => {
            matches!(auth_type, "/cosmos.authz.v1beta1.GenericAuthorization")
        }
        _ => {
            // Default: only GenericAuthorization allowed
            matches!(auth_type, "/cosmos.authz.v1beta1.GenericAuthorization")
        }
    }
}

/// Get display name for type URL
pub fn type_url_to_display_name(type_url: &str) -> &str {
    match type_url {
        "/cosmos.bank.v1beta1.MsgSend" => "Send Funds",
        "/cosmos.staking.v1beta1.MsgDelegate" => "Delegate Tokens",
        "/cosmos.staking.v1beta1.MsgUndelegate" => "Undelegate Tokens",
        "/cosmos.staking.v1beta1.MsgBeginRedelegate" => "Redelegate Tokens",
        "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" => "Withdraw Delegation Rewards",
        "/cosmos.gov.v1beta1.MsgVote" => "Vote on Proposal",
        "/ibc.applications.transfer.v1.MsgTransfer" => "IBC Transfer",
        "/cosmos.authz.v1beta1.MsgExec" => "Execute Authorization",
        "/cosmos.authz.v1beta1.MsgRevoke" => "Revoke Authorization",
        "/cosmos.crisis.v1beta1.MsgVerifyInvariant" => "Verify Invariant",
        "/cosmos.evidence.v1beta1.MsgSubmitEvidence" => "Submit Evidence",
        "/cosmos.feegrant.v1beta1.MsgGrantAllowance" => "Grant Fee Allowance",
        "/cosmos.feegrant.v1beta1.MsgRevokeAllowance" => "Revoke Fee Allowance",
        "/cosmos.gov.v1beta1.MsgDeposit" => "Deposit to Proposal",
        "/cosmos.gov.v1beta1.MsgSubmitProposal" => "Submit Governance Proposal",
        "/cosmos.slashing.v1beta1.MsgUnjail" => "Unjail Validator",
        "/cosmos.vesting.v1beta1.MsgCreateVestingAccount" => "Create Vesting Account",
        "/osmosis.tokenfactory.v1beta1.MsgMint" => "Mint Token",
        "/osmosis.tokenfactory.v1beta1.MsgBurn" => "Burn Token",
        "/cosmwasm.wasm.v1.MsgInstantiateContract" => "Instantiate a smart contract",
        "/cosmwasm.wasm.v1.MsgInstantiateContract2" => "Instantiate a smart contract (instantiate2)",
        "/cosmwasm.wasm.v1.MsgExecuteContract" => "Execute on a smart contract",
        _ => type_url,
    }
}
```

## 4. Implementation Notes

### 4.1 Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...

# Protobuf encoding (already in Cargo.toml)
prost = "0.12"

# Regex for coin parsing
regex = "1.10"

# Add base64 if not already present (should be there for PKCE)
# base64 = "0.22"  # Already present
```

### 4.2 CLI Implementation Strategy

1. **Phase 1: Config file support** (easiest to implement first)
   - Implement JSON config file parsing
   - Implement encoding module
   - Test with complex scenarios

2. **Phase 2: Flag-based support** (incremental enhancement)
   - Start with basic flags
   - Add fee grant flags
   - Add grant configuration flags
   - Handle repeatable flags for multiple grants

3. **Phase 3: Validation and UX**
   - Add comprehensive validation
   - Add helpful error messages
   - Add suggestions and examples

### 4.3 Potential Challenges

1. **Protobuf Compatibility**
   - Must match exact protobuf definitions from cosmos-sdk and cosmwasm
   - Type URLs must be exact
   - Field ordering must match proto definitions

2. **Coin Parsing Edge Cases**
   - IBC denoms with slashes (e.g., "ibc/xxx/yyy")
   - Multiple coins in one string
   - Spaces between amount and denom

3. **Complex Nested Structures**
   - AllowedMsgAllowance with nested allowance
   - ContractExecutionAuthorization with multiple contracts
   - IBC transfer with multiple allocations

4. **Validation Complexity**
   - Cross-field validation (e.g., allowed auth types for message types)
   - IBC channel format validation
   - Contract address validation

### 4.4 Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_coin_string() {
        assert_eq!(
            parse_coin_string("1000000uxion").unwrap(),
            vec![Coin { amount: "1000000".to_string(), denom: "uxion".to_string() }]
        );
        
        assert_eq!(
            parse_coin_string("1000000uxion,5000000uatom").unwrap(),
            vec![
                Coin { amount: "5000000".to_string(), denom: "uatom".to_string() },
                Coin { amount: "1000000".to_string(), denom: "uxion".to_string() },
            ]
        );
    }
    
    #[test]
    fn test_encode_basic_allowance() {
        let (type_url, value) = encode_basic_allowance("1000000uxion").unwrap();
        assert_eq!(type_url, "/cosmos.feegrant.v1beta1.BasicAllowance");
        assert!(!value.is_empty());
    }
    
    #[test]
    fn test_treasury_builder() {
        let request = TreasuryCreateBuilder::new()
            .redirect_url("https://example.com/callback")
            .icon_url("https://example.com/icon.png")
            .name("Test Treasury")
            .basic_fee("Test fee", "1000000uxion")
            .add_generic_grant(
                "/cosmos.bank.v1beta1.MsgSend",
                "Allow sending",
            )
            .build()
            .unwrap();
        
        assert_eq!(request.params.redirect_url, "https://example.com/callback");
    }
    
    #[test]
    fn test_json_config_parsing() {
        let json = r#"{
            "params": {
                "redirect_url": "https://example.com/callback",
                "icon_url": "https://example.com/icon.png"
            },
            "fee_config": {
                "allowance_type": "basic",
                "description": "Test",
                "spend_limit": "1000000uxion"
            },
            "grant_configs": [{
                "type_url": "/cosmos.bank.v1beta1.MsgSend",
                "description": "Send",
                "authorization": { "type": "generic" }
            }]
        }"#;
        
        let request: TreasuryCreateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.params.redirect_url, "https://example.com/callback");
    }
    
    #[test]
    fn test_full_encoding_flow() {
        let request = TreasuryCreateRequest {
            params: TreasuryParamsInput {
                redirect_url: "https://example.com/callback".to_string(),
                icon_url: "https://example.com/icon.png".to_string(),
                name: "Test".to_string(),
                is_oauth2_app: false,
            },
            fee_config: FeeConfigInput::Basic(BasicAllowanceInput {
                description: "Test fee".to_string(),
                spend_limit: "1000000uxion".to_string(),
            }),
            grant_configs: vec![GrantConfigInput {
                type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
                description: "Send funds".to_string(),
                authorization: AuthorizationInput::Send {
                    spend_limit: "1000000uxion".to_string(),
                    allow_list: vec![],
                },
                optional: false,
            }],
        };
        
        let encoded = encode_treasury_create(request, "xion1admin...".to_string()).unwrap();
        
        assert_eq!(encoded.type_urls.len(), 1);
        assert_eq!(encoded.grant_configs.len(), 1);
        assert!(!encoded.fee_config.allowance.value.is_empty());
    }
}
```

### 4.5 Integration with Existing Code

Update `src/cli/treasury.rs`:

```rust
// In handle_create function
async fn handle_create(
    config_path: Option<&str>,
    // ... flag parameters
) -> Result<()> {
    // 1. Load request from config file or build from flags
    let request = if let Some(path) = config_path {
        load_config_from_file(path)?
    } else {
        build_request_from_flags(/* ... */)?
    };
    
    // 2. Validate request
    request.validate()?;
    
    // 3. Get admin address
    let admin = get_admin_address().await?;
    
    // 4. Encode to chain format
    let message = encode_treasury_create(request, admin)?;
    
    // 5. Broadcast transaction
    let result = broadcast_treasury_create(message).await?;
    
    // 6. Output result
    print_json(&result)
}
```

## 5. Compatibility with Developer Portal

The design is fully compatible with Developer Portal format:

1. **Same type URLs** - Uses exact protobuf type URLs
2. **Same encoding** - Base64-encoded protobuf
3. **Same structure** - Matches `TreasuryInstantiateMessage` format
4. **Same validation rules** - Compatible authorization type restrictions

Import/Export flow:
- Config files generated by CLI can be imported to Dev Portal
- Config files exported from Dev Portal can be used by CLI
- Encoded values are identical between both tools

## 6. Future Enhancements

1. **Interactive Mode**: Wizard-style CLI for easier configuration
2. **Template Library**: Pre-built configurations for common use cases
3. **Import from Dev Portal**: Fetch existing treasury configs
4. **Upgrade Support**: Migrate existing treasuries to new configurations
5. **Simulation**: Dry-run to validate configuration before deployment

---

## Summary

This design provides:
- ✅ **Flexible CLI interface** - Flags for simple cases, config files for complex ones
- ✅ **Comprehensive data structures** - Type-safe Rust structs with validation
- ✅ **Robust encoding** - Prost-based protobuf encoding matching Dev Portal
- ✅ **Developer ergonomics** - Builder pattern, helpful errors, examples
- ✅ **Full compatibility** - Interchangeable with Developer Portal format

The implementation can proceed incrementally, starting with config file support and encoding, then adding flag-based CLI support.
