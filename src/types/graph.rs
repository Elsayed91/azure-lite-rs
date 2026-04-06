//! Types for the Microsoft Graph API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// An Entra ID (Azure AD) user object returned by Microsoft Graph.
///
/// **Azure API**: `graph.v1.GraphUser`
/// **Reference**: <https://learn.microsoft.com/en-us/graph/api/overview/GraphUser>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphUser {
    /// The unique identifier for the user (object ID / principal ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The display name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The user principal name (UPN). External/guest users have '#EXT#' in their UPN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,

    /// User type: 'Member' for internal users, 'Guest' for external/invited users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// Whether the user's sign-in is enabled in Entra ID.
    /// `false` means the account is blocked (sign-in disabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
}

impl GraphUser {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            display_name: Some("test-display_name".into()),
            user_principal_name: Some("test-user_principal_name".into()),
            user_type: Some("test-user_type".into()),
            account_enabled: Some(true),
        }
    }
}

/// A single request within a Microsoft Graph batch.
///
/// **Azure API**: `graph.v1.GraphBatchRequestItem`
/// **Reference**: <https://learn.microsoft.com/en-us/graph/api/overview/GraphBatchRequestItem>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphBatchRequestItem {
    /// Unique identifier for this request within the batch (used to correlate responses).
    pub id: String,

    /// HTTP method for this request (e.g. 'GET').
    pub method: String,

    /// Relative URL for this request (e.g. '/users/{id}').
    pub url: String,
}

impl GraphBatchRequestItem {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            method: "test-method".into(),
            url: "test-url".into(),
        }
    }
}

/// Request body for a Microsoft Graph batch operation ($batch).
///
/// **Azure API**: `graph.v1.GraphBatchRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/graph/api/overview/GraphBatchRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphBatchRequest {
    /// List of individual requests to execute in the batch (max 20 per batch).
    #[serde(default)]
    pub requests: Vec<GraphBatchRequestItem>,
}

impl GraphBatchRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { requests: vec![] }
    }
}

/// A single response within a Microsoft Graph batch response.
///
/// **Azure API**: `graph.v1.GraphBatchResponseItem`
/// **Reference**: <https://learn.microsoft.com/en-us/graph/api/overview/GraphBatchResponseItem>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphBatchResponseItem {
    /// Correlates to the request item ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// HTTP status code for this individual response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,

    /// The response body for this item (parsed as JSON).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

impl GraphBatchResponseItem {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            status: Some(100),
            ..Default::default()
        }
    }
}

/// Response from a Microsoft Graph batch operation ($batch).
///
/// **Azure API**: `graph.v1.GraphBatchResponse`
/// **Reference**: <https://learn.microsoft.com/en-us/graph/api/overview/GraphBatchResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphBatchResponse {
    /// List of individual responses, one per request in the batch.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub responses: Vec<GraphBatchResponseItem>,
}

impl GraphBatchResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { responses: vec![] }
    }
}
