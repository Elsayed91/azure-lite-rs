//! Operation contracts for the Azure Kubernetes Service API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/aks.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::aks::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Kubernetes Service API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::aks::AksClient`] instead.
pub struct AksOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> AksOps<'a> {
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

    /// List all managed clusters in the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.ContainerService/managedClusters`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`ManagedClusterListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_clusters(
        &self,
        subscription_id: &str,
    ) -> Result<ManagedClusterListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.ContainerService/managedClusters",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_clusters response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_clusters response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get a managed cluster.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    ///
    /// # Response
    /// [`ManagedCluster`]
    #[allow(dead_code)]
    pub(crate) async fn get_cluster(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<ManagedCluster> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_cluster response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_cluster response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update a managed cluster.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    ///
    /// # Request Body
    /// [`ManagedClusterCreateRequest`]
    ///
    /// # Response
    /// [`ManagedCluster`]
    #[allow(dead_code)]
    pub(crate) async fn create_cluster(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        body: &ManagedClusterCreateRequest,
    ) -> Result<ManagedCluster> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_cluster request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_cluster response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_cluster response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Delete a managed cluster.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_cluster(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// List agent pools in a managed cluster.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}/agentPools`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    ///
    /// # Response
    /// [`AgentPoolListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_node_pools(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<AgentPoolListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/agentPools",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_node_pools response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_node_pools response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get the specified agent pool in a managed cluster.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}/agentPools/{agentPoolName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    /// - `agentPoolName` —  *(required)*
    ///
    /// # Response
    /// [`AgentPool`]
    #[allow(dead_code)]
    pub(crate) async fn get_node_pool(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> Result<AgentPool> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/agentPools/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
            encode(agent_pool_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_node_pool response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_node_pool response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update an agent pool in a managed cluster.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}/agentPools/{agentPoolName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    /// - `agentPoolName` —  *(required)*
    ///
    /// # Request Body
    /// [`AgentPoolCreateRequest`]
    ///
    /// # Response
    /// [`AgentPool`]
    #[allow(dead_code)]
    pub(crate) async fn create_node_pool(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
        body: &AgentPoolCreateRequest,
    ) -> Result<AgentPool> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/agentPools/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
            encode(agent_pool_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_node_pool request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_node_pool response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_node_pool response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Delete an agent pool in a managed cluster.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}/agentPools/{agentPoolName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    /// - `agentPoolName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_node_pool(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/agentPools/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
            encode(agent_pool_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// List the user credentials of a managed cluster.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}/listClusterUserCredential`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    ///
    /// # Response
    /// [`CredentialResults`]
    #[allow(dead_code)]
    pub(crate) async fn get_credentials(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<CredentialResults> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/listClusterUserCredential",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_credentials response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_credentials response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Run a kubectl command against a managed cluster (async — returns 202).
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}/runCommand`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    ///
    /// # Request Body
    /// [`RunCommandRequest`]
    ///
    /// # Response
    /// [`RunCommandResult`]
    #[allow(dead_code)]
    pub(crate) async fn run_command(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        body: &RunCommandRequest,
    ) -> Result<RunCommandResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/runCommand",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize run_command request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read run_command response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse run_command response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get the result of a previously issued run command (poll until provisioningState is
    /// succeeded/failed).
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/managedClusters/{resourceName}/commandResults/{commandId}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `resourceName` —  *(required)*
    /// - `commandId` —  *(required)*
    ///
    /// # Response
    /// [`RunCommandResult`]
    #[allow(dead_code)]
    pub(crate) async fn get_command_result(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        command_id: &str,
    ) -> Result<RunCommandResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerService/managedClusters/{}/commandResults/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(resource_name),
            encode(command_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_command_result response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_command_result response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_clusters() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.ContainerService/managedClusters")
            .returning_json(serde_json::to_value(ManagedClusterListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops.list_clusters("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName")
            .returning_json(serde_json::to_value(ManagedCluster::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops
            .get_cluster(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName")
            .returning_json(serde_json::to_value(ManagedCluster::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let body = ManagedClusterCreateRequest::fixture();
        let result = ops
            .create_cluster(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_cluster() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops
            .delete_cluster(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_node_pools() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName/agentPools")
            .returning_json(serde_json::to_value(AgentPoolListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops
            .list_node_pools(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_node_pool() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName/agentPools/test-agentPoolName")
            .returning_json(serde_json::to_value(AgentPool::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops
            .get_node_pool(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
                "test-agentPoolName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_node_pool() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName/agentPools/test-agentPoolName")
            .returning_json(serde_json::to_value(AgentPool::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let body = AgentPoolCreateRequest::fixture();
        let result = ops
            .create_node_pool(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
                "test-agentPoolName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_node_pool() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName/agentPools/test-agentPoolName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops
            .delete_node_pool(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
                "test-agentPoolName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_credentials() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName/listClusterUserCredential")
            .returning_json(serde_json::to_value(CredentialResults::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops
            .get_credentials(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_command() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName/runCommand")
            .returning_json(serde_json::to_value(RunCommandResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let body = RunCommandRequest::fixture();
        let result = ops
            .run_command(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_command_result() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.ContainerService/managedClusters/test-resourceName/commandResults/test-commandId")
            .returning_json(serde_json::to_value(RunCommandResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = AksOps::new(&client);

        let result = ops
            .get_command_result(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-resourceName",
                "test-commandId",
            )
            .await;
        assert!(result.is_ok());
    }
}
