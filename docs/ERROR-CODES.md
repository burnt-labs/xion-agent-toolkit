# Error Codes Reference

> Complete error code reference for AI Agents and developers. All errors return structured JSON format.

## Exit Codes

The CLI returns standardized exit codes for CI/CD integration:

| Code | Name | Description |
|------|------|-------------|
| 0 | SUCCESS | Operation completed successfully |
| 1 | GENERAL_ERROR | General/unknown error |
| 10 | MAINNET_DISABLED | Mainnet mode is currently disabled |

### MAINNET_DISABLED (Exit Code 10)

**Description**: Mainnet mode is currently disabled.

**Error Message**:
```
Error: Mainnet mode is currently disabled.
  The xion-toolkit CLI is currently only available for testnet.
  Please use --network testnet or omit the flag (testnet is default).
```

**Solution**:
- Use `--network testnet` to operate on testnet
- Omit the `--network` flag (testnet is default)
- To enable mainnet, set `XION_MAINNET_DISABLED=false` (use with caution)

**Example**:
```bash
# This will fail with exit code 10
xion-toolkit --network mainnet auth status
echo $?  # Output: 10

# Use testnet instead
xion-toolkit --network testnet auth status

# Or omit the flag (testnet is default)
xion-toolkit auth status
```

**Configuration**:
```bash
# Check if mainnet is disabled (default)
echo $XION_MAINNET_DISABLED

# Enable mainnet (use with caution)
export XION_MAINNET_DISABLED=false
xion-toolkit --network mainnet auth status
```

---

## Error Response Format

### JSON Output (default for scripts/agents)

```json
{
  "success": false,
  "error": {
    "code": "ETREASURY001",
    "message": "Treasury not found: xion1...",
    "hint": "Run 'xion-toolkit treasury list' to see available treasuries.",
    "retryable": false
  }
}
```

### Human-readable Output

```
Error [ETREASURY001]: Treasury not found: xion1...

Hint: Run 'xion-toolkit treasury list' to see available treasuries.
```

---

## Error Code Schema

Format: `E{MODULE}{NUMBER}`

| Module | Code Range | Description |
|--------|------------|-------------|
| AUTH | EAUTH001-EAUTH099 | Authentication errors |
| TREASURY | ETREASURY001-ETREASURY099 | Treasury operations |
| ASSET | EASSET001-EASSET099 | Asset builder |
| BATCH | EBATCH001-EBATCH099 | Batch operations |
| CONFIG | ECONFIG001-ECONFIG099 | Configuration |
| NETWORK | ENETWORK001-ENETWORK099 | Network/API |
| TX | ETX001-ETX099 | Transaction monitoring |
| FAUCET | EFAUCET001-EFAUCET099 | Faucet operations |
| OAUTH_CLIENT | EOAUTHCLIENT001-EOAUTHCLIENT099 | OAuth2 client management |

---

## Authentication Errors (EAUTH001-EAUTH099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| EAUTH001 | Not authenticated | Run 'xion-toolkit auth login' first | No |
| EAUTH002 | Token expired | Token refreshed automatically, please retry | Yes |
| EAUTH003 | Refresh token expired | Re-login required: 'xion-toolkit auth login' | No |
| EAUTH004 | Invalid credentials | Check your credentials and try again | No |
| EAUTH005 | OAuth2 callback failed | Ensure callback URL is accessible and try again | No |
| EAUTH006 | PKCE verification failed | PKCE verification mismatch, restart login flow | No |
| EAUTH007 | Authentication timeout | Authentication took too long, please try again | No |

### Authentication Troubleshooting

```bash
# Check authentication status
xion-toolkit auth status

# Re-authenticate
xion-toolkit auth login

# Check current network
xion-toolkit config show
```

---

## Treasury Errors (ETREASURY001-ETREASURY099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| ETREASURY001 | Treasury not found | Run 'xion-toolkit treasury list' to see available treasuries | No |
| ETREASURY002 | Insufficient balance | Fund treasury with 'xion-toolkit treasury fund' | No |
| ETREASURY003 | Invalid treasury address | Verify the treasury address is a valid bech32 address | No |
| ETREASURY004 | Treasury creation failed | Check parameters and try again | No |
| ETREASURY005 | Treasury operation failed | Check treasury state and try again | No |
| ETREASURY006 | Grant config not found | Run 'xion-toolkit treasury grant-config list' to see available grants | No |
| ETREASURY007 | Fee config not found | Run 'xion-toolkit treasury fee-config query' to check fee config | No |
| ETREASURY008 | Not authorized for treasury operation | Ensure you are the admin of this treasury | No |
| ETREASURY009 | Treasury already exists | Use a different salt or address for the new treasury | No |

### Treasury Troubleshooting

```bash
# List all treasuries
xion-toolkit treasury list

# Query treasury details
xion-toolkit treasury query <ADDRESS>

# Fund treasury
xion-toolkit treasury fund <ADDRESS> <AMOUNT>

# Check grant configs
xion-toolkit treasury grant-config list <ADDRESS>

# Check fee config
xion-toolkit treasury fee-config query <ADDRESS>
```

---

## Asset Builder Errors (EASSET001-EASSET099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| EASSET001 | Invalid metadata | Check JSON structure against schema | No |
| EASSET002 | Asset creation failed | Check asset configuration and try again | No |
| EASSET003 | Invalid asset configuration | Verify all required fields are present | No |
| EASSET004 | Code ID not found | Check available code IDs with 'xion-toolkit asset code-ids' | No |
| EASSET005 | Invalid schema | Validate your schema against the expected format | No |

### Asset Builder Troubleshooting

```bash
# Check available code IDs
xion-toolkit asset code-ids

# Validate metadata JSON
cat metadata.json | jq .

# Create asset with debug output
xion-toolkit asset create --debug
```

---

## Batch Errors (EBATCH001-EBATCH099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| EBATCH001 | Batch too large | Maximum 50 messages per batch | No |
| EBATCH002 | Batch execution failed | Check individual message errors and retry | No |
| EBATCH003 | Partial batch failure | Some operations succeeded, check results for details | No |
| EBATCH004 | Invalid batch item | Verify batch item format and content | No |

### Batch Troubleshooting

```bash
# Validate batch file
cat batch.json | jq .

# Check batch size
jq '.messages | length' batch.json

# Run with debug output
xion-toolkit batch execute batch.json --debug
```

---

## Configuration Errors (ECONFIG001-ECONFIG099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| ECONFIG001 | Configuration not found | Run 'xion-toolkit config init' to create configuration | No |
| ECONFIG002 | Invalid configuration | Check configuration file format and values | No |
| ECONFIG003 | Encryption failed | Check encryption key availability | No |
| ECONFIG004 | Decryption failed | Check encryption key matches the one used for encryption | No |
| ECONFIG005 | Network not found in configuration | Specify network with '--network' flag or update config | No |

### Configuration Troubleshooting

```bash
# Show current configuration
xion-toolkit config show

# Check configuration file
cat ~/.xion-toolkit/config.json

# Check credentials exist
ls ~/.xion-toolkit/credentials/

# Re-initialize configuration
xion-toolkit config init
```

---

## Network Errors (ENETWORK001-ENETWORK099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| ENETWORK001 | Connection timeout | Check network connectivity, will retry | Yes |
| ENETWORK002 | Rate limited | Wait and retry, or reduce request frequency | Yes |
| ENETWORK003 | Service unavailable | Service is temporarily unavailable, retry later | Yes |
| ENETWORK004 | Invalid response from server | Server returned unexpected data, check API version | No |
| ENETWORK005 | Request failed | Check network settings and API endpoint | No |
| ENETWORK006 | Connection refused | Server is not accepting connections, check endpoint | Yes |
| ENETWORK007 | DNS resolution failed | Check DNS settings and network connectivity | Yes |
| ENETWORK008 | TLS error | Check TLS certificates and HTTPS configuration | No |

### Retry Behavior

Network errors with `retryable: true` are automatically retried with exponential backoff:

| Attempt | Delay | Max Delay |
|---------|-------|-----------|
| 1 | 100ms | 5000ms |
| 2 | 200ms | 5000ms |
| 3 | 400ms | 5000ms |

---

## Transaction Errors (ETX001-ETX099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| ETX001 | Transaction query failed | Check network connection and transaction hash | No |
| ETX002 | Transaction wait failed | Check network connection and wait parameters | No |
| ETX003 | Transaction timeout | Transaction took too long to confirm, check chain status | No |

### Transaction Troubleshooting

```bash
# Verify transaction exists
xion-toolkit tx status <tx_hash>

# Wait with longer timeout
xion-toolkit tx wait <tx_hash> --timeout 120

# Check network status
curl https://rpc.xion-testnet-2.burnt.com:443/status
```

### Network Troubleshooting

```bash
# Check API endpoint
curl -I https://oauth2.testnet.burnt.com/health

# Check RPC endpoint
curl https://rpc.xion-testnet-2.burnt.com:443/status

# Test with verbose output
xion-toolkit --verbose treasury list
```

---

## Faucet Errors (EFAUCET001-EFAUCET099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| EFAUCET001 | Faucet claim failed | Check cooldown status first with `faucet status`. Ensure receiver has less than 1 XION balance. | No |
| EFAUCET002 | Faucet query failed | Ensure address is valid. Check network connectivity. | No |
| EFAUCET003 | Not authenticated | Run `xion-toolkit auth login` first to authenticate. | No |
| EFAUCET004 | Faucet not available | Faucet is only available on testnet. Use `--network testnet`. | No |

### Faucet Troubleshooting

```bash
# Check faucet status before claiming
xion-toolkit faucet status

# Claim tokens
xion-toolkit faucet claim

# Check authentication
xion-toolkit auth status

# Ensure you're on testnet
xion-toolkit config show
```

---

## OAuth2 Client Errors (EOAUTHCLIENT001-EOAUTHCLIENT099)

| Code | Message | Hint | Retryable |
|------|---------|------|-----------|
| EOAUTHCLIENT001 | Bad request | Check request parameters and try again | No |
| EOAUTHCLIENT002 | Client ID is required | Provide a client ID | No |
| EOAUTHCLIENT003 | Redirect URIs are required | Provide at least one redirect URI | No |
| EOAUTHCLIENT004 | Binded treasury is required | Provide a treasury address with --treasury | No |
| EOAUTHCLIENT005 | Owner is required | Provide an owner user ID | No |
| EOAUTHCLIENT006 | Invalid grant type | Use a valid grant type (authorization_code, etc.) | No |
| EOAUTHCLIENT007 | Manager user ID is required | Provide a manager user ID | No |
| EOAUTHCLIENT008 | Authentication required | Run 'xion-toolkit auth login' first | No |
| EOAUTHCLIENT009 | User not found | Run 'xion-toolkit auth login' first | No |
| EOAUTHCLIENT010 | Insufficient scope | Re-authorize with xion:mgr:write scope | No |
| EOAUTHCLIENT011 | Only owner allowed | Only the client owner can perform this action | No |
| EOAUTHCLIENT012 | Client not found | Check the client ID and try again | No |
| EOAUTHCLIENT013 | Client extension not found | Check the client ID; extension may not exist | No |
| EOAUTHCLIENT014 | Treasury not found | Verify the treasury address is correct | No |
| EOAUTHCLIENT015 | Internal server error | Retry later or contact support | Yes |
| EOAUTHCLIENT016 | Treasury fetch error | Failed to fetch treasury data. Try again later. | Yes |
| EOAUTHCLIENT017 | Treasury query error | Failed to query treasury data. Try again later. | Yes |
| EOAUTHCLIENT018 | Unknown network | Verify network configuration and try again | No |

### OAuth2 Client Troubleshooting

```bash
# Check authentication status
xion-toolkit auth status

# List OAuth2 clients
xion-toolkit oauth2 client list

# Check network configuration
xion-toolkit config show
```

---

## Common Error Patterns

### 1. Check Authentication First

Most errors stem from authentication issues:

```bash
xion-toolkit auth status
```

### 2. Verify Network Configuration

Ensure you're on the correct network:

```bash
xion-toolkit config show
xion-toolkit config set network testnet
```

### 3. Validate Addresses

All addresses must be valid bech32 format starting with `xion1`:

```bash
# Valid address format
xion1abc123def456...
```

### 4. Check Balances

Insufficient balance is a common error:

```bash
xion-toolkit treasury query <ADDRESS>
```

### 5. Handle Rate Limiting

If you encounter rate limiting (ENETWORK002):

- Wait before retrying
- Reduce request frequency
- Use batch operations for multiple items

---

## Error Handling in Scripts

### Bash Example

```bash
#!/bin/bash

output=$(xion-toolkit treasury list 2>&1)
if echo "$output" | jq -e '.success == false' > /dev/null 2>&1; then
    code=$(echo "$output" | jq -r '.error.code')
    case $code in
        EAUTH001)
            echo "Not authenticated, running login..."
            xion-toolkit auth login
            ;;
        ENETWORK001|ENETWORK002|ENETWORK003)
            echo "Network error, will auto-retry..."
            sleep 5
            xion-toolkit treasury list
            ;;
        *)
            echo "Error: $code"
            echo "$output" | jq -r '.error.hint'
            exit 1
            ;;
    esac
fi
```

### Python Example

```python
import subprocess
import json

def run_command(cmd):
    result = subprocess.run(cmd, capture_output=True, text=True)
    try:
        data = json.loads(result.stdout)
        if not data.get('success', True):
            error = data.get('error', {})
            code = error.get('code')
            if code == 'EAUTH001':
                print("Not authenticated, running login...")
                subprocess.run(['xion-toolkit', 'auth', 'login'])
                return run_command(cmd)  # Retry
            elif error.get('retryable', False):
                print(f"Retryable error: {code}, retrying...")
                return run_command(cmd)
            else:
                raise Exception(f"{code}: {error.get('message')}")
        return data
    except json.JSONDecodeError:
        raise Exception(f"Failed to parse output: {result.stderr}")
```

---

## Quick Reference Card

| Scenario | Likely Code | Solution |
|----------|-------------|----------|
| Not logged in | EAUTH001 | `xion-toolkit auth login` |
| Token expired | EAUTH002 | Automatic refresh |
| Session expired | EAUTH003 | Re-login |
| Treasury missing | ETREASURY001 | `xion-toolkit treasury list` |
| No balance | ETREASURY002 | Fund treasury |
| TX not found | ETX001 | Check transaction hash |
| TX timeout | ETX003 | Increase wait timeout |
| Network timeout | ENETWORK001 | Auto-retry |
| Rate limited | ENETWORK002 | Wait and retry |
| Config missing | ECONFIG001 | `xion-toolkit config init` |

---

## Version History

| Version | Changes |
|---------|---------|
| 0.8.0 | Added TX error codes (ETX001-ETX003) for transaction monitoring |
| 0.7.0 | Introduced structured error codes with `E{MODULE}{NUMBER}` format |