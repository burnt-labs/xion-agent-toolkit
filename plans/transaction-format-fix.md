---
status: InProgress
created_at: 2026-03-08
updated_at: 2026-03-08
---
# Transaction Format Fix & Common Method Extraction

## Background

After refactoring to use base64-encoded JSON for `msg` field (matching `create_treasury`), all treasury operations still fail with "payload msg: invalid" error. Both old format (JSON object) and new format (base64 JSON) fail.

Need to investigate and fix transaction format by:
1. Ensuring `treasury create` works (as reference)
2. Extracting the working encoding pattern
3. Applying it to all other operations

## Goal

Fix all treasury transaction operations by using the correct message format.

## On-Chain Transaction Analysis (2026-03-08)

### Analyzed Transaction

**Tx Hash**: `0E09A0C8BF18FE8DE051C344D750A4ABE1DFE03DFE8A919CCC210317B2E672C3`

**Operation**: Treasury creation via `MsgInstantiateContract2`

**Status**: ✅ SUCCESS

### Key Findings

#### 1. MsgInstantiateContract2 Format

**`msg` field encoding**:
```
Original: {"admin":"xion1...","grant_configs":[...],"fee_config":{...}}
↓ JSON serialize
JSON string: '{"admin":"xion1...","grant_configs":[...]}'
↓ base64 encode
Final: eyJhZG1pbiI6Inhpb24xY3JxbTMtdD...
```

**`salt` field encoding**:
```
Original: 32 random bytes [0x12, 0x34, 0x56, ...]
↓ base64 encode (for JSON serialization)
Final: MTIzNDU2... (base64 string)
```

**Important**: In protobuf, both `msg` and `salt` are `bytes` fields, so when sending via JSON RPC, they must be base64-encoded strings.

#### 2. MsgExecuteContract Format (Expected)

Same pattern as `MsgInstantiateContract2`:

**`msg` field encoding**:
```
Original: {"update_grant_config":{...}}
↓ JSON serialize
JSON string: '{"update_grant_config":{...}}'
↓ base64 encode
Final: eyJ1cGRhdGVfZ3JhbnRfY29uZmlnI...
```

### OAuth2 API Service Format Support

From `~/workspace/xion/oauth2-api-service/src/utils/transactions.ts` analysis:

The OAuth2 API Service supports **three formats** for the `value` field:

| Format | Example | Usage |
|--------|---------|-------|
| **Protobuf bytes array** | `[1, 2, 3, ...]` | Direct protobuf encoding |
| **Base64 prefix string** | `"base64:ABC123..."` | Alternative protobuf format |
| **Raw JSON object (Amino)** | `{sender: "...", msg: {...}}` | Auto-converts via `fromAmino` |

**When using Raw JSON object (Amino) format**:
- OAuth2 API calls `MsgExecuteContract.fromAmino(object)`
- `fromAmino` expects `msg` field to be **raw JSON object**
- It automatically converts: `msg = toUtf8(JSON.stringify(object.msg))`

**When using Protobuf format** (our choice):
- We must **pre-encode** the `msg` field as base64
- Because we're sending base64-encoded protobuf bytes

### Root Cause

**Problem**: We were using **Raw JSON object (Amino) format** but treating `msg` as if we were using Protobuf format.

**Two valid approaches**:
1. **Amino format**: `msg: {update_grant_config: {...}}` (raw JSON object)
2. **Protobuf format**: `msg: "eyJ1cGRhdGVf..."` (base64-encoded JSON string)

**Our mistake**: We were inconsistent - using Amino format but trying to base64-encode the `msg` field.

### Solution

**Use Protobuf format consistently**:

For all `MsgExecuteContract` and `MsgInstantiateContract2` messages:

```rust
// Serialize contract message to JSON
let msg_json = serde_json::to_string(&contract_msg)?;

// Base64 encode for protobuf bytes field
let msg_base64 = base64::engine::general_purpose::STANDARD.encode(msg_json.as_bytes());

// Build the message
let msg_value = serde_json::json!({
    "sender": sender,
    "contract": contract,
    "msg": msg_base64,  // Base64-encoded JSON string
    "funds": []
});
```

## Approach

### Phase 1: Verify Treasury Create ✅
- [x] Analyze successful on-chain transaction
- [x] Document the exact encoding pattern
- [x] Verify: `msg` field is base64-encoded JSON string

### Phase 2: Apply Fix
- [x] Update `broadcast_execute_contract` to use base64 encoding
- [x] Update `withdraw_treasury` to use base64 encoding
- [x] Update `create_treasury` to use base64 encoding
- [x] All unit tests pass (330 tests)

### Phase 3: E2E Testing
- [ ] Test `treasury create` on testnet
- [ ] Test `treasury grant-config add` on testnet
- [ ] Test `treasury fee-config set` on testnet
- [ ] Test `treasury withdraw` on testnet

## Success Criteria

- [ ] Treasury create command works successfully
- [ ] All grant config operations work
- [ ] All fee config operations work
- [ ] E2E tests pass for all operations

## Implementation Details (2026-03-08 - Latest Fix)

### Changes Made

**File**: `src/treasury/api_client.rs`

#### 1. Added base64::Engine trait import
```rust
use base64::Engine;
```

#### 2. Fixed `broadcast_execute_contract` method (lines 174-194)
```rust
// Serialize execute message to JSON then to base64
let msg_json = serde_json::to_string(execute_msg)?;
let msg_base64 = base64::engine::general_purpose::STANDARD.encode(msg_json.as_bytes());

debug!("Execute message JSON:\n{}", msg_json);

let msg_value = serde_json::json!({
    "sender": sender,
    "contract": contract,
    "msg": msg_base64,  // Base64-encoded JSON string
    "funds": []
});
```

#### 3. Fixed `withdraw_treasury` method (lines 667-692)
```rust
// Create the Withdraw execute message
let withdraw_msg = serde_json::json!({
    "withdraw": {
        "coins": [{ "amount": amount_val, "denom": denom }]
    }
});

// Serialize to JSON then to base64
let msg_json = serde_json::to_string(&withdraw_msg)?;
let msg_base64 = base64::engine::general_purpose::STANDARD.encode(msg_json.as_bytes());

let request = BroadcastRequest {
    messages: vec![super::types::TransactionMessage {
        type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
        value: serde_json::json!({
            "sender": from_address,
            "contract": treasury_address,
            "msg": msg_base64,  // Base64-encoded JSON string
            "funds": []
        }),
    }],
    memo: Some(format!("Withdraw from treasury {}", treasury_address)),
};
```

#### 4. Fixed `create_treasury` method (lines 774-800)
```rust
// Build the instantiation message
let instantiate_msg = build_treasury_instantiate_msg(&request)?;

// Serialize instantiate message to JSON then to base64
let msg_json = serde_json::to_string(&instantiate_msg)?;
let msg_base64 = base64::engine::general_purpose::STANDARD.encode(msg_json.as_bytes());

// Convert salt to base64
let salt_base64 = base64::engine::general_purpose::STANDARD.encode(salt);

// Build the MsgInstantiateContract2 message
let msg_value = serde_json::json!({
    "sender": request.admin,
    "codeId": treasury_code_id,
    "label": format!("Treasury-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")),
    "msg": msg_base64,           // Base64-encoded JSON string
    "salt": salt_base64,          // Base64-encoded salt bytes
    "funds": [],
    "admin": request.admin,
    "fixMsg": false,
});
```

### Verification

- ✅ `cargo fmt` - passed
- ✅ `cargo clippy --all-targets --all-features -- -D warnings` - passed
- ✅ `cargo test` - **330 tests passed** (115 lib + 115 main + 29 treasury_create + 20 treasury + 37 doc-tests)
- ✅ `cargo build --release` - Binary built successfully

### Affected Operations

The fix affects all operations using these methods:
- `broadcast_execute_contract`: `add_grant_config`, `remove_grant_config`, `set_fee_config`, `revoke_allowance`
- `withdraw_treasury`: Withdraw funds from treasury
- `create_treasury`: Create new treasury contract

## Reference

### Successful Transaction Examples

**On-chain analysis (2026-03-08)**:
- Treasury Create: `0E09A0C8BF18FE8DE051C344D750A4ABE1DFE03DFE8A919CCC210317B2E672C3`

**From previous E2E tests (2026-03-08)**:
- Basic Fee Config: `A866A6D2394A0DC1923BCD497D1E0EC1F665F8EA19A6B13C73C7B8FEF26A2D2C`
- Periodic Fee Config: `DBB96A64AAD75B21A9FCB0F609815E0FAAF1C333572D90AA6C87B875C22F98D3`
- Send Grant Config: `FEE48BF3744A6DAA60852EE435496120483469B5797AEBE146120CE64C690DBE`

### Key Files
- `src/treasury/api_client.rs` - API client with broadcast methods
- `src/treasury/types.rs` - Transaction message types
- `src/treasury/encoding.rs` - Protobuf encoding
- `src/treasury/manager.rs` - High-level treasury operations

## Sign-off

| Date | Content | Status |
|------|---------|--------|
| 2026-03-08 | Plan created, starting investigation | In Progress |
| 2026-03-08 | Fixed `broadcast_execute_contract` - changed `msg` field from base64 to raw JSON object | Done (Incorrect) |
| 2026-03-08 | Analyzed on-chain successful transaction `0E09A0C8...` | ✅ Complete |
| 2026-03-08 | **FINAL FIX**: Use base64-encoded JSON string for `msg` field (Protobuf format) | Done |
| 2026-03-08 | E2E testing in progress | 🔄 Pending |
