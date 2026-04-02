# Archived Plans

This directory contains historical planning documents that have been completed.

## Archive History

| Phase | Date | Plans Count |
|-------|------|-------------|
| Initial Development | 2026-03-12 | 21 plans |
| Phase 1 Features | 2026-03-14 | 5 plans |
| Phase 2–3 + CLI Polish | 2026-03-15 ~ 2026-04-02 | 18 plans |
| **Total** | | **44 plans** |

---

## Phase 2–3 + CLI Polish (2026-03-15 ~ 2026-04-02)

### Ecosystem Polish & Infrastructure
| Plan | Description |
|------|-------------|
| `error-recovery-enhancement.md` | 40+ structured error codes, exponential backoff retry |
| `transaction-monitoring.md` | Tx status query + wait-for-confirmation |
| `unified-ci-workflow.md` | Merged ci/test-skills/e2e workflows into unified CI |
| `cicd-integration-output.md` | OutputFormat enum, exit_codes module, CLI output formatting |
| `skills-test-framework.md` | 48 tests, 58 mock scenarios for skill validation |
| `rest-api-url-fix.md` | Separate REST endpoint for chain queries (404 fix) |

### Treasury & Contract Features
| Plan | Description |
|------|-------------|
| `predicted-address-computation.md` | instantiate2 address prediction with `--predict` and `--salt` |
| `batch-treasury-operations.md` | Batch fund, batch grant-config, bulk export |

### OAuth2 Client Management
| Plan | Description |
|------|-------------|
| `oauth2-client-management-post-merge.md` | Full lifecycle: create/update/list/get + skills + docs |
| `oauth2-client-advanced-lifecycle.md` | `--force` safety guard, destructive op confirmation |

### Interactive CLI Mode
| Plan | Description |
|------|-------------|
| `2026-04-01-interactive-cli.md` | dialoguer prompts for missing args, `--no-interactive` flag |
| `2026-04-01-local-scope-validation.md` | Local scope validation for MGR API, `--dev-mode` flag |

### CLI & UX Improvements
| Plan | Description |
|------|-------------|
| `mainnet-disable-switch.md` | Mainnet feature disable with exit code 10 |
| `shell-completion.md` | bash/zsh/fish/powershell completion + `--install` flag |
| `faucet-command.md` | Testnet faucet claim command |
| `skill-patterns-optimization.md` | Skill design patterns, refresh-first auth |

### Quality Assurance
| Plan | Description |
|------|-------------|
| `qc-cross-review.md` | Tri-review: 12 critical + 30 warnings, all resolved |
| `skill-param-validation.md` | Parameter validation framework for 25 skill schemas |

---

## Phase 1 Features (2026-03-14)

| Plan | Description |
|------|-------------|
| `asset-builder.md` | CW721 NFT deployment, minting, address prediction |
| `batch-operations.md` | Multi-message batch transaction support |
| `metaaccount-info.md` | MetaAccount info command using OAuth2 API |
| `extended-grant-types.md` | 12 new grant presets for extended type support |
| `e2e-testing.md` | E2E testing for new modules |

---

## Initial Development Phase (2026-03-12)

### Release & Automation
| Plan | Description |
|------|-------------|
| `cargo-dist-release.md` | Cargo-dist release automation |
| `release-please-automation.md` | Release Please automation |

### Contract Operations
| Plan | Description |
|------|-------------|
| `cli-query-export-import.md` | CLI query, export & import commands |
| `contract-execute-command.md` | Contract execute command |
| `contract-instantiate-refactor.md` | Contract instantiate refactor |
| `generic-contract-instantiation-api.md` | Generic contract instantiation public API |
| `generic-contract-instantiation.md` | Generic contract instantiation |

### Treasury
| Plan | Description |
|------|-------------|
| `treasury-api-architecture.md` | Treasury API architecture |
| `treasury-automation.md` | Treasury automation plan |
| `treasury-create-enhancement.md` | Treasury create command enhancement |
| `treasury-e2e-testing.md` | Treasury E2E testing |
| `treasury-enhancements.md` | Treasury enhancements |
| `treasury-grant-fee-config.md` | Treasury grant config & fee config |
| `treasury-grant-fee-debug.md` | Treasury grant/fee config debug |

### Authentication
| Plan | Description |
|------|-------------|
| `oauth2-client-architecture.md` | OAuth2 client architecture |
| `oauth2-pkce-implementation.md` | OAuth2 PKCE implementation |

### Documentation
| Plan | Description |
|------|-------------|
| `documentation-improvement.md` | Documentation improvement |
| `documentation-update.md` | Documentation update |

### Testing & Integration
| Plan | Description |
|------|-------------|
| `integration-tests-fix.md` | Integration tests fix (wiremock) |
| `skills-integration.md` | Skills integration with xion-skills |

### Bug Fixes
| Plan | Description |
|------|-------------|
| `transaction-format-fix.md` | Transaction format fix |

---

## Current Status

See `.agents/plans/status.json` for the current project status and active plans.

## Reference Material

Ongoing reference material is maintained in `.agents/plans/knowledge/`.
