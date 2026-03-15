# Exit Codes Reference

> Standardized exit codes for CI/CD pipeline integration and shell scripting.

## Overview

The Xion Agent Toolkit returns standardized exit codes to enable reliable error handling in CI/CD pipelines and shell scripts.

## Exit Code Ranges

| Range | Category | Description |
|-------|----------|-------------|
| 0 | Success | Operation completed successfully |
| 1 | General | Unknown or unspecified error |
| 2-19 | Authentication | Login, token, credential errors |
| 20-39 | Configuration | Config file, encryption, network config |
| 40-59 | Network | Connection, timeout, rate limiting |
| 60-79 | Transaction | TX query, wait, timeout |
| 80-99 | Treasury | Treasury operations |
| 100-119 | Asset | NFT/asset operations |
| 120-139 | Batch | Batch execution errors |

---

## Success

| Code | Name | Description |
|------|------|-------------|
| 0 | SUCCESS | Operation completed successfully |

---

## General Errors

| Code | Name | Description |
|------|------|-------------|
| 1 | GENERAL_ERROR | Unspecified error occurred |

---

## Authentication Errors (2-19)

| Code | Name | Description | Action |
|------|------|-------------|--------|
| 2 | AUTH_REQUIRED | Not authenticated | Run `xion-toolkit auth login` |
| 3 | TOKEN_EXPIRED | Access token has expired | Token refreshed automatically, retry |
| 4 | REFRESH_TOKEN_EXPIRED | Refresh token expired | Re-login required |
| 5 | INVALID_CREDENTIALS | Invalid credentials | Check credentials |
| 6 | OAUTH_CALLBACK_FAILED | OAuth2 callback failed | Check callback URL |
| 7 | PKCE_FAILED | PKCE verification failed | Restart login flow |
| 8 | AUTH_TIMEOUT | Authentication timeout | Try again |

### Example: Handling Authentication Errors

```bash
#!/bin/bash

xion-toolkit auth status
exit_code=$?

case $exit_code in
    0)
        echo "Authenticated successfully"
        ;;
    2)
        echo "Not authenticated, logging in..."
        xion-toolkit auth login
        ;;
    3)
        echo "Token expired, refreshing..."
        xion-toolkit auth refresh
        ;;
    4)
        echo "Session expired, please login again"
        xion-toolkit auth login
        ;;
esac
```

---

## Configuration Errors (20-39)

| Code | Name | Description | Action |
|------|------|-------------|--------|
| 20 | CONFIG_NOT_FOUND | Configuration file not found | Run `xion-toolkit config init` |
| 21 | INVALID_CONFIG | Invalid configuration | Check config file format |
| 22 | ENCRYPTION_FAILED | Encryption operation failed | Check encryption key |
| 23 | DECRYPTION_FAILED | Decryption operation failed | Check encryption key |
| 24 | NETWORK_NOT_FOUND | Network not in configuration | Use `--network` flag |

### Example: Handling Config Errors

```bash
#!/bin/bash

xion-toolkit config show
exit_code=$?

if [ $exit_code -eq 20 ]; then
    echo "Initializing configuration..."
    xion-toolkit config init
fi
```

---

## Network Errors (40-59)

| Code | Name | Description | Retryable |
|------|------|-------------|-----------|
| 40 | NETWORK_TIMEOUT | Connection timeout | Yes |
| 41 | RATE_LIMITED | Rate limited by API | Yes (wait) |
| 42 | SERVICE_UNAVAILABLE | Service temporarily unavailable | Yes |
| 43 | INVALID_RESPONSE | Invalid response from server | No |
| 44 | REQUEST_FAILED | Request failed | No |
| 45 | CONNECTION_REFUSED | Connection refused | Yes |
| 46 | DNS_FAILED | DNS resolution failed | Yes |
| 47 | TLS_ERROR | TLS/SSL error | No |

### Example: Retrying Network Errors

```bash
#!/bin/bash

max_retries=3
retry_count=0

while [ $retry_count -lt $max_retries ]; do
    xion-toolkit treasury list
    exit_code=$?
    
    # Check for retryable network errors
    if [ $exit_code -eq 40 ] || [ $exit_code -eq 41 ] || [ $exit_code -eq 42 ]; then
        echo "Network error (code: $exit_code), retrying..."
        retry_count=$((retry_count + 1))
        sleep 5
    else
        break
    fi
done
```

---

## Transaction Errors (60-79)

| Code | Name | Description |
|------|------|-------------|
| 60 | TX_QUERY_FAILED | Failed to query transaction |
| 61 | TX_WAIT_FAILED | Failed to wait for transaction |
| 62 | TX_TIMEOUT | Transaction confirmation timeout |

### Example: Transaction Monitoring

```bash
#!/bin/bash

xion-toolkit tx wait $TX_HASH --timeout 60
exit_code=$?

if [ $exit_code -eq 62 ]; then
    echo "Transaction taking longer than expected"
    echo "Check status: xion-toolkit tx status $TX_HASH"
fi
```

---

## Treasury Errors (80-99)

| Code | Name | Description |
|------|------|-------------|
| 80 | TREASURY_NOT_FOUND | Treasury address not found |
| 81 | INSUFFICIENT_BALANCE | Not enough balance |
| 82 | INVALID_TREASURY_ADDRESS | Invalid treasury address format |
| 83 | TREASURY_CREATION_FAILED | Failed to create treasury |
| 84 | TREASURY_OPERATION_FAILED | General treasury operation failed |
| 85 | GRANT_CONFIG_NOT_FOUND | Grant configuration not found |
| 86 | FEE_CONFIG_NOT_FOUND | Fee configuration not found |
| 87 | NOT_AUTHORIZED | Not authorized for operation |
| 88 | TREASURY_ALREADY_EXISTS | Treasury already exists |

### Example: Treasury Funding

```bash
#!/bin/bash

xion-toolkit treasury fund $ADDRESS 1000000uxion
exit_code=$?

case $exit_code in
    80)
        echo "Treasury not found: $ADDRESS"
        xion-toolkit treasury list
        ;;
    81)
        echo "Insufficient balance"
        ;;
    87)
        echo "Not authorized - must be treasury admin"
        ;;
esac
```

---

## Asset Errors (100-119)

| Code | Name | Description |
|------|------|-------------|
| 100 | INVALID_METADATA | Invalid NFT metadata |
| 101 | ASSET_CREATION_FAILED | Failed to create asset |
| 102 | INVALID_ASSET_CONFIG | Invalid asset configuration |
| 103 | CODE_ID_NOT_FOUND | Code ID not found |
| 104 | INVALID_SCHEMA | Invalid schema format |

---

## Batch Errors (120-139)

| Code | Name | Description |
|------|------|-------------|
| 120 | BATCH_TOO_LARGE | Batch exceeds 50 messages |
| 121 | BATCH_EXECUTION_FAILED | Batch execution failed |
| 122 | BATCH_PARTIAL_FAILURE | Some operations failed |
| 123 | INVALID_BATCH_ITEM | Invalid item in batch |

---

## Complete Exit Code Reference

| Code | Name | Category |
|------|------|----------|
| 0 | SUCCESS | Success |
| 1 | GENERAL_ERROR | General |
| 2 | AUTH_REQUIRED | Authentication |
| 3 | TOKEN_EXPIRED | Authentication |
| 4 | REFRESH_TOKEN_EXPIRED | Authentication |
| 5 | INVALID_CREDENTIALS | Authentication |
| 6 | OAUTH_CALLBACK_FAILED | Authentication |
| 7 | PKCE_FAILED | Authentication |
| 8 | AUTH_TIMEOUT | Authentication |
| 20 | CONFIG_NOT_FOUND | Configuration |
| 21 | INVALID_CONFIG | Configuration |
| 22 | ENCRYPTION_FAILED | Configuration |
| 23 | DECRYPTION_FAILED | Configuration |
| 24 | NETWORK_NOT_FOUND | Configuration |
| 40 | NETWORK_TIMEOUT | Network |
| 41 | RATE_LIMITED | Network |
| 42 | SERVICE_UNAVAILABLE | Network |
| 43 | INVALID_RESPONSE | Network |
| 44 | REQUEST_FAILED | Network |
| 45 | CONNECTION_REFUSED | Network |
| 46 | DNS_FAILED | Network |
| 47 | TLS_ERROR | Network |
| 60 | TX_QUERY_FAILED | Transaction |
| 61 | TX_WAIT_FAILED | Transaction |
| 62 | TX_TIMEOUT | Transaction |
| 80 | TREASURY_NOT_FOUND | Treasury |
| 81 | INSUFFICIENT_BALANCE | Treasury |
| 82 | INVALID_TREASURY_ADDRESS | Treasury |
| 83 | TREASURY_CREATION_FAILED | Treasury |
| 84 | TREASURY_OPERATION_FAILED | Treasury |
| 85 | GRANT_CONFIG_NOT_FOUND | Treasury |
| 86 | FEE_CONFIG_NOT_FOUND | Treasury |
| 87 | NOT_AUTHORIZED | Treasury |
| 88 | TREASURY_ALREADY_EXISTS | Treasury |
| 100 | INVALID_METADATA | Asset |
| 101 | ASSET_CREATION_FAILED | Asset |
| 102 | INVALID_ASSET_CONFIG | Asset |
| 103 | CODE_ID_NOT_FOUND | Asset |
| 104 | INVALID_SCHEMA | Asset |
| 120 | BATCH_TOO_LARGE | Batch |
| 121 | BATCH_EXECUTION_FAILED | Batch |
| 122 | BATCH_PARTIAL_FAILURE | Batch |
| 123 | INVALID_BATCH_ITEM | Batch |

---

## GitHub Actions Integration

When using `--output github-actions`, exit codes can be used with GitHub Actions conditionals:

```yaml
name: Treasury Operations

on: push

jobs:
  treasury:
    runs-on: ubuntu-latest
    steps:
      - name: Check Authentication
        id: auth
        run: |
          xion-toolkit auth status --output github-actions
        continue-on-error: true
        
      - name: Login if needed
        if: steps.auth.outcome == 'failure' && steps.auth.outputs.exit_code == '2'
        run: |
          xion-toolkit auth login
          
      - name: Create Treasury
        id: create
        run: |
          xion-toolkit treasury create \
            --redirect-url "https://example.com/callback" \
            --name "CI Treasury" \
            --output github-actions
```

---

## Shell Script Templates

### Basic Error Handling

```bash
#!/bin/bash
set -e

# Function to handle errors
handle_error() {
    local exit_code=$1
    local operation=$2
    
    case $exit_code in
        2|3|4)
            echo "Authentication error during $operation"
            xion-toolkit auth login
            ;;
        40|41|42|45|46)
            echo "Network error during $operation, retrying..."
            sleep 5
            return 1  # Signal retry
            ;;
        *)
            echo "Error ($exit_code) during $operation"
            return 0  # No retry
            ;;
    esac
}

# Example usage
perform_operation() {
    xion-toolkit treasury list
    return $?
}

# Main
perform_operation || handle_error $? "list treasuries"
```

### CI/CD Pipeline Script

```bash
#!/bin/bash
# CI/CD pipeline script with proper exit code handling

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check authentication
check_auth() {
    xion-toolkit auth status > /dev/null 2>&1
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        log_info "Authenticated"
        return 0
    elif [ $exit_code -eq 2 ]; then
        log_warn "Not authenticated"
        return 1
    elif [ $exit_code -eq 3 ]; then
        log_warn "Token expired"
        return 2
    else
        log_error "Auth check failed: $exit_code"
        return $exit_code
    fi
}

# Main execution
main() {
    log_info "Starting CI/CD pipeline"
    
    # Auth check with retry
    max_attempts=2
    attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        check_auth
        auth_status=$?
        
        if [ $auth_status -eq 0 ]; then
            break
        elif [ $auth_status -eq 1 ]; then
            log_info "Logging in..."
            xion-toolkit auth login
        elif [ $auth_status -eq 2 ]; then
            log_info "Refreshing token..."
            xion-toolkit auth refresh
        fi
        
        attempt=$((attempt + 1))
    done
    
    if [ $attempt -gt $max_attempts ]; then
        log_error "Authentication failed after $max_attempts attempts"
        exit 2
    fi
    
    # Continue with operations...
    log_info "Pipeline completed successfully"
    exit 0
}

main
```

---

## Version History

| Version | Changes |
|---------|---------|
| 0.9.0 | Introduced standardized exit codes with categorized ranges |