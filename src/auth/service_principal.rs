//! Azure AD service principal authentication.
//!
//! Authenticates using client credentials flow:
//! `POST https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token`
//! with `client_id`, `client_secret`, `grant_type=client_credentials`.

use crate::auth::token::{AccessToken, CachedToken};
use crate::error::AzureError;

const AZURE_TOKEN_URL: &str = "https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token";
const ARM_SCOPE: &str = "https://management.azure.com/.default";

/// Authenticates as a service principal using client credentials.
pub struct ServicePrincipalCredential {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    http: reqwest::Client,
    cache: CachedToken,
}

impl ServicePrincipalCredential {
    /// Create from explicit values.
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        tenant_id: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            tenant_id: tenant_id.into(),
            http: reqwest::Client::new(),
            cache: CachedToken::new(),
        }
    }

    /// Load from `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_TENANT_ID` env vars.
    pub fn from_env() -> Option<Self> {
        let client_id = std::env::var("AZURE_CLIENT_ID").ok()?;
        let client_secret = std::env::var("AZURE_CLIENT_SECRET").ok()?;
        let tenant_id = std::env::var("AZURE_TENANT_ID").ok()?;
        Some(Self::new(client_id, client_secret, tenant_id))
    }

    /// Acquire an access token for the ARM management plane.
    pub async fn get_token(&self) -> Result<AccessToken, AzureError> {
        if let Some(cached) = self.cache.get().await {
            return Ok(cached);
        }

        let token = self.fetch_token(ARM_SCOPE).await?;
        self.cache.set(token.clone()).await;
        Ok(token)
    }

    /// Acquire an access token for an arbitrary OAuth2 scope (e.g. Graph).
    ///
    /// Unlike `get_token`, this does not cache — callers that need caching
    /// should manage it themselves (Graph tokens are acquired infrequently).
    pub(crate) async fn get_token_for_scope(&self, scope: &str) -> Result<AccessToken, AzureError> {
        self.fetch_token(scope).await
    }

    async fn fetch_token(&self, scope: &str) -> Result<AccessToken, AzureError> {
        let url = AZURE_TOKEN_URL.replace("{tenant}", &self.tenant_id);

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("scope", scope),
        ];

        let response = self
            .http
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| AzureError::Auth {
                message: format!("Service principal token request failed: {e}"),
            })?;

        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();

        if status != 200 {
            let parsed: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();
            let desc = parsed
                .get("error_description")
                .and_then(|v| v.as_str())
                .unwrap_or(&body)
                .to_string();
            return Err(AzureError::Auth {
                message: format!("Service principal auth failed ({status}): {desc}"),
            });
        }

        parse_token_response(&body)
    }
}

/// Parse a successful OAuth2 token response.
pub(crate) fn parse_token_response(body: &str) -> Result<AccessToken, AzureError> {
    let val: serde_json::Value =
        serde_json::from_str(body).map_err(|e| AzureError::InvalidResponse {
            message: format!("Failed to parse token response: {e}"),
            body: Some(body.to_string()),
        })?;

    let token = val
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AzureError::InvalidResponse {
            message: "Token response missing access_token".into(),
            body: Some(body.to_string()),
        })?
        .to_string();

    // Azure returns `expires_in` (seconds from now) or `expires_on` (unix timestamp).
    let expires_at = if let Some(expires_on) = val.get("expires_on").and_then(|v| v.as_str()) {
        expires_on.parse::<u64>().unwrap_or(0)
    } else if let Some(expires_in) = val.get("expires_in").and_then(|v| v.as_u64()) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now + expires_in
    } else {
        // Fallback: assume 1 hour
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now + 3600
    };

    Ok(AccessToken::new(token, expires_at))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_token_response_with_expires_in() {
        let body = r#"{"access_token": "eyJhb...", "expires_in": 3599, "token_type": "Bearer"}"#;
        let tok = parse_token_response(body).unwrap();
        assert_eq!(tok.token, "eyJhb...");
        assert!(tok.seconds_remaining() > 3000);
    }

    #[test]
    fn parse_token_response_with_expires_on() {
        let body =
            r#"{"access_token": "mytoken", "expires_on": "9999999999", "token_type": "Bearer"}"#;
        let tok = parse_token_response(body).unwrap();
        assert_eq!(tok.token, "mytoken");
        assert_eq!(tok.expires_at, 9_999_999_999);
    }

    #[test]
    fn parse_token_response_missing_token() {
        let body = r#"{"expires_in": 3600}"#;
        let err = parse_token_response(body).unwrap_err();
        assert!(matches!(err, AzureError::InvalidResponse { .. }));
    }

    #[test]
    fn parse_token_response_invalid_json() {
        let err = parse_token_response("not json").unwrap_err();
        assert!(matches!(err, AzureError::InvalidResponse { .. }));
    }
}
