//! Azure Managed Identity (IMDS) authentication.
//!
//! When running on Azure (VMs, App Service, AKS, etc.), the Instance Metadata
//! Service (IMDS) provides tokens without any credentials.
//!
//! Endpoint: `http://169.254.169.254/metadata/identity/oauth2/token?api-version=2018-02-01&resource={resource}`

use crate::auth::service_principal::parse_token_response;
use crate::auth::token::{AccessToken, CachedToken};
use crate::error::AzureError;

const IMDS_BASE: &str =
    "http://169.254.169.254/metadata/identity/oauth2/token?api-version=2018-02-01&resource=";

/// Maximum time to wait for the IMDS endpoint to respond.
const IMDS_TIMEOUT_SECS: u64 = 3;

/// Azure Managed Identity credential via IMDS.
pub struct ManagedIdentityCredential {
    /// Optional: client ID of a user-assigned managed identity.
    client_id: Option<String>,
    http: reqwest::Client,
    cache: CachedToken,
}

impl ManagedIdentityCredential {
    /// Create for the system-assigned managed identity.
    pub fn new() -> Self {
        Self {
            client_id: None,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(IMDS_TIMEOUT_SECS))
                .build()
                .unwrap_or_default(),
            cache: CachedToken::new(),
        }
    }

    /// Create for a user-assigned managed identity by client ID.
    pub fn with_client_id(client_id: impl Into<String>) -> Self {
        Self {
            client_id: Some(client_id.into()),
            ..Self::new()
        }
    }

    /// Acquire an access token from the IMDS endpoint.
    pub async fn get_token(&self) -> Result<AccessToken, AzureError> {
        if let Some(cached) = self.cache.get().await {
            return Ok(cached);
        }

        let token = self.fetch_token("https://management.azure.com/").await?;
        self.cache.set(token.clone()).await;
        Ok(token)
    }

    /// Acquire a token for an arbitrary resource (e.g. `https://graph.microsoft.com/.default`).
    ///
    /// Converts an OAuth2 scope to the resource URL expected by IMDS by stripping `/.default`.
    pub(crate) async fn get_token_for_scope(&self, scope: &str) -> Result<AccessToken, AzureError> {
        let resource = scope.trim_end_matches("/.default");
        let resource = if resource.ends_with('/') {
            resource.to_string()
        } else {
            format!("{resource}/")
        };
        self.fetch_token(&resource).await
    }

    async fn fetch_token(&self, resource: &str) -> Result<AccessToken, AzureError> {
        let mut url = format!("{IMDS_BASE}{}", urlencoding::encode(resource));
        if let Some(ref cid) = self.client_id {
            url.push_str(&format!("&client_id={}", urlencoding::encode(cid)));
        }

        let response = self
            .http
            .get(&url)
            .header("Metadata", "true")
            .send()
            .await
            .map_err(|e| AzureError::Auth {
                message: format!("IMDS request failed: {e}"),
            })?;

        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();

        if status != 200 {
            return Err(AzureError::Auth {
                message: format!("IMDS auth failed ({status}): {body}"),
            });
        }

        parse_token_response(&body)
    }
}

impl Default for ManagedIdentityCredential {
    fn default() -> Self {
        Self::new()
    }
}
