//! Operation contracts for the Azure Resource Graph API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/resource_graph.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::resource_graph::*;
use crate::{AzureHttpClient, Result};

/// Raw HTTP operations for the Azure Resource Graph API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::resource_graph::ResourceGraphClient`] instead.
pub struct ResourceGraphOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> ResourceGraphOps<'a> {
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

    /// Execute a KQL query across one or more subscriptions.
    ///
    /// **Azure API**: `POST /providers/Microsoft.ResourceGraph/resources`
    ///
    /// # Request Body
    /// [`ResourceGraphRequest`]
    ///
    /// # Response
    /// [`ResourceGraphResponse`]
    #[allow(dead_code)]
    pub(crate) async fn query_resources(
        &self,
        body: &ResourceGraphRequest,
    ) -> Result<ResourceGraphResponse> {
        let url = format!(
            "{}/providers/Microsoft.ResourceGraph/resources",
            self.base_url(),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2021-03-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize query_resources request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read query_resources response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse query_resources response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_resources() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/providers/Microsoft.ResourceGraph/resources")
            .returning_json(serde_json::to_value(ResourceGraphResponse::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ResourceGraphOps::new(&client);

        let body = ResourceGraphRequest::fixture();
        let result = ops.query_resources(&body).await;
        assert!(result.is_ok());
    }
}
