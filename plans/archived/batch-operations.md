---
status: Done
created_at: 2026-03-13
updated_at: 2026-03-13
done_at: 2026-03-13
research_completed_at: 2026-03-13
research_by: @architect
---

# Batch Operations

## Research Summary

**OAuth2 API Batch Support: ✅ FULLY SUPPORTED**

| Feature | Status | Notes |
|---------|--------|-------|
| Message arrays | ✅ Supported | `messages: [...]` with `.min(1)` |
| Multiple message types | ✅ Supported | 12 types including bank, wasm, staking |
| Raw JSON value format | ✅ Supported | Recommended for CLI |
| Atomic execution | ✅ Standard | All-or-nothing transaction |
| Partial failure | ❌ Not supported | Cosmos SDK limitation |
| API size limits | ❌ None | Chain gas limits apply |
| Gas simulation | ✅ Available | `/api/v1/transaction/simulate` |

**Recommendation: IMPLEMENT NOW**

## Goal

Support executing multiple messages in a single transaction via CLI.

## API Design

```bash
# Execute batch from JSON file
xion-toolkit batch execute --from-file batch.json [--simulate] [--memo "text"]

# Validate batch file (schema only)
xion-toolkit batch validate --from-file batch.json
```

### Input Format (`batch.json`)

```json
{
  "messages": [
    {
      "typeUrl": "/cosmos.bank.v1beta1.MsgSend",
      "value": {
        "toAddress": "xion1...",
        "amount": [{ "denom": "uxion", "amount": "1000000" }]
      }
    },
    {
      "typeUrl": "/cosmwasm.wasm.v1.MsgExecuteContract",
      "value": {
        "contract": "xion1...",
        "msg": { "some_action": {} },
        "funds": []
      }
    }
  ],
  "memo": "Optional memo"
}
```

### Output Format

```json
{
  "success": true,
  "tx_hash": "ABC123...",
  "from": "xion1...",
  "gas_used": "150000",
  "gas_wanted": "200000",
  "message_count": 2
}
```

## Implementation

### Tasks

- [x] Create `src/batch/` module (mod.rs, executor.rs, types.rs)
- [x] Implement `BatchRequest` and `BatchMessage` types
- [x] Implement `batch execute` CLI command
- [x] Add input validation (max 50 messages)
- [x] Support raw JSON message format
- [x] Add `--simulate` flag
- [x] Implement `batch validate` command
- [x] Add unit tests
- [x] Add example batch files in `examples/`
- [ ] Update documentation (deferred - CLI help is sufficient for now)

### Type Definitions

```rust
// src/batch/types.rs

#[derive(Debug, Clone, Deserialize)]
pub struct BatchRequest {
    pub messages: Vec<BatchMessage>,
    #[serde(default)]
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchMessage {
    pub type_url: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchResult {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub from: String,
    pub gas_used: Option<String>,
    pub gas_wanted: Option<String>,
    pub message_count: usize,
}
```

### Constraints

- Max 50 messages per batch (configurable)
- No partial failure support (atomic execution)
- `msg` field must be raw JSON (not base64) when using raw JSON format

### Files

```
src/
├── batch/
│   ├── mod.rs          # NEW
│   ├── executor.rs     # NEW
│   └── types.rs        # NEW
├── cli/
│   ├── batch.rs        # NEW
│   └── mod.rs          # MODIFY
examples/
└── batch-treasury-setup.json   # NEW
```

## Supported Message Types

| Type URL | CLI Shorthand (future) |
|----------|----------------------|
| `/cosmos.bank.v1beta1.MsgSend` | `send` |
| `/cosmos.bank.v1beta1.MsgMultiSend` | `multi-send` |
| `/cosmwasm.wasm.v1.MsgExecuteContract` | `execute` |
| `/cosmwasm.wasm.v1.MsgInstantiateContract` | `instantiate` |
| `/cosmwasm.wasm.v1.MsgInstantiateContract2` | `instantiate2` |
| `/cosmos.staking.v1beta1.MsgDelegate` | `delegate` |
| `/cosmos.staking.v1beta1.MsgUndelegate` | `undelegate` |
| `/cosmos.staking.v1beta1.MsgBeginRedelegate` | `redelegate` |
| `/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward` | `withdraw-rewards` |
| `/cosmos.gov.v1beta1.MsgVote` | `vote` |

## Error Handling

| Code | Description |
|------|-------------|
| `BATCH_TOO_LARGE` | Exceeds 50 messages |
| `INVALID_MESSAGE_TYPE` | Unsupported typeUrl |
| `SIMULATION_FAILED` | Pre-flight gas check failed |
| `TX_FAILED` | Transaction broadcast failed |

## Dependencies

- No new dependencies required
- Uses existing OAuth2 API client

## Acceptance Criteria

- [x] OAuth2 API batch capability documented
- [x] Batch execute command implemented
- [x] Schema validation for batch files
- [x] `--simulate` flag for dry-run
- [x] Clear documentation of atomic behavior
- [x] Example batch files in `examples/`
- [x] Unit tests pass
- [x] `cargo clippy` passes
- [x] QA verification completed

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | Done |
| 2026-03-13 | @architect | Research completed | Done |
| 2026-03-13 | @fullstack-dev | Implementation completed | Done |
| 2026-03-13 | @qc-specialist | Code review passed | Done |
| 2026-03-13 | @qa-engineer | QA verification completed | Done |
| 2026-03-13 | @project-manager | Final sign-off | Done |
