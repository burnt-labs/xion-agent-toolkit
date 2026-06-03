# Treasury Types Comparison Analysis

## Critical Mismatches Found

### 1. FeeConfigChain Missing `expiration` Field

**Official:**
```rust
pub struct FeeConfig {
    description: String,
    pub allowance: Option<Any>,
    pub expiration: Option<u32>,  // ← MISSING IN OUR IMPLEMENTATION
}
```

**Our Implementation:**
```rust
pub struct FeeConfigChain {
    pub description: String,
    pub allowance: ProtobufAny,
    // expiration field is MISSING!
}
```

**Impact:** HIGH - Fee configuration transactions will fail if expiration is needed.

### 2. TreasuryInstantiateMsg Incorrect Field Types

**Official:**
```rust
pub struct InstantiateMsg {
    pub admin: Option<Addr>,      // ← We use String, not Option<Addr>
    pub type_urls: Vec<String>,
    pub grant_configs: Vec<GrantConfig>,
    pub fee_config: FeeConfig,    // ← We have Option<FeeConfigChain>
}
```

**Our Implementation:**
```rust
pub struct TreasuryInstantiateMsg {
    pub admin: String,                     // ← Should be Option<String> for JSON
    pub params: TreasuryParamsChain,       // ← Official doesn't have this in InstantiateMsg
    pub fee_config: Option<FeeConfigChain>, // ← Should be required!
    pub grant_configs: Vec<GrantConfigChain>,
    pub type_urls: Vec<String>,
}
```

**Impact:** MEDIUM - Treasury creation might work but field order matters for some parsers.

### 3. TreasuryExecuteMsg Type Mismatches

**Official:**
```rust
pub enum ExecuteMsg {
    RevokeAllowance {
        grantee: Addr,    // ← We use String
    },
    Withdraw {
        coins: Vec<Coin>, // ← We use Vec<CoinInput>
    },
}
```

**Our Implementation:**
```rust
pub enum TreasuryExecuteMsg {
    RevokeAllowance {
        grantee: String,    // ← String is OK for JSON serialization
    },
    Withdraw {
        coins: Vec<CoinInput>, // ← Should be Vec<Coin> or verify CoinInput matches
    },
}
```

**Impact:** LOW - String vs Addr works for JSON, CoinInput needs verification.

### 4. Params Structure Missing in InstantiateMsg

**Official:** Params is in state, not in InstantiateMsg
**Our Implementation:** We have params in InstantiateMsg

**Impact:** MEDIUM - Need to verify if params should be in InstantiateMsg or not.

## Actions Required

1. ✅ Add `expiration: Option<u32>` to FeeConfigChain
2. ✅ Make FeeConfigChain.allowance Option<ProtobufAny> (matches official)
3. ✅ Verify TreasuryInstantiateMsg structure against actual usage
4. ✅ Check if CoinInput matches Coin structure
5. ✅ Run tests to verify serialization

## Field Visibility Note

Official types have private `description` fields, but our public fields work fine because:
- Serde can serialize/deserialize private fields
- JSON serialization doesn't care about Rust visibility
- The binary format is the same

## Binary Type Verification

Both use `cosmwasm_std::Binary` which handles base64 encoding/decoding automatically.