# Xion Agent Toolkit

**Build on Xion with gasless transactions.**

A command-line tool for managing Xion MetaAccounts, Treasury contracts, and NFT assets — all without handling private keys or paying gas fees.

---

## What You Can Do

- **🔐 Login with Google, Email, or Passkey** — No seed phrases, no private keys
- **💰 Manage Treasuries** — Create, fund, withdraw, configure permissions
- **🎨 Create NFTs** — Deploy and mint CW721 collections
- **💧 Claim Testnet Tokens** — Get testnet XION from the faucet
- **🔮 Predict Addresses** — Know contract addresses before deployment
- **📦 Batch Operations** — Manage multiple treasuries at once
- **🔄 CI/CD Ready** — JSON output, exit codes, GitHub Actions support

---

## Installation

### Quick Install (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
```

### Windows (PowerShell)

```powershell
powershell -c "irm https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1 | iex"
```

### From Source

```bash
git clone https://github.com/burnt-labs/xion-agent-toolkit
cd xion-agent-toolkit

# Required for local builds: set up environment variables
# This file contains OAuth2 client IDs needed for compilation
cp .env.example .env

cargo install --path .
```

---

## Quick Start

### 1. Login

```bash
xion-toolkit auth login
```

Opens your browser for authentication. Supports Google, Email, Passkey, and more.

### 2. Check Status

```bash
xion-toolkit status
```

```json
{
  "success": true,
  "network": "testnet",
  "authenticated": true,
  "xion_address": "xion1abc123..."
}
```

### 3. Claim Testnet Tokens

```bash
# Check if you can claim (recommended before claiming)
xion-toolkit faucet status

# Claim tokens for yourself
xion-toolkit faucet claim

# Claim tokens for another address
xion-toolkit faucet claim --receiver xion1abc123...
```

Each claim provides 1 XION (1,000,000 uxion) with a 24-hour cooldown.

### 4. Create a Treasury

```bash
# Create a new treasury
xion-toolkit treasury create --redirect-url "https://..." --name "My Treasury"

# Or predict the address first (without deploying)
xion-toolkit treasury create --predict --salt "my-treasury-v1"

# Then fund it separately
xion-toolkit treasury fund xion1... --amount 1000000uxion
```

### 5. Manage Your Treasury

```bash
# List all treasuries
xion-toolkit treasury list

# Fund a treasury
xion-toolkit treasury fund xion1... --amount 1000000uxion

# Withdraw funds
xion-toolkit treasury withdraw xion1... --amount 500000uxion --to xion1recipient...

# Export for backup
xion-toolkit treasury export --output backup.json
```

### 6. Create an NFT Collection

```bash
# List available NFT types
xion-toolkit asset types

# Create a collection
xion-toolkit asset create --type cw721-base --name "My Collection" --symbol "NFT"

# Mint tokens
xion-toolkit asset mint --contract xion1... --token-id "1" --owner xion1...
```

---

## Common Tasks

### Configure Permissions (Grants)

Allow your treasury to perform specific actions:

```bash
# Allow sending funds (up to 1 XION)
xion-toolkit treasury grant-config add xion1... \
  --type-url "/cosmos.bank.v1beta1.MsgSend" \
  --auth-type send \
  --spend-limit "1000000uxion"

# Or use preset shortcuts
xion-toolkit treasury grant-config add xion1... --preset send
xion-toolkit treasury grant-config add xion1... --preset execute
xion-toolkit treasury grant-config add xion1... --preset instantiate
```

### Configure Gasless Fees

Set up fee allowance for gasless transactions:

```bash
xion-toolkit treasury fee-config set xion1... --fee-config fee-config.json
```

**Note:** `fee-config set` requires a JSON config file. See [CLI Reference](./docs/cli-reference.md#treasury-fee-config-set) for the file format.

### Batch Operations

Manage multiple treasuries at once:

```bash
# Validate batch config before execution
xion-toolkit batch validate --from-file funds.json

# Execute batch operations
xion-toolkit batch execute --from-file funds.json [--simulate] [--memo "notes"]

# Or use treasury batch commands (legacy)
xion-toolkit treasury batch fund --config funds.json
xion-toolkit treasury batch grant-config --config grants.json
```

### Transfer Admin Rights

```bash
# Propose new admin
xion-toolkit treasury admin propose xion1... --new-admin xion1new...

# New admin accepts (run by the new admin)
xion-toolkit treasury admin accept xion1...
```

---

## Command Reference

### Authentication

| Command | Description |
|---------|-------------|
| `auth login` | Login via OAuth2 (browser) |
| `auth login --force` | Force new browser auth (skip refresh) |
| `auth login --dev-mode` | Login with Manager API scopes |
| `auth logout` | Clear stored credentials |
| `auth status` | Check authentication status |
| `auth refresh` | Refresh access token |

### Treasury

| Command | Description |
|---------|-------------|
| `treasury list` | List all your treasuries |
| `treasury query <address>` | Get treasury details |
| `treasury create` | Create a new treasury |
| `treasury fund <address> --amount` | Fund a treasury |
| `treasury withdraw <address> --amount --to` | Withdraw funds |
| `treasury export [--output]` | Export treasury configs |
| `treasury import <address> --from-file` | Import treasury config |

**Predicted Address:**
```bash
treasury create --predict --salt <salt>  # Get address before deploying
treasury create --is-oauth2-app          # Mark as OAuth2 application
```

**Chain Queries:**
```bash
treasury chain-query grants <ADDRESS>      # Query on-chain authz grants
treasury chain-query allowances <ADDRESS>  # Query on-chain fee allowances
```

**Update Parameters:**
```bash
treasury params update <ADDRESS> [--redirect-url] [--icon-url] [--name] [--is-oauth2-app]
```

**Batch Operations:**
```bash
treasury batch fund --config <file>           # Fund multiple
treasury batch grant-config --config <file>   # Configure multiple
```

### Asset (NFT)

| Command | Description |
|---------|-------------|
| `asset types` | List NFT contract types |
| `asset create --type --name --symbol` | Create NFT collection |
| `asset create --salt` | Predictable address (hex) |
| `asset mint --contract --token-id --owner` | Mint NFT |
| `asset mint --asset-type <TYPE>` | Asset type (default: cw721-base) |
| `asset mint --royalty-address --royalty-percentage` | CW2981 royalties |
| `asset mint --expires-at <TIMESTAMP>` | cw721-expiration |
| `asset predict --type --name --symbol --salt` | Predict contract address |
| `asset batch-mint --contract --tokens-file` | Batch mint tokens |
| `asset query --contract --msg` | Query NFT contract |

### Faucet

| Command | Description |
|---------|-------------|
| `faucet claim` | Claim testnet tokens (1 XION) |
| `faucet claim --receiver <address>` | Claim tokens for another address |
| `faucet status` | Check claim cooldown status |
| `faucet info` | Query faucet configuration |

**Note:** Faucet is testnet-only. Each claim provides 1 XION with 24-hour cooldown.

### Contract

| Command | Description |
|---------|-------------|
| `contract instantiate --code-id --label --msg` | Deploy contract |
| `contract instantiate2 --code-id --label --msg --salt` | Deploy with predicted address |
| `contract execute --contract --msg` | Execute contract message |
| `contract query --contract --msg` | Query contract (read-only) |

### OAuth2 Client

| Command | Description |
|---------|-------------|
| `oauth2 client list [--limit] [--cursor]` | List your OAuth clients |
| `oauth2 client create --redirect-uris --treasury` | Create new OAuth client |
| `oauth2 client get <CLIENT_ID>` | Get client details |
| `oauth2 client update <CLIENT_ID>` | Update client metadata |
| `oauth2 client delete <CLIENT_ID> --force` | Delete client |
| `oauth2 client extension get/update <ID>` | Manage client extension |
| `oauth2 client managers add/remove <ID>` | Manage client managers |
| `oauth2 client transfer-ownership <ID>` | Transfer ownership |
| `oauth2 client rotate-secret <ID>` | Rotate client secret |

**Note:** OAuth2 client management requires `--dev-mode` authentication for Manager API scopes.

### Transaction

| Command | Description |
|---------|-------------|
| `tx status <HASH>` | Query transaction status |
| `tx wait <HASH> [--timeout] [--interval]` | Wait for transaction confirmation |

**Example:**
```bash
# Wait for transaction with custom timeout
xion-toolkit tx wait ABC123... --timeout 120 --interval 5
```

### Account

| Command | Description |
|---------|-------------|
| `account info` | Show current MetaAccount info (address, balances) |

### Configuration

| Command | Description |
|---------|-------------|
| `config show` | Show current config |
| `config set-network <network>` | Switch network |
| `config get <KEY>` | Get config value |
| `config reset` | Reset to defaults |
| `completions <shell>` | Generate shell completion scripts |
| `status` | Show status |

---

## Global Options

```bash
xion-toolkit --network <testnet|mainnet>  # Network override
xion-toolkit --output <format>             # Output format
xion-toolkit --config <CONFIG>             # Path to config file
xion-toolkit --no-interactive              # Disable prompts
xion-toolkit --help                        # Show help
xion-toolkit --version                     # Show version
```

### Output Formats

| Format | Use Case |
|--------|----------|
| `json` (default) | Human reading, debugging |
| `json-compact` | CI/CD pipelines, parsing |
| `github-actions` | GitHub Actions workflow commands |
| `human` | Simplified human-readable output |

```bash
xion-toolkit treasury list --output json-compact
```

---

## Networks

| Network | Description |
|---------|-------------|
| `testnet` | Default, for development |
| `mainnet` | Production |

```bash
xion-toolkit config set-network mainnet
xion-toolkit --network testnet status
```

---

## Error Handling

Errors include actionable hints:

```json
{
  "success": false,
  "error": "Not authenticated",
  "error_code": "EAUTH001",
  "hint": "Run 'xion-toolkit auth login' to authenticate"
}
```

See [docs/ERROR-CODES.md](./docs/ERROR-CODES.md) for all error codes.

Exit codes are standardized for CI/CD — see [docs/EXIT-CODES.md](./docs/EXIT-CODES.md).

---

## Security

- **No Private Keys** — Uses OAuth2 and MetaAccount authentication
- **PKCE (RFC 7636)** — Prevents authorization code interception
- **AES-256-GCM** — Encrypted credential storage
- **Localhost Only** — Callback server only accepts localhost
- **HTTPS Only** — All communications encrypted

---

## Documentation

| Document | Description |
|----------|-------------|
| [CLI Reference](./docs/cli-reference.md) | Detailed command docs |
| [Quick Reference](./docs/QUICK-REFERENCE.md) | Condensed reference for AI |
| [Error Codes](./docs/ERROR-CODES.md) | Complete error reference |
| [Exit Codes](./docs/EXIT-CODES.md) | CI/CD exit codes |
| [Configuration](./docs/configuration.md) | Setup guide |

---

## For AI Agents

If you want your AI Agent to install and use this toolkit, give it this instruction:

```
Follow this guide https://raw.githubusercontent.com/burnt-labs/xion-agent-toolkit/main/INSTALL-FOR-AGENTS.md to install and configure the Xion Agent Toolkit skills for AI agents.
```

For building AI agents with this toolkit:
- See [INSTALL-FOR-AGENTS.md](./INSTALL-FOR-AGENTS.md) for integration instructions
- Use [QUICK-REFERENCE.md](./docs/QUICK-REFERENCE.md) for condensed CLI reference

---

## Shell Completion

Enable shell completion for easier CLI usage:

```bash
# Auto-detect and install
xion-toolkit completions --install

# Or specify shell
xion-toolkit completions bash --install
xion-toolkit completions zsh --install
xion-toolkit completions fish --install
xion-toolkit completions powershell --install
```

Restart your shell after installation.

---

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Contributing Guide](./CONTRIBUTING.md)
- [xion-skills](https://github.com/burnt-labs/xion-skills) — Advanced chain operations

---

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.
