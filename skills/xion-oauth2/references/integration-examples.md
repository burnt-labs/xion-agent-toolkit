# xion-oauth2 Integration Examples

## Using with Claude Code

```javascript
// In your Claude Code skill
{
  "tools": [
    {
      "name": "xion_login",
      "description": "Authenticate with Xion blockchain",
      "command": "./skills/xion-oauth2/scripts/login.sh"
    }
  ]
}
```

## Programmatic Usage

```python
import subprocess
import json

# Login
result = subprocess.run(
    ['./skills/xion-oauth2/scripts/login.sh'],
    capture_output=True,
    text=True
)

if result.returncode == 0:
    data = json.loads(result.stdout)
    if data['success']:
        print(f"Authenticated on {data['network']}")
    else:
        print(f"Error: {data['error']}")
else:
    print(f"Script failed: {result.stderr}")
```

## Security Considerations

1. **PKCE Protection** - All authorization requests use PKCE (Proof Key for Code Exchange)
2. **Localhost Callback** - Callback server only accepts localhost connections
3. **Encrypted Storage** - Tokens are stored in OS-native keyring
4. **Network Isolation** - Credentials are isolated per network
