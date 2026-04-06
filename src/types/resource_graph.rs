//! Types for the Azure Resource Graph API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Options for a Resource Graph query page.
///
/// **Azure API**: `resource_graph.v1.QueryOptions`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/azureresourcegraph//QueryOptions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOptions {
    /// Maximum number of results per page (default: 1000, max: 1000)
    #[serde(rename = "$top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,

    /// Result format: objectArray (default) or table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_format: Option<String>,

    /// Pagination token for fetching the next page of results
    #[serde(rename = "$skipToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
}

impl QueryOptions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            top: Some(100),
            result_format: Some("test-result_format".into()),
            skip_token: Some("test-skip_token".into()),
        }
    }
}

/// Request body for a Resource Graph query.
///
/// **Azure API**: `resource_graph.v1.ResourceGraphRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/azureresourcegraph//ResourceGraphRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGraphRequest {
    /// List of subscription IDs to query
    #[serde(default)]
    pub subscriptions: Vec<String>,

    /// KQL query to execute
    pub query: String,

    /// Query options (pagination, result format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<QueryOptions>,
}

impl ResourceGraphRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            subscriptions: vec![],
            query: "test-query".into(),
            options: Some(QueryOptions::fixture()),
        }
    }
}

/// Response from a Resource Graph query.
///
/// **Azure API**: `resource_graph.v1.ResourceGraphResponse`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/azureresourcegraph//ResourceGraphResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceGraphResponse {
    /// Total number of records matching the query (across all pages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,

    /// Number of records returned in this page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    /// Whether more pages are available (true or false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_truncated: Option<String>,

    /// Query results as JSON objects
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<serde_json::Value>,

    /// Pagination token; present when resultTruncated is true
    #[serde(rename = "$skipToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
}

impl ResourceGraphResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            total_records: Some(100),
            count: Some(100),
            result_truncated: Some("test-result_truncated".into()),
            data: vec![],
            skip_token: Some("test-skip_token".into()),
        }
    }
}
