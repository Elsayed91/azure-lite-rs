//! Operation contracts for the Azure Container Registry API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/acr.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::acr::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Container Registry API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::acr::AcrClient`] instead.
pub struct AcrOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> AcrOps<'a> {
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

    /// Lists all the container registries under the specified subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.ContainerRegistry/registries`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`RegistryListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_registries(
        &self,
        subscription_id: &str,
    ) -> Result<RegistryListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.ContainerRegistry/registries",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_registries response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_registries response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the properties of the specified container registry.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerRegistry/registries/{registryName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `registryName` —  *(required)*
    ///
    /// # Response
    /// [`Registry`]
    #[allow(dead_code)]
    pub(crate) async fn get_registry(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> Result<Registry> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(registry_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_registry response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_registry response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a container registry.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerRegistry/registries/{registryName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `registryName` —  *(required)*
    ///
    /// # Request Body
    /// [`RegistryCreateRequest`]
    ///
    /// # Response
    /// [`Registry`]
    #[allow(dead_code)]
    pub(crate) async fn create_registry(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
        body: &RegistryCreateRequest,
    ) -> Result<Registry> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(registry_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_registry request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_registry response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_registry response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a container registry.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerRegistry/registries/{registryName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `registryName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_registry(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(registry_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_registries() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.ContainerRegistry/registries",
        )
        .returning_json(serde_json::to_value(RegistryListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AcrOps::new(&client);

        let result = ops.list_registries("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_registry() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerRegistry/registries/test-registryName")
            .returning_json(serde_json::to_value(Registry::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AcrOps::new(&client);

        let result = ops
            .get_registry(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-registryName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_registry() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerRegistry/registries/test-registryName")
            .returning_json(serde_json::to_value(Registry::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AcrOps::new(&client);

        let body = RegistryCreateRequest::fixture();
        let result = ops
            .create_registry(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-registryName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_registry() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerRegistry/registries/test-registryName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AcrOps::new(&client);

        let result = ops
            .delete_registry(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-registryName",
            )
            .await;
        assert!(result.is_ok());
    }
}
