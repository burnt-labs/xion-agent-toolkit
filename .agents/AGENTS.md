# AGENTS.md — Agent skill context (`.agents/`)

Rules for how Codex, Cursor, and similar coding agents use **skills** with this repository.

## What this directory is for

The repo keeps `.agents/` as a **conventional anchor** next to the global install layout `~/.agents/skills/`. It holds **no** source code, plans, or review artifacts—only this guide for skill discovery.

Bundled skill packages live at **`skills/`** (repository root). Each package includes `SKILL.md`, optional `scripts/`, and `schemas/`.

## Install and discovery

| Step | Location |
|------|----------|
| Full install guide | `INSTALL-FOR-AGENTS.md` (repository root) |
| Skill conventions & schemas | `docs/skills-guide.md` |
| Global skill install root | `~/.agents/skills/<skill-name>/` (e.g. via `npx skills add` / `skills.sh -g`) |
| Validate frontmatter in CI | `scripts/validate-skill-frontmatter.sh` |

Install skills so **`xion-dev`** sits beside other Xion skills under the same parent directory; param validation resolves schemas from sibling skill folders.

## Bundled skills (this repo)

`xion-dev`, `xion-toolkit-init`, `xion-oauth2`, `xion-oauth2-client`, `xion-treasury`, `xion-faucet`, `xion-asset`.

Before treasury, faucet, or NFT operations: authenticate per `xion-oauth2` / `xion-toolkit-init` skill docs.

## Skill authoring rules

- Skill `description` in YAML frontmatter must stay within loader limits (Codex ≤ 1024 characters); CI enforces via `validate_skill_frontmatter`.
- Scripts invoked by skills must use `--output json` and structured stderr on failure (see root `AGENTS.md` CLI output rules).
- Colocate skill-specific scripts under `skills/<name>/scripts/`; shared helpers belong in the skill that owns them.

## Rust / CLI work in this repo

Use repository root **`AGENTS.md`** for `cargo` workflows, error codes, test serialization, and credential safety.
