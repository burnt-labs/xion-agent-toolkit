# AGENTS.md — harness (`.mstar/`)

Rules for the harness subtree only. Rust CLI build, test, and security rules live in the repository root `AGENTS.md`.

## Path symbols (this repository)

| Symbol | Path |
|--------|------|
| `{HARNESS_DIR}` | `.mstar/` |
| `{PLAN_DIR}` | `.mstar/plans/` |
| `{KNOWLEDGE_DIR}` | `.mstar/knowledge/` |
| `{ITERATION_DIR}` | `.mstar/iterations/` |
| `{SPECS_DIR}` | `specs/` if present, else `designs/` if present |

Human-facing product docs stay in `docs/`. Harness state, plans, knowledge, and QC reports stay under `{HARNESS_DIR}`.

## Layout

| Location | Purpose |
|----------|---------|
| `docs/` | Install, CLI reference, error codes, release process |
| `{HARNESS_DIR}/status.json` | Plan rows, roadmap pointers, `residual_findings` SSOT |
| `{PLAN_DIR}/` | Active implementation plans |
| `{PLAN_DIR}/archived/` | Completed plans (`archived/README.md` index) |
| `{PLAN_DIR}/reports/<plan-id>/` | QC / review reports (not `docs/`) |
| `{PLAN_DIR}/residuals/<plan-id>/` | Optional long-form open residual notes |
| `{KNOWLEDGE_DIR}/` | Durable design notes (`README.md` index) |
| `{ITERATION_DIR}/` | Iteration compasses (`README.md` index) |

Do not store dynamic plan progress in root `AGENTS.md` or `CLAUDE.md`.

## Status and roadmap

- Read and update `{HARNESS_DIR}/status.json` for active plans, archived index, knowledge index, and phase summary.
- Active roadmap: `status.json` → `active_roadmap` (under `{KNOWLEDGE_DIR}/`).
- Per-plan assignment templates: colocate with the plan; archive alongside it when the plan is `Done` (see `{PLAN_DIR}/archived/`).
- Register new knowledge in `{KNOWLEDGE_DIR}/README.md` and `status.json` `knowledge[]`.

## Working on harness artifacts

1. Confirm plan status in `status.json` before large edits.
2. Put QC conclusions under `{PLAN_DIR}/reports/<plan-id>/`.
3. Move completed main plans to `{PLAN_DIR}/archived/` and update `archived/README.md`.
4. Open residual findings: root `residual_findings[<plan-id>]` only (no dual-write to plan metadata).

## Git

Commit harness changes with the plan or docs scope (for example `docs(plan): ...`). Do not git-ignore the whole `{HARNESS_DIR}`; clones need handoff artifacts.
