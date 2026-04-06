//! Types for the Azure CosmosDB API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The consistency policy for the Cosmos DB account.
///
/// **Azure API**: `cosmosdb.v1.ConsistencyPolicy`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//ConsistencyPolicy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsistencyPolicy {
    /// The default consistency level (BoundedStaleness, ConsistentPrefix, Eventual, Session,
    /// Strong)
    pub default_consistency_level: String,

    /// When used with BoundedStaleness, represents the number of stale requests tolerated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_staleness_prefix: Option<i64>,

    /// When used with BoundedStaleness, represents the time (seconds) of staleness tolerated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_interval_in_seconds: Option<i32>,
}

impl ConsistencyPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            default_consistency_level: "test-default_consistency_level".into(),
            max_staleness_prefix: Some(100),
            max_interval_in_seconds: Some(100),
        }
    }
}

/// A region in which the Azure Cosmos DB database account is deployed.
///
/// **Azure API**: `cosmosdb.v1.Location`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//Location>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// The name of the region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,

    /// The status of the Cosmos DB account at the time the operation was called
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The failover priority of the region (0 = write region)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_priority: Option<i32>,

    /// Flag to indicate whether or not this region is an AvailabilityZone region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_zone_redundant: Option<bool>,
}

impl Location {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location_name: Some("test-location_name".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            failover_priority: Some(100),
            is_zone_redundant: Some(false),
        }
    }
}

/// Properties for the database account.
///
/// **Azure API**: `cosmosdb.v1.DatabaseAccountProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//DatabaseAccountProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAccountProperties {
    /// The connection endpoint for the Cosmos DB database account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_endpoint: Option<String>,

    /// The status of the Cosmos DB account at the time the operation was called
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The offer type for the Cosmos DB database account (Standard)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_account_offer_type: Option<String>,

    /// The consistency policy for the Cosmos DB account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_policy: Option<ConsistencyPolicy>,

    /// Enables automatic failover of the write region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_automatic_failover: Option<bool>,

    /// Enables the account to write in multiple locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multiple_write_locations: Option<bool>,
}

impl DatabaseAccountProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            document_endpoint: Some("test-document_endpoint".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            database_account_offer_type: Some("test-database_account_offer_type".into()),
            consistency_policy: Some(ConsistencyPolicy::fixture()),
            enable_automatic_failover: Some(false),
            enable_multiple_write_locations: Some(false),
        }
    }
}

/// An Azure Cosmos DB database account.
///
/// **Azure API**: `cosmosdb.v1.DatabaseAccount`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//DatabaseAccount>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAccount {
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

    /// Indicates the type of database account (GlobalDocumentDB, MongoDB, Parse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Account properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseAccountProperties>,
}

impl DatabaseAccount {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-database_account".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            kind: Some("test-kind".into()),
            tags: Default::default(),
            properties: Some(DatabaseAccountProperties::fixture()),
        }
    }
}

/// The List operation response, that contains the database accounts and their properties.
///
/// **Azure API**: `cosmosdb.v1.DatabaseAccountListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//DatabaseAccountListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAccountListResult {
    /// List of database account and their properties
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseAccount>,
}

impl DatabaseAccountListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}

/// Properties to create and update Azure Cosmos DB database accounts.
///
/// **Azure API**: `cosmosdb.v1.DatabaseAccountCreateUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//DatabaseAccountCreateUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAccountCreateUpdateProperties {
    /// The offer type for the database (Standard)
    pub database_account_offer_type: String,

    /// The consistency policy for the Cosmos DB account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_policy: Option<ConsistencyPolicy>,

    /// Enables automatic failover of the write region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_automatic_failover: Option<bool>,

    /// Enables the account to write in multiple locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multiple_write_locations: Option<bool>,
}

impl DatabaseAccountCreateUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            database_account_offer_type: "test-database_account_offer_type".into(),
            consistency_policy: Some(ConsistencyPolicy::fixture()),
            enable_automatic_failover: Some(false),
            enable_multiple_write_locations: Some(false),
        }
    }
}

/// Parameters to create and update Cosmos DB database accounts.
///
/// **Azure API**: `cosmosdb.v1.DatabaseAccountCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//DatabaseAccountCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseAccountCreateRequest {
    /// The location of the resource group to which the resource belongs
    pub location: String,

    /// Indicates the type of database account (GlobalDocumentDB, MongoDB, Parse)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties for the database account
    pub properties: DatabaseAccountCreateUpdateProperties,
}

impl DatabaseAccountCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            kind: Some("test-kind".into()),
            tags: Default::default(),
            properties: DatabaseAccountCreateUpdateProperties::fixture(),
        }
    }
}

/// Cosmos DB SQL database resource object.
///
/// **Azure API**: `cosmosdb.v1.SqlDatabaseResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlDatabaseResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDatabaseResource {
    /// Name of the Cosmos DB SQL database
    pub id: String,
}

impl SqlDatabaseResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
        }
    }
}

/// The Cosmos DB SQL database resource.
///
/// **Azure API**: `cosmosdb.v1.SqlDatabaseGetPropertiesResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlDatabaseGetPropertiesResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDatabaseGetPropertiesResource {
    /// Name of the Cosmos DB SQL database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A system generated property that specified the addressable path of the collections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colls: Option<String>,

    /// A system generated property that specifies the addressable path of the users resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<String>,
}

impl SqlDatabaseGetPropertiesResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            colls: Some("test-colls".into()),
            users: Some("test-users".into()),
        }
    }
}

/// The properties of an Azure Cosmos DB SQL database.
///
/// **Azure API**: `cosmosdb.v1.SqlDatabaseGetProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlDatabaseGetProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDatabaseGetProperties {
    /// The resource of the SQL database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<SqlDatabaseGetPropertiesResource>,
}

impl SqlDatabaseGetProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource: Some(SqlDatabaseGetPropertiesResource::fixture()),
        }
    }
}

/// An Azure Cosmos DB SQL database.
///
/// **Azure API**: `cosmosdb.v1.SqlDatabaseGetResults`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlDatabaseGetResults>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDatabaseGetResults {
    /// The unique resource identifier of the ARM resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the ARM resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of Azure resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The location of the resource group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Tags are a list of key-value pairs
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The properties of an Azure Cosmos DB SQL database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDatabaseGetProperties>,
}

impl SqlDatabaseGetResults {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-sql_database_get_results".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(SqlDatabaseGetProperties::fixture()),
        }
    }
}

/// The List operation response, that contains the SQL databases and their properties.
///
/// **Azure API**: `cosmosdb.v1.SqlDatabaseListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlDatabaseListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDatabaseListResult {
    /// List of SQL databases and their properties
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlDatabaseGetResults>,
}

impl SqlDatabaseListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}

/// Properties to create and update Azure Cosmos DB SQL database.
///
/// **Azure API**: `cosmosdb.v1.SqlDatabaseCreateUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlDatabaseCreateUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDatabaseCreateUpdateProperties {
    /// The standard JSON format of a Cosmos DB SQL database
    pub resource: SqlDatabaseResource,
}

impl SqlDatabaseCreateUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource: SqlDatabaseResource::fixture(),
        }
    }
}

/// Parameters to create and update Cosmos DB SQL database.
///
/// **Azure API**: `cosmosdb.v1.SqlDatabaseCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlDatabaseCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlDatabaseCreateRequest {
    /// The location of the resource group to which the resource belongs
    pub location: String,

    /// Tags are a list of key-value pairs
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties to create and update Azure Cosmos DB SQL database
    pub properties: SqlDatabaseCreateUpdateProperties,
}

impl SqlDatabaseCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: SqlDatabaseCreateUpdateProperties::fixture(),
        }
    }
}

/// The configuration of the partition key to be used for partitioning data.
///
/// **Azure API**: `cosmosdb.v1.ContainerPartitionKey`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//ContainerPartitionKey>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPartitionKey {
    /// Indicates the kind of algorithm used for partitioning (Hash, Range, MultiHash)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Version of the partition key definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl ContainerPartitionKey {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kind: Some("test-kind".into()),
            version: Some(100),
        }
    }
}

/// Cosmos DB SQL container resource object.
///
/// **Azure API**: `cosmosdb.v1.SqlContainerResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlContainerResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlContainerResource {
    /// Name of the Cosmos DB SQL container
    pub id: String,

    /// The configuration of the partition key to be used for partitioning data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<ContainerPartitionKey>,
}

impl SqlContainerResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            partition_key: Some(ContainerPartitionKey::fixture()),
        }
    }
}

/// The resource of the SQL container.
///
/// **Azure API**: `cosmosdb.v1.SqlContainerGetPropertiesResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlContainerGetPropertiesResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlContainerGetPropertiesResource {
    /// Name of the Cosmos DB SQL container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The configuration of the partition key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<ContainerPartitionKey>,
}

impl SqlContainerGetPropertiesResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            partition_key: Some(ContainerPartitionKey::fixture()),
        }
    }
}

/// The properties of an Azure Cosmos DB container.
///
/// **Azure API**: `cosmosdb.v1.SqlContainerGetProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlContainerGetProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlContainerGetProperties {
    /// The resource of the SQL container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<SqlContainerGetPropertiesResource>,
}

impl SqlContainerGetProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource: Some(SqlContainerGetPropertiesResource::fixture()),
        }
    }
}

/// An Azure Cosmos DB container.
///
/// **Azure API**: `cosmosdb.v1.SqlContainerGetResults`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlContainerGetResults>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlContainerGetResults {
    /// The unique resource identifier of the ARM resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the ARM resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of Azure resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The location of the resource group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Tags are a list of key-value pairs
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The properties of an Azure Cosmos DB container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlContainerGetProperties>,
}

impl SqlContainerGetResults {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-sql_container_get_results".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(SqlContainerGetProperties::fixture()),
        }
    }
}

/// The List operation response, that contains the containers and their properties.
///
/// **Azure API**: `cosmosdb.v1.SqlContainerListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlContainerListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlContainerListResult {
    /// List of containers and their properties
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlContainerGetResults>,
}

impl SqlContainerListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}

/// Properties to create and update Azure Cosmos DB container.
///
/// **Azure API**: `cosmosdb.v1.SqlContainerCreateUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cosmos-db-resource-provider//SqlContainerCreateUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlContainerCreateUpdateProperties {
    /// The standard JSON format of a Cosmos DB SQL container
    pub resource: SqlContainerResource,
}

impl SqlContainerCreateUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            resource: SqlContainerResource::fixture(),
        }
    }
}
