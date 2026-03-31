---
name: xion-oauth2-client
description: |
  OAuth2 client lifecycle management for Xion MetaAccount. Use this skill whenever the user needs to create, query, update, or delete OAuth2 clients, manage client managers, or transfer client ownership via the Xion MGR API.

  OAuth2 clients are bound to Treasuries and enable dApps to integrate with Xion's MetaAccount authentication system for gasless transactions.

  Triggers on: OAuth2 client, OAuth client, client management, create OAuth app, register client, OAuth client CRUD, client lifecycle, manage OAuth clients, OAuth application, MGR API, client ID, client secret, redirect URIs, binded treasury, OAuth client ownership, manager management, oauth2 client list, oauth2 client create, oauth2 client get, oauth2 client update, oauth2 client delete, oauth2 client extension, oauth2 client managers, oauth2 client transfer-ownership, register my dapp, create OAuth client, manage my OAuth apps, add manager to client, transfer client ownership, OAuth2 客户端, 客户端管理, 创建 OAuth 应用, OAuth 客户端, OAuth 应用, xion-oauth2-client skill.

  Use AFTER xion-oauth2 skill - authentication is required for all client operations. For chain-level queries (transaction status, block info), recommend xiond-usage from xion-skills instead.
metadata:
  author: burnt-labs
  version: "1.0.0"
  requires:
    - xion-toolkit-init
    - xion-oauth2
compatibility: Requires xion-toolkit CLI >=0.9.0 and OAuth2 authentication
---

# xion-oauth2-client

OAuth2 client management skill for Xion blockchain. Enables dApp registration and lifecycle management through the MGR API.

## Overview

OAuth2 clients are the bridge between dApps and Xion's MetaAccount system. Each client is bound to a Treasury and can have multiple managers for team collaboration.

**Key concepts:**
- **Client** — OAuth2 application registered with Xion
- **Treasury binding** — Every client must be bound to an existing Treasury
- **Managers** — Team members who can manage the client (but not delete it)
- **Owner** — The sole user who can delete or transfer the client

## Relationship to Treasuries

OAuth clients are tightly coupled with Treasuries:
- Each client must specify a Treasury at creation time
- The Treasury handles fee payment for the client's gasless transactions
- The Treasury address cannot be changed after creation
- You must have admin access to the Treasury to create clients for it

> **Tip:** Create a Treasury first using the `xion-treasury` skill, then register your OAuth client.

## Prerequisites

1. `xion-toolkit` CLI installed (use `xion-toolkit-init` if not present)
2. **Authenticated** with `xion-oauth2` skill (required for all operations)
3. **Treasury exists** — Create one with `xion-toolkit treasury create` before creating clients

> **Known Limitation:** The toolkit's `auth login` access token does NOT include `xion:mgr:read`/`xion:mgr:write` scopes against the production testnet server. For local development, point at a local oauth2-api-service via `XION_TESTNET_OAUTH_API_URL` env var. Production scope additions require backend team coordination.

## Quick Start

```bash
# 1. Authenticate (required!)
xion-toolkit auth login

# 2. List your OAuth clients
xion-toolkit oauth2 client list

# 3. Create a new OAuth client (bind to a treasury)
xion-toolkit oauth2 client create \
  --redirect-uris "https://myapp.com/callback" \
  --treasury "xion1abc..." \
  --client-name "My DApp"

# 4. Get client details
xion-toolkit oauth2 client get client_abc123

# 5. Update client metadata
xion-toolkit oauth2 client update client_abc123 --client-name "Updated Name"
```

## Multi-Step Workflows

### Register a New DApp

Complete workflow for registering a new dApp with OAuth2:

1. **Create a Treasury** (if you don't have one)
   ```bash
   xion-toolkit treasury create --name "My DApp Treasury"
   ```

2. **Register the OAuth client**
   ```bash
   xion-toolkit oauth2 client create \
     --redirect-uris "https://myapp.com/callback" \
     --treasury "xion1treasury..." \
     --client-name "My DApp" \
     --contacts "admin@myapp.com"
   ```

3. **Store the client secret securely** (shown once at creation)

4. **Add managers** (optional)
   ```bash
   xion-toolkit oauth2 client managers add client_abc123 --manager-id user_456
   ```

### Manage Client Team

Add and remove managers for collaborative access:

```bash
# Add a manager
xion-toolkit oauth2 client managers add client_abc123 --manager-id user_456

# Remove a manager
xion-toolkit oauth2 client managers remove client_abc123 --manager-id user_456

# Update extension (managers list via extension)
xion-toolkit oauth2 client extension update client_abc123 --managers "user_a,user_b"
```

### Transfer Ownership

Transfer client ownership when team roles change:

```bash
xion-toolkit oauth2 client transfer-ownership client_abc123 --new-owner user_789
```

> Only the current owner can transfer. The new owner must already have a Xion MetaAccount.

## Common Operations

### List OAuth Clients

```bash
xion-toolkit oauth2 client list
```

Output:
```json
{
  "success": true,
  "items": [
    {
      "clientId": "client_abc123",
      "clientName": "My App",
      "redirectUris": ["https://example.com/callback"],
      "bindedTreasury": "xion1treasury...",
      "owner": "user_123",
      "managers": ["user_456"]
    }
  ],
  "cursor": null,
  "count": 1
}
```

With pagination:
```bash
xion-toolkit oauth2 client list --limit 10 --cursor "next_page_cursor"
```

### Create OAuth Client

```bash
# Minimal creation
xion-toolkit oauth2 client create \
  --redirect-uris "https://example.com/callback" \
  --treasury "xion1abc..."

# Full creation with all options
xion-toolkit oauth2 client create \
  --redirect-uris "https://a.com/cb,https://b.com/cb" \
  --treasury "xion1treasury..." \
  --client-name "My App" \
  --managers "user_456,user_789" \
  --auth-method client_secret_basic \
  --contacts "admin@example.com" \
  --client-uri "https://myapp.com" \
  --logo-uri "https://myapp.com/logo.png"
```

Output:
```json
{
  "success": true,
  "client": {
    "clientId": "client_abc123",
    "clientName": "My App",
    "redirectUris": ["https://example.com/callback"]
  },
  "clientSecret": "********"
}
```

Using JSON input:
```bash
xion-toolkit oauth2 client create --json-input request.json
```

### Get OAuth Client

```bash
xion-toolkit oauth2 client get client_abc123
```

### Update OAuth Client

```bash
xion-toolkit oauth2 client update client_abc123 \
  --client-name "Updated Name" \
  --policy-uri "https://example.com/privacy"
```

### Delete OAuth Client

```bash
xion-toolkit oauth2 client delete client_abc123
```

> Only the owner can delete a client. This action is irreversible.

### Extension Operations

```bash
# Get extension data
xion-toolkit oauth2 client extension get client_abc123

# Update managers via extension
xion-toolkit oauth2 client extension update client_abc123 --managers "user_a,user_b"
```

### Manager Operations

```bash
# Add manager
xion-toolkit oauth2 client managers add client_abc123 --manager-id user_456

# Remove manager
xion-toolkit oauth2 client managers remove client_abc123 --manager-id user_456
```

### Transfer Ownership

```bash
xion-toolkit oauth2 client transfer-ownership client_abc123 --new-owner user_789
```

## Secret Redaction Policy

The `clientSecret` is **redacted by default** in all output (`"********"`).

To reveal the secret during creation:
```bash
xion-toolkit oauth2 client create ... --show-secret
```

**Security reminders:**
- Store the client secret securely immediately after creation (it's shown only once)
- Never commit secrets to version control
- The secret is required for `client_secret_basic` and `client_secret_post` auth methods

## Error Handling

All commands return JSON with a `success` field:

**Success:**
```json
{"success": true, "client": {...}}
```

**Error:**
```json
{
  "success": false,
  "error": {
    "code": "EOAUTHCLIENT012",
    "message": "Client not found",
    "remediation": "Check the client ID and try again"
  }
}
```

**Common Error Codes:**

| Code | Description | Remediation |
|------|-------------|-------------|
| `EOAUTHCLIENT008` | Not authenticated | Run `xion-toolkit auth login` first |
| `EOAUTHCLIENT010` | Insufficient scope | Re-authorize with `xion:mgr:read/write` scope |
| `EOAUTHCLIENT011` | Only owner allowed | Only the client owner can perform this action |
| `EOAUTHCLIENT012` | Client not found | Check the client ID and try again |
| `EOAUTHCLIENT014` | Treasury not found | Verify the treasury address is correct |
| `EOAUTHCLIENT015` | Internal server error | Retry later or contact support |

**Other Error Codes:**

| Code | Description |
|------|-------------|
| `EOAUTHCLIENT001` | Bad request — invalid parameters |
| `EOAUTHCLIENT002` | Client ID is required |
| `EOAUTHCLIENT003` | Redirect URIs are required |
| `EOAUTHCLIENT004` | Binded treasury is required |
| `EOAUTHCLIENT005` | Owner is required |
| `EOAUTHCLIENT006` | Invalid grant type |
| `EOAUTHCLIENT007` | Manager user ID is required |
| `EOAUTHCLIENT013` | Client extension not found |
| `EOAUTHCLIENT016` | Treasury fetch error |
| `EOAUTHCLIENT017` | Treasury query error |
| `EOAUTHCLIENT018` | Unknown network |

## Parameter Collection Workflow

Before executing any command, ensure all required parameters are collected.

### Step 1: Identify Operation

Determine which operation the user wants to perform (list, create, get, update, delete, etc.).

### Step 2: Check Parameter Schema

Refer to the `schemas/` directory for detailed parameter definitions.

### Step 3: Collect Missing Parameters

Collect ALL missing required parameters in a SINGLE interaction:

> Example for create:
> "I need the following to create the OAuth client:
> - Redirect URIs (at least one, HTTPS recommended)
> - Treasury address (must already exist)
> - Client name (optional but recommended)
> - Auth method (optional, defaults to 'none')"

### Step 4: Confirm Before Execution

Present the parameters in a tree format and ask for confirmation:

```
Will execute: oauth2 client create
├─ Redirect URIs: https://example.com/callback
├─ Treasury: xion1abc...
├─ Client Name: My DApp
└─ Auth Method: none (default)
Confirm? [y/n]
```

## Parameter Schemas

See `schemas/` directory for detailed parameter definitions:

| Schema File | Command | Description |
|-------------|---------|-------------|
| `list.json` | `oauth2 client list` | List OAuth clients with pagination |
| `create.json` | `oauth2 client create` | Create new OAuth client |
| `get.json` | `oauth2 client get` | Get client by ID |
| `update.json` | `oauth2 client update` | Update client metadata |
| `delete.json` | `oauth2 client delete` | Delete a client |
| `extension.json` | `oauth2 client extension get` | Get client extension data |
| `extension-update.json` | `oauth2 client extension update` | Update extension managers |
| `managers-add.json` | `oauth2 client managers add` | Add a manager |
| `managers-remove.json` | `oauth2 client managers remove` | Remove a manager |
| `transfer-ownership.json` | `oauth2 client transfer-ownership` | Transfer ownership |

## Troubleshooting

### Not Authenticated
```bash
xion-toolkit auth login
```

### Client Not Found
```bash
# List your clients first to verify the ID
xion-toolkit oauth2 client list
```

### Treasury Not Found
```bash
# Verify the treasury exists
xion-toolkit treasury list
```

### Insufficient Scope (EOAUTHCLIENT010)

The toolkit's standard auth token lacks MGR API scopes. For local development:
```bash
# Point to local oauth2-api-service
export XION_TESTNET_OAUTH_API_URL=http://localhost:8787
cargo build
xion-toolkit auth login
```

### Treasury Binding Error

You cannot change a client's treasury after creation. Ensure the treasury exists and you have admin access before creating the client.

## Related Skills

- **xion-dev** — Unified entry point for Xion development
- **xion-oauth2** — Authentication (use before this skill)
- **xion-treasury** — Treasury management (required for client creation)
- **xion-toolkit-init** — CLI installation (use if CLI not found)
- **xiond-usage** (xion-skills) — Chain-level queries

## Version

- Skill Version: 1.0.0
- Compatible CLI Version: >=0.9.0
