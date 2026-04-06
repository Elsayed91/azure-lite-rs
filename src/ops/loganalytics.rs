//! Operation contracts for the Azure Log Analytics API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/loganalytics.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::loganalytics::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Log Analytics API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::loganalytics::LoganalyticsClient`] instead.
pub struct LoganalyticsOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> LoganalyticsOps<'a> {
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

    /// Gets the workspaces in a subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.OperationalInsights/workspaces`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`WorkspaceListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_workspaces(
        &self,
        subscription_id: &str,
    ) -> Result<WorkspaceListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.OperationalInsights/workspaces",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-09-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_workspaces response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_workspaces response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a workspace instance.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.OperationalInsights/workspaces/{workspaceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `workspaceName` —  *(required)*
    ///
    /// # Response
    /// [`Workspace`]
    #[allow(dead_code)]
    pub(crate) async fn get_workspace(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> Result<Workspace> {
        let url = format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(workspace_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-09-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_workspace response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_workspace response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update a workspace.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.OperationalInsights/workspaces/{workspaceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `workspaceName` —  *(required)*
    ///
    /// # Request Body
    /// [`WorkspaceCreateRequest`]
    ///
    /// # Response
    /// [`Workspace`]
    #[allow(dead_code)]
    pub(crate) async fn create_workspace(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
        body: &WorkspaceCreateRequest,
    ) -> Result<Workspace> {
        let url = format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(workspace_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-09-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_workspace request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_workspace response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_workspace response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a workspace resource instance.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourcegroups/{resourceGroupName}/providers/Microsoft.OperationalInsights/workspaces/{workspaceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `workspaceName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_workspace(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(workspace_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-09-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Execute a KQL query against a Log Analytics workspace.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.OperationalInsights/workspaces/{workspaceName}/query`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `workspaceName` —  *(required)*
    ///
    /// # Request Body
    /// [`LogQueryBody`]
    ///
    /// # Response
    /// [`LogQueryResult`]
    #[allow(dead_code)]
    pub(crate) async fn query_logs(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
        body: &LogQueryBody,
    ) -> Result<LogQueryResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/query",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(workspace_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-09-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize query_logs request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read query_logs response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse query_logs response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the saved searches for a given Log Analytics workspace.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.OperationalInsights/workspaces/{workspaceName}/savedSearches`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `workspaceName` —  *(required)*
    ///
    /// # Response
    /// [`SavedSearchListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_saved_searches(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> Result<SavedSearchListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.OperationalInsights/workspaces/{}/savedSearches",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(workspace_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-09-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_saved_searches response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_saved_searches response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_workspaces() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.OperationalInsights/workspaces",
        )
        .returning_json(serde_json::to_value(WorkspaceListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = LoganalyticsOps::new(&client);

        let result = ops.list_workspaces("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_workspace() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourcegroups/test-resourceGroupName/providers/Microsoft.OperationalInsights/workspaces/test-workspaceName")
            .returning_json(serde_json::to_value(Workspace::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = LoganalyticsOps::new(&client);

        let result = ops
            .get_workspace(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-workspaceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_workspace() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourcegroups/test-resourceGroupName/providers/Microsoft.OperationalInsights/workspaces/test-workspaceName")
            .returning_json(serde_json::to_value(Workspace::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = LoganalyticsOps::new(&client);

        let body = WorkspaceCreateRequest::fixture();
        let result = ops
            .create_workspace(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-workspaceName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_workspace() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourcegroups/test-resourceGroupName/providers/Microsoft.OperationalInsights/workspaces/test-workspaceName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = LoganalyticsOps::new(&client);

        let result = ops
            .delete_workspace(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-workspaceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_logs() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.OperationalInsights/workspaces/test-workspaceName/query")
            .returning_json(serde_json::to_value(LogQueryResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = LoganalyticsOps::new(&client);

        let body = LogQueryBody::fixture();
        let result = ops
            .query_logs(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-workspaceName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_saved_searches() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.OperationalInsights/workspaces/test-workspaceName/savedSearches")
            .returning_json(serde_json::to_value(SavedSearchListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = LoganalyticsOps::new(&client);

        let result = ops
            .list_saved_searches(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-workspaceName",
            )
            .await;
        assert!(result.is_ok());
    }
}
