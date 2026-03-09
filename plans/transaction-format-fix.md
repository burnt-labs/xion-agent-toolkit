---
status: InProgress
created_at: 2026-03-08
updated_at: 2026-03-09
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

## Root Cause Deep-Dive (2026-03-09)

### Full Encoding Chain Analysis

Traced the complete path: **CLI → OAuth2 API `buildMessage` → CosmJS Registry → `fromPartial` → `BinaryWriter` → chain**.

#### Step 1: CLI sends JSON request

Our CLI sends the `value` field as a **JSON object** (not protobuf bytes):

```json
{
  "typeUrl": "/cosmwasm.wasm.v1.MsgExecuteContract",
  "value": {
    "sender": "xion1...",
    "contract": "xion1...",
    "msg": "eyJ1cGRhdGVf...",  // base64 string (current code)
    "funds": []
  }
}
```

#### Step 2: OAuth2 API `buildMessage` — enters `else` branch

File: `~/workspace/xion/oauth2-api-service/src/utils/transactions.ts`

The API checks if `value` is protobuf bytes (number array or `"base64:"` prefix string).
Since our `value` is a JSON object, it falls through to the `else` branch:

```typescript
} else {
    const value: Record<string, unknown> =
      txRequest.value && typeof txRequest.value === 'object' ? { ...txRequest.value } : {}
    // Sets sender/fromAddress only, msg field is NOT converted!
    encodeObj = { typeUrl: txRequest.typeUrl, value: value }
}
```

**Critical**: The `else` branch does NOT call `fromAmino`. It just clones the object and passes it through. The previous analysis claiming it calls `fromAmino` was **incorrect**.

#### Step 3: CosmJS Registry uses `fromPartial`, NOT `fromAmino`

File: `oauth2-api-service/node_modules/@cosmjs/proto-signing/build/registry.js`

```javascript
const instance = isTelescopeGeneratedType(type) || isTsProtoGeneratedType(type)
    ? type.fromPartial(value)   // ← Always fromPartial, never fromAmino!
    : type.create(value);
return type.encode(instance).finish();
```

#### Step 4: `fromPartial` does NOT convert `msg`

File: `@burnt-labs/xion-types/types/cosmwasm/wasm/v1/tx.ts`

```typescript
fromPartial(object: Partial<MsgExecuteContract>): MsgExecuteContract {
    const message = createBaseMsgExecuteContract();
    message.sender = object.sender ?? "";
    message.contract = object.contract ?? "";
    message.msg = object.msg ?? new Uint8Array();  // Direct assignment, no conversion!
    message.funds = object.funds?.map(e => Coin.fromPartial(e)) || [];
    return message;
},
```

`fromPartial` expects `msg` to **already be `Uint8Array`**. It does not convert strings or objects.

In contrast, `fromAmino` DOES convert (but is never called in this path):

```typescript
fromAmino(object: MsgExecuteContractAmino): MsgExecuteContract {
    // ...
    if (object.msg !== undefined && object.msg !== null) {
      message.msg = toUtf8(JSON.stringify(object.msg));  // ← Converts JSON to Uint8Array
    }
},
```

#### Step 5: `BinaryWriter.bytes()` fails with wrong types

File: `oauth2-api-service/node_modules/cosmjs-types/binary.js`

```javascript
bytes(value) {
    const len = value.length >>> 0;
    if (!len) return this._push(varint_1.writeByte, 1, 0);
    return this.uint32(len)._push(writeBytes, len, value);
}

function writeBytes(val, buf, pos) {
    if (typeof Uint8Array !== "undefined") {
        buf.set(val, pos);  // Uint8Array.set() requires array-like input
    }
}
```

| `msg` value type | `value.length` | `buf.set(val, pos)` | Result |
|------------------|----------------|----------------------|--------|
| Base64 string `"eyJ1cGRhdGVf..."` | string length | `buf.set(string)` → TypeError or wrong bytes | **msg invalid** |
| JSON object `{"withdraw": {...}}` | `undefined` → `0` | Skipped (writes 0 bytes) | **msg invalid** |
| Number array `[123, 34, ...]` | array length (correct) | `buf.set(array)` → correct bytes | **✅ Success** |
| `Uint8Array` | byte length (correct) | `buf.set(uint8array)` → correct bytes | **✅ Success** |

### Root Cause Summary

**The OAuth2 API's JSON object path is broken for CosmWasm `bytes` fields.**

The `else` branch in `buildMessage` passes the JSON object directly to `signAndBroadcast`, which uses `fromPartial` (not `fromAmino`). Since `fromPartial` does not convert the `msg` field to `Uint8Array`, any non-`Uint8Array` value for `msg` results in incorrect protobuf encoding.

**Neither base64 string nor raw JSON object works** when the outer `value` is a JSON object. The only types that work are `Uint8Array` and plain number arrays (array-like objects).

### Why the Developer Portal Works

The Developer Portal does **NOT** go through the OAuth2 API. It uses CosmJS directly in the browser:

```typescript
// Developer Portal: converts msg to Uint8Array BEFORE fromPartial
const msgs = instructions.map((i) => ({
    typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
    value: MsgExecuteContract.fromPartial({
        sender: senderAddress,
        contract: i.contractAddress,
        msg: toUtf8(JSON.stringify(i.msg)),  // ← Already Uint8Array!
        funds: [...(i.funds || [])],
    }),
}));
```

### Previous Incorrect Analysis (2026-03-08)

The earlier analysis (now superseded) incorrectly stated:
- ❌ "OAuth2 API calls `MsgExecuteContract.fromAmino(object)`" — It uses `fromPartial`, not `fromAmino`
- ❌ "Use Protobuf format (base64) consistently" — Base64 strings don't work in the JSON object path either
- ❌ "Our mistake: inconsistent format" — The real issue is that `fromPartial` doesn't convert `msg` at all

## Solution (2026-03-09)

### Approach: JSON Object with `msg` as Number Array

Keep the outer `value` as a JSON object, but encode `msg` (and `salt`) as **number arrays** instead of base64 strings. This is the JSON representation of `Uint8Array`.

#### Why This Works

1. CLI sends `msg: [123, 34, 119, ...]` (UTF-8 bytes of JSON as number array)
2. API `buildMessage` passes it through (correct — no conversion needed)
3. `fromPartial` assigns: `message.msg = [123, 34, 119, ...]` (array-like, accepted)
4. `BinaryWriter.bytes()`: `buf.set([123, 34, ...], pos)` — `Uint8Array.set()` accepts array-like objects
5. Chain receives correct UTF-8 bytes of `{"update_grant_config": {...}}` → **parses successfully**

#### Implementation

For `MsgExecuteContract`:

```rust
let msg_json = serde_json::to_string(execute_msg)?;
let msg_bytes = serde_json::to_value(msg_json.as_bytes())?;  // [123, 34, 119, ...]

let msg_value = serde_json::json!({
    "sender": sender,
    "contract": contract,
    "msg": msg_bytes,   // Number array, not base64 string
    "funds": []
});
```

For `MsgInstantiateContract2`:

```rust
let msg_json = serde_json::to_string(&instantiate_msg)?;
let msg_bytes = serde_json::to_value(msg_json.as_bytes())?;  // [123, 34, 119, ...]
let salt_bytes = serde_json::to_value(salt)?;                 // [1, 2, 3, ...]

let msg_value = serde_json::json!({
    "sender": request.admin,
    "codeId": treasury_code_id,
    "label": label,
    "msg": msg_bytes,      // Number array
    "salt": salt_bytes,    // Number array
    "funds": [],
    "admin": request.admin,
    "fixMsg": false,
});
```

#### Alternative Approaches Considered

| Approach | Description | Pros | Cons |
|----------|-------------|------|------|
| **A. Number array (chosen)** | `msg: [123, 34, ...]` | Simple Rust changes; no API changes | Slightly larger JSON payload |
| **B. Full protobuf encoding** | `value: "base64:..."` | Cleanest wire format | Must protobuf-encode entire message in Rust |
| **C. Fix OAuth2 API** | Add `msg` conversion in `buildMessage` | Fixes for all clients | Requires API deployment; not our repo |

## Approach

### Phase 1: Verify Treasury Create ✅
- [x] Analyze successful on-chain transaction
- [x] Document the exact encoding pattern
- [x] Verify: `msg` field is base64-encoded JSON string

### Phase 2: Deep-Dive Root Cause Analysis (2026-03-09) ✅
- [x] Trace full encoding chain: CLI → API → Registry → BinaryWriter → chain
- [x] Identify `fromPartial` vs `fromAmino` mismatch
- [x] Verify that neither base64 string nor raw JSON object works
- [x] Confirm number array format works with `BinaryWriter.bytes()` and `Uint8Array.set()`
- [x] Document the correct fix approach

### Phase 3: Apply Fix ✅
- [x] Update `broadcast_execute_contract` — `msg` → number array
- [x] Update `withdraw_treasury` — `msg` → number array
- [x] Update `create_treasury` — `msg` + `salt` → number arrays
- [x] Add helper function `bytes_to_json_array`
- [x] Remove unused `base64` import
- [x] All unit tests pass (115 tests)
- [x] `cargo clippy` + `cargo fmt` pass

### Phase 4: E2E Testing (2026-03-09)
- [x] Test `treasury grant-config add` on testnet — **SUCCESS** (tx: `30EA09A1C0E6D88D2F5A725B18F8412D44AE2DF93593965B1C8922780FA5937B`)
- [ ] Test `treasury fee-config set` on testnet — **AUTH ISSUE** (format OK, OAuth2 API returns "GRANTED_FAILED: Authorization was not granted by user")
- [ ] Test `treasury withdraw` on testnet — **AUTH ISSUE** (format OK, OAuth2 API returns "GRANTED_FAILED")
- [ ] Test `treasury create` on testnet — **AUTH ISSUE** (format OK, OAuth2 API returns "GRANTED_FAILED")

### Key Findings (2026-03-09)

1. **Transaction Format Fix is Working**: The number array format for `msg` and `salt` fields is correct. We're no longer getting "msg: invalid" errors.

2. **Grant-Config Add Succeeded**: The `treasury grant-config add` command successfully broadcasted a transaction.

3. **Authorization Issues**: Some operations fail with "GRANTED_FAILED: Authorization was not granted by user". This is an OAuth2 API / session key / authz grant issue, not a transaction format issue. This requires further investigation of the OAuth2 API's grant flow.

4. **Refactoring Complete**: `withdraw_treasury` now uses the unified `broadcast_execute_contract` method.

## Success Criteria

- [x] Transaction format fixed (number array for `msg` and `salt`)
- [x] All MsgExecuteContract operations use unified `broadcast_execute_contract`
- [x] Unit tests pass
- [x] Grant config operations work (format verified)
- [ ] Treasury create command works end-to-end (blocked by auth issue)
- [ ] Fee config operations work end-to-end (blocked by auth issue)
- [ ] Withdraw operations work end-to-end (blocked by auth issue)

## On-Chain Transaction Analysis (2026-03-08)

### Analyzed Transaction

**Tx Hash**: `0E09A0C8BF18FE8DE051C344D750A4ABE1DFE03DFE8A919CCC210317B2E672C3`

**Operation**: Treasury creation via `MsgInstantiateContract2`

**Status**: ✅ SUCCESS (created via Developer Portal, not CLI)

### Key Findings

This transaction was sent through the **Developer Portal** (which uses CosmJS directly, not the OAuth2 API). The Developer Portal calls `toUtf8(JSON.stringify(msg))` to convert `msg` to `Uint8Array` before passing to `fromPartial`.

The on-chain `msg` field contains UTF-8 bytes of the JSON string, which is base64-encoded when displayed in block explorers. This is the correct format — the question is how to get the OAuth2 API path to produce the same result.

## Reference

### Key Source Files

**OAuth2 API Service** (`~/workspace/xion/oauth2-api-service`):
- `src/utils/transactions.ts` — `buildMessage` function (3 format paths)
- `src/routes/api/transaction/broadcast.ts` — Transaction broadcast handler

**CosmJS / xion-types**:
- `node_modules/@cosmjs/proto-signing/build/registry.js` — Uses `fromPartial`, line 94-98
- `node_modules/@burnt-labs/xion-types/types/cosmwasm/wasm/v1/tx.ts` — `fromPartial` (line 2166) vs `fromAmino` (line 2174)
- `node_modules/cosmjs-types/binary.js` — `BinaryWriter.bytes()` (line 299), `writeBytes` (line 311)

**Developer Portal** (`~/workspace/xion/xion-developer-portal`):
- `node_modules/@cosmjs/cosmwasm-stargate/build/signingcosmwasmclient.js` — `toUtf8(JSON.stringify(msg))` pattern

**CLI** (`src/treasury/api_client.rs`):
- `broadcast_execute_contract` — Generic execute contract helper
- `withdraw_treasury` — Withdraw funds
- `create_treasury` — Create new treasury contract

### Successful Transaction Examples

**On-chain analysis (2026-03-08)**:
- Treasury Create: `0E09A0C8BF18FE8DE051C344D750A4ABE1DFE03DFE8A919CCC210317B2E672C3` (via Developer Portal)

**From previous E2E tests (2026-03-08)**:
- Basic Fee Config: `A866A6D2394A0DC1923BCD497D1E0EC1F665F8EA19A6B13C73C7B8FEF26A2D2C`
- Periodic Fee Config: `DBB96A64AAD75B21A9FCB0F609815E0FAAF1C333572D90AA6C87B875C22F98D3`
- Send Grant Config: `FEE48BF3744A6DAA60852EE435496120483469B5797AEBE146120CE64C690DBE`

## Sign-off

| Date | Content | Status |
|------|---------|--------|
| 2026-03-08 | Plan created, starting investigation | In Progress |
| 2026-03-08 | Fixed `broadcast_execute_contract` - changed `msg` field from base64 to raw JSON object | Done (Incorrect) |
| 2026-03-08 | Analyzed on-chain successful transaction `0E09A0C8...` | ✅ Complete |
| 2026-03-08 | Attempted fix: Use base64-encoded JSON string for `msg` field | ❌ Incorrect — `fromPartial` doesn't decode base64 |
| 2026-03-09 | **Deep-dive root cause analysis**: Traced full chain CLI→API→Registry→BinaryWriter | ✅ Complete |
| 2026-03-09 | Found: API uses `fromPartial` (not `fromAmino`); `msg` must be `Uint8Array`/number array | ✅ Complete |
| 2026-03-09 | **Fix implementation**: Changed `msg` and `salt` to number arrays; added `bytes_to_json_array` helper | ✅ Complete |
| 2026-03-09 | **Refactoring**: `withdraw_treasury` now uses unified `broadcast_execute_contract` | ✅ Complete |
| 2026-03-09 | **Unit tests**: All 115 tests pass | ✅ Complete |
| 2026-03-09 | **E2E test**: `treasury grant-config add` SUCCESS (tx: `30EA09A1...`) | ✅ Format verified |
| 2026-03-09 | **E2E tests**: `fee-config set`, `withdraw`, `create` blocked by OAuth2 GRANTED_FAILED | 🔄 Auth issue TBD |
| 2026-03-09 | **Implementation complete**: All 3 functions updated, tests pass | ✅ Done |
