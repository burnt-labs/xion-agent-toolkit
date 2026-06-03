# FeeConfig Expiration — Research Report

**Date**: 2026-04-03
**Agent**: @architect
**Status**: Complete
**Task**: Determine necessity of implementing FeeConfig expiration support

---

## 1. Executive Summary

**Recommendation: Remove the TODOs and leave `expiration: None`.** Do NOT implement expiration support at this time.

**Rationale**:
- The on-chain contract supports expiration (`Option<u32>` as relative seconds), but neither the developer portal nor any known user sets it.
- The toolkit's existing `FeeConfigChain.expiration` type (`Option<String>` — ISO 8601) is **incorrect** relative to the contract's `Option<u32>` (relative seconds), indicating an earlier misunderstanding.
- Implementing correctly would require changes across 3 layers (input types, encoding, and API client) for a feature with zero known users.
- No documentation, guides, or use-case evidence exists for setting expiration on fee grants.

---

## 2. Findings

### 2.1 Treasury Contract — Expiration Definition

**Source**: `~/workspace/xion/contracts/contracts/treasury/src/grant.rs` (lines 13-18)

```rust
#[cw_serde]
pub struct FeeConfig {
    description: String,
    pub allowance: Option<Any>,
    pub expiration: Option<u32>,  // Relative duration in seconds
}
```

**Storage**: Singleton `Item<FeeConfig>` under key `"fee_config"` in `state.rs:9`.

**Semantics**: `Option<u32>` represents a **relative duration in seconds** from the moment `deploy_fee_grant` is executed. The contract converts this to an absolute `cosmos_sdk_proto::Timestamp` by adding it to `env.block.time`:

```rust
// execute.rs:316-384 (simplified)
let fee_config = FEE_CONFIG.load(deps.storage)?;
let expiration = match fee_config.expiration {
    None => None,
    Some(seconds) => {
        let expiration_time = env.block.time.plus_seconds(seconds as u64);
        Some(Timestamp { seconds: expiration_time.seconds() as i64, ... })
    }
};
```

The computed `Timestamp` is then injected into the protobuf `BasicAllowance.expiration` field via `format_allowance()`.

**No validation** is performed — any `u32` value is accepted as-is.

### 2.2 Developer Portal — Expiration Usage

**Source**: `~/workspace/xion/xion-developer-portal/`

| Location | Finding |
|----------|---------|
| `src/lib/types.ts:51` | `expiration?: number` — defined as optional on the FeeConfig interface |
| `src/components/Treasury/Overview/FeeConfigCard.tsx:113` | Displays `expiration \|\| "N/A"` — raw number, no date formatting |
| `src/components/Treasury/NewTreasury/AllowanceEncoder.tsx` | **No input field for expiration** — the form only collects description, allowance type, and allowance-specific fields |
| `src/core/encoding.ts` | `BasicAllowance.fromPartial()` called with only `spendLimit` — expiration never set |
| `src/core/allowance-decoding.ts` | `decodeBasicAllowance` returns only `spendLimit` — expiration never decoded |
| `CLAUDE.md` import/export format | **expiration omitted entirely** from the documented JSON schema |

**Conclusion**: The developer portal treats expiration as a vestigial field — it exists in the type definition and is displayed read-only, but it is never populated, encoded, decoded, or documented.

### 2.3 Current Toolkit — TODO Locations

#### TODO #1: `src/treasury/manager.rs:1939`

```rust
Ok(super::types::FeeConfigChain {
    description,
    allowance: Some(super::types::ProtobufAny {
        type_url: allowance_type_url,
        value: allowance_value,
    }),
    expiration: None, // TODO: Add expiration support
})
```

Context: Inside `encode_fee_config_input()` which converts `FeeConfigInput` → `FeeConfigChain`.

#### TODO #2: `src/treasury/api_client.rs:1659`

```rust
let fee_config_chain = super::types::FeeConfigChain {
    description: /* ... */,
    allowance: Some(super::types::ProtobufAny { /* ... */ }),
    expiration: None, // TODO: Add expiration support in FeeConfigInput
};
```

Context: Inside `set_fee_config()` which builds `FeeConfigChain` for `update_fee_config` execute message.

#### Existing Type Mismatch

| Layer | Type | Semantics |
|-------|------|-----------|
| On-chain `FeeConfig` | `Option<u32>` | Relative seconds from deploy time |
| Toolkit `FeeConfigChain` | `Option<String>` | Comment says "ISO 8601 string (RFC 3339)" — **WRONG** |
| Toolkit `FeeConfigInput` | Not present | No field at all |
| Portal `FeeConfig` | `expiration?: number` | No documentation of semantics |

The toolkit's `FeeConfigChain.expiration` comment claims ISO 8601 format, but the contract expects `u32` (relative seconds). This mismatch means any implementation based on the current comment would produce incorrect on-chain behavior.

### 2.4 Encoding Layer

**Source**: `src/treasury/encoding.rs:277`

```rust
// Field 2: expiration is not set (optional)
```

The `encode_basic_allowance()` function explicitly does NOT encode the `expiration` field into the protobuf. The same applies to `encode_periodic_allowance()` and `encode_allowed_msg_allowance()`. This is correct for the current `None` behavior, but adding expiration support would require modifying all three encoding functions.

### 2.5 Test Code

All existing tests use `expiration: None`:

- `tests/treasury_create_integration_test.rs` (lines 503, 579, 812): `expiration: None`
- `src/treasury/types.rs` (line 1379): `expiration: None` in `test_treasury_export_data_serialization`
- `src/treasury/types.rs` (line 1469): `expiration: Some("2025-01-01T00:00:00Z".to_string())` in `test_treasury_export_data_roundtrip` — this tests roundtrip serialization but the value is arbitrary and not validated against on-chain behavior

---

## 3. Decision Analysis

### Option A: Implement Expiration Support

**Required changes**:
1. Fix `FeeConfigChain.expiration` type from `Option<String>` to `Option<u32>` (or keep `String` but validate it as relative seconds)
2. Add `expiration: Option<u32>` to all `FeeConfigInput` variants (`Basic`, `Periodic`, `AllowedMsg`)
3. Modify `encode_basic_allowance()`, `encode_periodic_allowance()`, `encode_allowed_msg_allowance()` to encode expiration into protobuf `Timestamp`
4. Update `encode_fee_config_input()` in `manager.rs` to propagate expiration
5. Update `set_fee_config()` in `api_client.rs` to propagate expiration
6. Update CLI arguments for `fee-config set` commands to accept `--expiration <seconds>`
7. Update tests to cover expiration paths
8. Fix the incorrect ISO 8601 comment on `FeeConfigChain.expiration`

**Effort**: S-M (1 focused agent session) — but the API surface change is breaking if any consumer relies on the current `Option<String>` type.

**Benefits**: Completes the feature surface. Enables time-limited fee grants.

**Risks**: Type change is potentially breaking. No known users requesting this feature.

### Option B: Remove TODOs, Keep `expiration: None` (Recommended)

**Required changes**:
1. Replace `expiration: None, // TODO: Add expiration support` with `expiration: None,` in `manager.rs:1939`
2. Replace `expiration: None, // TODO: Add expiration support in FeeConfigInput` with `expiration: None,` in `api_client.rs:1659`
3. Fix the incorrect `FeeConfigChain.expiration` comment (remove or correct the ISO 8601 reference)
4. Optionally add a doc comment explaining why expiration is intentionally not exposed

**Effort**: XS (< 30 minutes)

**Benefits**: Reduces TODO count. Removes misleading comments. No API surface change.

**Risks**: None. The field remains `Option<u32>` on-chain and `Option<String>` in the toolkit — `None` is always valid.

### Option C: Remove Expiration Field Entirely

**Not recommended** — the on-chain `FeeConfig` struct includes `expiration`, so removing it from `FeeConfigChain` would break deserialization if a contract ever stores a non-None value.

---

## 4. Recommendation

**Choose Option B: Remove TODOs, keep `expiration: None`.**

### Evidence Summary

| Evidence | Weight |
|----------|--------|
| Developer portal does not expose expiration in creation/editing UI | Strong |
| No documentation mentions expiration for FeeConfig | Strong |
| Type mismatch in toolkit (ISO 8601 comment vs on-chain `u32`) | Strong |
| Zero known user requests for expiration | Moderate |
| On-chain contract accepts `None` without issue | Strong |
| No tests validate expiration behavior end-to-end | Moderate |

### Future Implementation Path (if needed)

If a user requests expiration support in the future, the implementation path is clear:
1. Add `expiration: Option<u32>` to `FeeConfigInput` variants
2. Pass through `encode_fee_config_input()` → `FeeConfigChain`
3. The on-chain contract already handles conversion to absolute `Timestamp`
4. No encoding layer changes needed (contract handles the protobuf injection, not the toolkit)

**Note**: The toolkit does NOT encode `expiration` into the `BasicAllowance` protobuf directly — it only sets `FeeConfig.expiration` on the contract, and the contract itself injects the expiration into the allowance during `deploy_fee_grant`. This means the toolkit's encoding layer does NOT need modification for expiration support.

---

## 5. Artifacts Referenced

| Artifact | Path |
|----------|------|
| Treasury contract FeeConfig | `~/workspace/xion/contracts/contracts/treasury/src/grant.rs:13-18` |
| Treasury contract deploy flow | `~/workspace/xion/contracts/contracts/treasury/src/execute.rs:316-384` |
| Toolkit FeeConfigChain type | `src/treasury/types.rs:375-382` |
| Toolkit FeeConfigInput enum | `src/treasury/types.rs:431-450` |
| Toolkit TODO #1 | `src/treasury/manager.rs:1939` |
| Toolkit TODO #2 | `src/treasury/api_client.rs:1659` |
| Toolkit encoding comment | `src/treasury/encoding.rs:277` |
| Portal FeeConfig type | `~/workspace/xion/xion-developer-portal/src/lib/types.ts:48-52` |
| Portal AllowanceEncoder (no expiration field) | `~/workspace/xion/xion-developer-portal/src/components/Treasury/NewTreasury/AllowanceEncoder.tsx` |
| Portal CLAUDE.md (omits expiration) | `~/workspace/xion/xion-developer-portal/CLAUDE.md:134-173` |
