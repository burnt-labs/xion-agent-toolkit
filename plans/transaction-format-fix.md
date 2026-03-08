---
status: InProgress
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

## Tasks

- [ ] Test treasury create with detailed logging
- [ ] Analyze successful transaction format
- [ ] Extract common broadcast method
- [ ] Update grant config operations
- [ ] Update fee config operations
- [ ] Run E2E tests
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
