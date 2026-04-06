//! Operation contracts for the Azure RBAC API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/rbac.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::rbac::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure RBAC API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::rbac::RbacClient`] instead.
pub struct RbacOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> RbacOps<'a> {
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

    /// Lists all role definitions applicable at the subscription scope.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Authorization/roleDefinitions`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`RoleDefinitionListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_role_definitions(
        &self,
        subscription_id: &str,
    ) -> Result<RoleDefinitionListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/roleDefinitions",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2022-04-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_role_definitions response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_role_definitions response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a role definition by ID.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Authorization/roleDefinitions/{roleDefinitionId}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `roleDefinitionId` —  *(required)*
    ///
    /// # Response
    /// [`RoleDefinition`]
    #[allow(dead_code)]
    pub(crate) async fn get_role_definition(
        &self,
        subscription_id: &str,
        role_definition_id: &str,
    ) -> Result<RoleDefinition> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/roleDefinitions/{}",
            self.base_url(),
            encode(subscription_id),
            encode(role_definition_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2022-04-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_role_definition response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_role_definition response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists all role assignments for the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Authorization/roleAssignments`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`RoleAssignmentListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_role_assignments(
        &self,
        subscription_id: &str,
    ) -> Result<RoleAssignmentListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/roleAssignments",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2022-04-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_role_assignments response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_role_assignments response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a role assignment by ID.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentId}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `roleAssignmentId` —  *(required)*
    ///
    /// # Response
    /// [`RoleAssignment`]
    #[allow(dead_code)]
    pub(crate) async fn get_role_assignment(
        &self,
        subscription_id: &str,
        role_assignment_id: &str,
    ) -> Result<RoleAssignment> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/roleAssignments/{}",
            self.base_url(),
            encode(subscription_id),
            encode(role_assignment_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2022-04-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_role_assignment response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_role_assignment response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates a role assignment at the subscription scope.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `roleAssignmentName` —  *(required)*
    ///
    /// # Request Body
    /// [`RoleAssignmentCreateRequest`]
    ///
    /// # Response
    /// [`RoleAssignment`]
    #[allow(dead_code)]
    pub(crate) async fn create_role_assignment(
        &self,
        subscription_id: &str,
        role_assignment_name: &str,
        body: &RoleAssignmentCreateRequest,
    ) -> Result<RoleAssignment> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/roleAssignments/{}",
            self.base_url(),
            encode(subscription_id),
            encode(role_assignment_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2022-04-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_role_assignment request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_role_assignment response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_role_assignment response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a role assignment.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentId}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `roleAssignmentId` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_role_assignment(
        &self,
        subscription_id: &str,
        role_assignment_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Authorization/roleAssignments/{}",
            self.base_url(),
            encode(subscription_id),
            encode(role_assignment_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2022-04-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_role_definitions() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Authorization/roleDefinitions",
        )
        .returning_json(serde_json::to_value(RoleDefinitionListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RbacOps::new(&client);

        let result = ops.list_role_definitions("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_role_definition() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Authorization/roleDefinitions/test-roleDefinitionId")
            .returning_json(serde_json::to_value(RoleDefinition::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RbacOps::new(&client);

        let result = ops
            .get_role_definition("test-subscriptionId", "test-roleDefinitionId")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_role_assignments() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Authorization/roleAssignments",
        )
        .returning_json(serde_json::to_value(RoleAssignmentListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RbacOps::new(&client);

        let result = ops.list_role_assignments("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_role_assignment() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Authorization/roleAssignments/test-roleAssignmentId")
            .returning_json(serde_json::to_value(RoleAssignment::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RbacOps::new(&client);

        let result = ops
            .get_role_assignment("test-subscriptionId", "test-roleAssignmentId")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_role_assignment() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/providers/Microsoft.Authorization/roleAssignments/test-roleAssignmentName")
            .returning_json(serde_json::to_value(RoleAssignment::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RbacOps::new(&client);

        let body = RoleAssignmentCreateRequest::fixture();
        let result = ops
            .create_role_assignment("test-subscriptionId", "test-roleAssignmentName", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_role_assignment() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/providers/Microsoft.Authorization/roleAssignments/test-roleAssignmentId")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = RbacOps::new(&client);

        let result = ops
            .delete_role_assignment("test-subscriptionId", "test-roleAssignmentId")
            .await;
        assert!(result.is_ok());
    }
}
