# Skills Guide

## Overview

### What are Skills

Skills are agent-friendly bash scripts that wrap the `xion-toolkit` CLI tool to provide AI Agents with structured, JSON-output capabilities for interacting with the Xion blockchain. Each skill:

- Outputs machine-readable JSON to stdout
- Writes progress and status messages to stderr
- Follows the [Agent Skills](https://agentskills.io/) format
- Uses `set -e` for strict error handling
- Provides consistent error codes and remediation hints

### Why use Skills with AI Agents

Skills are designed specifically for AI Agent integration:

1. **Structured Output**: All skills return JSON, making it easy for agents to parse results programmatically
2. **Error Handling**: Comprehensive error codes with actionable hints
3. **Progress Tracking**: Status messages to stderr don't interfere with JSON parsing
4. **Reusability**: Skills can be shared, versioned, and composed
5. **Safety**: Built-in validation and clear error messages prevent common mistakes

### Skills vs CLI Direct Usage

| Feature | Skills | CLI Direct |
|---------|--------|------------|
| Output Format | JSON (machine-readable) | JSON + human-readable |
| Error Handling | Structured error codes | Standard exit codes |
| Progress Messages | stderr (non-blocking) | Mixed output |
| Agent Optimization | ✅ Designed for agents | ⚠️ Requires parsing |
| Installation | `npx skills add ...` | Cargo install / binary |
| Use Case | AI Agents, automation | Developers, manual ops |

**Recommendation**: Use Skills for AI Agents and automation. Use CLI directly for manual development and debugging.

---

## Installation

### Via skills.sh (Recommended)

The [skills.sh](https://skills.sh) ecosystem provides a unified package manager for agent skills:

```bash
# Install xion-agent-toolkit skills (OAuth2 + Treasury)
npx skills add burnt-labs/xion-agent-toolkit

# Optionally, also install xion-skills for xiond CLI operations
npx skills add burnt-labs/xion-skills
```

#### What it installs

**burnt-labs/xion-agent-toolkit:**

| Skill | Purpose |
|-------|---------|
| `xion-dev` | Unified entry point - routes to correct skill based on user needs |
| `xion-toolkit-init` | Install xion-toolkit CLI automatically |
| `xion-oauth2` | OAuth2 authentication (login, logout, status, refresh) |
| `xion-treasury` | Treasury management (create, query, fund, withdraw, grants, fees) |

**burnt-labs/xion-skills (optional):**

| Skill | Purpose |
|-------|---------|
| `xiond-init` | Install and configure xiond CLI |
| `xiond-usage` | Chain queries, account management, transactions |
| `xiond-wasm` | CosmWasm contract deployment |

#### Dependency Graph

```
burnt-labs/xion-agent-toolkit
├── xion-dev (entry point - routes to correct skill)
├── xion-toolkit-init (installs xion-toolkit CLI)
├── xion-oauth2 (requires xion-toolkit)
└── xion-treasury (requires xion-oauth2)

burnt-labs/xion-skills (optional, for advanced operations)
├── xiond-init (installs xiond CLI)
├── xiond-usage (chain queries, requires xiond)
└── xiond-wasm (contract deployment, requires xiond)
```

#### When to Use Which Toolkit

| Use xion-agent-toolkit when... | Use xion-skills when... |
|-------------------------------|-------------------------|
| Building Xion applications | Deploying CosmWasm contracts |
| Managing Treasury contracts | Querying chain data (blocks, txs) |
| Gasless transactions | Checking transaction status |
| OAuth2 authentication | Mnemonic wallet management |
| Authz/Fee grant configuration | Validator operations |

### Manual Installation (Alternative)

If skills.sh is not available, you can install skills manually:

```bash
# Install xion-toolkit CLI first
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh
cp -r "${SKILLS_DIR}/repo/skills/"* "$SKILLS_DIR/"

# Remove the cloned repo
rm -rf "${SKILLS_DIR}/repo"

# Make all scripts executable
chmod +x "$SKILLS_DIR"/*/scripts/*.sh

echo "Skills installed to: $SKILLS_DIR"
```

Or use the init script:

```bash
curl -fsSL https://raw.githubusercontent.com/burnt-labs/xion-agent-toolkit/main/skills/xion-toolkit-init/scripts/install.sh | bash
```

---

## Available Skills

### xion-dev

**Purpose**: Unified entry point for ALL Xion blockchain development. This skill helps route users to the correct tool based on their needs.

**When to Use**: This skill should be triggered whenever the user mentions anything related to Xion, MetaAccount, gasless transactions, Treasury, or Xion development in general.

**Core Philosophy**: Xion developers should primarily use MetaAccount for a gasless experience. The `xiond` CLI (from xion-skills) is reserved for advanced scenarios like contract deployment and chain queries.

**Decision Matrix**:

| User Needs | Recommended Skill | Why |
|------------|-------------------|-----|
| Login / Authentication | `xion-oauth2` | MetaAccount, gasless |
| Create / Manage Treasury | `xion-treasury` | Core functionality |
| Fund / Withdraw | `xion-treasury` | Gasless transactions |
| Authz / Fee Grant | `xion-treasury` | Specialized feature |
| Query chain data | `xiond-usage` (xion-skills) | More powerful queries |
| Deploy CosmWasm | `xiond-wasm` (xion-skills) | Contract developer tool |

---

### xion-toolkit-init

**Purpose**: Install xion-toolkit CLI when not present in the environment.

**Scripts**:
- `install.sh` - Download and install xion-toolkit from GitHub Releases

**Usage**:
```bash
# Basic installation
bash /path/to/xion-toolkit-init/scripts/install.sh

# Install with xion-skills dependency
bash /path/to/xion-toolkit-init/scripts/install.sh --with-xion-skills
```

**Output**:
```json
{
  "success": true,
  "message": "xion-toolkit installed successfully",
  "version": "xion-agent-toolkit 0.4.3",
  "path": "/home/user/.cargo/bin/xion-toolkit"
}
```

---

### xion-oauth2

OAuth2 authentication skill for Xion blockchain development. Enables AI agents to authenticate with Xion's MetaAccount system using browser-based OAuth2 flow with PKCE security.

**Prerequisites**: `xion-toolkit` CLI installed (use `xion-toolkit-init` if not present)

#### Purpose

- Authenticate with Xion blockchain using OAuth2 (no mnemonics)
- Manage authentication tokens (refresh, status, logout)
- Support multiple networks (local, testnet, mainnet)

#### Scripts

| Script | Description |
|--------|-------------|
| `login.sh` | Initiate OAuth2 login flow via browser |
| `status.sh` | Check current authentication status |
| `logout.sh` | Clear stored credentials |
| `refresh.sh` | Manually refresh access token |

#### Usage Examples

**Login:**

```bash
xion-toolkit auth login
# Or via skill script:
./skills/xion-oauth2/scripts/login.sh
```

**Output (Success):**

```json
{
  "success": true,
  "network": "testnet",
  "authenticated": true,
  "token_type": "Bearer",
  "expires_in": 3600,
  "scope": "treasury:manage"
}
```

**Check Status:**

```bash
./skills/xion-oauth2/scripts/status.sh
```

**Output (Success):**

```json
{
  "success": true,
  "authenticated": true,
  "network": "testnet",
  "token_info": {
    "expires_at": "2024-01-15T10:30:00Z",
    "expires_in_seconds": 3600,
    "needs_refresh": false
  }
}
```

**Logout:**

```bash
./skills/xion-oauth2/scripts/logout.sh --network testnet
```

**Output (Success):**

```json
{
  "success": true,
  "message": "Successfully logged out from testnet"
}
```

**Refresh Token:**

```bash
./skills/xion-oauth2/scripts/refresh.sh
```

**Output (Success):**

```json
{
  "success": true,
  "message": "Token refreshed successfully",
  "expires_at": "2024-01-15T11:30:00Z",
  "expires_in_seconds": 3600
}
```

**Output (Error):**

```json
{
  "success": false,
  "error": "Failed to start callback server: Port 54321 already in use",
  "error_code": "PORT_IN_USE"
}
```

---

### xion-treasury

Treasury management skill for Xion blockchain development. Enables AI agents to create, query, and manage Treasury contracts for gasless transactions.

#### Purpose

- List and query Treasury contracts
- Create new Treasuries with fee/grant configuration
- Fund and withdraw from Treasuries
- Manage Authz grants and Fee allowances
- Perform admin operations

#### Scripts

| Script | Description |
|--------|-------------|
| `list.sh` | List all Treasury contracts owned by authenticated user |
| `query.sh` | Query detailed information about a specific Treasury |
| `create.sh` | Create a new Treasury with configuration |
| `fund.sh` | Fund a Treasury contract |
| `withdraw.sh` | Withdraw funds from a Treasury |
| `grant-config.sh` | Configure Authz Grants (add, remove, list) |
| `fee-config.sh` | Configure Fee Grants (set, remove, query) |
| `admin.sh` | Admin management (propose, accept, cancel) |
| `update-params.sh` | Update Treasury parameters |

> **Note**: For chain-level queries (transaction status, block info), use `xiond-usage` from [xion-skills](https://github.com/burnt-labs/xion-skills).

#### Usage Examples

**List Treasuries:**

```bash
./skills/xion-treasury/scripts/list.sh
```

**Output (Success):**

```json
{
  "success": true,
  "treasuries": [
    {
      "address": "xion1abc123...",
      "balance": "10000000",
      "denom": "uxion",
      "admin": "xion1admin...",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "count": 1,
  "cached": false
}
```

**Query Treasury:**

```bash
./skills/xion-treasury/scripts/query.sh xion1abc123... --include-grants --include-fee
```

**Output (Success):**

```json
{
  "success": true,
  "treasury": {
    "address": "xion1abc123...",
    "balance": {
      "denom": "uxion",
      "amount": "10000000"
    },
    "admin": "xion1admin...",
    "grants": [
      {
        "granter": "xion1granter...",
        "grantee": "xion1grantee...",
        "authorization_type": "ContractExecution"
      }
    ],
    "fee_grant": {
      "allowance_type": "BasicAllowance",
      "spend_limit": [
        {
          "denom": "uxion",
          "amount": "1000000"
        }
      ]
    }
  }
}
```

**Create Treasury:**

```bash
./skills/xion-treasury/scripts/create.sh \
  --name "My Treasury" \
  --redirect-url "https://example.com/callback" \
  --fee-allowance basic \
  --fee-spend-limit "1000000uxion" \
  --grant-auth-type send \
  --grant-spend-limit "1000000uxion"
```

**Output (Success):**

```json
{
  "success": true,
  "treasury": {
    "address": "xion1abc123...",
    "admin": "xion1admin...",
    "balance": "0",
    "denom": "uxion",
    "params": {
      "redirect_url": "https://example.com/callback",
      "icon_url": "",
      "metadata": {
        "name": "My Treasury",
        "archived": false,
        "is_oauth2_app": false
      }
    }
  },
  "tx_hash": "ABC123..."
}
```

**Fund Treasury:**

```bash
./skills/xion-treasury/scripts/fund.sh xion1abc123... 1000000uxion
```

**Output (Success):**

```json
{
  "success": true,
  "treasury": "xion1abc123...",
  "amount": "1000000uxion",
  "tx_hash": "ABC123...",
  "new_balance": "2000000uxion"
}
```

**Withdraw from Treasury:**

```bash
./skills/xion-treasury/scripts/withdraw.sh xion1abc123... 500000uxion
```

**Output (Success):**

```json
{
  "success": true,
  "treasury": "xion1abc123...",
  "amount": "500000uxion",
  "recipient": "xion1admin...",
  "tx_hash": "ABC123...",
  "remaining_balance": "1500000uxion"
}
```

**Configure Authz Grant:**

```bash
./skills/xion-treasury/scripts/grant-config.sh xion1abc123... \
  --action add \
  --config grant-config.json
```

**Config File (grant-config.json):**

```json
{
  "type_url": "/cosmos.bank.v1beta1.MsgSend",
  "description": "Allow sending funds",
  "authorization": {
    "auth_type": "send",
    "spend_limit": "1000000uxion",
    "allow_list": ["xion1recipient..."]
  },
  "optional": false
}
```

**Configure Fee Allowance:**

```bash
./skills/xion-treasury/scripts/fee-config.sh xion1abc123... \
  --action set \
  --config fee-config.json
```

**Config File (fee-config.json - Basic):**

```json
{
  "basic": {
    "spend_limit": "1000000uxion",
    "description": "Basic fee allowance for gasless transactions"
  }
}
```

**Admin Operations:**

```bash
# Propose new admin
./skills/xion-treasury/scripts/admin.sh xion1abc123... propose --new-admin xion1newadmin...

# Accept admin (by pending admin)
./skills/xion-treasury/scripts/admin.sh xion1abc123... accept

# Cancel proposed admin
./skills/xion-treasury/scripts/admin.sh xion1abc123... cancel
```

**Output (Propose Success):**

```json
{
  "success": true,
  "treasury_address": "xion1abc123...",
  "operation": "propose_admin",
  "new_admin": "xion1newadmin...",
  "tx_hash": "ABC123..."
}
```

---

## Chain Queries

For chain-level queries (transaction status, block info, balance for any address), use `xiond-usage` from [xion-skills](https://github.com/burnt-labs/xion-skills):

```bash
# Query transaction status
xiond query tx <txhash>

# Query block info
xiond query block

# Query balance for any address
xiond query bank balances <address>
```

See the [xion-skills documentation](https://github.com/burnt-labs/xion-skills) for more details.

---

## Integration with AI Agents

### Claude Code

#### Installation

Add skills to your Claude Code configuration:

```json
{
  "tools": [
    {
      "name": "xion_login",
      "description": "Authenticate with Xion blockchain using OAuth2",
      "command": "~/.xion-toolkit/skills/xion-oauth2/scripts/login.sh",
      "output": "json"
    },
    {
      "name": "xion_status",
      "description": "Check current authentication status",
      "command": "~/.xion-toolkit/skills/xion-oauth2/scripts/status.sh",
      "output": "json"
    },
    {
      "name": "xion_treasury_list",
      "description": "List all Treasury contracts",
      "command": "~/.xion-toolkit/skills/xion-treasury/scripts/list.sh",
      "output": "json"
    },
    {
      "name": "xion_treasury_query",
      "description": "Query Treasury details by address",
      "command": "~/.xion-toolkit/skills/xion-treasury/scripts/query.sh ${args.address}",
      "output": "json"
    },
    {
      "name": "xion_treasury_create",
      "description": "Create a new Treasury contract",
      "command": "~/.xion-toolkit/skills/xion-treasury/scripts/create.sh ${args.name} ${args.options}",
      "output": "json"
    }
  ]
}
```

#### Example Prompts

```
Check if I'm authenticated with Xion, and if not, log me in.
Then list all my treasuries and show me the balance of each.
```

```
Create a new Treasury named "My App Treasury" with a basic fee allowance
of 5 XION. After creation, fund it with 10 XION.
```

```
Query the treasury at xion1abc123... and tell me:
1. Current balance
2. What authz grants are configured
3. What fee allowance is set
```

### Cursor / Windsurf / Other Agents

#### General Integration Approach

1. **Install skills** using Git clone or curl method
2. **Configure tool definitions** in your agent's tool registry
3. **Parse JSON output** from stdout
4. **Monitor stderr** for progress messages

#### Example Integration (TypeScript)

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

interface SkillResult<T> {
  success: boolean;
  data?: T;
  error?: string;
  errorCode?: string;
}

async function runSkill<T>(script: string, args: string[] = []): Promise<SkillResult<T>> {
  try {
    const { stdout, stderr } = await execAsync(
      `~/.xion-toolkit/skills/${script} ${args.join(' ')}`
    );

    // Log progress messages
    if (stderr) {
      console.error('[Skill Progress]', stderr);
    }

    // Parse JSON output
    const result = JSON.parse(stdout);
    return result;
  } catch (error) {
    return {
      success: false,
      error: error.message,
      errorCode: 'EXECUTION_FAILED'
    };
  }
}

// Usage
const treasuries = await runSkill('xion-treasury/scripts/list.sh');
if (treasuries.success) {
  console.log('Treasuries:', treasuries.data);
}
```

#### Example Integration (Python)

```python
import subprocess
import json
from typing import Optional, Dict, Any

def run_skill(skill_path: str, args: list = None) -> Dict[str, Any]:
    """Run a skill script and return parsed JSON result."""
    args = args or []
    cmd = [skill_path] + args

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            check=True
        )

        # Log progress to stderr
        if result.stderr:
            print(f"[Skill Progress] {result.stderr}", file=sys.stderr)

        # Parse JSON from stdout
        return json.loads(result.stdout)

    except subprocess.CalledProcessError as e:
        return {
            "success": False,
            "error": e.stderr,
            "errorCode": "EXECUTION_FAILED"
        }
    except json.JSONDecodeError as e:
        return {
            "success": False,
            "error": f"Invalid JSON output: {e}",
            "errorCode": "PARSE_ERROR"
        }

# Usage
treasuries = run_skill(
    "~/.xion-toolkit/skills/xion-treasury/scripts/list.sh"
)

if treasuries["success"]:
    print(f"Found {treasuries['count']} treasuries")
else:
    print(f"Error: {treasuries['error']}")
```

---

## Output Format

### Success Format

All skills return JSON with a `success` field set to `true`:

```json
{
  "success": true,
  "data": "...",
  "message": "Optional success message",
  "tx_hash": "Optional transaction hash",
  "cached": false
}
```

**Common Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `success` | boolean | Always `true` for successful execution |
| `data` | any | Result data (varies by script) |
| `message` | string | Optional human-readable message |
| `tx_hash` | string | Transaction hash (for write operations) |
| `cached` | boolean | Whether data was served from cache |

### Error Format

All skills return JSON with a `success` field set to `false`:

```json
{
  "success": false,
  "error": "Human-readable error message",
  "error_code": "ERROR_CODE",
  "hint": "Optional remediation hint"
}
```

**Common Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `success` | boolean | Always `false` for errors |
| `error` | string | Human-readable error description |
| `error_code` | string | Machine-readable error code |
| `hint` | string | Optional suggestion for fixing the error |

**Example Error Responses:**

```json
{
  "success": false,
  "error": "Not authenticated. Please login first.",
  "error_code": "NOT_AUTHENTICATED",
  "hint": "Run './skills/xion-oauth2/scripts/login.sh' to authenticate"
}
```

```json
{
  "success": false,
  "error": "Treasury xion1abc... not found",
  "error_code": "TREASURY_NOT_FOUND",
  "hint": "Use 'list.sh' to see your treasuries"
}
```

```json
{
  "success": false,
  "error": "xion-toolkit CLI not found in PATH",
  "error_code": "CLI_NOT_FOUND",
  "hint": "Install xion-toolkit: curl --proto '=https' --tlsv1.2 -LsSf https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh"
}
```

### Common Error Codes

| Error Code | Description | Resolution |
|------------|-------------|------------|
| `CLI_NOT_FOUND` | xion-toolkit CLI not installed | Install CLI using installer script |
| `NOT_AUTHENTICATED` | User not logged in | Run `login.sh` to authenticate |
| `TOKEN_EXPIRED` | Access token has expired | Run `refresh.sh` or `login.sh` |
| `TREASURY_NOT_FOUND` | Treasury address not found | Verify address, check network |
| `INVALID_ADDRESS` | Invalid address format | Use valid bech32 address |
| `NETWORK_ERROR` | Cannot connect to API | Check internet, firewall |
| `PORT_IN_USE` | Callback server port busy | Use `--port` to specify different port |
| `INSUFFICIENT_BALANCE` | Not enough funds | Fund the treasury or account |
| `FEATURE_NOT_AVAILABLE` | Feature not implemented | Check documentation for alternatives |

---

## Best Practices

### 1. Parse JSON from stdout Only

Skills write JSON to stdout and progress messages to stderr. Always separate them:

```bash
# Correct: Parse only stdout
RESULT=$(./skills/xion-treasury/scripts/list.sh 2>/dev/null)
echo "$RESULT" | jq '.treasuries'

# Or capture both separately
OUTPUT=$(./skills/xion-treasury/scripts/list.sh)
STATUS=$?
# OUTPUT contains JSON from stdout
# stderr was printed to terminal
```

### 2. Always Check stderr for Progress

Progress messages, warnings, and hints go to stderr:

```bash
# Capture both stdout and stderr
OUTPUT=$(./skills/xion-oauth2/scripts/login.sh 2>&1)
STDOUT=$(echo "$OUTPUT" | grep -v '^\[')
STDERR=$(echo "$OUTPUT" | grep '^\[')

echo "Progress: $STDERR"
echo "Result: $STDOUT" | jq '.'
```

### 3. Handle Both Success and Error Responses

Always check the `success` field before using result data:

```python
result = run_skill('xion-treasury/scripts/list.sh')

if result.get('success'):
    # Process successful result
    for treasury in result.get('treasuries', []):
        print(f"Treasury: {treasury['address']}")
else:
    # Handle error
    error_code = result.get('error_code', 'UNKNOWN')
    error_msg = result.get('error', 'Unknown error')
    hint = result.get('hint', '')

    print(f"Error [{error_code}]: {error_msg}")
    if hint:
        print(f"Hint: {hint}")
```

### 4. Check Authentication Before Operations

Most Treasury operations require authentication. Check first:

```bash
#!/bin/bash
set -e

SKILLS_DIR="$HOME/.xion-toolkit/skills"

# Check authentication
AUTH_STATUS=$("$SKILLS_DIR/xion-oauth2/scripts/status.sh" 2>/dev/null)
AUTHENTICATED=$(echo "$AUTH_STATUS" | jq -r '.authenticated')

if [[ "$AUTHENTICATED" != "true" ]]; then
    echo "Not authenticated. Please login..."
    "$SKILLS_DIR/xion-oauth2/scripts/login.sh"
fi

# Now safe to use treasury operations
"$SKILLS_DIR/xion-treasury/scripts/list.sh"
```

### 5. Use Network Flag Consistently

When working with multiple networks:

```bash
# Testnet (default)
./skills/xion-treasury/scripts/list.sh

# Mainnet
./skills/xion-treasury/scripts/list.sh --network mainnet

# Local development
./skills/xion-treasury/scripts/list.sh --network local
```

### 6. Bypass Cache When Needed

Skills cache data for 5 minutes. Use `--no-cache` for fresh data:

```bash
# Use cached data (faster)
./skills/xion-treasury/scripts/list.sh

# Force fresh data
./skills/xion-treasury/scripts/list.sh --no-cache
```

### 7. Use Config Files for Complex Operations

For Treasury creation with multiple options, use config files:

```bash
# Create config file
cat > treasury-config.json <<EOF
{
  "params": {
    "redirect_url": "https://example.com/callback",
    "icon_url": "https://example.com/icon.png",
    "metadata": {
      "name": "My Treasury",
      "is_oauth2_app": true
    }
  },
  "fee_config": {
    "description": "Basic fee allowance",
    "allowance_type": "basic",
    "spend_limit": "1000000uxion"
  },
  "grant_configs": [
    {
      "type_url": "/cosmos.bank.v1beta1.MsgSend",
      "description": "Allow sending funds",
      "authorization": {
        "auth_type": "send",
        "spend_limit": "1000000uxion"
      }
    }
  ]
}
EOF

# Use config file
./skills/xion-treasury/scripts/create.sh --config treasury-config.json
```

---

## Troubleshooting

### Common Issues and Solutions

#### Issue 1: Not Authenticated

**Error:**
```json
{
  "success": false,
  "error": "Not authenticated",
  "error_code": "NOT_AUTHENTICATED"
}
```

**Solution:**
```bash
./skills/xion-oauth2/scripts/login.sh
```

#### Issue 2: CLI Not Found

**Error:**
```json
{
  "success": false,
  "error": "xion-toolkit CLI not found in PATH",
  "error_code": "CLI_NOT_FOUND"
}
```

**Solution:**
```bash
# Install CLI
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh | sh

# Add to PATH
export PATH="$HOME/.local/bin:$PATH"

# Verify
which xion-toolkit
```

#### Issue 3: Port Already in Use

**Error:**
```json
{
  "success": false,
  "error": "Failed to start callback server: Port 54321 already in use",
  "error_code": "PORT_IN_USE"
}
```

**Solution:**
```bash
# Use different port
./skills/xion-oauth2/scripts/login.sh --port 54322

# Or find and kill the process
lsof -i :54321
kill <PID>
```

#### Issue 4: Treasury Not Found

**Error:**
```json
{
  "success": false,
  "error": "Treasury xion1abc... not found",
  "error_code": "TREASURY_NOT_FOUND"
}
```

**Solution:**
```bash
# Verify address format
# Treasury addresses start with 'xion1'

# List your treasuries to confirm
./skills/xion-treasury/scripts/list.sh

# Check you're on the right network
./skills/xion-treasury/scripts/list.sh --network testnet
```

#### Issue 5: Stale Cache

**Symptom:** Data doesn't reflect recent changes

**Solution:**
```bash
# Bypass cache
./skills/xion-treasury/scripts/list.sh --no-cache
```

#### Issue 6: Token Expired

**Error:**
```json
{
  "success": false,
  "error": "Token has expired",
  "error_code": "TOKEN_EXPIRED"
}
```

**Solution:**
```bash
# Refresh token
./skills/xion-oauth2/scripts/refresh.sh

# Or re-login
./skills/xion-oauth2/scripts/login.sh
```

#### Issue 7: Permission Denied

**Error:**
```
bash: ./skills/xion-oauth2/scripts/login.sh: Permission denied
```

**Solution:**
```bash
# Make scripts executable
chmod +x ~/.xion-toolkit/skills/*/scripts/*.sh
```

---

## Resources

### Official Documentation

| Resource | URL |
|----------|-----|
| Xion Agent Toolkit | https://github.com/burnt-labs/xion-agent-toolkit |
| Agent Skills Format | https://agentskills.io/ |
| Xion Documentation | https://docs.burnt.com/xion |
| Developer Portal | https://dev.testnet2.burnt.com |

### Related Skills Packages

| Package | Installation | Purpose |
|---------|--------------|---------|
| `xion-skills` | `npx skills add burnt-labs/xion-skills` | xiond CLI operations |
| `xion-agent-toolkit skills` | Git clone / curl | OAuth2 + Treasury |

### Installation Scripts

- **Shell Installer**: `https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh`
- **PowerShell Installer**: `https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1`

### Support

- **GitHub Issues**: https://github.com/burnt-labs/xion-agent-toolkit/issues
- **Discussions**: https://github.com/burnt-labs/xion-agent-toolkit/discussions

---

*Document Version: 1.1.0*
*Last Updated: 2026-03-11*
*Compatible CLI Version: >=0.1.0*
