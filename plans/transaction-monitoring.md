---
status: Todo
created_at: 2026-03-15
updated_at: 2026-03-15
---

# Transaction Monitoring

## Background

After submitting transactions, users have no way to check transaction status or wait for confirmation within the CLI. This is essential for automation scripts and CI/CD pipelines.

## Goal

Add transaction status tracking and waiting capabilities with configurable timeouts.

## Approach

1. Add `tx` command group with `status` and `wait` subcommands
2. Query transaction status from RPC endpoint
3. Implement polling loop with configurable parameters
4. Return structured status information

---

## Tasks

### 1. Transaction Types
- [ ] Create `src/tx/` module directory
- [ ] Create `src/tx/mod.rs` with module exports
- [ ] Create `src/tx/types.rs` with:
  - `TxStatus` enum: `Pending`, `Success`, `Failed`, `Timeout`
  - `TxInfo` struct: `tx_hash`, `status`, `height`, `timestamp`, `gas_used`, `error` (optional)
  - `TxWaitResult` struct: `status`, `tx_info` (optional), `wait_time_ms`

### 2. Transaction Client
- [ ] Create `src/tx/client.rs` with `TxClient`
- [ ] Implement `get_tx(hash: &str) -> Result<Option<TxInfo>>`
  - Query RPC endpoint: `{rpc_url}/tx?hash=0x{hash}`
  - Parse response into `TxInfo`
  - Handle "not found" case (pending transaction)
- [ ] Implement `wait_tx(hash: &str, timeout: u64, interval: u64) -> Result<TxWaitResult>`
  - Poll `get_tx` until success/failure or timeout
  - Use tokio::time for async waiting
  - Return timeout status if exceeded

### 3. CLI Commands
- [ ] Create `src/cli/tx.rs` with command handlers
- [ ] Add `TxCommands` enum to `src/cli/mod.rs`:
  ```rust
  enum TxCommands {
      Status {
          hash: String,
      },
      Wait {
          hash: String,
          #[arg(long, default_value = "60")]
          timeout: u64,  // seconds
          #[arg(long, default_value = "2")]
          interval: u64, // seconds
      },
  }
  ```
- [ ] Implement `handle_status` command
- [ ] Implement `handle_wait` command with progress indicator (stderr)

### 4. Output Formatting
- [ ] JSON output for `tx status`:
  ```json
  {
    "tx_hash": "ABC123...",
    "status": "success",
    "height": 123456,
    "timestamp": "2026-03-15T10:00:00Z",
    "gas_used": 150000
  }
  ```
- [ ] JSON output for `tx wait`:
  ```json
  {
    "status": "success",
    "wait_time_ms": 4500,
    "tx_info": { ... }
  }
  ```
- [ ] Human output with colored status indicators

### 5. Skills Integration
- [ ] Add `skills/xion-tx/` directory (optional, or extend existing skill)
- [ ] Create `wait-tx.sh` script wrapping CLI command
- [ ] Update `skills/xion-dev/SKILL.md` with tx monitoring reference

### 6. Testing
- [ ] Unit tests for `TxStatus` parsing
- [ ] Unit tests for timeout logic (use mock time)
- [ ] Integration test with mock RPC server
- [ ] E2E test: submit tx and wait for confirmation

---

## Command Examples

### tx status
```bash
# Check transaction status
xion-toolkit tx status ABC123DEF456 --output json

# Output (success):
{
  "tx_hash": "ABC123DEF456...",
  "status": "success",
  "height": 12345678,
  "timestamp": "2026-03-15T10:00:00Z",
  "gas_used": 150000
}

# Output (pending):
{
  "tx_hash": "ABC123DEF456...",
  "status": "pending"
}

# Output (failed):
{
  "tx_hash": "ABC123DEF456...",
  "status": "failed",
  "error": "insufficient funds"
}
```

### tx wait
```bash
# Wait for transaction with defaults (60s timeout, 2s interval)
xion-toolkit tx wait ABC123DEF456 --output json

# Custom timeout and interval
xion-toolkit tx wait ABC123DEF456 --timeout 120 --interval 5 --output json

# Output (success):
{
  "status": "success",
  "wait_time_ms": 4500,
  "tx_info": {
    "tx_hash": "ABC123DEF456...",
    "status": "success",
    "height": 12345678,
    "timestamp": "2026-03-15T10:00:00Z",
    "gas_used": 150000
  }
}

# Output (timeout):
{
  "status": "timeout",
  "wait_time_ms": 60000
}
```

---

## RPC Query Details

### Transaction Query
```
GET {rpc_url}/tx?hash=0x{tx_hash}

Response (success):
{
  "tx_result": {
    "height": "12345678",
    "result": {
      "code": 0,
      "gas_used": "150000"
    }
  }
}

Response (not found - pending):
HTTP 404 or empty response
```

---

## Dependencies

- Existing `src/api/` module for HTTP client
- `src/config/` for network RPC endpoint
- `tokio` for async (already in dependencies)
- `serde_json` for parsing

---

## Acceptance Criteria

- [ ] `tx status <hash>` returns current transaction state
- [ ] `tx wait <hash>` polls until confirmation or timeout
- [ ] Supports both testnet and mainnet via `--network` flag
- [ ] Exit code 0 for success, 1 for failure/timeout
- [ ] JSON output is parseable by jq
- [ ] All tests pass

---

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
