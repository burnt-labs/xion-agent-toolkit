---
status: Done
created_at: 2026-03-06
updated_at: 2026-03-08
done_at: 2026-03-07
---
# Treasury Grant Config & Fee Config Implementation

## Background

Treasury contracts support Grant Config and Fee Config to enable gasless transactions and delegated authorization:

- **Grant Config**: Configures Authz Grants, allowing the Treasury to authorize specific message types on behalf of the admin
- **Fee Config**: Configures Fee Grants, allowing the Treasury to pay transaction fees for authorized agents

Currently, these features are placeholder in the CLI. Users must use the Developer Portal to configure grants.

## Goal

Implement full CLI support for Grant Config and Fee Config operations:
1. Add/Update/Remove grant configurations
2. Add/Update/Remove fee configurations
3. Query existing configurations
4. Update skills scripts to use CLI instead of placeholders

## Approach

### Phase 1: Grant Config Implementation

#### 1.1 CLI Commands

```bash
# Add/Update grant config
xion-toolkit treasury grant-config add --address <TREASURY> --config <CONFIG_FILE>
xion-toolkit treasury grant-config add --address <TREASURY> \
  --type-url "/cosmwasm.wasm.v1.MsgExecuteContract" \
  --auth-type contract-execution \
  --description "Allow contract execution" \
  --contract-address "xion1..." \
  --max-calls 100 \
  --max-funds "1000000uxion"

# Remove grant config
xion-toolkit treasury grant-config remove --address <TREASURY> --type-url <TYPE_URL>

# List grant configs
xion-toolkit treasury grant-config list --address <TREASURY>
```

#### 1.2 Supported Authorization Types

| Type | Type URL | Authorization | Parameters |
|------|----------|---------------|------------|
| Send | `/cosmos.bank.v1beta1.MsgSend` | SendAuthorization | spend_limit, allow_list |
| IBC Transfer | `/ibc.applications.transfer.v1.MsgTransfer` | TransferAuthorization | allocations (port, channel, spend_limit, allow_list) |
| Stake | `/cosmos.staking.v1beta1.MsgDelegate` | StakeAuthorization | max_tokens, allow_list, deny_list, auth_type |
| Contract Exec | `/cosmwasm.wasm.v1.MsgExecuteContract` | ContractExecutionAuthorization | contracts (address, max_calls, max_funds, filter) |
| Generic | Any | GenericAuthorization | (none) |

#### 1.3 Types (Rust)

```rust
pub enum GrantAuthType {
    Generic,
    Send {
        spend_limit: Vec<Coin>,
        allow_list: Option<Vec<String>>,
    },
    IbcTransfer {
        allocations: Vec<IbcAllocation>,
    },
    Stake {
        max_tokens: Option<Coin>,
        allow_list: Option<Vec<String>>,
        deny_list: Option<Vec<String>>,
        auth_type: StakeAuthType,
    },
    ContractExecution {
        grants: Vec<ContractGrant>,
    },
}

pub struct ContractGrant {
    pub address: String,
    pub max_calls: Option<u64>,
    pub max_funds: Option<Vec<Coin>>,
    pub filter: ContractFilter,
}

pub enum ContractFilter {
    AllowAll,
    AcceptedKeys(Vec<String>),
}
```

### Phase 2: Fee Config Implementation

#### 2.1 CLI Commands

```bash
# Add/Update fee config
xion-toolkit treasury fee-config set --address <TREASURY> --config <CONFIG_FILE>
xion-toolkit treasury fee-config set --address <TREASURY> \
  --allowance-type basic \
  --spend-limit "1000000uxion" \
  --expiration "2024-12-31T23:59:59Z"

# Remove fee config
xion-toolkit treasury fee-config remove --address <TREASURY>

# Query fee config
xion-toolkit treasury fee-config query --address <TREASURY>
```

#### 2.2 Supported Allowance Types

| Type | Description | Parameters |
|------|-------------|------------|
| BasicAllowance | One-time allowance | spend_limit, expiration |
| PeriodicAllowance | Recurring allowance | basic + period, period_spend_limit, period_reset |

#### 2.3 Types (Rust)

```rust
pub enum FeeAllowanceType {
    Basic {
        spend_limit: Option<Vec<Coin>>,
        expiration: Option<DateTime<Utc>>,
    },
    Periodic {
        basic: BasicAllowance,
        period: Duration,
        period_spend_limit: Vec<Coin>,
        period_can_spend: Vec<Coin>,
        period_reset: Option<DateTime<Utc>>,
    },
}
```

### Phase 3: API Integration

#### 3.1 Treasury Contract Messages

Based on Developer Portal reference:

```rust
// AddGrantConfig - adds or updates a grant configuration
pub struct AddGrantConfigMsg {
    pub type_url: String,
    pub grant_config: GrantConfig,
}

// RemoveGrantConfig - removes a grant configuration
pub struct RemoveGrantConfigMsg {
    pub type_url: String,
}

// SetFeeConfig - sets or updates fee configuration
pub struct SetFeeConfigMsg {
    pub fee_config: FeeConfig,
}

// RemoveFeeConfig - removes fee configuration
pub struct RemoveFeeConfigMsg {}
```

#### 3.2 OAuth2 API Flow

1. Build contract message (AddGrantConfig, etc.)
2. Encode to base64
3. Call OAuth2 API `/tx/broadcast` with `MsgExecuteContract`
4. Poll for transaction result

## Tasks

### Grant Config
- [ ] Add `GrantConfigInput` and related types to `treasury/types.rs`
- [ ] Implement grant config encoding in `treasury/encoding.rs`
- [ ] Add `grant-config` subcommands to `cli/treasury.rs`
- [ ] Implement `add_grant_config`, `remove_grant_config`, `list_grant_configs` in `treasury/api_client.rs`
- [ ] Update `skills/xion-treasury/scripts/grant-config.sh`

### Fee Config
- [ ] Add `FeeConfigInput` and related types to `treasury/types.rs`
- [ ] Implement fee config encoding in `treasury/encoding.rs`
- [ ] Add `fee-config` subcommands to `cli/treasury.rs`
- [ ] Implement `set_fee_config`, `remove_fee_config`, `query_fee_config` in `treasury/api_client.rs`
- [ ] Update `skills/xion-treasury/scripts/fee-config.sh`

### Testing & Documentation
- [ ] Add integration tests for grant-config commands
- [ ] Add integration tests for fee-config commands
- [ ] Update `skills/xion-treasury/SKILL.md`

## Acceptance Criteria

- [ ] `xion-toolkit treasury grant-config add --address <ADDR> --config config.json` works
- [ ] `xion-toolkit treasury grant-config remove --address <ADDR> --type-url <TYPE>` works
- [ ] `xion-toolkit treasury grant-config list --address <ADDR>` works
- [ ] `xion-toolkit treasury fee-config set --address <ADDR> --config config.json` works
- [ ] `xion-toolkit treasury fee-config remove --address <ADDR>` works
- [ ] `xion-toolkit treasury fee-config query --address <ADDR>` works
- [ ] Skills scripts work with CLI instead of placeholders
- [ ] All commands output JSON to stdout
- [ ] Errors include error codes and suggestions

## Technical Notes

### OAuth2 API Supported Messages

OAuth2 API supports `MsgExecuteContract` which we'll use to call Treasury contract methods:
- `add_grant_config`
- `remove_grant_config`
- `set_fee_config`
- `remove_fee_config`

### Encoding Pattern

Follow existing pattern from Developer Portal:

```typescript
// From Developer Portal: encoding.ts
const msg = {
  add_grant_config: {
    type_url: typeUrl,
    grant_config: {
      description: description,
      authorization: {
        type_url: authTypeUrl,
        value: encodedValue, // Base64 encoded authorization
      },
      optional: false,
    },
  },
};

// Encode contract message to base64
const encodedMsg = toBase64(toUtf8(JSON.stringify(msg)));
```

### Reference Implementations

1. **Developer Portal**:
   - `src/components/Treasury/NewTreasury/GrantConfigEncoder.tsx` - Grant config UI + encoding
   - `src/components/Treasury/NewTreasury/AllowanceEncoder.tsx` - Fee config UI + encoding
   - `src/components/Treasury/RemoveGrantConfig/RemoveGrantConfigForm.tsx` - Remove grant

2. **abstraxion-core**:
   - `encodeGrantConfig()` - Authorization encoding
   - `decodeAuthorization()` - Authorization decoding
   - Authorization types: SendAuthorization, StakeAuthorization, etc.

## Sign-off

| Date | Content | Status |
|------|---------|--------|
| 2026-03-06 | Initial plan created | Draft |
