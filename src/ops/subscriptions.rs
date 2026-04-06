//! Operation contracts for the Azure Subscriptions API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/subscriptions.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::subscriptions::*;
use crate::{AzureHttpClient, Result};

/// Raw HTTP operations for the Azure Subscriptions API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::subscriptions::SubscriptionsClient`] instead.
pub struct SubscriptionsOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> SubscriptionsOps<'a> {
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://management.azure.com"
    }

    /// List all subscriptions accessible to the authenticated principal.
    ///
    /// **Azure API**: `GET /subscriptions`
    ///
    /// # Response
    /// [`SubscriptionListResponse`]
    #[allow(dead_code)]
    pub(crate) async fn list_subscriptions(&self) -> Result<SubscriptionListResponse> {
        let url = format!("{}/subscriptions", self.base_url(),);
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2022-12-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_subscriptions response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_subscriptions response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_subscriptions() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions")
            .returning_json(serde_json::to_value(SubscriptionListResponse::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SubscriptionsOps::new(&client);

        let result = ops.list_subscriptions().await;
        assert!(result.is_ok());
    }
}
