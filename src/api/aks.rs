//! Azure Kubernetes Service (AKS) API client.
//!
//! Wraps the ARM management plane operations for AKS: managed clusters and
//! agent pools. All URL construction is in `ops::aks::AksOps`.
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::aks::AksOps,
    types::aks::{
        AgentPool, AgentPoolCreateRequest, AgentPoolListResult, CredentialResults, ManagedCluster,
        ManagedClusterCreateRequest, ManagedClusterListResult, RunCommandRequest, RunCommandResult,
    },
};

/// Client for the Azure Kubernetes Service ARM management plane.
///
/// Wraps [`AksOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct AksClient<'a> {
    ops: AksOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> AksClient<'a> {
    /// Create a new Azure Kubernetes Service API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: AksOps::new(client),
            client,
        }
    }

    // --- Cluster operations ---

    /// Lists all managed clusters in the subscription.
    pub async fn list_clusters(&self) -> Result<ManagedClusterListResult> {
        self.ops.list_clusters(self.client.subscription_id()).await
    }

    /// Gets a managed cluster.
    pub async fn get_cluster(
        &self,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<ManagedCluster> {
        self.ops
            .get_cluster(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
            )
            .await
    }

    /// Creates or updates a managed cluster.
    pub async fn create_cluster(
        &self,
        resource_group_name: &str,
        resource_name: &str,
        body: &ManagedClusterCreateRequest,
    ) -> Result<ManagedCluster> {
        self.ops
            .create_cluster(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
                body,
            )
            .await
    }

    /// Deletes a managed cluster.
    pub async fn delete_cluster(
        &self,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_cluster(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
            )
            .await
    }

    // --- Node pool (agent pool) operations ---

    /// Lists all agent pools in a managed cluster.
    pub async fn list_node_pools(
        &self,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<AgentPoolListResult> {
        self.ops
            .list_node_pools(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
            )
            .await
    }

    /// Gets a specific agent pool in a managed cluster.
    pub async fn get_node_pool(
        &self,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> Result<AgentPool> {
        self.ops
            .get_node_pool(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
                agent_pool_name,
            )
            .await
    }

    /// Creates or updates an agent pool in a managed cluster.
    pub async fn create_node_pool(
        &self,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
        body: &AgentPoolCreateRequest,
    ) -> Result<AgentPool> {
        self.ops
            .create_node_pool(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
                agent_pool_name,
                body,
            )
            .await
    }

    /// Deletes an agent pool in a managed cluster.
    pub async fn delete_node_pool(
        &self,
        resource_group_name: &str,
        resource_name: &str,
        agent_pool_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_node_pool(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
                agent_pool_name,
            )
            .await
    }

    // --- Credentials + Run Command operations ---

    /// Lists the user credentials (kubeconfig) for a managed cluster.
    pub async fn get_credentials(
        &self,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<CredentialResults> {
        self.ops
            .get_credentials(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
            )
            .await
    }

    /// Runs a kubectl command against a managed cluster.
    ///
    /// NOTE: This is an async LRO — the live API returns 202 Accepted.
    /// Poll the result with `get_command_result(rg, cluster, id)` until
    /// `provisioningState` is `"succeeded"` or `"failed"`.
    pub async fn run_command(
        &self,
        resource_group_name: &str,
        resource_name: &str,
        body: &RunCommandRequest,
    ) -> Result<RunCommandResult> {
        self.ops
            .run_command(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
                body,
            )
            .await
    }

    /// Gets the result of a previously issued run command.
    ///
    /// Poll until `properties.provisioningState` is `"succeeded"` or `"failed"`.
    pub async fn get_command_result(
        &self,
        resource_group_name: &str,
        resource_name: &str,
        command_id: &str,
    ) -> Result<RunCommandResult> {
        self.ops
            .get_command_result(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
                command_id,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const CLUSTER: &str = "test-cluster";
    const NODE_POOL: &str = "nodepool1";
    const CMD_ID: &str = "abc123";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn cluster_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}"),
            "name": CLUSTER,
            "type": "Microsoft.ContainerService/ManagedClusters",
            "location": "eastus",
            "identity": { "type": "SystemAssigned", "principalId": "abc-123", "tenantId": "tenant-123" },
            "properties": {
                "kubernetesVersion": "1.33",
                "currentKubernetesVersion": "1.33.6",
                "provisioningState": "Succeeded",
                "dnsPrefix": "test-cluster",
                "fqdn": "test-cluster-abc.hcp.eastus.azmk8s.io",
                "enableRbac": true,
                "nodeResourceGroup": "MC_test-rg_test-cluster_eastus",
                "agentPoolProfiles": [{
                    "name": NODE_POOL,
                    "count": 1,
                    "vmSize": "Standard_D2s_v3",
                    "osType": "Linux",
                    "mode": "System",
                    "type": "VirtualMachineScaleSets"
                }]
            }
        })
    }

    fn node_pool_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}/agentPools/{NODE_POOL}"),
            "name": NODE_POOL,
            "type": "Microsoft.ContainerService/managedClusters/agentPools",
            "properties": {
                "count": 1,
                "vmSize": "Standard_D2s_v3",
                "osType": "Linux",
                "mode": "System",
                "type": "VirtualMachineScaleSets",
                "provisioningState": "Succeeded"
            }
        })
    }

    fn credentials_json() -> serde_json::Value {
        serde_json::json!({
            "kubeconfigs": [{
                "name": "clusterUser",
                "value": "YXBpVmVyc2lvbjogdjEK"
            }]
        })
    }

    fn run_command_result_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}/commandResults/{CMD_ID}"),
            "properties": {
                "provisioningState": "succeeded",
                "exitCode": 0,
                "logs": "NAME          STATUS   ROLES    AGE   VERSION\nnodepool1     Ready    agent    10m   v1.33.6\n",
                "startedAt": "2026-02-19T07:00:00Z",
                "finishedAt": "2026-02-19T07:00:05Z"
            }
        })
    }

    #[tokio::test]
    async fn list_clusters_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.ContainerService/managedClusters"
        ))
        .returning_json(serde_json::json!({ "value": [cluster_json()] }));
        let client = make_client(mock);
        let result = client
            .aks()
            .list_clusters()
            .await
            .expect("list_clusters failed");
        assert_eq!(result.value.len(), 1);
        let c = &result.value[0];
        assert_eq!(c.name.as_deref(), Some(CLUSTER));
        let props = c.properties.as_ref().unwrap();
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
        assert_eq!(props.enable_rbac, Some(true));
    }

    #[tokio::test]
    async fn get_cluster_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}"),
        )
        .returning_json(cluster_json());
        let client = make_client(mock);
        let c = client
            .aks()
            .get_cluster(RG, CLUSTER)
            .await
            .expect("get_cluster failed");
        assert_eq!(c.name.as_deref(), Some(CLUSTER));
        let props = c.properties.as_ref().unwrap();
        assert_eq!(
            props.fqdn.as_deref(),
            Some("test-cluster-abc.hcp.eastus.azmk8s.io")
        );
        assert_eq!(props.current_kubernetes_version.as_deref(), Some("1.33.6"));
        assert_eq!(
            props.node_resource_group.as_deref(),
            Some("MC_test-rg_test-cluster_eastus")
        );
        let identity = c.identity.as_ref().unwrap();
        assert_eq!(identity.r#type.as_deref(), Some("SystemAssigned"));
    }

    #[tokio::test]
    async fn create_cluster_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}"),
        )
        .returning_json(cluster_json());
        let client = make_client(mock);
        use crate::types::aks::{
            ManagedClusterAgentPoolProfile, ManagedClusterCreateOrUpdateProperties,
            ManagedClusterIdentity,
        };
        let body = ManagedClusterCreateRequest {
            location: "eastus".into(),
            identity: Some(ManagedClusterIdentity {
                r#type: Some("SystemAssigned".into()),
                ..Default::default()
            }),
            properties: Some(ManagedClusterCreateOrUpdateProperties {
                dns_prefix: "test-cluster".into(),
                agent_pool_profiles: vec![ManagedClusterAgentPoolProfile {
                    name: NODE_POOL.into(),
                    count: Some(1),
                    vm_size: Some("Standard_D2s_v3".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let c = client
            .aks()
            .create_cluster(RG, CLUSTER, &body)
            .await
            .expect("create_cluster failed");
        assert_eq!(c.name.as_deref(), Some(CLUSTER));
    }

    #[tokio::test]
    async fn delete_cluster_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .aks()
            .delete_cluster(RG, CLUSTER)
            .await
            .expect("delete_cluster failed");
    }

    #[tokio::test]
    async fn list_node_pools_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}/agentPools"),
        )
        .returning_json(serde_json::json!({ "value": [node_pool_json()] }));
        let client = make_client(mock);
        let result = client
            .aks()
            .list_node_pools(RG, CLUSTER)
            .await
            .expect("list_node_pools failed");
        assert_eq!(result.value.len(), 1);
        let p = &result.value[0];
        assert_eq!(p.name.as_deref(), Some(NODE_POOL));
        let props = p.properties.as_ref().unwrap();
        assert_eq!(props.mode.as_deref(), Some("System"));
        assert_eq!(props.vm_size.as_deref(), Some("Standard_D2s_v3"));
    }

    #[tokio::test]
    async fn get_node_pool_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}/agentPools/{NODE_POOL}"),
        )
        .returning_json(node_pool_json());
        let client = make_client(mock);
        let p = client
            .aks()
            .get_node_pool(RG, CLUSTER, NODE_POOL)
            .await
            .expect("get_node_pool failed");
        assert_eq!(p.name.as_deref(), Some(NODE_POOL));
        let props = p.properties.as_ref().unwrap();
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
        assert_eq!(props.count, Some(1));
    }

    #[tokio::test]
    async fn get_credentials_returns_kubeconfigs() {
        let mut mock = MockClient::new();
        mock.expect_post(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}/listClusterUserCredential"),
        )
        .returning_json(credentials_json());
        let client = make_client(mock);
        let creds = client
            .aks()
            .get_credentials(RG, CLUSTER)
            .await
            .expect("get_credentials failed");
        assert_eq!(creds.kubeconfigs.len(), 1);
        let kc = &creds.kubeconfigs[0];
        assert_eq!(kc.name.as_deref(), Some("clusterUser"));
        assert!(kc.value.as_deref().is_some());
    }

    #[tokio::test]
    async fn get_command_result_deserializes_logs() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerService/managedClusters/{CLUSTER}/commandResults/{CMD_ID}"),
        )
        .returning_json(run_command_result_json());
        let client = make_client(mock);
        let result = client
            .aks()
            .get_command_result(RG, CLUSTER, CMD_ID)
            .await
            .expect("get_command_result failed");
        let props = result.properties.as_ref().unwrap();
        assert_eq!(props.provisioning_state.as_deref(), Some("succeeded"));
        assert_eq!(props.exit_code, Some(0));
        assert!(props.logs.as_deref().unwrap().contains("Ready"));
    }
}
