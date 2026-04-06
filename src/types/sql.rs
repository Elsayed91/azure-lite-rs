//! Types for the Azure SQL API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The properties of a server.
///
/// **Azure API**: `sql.v1.ServerProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//ServerProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerProperties {
    /// Administrator username for the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_login: Option<String>,

    /// The fully qualified domain name of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,

    /// The state of the server (Ready, Disabled, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The version of the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ServerProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            administrator_login: Some("test-administrator_login".into()),
            fully_qualified_domain_name: Some("test-fully_qualified_domain_name".into()),
            state: Some("test-state".into()),
            version: Some("test-version".into()),
        }
    }
}

/// An Azure SQL Database server.
///
/// **Azure API**: `sql.v1.Server`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//Server>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
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

    /// Server resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
}

impl Server {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-server".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(ServerProperties::fixture()),
        }
    }
}

/// A list of servers.
///
/// **Azure API**: `sql.v1.ServerListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//ServerListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerListResult {
    /// Array of results
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Server>,

    /// Link to retrieve next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl ServerListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// The properties used to create a new server.
///
/// **Azure API**: `sql.v1.ServerCreateOrUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//ServerCreateOrUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCreateOrUpdateProperties {
    /// Administrator username for the server
    pub administrator_login: String,

    /// Administrator login password (required for create)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_login_password: Option<String>,

    /// The version of the server (e.g. 12.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ServerCreateOrUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            administrator_login: "test-administrator_login".into(),
            administrator_login_password: Some("test-administrator_login_password".into()),
            version: Some("test-version".into()),
        }
    }
}

/// An Azure SQL Database server.
///
/// **Azure API**: `sql.v1.ServerCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//ServerCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerCreateOrUpdateProperties>,
}

impl ServerCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(ServerCreateOrUpdateProperties::fixture()),
        }
    }
}

/// The database's properties.
///
/// **Azure API**: `sql.v1.DatabaseProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//DatabaseProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseProperties {
    /// The status of the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The ID of the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,

    /// The collation of the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,

    /// The max size of the database expressed in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,

    /// The name of the configured service level objective
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_service_objective_name: Option<String>,

    /// The current service level objective name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_service_objective_name: Option<String>,

    /// The default secondary region for this database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_secondary_location: Option<String>,
}

impl DatabaseProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            status: Some("test-status".into()),
            database_id: Some("test-database_id".into()),
            collation: Some("test-collation".into()),
            max_size_bytes: Some(100),
            requested_service_objective_name: Some("test-requested_service_objective_name".into()),
            current_service_objective_name: Some("test-current_service_objective_name".into()),
            default_secondary_location: Some("test-default_secondary_location".into()),
        }
    }
}

/// A database resource.
///
/// **Azure API**: `sql.v1.Database`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//Database>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
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

    /// The database SKU
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<DatabaseSku>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}

impl Database {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-database".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            sku: Some(DatabaseSku::fixture()),
            tags: Default::default(),
            properties: Some(DatabaseProperties::fixture()),
        }
    }
}

/// An ARM Resource SKU.
///
/// **Azure API**: `sql.v1.DatabaseSku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//DatabaseSku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseSku {
    /// The name of the SKU (e.g. Basic, Standard, Premium)
    pub name: String,

    /// The tier or edition of the particular SKU
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,

    /// Capacity of the particular SKU
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}

impl DatabaseSku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-database_sku".into(),
            tier: Some("test-tier".into()),
            capacity: Some(100),
        }
    }
}

/// A list of databases.
///
/// **Azure API**: `sql.v1.DatabaseListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//DatabaseListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseListResult {
    /// Array of results
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,

    /// Link to retrieve next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl DatabaseListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// The database's create or update properties.
///
/// **Azure API**: `sql.v1.DatabaseCreateOrUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//DatabaseCreateOrUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCreateOrUpdateProperties {
    /// The collation of the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,

    /// The max size of the database expressed in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_bytes: Option<i64>,
}

impl DatabaseCreateOrUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            collation: Some("test-collation".into()),
            max_size_bytes: Some(100),
        }
    }
}

/// A database resource.
///
/// **Azure API**: `sql.v1.DatabaseCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//DatabaseCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCreateRequest {
    /// Resource location
    pub location: String,

    /// The database SKU
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<DatabaseSku>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseCreateOrUpdateProperties>,
}

impl DatabaseCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            sku: Some(DatabaseSku::fixture()),
            tags: Default::default(),
            properties: Some(DatabaseCreateOrUpdateProperties::fixture()),
        }
    }
}

/// The properties of a server firewall rule.
///
/// **Azure API**: `sql.v1.FirewallRuleProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//FirewallRuleProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleProperties {
    /// The start IP address of the firewall rule
    pub start_ip_address: String,

    /// The end IP address of the firewall rule
    pub end_ip_address: String,
}

impl FirewallRuleProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            start_ip_address: "test-start_ip_address".into(),
            end_ip_address: "test-end_ip_address".into(),
        }
    }
}

/// A server firewall rule.
///
/// **Azure API**: `sql.v1.FirewallRule`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//FirewallRule>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRule {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallRuleProperties>,
}

impl FirewallRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-firewall_rule".into()),
            r#type: Some("test-type".into()),
            properties: Some(FirewallRuleProperties::fixture()),
        }
    }
}

/// The response to a list firewall rules request.
///
/// **Azure API**: `sql.v1.FirewallRuleListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//FirewallRuleListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleListResult {
    /// Array of results
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,

    /// Link to retrieve next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl FirewallRuleListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// A server firewall rule.
///
/// **Azure API**: `sql.v1.FirewallRuleCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql//FirewallRuleCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleCreateRequest {
    /// Resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallRuleProperties>,
}

impl FirewallRuleCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            properties: Some(FirewallRuleProperties::fixture()),
        }
    }
}

/// Properties of a server blob auditing policy.
///
/// **Azure API**: `sql.v1.ServerBlobAuditingPolicyProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql/server-blob-auditing-policies/get>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerBlobAuditingPolicyProperties {
    /// The state of the auditing policy (`"Enabled"` or `"Disabled"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The blob storage endpoint to send audit logs to (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,

    /// Days to retain audit logs in the storage account. 0 = unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,

    /// Whether Azure Monitor is enabled as an audit log destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,
}

impl ServerBlobAuditingPolicyProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            state: Some("Enabled".into()),
            storage_endpoint: None,
            retention_days: Some(90),
            is_azure_monitor_target_enabled: Some(true),
        }
    }
}

/// A server blob auditing policy (the `default` auditing settings for an Azure SQL Server).
///
/// **Azure API**: `sql.v1.ServerBlobAuditingPolicy`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/sql/server-blob-auditing-policies/get>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerBlobAuditingPolicy {
    /// Resource ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Auditing policy properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerBlobAuditingPolicyProperties>,
}

impl ServerBlobAuditingPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("/subscriptions/test-sub/resourceGroups/test-rg/providers/Microsoft.Sql/servers/test-server/auditingSettings/default".into()),
            name: Some("default".into()),
            properties: Some(ServerBlobAuditingPolicyProperties::fixture()),
        }
    }
}

/// Request body for enabling server-level blob auditing.
///
/// **Azure API**: `PUT .../auditingSettings/default`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableServerAuditingRequest {
    pub properties: EnableServerAuditingProperties,
}

/// Properties for enabling server auditing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableServerAuditingProperties {
    /// Must be `"Enabled"`.
    pub state: String,

    /// The blob storage endpoint (e.g. `https://myaccount.blob.core.windows.net`).
    /// Required when the server already has blob-storage auditing configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_endpoint: Option<String>,

    /// Primary access key for the storage account. Required when `storage_endpoint` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_account_access_key: Option<String>,

    /// Enable Azure Monitor as the audit log destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_azure_monitor_target_enabled: Option<bool>,

    /// Days to retain audit logs in the storage account. 0 = unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}

impl EnableServerAuditingRequest {
    /// Build a request that enables auditing to Azure Monitor.
    pub fn enable_to_azure_monitor() -> Self {
        Self {
            properties: EnableServerAuditingProperties {
                state: "Enabled".into(),
                storage_endpoint: None,
                storage_account_access_key: None,
                is_azure_monitor_target_enabled: Some(true),
                retention_days: None,
            },
        }
    }
}
