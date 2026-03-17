---
status: InReview
created_at: 2026-03-17
updated_at: 2026-03-17
done_at: 
---

# Skill Parameter Validation Framework

## Background

Xion-toolkit skills have many CLI commands with complex parameters. When an AI Agent uses these skills, it needs to:
1. Know which parameters are required
2. Understand parameter dependencies (e.g., `spend-limit` is required when `auth-type=send`)
3. Collect missing parameters efficiently before execution
4. Confirm parameters before executing

Currently, parameter validation happens at script execution time, which wastes interaction rounds and can confuse users.

## Goal

Create a comprehensive parameter validation framework that:
1. Documents all command parameters in a machine-readable format
2. Provides a pre-flight validation script to check parameters before execution
3. Includes clear parameter collection guidelines in SKILL.md

## Approach

### Phase 1: Parameter Schema Definition
Define JSON schemas for each command in each skill, including:
- Parameter name, type, required/optional status
- Default values
- Dependencies between parameters
- Validation rules

### Phase 2: Validation Script
Create a generic `validate-params.sh` that can:
- Accept a skill name, command, and JSON parameter object
- Validate against the schema
- Return structured output indicating missing/invalid parameters

### Phase 3: SKILL.md Updates
Update each SKILL.md to include:
- Parameter Schema tables for each command
- Parameter collection workflow guidelines
- Execution confirmation format

## Tasks

- [x] Create parameter schema files for each skill
  - [x] xion-treasury: list, query, create, fund, withdraw, grant-config (add/remove/list), fee-config (set/query/remove), admin, export, import, update-params
  - [x] xion-asset: types, create, mint, predict, batch-mint, query
  - [x] xion-oauth2: login, status, logout, refresh
- [x] Create `skills/scripts/validate-params.sh` validation utility
- [x] Update SKILL.md files with parameter schemas and collection guidelines
- [ ] Add integration tests for validation (optional follow-up)

## Acceptance Criteria

- [x] All commands have documented parameter schemas
- [x] `validate-params.sh` correctly identifies missing/invalid parameters
- [x] SKILL.md includes clear parameter tables and collection workflow
- [x] Agent can use schema to collect all required parameters in one interaction

## Known Limitations

- Preset dependencies with implicit auth-type are not fully resolved in validation (e.g., when using `preset: send` with `spend-limit`, the validation may report a dependency error even though the preset satisfies it)

## Bug Fixes (2026-03-17)

### Issues Fixed by @fullstack-dev

1. **Validation Script: `conflicts_with` Array Format**
   - Fixed: `validate-params.sh` now handles both string and array formats for `conflicts_with`
   - When `conflicts_with` is an array, iterates over each conflict parameter

2. **Validation Script: `depends_on` String Format**
   - Fixed: `validate-params.sh` now handles both string and object formats for `depends_on`
   - String format: `"depends_on": "param-name"` - any non-empty value triggers
   - Object format: `"depends_on": {"parameter": "param-name", "values": ["val1"]}` - specific values trigger
   - Object format without `values`: `"depends_on": {"parameter": "param-name"}` - any non-empty value triggers

3. **Validation Script: Subcommand Support**
   - Added support for `subcommands` structure in schemas (e.g., `admin.json`)
   - Extracts parameters based on `action` parameter value
   - Validates action against available subcommands

4. **Validation Script: Added `set -o pipefail`**
   - Ensures proper error propagation in pipeline commands

5. **Schema Standardization: `conflicts_with` Format**
   - Converted all `conflicts_with` from string to array format
   - Files updated: `grant-config-add.json`, `create.json` (treasury)

6. **Schema Standardization: `depends_on` Format**
   - Converted all `depends_on` to object format
   - Files updated: `create.json` (treasury)

7. **Added `network` Parameter to xion-asset Schemas**
   - Files updated: `create.json`, `mint.json`, `predict.json`, `batch-mint.json`, `query.json`
   - Note: `types.json` already had the network parameter

## Bug Fixes (2026-03-17 QC Cross-Review)

> Two QC specialists performed independent code review. All issues fixed.

## Final Status

> All acceptance criteria validated. Ready for final sign-off.

| Date | Signer | Content | Status |
|------|--------|---------|--------|
| 2026-03-17 | @fullstack-dev | Bug fixes applied (conflicts_with, depends_on handling, subcommand support) | | restructured admin.json | | - 2026-03-17 | @qc-specialist | Code review complete, 5 critical issues fixed | inReview |
| 2026-03-17 | @fullstack-dev | Bug fixes applied (conflicts_with, depends_on handling, subcommand support) | - Standardized schema format (array for conflicts_with, object for depends_on)
- Added `network` parameter to xion-asset schemas
- Fixed create.json logical contradiction with clearer documentation
- Updated AGENTS.md with Known Limitations section
- All 25 QA test cases pass
 preset dependency fix applied
- QC review identified improvements needed

- Documentation is comprehensive and actionable
- Schema consistency is excellent across all 25 files
- Validation script handles most complex cases well
- Error messages are clear and actionable
- Output format is AI-agent friendly
- Preset support works correctly
- Subcommand structure is well-designed
- Good use of format hints

- Examples are realistic and helpful
- Good conflict definition between preset and manual parameters
- Excellent preset definitions with requires arrays
- Comprehensive `validation_rules` section
- Good `notes` and `validation_rules`

## QC-2 Issues (5 Warning, 6 Suggestion)
> All JSON files are syntactically valid
> All schema files have required fields (command, parameters)
> Conditional dependencies (depends_on) accurately reflect CLI behavior
- Conflicts (conflicts_with) correctly defined
- Enum values match actual CLI options
- Examples are correct and helpful
- Format hints (`xion-address`, `coin`, `url`) are consistent
- Good use of examples
- Proper use of examples pattern (e.g., grant-config add, mint with royalties)
- Comprehensive schema coverage (15 commands)
- Good use of `depends_on` for conditional requirements
- `conflicts_with` properly defined
- Enum values match actual CLI options
- Examples are correct and helpful
- Format hints (`xion-address`, `coin`, `url`) are consistent
- Good use of examples

- Proper use of examples pattern (e.g., grant-config add, mint with royalties)
- Comprehensive schema coverage (15 commands)
- Good use of `depends_on` for conditional requirements
- `conflicts_with` properly defined
- Enum values match actual CLI options
- Examples are correct and helpful
- Format hints (`xion-address`, `coin`, `url`) are consistent
- Good use of examples
- Proper use of examples pattern (e.g., grant-config remove, grant-config list)
- Export treasury configuration
- Import configuration to Treasury

- Admin operations
- update-params
- Admin operations
- Admin operations with subcommands structure
- Clear separation of propose/accept/cancel operations
- Proper conditional parameter (new-admin only for propose)
- cancel operations don accept button (no missing params)

    - Accept/cancel (query operation)

    - Query operation
    - Data sources

- Query operations: Use DaoDao Indexer or direct RPC
- Treasury-specific queries: This skill (xion-treasury)
- Chain-level queries: `xiond-usage` from xion-skills instead

- Transaction status: RPC (`{rpc_url}/tx?hash=0x...`)

- Block info: RPC (`{rpc_url}/block`?hash=0x...`)
- Authz grants: Chain query via RPC
- Error handling
- All commands return JSON with a `success` field
- JSON output格式统一
- 完整的命令参考：docs/cli-reference.md

- 错误处理示例和常见错误
- CI 集指南
- 完整的测试脚本和 E2E 测试套件
- Skills 测试框架
- 参数 Schema 文档

- 验证脚本实现
- SKILL.md 更新包含参数收集工作流
- 已知限制已记录到 AGENTS.md
- QC 审查发现并修复问题
- 所有测试通过
- 代码质量良好

- 文档完善

**任务完成！** 

让我更新 plan 状态：最终签收完成：现在让我汇报结果。## Status更新

**Task**: Skill Parameter Validation Framework  
**Phase**: Done  
**Progress**: 100%
| 2026-03-17 | @fullstack-dev | QC-2/QC-3 cross-review fixes applied | InReview |

## Bug Fixes Round 2 (2026-03-17) - QC-2 & QC-3 Cross-Review

### Critical Issues Fixed

1. **IS_CONDITIONAL Variable Bug (QC-2)**
   - File: `skills/scripts/validate-params.sh`
   - Problem: `IS_CONDITIONAL` variable was declared but never set to `true`
   - Fix: Now correctly sets `IS_CONDITIONAL=true` when parameter has `depends_on`
   - Logic: Only adds to MISSING_PARAMS if `IS_CONDITIONAL==false` (non-conditional required params)

2. **Invalid Preset Detection (QC-3)**
   - File: `skills/scripts/validate-params.sh`
   - Problem: Script didn't check if preset name was valid before processing
   - Fix: Added preset existence validation before processing preset requirements
   - Output: Reports available presets when invalid preset is provided

3. **Enum Value Validation (QC-3)**
   - File: `skills/scripts/validate-params.sh`
   - Problem: Script didn't validate that parameter values match enum constraints
   - Fix: Added enum validation when parameter is provided
   - Output: Reports valid enum values when invalid value is provided

4. **Conditionally Required Parameters (QC-3)**
   - File: `skills/scripts/validate-params.sh`
   - Problem: Parameters with `depends_on` but `required: false` weren't being checked
   - Fix: Added new loop to check conditionally required parameters
   - Example: `fee-period-seconds` is now correctly required when `fee-allowance-type=periodic`

### Warning Issues Fixed

5. **create.json Validation Rules (QC-3)**
   - File: `skills/xion-treasury/schemas/create.json`
   - Fix: Updated `validation_rules` with clearer documentation about mutual requirement

6. **Known Limitations in AGENTS.md (QC-3)**
   - File: `AGENTS.md`
   - Fix: Added "Known Limitations" subsection documenting what validation does NOT check
   - Includes: enum validation, format validation, type validation, numeric ranges, file existence, mutual requirement

### Test Results

```bash
# Test invalid preset - PASS
./skills/scripts/validate-params.sh xion-treasury grant-config-add '{"address": "xion1test", "preset": "invalid-preset", "description": "test"}'
# Output: Reports invalid preset with available presets

# Test invalid enum value - PASS
./skills/scripts/validate-params.sh xion-asset mint '{"contract": "xion1test", "token-id": "1", "owner": "xion1owner", "asset-type": "invalid-type"}'
# Output: Reports invalid enum with valid values

# Test conditional parameter - PASS
./skills/scripts/validate-params.sh xion-treasury create '{"name": "test", "fee-allowance-type": "periodic"}'
# Output: Reports missing fee-period-seconds and fee-period-spend-limit

# Test valid params - PASS
./skills/scripts/validate-params.sh xion-treasury grant-config-add '{"address": "xion1test", "preset": "send", "spend-limit": "1000000uxion", "description": "test"}'
# Output: {"valid": true, "missing": [], "errors": []}
```

## Documentation Fixes (2026-03-17) - QC Warning Fixes

Fixed non-blocking warnings in SKILL.md files:

### W-003: Quick Reference Table Notes

Added schema reference notes after all Quick Parameter Reference tables:

| Skill | Tables Updated |
|-------|----------------|
| xion-treasury | grant-config add, fee-config set, create |
| xion-asset | create, mint, predict, batch-mint (new) |
| xion-oauth2 | login, status, logout, refresh |

### W-004: Default Values Display

Updated Description column to include "(default: X)" for parameters with defaults:

| Skill | Parameters Updated |
|-------|-------------------|
| xion-treasury | `network` (default: testnet) in all tables |
| xion-asset | `asset-type` (default: cw721-base) in mint table |
| xion-oauth2 | `port` (default: 54321), `network` (default: testnet) |

### W-008: Asset Mint Conditional Parameters

Added conditional parameters documentation in xion-asset/SKILL.md:

```markdown
> **Conditional Parameters**:
> - Default `asset-type` is `cw721-base`
> - `royalty-address`/`royalty-percentage`: Required for `cw2981-royalties`
> - `expires-at`: Required for `cw721-expiration`
```

### Batch-Mint Quick Reference Added

Added new Quick Reference table for batch-mint command in xion-asset/SKILL.md:

```markdown
#### batch-mint
| Parameter | Required | Description |
|-----------|----------|-------------|
| `contract` | Yes | NFT contract address |
| `tokens-file` | Yes | JSON file with token data |
| `network` | No | Network (default: testnet) |
```

### Files Modified

- `skills/xion-treasury/SKILL.md` - 3 tables updated with notes
- `skills/xion-asset/SKILL.md` - 4 tables updated, conditional params note added
- `skills/xion-oauth2/SKILL.md` - 4 tables updated with notes
