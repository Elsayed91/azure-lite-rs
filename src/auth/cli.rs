//! Azure CLI credential provider.
//!
//! Calls `az account get-access-token --resource https://management.azure.com/ --output json`
//! and parses the resulting JSON for `accessToken` and `expiresOn`.

use crate::auth::token::{AccessToken, CachedToken};
use crate::error::AzureError;

/// Azure CLI credential provider.
///
/// Delegates to `az account get-access-token` — requires the user to be logged
/// in via `az login` or `az login --service-principal`.
pub struct AzureCliCredential {
    cache: CachedToken,
}

impl Default for AzureCliCredential {
    fn default() -> Self {
        Self::new()
    }
}

impl AzureCliCredential {
    /// Create a new Azure CLI credential.
    pub fn new() -> Self {
        Self {
            cache: CachedToken::new(),
        }
    }

    /// Acquire a token by calling the Azure CLI.
    pub async fn get_token(&self) -> Result<AccessToken, AzureError> {
        if let Some(cached) = self.cache.get().await {
            return Ok(cached);
        }

        let token = self.fetch_token("https://management.azure.com/").await?;
        self.cache.set(token.clone()).await;
        Ok(token)
    }

    /// Acquire a token for an arbitrary resource URL (e.g. `https://graph.microsoft.com/`).
    ///
    /// Converts an OAuth2 scope (`https://graph.microsoft.com/.default`) to the
    /// resource URL the CLI expects by stripping the `/.default` suffix.
    pub(crate) async fn get_token_for_scope(&self, scope: &str) -> Result<AccessToken, AzureError> {
        let resource = scope.trim_end_matches("/.default");
        // Ensure trailing slash for az CLI resource URLs
        let resource = if resource.ends_with('/') {
            resource.to_string()
        } else {
            format!("{resource}/")
        };
        self.fetch_token(&resource).await
    }

    async fn fetch_token(&self, resource: &str) -> Result<AccessToken, AzureError> {
        let output = tokio::process::Command::new("az")
            .args([
                "account",
                "get-access-token",
                "--resource",
                resource,
                "--output",
                "json",
            ])
            .output()
            .await
            .map_err(|e| AzureError::Auth {
                message: format!("Failed to run 'az account get-access-token': {e}"),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AzureError::Auth {
                message: format!("az account get-access-token failed: {stderr}"),
            });
        }

        let body = String::from_utf8_lossy(&output.stdout);
        parse_cli_token_response(&body)
    }
}

/// Parse the JSON output of `az account get-access-token`.
///
/// Example output:
/// ```json
/// {
///   "accessToken": "eyJhb...",
///   "expiresOn": "2024-01-01 12:00:00.000000",
///   "subscription": "...",
///   "tenant": "...",
///   "tokenType": "Bearer"
/// }
/// ```
fn parse_cli_token_response(body: &str) -> Result<AccessToken, AzureError> {
    let val: serde_json::Value =
        serde_json::from_str(body.trim()).map_err(|e| AzureError::InvalidResponse {
            message: format!("Failed to parse az CLI token response: {e}"),
            body: Some(body.to_string()),
        })?;

    let token = val
        .get("accessToken")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AzureError::InvalidResponse {
            message: "az CLI token response missing accessToken".into(),
            body: Some(body.to_string()),
        })?
        .to_string();

    // `expiresOn` is a datetime string like "2024-01-01 12:00:00.000000"
    // or the newer `expires_on` (unix timestamp string).
    let expires_at = if let Some(ts) = val.get("expires_on").and_then(|v| v.as_str()) {
        ts.parse::<u64>().unwrap_or_else(|_| fallback_expiry())
    } else if let Some(datetime_str) = val.get("expiresOn").and_then(|v| v.as_str()) {
        parse_expires_on(datetime_str)
    } else {
        fallback_expiry()
    };

    Ok(AccessToken::new(token, expires_at))
}

/// Parse "2024-01-01 12:00:00.000000" into a Unix timestamp.
///
/// If parsing fails, falls back to 55 minutes from now.
fn parse_expires_on(s: &str) -> u64 {
    // Format: "YYYY-MM-DD HH:MM:SS.ffffff"
    let s = s.trim();
    // Try to parse with chrono — if unavailable, fall back
    // We parse manually to avoid adding a chrono dependency here
    // Format: YYYY-MM-DD HH:MM:SS[.ffffff]
    let date_time = s.split('.').next().unwrap_or(s);
    let parts: Vec<&str> = date_time.split_whitespace().collect();
    if parts.len() != 2 {
        return fallback_expiry();
    }
    let date_parts: Vec<u32> = parts[0].split('-').filter_map(|p| p.parse().ok()).collect();
    let time_parts: Vec<u32> = parts[1].split(':').filter_map(|p| p.parse().ok()).collect();
    if date_parts.len() != 3 || time_parts.len() != 3 {
        return fallback_expiry();
    }

    // Approximate: days since epoch (UTC, ignoring leap seconds)
    let (year, month, day) = (date_parts[0], date_parts[1], date_parts[2]);
    let (hour, min, sec) = (time_parts[0], time_parts[1], time_parts[2]);

    // Days from 1970-01-01 to year-month-day (Gregorian, no timezone)
    let days = days_since_epoch(year, month, day);
    days * 86400 + hour as u64 * 3600 + min as u64 * 60 + sec as u64
}

fn days_since_epoch(year: u32, month: u32, day: u32) -> u64 {
    // Zeller-style calculation for days from 1970-01-01
    // This is not perfectly accurate for all edge cases but good enough for token expiry
    let y = year as i64;
    let m = month as i64;
    let d = day as i64;

    // Julian Day Number formula
    let a = (14 - m) / 12;
    let yr = y + 4800 - a;
    let mr = m + 12 * a - 3;
    let jdn = d + (153 * mr + 2) / 5 + 365 * yr + yr / 4 - yr / 100 + yr / 400 - 32045;

    // Julian Day Number of 1970-01-01 is 2440588
    let epoch_jdn: i64 = 2_440_588;
    (jdn - epoch_jdn).max(0) as u64
}

fn fallback_expiry() -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    now + 55 * 60 // 55 minutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli_response_with_expires_on_timestamp() {
        let body =
            r#"{"accessToken": "tok123", "expires_on": "9999999999", "tokenType": "Bearer"}"#;
        let tok = parse_cli_token_response(body).unwrap();
        assert_eq!(tok.token, "tok123");
        assert_eq!(tok.expires_at, 9_999_999_999);
    }

    #[test]
    fn parse_cli_response_with_expires_on_datetime() {
        // A date far in the future so seconds_remaining() > 0
        let body = r#"{"accessToken": "tok456", "expiresOn": "2099-01-01 12:00:00.000000", "tokenType": "Bearer"}"#;
        let tok = parse_cli_token_response(body).unwrap();
        assert_eq!(tok.token, "tok456");
        assert!(tok.expires_at > 0);
        assert!(tok.seconds_remaining() > 0);
    }

    #[test]
    fn parse_cli_response_missing_token() {
        let body = r#"{"expiresOn": "2099-01-01 12:00:00"}"#;
        let err = parse_cli_token_response(body).unwrap_err();
        assert!(matches!(err, AzureError::InvalidResponse { .. }));
    }

    #[test]
    fn days_since_epoch_known_dates() {
        // 1970-01-01 = day 0
        assert_eq!(days_since_epoch(1970, 1, 1), 0);
        // 1970-01-02 = day 1
        assert_eq!(days_since_epoch(1970, 1, 2), 1);
        // 1970-02-01 = day 31
        assert_eq!(days_since_epoch(1970, 2, 1), 31);
    }

    #[test]
    fn parse_expires_on_known_date() {
        // 1970-01-01 00:00:00 => 0
        let ts = parse_expires_on("1970-01-01 00:00:00.000000");
        assert_eq!(ts, 0);

        // 1970-01-01 01:00:00 => 3600
        let ts = parse_expires_on("1970-01-01 01:00:00.000000");
        assert_eq!(ts, 3600);
    }
}
