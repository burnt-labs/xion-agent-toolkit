---
status: InProgress
created_at: 2026-03-06
updated_at: 2026-03-06
---

# Treasury Create Command Enhancement

## Background

The current Treasury create command is a placeholder with minimal functionality. To match the Developer Portal capabilities, we need to implement complete configuration options including params settings, fee grant configuration, and permissions/grant configuration.

## Goal

Implement a fully-featured Treasury create command that supports:
1. **Complete params settings** - redirect_url, icon_url, metadata (name, archived, is_oauth2_app)
2. **Fee grant configuration** - BasicAllowance, PeriodicAllowance, AllowedMsgAllowance
3. **Permissions configuration** - Common grant/authorization configurations for Cosmos SDK messages

## Technical Approach

### Architecture Design

Reference implementation from Developer Portal:
- Location: `~/workspace/xion/xion-developer-portal/src/components/Treasury/NewTreasury/`
- Key files:
  - `NewTreasuryForm.tsx` - Main form with message structure
  - `AllowanceEncoder.tsx` - Fee grant encoding
  - `GrantConfigEncoder.tsx` - Permission encoding
  - `src/core/encoding.ts` - Core encoding functions

### Data Structures

#### Treasury Params
```rust
pub struct TreasuryParams {
    pub redirect_url: String,
    pub icon_url: String,
    pub metadata: TreasuryMetadata,
}

pub struct TreasuryMetadata {
    pub name: String,
    pub archived: bool,
    pub is_oauth2_app: bool,
}
```

#### Fee Grant Types
```rust
pub enum FeeAllowanceType {
    Basic,
    Periodic,
    AllowedMsg,
}

pub struct FeeConfig {
    pub description: String,
    pub allowance: ProtobufAny,
}

pub struct BasicAllowanceData {
    pub spend_limit: Vec<Coin>,
}

pub struct PeriodicAllowanceData {
    pub basic: Option<BasicAllowanceData>,
    pub period: Duration,
    pub period_spend_limit: Vec<Coin>,
}

pub struct AllowedMsgAllowanceData {
    pub allowed_messages: Vec<String>,
    pub allowance: Box<AllowanceData>,
}
```

#### Permission/Grant Types
```rust
pub enum AuthorizationType {
    Generic,
    Send,
    Stake,
    IbcTransfer,
    ContractExecution,
}

pub struct GrantConfig {
    pub description: String,
    pub authorization: ProtobufAny,
    pub optional: bool,
}

pub struct GrantConfigWithTypeUrl {
    pub type_url: String,
    pub grant_config: GrantConfig,
}
```

### Encoding Strategy

Use `prost` crate for protobuf encoding (already in dependencies):
1. Define protobuf message structures
2. Implement `prost::Message` trait
3. Encode to bytes, then base64
4. Wrap in `ProtobufAny { type_url, value }`

### CLI Interface Design

```bash
# Basic usage
xion-toolkit treasury create \
  --redirect-url "https://example.com/callback" \
  --icon-url "https://example.com/icon.png" \
  --name "My Treasury"

# With fee grant (basic)
xion-toolkit treasury create \
  --redirect-url "..." \
  --icon-url "..." \
  --name "..." \
  --fee-allowance basic \
  --fee-spend-limit "1000000uxion" \
  --fee-description "Basic fee allowance"

# With fee grant (periodic)
xion-toolkit treasury create \
  --redirect-url "..." \
  --icon-url "..." \
  --name "..." \
  --fee-allowance periodic \
  --fee-period-seconds 86400 \
  --fee-period-spend-limit "100000uxion" \
  --fee-description "Daily fee allowance"

# With permissions
xion-toolkit treasury create \
  --redirect-url "..." \
  --icon-url "..." \
  --name "..." \
  --grant-permission "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type "send" \
  --grant-spend-limit "1000000uxion" \
  --grant-description "Allow sending funds"
```

### Alternative: Config File Approach

For complex configurations, support JSON config file:
```bash
xion-toolkit treasury create --config treasury-config.json
```

```json
{
  "params": {
    "redirect_url": "https://example.com/callback",
    "icon_url": "https://example.com/icon.png",
    "metadata": {
      "name": "My Treasury",
      "is_oauth2_app": true
    }
  },
  "fee_config": {
    "description": "Basic fee allowance",
    "allowance_type": "basic",
    "spend_limit": "1000000uxion"
  },
  "grant_configs": [
    {
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "description": "Allow sending funds",
      "authorization_type": "send",
      "spend_limit": "1000000uxion"
    }
  ]
}
```

## Tasks

### Phase 1: Architecture & Design ✅
- [x] Research Developer Portal implementation
- [x] Design CLI interface - See `.agents/plans/knowledge/treasury-create-design.md`
- [x] Define data structures - Complete with builder pattern
- [x] Plan encoding strategy - Using prost for protobuf

### Phase 2: Implementation
- [x] Create encoding module (`src/treasury/encoding.rs`)
  - [x] Implement fee allowance encoding (Basic, Periodic, AllowedMsg)
  - [x] Implement grant authorization encoding (Generic, Send, Stake, IbcTransfer, ContractExecution)
  - [x] Add coin parsing utilities
  - [x] 36 unit tests passing
- [x] Update types (`src/treasury/types.rs`)
  - [x] Add TreasuryParams and metadata
  - [x] Add fee allowance types (Input types)
  - [x] Add grant authorization types (Input types)
  - [x] Add chain types for API communication
  - [x] 11 type tests passing
- [x] Update CLI (`src/cli/treasury.rs`)
  - [x] Add params flags
  - [x] Add fee grant flags
  - [x] Add permission flags (simplified)
  - [x] Support config file input
  - [x] Validation functions
- [x] Update manager (`src/treasury/manager.rs`)
  - [x] Implement create method with full params
  - [x] Add encoding calls (via helper functions)
  - [x] Build CreateTreasuryRequest for API
  - [x] Return TreasuryInfo result

### Phase 3: Testing
- [x] Unit tests for encoding functions (36 tests passing)
- [ ] Integration tests for CLI commands (in-progress)
- [ ] Testnet deployment test
- [ ] Verify against Developer Portal compatibility

### Phase 4: Documentation
- [ ] Update CLI reference docs
- [ ] Add examples to README
- [ ] Document config file format

## Acceptance Criteria

- [ ] Can create treasury with all params options
- [ ] Can configure BasicAllowance fee grant
- [ ] Can configure PeriodicAllowance fee grant
- [ ] Can configure AllowedMsgAllowance fee grant
- [ ] Can add permissions with GenericAuthorization
- [ ] Can add permissions with SendAuthorization
- [ ] Can add permissions with StakeAuthorization
- [ ] Can add permissions with IbcTransferAuthorization
- [ ] Can add permissions with ContractExecutionAuthorization
- [ ] CLI output is JSON formatted
- [ ] Errors include actionable messages
- [ ] Testnet deployment works end-to-end
- [ ] Compatible with Developer Portal format

## Sign-off

| Date | Content | Status |
|------|---------|--------|
| 2026-03-06 | Architecture design completed | ✅ |
| 2026-03-06 | Encoding module implemented (33 tests passing) | ✅ |
| 2026-03-06 | Types and manager implementation completed | ✅ |
| 2026-03-06 | CLI commands with full flag support | ✅ |
| 2026-03-06 | Config file support (--config flag) | ✅ |
| 2026-03-06 | Polling mechanism for treasury indexing | ✅ |
| 2026-03-06 | Unit tests: 47 tests passing | ✅ |
| 2026-03-06 | Documentation updated (SKILL.md + create.sh) | ✅ |
| 2026-03-06 | Phase 3 testing: in-progress | 🔄 |
