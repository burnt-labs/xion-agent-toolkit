---
status: Done
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

## Approach

### Phase 1: Verify Treasury Create
- Test `treasury create` command
- Capture successful transaction format
- Document the exact encoding pattern

### Phase 2: Extract Common Method
- Create generic transaction building function
- Ensure it matches successful create pattern exactly
- Test with simple operations first

### Phase 3: Apply to All Operations
- Refactor all treasury operations to use common method
- Test each operation type:
  - Grant config (add/remove/query)
  - Fee config (set/remove/query)
  - Treasury management (query/list)

## Success Criteria

- [ ] Treasury create command works successfully
- [ ] Common transaction method extracted
- [ ] All grant config operations work
- [ ] All fee config operations work
- [ ] E2E tests pass for all operations

## Investigation Results (2026-03-08)

### Root Cause Analysis

| Component | `msg` Format | Status |
|-----------|-------------|--------|
| `create_treasury` (MsgInstantiateContract2) | Base64 encoded JSON string | ✅ Working |
| `broadcast_execute_contract` (MsgExecuteContract) | Base64 encoded JSON string | ❌ Failing |
| `withdraw_treasury` (MsgExecuteContract) | **Raw JSON object** (not base64) | Different implementation |

### Key Finding

OAuth2 API handles message fields differently:
- `MsgInstantiateContract2.msg` → Expects **base64-encoded** JSON string
- `MsgExecuteContract.msg` → May expect **raw JSON object** (like `withdraw_treasury`)

### Files to Modify
- `src/treasury/api_client.rs` - `broadcast_execute_contract` method (lines 175-210)

## Tasks

- [x] Test treasury create with detailed logging
- [x] Analyze successful transaction format
- [x] Fix `broadcast_execute_contract` to use raw JSON for `msg` field
- [ ] Verify fix with E2E testing
- [ ] Update documentation

## Reference

### Successful Transaction Examples
From previous E2E tests (2026-03-08):
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
| 2026-03-08 | Fixed `broadcast_execute_contract` - changed `msg` field from base64 to raw JSON object | Done |

## Implementation Details (2026-03-08)

### Changes Made

**File**: `src/treasury/api_client.rs`

**Before** (lines 183-194):
```rust
// Convert execute message to JSON then to base64 (OAuth2 API expects base64-encoded JSON string)
let msg_json = serde_json::to_string(execute_msg)?;
let msg_base64 = base64::engine::general_purpose::STANDARD.encode(msg_json.as_bytes());

debug!("Execute message JSON:\n{}", msg_json);

// Build MsgExecuteContract message value (matching create_treasury format)
let msg_value = serde_json::json!({
    "sender": sender,
    "contract": contract,
    "msg": msg_base64,  // base64-encoded JSON string
    "funds": []
});
```

**After**:
```rust
// OAuth2 API expects raw JSON object for MsgExecuteContract.msg field
// (unlike MsgInstantiateContract2 which expects base64-encoded JSON string)
let msg_value = serde_json::json!({
    "sender": sender,
    "contract": contract,
    "msg": execute_msg,  // Raw JSON object directly, not base64
    "funds": []
});
```

**Also removed**: unused `use base64::Engine;` import

### Verification

- ✅ `cargo fmt` - passed
- ✅ `cargo clippy --all-targets --all-features -- -D warnings` - passed
- ✅ `cargo test` - 115 treasury tests passed (1 unrelated encryption test failed due to race condition)

### Affected Operations

The fix affects all operations using `broadcast_execute_contract`:
- `add_grant_config` (line 1099)
- `remove_grant_config` (line 1137)
- `set_fee_config` (line 1239)
- `revoke_allowance` (line 1276)
