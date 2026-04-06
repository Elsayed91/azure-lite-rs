//! Types for the Azure Kubernetes Service API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Profile for the container service agent pool.
///
/// **Azure API**: `aks.v1.ManagedClusterAgentPoolProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//ManagedClusterAgentPoolProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedClusterAgentPoolProfile {
    /// Name of the agent pool
    pub name: String,

    /// Number of agents (VMs) in the node pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// VM size of the agent nodes (e.g. Standard_D2s_v3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,

    /// Operating system type (Linux, Windows)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,

    /// Agent pool mode (System, User)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Agent pool type (VirtualMachineScaleSets, AvailabilitySet)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Maximum number of pods that can run on a node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,

    /// List of availability zones for the agent pool
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<String>,

    /// Node labels to be applied to the nodes
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub node_labels: HashMap<String, String>,
}

impl ManagedClusterAgentPoolProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-managed_cluster_agent_pool_profile".into(),
            count: Some(100),
            vm_size: Some("test-vm_size".into()),
            os_type: Some("test-os_type".into()),
            mode: Some("test-mode".into()),
            r#type: Some("test-type".into()),
            max_pods: Some(100),
            availability_zones: vec![],
            node_labels: Default::default(),
        }
    }
}

/// Properties of a managed cluster.
///
/// **Azure API**: `aks.v1.ManagedClusterProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//ManagedClusterProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedClusterProperties {
    /// Kubernetes version for the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,

    /// The current Kubernetes version running on the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_kubernetes_version: Option<String>,

    /// Provisioning state (Succeeded, Failed, Creating, Deleting, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// DNS prefix used in creating the FQDN for the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_prefix: Option<String>,

    /// FQDN for the master pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,

    /// Name of the resource group containing agent pool nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_resource_group: Option<String>,

    /// Whether to enable Kubernetes Role-Based Access Control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_rbac: Option<bool>,

    /// The max number of agent pools for the managed cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_agent_pools: Option<i32>,

    /// Properties of the agent pool
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_profiles: Vec<ManagedClusterAgentPoolProfile>,
}

impl ManagedClusterProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kubernetes_version: Some("test-kubernetes_version".into()),
            current_kubernetes_version: Some("test-current_kubernetes_version".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            dns_prefix: Some("test-dns_prefix".into()),
            fqdn: Some("test-fqdn".into()),
            node_resource_group: Some("test-node_resource_group".into()),
            enable_rbac: Some(false),
            max_agent_pools: Some(100),
            agent_pool_profiles: vec![],
        }
    }
}

/// Identity for the managed cluster.
///
/// **Azure API**: `aks.v1.ManagedClusterIdentity`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//ManagedClusterIdentity>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedClusterIdentity {
    /// Identity type (SystemAssigned, UserAssigned, None)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Principal ID of the system-assigned identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    /// Tenant ID of the system-assigned identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

impl ManagedClusterIdentity {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            r#type: Some("test-type".into()),
            principal_id: Some("test-principal_id".into()),
            tenant_id: Some("test-tenant_id".into()),
        }
    }
}

/// Managed cluster resource.
///
/// **Azure API**: `aks.v1.ManagedCluster`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//ManagedCluster>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedCluster {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The identity of the managed cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedClusterIdentity>,

    /// Managed cluster properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterProperties>,
}

impl ManagedCluster {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-managed_cluster".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            identity: Some(ManagedClusterIdentity::fixture()),
            properties: Some(ManagedClusterProperties::fixture()),
        }
    }
}

/// The response from the List Managed Clusters operation.
///
/// **Azure API**: `aks.v1.ManagedClusterListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//ManagedClusterListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedClusterListResult {
    /// The list of managed clusters
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedCluster>,

    /// The URL to get the next set of managed cluster results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl ManagedClusterListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties for creating or updating a managed cluster.
///
/// **Azure API**: `aks.v1.ManagedClusterCreateOrUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//ManagedClusterCreateOrUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedClusterCreateOrUpdateProperties {
    /// Kubernetes version for the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,

    /// DNS prefix for the cluster FQDN
    pub dns_prefix: String,

    /// Whether to enable Kubernetes RBAC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_rbac: Option<bool>,

    /// Agent pool configurations for the cluster
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub agent_pool_profiles: Vec<ManagedClusterAgentPoolProfile>,
}

impl ManagedClusterCreateOrUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kubernetes_version: Some("test-kubernetes_version".into()),
            dns_prefix: "test-dns_prefix".into(),
            enable_rbac: Some(false),
            agent_pool_profiles: vec![],
        }
    }
}

/// Request body for creating or updating a managed cluster.
///
/// **Azure API**: `aks.v1.ManagedClusterCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//ManagedClusterCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedClusterCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The identity of the managed cluster (use type=SystemAssigned)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedClusterIdentity>,

    /// Managed cluster create/update properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedClusterCreateOrUpdateProperties>,
}

impl ManagedClusterCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            identity: Some(ManagedClusterIdentity::fixture()),
            properties: Some(ManagedClusterCreateOrUpdateProperties::fixture()),
        }
    }
}

/// Properties of an agent pool.
///
/// **Azure API**: `aks.v1.AgentPoolProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//AgentPoolProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPoolProperties {
    /// Provisioning state (Succeeded, Failed, Creating, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Number of nodes in the pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /// VM size for the agent nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,

    /// Operating system type (Linux, Windows)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,

    /// Agent pool mode (System, User)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Agent pool type (VirtualMachineScaleSets, AvailabilitySet)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Maximum pods that can run on a node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,

    /// Availability zones for the node pool
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<String>,

    /// Node labels
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub node_labels: HashMap<String, String>,
}

impl AgentPoolProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provisioning_state: Some("test-provisioning_state".into()),
            count: Some(100),
            vm_size: Some("test-vm_size".into()),
            os_type: Some("test-os_type".into()),
            mode: Some("test-mode".into()),
            r#type: Some("test-type".into()),
            max_pods: Some(100),
            availability_zones: vec![],
            node_labels: Default::default(),
        }
    }
}

/// Agent pool resource.
///
/// **Azure API**: `aks.v1.AgentPool`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//AgentPool>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPool {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Agent pool properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgentPoolProperties>,
}

impl AgentPool {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-agent_pool".into()),
            r#type: Some("test-type".into()),
            properties: Some(AgentPoolProperties::fixture()),
        }
    }
}

/// The response from the List Agent Pools operation.
///
/// **Azure API**: `aks.v1.AgentPoolListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//AgentPoolListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPoolListResult {
    /// The list of agent pools
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AgentPool>,

    /// The URL to get the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl AgentPoolListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties for creating or updating an agent pool.
///
/// **Azure API**: `aks.v1.AgentPoolCreateOrUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//AgentPoolCreateOrUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPoolCreateOrUpdateProperties {
    /// Number of nodes in the pool
    pub count: i32,

    /// VM size for agent nodes
    pub vm_size: String,

    /// OS type (Linux, Windows)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,

    /// Agent pool mode (System, User)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Max pods per node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pods: Option<i32>,

    /// Node labels
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub node_labels: HashMap<String, String>,
}

impl AgentPoolCreateOrUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            count: 100,
            vm_size: "test-vm_size".into(),
            os_type: Some("test-os_type".into()),
            mode: Some("test-mode".into()),
            max_pods: Some(100),
            node_labels: Default::default(),
        }
    }
}

/// Request body for creating or updating an agent pool.
///
/// **Azure API**: `aks.v1.AgentPoolCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//AgentPoolCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPoolCreateRequest {
    /// Agent pool create/update properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgentPoolCreateOrUpdateProperties>,
}

impl AgentPoolCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            properties: Some(AgentPoolCreateOrUpdateProperties::fixture()),
        }
    }
}

/// A single credential entry.
///
/// **Azure API**: `aks.v1.CredentialResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//CredentialResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialResult {
    /// Name of the credential
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Base64-encoded kubeconfig string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CredentialResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-credential_result".into()),
            value: Some("test-value".into()),
        }
    }
}

/// The list of credential result response.
///
/// **Azure API**: `aks.v1.CredentialResults`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//CredentialResults>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialResults {
    /// List of credential results
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub kubeconfigs: Vec<CredentialResult>,
}

impl CredentialResults {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kubeconfigs: vec![],
        }
    }
}

/// Request body for running a command against a managed cluster.
///
/// **Azure API**: `aks.v1.RunCommandRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//RunCommandRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunCommandRequest {
    /// The command to run (e.g. 'kubectl get pods')
    pub command: String,

    /// Base64-encoded zip file with additional files for the command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// AuthToken issued by the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_token: Option<String>,
}

impl RunCommandRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            command: "test-command".into(),
            context: Some("test-context".into()),
            cluster_token: Some("test-cluster_token".into()),
        }
    }
}

/// Properties of a run command result.
///
/// **Azure API**: `aks.v1.RunCommandResultProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//RunCommandResultProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunCommandResultProperties {
    /// Provisioning state of the command execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Exit code of the command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,

    /// Time the command started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,

    /// Time the command finished
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,

    /// Stdout/stderr output of the command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,

    /// Reason for the provisioning state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RunCommandResultProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provisioning_state: Some("test-provisioning_state".into()),
            exit_code: Some(100),
            started_at: Some("test-started_at".into()),
            finished_at: Some("test-finished_at".into()),
            logs: Some("test-logs".into()),
            reason: Some("test-reason".into()),
        }
    }
}

/// Run command result.
///
/// **Azure API**: `aks.v1.RunCommandResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/aks//RunCommandResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunCommandResult {
    /// Fully qualified resource ID of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Run command result properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunCommandResultProperties>,
}

impl RunCommandResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            properties: Some(RunCommandResultProperties::fixture()),
        }
    }
}
