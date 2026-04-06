//! Types for the Azure Log Analytics API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The SKU (pricing tier) of a Log Analytics workspace.
///
/// **Azure API**: `loganalytics.v1.WorkspaceSku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//WorkspaceSku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSku {
    /// The name of the SKU (e.g., PerGB2018, Free, Standard, Premium)
    pub name: String,
}

impl WorkspaceSku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-workspace_sku".into(),
        }
    }
}

/// Workspace properties.
///
/// **Azure API**: `loganalytics.v1.WorkspaceProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//WorkspaceProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceProperties {
    /// The workspace data retention in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,

    /// The SKU of the workspace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<WorkspaceSku>,

    /// The network access type for accessing Log Analytics ingestion (Enabled/Disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_ingestion: Option<String>,

    /// The network access type for accessing Log Analytics query (Enabled/Disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_network_access_for_query: Option<String>,

    /// This is a read-only property. Represents the ID associated with the workspace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    /// The provisioning state of the workspace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}

impl WorkspaceProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            retention_in_days: Some(100),
            sku: Some(WorkspaceSku::fixture()),
            public_network_access_for_ingestion: Some(
                "test-public_network_access_for_ingestion".into(),
            ),
            public_network_access_for_query: Some("test-public_network_access_for_query".into()),
            customer_id: Some("test-customer_id".into()),
            provisioning_state: Some("test-provisioning_state".into()),
        }
    }
}

/// The top level Workspace resource container.
///
/// **Azure API**: `loganalytics.v1.Workspace`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//Workspace>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    /// Azure resource identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Azure resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Azure resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Workspace properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}

impl Workspace {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-workspace".into()),
            r#type: Some("test-type".into()),
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(WorkspaceProperties::fixture()),
        }
    }
}

/// The list workspaces operation response.
///
/// **Azure API**: `loganalytics.v1.WorkspaceListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//WorkspaceListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceListResult {
    /// A list of workspaces
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
}

impl WorkspaceListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}

/// The request body for creating or updating a Log Analytics workspace.
///
/// **Azure API**: `loganalytics.v1.WorkspaceCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//WorkspaceCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Workspace properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}

impl WorkspaceCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(WorkspaceProperties::fixture()),
        }
    }
}

/// The request body for a Log Analytics query.
///
/// **Azure API**: `loganalytics.v1.LogQueryBody`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//LogQueryBody>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryBody {
    /// The KQL query to execute
    pub query: String,

    /// Optional ISO 8601 duration or time range for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timespan: Option<String>,
}

impl LogQueryBody {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            query: "test-query".into(),
            timespan: Some("test-timespan".into()),
        }
    }
}

/// A column in a Log Analytics query result table.
///
/// **Azure API**: `loganalytics.v1.LogQueryColumn`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//LogQueryColumn>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryColumn {
    /// The column name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The column data type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl LogQueryColumn {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-log_query_column".into()),
            r#type: Some("test-type".into()),
        }
    }
}

/// A table in the Log Analytics query result.
///
/// **Azure API**: `loganalytics.v1.LogQueryTable`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//LogQueryTable>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryTable {
    /// The name of the table (usually PrimaryResult)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The column schema for this table
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<LogQueryColumn>,

    /// The result rows (each row is a JSON array of values)
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<serde_json::Value>,
}

impl LogQueryTable {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-log_query_table".into()),
            columns: vec![],
            rows: vec![],
        }
    }
}

/// The response to a Log Analytics query.
///
/// **Azure API**: `loganalytics.v1.LogQueryResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//LogQueryResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryResult {
    /// The list of tables, columns and rows
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<LogQueryTable>,
}

impl LogQueryResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { tables: vec![] }
    }
}

/// Value object for saved search results.
///
/// **Azure API**: `loganalytics.v1.SavedSearchProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//SavedSearchProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearchProperties {
    /// The display name of the saved search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The category of the saved search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// The query expression for the saved search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// The version number of the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl SavedSearchProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            display_name: Some("test-display_name".into()),
            category: Some("test-category".into()),
            query: Some("test-query".into()),
            version: Some(100),
        }
    }
}

/// Value object for saved search results.
///
/// **Azure API**: `loganalytics.v1.SavedSearch`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//SavedSearch>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearch {
    /// The resource ID of the saved search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the saved search resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The ETag of the saved search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The properties of the saved search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavedSearchProperties>,
}

impl SavedSearch {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-saved_search".into()),
            etag: Some("test-etag".into()),
            properties: Some(SavedSearchProperties::fixture()),
        }
    }
}

/// The saved search list operation response.
///
/// **Azure API**: `loganalytics.v1.SavedSearchListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/loganalytics//SavedSearchListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearchListResult {
    /// The array of result values
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SavedSearch>,
}

impl SavedSearchListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}
