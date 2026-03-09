---
status: Todo
created_at: 2026-03-09
updated_at: 2026-03-09
---

# Treasury E2E Testing

## Background

After completing Treasury Enhancements and Documentation Update, we need comprehensive E2E tests that cover the full treasury lifecycle from creation to withdrawal. This will validate all CLI commands work correctly in a real environment.

## Goal

Create comprehensive E2E test script that covers the complete treasury lifecycle:
1. Create → Fund → Configure → Manage → Query → Withdraw

## Approach

### Test Flow

```
1. Pre-flight Check
   - Verify CLI is built
   - Check authentication status
   - If not authenticated, prompt for login

2. Treasury Creation
   - Create new treasury with basic config
   - Wait for indexing
   - Verify treasury appears in list

3. Funding
   - Fund treasury with test amount
   - Verify balance updated

4. Grant Configuration
   - Add grant config for MsgSend
   - List grant configs
   - Verify grant config appears

5. Fee Configuration
   - Set basic fee allowance
   - Query fee config
   - Verify fee config is set

6. Admin Management (Optional - requires second account)
   - Propose new admin
   - Cancel proposal
   - Or accept with second account

7. Params Update
   - Update treasury params (redirect_url, etc.)
   - Query treasury to verify update

8. Chain Query
   - List authz grants via chain query
   - List fee allowances via chain query

9. Withdraw
   - Withdraw remaining funds
   - Verify balance updated

10. Cleanup (Optional)
    - Archive test treasury or leave for reference
```

### Test Script Structure

```bash
tests/
├── archived/           # Old test files
│   ├── test-auth.sh
│   └── test_create_debug.rs
├── treasury_integration_test.rs      # Mock API tests (keep)
├── treasury_create_integration_test.rs # Mock API tests (keep)
└── e2e_treasury_lifecycle.sh         # New E2E test
```

## Tasks

### Setup
- [ ] Create `tests/archived/` directory
- [ ] Move `test-auth.sh` to archived
- [ ] Move `test_create_debug.rs` to archived

### E2E Test Script
- [ ] Create `tests/e2e_treasury_lifecycle.sh`
- [ ] Add pre-flight checks (CLI built, auth status)
- [ ] Add treasury creation test
- [ ] Add fund test
- [ ] Add grant-config test
- [ ] Add fee-config test
- [ ] Add admin management test (optional, requires 2nd account)
- [ ] Add params update test
- [ ] Add chain-query tests
- [ ] Add withdraw test
- [ ] Add summary report at end

### Documentation
- [ ] Add E2E test instructions to CONTRIBUTING.md
- [ ] Document how to run E2E tests

## Acceptance Criteria

- [ ] E2E script runs successfully on testnet
- [ ] All treasury operations are tested
- [ ] Clear pass/fail output
- [ ] Old tests archived but preserved
- [ ] Script handles errors gracefully

## Running the Test

```bash
# Build CLI first
cargo build --release

# Run E2E test (requires authentication)
./tests/e2e_treasury_lifecycle.sh

# Or with specific network
./tests/e2e_treasury_lifecycle.sh --network testnet
```

## Expected Output

```
================================
Treasury E2E Lifecycle Test
================================

[1/10] Pre-flight Check...      ✓ PASS
[2/10] Treasury Create...       ✓ PASS (tx: ABC123...)
[3/10] Treasury Fund...         ✓ PASS (balance: 1000000uxion)
[4/10] Grant Config...          ✓ PASS
[5/10] Fee Config...            ✓ PASS
[6/10] Admin Management...      ⊘ SKIP (requires 2nd account)
[7/10] Params Update...         ✓ PASS
[8/10] Chain Query Grants...    ✓ PASS (1 grant found)
[9/10] Chain Query Allowances...✓ PASS (1 allowance found)
[10/10] Withdraw...             ✓ PASS

================================
Results: 9 PASS, 0 FAIL, 1 SKIP
================================
```

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
