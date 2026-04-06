//! Token types for Azure AD authentication.

use std::sync::Arc;
use tokio::sync::RwLock;

/// An Azure AD access token with its expiration time.
#[derive(Debug, Clone)]
pub struct AccessToken {
    /// The bearer token string.
    pub token: String,
    /// Unix timestamp (seconds since epoch) when the token expires.
    pub expires_at: u64,
}

impl AccessToken {
    /// Create a new access token.
    pub fn new(token: impl Into<String>, expires_at: u64) -> Self {
        Self {
            token: token.into(),
            expires_at,
        }
    }

    /// Seconds remaining until expiry (0 if already expired).
    pub fn seconds_remaining(&self) -> u64 {
        let now = now_secs();
        self.expires_at.saturating_sub(now)
    }

    /// Returns true if the token will expire within `margin_secs` seconds.
    pub fn expires_soon(&self, margin_secs: u64) -> bool {
        let now = now_secs();
        now + margin_secs >= self.expires_at
    }
}

/// Thread-safe token cache.
///
/// Tokens are reused if >5 minutes remain; refreshed proactively at 55-min mark
/// (Azure AD tokens are issued for 60-90 minutes, so this is conservative).
#[derive(Debug, Clone)]
pub struct CachedToken {
    inner: Arc<RwLock<Option<AccessToken>>>,
}

/// Minimum seconds remaining for a cached token to be considered valid.
/// This is 5 minutes (300 seconds).
const MIN_REMAINING_SECS: u64 = 300;

impl Default for CachedToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CachedToken {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(None)),
        }
    }

    /// Return the cached token if it has >5 minutes remaining.
    pub async fn get(&self) -> Option<AccessToken> {
        let guard = self.inner.read().await;
        if let Some(ref tok) = *guard
            && !tok.expires_soon(MIN_REMAINING_SECS)
        {
            return Some(tok.clone());
        }
        None
    }

    /// Store a new token, replacing any existing one.
    pub async fn set(&self, token: AccessToken) {
        let mut guard = self.inner.write().await;
        *guard = Some(token);
    }

    /// Clear the cache (e.g., after a 401 response).
    pub async fn clear(&self) {
        let mut guard = self.inner.write().await;
        *guard = None;
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_seconds_remaining_future() {
        let now = now_secs();
        let tok = AccessToken::new("t", now + 3600);
        // Should be close to 3600
        assert!(tok.seconds_remaining() > 3500);
    }

    #[test]
    fn token_seconds_remaining_past() {
        let tok = AccessToken::new("t", 0);
        assert_eq!(tok.seconds_remaining(), 0);
    }

    #[test]
    fn token_expires_soon_short_margin() {
        let now = now_secs();
        let tok = AccessToken::new("t", now + 100);
        assert!(tok.expires_soon(200));
        assert!(!tok.expires_soon(50));
    }

    #[tokio::test]
    async fn cached_token_empty() {
        let cache = CachedToken::new();
        assert!(cache.get().await.is_none());
    }

    #[tokio::test]
    async fn cached_token_valid_token() {
        let now = now_secs();
        let cache = CachedToken::new();
        cache.set(AccessToken::new("abc", now + 3600)).await;
        let tok = cache.get().await.expect("should have token");
        assert_eq!(tok.token, "abc");
    }

    #[tokio::test]
    async fn cached_token_expired_token_not_returned() {
        let cache = CachedToken::new();
        // Token that expires in only 1 second (below 5-minute threshold)
        let now = now_secs();
        cache.set(AccessToken::new("stale", now + 1)).await;
        assert!(cache.get().await.is_none());
    }

    #[tokio::test]
    async fn cached_token_clear() {
        let now = now_secs();
        let cache = CachedToken::new();
        cache.set(AccessToken::new("x", now + 3600)).await;
        cache.clear().await;
        assert!(cache.get().await.is_none());
    }
}
