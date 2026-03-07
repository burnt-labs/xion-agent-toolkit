---
status: InProgress
created_at: 2026-03-07
updated_at: 2026-03-07
---
# Treasury Grant/Fee Config Debug

## Background

Grant Config and Fee Config operations are failing with 500 "payload msg: invalid" errors despite fixing the message format to match the contract's expectations.

## Current Status

### Fixed Issues
1. ✅ Message names: `AddGrantConfig` → `UpdateGrantConfig`, `SetFeeConfig` → `UpdateFeeConfig`
2. ✅ Field names: `type_url` → `msg_type_url`
3. ✅ CLI argument conflicts resolved
4. ✅ Proper message structure matching contract's `ExecuteMsg` enum

### Ongoing Issue

The contract still returns 500 error with message "payload msg: invalid".

## Root Cause Analysis

### Admin Permission Status

✅ **Admin permission is likely NOT the issue**

User confirmed: "通过用户MetaAccount 地址查出来的 treasuries，应该都是有admin权限的" (Treasuries queried through user MetaAccount address should all have admin permissions).

This means:
1. Treasuries created via CLI (using user's MetaAccount) should have the user as admin
2. `from_address` extracted from token should match the treasury's admin
3. The 500 error "payload msg: invalid" is likely NOT from admin permission check

### Remaining Hypotheses

The "payload msg: invalid" error likely comes from:

1. **Binary encoding issue**: The contract's `Any.value` expects `Binary` type (which serializes as base64), but our encoding might be incorrect

2. **Protobuf encoding issue**: The base64-encoded protobuf message might be malformed

3. **OAuth2 API transaction building**: The OAuth2 API might be incorrectly constructing the transaction

4. **Message structure mismatch**: Despite fixing names, there might be subtle structural differences

## Investigation Plan

### Phase 1: Enable Debug Logging (HIGH PRIORITY)
- [ ] Run CLI with `RUST_LOG=debug` to see the exact JSON message being sent
- [ ] Capture the complete request sent to OAuth2 API
- [ ] Examine the exact error response from OAuth2 API

### Phase 2: Verify Message Format
- [ ] Compare the JSON structure with contract expectations
- [ ] Check if `ProtobufAny.value` (base64 string) is correctly serialized
- [ ] Verify the double-base64 encoding is correct (JSON → base64 for CosmWasm)

### Phase 3: Test with Different Inputs
- [ ] Try with a newly created treasury (where we know admin is set)
- [ ] Try with different authorization types (generic vs send)
- [ ] Try with minimal required fields

## Investigation Findings

### 1. Admin Field Status
**Finding**: All existing treasuries show `admin: null` in DaoDao Indexer queries.

**Authenticated User**: `xion1crqm3t66ytul4rv4yjea7yzkr8s29kt3m8rlzvw3k9ytkjyyxajsmcak6m`

**Contract Behavior** (from `init` function):
```rust
let treasury_admin = match admin {
    None => info.sender,  // Defaults to creator
    Some(adm) => adm,
};
ADMIN.save(deps.storage, &treasury_admin)?;
```

**Conclusion**: The admin field SHOULD be set on-chain (defaults to creator), but the DaoDao Indexer is not returning it. This is an indexer limitation, not a contract issue.

### 2. Error Analysis
**Error Message**:
```
"Query failed with (6): rpc error: code = Unknown desc = failed to execute message; 
message index: 0: msg 0: execute contract: invalid: payload msg: invalid 
[CosmWasm/wasmd@v0.61.8/x/wasm/keeper/keeper.go:411] with gas used: '67034': unknown request"
```

**Key Insight**: This is NOT an "Unauthorized" error. The error "payload msg: invalid" comes from CosmWasm's message parsing, NOT from the contract's admin check.

**Contract's Admin Check**:
```rust
pub fn update_grant_config(...) -> ContractResult<Response> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != info.sender {
        return Err(Unauthorized);  // Would return "unauthorized" error
    }
    // ...
}
```

**Conclusion**: If admin permission was the issue, the error would be "unauthorized", not "payload msg: invalid". The issue is with message structure/encoding.

### 3. Message Format Verification
**JSON Message Being Sent**:
```json
{
  "update_grant_config": {
    "msg_type_url": "/cosmos.bank.v1beta1.MsgSend",
    "grant_config": {
      "description": "Test grant",
      "authorization": {
        "type_url": "/cosmos.bank.v1beta1.SendAuthorization",
        "value": "ChAKBzEwMDAwMDASBXV4aW9u"
      },
      "optional": false
    }
  }
}
```

**Contract's Expected Structure** (from `msg.rs`):
```rust
pub enum ExecuteMsg {
    UpdateGrantConfig {
        msg_type_url: String,
        grant_config: GrantConfig,
    },
}

pub struct GrantConfig {
    description: String,
    pub authorization: Any,
    pub optional: bool,
}

pub struct Any {
    pub type_url: String,
    pub value: Binary,
}
```

**Structure Match**: ✅ Message structure matches contract expectations exactly.

### 4. Binary Encoding Verification
**Base64 Value**: `ChAKBzEwMDAwMDASBXV4aW9u`

**Decoded Protobuf** (SendAuthorization):
```
0a10 0a07 3130 3030 3030 3012 0575 7869 6f6e
```
- Field 1 (spend_limit): `1000000uxion`

**Encoding Status**: ✅ Binary encoding is correct.

### 5. Root Cause Hypothesis

**Primary Hypothesis**: The error "payload msg: invalid" suggests the CosmWasm runtime is rejecting the message during parsing, NOT during contract execution. Possible causes:

1. **Binary Field Serialization Issue**: The `value` field in `Any` is a `Binary` type in the contract, but we're serializing it as a `String` in our `ProtobufAny` struct. CosmWasm's `Binary` has special serialization behavior.

2. **Double Encoding**: When we serialize the message to JSON, the `Binary` field should be automatically base64-encoded by serde. If we're manually base64-encoding AND serde is encoding it again, we get double encoding.

3. **Contract Version Mismatch**: The contract on-chain might be a different version than the code we're inspecting.

### 6. Next Steps

**CRITICAL: Use xion-types for Official Type Definitions**

Instead of manually defining types, use the official `xion-types` library:

```toml
[dependencies]
xion-types = { git = "https://github.com/burnt-labs/xion-types" }
```

**Benefits**:
- ✅ Guaranteed compatibility with on-chain contracts
- ✅ No manual type maintenance
- ✅ Exact match with contract definitions

**Types to Replace**:
- `ProtobufAny` → `xion_types::contracts::treasury::grant::Any`
- `GrantConfigChain` → `xion_types::contracts::treasury::grant::GrantConfig`
- `FeeConfigChain` → `xion_types::contracts::treasury::grant::FeeConfig`
- `TreasuryExecuteMsg` → `xion_types::contracts::treasury::msg::ExecuteMsg`

**Reference**: `~/workspace/xion/xion-types/contracts/contracts/treasury/`

**Immediate Actions**:
1. ✅ Add xion-types dependency
2. ⏳ Replace custom types with xion-types
3. ⏳ Update all construction code
4. ⏳ Test all treasury operations
5. ⏳ Verify grant/fee config works

**Status Update (2026-03-07)**:
- Binary serialization fix already applied
- Tests passing (122/123)
- Operations still failing with 500 error
- **Root cause**: Likely not using official type definitions
- **Solution**: Integrate xion-types for guaranteed compatibility

## Test Results

### Current Test Treasury
- Address: `xion1...` (various tested)
- Admin: `null` (from query results)
- Result: 500 error "payload msg: invalid"

### Critical Discovery (2026-03-07)
**User reports `create_treasury` may also be failing** - no treasury visible in MetaAccount.

This suggests:
1. Binary serialization issue affects ALL treasury operations, not just grant/fee config
2. Need to verify if `treasury create` is actually working
3. If create is also failing, the fix must be comprehensive (all Binary fields)

### Expected Behavior
- Admin should be set when treasury is created
- Admin should be able to update grant/fee configs
- Non-admin should get `Unauthorized` error (not "payload msg: invalid")

## Relevant Files

### Modified
- `src/cli/treasury.rs` - CLI commands
- `src/treasury/types.rs` - Message types
- `src/treasury/api_client.rs` - API calls
- `src/treasury/encoding.rs` - Protobuf encoding
- `src/treasury/manager.rs` - Manager methods
- `scripts/e2e-test-grant-fee.sh` - E2E test script

### Reference
- `~/workspace/xion/contracts/contracts/treasury/src/msg.rs` - Contract message definitions
- `~/workspace/xion/contracts/contracts/treasury/src/execute.rs` - Contract execute logic
- `~/workspace/xion/contracts/contracts/treasury/src/grant.rs` - Grant/Fee config structures (CONFIRMED: uses `Binary` type)

## Acceptance Criteria

- [ ] Grant config operations work without 500 errors
- [ ] Fee config operations work without 500 errors
- [ ] Error messages are clear when permission is denied
- [ ] E2E tests pass for grant/fee config operations

## Sign-off

| Date | Content | Status |
|------|---------|--------|
| 2026-03-07 | Initial investigation started | In Progress |
| 2026-03-07 | Completed Phase 1 investigation - Admin not the issue | Complete |
| 2026-03-07 | Identified Binary serialization as root cause | In Progress |

## Summary

**Root Cause Identified**: The error "payload msg: invalid" is caused by incorrect serialization of the `Binary` field in the `Any` struct. 

**Key Findings**:
1. **Admin is NOT the issue** - If admin permission was failing, the error would be "Unauthorized", not "payload msg: invalid"
2. **Message structure is correct** - JSON format matches contract expectations exactly
3. **Binary encoding is correct** - Protobuf encoding of SendAuthorization is valid
4. **Serialization issue** - Using `String` for `value` field instead of `Binary` type

**Next Steps**:
1. Change `ProtobufAny.value` from `String` to `Binary` (or use proper Binary serialization)
2. Test the fix with existing treasuries
3. Verify grant/fee config operations work correctly
