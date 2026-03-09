---
status: Done
created_at: 2026-03-09
updated_at: 2026-03-09
done_at: 2026-03-09
---

# Generic Contract Instantiation

## Background

The `execute_contract` function has been generalized as `broadcast_execute_contract<T: Serialize>`, which accepts any serializable message type. However, contract instantiation methods (`MsgInstantiateContract` and `MsgInstantiateContract2`) are still implemented as specific methods tied to treasury creation.

## Goal

1. Create generic `broadcast_instantiate_contract<T: Serialize>` for `MsgInstantiateContract` (v1)
2. Create generic `broadcast_instantiate_contract2<T: Serialize>` for `MsgInstantiateContract2` (v2, predictable addresses)
3. Refactor `create_treasury` to use the generic `broadcast_instantiate_contract2` method

## Approach

Follow the same pattern as `broadcast_execute_contract<T: Serialize>`:
- Accept any serializable instantiate message type
- Serialize to JSON bytes, then convert to number array
- Construct the appropriate message type with raw JSON object format

## Tasks

- [x] Add generic `broadcast_instantiate_contract<T: Serialize>` method
- [x] Add generic `broadcast_instantiate_contract2<T: Serialize>` method
- [x] Refactor `create_treasury` to use `broadcast_instantiate_contract2`
- [x] Add unit tests for generic methods
- [x] Run all tests to ensure no regression

## Acceptance Criteria

- [x] Generic instantiate methods follow the same pattern as `broadcast_execute_contract`
- [x] `create_treasury` uses the generic method internally
- [x] All existing tests pass
- [x] New unit tests for generic methods added

## Sign-off

> Only @qa-engineer or @project-manager may sign off completion.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-09 | @qa-engineer | Verified: All tests pass, clippy passes, fmt passes. Generic methods implemented correctly. | Approved |
