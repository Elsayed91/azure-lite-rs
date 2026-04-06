//! Operation contracts for the Azure Managed Identities API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/identity.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::identity::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Managed Identities API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::identity::IdentityClient`] instead.
pub struct IdentityOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> IdentityOps<'a> {
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

    /// Lists all the userAssignedIdentities available under the specified subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.ManagedIdentity/userAssignedIdentities`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`UserAssignedIdentityListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_user_assigned_identities(
        &self,
        subscription_id: &str,
    ) -> Result<UserAssignedIdentityListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-01-31", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_user_assigned_identities response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_user_assigned_identities response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists all the userAssignedIdentities available under the specified ResourceGroup.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`UserAssignedIdentityListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_user_assigned_identities_in_group(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<UserAssignedIdentityListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-01-31", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!(
                        "Failed to read list_user_assigned_identities_in_group response: {e}"
                    ),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!(
                "Failed to parse list_user_assigned_identities_in_group response: {e}"
            ),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the identity.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{resourceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    ///
    /// # Response
    /// [`UserAssignedIdentity`]
    #[allow(dead_code)]
    pub(crate) async fn get_identity(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<UserAssignedIdentity> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-01-31", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_identity response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_identity response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update an identity in the specified subscription and resource group.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{resourceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    ///
    /// # Request Body
    /// [`UserAssignedIdentityRequest`]
    ///
    /// # Response
    /// [`UserAssignedIdentity`]
    #[allow(dead_code)]
    pub(crate) async fn create_identity(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        body: &UserAssignedIdentityRequest,
    ) -> Result<UserAssignedIdentity> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-01-31", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_identity request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_identity response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_identity response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes the identity.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{resourceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_identity(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-01-31", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Gets the systemAssignedIdentity available under the specified SubscriptionId,
    /// ResourceGroupName, and resource name.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{parentResourceProvider}/{parentResourceType}/{parentResourceName}/providers/Microsoft.ManagedIdentity/identities/default`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `parentResourceProvider` —  *(required)*
    /// - `parentResourceType` —  *(required)*
    /// - `parentResourceName` —  *(required)*
    ///
    /// # Response
    /// [`SystemAssignedIdentity`]
    #[allow(dead_code)]
    pub(crate) async fn get_system_assigned_identity(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        parent_resource_provider: &str,
        parent_resource_type: &str,
        parent_resource_name: &str,
    ) -> Result<SystemAssignedIdentity> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/{}/{}/{}/providers/Microsoft.ManagedIdentity/identities/default",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(parent_resource_provider),
            encode(parent_resource_type),
            encode(parent_resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-01-31", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_system_assigned_identity response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_system_assigned_identity response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_user_assigned_identities() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.ManagedIdentity/userAssignedIdentities")
            .returning_json(serde_json::to_value(UserAssignedIdentityListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = IdentityOps::new(&client);

        let result = ops
            .list_user_assigned_identities("test-subscriptionId")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_user_assigned_identities_in_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ManagedIdentity/userAssignedIdentities")
            .returning_json(serde_json::to_value(UserAssignedIdentityListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = IdentityOps::new(&client);

        let result = ops
            .list_user_assigned_identities_in_group("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_identity() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ManagedIdentity/userAssignedIdentities/test-resourceName")
            .returning_json(serde_json::to_value(UserAssignedIdentity::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = IdentityOps::new(&client);

        let result = ops
            .get_identity(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_identity() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ManagedIdentity/userAssignedIdentities/test-resourceName")
            .returning_json(serde_json::to_value(UserAssignedIdentity::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = IdentityOps::new(&client);

        let body = UserAssignedIdentityRequest::fixture();
        let result = ops
            .create_identity(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_identity() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ManagedIdentity/userAssignedIdentities/test-resourceName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = IdentityOps::new(&client);

        let result = ops
            .delete_identity(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_system_assigned_identity() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/test-parentResourceProvider/test-parentResourceType/test-parentResourceName/providers/Microsoft.ManagedIdentity/identities/default")
            .returning_json(serde_json::to_value(SystemAssignedIdentity::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = IdentityOps::new(&client);

        let result = ops
            .get_system_assigned_identity(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-parentResourceProvider",
                "test-parentResourceType",
                "test-parentResourceName",
            )
            .await;
        assert!(result.is_ok());
    }
}
