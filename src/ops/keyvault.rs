//! Operation contracts for the Azure Key Vault API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/keyvault.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::keyvault::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Key Vault API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::keyvault::KeyvaultClient`] instead.
pub struct KeyvaultOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> KeyvaultOps<'a> {
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

    /// The List operation gets information about the vaults associated with the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.KeyVault/vaults`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`VaultListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_vaults(&self, subscription_id: &str) -> Result<VaultListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.KeyVault/vaults",
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
                    message: format!("Failed to read list_vaults response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_vaults response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// The List operation gets information about the vaults associated with the subscription
    /// and within the specified resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`VaultListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_vaults_in_group(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<VaultListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
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
                    message: format!("Failed to read list_vaults_in_group response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_vaults_in_group response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the specified Azure key vault.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    ///
    /// # Response
    /// [`Vault`]
    #[allow(dead_code)]
    pub(crate) async fn get_vault(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> Result<Vault> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
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
                    message: format!("Failed to read get_vault response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_vault response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update a key vault in the specified subscription.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    ///
    /// # Request Body
    /// [`VaultCreateRequest`]
    ///
    /// # Response
    /// [`Vault`]
    #[allow(dead_code)]
    pub(crate) async fn create_vault(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        body: &VaultCreateRequest,
    ) -> Result<Vault> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_vault request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_vault response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_vault response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes the specified Azure key vault.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_vault(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// The List operation gets information about the secrets in a vault. NOTE: This operation
    /// does not return secret values.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}/secrets`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    ///
    /// # Response
    /// [`SecretListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_secrets(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> Result<SecretListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}/secrets",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
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
                    message: format!("Failed to read list_secrets response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_secrets response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the specified secret. NOTE: This operation does not return the secret value.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}/secrets/{secretName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    /// - `secretName` —  *(required)*
    ///
    /// # Response
    /// [`Secret`]
    #[allow(dead_code)]
    pub(crate) async fn get_secret(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
    ) -> Result<Secret> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}/secrets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
            encode(secret_name),
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
                    message: format!("Failed to read get_secret response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update a secret in a key vault in the specified subscription.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}/secrets/{secretName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    /// - `secretName` —  *(required)*
    ///
    /// # Request Body
    /// [`SecretCreateRequest`]
    ///
    /// # Response
    /// [`Secret`]
    #[allow(dead_code)]
    pub(crate) async fn set_secret(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
        body: &SecretCreateRequest,
    ) -> Result<Secret> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}/secrets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
            encode(secret_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize set_secret request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read set_secret response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse set_secret response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the keys in the specified key vault.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}/keys`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    ///
    /// # Response
    /// [`KeyListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_keys(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> Result<KeyListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}/keys",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
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
                    message: format!("Failed to read list_keys response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_keys response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the current version of the specified key from the specified key vault.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}/keys/{keyName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    /// - `keyName` —  *(required)*
    ///
    /// # Response
    /// [`Key`]
    #[allow(dead_code)]
    pub(crate) async fn get_key(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
    ) -> Result<Key> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}/keys/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
            encode(key_name),
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
                    message: format!("Failed to read get_key response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates the first version of a new key if it does not exist. If it already exists, then
    /// the existing key is returned without any write operations being performed.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}/keys/{keyName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vaultName` —  *(required)*
    /// - `keyName` —  *(required)*
    ///
    /// # Request Body
    /// [`KeyCreateRequest`]
    ///
    /// # Response
    /// [`Key`]
    #[allow(dead_code)]
    pub(crate) async fn create_key(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
        body: &KeyCreateRequest,
    ) -> Result<Key> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.KeyVault/vaults/{}/keys/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vault_name),
            encode(key_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_key request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_key response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_key response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_vaults() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.KeyVault/vaults")
            .returning_json(serde_json::to_value(VaultListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops.list_vaults("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_vaults_in_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults")
            .returning_json(serde_json::to_value(VaultListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops
            .list_vaults_in_group("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_vault() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName")
            .returning_json(serde_json::to_value(Vault::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops
            .get_vault(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_vault() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName")
            .returning_json(serde_json::to_value(Vault::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let body = VaultCreateRequest::fixture();
        let result = ops
            .create_vault(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_vault() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops
            .delete_vault(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName/secrets")
            .returning_json(serde_json::to_value(SecretListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops
            .list_secrets(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName/secrets/test-secretName")
            .returning_json(serde_json::to_value(Secret::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops
            .get_secret(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
                "test-secretName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_secret() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName/secrets/test-secretName")
            .returning_json(serde_json::to_value(Secret::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let body = SecretCreateRequest::fixture();
        let result = ops
            .set_secret(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
                "test-secretName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_keys() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName/keys")
            .returning_json(serde_json::to_value(KeyListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops
            .list_keys(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName/keys/test-keyName")
            .returning_json(serde_json::to_value(Key::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let result = ops
            .get_key(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
                "test-keyName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_key() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.KeyVault/vaults/test-vaultName/keys/test-keyName")
            .returning_json(serde_json::to_value(Key::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = KeyvaultOps::new(&client);

        let body = KeyCreateRequest::fixture();
        let result = ops
            .create_key(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vaultName",
                "test-keyName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }
}
