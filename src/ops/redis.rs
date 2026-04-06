//! Operation contracts for the Azure Redis Cache API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/redis.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::redis::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Redis Cache API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::redis::RedisClient`] instead.
pub struct RedisOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> RedisOps<'a> {
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

    /// Gets all Redis caches in the specified subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Cache/redis`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`RedisListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_caches(&self, subscription_id: &str) -> Result<RedisListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Cache/redis",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_caches response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_caches response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists all Redis caches in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`RedisListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_caches_by_resource_group(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<RedisListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_caches_by_resource_group response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_caches_by_resource_group response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a Redis cache (resource description).
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Response
    /// [`RedisResource`]
    #[allow(dead_code)]
    pub(crate) async fn get_cache(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> Result<RedisResource> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_cache response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_cache response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or replace (overwrite/recreate, with potential downtime) an existing Redis cache.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Request Body
    /// [`RedisCreateRequest`]
    ///
    /// # Response
    /// [`RedisResource`]
    #[allow(dead_code)]
    pub(crate) async fn create_cache(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        body: &RedisCreateRequest,
    ) -> Result<RedisResource> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_cache request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_cache response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_cache response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a Redis cache.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_cache(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Retrieve a Redis cache's access keys.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}/listKeys`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Response
    /// [`RedisAccessKeys`]
    #[allow(dead_code)]
    pub(crate) async fn list_keys(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> Result<RedisAccessKeys> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}/listKeys",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_keys response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_keys response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Regenerate Redis cache's access keys.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}/regenerateKey`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Request Body
    /// [`RedisRegenerateKeyParameters`]
    ///
    /// # Response
    /// [`RedisAccessKeys`]
    #[allow(dead_code)]
    pub(crate) async fn regenerate_key(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        body: &RedisRegenerateKeyParameters,
    ) -> Result<RedisAccessKeys> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}/regenerateKey",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize regenerate_key request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read regenerate_key response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse regenerate_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Reboot specified Redis node(s).
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}/forceReboot`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Request Body
    /// [`RedisRebootParameters`]
    ///
    /// # Response
    /// [`RedisForceRebootResponse`]
    #[allow(dead_code)]
    pub(crate) async fn force_reboot(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        body: &RedisRebootParameters,
    ) -> Result<RedisForceRebootResponse> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}/forceReboot",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize force_reboot request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read force_reboot response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse force_reboot response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Import data into Redis cache.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}/import`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Request Body
    /// [`ImportRDBParameters`]
    #[allow(dead_code)]
    pub(crate) async fn import_data(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        body: &ImportRDBParameters,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}/import",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize import_data request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Export data from the redis cache to blobs in a container.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Cache/redis/{name}/export`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Request Body
    /// [`ExportRDBParameters`]
    #[allow(dead_code)]
    pub(crate) async fn export_data(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        body: &ExportRDBParameters,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/redis/{}/export",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize export_data request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        response.error_for_status().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_caches() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Cache/redis")
            .returning_json(serde_json::to_value(RedisListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let result = ops.list_caches("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_caches_by_resource_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis")
            .returning_json(serde_json::to_value(RedisListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let result = ops
            .list_caches_by_resource_group("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_cache() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name")
            .returning_json(serde_json::to_value(RedisResource::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let result = ops
            .get_cache("test-subscriptionId", "test-resourceGroupName", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_cache() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name")
            .returning_json(serde_json::to_value(RedisResource::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let body = RedisCreateRequest::fixture();
        let result = ops
            .create_cache(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_cache() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let result = ops
            .delete_cache("test-subscriptionId", "test-resourceGroupName", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name/listKeys")
            .returning_json(serde_json::to_value(RedisAccessKeys::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let result = ops
            .list_keys("test-subscriptionId", "test-resourceGroupName", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_regenerate_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name/regenerateKey")
            .returning_json(serde_json::to_value(RedisAccessKeys::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let body = RedisRegenerateKeyParameters::fixture();
        let result = ops
            .regenerate_key(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_force_reboot() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name/forceReboot")
            .returning_json(serde_json::to_value(RedisForceRebootResponse::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let body = RedisRebootParameters::fixture();
        let result = ops
            .force_reboot(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_import_data() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name/import")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let body = ImportRDBParameters::fixture();
        let result = ops
            .import_data(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_export_data() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Cache/redis/test-name/export")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RedisOps::new(&client);

        let body = ExportRDBParameters::fixture();
        let result = ops
            .export_data(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }
}
