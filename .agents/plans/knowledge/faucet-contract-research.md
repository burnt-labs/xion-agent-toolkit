# Faucet Contract Research

**Date**: 2026-03-18
**Status**: ✅ Contract Deployed - Ready for CLI Implementation
**Author**: AI Agent

---

## Overview

This document records the research and deployment for adding faucet functionality to xion-toolkit CLI.

---

## ✅ Deployment Success (2026-03-18)

The faucet contract has been successfully deployed to Xion testnet:

| Item | Value |
|------|-------|
| **Chain ID** | `xion-testnet-2` |
| **Code ID** | `2066` |
| **Contract Address** | `xion1kv2mz7yjk5azuuq7ptd7hrl7trwphu5enereqv8t66rkre00dxxqac9ywl` |
| **Store TxHash** | `47EDF1CC22FE99CE2C92199C94E17F00B15602ABBA5AD42A7041FCFD736EB76D` |
| **Instantiate TxHash** | `857D8D1244FAC18BEE740B708A9F6BB2C7287D817C09F01D79AB285E012FC7AF` |
| **Deployer** | `xion1epzznazp28up4asses7jdcyqnw3n8lu7f5g9xs` |
| **Source** | `~/workspace/xion/faucet-contract` |

### Instantiation Parameters

```json
{
  "amount_to_faucet": "1000000",
  "cool_down_period": 86400,
  "denom": "uxion"
}
```

- **Amount per claim**: 1,000,000 uxion (1 XION)
- **Cooldown period**: 86,400 seconds (24 hours)
- **Denom**: uxion

### Contract Limits & Rules

1. **Per-address cooldown**: Each address can only claim once every 24 hours
2. **Balance gate**: Receiver must have **< 1,000,000 uxion** balance to claim
3. **Contract funding**: Contract must have sufficient balance to serve requests
4. **Owner-only withdrawal**: Only the contract owner can withdraw funds

---

## 1. Research Findings

### 1.1 HTTP Faucet (faucet.xion.burnt.com)

**Conclusion**: Not suitable for CLI integration

| Aspect | Finding |
|--------|---------|
| **Protection** | Cloudflare Turnstile CAPTCHA |
| **API Endpoint** | `POST /api/credit` |
| **Bypass Possible** | ❌ No - requires browser interaction |
| **Amount Given** | 2,000,000 uxion (2 XION) |
| **Cooldown** | 24 hours |
| **Source Code** | https://github.com/burnt-labs/xion-faucet |

**Key Finding**: The HTTP faucet uses direct bank sends via CosmJS `SigningStargateClient`, not a smart contract. It protection makes it impossible to call from CLI.

### 1.2 @cosmjs/faucet Package

**Conclusion**: Not applicable - it is a server implementation

| Aspect | Finding |
|--------|---------|
| **Type** | Faucet server (not client library) |
| **Use Case** | Running a faucet service |
| **CLI Integration** | Not possible - no client API |

### 1.3 Faucet Contract (~/workspace/xion/faucet-contract)

**Conclusion**: Suitable for CLI integration - requires deployment

| Aspect | Finding |
|--------|---------|
| **Type** | CosmWasm smart contract |
| **Messages** | `Faucet`, `Delegate`, `Withdraw` |
| **Queries** | `GetDenom`, `GetAddressLastFaucetTimestamp` |
| **Rate Limiting** | On-chain state (cooldown per address) |
| **Funds Storage** | Contract balance |

**Contract Flow**:
```
User → CLI → OAuth2 API → ExecuteMsg::Faucet{} → Contract sends tokens
```

---

## Contract Messages

### Execute Messages

| Message | Description | Parameters |
|---------|-------------|------------|
| `Faucet{}` | Claim tokens for sender | None |
| `Delegate{receiver_address}` | Claim tokens for another address | `receiver_address: String` |
| `Withdraw{}` | Withdraw contract balance (owner-only) | None |

### Query Messages

| Message | Description | Returns |
|---------|-------------|---------|
| `GetDenom{}` | Get faucet denom | `{"denom": "uxion"}` |
| `GetAddressLastFaucetTimestamp{address}` | Get last claim time | `{"timestamp": 1234567890}` |

---

## CLI Implementation Plan

See `.agents/plans/faucet-command.md` for the full development plan.

### Command Design

```bash
# Claim tokens for current authenticated account
xion-toolkit faucet claim

# Claim tokens for another address (uses Delegate message)
xion-toolkit faucet claim --receiver xion1xxx...

# Check if an address can claim (cooldown status)
xion-toolkit faucet status [--address xion1xxx...]

# Query faucet configuration
xion-toolkit faucet info
```

### Configuration Schema

```json
{
  "version": "1.0",
  "network": "testnet",
  "faucet": {
    "contract_address": "xion1kv2mz7yjk5azuuq7ptd7hrl7trwphu5enereqv8t66rkre00dxxqac9ywl",
    "amount": "1000000",
    "denom": "uxion",
    "cooldown_seconds": 86400
  }
}
```

### Implementation Approach

1. **Use OAuth2 API** for `Faucet` and `Delegate` execute messages
2. **Use RPC queries** for `GetDenom` and `GetAddressLastFaucetTimestamp`
3. **Network-aware defaults** - testnet uses deployed contract, mainnet TBD

---

## References

- **Deployed Contract**: `xion1kv2mz7yjk5azuuq7ptd7hrl7trwphu5enereqv8t66rkre00dxxqac9ywl` (xion-testnet-2)
- **Contract Source**: `~/workspace/xion/faucet-contract`
- **HTTP Faucet**: https://faucet.xion.burnt.com/ (requires CAPTCHA, not CLI-friendly)
- **Xion Docs**: https://docs.burnt.com/xion

---

## Lessons Learned

1. **HashMap in State is problematic**: CosmWasm contracts should use `Map` storage instead of `HashMap` in state structs
2. **Memory allocation errors can be version-related**: The fix was to use `cw-storage-plus::Map` instead of `HashMap`
3. **HTTP faucets with CAPTCHA are not CLI-friendly**: Smart contract approach is better for automation
4. **Deployment verification is essential**: Contract must be deployed and funded before CLI implementation
