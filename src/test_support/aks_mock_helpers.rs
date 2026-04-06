//! MockClient helpers for Azure Kubernetes Service API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Kubernetes Service helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait AksMockHelpers {
    /// Helper to expect `list_clusters`: List all managed clusters in the subscription.
    fn expect_list_clusters(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_cluster`: Get a managed cluster.
    fn expect_get_cluster(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_cluster`: Create or update a managed cluster.
    fn expect_create_cluster(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_cluster`: Delete a managed cluster.
    fn expect_delete_cluster(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_node_pools`: List agent pools in a managed cluster.
    fn expect_list_node_pools(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_node_pool`: Get the specified agent pool in a managed cluster.
    fn expect_get_node_pool(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_node_pool`: Create or update an agent pool in a managed cluster.
    fn expect_create_node_pool(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_node_pool`: Delete an agent pool in a managed cluster.
    fn expect_delete_node_pool(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_credentials`: List the user credentials of a managed cluster.
    fn expect_get_credentials(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `run_command`: Run a kubectl command against a managed cluster (async —
    /// returns 202).
    fn expect_run_command(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_command_result`: Get the result of a previously issued run command
    /// (poll until provisioningState is succeeded/failed).
    fn expect_get_command_result(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        command_id: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl AksMockHelpers for MockClient {
    /// Helper to expect `list_clusters`: List all managed clusters in the subscription.
    fn expect_list_clusters(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.ContainerService/managedClusters"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_cluster`: Get a managed cluster.
    fn expect_get_cluster(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_cluster`: Create or update a managed cluster.
    fn expect_create_cluster(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_cluster`: Delete a managed cluster.
    fn expect_delete_cluster(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_node_pools`: List agent pools in a managed cluster.
    fn expect_list_node_pools(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}/agentPools"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_node_pool`: Get the specified agent pool in a managed cluster.
    fn expect_get_node_pool(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}/agentPools/{agent_pool_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_node_pool`: Create or update an agent pool in a managed cluster.
    fn expect_create_node_pool(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}/agentPools/{agent_pool_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_node_pool`: Delete an agent pool in a managed cluster.
    fn expect_delete_node_pool(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}/agentPools/{agent_pool_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `get_credentials`: List the user credentials of a managed cluster.
    fn expect_get_credentials(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}/listClusterUserCredential"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `run_command`: Run a kubectl command against a managed cluster (async —
    /// returns 202).
    fn expect_run_command(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}/runCommand"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `get_command_result`: Get the result of a previously issued run command
    /// (poll until provisioningState is succeeded/failed).
    fn expect_get_command_result(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
        command_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerService/managedClusters/{resource_name}/commandResults/{command_id}"
        );
        self.expect_get(&path)
    }
}
