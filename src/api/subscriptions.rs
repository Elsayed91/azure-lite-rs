//! Azure Subscriptions API client.
//!
//! Lists all Azure subscriptions accessible to the authenticated principal.
//! This is a tenant-level call — no subscription ID appears in the path.
//! The bearer token issued by `AzureCredential::get_token()` is tenant-scoped
//! and covers all subscriptions.

use crate::{
    AzureError, AzureHttpClient, Result,
    ops::subscriptions::SubscriptionsOps,
    types::subscriptions::{SubscriptionInfo, SubscriptionListResponse},
};

/// Client for the Azure Subscriptions API.
///
/// Wraps [`SubscriptionsOps`] with an ergonomic, auto-paginating method.
pub struct SubscriptionsClient<'a> {
    ops: SubscriptionsOps<'a>,
}

impl<'a> SubscriptionsClient<'a> {
    /// Create a new Azure Subscriptions API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: SubscriptionsOps::new(client),
        }
    }

    /// List all subscriptions accessible to the authenticated principal.
    ///
    /// Handles pagination automatically — returns **all** subscriptions.
    ///
    /// This is a tenant-level call; no subscription ID is required in the path.
    /// The bearer token used by the underlying client is tenant-scoped and
    /// grants access to every subscription under the tenant.
    pub async fn list(&self) -> Result<Vec<SubscriptionInfo>> {
        let mut all = Vec::new();
        let first_page = self.ops.list_subscriptions().await?;
        all.extend(first_page.value);

        let mut next_link = first_page.next_link;
        while let Some(url) = next_link {
            let page = self.fetch_page(&url).await?;
            all.extend(page.value);
            next_link = page.next_link;
        }

        Ok(all)
    }

    async fn fetch_page(&self, url: &str) -> Result<SubscriptionListResponse> {
        let response = self.ops.client.get(url).await?;
        let response = response.error_for_status().await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|e| AzureError::InvalidResponse {
                message: format!("Failed to read subscriptions page: {e}"),
                body: None,
            })?;
        serde_json::from_slice(&bytes).map_err(|e| AzureError::InvalidResponse {
            message: format!("Failed to parse subscriptions page: {e}"),
            body: Some(String::from_utf8_lossy(&bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    #[tokio::test]
    async fn list_returns_all_subscriptions_single_page() {
        let mut mock = MockClient::new();
        mock.expect_get("/subscriptions")
            .returning_json(serde_json::json!({
                "value": [
                    {
                        "subscriptionId": "35010625-993d-403a-a18c-9f32945c327e",
                        "displayName": "Fractal Production",
                        "tenantId": "1dff187b-fb27-4db3-a7b9-286a24aaa818",
                        "state": "Enabled"
                    }
                ]
            }));
        let client = make_client(mock);
        let subs = client.subscriptions().list().await.expect("list failed");
        assert_eq!(subs.len(), 1);
        assert_eq!(
            subs[0].subscription_id.as_deref(),
            Some("35010625-993d-403a-a18c-9f32945c327e")
        );
        assert_eq!(subs[0].display_name.as_deref(), Some("Fractal Production"));
        assert_eq!(subs[0].state.as_deref(), Some("Enabled"));
    }

    #[tokio::test]
    async fn list_returns_empty_when_no_subscriptions() {
        let mut mock = MockClient::new();
        mock.expect_get("/subscriptions")
            .returning_json(serde_json::json!({ "value": [] }));
        let client = make_client(mock);
        let subs = client.subscriptions().list().await.expect("list failed");
        assert_eq!(subs.len(), 0);
    }
}
