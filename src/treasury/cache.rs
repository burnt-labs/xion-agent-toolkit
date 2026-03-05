//! Treasury Data Cache
//!
//! In-memory cache for treasury data to reduce API calls.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::types::{TreasuryInfo, TreasuryListItem};

/// Cache entry with expiration
#[derive(Debug)]
struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

impl<T> CacheEntry<T> {
    fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            expires_at: Instant::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

/// Treasury data cache
///
/// Provides in-memory caching for treasury data with configurable TTL.
/// Reduces the number of API calls needed for frequently accessed data.
#[derive(Debug)]
pub struct TreasuryCache {
    /// Treasury list cache
    treasury_list: Option<CacheEntry<Vec<TreasuryListItem>>>,
    /// Individual treasury cache
    treasuries: HashMap<String, CacheEntry<TreasuryInfo>>,
    /// Default TTL (5 minutes)
    default_ttl: Duration,
}

impl TreasuryCache {
    /// Create new cache with default TTL of 5 minutes
    pub fn new() -> Self {
        Self {
            treasury_list: None,
            treasuries: HashMap::new(),
            default_ttl: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Get cached treasury list
    ///
    /// Returns `Some(list)` if cached and not expired, `None` otherwise
    pub fn get_treasury_list(&self) -> Option<Vec<TreasuryListItem>> {
        self.treasury_list
            .as_ref()
            .filter(|entry| !entry.is_expired())
            .map(|entry| entry.data.clone())
    }

    /// Set treasury list cache
    pub fn set_treasury_list(&mut self, list: Vec<TreasuryListItem>) {
        self.treasury_list = Some(CacheEntry::new(list, self.default_ttl));
    }

    /// Get cached treasury by address
    ///
    /// Returns `Some(treasury)` if cached and not expired, `None` otherwise
    pub fn get_treasury(&self, address: &str) -> Option<TreasuryInfo> {
        self.treasuries
            .get(address)
            .filter(|entry| !entry.is_expired())
            .map(|entry| entry.data.clone())
    }

    /// Set treasury cache for a specific address
    pub fn set_treasury(&mut self, address: String, treasury: TreasuryInfo) {
        self.treasuries
            .insert(address, CacheEntry::new(treasury, self.default_ttl));
    }

    /// Clear all cached data
    pub fn clear(&mut self) {
        self.treasury_list = None;
        self.treasuries.clear();
    }

    /// Set custom TTL for cache entries
    ///
    /// Only affects new entries, not existing ones
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = ttl;
        self
    }

    /// Remove expired entries
    pub fn cleanup_expired(&mut self) {
        // Clean up expired treasury list
        if let Some(ref entry) = self.treasury_list {
            if entry.is_expired() {
                self.treasury_list = None;
            }
        }

        // Clean up expired individual treasuries
        self.treasuries.retain(|_, entry| !entry.is_expired());
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let total_treasuries = self.treasuries.len();
        let expired_treasuries = self
            .treasuries
            .values()
            .filter(|entry| entry.is_expired())
            .count();

        CacheStats {
            has_list: self.treasury_list.is_some(),
            list_expired: self
                .treasury_list
                .as_ref()
                .map(|e| e.is_expired())
                .unwrap_or(false),
            total_treasuries,
            expired_treasuries,
        }
    }
}

impl Default for TreasuryCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Whether treasury list is cached
    pub has_list: bool,
    /// Whether cached list is expired
    pub list_expired: bool,
    /// Total number of cached treasuries
    pub total_treasuries: usize,
    /// Number of expired treasury entries
    pub expired_treasuries: usize,
}

#[cfg(test)]
mod tests {
    use super::super::types::TreasuryParams;
    use super::*;

    #[test]
    fn test_cache_new() {
        let cache = TreasuryCache::new();
        let stats = cache.stats();
        assert!(!stats.has_list);
        assert_eq!(stats.total_treasuries, 0);
    }

    #[test]
    fn test_cache_treasury_list() {
        let mut cache = TreasuryCache::new();

        // Initially no cached list
        assert!(cache.get_treasury_list().is_none());

        // Set list
        let list = vec![TreasuryListItem {
            address: "xion1abc".to_string(),
            admin: None,
            balance: "1000".to_string(),
            name: None,
            created_at: None,
        }];
        cache.set_treasury_list(list.clone());

        // Get cached list
        let cached = cache.get_treasury_list();
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);
    }

    #[test]
    fn test_cache_treasury() {
        let mut cache = TreasuryCache::new();

        let address = "xion1abc";

        // Initially no cached treasury
        assert!(cache.get_treasury(address).is_none());

        // Set treasury
        let treasury = TreasuryInfo {
            address: address.to_string(),
            admin: None,
            balance: "1000".to_string(),
            params: TreasuryParams {
                display_url: None,
                redirect_url: "https://example.com".to_string(),
                icon_url: "https://example.com/icon.png".to_string(),
                metadata: None,
            },
            fee_config: None,
            grant_configs: None,
        };
        cache.set_treasury(address.to_string(), treasury);

        // Get cached treasury
        let cached = cache.get_treasury(address);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().address, address);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = TreasuryCache::new();

        // Add some data
        cache.set_treasury_list(vec![]);
        cache.set_treasury(
            "xion1abc".to_string(),
            TreasuryInfo {
                address: "xion1abc".to_string(),
                admin: None,
                balance: "1000".to_string(),
                params: TreasuryParams {
                    display_url: None,
                    redirect_url: "https://example.com".to_string(),
                    icon_url: "https://example.com/icon.png".to_string(),
                    metadata: None,
                },
                fee_config: None,
                grant_configs: None,
            },
        );

        // Clear cache
        cache.clear();

        // Verify cleared
        assert!(cache.get_treasury_list().is_none());
        assert!(cache.get_treasury("xion1abc").is_none());
    }

    #[test]
    fn test_cache_custom_ttl() {
        let cache = TreasuryCache::new().with_ttl(Duration::from_secs(60));
        assert_eq!(cache.default_ttl, Duration::from_secs(60));
    }

    #[test]
    fn test_cache_expiration() {
        let mut cache = TreasuryCache::new().with_ttl(Duration::from_millis(10));

        // Set treasury
        let treasury = TreasuryInfo {
            address: "xion1abc".to_string(),
            admin: None,
            balance: "1000".to_string(),
            params: TreasuryParams {
                display_url: None,
                redirect_url: "https://example.com".to_string(),
                icon_url: "https://example.com/icon.png".to_string(),
                metadata: None,
            },
            fee_config: None,
            grant_configs: None,
        };
        cache.set_treasury("xion1abc".to_string(), treasury);

        // Should be cached immediately
        assert!(cache.get_treasury("xion1abc").is_some());

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(20));

        // Should be expired now
        assert!(cache.get_treasury("xion1abc").is_none());
    }

    #[test]
    fn test_cache_cleanup_expired() {
        let mut cache = TreasuryCache::new().with_ttl(Duration::from_millis(10));

        // Set treasuries
        for i in 0..3 {
            let treasury = TreasuryInfo {
                address: format!("xion1abc{}", i),
                admin: None,
                balance: "1000".to_string(),
                params: TreasuryParams {
                    display_url: None,
                    redirect_url: "https://example.com".to_string(),
                    icon_url: "https://example.com/icon.png".to_string(),
                    metadata: None,
                },
                fee_config: None,
                grant_configs: None,
            };
            cache.set_treasury(format!("xion1abc{}", i), treasury);
        }

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(20));

        // Cleanup
        cache.cleanup_expired();

        // All should be cleaned up
        let stats = cache.stats();
        assert_eq!(stats.total_treasuries, 0);
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = TreasuryCache::new();

        let stats = cache.stats();
        assert!(!stats.has_list);
        assert_eq!(stats.total_treasuries, 0);

        // Add some data
        cache.set_treasury_list(vec![]);
        let treasury = TreasuryInfo {
            address: "xion1abc".to_string(),
            admin: None,
            balance: "1000".to_string(),
            params: TreasuryParams {
                display_url: None,
                redirect_url: "https://example.com".to_string(),
                icon_url: "https://example.com/icon.png".to_string(),
                metadata: None,
            },
            fee_config: None,
            grant_configs: None,
        };
        cache.set_treasury("xion1abc".to_string(), treasury);

        let stats = cache.stats();
        assert!(stats.has_list);
        assert!(!stats.list_expired);
        assert_eq!(stats.total_treasuries, 1);
        assert_eq!(stats.expired_treasuries, 0);
    }
}
