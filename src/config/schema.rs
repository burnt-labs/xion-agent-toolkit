use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Current active network (testnet, mainnet)
    pub network: String,

    /// Version of the config schema
    pub version: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: "testnet".to_string(),
            version: "1.0".to_string(),
        }
    }
}

/// Default refresh token lifetime in seconds (30 days)
pub const DEFAULT_REFRESH_TOKEN_LIFETIME_SECS: i64 = 30 * 24 * 60 * 60; // 30 days

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    /// User's access token
    pub access_token: String,

    /// User's refresh token
    pub refresh_token: String,

    /// Access token expiration time (ISO 8601 format)
    pub expires_at: String,

    /// Refresh token expiration time (ISO 8601 format)
    /// Defaults to 30 days from creation if not provided by the OAuth2 server
    #[serde(default)]
    pub refresh_token_expires_at: Option<String>,

    /// Optional: User's Xion address
    pub xion_address: Option<String>,
    /// Space-separated OAuth2 scopes granted by the authorization server.
    /// None if credentials were created before scope tracking was added.
    #[serde(default)]
    pub scope: Option<String>,
}

impl UserCredentials {
    /// Check if the credentials contain a specific OAuth2 scope.
    pub fn has_scope(&self, required_scope: &str) -> bool {
        self.scope
            .as_ref()
            .map(|s| s.split_whitespace().any(|scope| scope == required_scope))
            .unwrap_or(false)
    }

    /// Check if the credentials contain all specified OAuth2 scopes.
    pub fn has_all_scopes(&self, required_scopes: &[&str]) -> bool {
        required_scopes.iter().all(|s| self.has_scope(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_creds(scope: Option<&str>) -> UserCredentials {
        UserCredentials {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
            refresh_token_expires_at: None,
            xion_address: None,
            scope: scope.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_has_scope_with_valid_scope() {
        let creds = make_creds(Some(
            "xion:identity:read xion:blockchain:read xion:transactions:submit",
        ));
        assert!(creds.has_scope("xion:identity:read"));
        assert!(creds.has_scope("xion:blockchain:read"));
        assert!(creds.has_scope("xion:transactions:submit"));
        assert!(!creds.has_scope("xion:mgr:read"));
    }

    #[test]
    fn test_has_scope_with_dev_mode_scopes() {
        let creds = make_creds(Some("xion:identity:read xion:blockchain:read xion:transactions:submit xion:mgr:read xion:mgr:write"));
        assert!(creds.has_all_scopes(&["xion:mgr:read", "xion:mgr:write"]));
    }

    #[test]
    fn test_has_scope_with_none_scope() {
        let creds = make_creds(None);
        assert!(!creds.has_scope("xion:identity:read"));
        assert!(!creds.has_all_scopes(&["xion:mgr:read"]));
    }

    #[test]
    fn test_has_all_scopes_partial_match() {
        let creds = make_creds(Some("xion:identity:read xion:mgr:read"));
        assert!(!creds.has_all_scopes(&["xion:mgr:read", "xion:mgr:write"]));
        assert!(creds.has_all_scopes(&["xion:identity:read", "xion:mgr:read"]));
    }
}
