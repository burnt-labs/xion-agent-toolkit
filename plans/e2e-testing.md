---
status: InProgress
created_at: 2026-03-07
updated_at: 2026-03-07
---

# E2E Testing Plan

## Background

With all treasury CLI commands implemented and 330+ unit tests passing, we need to perform end-to-end testing against the Xion testnet to verify the complete workflow.

## Goal

Validate all treasury operations work correctly with real OAuth2 API and testnet.

## Prerequisites

1. **Build the toolkit**: `cargo build --release`
2. **Login to testnet**: `./target/release/xion-toolkit auth login --network testnet`
3. **Get testnet tokens**: Use the Xion faucet

## Test Scenarios

### 1. Authentication Flow
```bash
# Check auth status
./target/release/xion-toolkit auth status --network testnet

# View configuration
./target/release/xion-toolkit config show --network testnet
```

### 2. Treasury List (DaoDao Indexer)
```bash
# List all treasuries
./target/release/xion-toolkit treasury list --network testnet --output json
```

### 3. Treasury Create
```bash
# Create basic treasury
./target/release/xion-toolkit treasury create \
  --network testnet \
  --name "Test Treasury" \
  --redirect-url "https://example.com/callback" \
  --icon-url "https://example.com/icon.png" \
  --output json

# Create with fee grant
./target/release/xion-toolkit treasury create \
  --network testnet \
  --name "Test with Fee Grant" \
  --fee-allowance-type basic \
  --fee-spend-limit "1000000uxion" \
  --output json

# Create with grant config
./target/release/xion-toolkit treasury create \
  --network testnet \
  --name "Test with Grant" \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --output json
```

### 4. Treasury Query
```bash
# Query specific treasury
./target/release/xion-toolkit treasury query xion1... --network testnet --output json
```

### 5. Grant Config (Authz)
```bash
# Using presets (recommended)
./target/release/xion-toolkit treasury grant-config xion1... \
  --network testnet \
  --preset send \
  --grant-spend-limit "1000000uxion" \
  --output json

./target/release/xion-toolkit treasury grant-config xion1... \
  --network testnet \
  --preset execute \
  --grant-max-calls 100 \
  --output json

# Manual configuration
./target/release/xion-toolkit treasury grant-config xion1... \
  --network testnet \
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion" \
  --output json
```

### 6. Fee Config
```bash
# Basic allowance
./target/release/xion-toolkit treasury fee-config xion1... \
  --network testnet \
  --fee-allowance-type basic \
  --fee-spend-limit "1000000uxion" \
  --output json

# Periodic allowance
./target/release/xion-toolkit treasury fee-config xion1... \
  --network testnet \
  --fee-allowance-type periodic \
  --fee-period-seconds 86400 \
  --fee-period-spend-limit "100000uxion" \
  --output json
```

### 7. Token Refresh
```bash
# Manually refresh token
./target/release/xion-toolkit auth refresh --network testnet --output json
```

## Automated Test Script

Run the E2E test script after logging in:

```bash
# First login
./target/release/xion-toolkit auth login --network testnet

# Then run E2E tests
./scripts/e2e-test.sh

# Or skip treasury creation (if you don't have tokens)
./scripts/e2e-test.sh --skip-create
```

## Test Checklist

- [ ] Authentication status shows correct info
- [ ] Configuration display works
- [ ] Treasury list returns valid JSON
- [ ] Treasury create works (requires testnet tokens)
- [ ] Treasury query returns details
- [ ] Grant config with presets works
- [ ] Fee config works
- [ ] Token refresh works
- [ ] DaoDao Indexer returns correct format

## Known Limitations

1. **Treasury Create**: Requires testnet tokens for gas fees
2. **OAuth2 Login**: Requires browser interaction
3. **DaoDao Indexer**: May have slight delay for newly created treasuries

## Troubleshooting

### "Not authenticated"
```bash
./target/release/xion-toolkit auth login --network testnet
```

### "Insufficient balance"
Get tokens from the Xion testnet faucet.

### "Token expired"
```bash
./target/release/xion-toolkit auth refresh --network testnet
```

### "Treasury not found"
Wait a few seconds for DaoDao Indexer to sync, then retry.

## Sign-off

| Date | Content | Status |
|------|---------|--------|
| 2026-03-07 | E2E test plan created | ✅ |
| 2026-03-07 | E2E test script created | ✅ |
| 2026-03-07 | Manual E2E testing | 🔄 |
