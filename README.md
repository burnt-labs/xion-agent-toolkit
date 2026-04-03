# Xion Agent Toolkit

**Gasless Xion development toolkit for humans and AI agents.**

A command-line tool for managing Xion MetaAccounts, Treasury contracts, and NFT assets — all without handling private keys or paying gas fees.

---

## Core Features

- **Auth** — OAuth2/MetaAccount login with Google, Email, or Passkey (no private keys)
- **Treasury** — Create, fund, withdraw, and configure gasless transaction contracts
- **NFT / Asset** — Deploy CW721 collections, mint tokens, configure royalties
- **Faucet** — Claim testnet XION tokens (1 XION per claim, 24-hour cooldown)
- **Contract** — Instantiate, execute, and query CosmWasm smart contracts

---

## Installation

### Install CLI

**macOS / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
```

**Windows (PowerShell):**
```powershell
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

**From Source:**
```bash
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit
cp .env.example .env
cargo install --path .
```

### Install Skills (for AI Agents)

```bash
# 1. Install CLI first (see above)

# 2. Install skills
npx skills add burnt-labs/xion-agent-toolkit -g

# 3. Authenticate
xion-toolkit auth login
```

Available skills: `xion-dev`, `xion-toolkit-init`, `xion-oauth2`, `xion-treasury`, `xion-faucet`, `xion-asset`, `xion-oauth2-client`. See [INSTALL-FOR-AGENTS.md](./INSTALL-FOR-AGENTS.md) for full details.

### Authenticate

```bash
xion-toolkit auth login
```

Opens your browser for OAuth2 authentication. Tokens are stored encrypted locally.

---

## Quick Start

### 1. Login

```bash
xion-toolkit auth login
```

Authenticate with Google, Email, or Passkey — no seed phrases required.

### 2. Claim Testnet Tokens

```bash
xion-toolkit faucet claim
```

Receive 1 XION (1,000,000 uxion) for testing. Check status with `faucet status`.

### 3. Create a Treasury

```bash
xion-toolkit treasury create --name "My Treasury" --redirect-url "https://your-app.com/callback"
```

Creates a gasless transaction contract. Fund it with claimed tokens:

```bash
xion-toolkit treasury fund xion1... --amount 1000000uxion
```

### 4. (Optional) Create NFT Collection

```bash
xion-toolkit asset create --type cw721-base --name "My Collection" --symbol "NFT"
xion-toolkit asset mint --contract xion1... --token-id "1" --owner xion1...
```

Deploy a CW721 collection and mint your first NFT.

---

## For AI Agents

If you want your AI agent to install and use this toolkit, give it this instruction:

```
Follow this guide https://raw.githubusercontent.com/burnt-labs/xion-agent-toolkit/main/INSTALL-FOR-AGENTS.md to install and configure the Xion Agent Toolkit skills for AI agents.
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [CLI Reference](./docs/cli-reference.md) | Complete command documentation |
| [Quick Reference (AI)](./docs/QUICK-REFERENCE.md) | Condensed reference for AI agents |
| [Skills Guide](./docs/skills-guide.md) | AI agent skills usage |
| [Error Codes](./docs/ERROR-CODES.md) | Error code reference |
| [Exit Codes](./docs/EXIT-CODES.md) | CI/CD exit codes |
| [Configuration](./docs/configuration.md) | Network and config settings |
| [Install for AI Agents](./INSTALL-FOR-AGENTS.md) | Agent integration guide |
| [Contributing](./CONTRIBUTING.md) | Contribution guidelines |

---

## Security

- **No Private Keys** — OAuth2 and MetaAccount authentication only
- **PKCE (RFC 7636)** — Prevents authorization code interception
- **AES-256-GCM** — Encrypted credential storage
- **Localhost Only** — Callback server accepts localhost connections only
- **HTTPS Only** — All API communications encrypted

---

## Troubleshooting

**CLI not found after install:**

```bash
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
```

**Token expired:**

```bash
xion-toolkit auth refresh
```

**Port in use during login:**

```bash
xion-toolkit auth login --port 54322
```

---

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.

---

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Contributing Guide](./CONTRIBUTING.md)
- [xion-skills](https://github.com/burnt-labs/xion-skills) — Advanced chain operations
