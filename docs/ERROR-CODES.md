# Error Codes Reference

> Complete error code reference for AI Agents. All errors return JSON format: `{"success": false, "error": "...", "code": "...", "suggestion": "..."}`

## Authentication Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `NOT_AUTHENTICATED` | Not authenticated | Run `xion-toolkit auth login` |
| `TOKEN_EXPIRED` | Token has expired | Run `xion-toolkit auth refresh` |
| `AUTH_LOGIN_FAILED` | Login failed | Retry or check browser for authorization |
| `AUTH_LOGOUT_FAILED` | Logout failed | Try again or manually clear credentials |
| `AUTH_REFRESH_FAILED` | Token refresh failed | Session expired, run `auth login` to re-authenticate |
| `PORT_IN_USE` | Callback port already in use | Use `--port <different-port>` for login |
| `CALLBACK_TIMEOUT` | Callback server timeout | Re-initiate login flow |
| `STATE_MISMATCH` | OAuth state parameter mismatch | Security error, restart login flow |
| `INVALID_CODE` | Authorization code invalid | Restart login flow |
| `PKCE_FAILED` | PKCE generation failed | Retry login |

## Treasury Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `TREASURY_NOT_FOUND` | Treasury not found | Verify address, check network |
| `TREASURY_LIST_FAILED` | Failed to list treasuries | Check network connection, verify auth |
| `TREASURY_QUERY_FAILED` | Failed to query treasury | Verify address is valid treasury |
| `TREASURY_CREATE_FAILED` | Failed to create treasury | Check parameters, ensure sufficient balance |
| `INSUFFICIENT_BALANCE` | Not enough balance | Fund the treasury or account |
| `INVALID_AMOUNT` | Invalid amount format | Use format `amountdenom` (e.g., `1000000uxion`) |
| `INVALID_ADDRESS` | Invalid address format | Use valid bech32 address starting with `xion1` |
| `UNAUTHORIZED` | Not authorized | Only admin can perform this action |
| `GRANT_CONFIG_ADD_FAILED` | Failed to add grant config | Check grant parameters |
| `GRANT_CONFIG_REMOVE_FAILED` | Failed to remove grant config | Verify grant exists |
| `GRANT_CONFIG_LIST_FAILED` | Failed to list grants | Network error or invalid address |
| `FEE_CONFIG_SET_FAILED` | Failed to set fee config | Check fee parameters |
| `FEE_CONFIG_QUERY_FAILED` | Failed to query fee config | Network error |
| `REVOKE_ALLOWANCE_FAILED` | Failed to revoke allowance | Verify allowance exists |
| `PROPOSE_ADMIN_FAILED` | Failed to propose admin | Check admin address validity |
| `ACCEPT_ADMIN_FAILED` | Failed to accept admin | Must be pending admin |
| `CANCEL_ADMIN_FAILED` | Failed to cancel admin | Must be current admin |
| `UPDATE_PARAMS_FAILED` | Failed to update params | Check parameter values |
| `EXPORT_FAILED` | Export failed | Verify treasury address |
| `IMPORT_FAILED` | Import failed | Check import file format |
| `QUERY_GRANTS_FAILED` | Query grants failed | Network error |
| `QUERY_ALLOWANCES_FAILED` | Query allowances failed | Network error |

## Network & API Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `NETWORK_ERROR` | Connection failed | Check internet connection |
| `TIMEOUT` | Request timed out | Retry or check network |
| `API_ERROR` | API request failed | Check API endpoint status |
| `RPC_ERROR` | RPC call failed | Verify RPC endpoint |
| `HTTPS_REQUIRED` | HTTPS required | Use https:// endpoint |
| `CERTIFICATE_ERROR` | SSL certificate error | Check system certificates |

## Input & Validation Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `INVALID_INPUT` | Invalid input | Check command syntax |
| `INVALID_JSON` | Invalid JSON format | Validate JSON syntax |
| `MISSING_REQUIRED` | Missing required field | Check required arguments |
| `INVALID_DENOM` | Invalid denomination | Use valid denom (uxion, uusdc, etc.) |
| `INVALID_COIN_FORMAT` | Invalid coin format | Use format `amountdenom` |
| `INVALID_TYPE_URL` | Invalid message type URL | Check type URL format |
| `INVALID_AUTH_TYPE` | Invalid authorization type | Use: generic, send, stake, ibc-transfer, contract-execution |
| `INVALID_FEE_TYPE` | Invalid fee allowance type | Use: basic, periodic, allowed-msg |
| `INVALID_FILTER_TYPE` | Invalid filter type | Use: allow_all, accepted_keys |
| `EMPTY_VALUE` | Empty value not allowed | Provide non-empty value |

## Encoding Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `ENCODING_FAILED` | Protobuf encoding failed | Check input data format |
| `DECODING_FAILED` | Failed to decode data | Verify data integrity |
| `BASE64_ERROR` | Base64 encoding failed | Check input data |
| `SERIALIZATION_ERROR` | Serialization failed | Check data structure |

## Configuration Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `CONFIG_ERROR` | Configuration error | Check config file |
| `CONFIG_NOT_FOUND` | Configuration not found | Run `config init` or create config |
| `NETWORK_NOT_FOUND` | Network not found | Use `testnet` or `mainnet` |
| `CREDENTIALS_NOT_FOUND` | Credentials not found | Run `auth login` |
| `CREDENTIALS_CORRUPTED` | Credentials corrupted | Run `auth login` to re-authenticate |
| `ENCRYPTION_FAILED` | Encryption failed | Check encryption key |
| `DECRYPTION_FAILED` | Decryption failed | Credentials may be corrupted |

## Contract Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `CONTRACT_INSTANTIATION_FAILED` | Contract instantiation failed | Check code ID, msg format |
| `CONTRACT_EXECUTION_FAILED` | Contract execution failed | Check contract address, msg, funds |
| `CONTRACT_QUERY_FAILED` | Contract query failed | Verify contract address |
| `INVALID_CONTRACT_ADDRESS` | Invalid contract address | Use valid bech32 contract address |
| `INVALID_MSG_FORMAT` | Invalid message format | Check JSON message syntax |
| `INVALID_FUNDS` | Invalid funds format | Use format `amountdenom` |
| `CODE_ID_NOT_FOUND` | Code ID not found | Verify code ID exists on chain |
| `INVALID_LABEL` | Invalid label | Use alphanumeric label |

## Asset Builder Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `INVALID_ASSET_TYPE` | Unknown asset type | Use: cw721-base, cw2981-royalties, cw721-expiration, cw721-metadata-onchain, cw721-non-transferable |
| `CODE_ID_NOT_FOUND` | Code ID not configured | Check network configuration |
| `MISSING_REQUIRED_FIELD` | Required field missing | Check required arguments |
| `INSTANTIATION_FAILED` | Contract instantiation failed | Check instantiate message format |
| `MINT_FAILED` | Token minting failed | Check mint parameters |
| `QUERY_FAILED` | Contract query failed | Verify contract address |
| `INVALID_OPTION_FOR_TYPE` | Option not valid for asset type | Royalty options require cw2981-royalties, expires-at requires cw721-expiration |
| `INVALID_ROYALTY_PERCENTAGE` | Royalty percentage must be 0.0-1.0 | Use decimal format (0.05 for 5%) |
| `INCOMPLETE_ROYALTY_INFO` | Both royalty-address and royalty-percentage required | Provide both or neither |
| `FILE_NOT_FOUND` | Tokens file not found | Check file path |
| `FILE_PARSE_ERROR` | Failed to parse tokens file | Validate JSON format |

## OAuth2 API Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `OAUTH2_ERROR` | OAuth2 API error | Check OAuth2 service status |
| `INVALID_GRANT` | Invalid grant | Authorization code expired or invalid |
| `INVALID_CLIENT` | Invalid client ID | Check client ID configuration |
| `INVALID_REDIRECT_URI` | Invalid redirect URI | Redirect URI must match registration |
| `UNAUTHORIZED_CLIENT` | Unauthorized client | Client not authorized for this flow |
| `UNSUPPORTED_GRANT_TYPE` | Unsupported grant type | Use authorization_code or refresh_token |
| `INVALID_SCOPE` | Invalid scope | Check requested scopes |
| `ACCESS_DENIED` | Access denied | User denied authorization |

## Transaction Errors

| Code | Message | Suggestion |
|------|---------|------------|
| `BROADCAST_FAILED` | Transaction broadcast failed | Check network, gas settings |
| `SIGNING_FAILED` | Transaction signing failed | Check credentials, session key |
| `INVALID_TX` | Invalid transaction | Check transaction format |
| `GAS_ESTIMATION_FAILED` | Gas estimation failed | Set gas manually |
| `OUT_OF_GAS` | Out of gas | Increase gas limit |
| `TX_TIMEOUT` | Transaction timeout | Check transaction status on explorer |
| `NONCE_ERROR` | Nonce error | Refresh credentials |

## Common Error Patterns

### Error Response Format
```json
{
  "success": false,
  "error": "Human-readable error message",
  "code": "ERROR_CODE",
  "suggestion": "Actionable remediation hint"
}
```

### Error Handling Best Practices

1. **Check `NOT_AUTHENTICATED` first** - Most common error
2. **Verify network** - testnet vs mainnet mismatch
3. **Validate addresses** - Must be bech32 format starting with `xion1`
4. **Check balances** - `INSUFFICIENT_BALANCE` is common
5. **Retry on timeout** - Network errors are often transient

### Quick Troubleshooting

```bash
# 1. Check authentication
xion-toolkit auth status

# 2. Check network
xion-toolkit config show

# 3. Verify address format
echo "Address should start with xion1..."

# 4. Check balance
xion-toolkit treasury query <ADDRESS>

# 5. Refresh token if expired
xion-toolkit auth refresh
```
