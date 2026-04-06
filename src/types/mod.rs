//! AZURE API type definitions.

pub mod acr;
pub mod aks;
pub mod compute;
pub mod cosmosdb;
pub mod cost;
pub mod dns;
pub mod functions;
pub mod graph;
pub mod identity;
pub mod keyvault;
pub mod loganalytics;
pub mod monitor;
pub mod networking;
pub mod rbac;
pub mod redis;
pub mod resource_graph;
pub mod security;
pub mod sql;
pub mod storage;
pub mod subscriptions;

pub use acr::{
    CatalogResult, Registry, RegistryCreateProperties, RegistryCreateRequest, RegistryListResult,
    RegistryProperties, RegistrySku, Repository, RepositoryChangeableAttributes, TagListResult,
};
pub use aks::{
    AgentPool, AgentPoolCreateOrUpdateProperties, AgentPoolCreateRequest, AgentPoolListResult,
    AgentPoolProperties, CredentialResult, CredentialResults, ManagedCluster,
    ManagedClusterAgentPoolProfile, ManagedClusterCreateOrUpdateProperties,
    ManagedClusterCreateRequest, ManagedClusterIdentity, ManagedClusterListResult,
    ManagedClusterProperties, RunCommandRequest, RunCommandResult, RunCommandResultProperties,
};
pub use compute::{
    AccessUri, BootDiagnostics, DataDisk, DiagnosticsProfile, Disk, DiskCreateRequest,
    DiskCreationData, DiskListResult, DiskProperties, DiskSku, DiskUpdateRequest, Encryption,
    EncryptionSettingsCollection, EncryptionSettingsElement, GrantAccessData, HardwareProfile,
    ImageDiskReference, ImageReference, InstanceViewStatus, KeyVaultAndKeyReference,
    KeyVaultAndSecretReference, LinuxConfiguration, ManagedDiskParameters,
    NetworkInterfaceReference, NetworkInterfaceReferenceProperties, NetworkProfile, OsDisk,
    OsProfile, Sku, SshConfiguration, SshPublicKey, StorageProfile, UpgradePolicy, VirtualMachine,
    VirtualMachineAgentInstanceView, VirtualMachineCreateRequest, VirtualMachineInstanceView,
    VirtualMachineInstanceViewResult, VirtualMachineListResult, VirtualMachineProperties,
    VirtualMachineScaleSet, VirtualMachineScaleSetCreateRequest,
    VirtualMachineScaleSetIPConfiguration, VirtualMachineScaleSetIPConfigurationProperties,
    VirtualMachineScaleSetListResult, VirtualMachineScaleSetNetworkConfiguration,
    VirtualMachineScaleSetNetworkConfigurationProperties, VirtualMachineScaleSetNetworkProfile,
    VirtualMachineScaleSetOsProfile, VirtualMachineScaleSetProperties, VirtualMachineScaleSetVM,
    VirtualMachineScaleSetVMInstanceIDs, VirtualMachineScaleSetVMListResult,
    VirtualMachineScaleSetVMProfile, VirtualMachineScaleSetVMProperties, WindowsConfiguration,
};
pub use cosmosdb::{
    ConsistencyPolicy, ContainerPartitionKey, DatabaseAccount, DatabaseAccountCreateRequest,
    DatabaseAccountCreateUpdateProperties, DatabaseAccountListResult, DatabaseAccountProperties,
    Location, SqlContainerCreateUpdateProperties, SqlContainerGetProperties,
    SqlContainerGetPropertiesResource, SqlContainerGetResults, SqlContainerListResult,
    SqlContainerResource, SqlDatabaseCreateRequest, SqlDatabaseCreateUpdateProperties,
    SqlDatabaseGetProperties, SqlDatabaseGetPropertiesResource, SqlDatabaseGetResults,
    SqlDatabaseListResult, SqlDatabaseResource,
};
pub use cost::{
    Budget, BudgetCreateRequest, BudgetListResult, BudgetProperties, BudgetTimePeriod,
    CurrentSpend, ForecastDefinition, QueryColumn, QueryDataset, QueryDefinition, QueryProperties,
    QueryResult, QueryTimePeriod, UsageDetail, UsageDetailProperties, UsageDetailsListResult,
};
pub use dns::{
    ARecord, AaaaRecord, CnameRecord, MxRecord, NsRecord, RecordSet, RecordSetCreateRequest,
    RecordSetListResult, RecordSetProperties, TxtRecord, Zone, ZoneCreateRequest, ZoneListResult,
    ZoneProperties,
};
pub use functions::{
    AppSettingsResult, AppSettingsUpdateRequest, Function, FunctionApp,
    FunctionAppCreateOrUpdateProperties, FunctionAppCreateRequest, FunctionAppListResult,
    FunctionAppProperties, FunctionAppSiteConfig, FunctionListResult, FunctionProperties,
};
pub use graph::{
    GraphBatchRequest, GraphBatchRequestItem, GraphBatchResponse, GraphBatchResponseItem, GraphUser,
};
pub use identity::{
    SystemAssignedIdentity, SystemAssignedIdentityProperties, UserAssignedIdentity,
    UserAssignedIdentityListResult, UserAssignedIdentityProperties, UserAssignedIdentityRequest,
};
pub use keyvault::{
    AccessPermissions, AccessPolicyEntry, IpRule, Key, KeyAttributes, KeyCreateProperties,
    KeyCreateRequest, KeyListResult, KeyProperties, NetworkRuleSet, Secret, SecretAttributes,
    SecretCreateOrUpdateProperties, SecretCreateRequest, SecretListResult, SecretProperties, Vault,
    VaultCreateOrUpdateProperties, VaultCreateRequest, VaultListResult, VaultProperties, VaultSku,
    VirtualNetworkRule,
};
pub use loganalytics::{
    LogQueryBody, LogQueryColumn, LogQueryResult, LogQueryTable, SavedSearch,
    SavedSearchListResult, SavedSearchProperties, Workspace, WorkspaceCreateRequest,
    WorkspaceListResult, WorkspaceProperties, WorkspaceSku,
};
pub use monitor::{
    EventData, EventDataCollection, Metric, MetricAlertCreateRequest, MetricAlertProperties,
    MetricAlertResource, MetricAlertResourceCollection, MetricAlertResourcePatch,
    MetricAvailability, MetricDefinition, MetricDefinitionCollection, MetricValue, MetricsResponse,
    TimeSeriesElement,
};
pub use networking::{
    AddressSpace, BackendAddressPool, BackendAddressPoolPropertiesFormat, DhcpOptions,
    FrontendIPConfiguration, FrontendIPConfigurationPropertiesFormat, LoadBalancer,
    LoadBalancerBackendAddress, LoadBalancerBackendAddressPropertiesFormat,
    LoadBalancerCreateRequest, LoadBalancerListResult, LoadBalancerPropertiesFormat,
    LoadBalancerSku, LoadBalancingRule, LoadBalancingRulePropertiesFormat, NetworkSecurityGroup,
    NetworkSecurityGroupCreateRequest, NetworkSecurityGroupListResult,
    NetworkSecurityGroupPropertiesFormat, Probe, ProbePropertiesFormat, SecurityRule,
    SecurityRuleListResult, SecurityRulePropertiesFormat, Subnet, SubnetListResult,
    SubnetPropertiesFormat, VirtualNetwork, VirtualNetworkCreateRequest, VirtualNetworkListResult,
    VirtualNetworkPeering, VirtualNetworkPeeringPropertiesFormat, VirtualNetworkPropertiesFormat,
};
pub use rbac::{
    Permission, RoleAssignment, RoleAssignmentCreateRequest, RoleAssignmentListResult,
    RoleAssignmentProperties, RoleAssignmentRequestProperties, RoleDefinition,
    RoleDefinitionListResult, RoleDefinitionProperties,
};
pub use redis::{
    ExportRDBParameters, ImportRDBParameters, RedisAccessKeys, RedisConfiguration,
    RedisCreateProperties, RedisCreateRequest, RedisForceRebootResponse, RedisListResult,
    RedisProperties, RedisRebootParameters, RedisRegenerateKeyParameters, RedisResource, RedisSku,
};
pub use resource_graph::{QueryOptions, ResourceGraphRequest, ResourceGraphResponse};
pub use security::{
    Alert, AlertListResult, AlertProperties, Assessment, AssessmentListResult,
    AssessmentProperties, AssessmentStatus, ScoreDetails, SecureScore, SecureScoreListResult,
    SecureScoreProperties,
};
pub use sql::{
    Database, DatabaseCreateOrUpdateProperties, DatabaseCreateRequest, DatabaseListResult,
    DatabaseProperties, DatabaseSku, FirewallRule, FirewallRuleCreateRequest,
    FirewallRuleListResult, FirewallRuleProperties, Server, ServerCreateOrUpdateProperties,
    ServerCreateRequest, ServerListResult, ServerProperties,
};
pub use storage::{
    ManagementPolicy, ManagementPolicyProperties, ManagementPolicyRule, ManagementPolicySchema,
    StorageAccount, StorageAccountCreateRequest, StorageAccountKey, StorageAccountListKeysResult,
    StorageAccountListResult, StorageAccountProperties, StorageAccountRegenerateKeyRequest,
    StorageAccountSku, StorageAccountUpdateProperties, StorageAccountUpdateRequest,
};
pub use subscriptions::{SubscriptionInfo, SubscriptionListResponse};
