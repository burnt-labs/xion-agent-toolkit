# Xion Agent Toolkit

**Build on Xion with gasless transactions.**

A command-line tool for managing Xion MetaAccounts, Treasury contracts, and NFT assets — all without handling private keys or paying gas fees.

---

## What You Can Do

- **🔐 Login with Google, Email, or Passkey** — No seed phrases, no private keys
- **💰 Manage Treasuries** — Create, fund, withdraw, configure permissions
- **🎨 Create NFTs** — Deploy and mint CW721 collections
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

### 3. Create a Treasury

```bash
# Create and fund a new treasury
xion-toolkit treasury create --fund 1000000uxion

# Or predict the address first (without deploying)
xion-toolkit treasury create --predict --salt "my-treasury-v1"
```

### 4. Manage Your Treasury

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

### 5. Create an NFT Collection

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
  --grant-type-url "/cosmos.bank.v1beta1.MsgSend" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion"
```

### Configure Gasless Fees

Set up fee allowance for gasless transactions:

```bash
xion-toolkit treasury fee-config set xion1... \
  --fee-allowance-type basic \
  --fee-spend-limit "5000000uxion"
```

### Batch Operations

Manage multiple treasuries at once:

```bash
# Create a config file (see examples/batch-fund.json)
xion-toolkit treasury batch fund --config funds.json

# Configure grants for multiple treasuries
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
| `xion-toolkit auth login` | Login via OAuth2 (browser) |
| `xion-toolkit auth logout` | Clear stored credentials |
| `xion-toolkit auth status` | Check authentication status |
| `xion-toolkit auth refresh` | Refresh access token |

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
| `asset mint --contract --token-id --owner` | Mint NFT |
| `asset predict --type --name --symbol --salt` | Predict contract address |
| `asset batch-mint --contract --tokens-file` | Batch mint tokens |
| `asset query --contract --msg` | Query NFT contract |

### Contract

| Command | Description |
|---------|-------------|
| `contract instantiate --code-id --label --msg` | Deploy contract |
| `contract instantiate2 --code-id --label --msg --salt` | Deploy with predicted address |
| `contract execute --contract --msg` | Execute contract message |
| `contract query --contract --msg` | Query contract (read-only) |

### Configuration

| Command | Description |
|---------|-------------|
| `config show` | Show current config |
| `config set-network <network>` | Switch network |
| `completions <shell>` | Generate shell completion scripts |
| `status` | Show status |

---

## Global Options

```bash
xion-toolkit --network <testnet|mainnet>  # Network override
xion-toolkit --output <format>             # Output format
xion-toolkit --help                        # Show help
xion-toolkit --version                     # Show version
```

### Output Formats

| Format | Use Case |
|--------|----------|
| `json` (default) | Human reading, debugging |
| `json-compact` | CI/CD pipelines, parsing |
| `github-actions` | GitHub Actions workflow commands |

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

The CLI supports shell completion for bash, zsh, fish, and PowerShell.

### Quick Install (Recommended)

Install completions with a single command:

```bash
# Auto-detect shell and install
xion-toolkit completions --install

# Or specify the shell explicitly
xion-toolkit completions bash --install
xion-toolkit completions zsh --install
xion-toolkit completions fish --install
xion-toolkit completions powershell --install
```

After installation, restart your shell or source the profile file.

### Manual Installation

If you prefer manual installation:

### Bash

```bash
# Generate and install completions
xion-toolkit completions bash > ~/.local/share/bash-completion/completions/xion-toolkit

# Then source it (or restart your shell)
source ~/.local/share/bash-completion/completions/xion-toolkit
```

For older bash versions, you may need to source the file directly in your `.bashrc`:

```bash
echo 'source ~/.local/share/bash-completion/completions/xion-toolkit' >> ~/.bashrc
```

### Zsh

```bash
# Generate completions to a directory in your fpath
mkdir -p ~/.zfunc
xion-toolkit completions zsh > ~/.zfunc/_xion-toolkit

# Add to your .zshrc (if not already present)
echo 'fpath+=~/.zfunc' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc

# Then restart your shell or run:
autoload -U compinit && compinit
```

### Fish

```bash
# Generate and install completions
xion-toolkit completions fish > ~/.config/fish/completions/xion-toolkit.fish

# Completions will be available in new fish sessions
```

### PowerShell

```powershell
# Generate completions
xion-toolkit completions powershell > xion-toolkit.ps1

# Source the file in your PowerShell profile
. ./xion-toolkit.ps1
```

### Available Shells

Run `xion-toolkit completions --help` to see all supported shells:

```
xion-toolkit completions [SHELL]

Arguments:
  [SHELL]  Shell type to generate completions for
           If not specified, auto-detects from $SHELL
           [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -i, --install    Install completions to shell profile
```

---

## Resources

- [Xion Documentation](https://docs.burnt.com/xion)
- [Developer Portal](https://dev.testnet2.burnt.com)
- [Contributing Guide](./CONTRIBUTING.md)
- [xion-skills](https://github.com/burnt-labs/xion-skills) — Advanced chain operations

---

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.
