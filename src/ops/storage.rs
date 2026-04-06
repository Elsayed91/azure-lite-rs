//! Operation contracts for the Azure Storage API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/storage.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::storage::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Storage API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::storage::StorageClient`] instead.
pub struct StorageOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> StorageOps<'a> {
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

    /// Lists all storage accounts in a subscription
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Storage/storageAccounts`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`StorageAccountListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_storage_accounts(
        &self,
        subscription_id: &str,
    ) -> Result<StorageAccountListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Storage/storageAccounts",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_storage_accounts response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_storage_accounts response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists all storage accounts in a resource group
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`StorageAccountListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_storage_accounts_by_resource_group(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<StorageAccountListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!(
                        "Failed to read list_storage_accounts_by_resource_group response: {e}"
                    ),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!(
                "Failed to parse list_storage_accounts_by_resource_group response: {e}"
            ),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Returns the properties of a storage account
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Response
    /// [`StorageAccount`]
    #[allow(dead_code)]
    pub(crate) async fn get_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<StorageAccount> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_storage_account response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_storage_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates a new storage account
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Request Body
    /// [`StorageAccountCreateRequest`]
    ///
    /// # Response
    /// [`StorageAccount`]
    #[allow(dead_code)]
    pub(crate) async fn create_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        body: &StorageAccountCreateRequest,
    ) -> Result<StorageAccount> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_storage_account request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_storage_account response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_storage_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a storage account
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Response
    /// [`StorageAccount`]
    #[allow(dead_code)]
    pub(crate) async fn delete_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<StorageAccount> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
        let response = self.client.delete(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read delete_storage_account response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse delete_storage_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the access keys for a storage account
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/listKeys`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Response
    /// [`StorageAccountListKeysResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_keys(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<StorageAccountListKeysResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/listKeys",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
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

    /// Regenerates one of the access keys for a storage account
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/regenerateKey`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Request Body
    /// [`StorageAccountRegenerateKeyRequest`]
    ///
    /// # Response
    /// [`StorageAccountListKeysResult`]
    #[allow(dead_code)]
    pub(crate) async fn regenerate_key(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        body: &StorageAccountRegenerateKeyRequest,
    ) -> Result<StorageAccountListKeysResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/regenerateKey",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
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

    /// Updates properties of an existing storage account (partial PATCH — only provided fields
    /// are changed)
    ///
    /// **Azure API**: `PATCH /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Request Body
    /// [`StorageAccountUpdateRequest`]
    ///
    /// # Response
    /// [`StorageAccount`]
    #[allow(dead_code)]
    pub(crate) async fn patch_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        body: &StorageAccountUpdateRequest,
    ) -> Result<StorageAccount> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize patch_storage_account request: {e}"),
                body: None,
            })?;
        let response = self.client.patch(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read patch_storage_account response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse patch_storage_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the blob lifecycle management policy for a storage account
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Storage/storageAccounts/{accountName}/managementPolicies/default`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Response
    /// [`ManagementPolicy`]
    #[allow(dead_code)]
    pub(crate) async fn get_management_policy(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<ManagementPolicy> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Storage/storageAccounts/{}/managementPolicies/default",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_management_policy response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_management_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_storage_accounts() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Storage/storageAccounts",
        )
        .returning_json(serde_json::to_value(StorageAccountListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops.list_storage_accounts("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_storage_accounts_by_resource_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts")
            .returning_json(serde_json::to_value(StorageAccountListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .list_storage_accounts_by_resource_group(
                "test-subscriptionId",
                "test-resourceGroupName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_storage_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts/test-accountName")
            .returning_json(serde_json::to_value(StorageAccount::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .get_storage_account(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_storage_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts/test-accountName")
            .returning_json(serde_json::to_value(StorageAccount::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = StorageAccountCreateRequest::fixture();
        let result = ops
            .create_storage_account(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_storage_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts/test-accountName")
            .returning_json(serde_json::to_value(StorageAccount::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .delete_storage_account(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts/test-accountName/listKeys")
            .returning_json(serde_json::to_value(StorageAccountListKeysResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .list_keys(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_regenerate_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts/test-accountName/regenerateKey")
            .returning_json(serde_json::to_value(StorageAccountListKeysResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = StorageAccountRegenerateKeyRequest::fixture();
        let result = ops
            .regenerate_key(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_patch_storage_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts/test-accountName")
            .returning_json(serde_json::to_value(StorageAccount::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let body = StorageAccountUpdateRequest::fixture();
        let result = ops
            .patch_storage_account(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_management_policy() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Storage/storageAccounts/test-accountName/managementPolicies/default")
            .returning_json(serde_json::to_value(ManagementPolicy::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = StorageOps::new(&client);

        let result = ops
            .get_management_policy(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
            )
            .await;
        assert!(result.is_ok());
    }
}
