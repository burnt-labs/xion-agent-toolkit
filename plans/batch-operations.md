---
status: Todo
created_at: 2026-03-13
updated_at: 2026-03-13
---

# Batch Operations

## Background

Developer Portal supports batch transactions for:
- Asset deployment (multiple instantiate messages)
- Treasury configuration (grant + fee config in one tx)

## Goal

Support executing multiple messages in a single transaction via CLI.

## API Design

```bash
# Execute batch from JSON file
xion-toolkit batch execute --from-file batch.json

# batch.json format
{
  "messages": [
    {
      "type": "execute",
      "contract": "xion1...",
      "msg": { "transfer": { ... } }
    },
    {
      "type": "instantiate",
      "code_id": 1260,
      "label": "...",
      "msg": { ... }
    }
  ]
}
```

## Research Required

### OAuth2 API Compatibility

**Critical**: Must verify if OAuth2 API `/api/v1/transaction` supports multiple messages.

Reference: `~/workspace/xion/oauth2-api-service/src/routes/api/transaction/broadcast.ts`

Questions to answer:
1. Does the API accept an array of messages?
2. What is the message size limit?
3. How are partial failures handled?

### Implementation Phases

**Phase 1: Research**
- Verify OAuth2 API batch support
- Check message size limits
- Test with Developer Portal patterns

**Phase 2a: If API supports batches**
- Add `src/batch/` module
- Add `batch execute` CLI command
- Support JSON input format

**Phase 2b: If API doesn't support batches**
- Document limitation
- Consider sequential execution with rollback
- Or defer feature

## Type Definitions

```rust
// src/batch/types.rs

#[derive(Debug, Clone, Deserialize)]
pub struct BatchRequest {
    pub messages: Vec<BatchMessage>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum BatchMessage {
    #[serde(rename = "execute")]
    Execute {
        contract: String,
        msg: serde_json::Value,
        funds: Option<Vec<Coin>>,
    },
    #[serde(rename = "instantiate")]
    Instantiate {
        code_id: u64,
        label: String,
        msg: serde_json::Value,
        admin: Option<String>,
    },
    #[serde(rename = "instantiate2")]
    Instantiate2 {
        code_id: u64,
        label: String,
        msg: serde_json::Value,
        salt: String,
        admin: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchResult {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub results: Vec<MessageResult>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageResult {
    pub index: usize,
    pub success: bool,
    pub error: Option<String>,
}
```

## Files to Create/Modify

```
src/
├── batch/
│   ├── mod.rs          # NEW
│   ├── executor.rs     # NEW
│   └── types.rs        # NEW
├── cli/
│   ├── batch.rs        # NEW
│   └── mod.rs          # MODIFY
```

## Acceptance Criteria

- [ ] OAuth2 API batch capability documented
- [ ] If supported: batch execute command works
- [ ] Proper error handling for partial failures
- [ ] Transaction hash returned for all messages
- [ ] Documentation with example batch files

## Dependencies

- No new dependencies required
- Uses existing OAuth2 API client

## Sign-off

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-13 | @project-manager | Plan created | Todo |
