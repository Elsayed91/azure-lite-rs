//! Types for the Azure Managed Identities API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The properties associated with the user assigned identity.
///
/// **Azure API**: `identity.v1.UserAssignedIdentityProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/managedidentity//UserAssignedIdentityProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAssignedIdentityProperties {
    /// The id of the tenant which the identity belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    /// The id of the service principal object associated with the created identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    /// The id of the app associated with the identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl UserAssignedIdentityProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tenant_id: Some("test-tenant_id".into()),
            principal_id: Some("test-principal_id".into()),
            client_id: Some("test-client_id".into()),
        }
    }
}

/// Describes an identity resource.
///
/// **Azure API**: `identity.v1.UserAssignedIdentity`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/managedidentity//UserAssignedIdentity>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAssignedIdentity {
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

    /// The geo-location where the resource lives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The properties associated with the identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserAssignedIdentityProperties>,
}

impl UserAssignedIdentity {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-user_assigned_identity".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(UserAssignedIdentityProperties::fixture()),
        }
    }
}

/// Values returned by the List operation.
///
/// **Azure API**: `identity.v1.UserAssignedIdentityListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/managedidentity//UserAssignedIdentityListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAssignedIdentityListResult {
    /// The collection of userAssignedIdentities returned by the listing operation
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserAssignedIdentity>,

    /// The url to get the next page of results, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl UserAssignedIdentityListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a user-assigned identity.
///
/// **Azure API**: `identity.v1.UserAssignedIdentityRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/managedidentity//UserAssignedIdentityRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAssignedIdentityRequest {
    /// The geo-location where the resource lives
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}

impl UserAssignedIdentityRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
        }
    }
}

/// The properties associated with the system assigned identity.
///
/// **Azure API**: `identity.v1.SystemAssignedIdentityProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/managedidentity//SystemAssignedIdentityProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemAssignedIdentityProperties {
    /// The id of the tenant which the identity belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    /// The id of the service principal object associated with the created identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    /// The id of the app associated with the identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// The ManagedServiceIdentity DataPlane URL that can be queried to obtain the identity
    /// credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_url: Option<String>,
}

impl SystemAssignedIdentityProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tenant_id: Some("test-tenant_id".into()),
            principal_id: Some("test-principal_id".into()),
            client_id: Some("test-client_id".into()),
            client_secret_url: Some("test-client_secret_url".into()),
        }
    }
}

/// Describes a system assigned identity resource.
///
/// **Azure API**: `identity.v1.SystemAssignedIdentity`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/managedidentity//SystemAssignedIdentity>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemAssignedIdentity {
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

    /// The geo-location where the resource lives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The properties associated with the identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SystemAssignedIdentityProperties>,
}

impl SystemAssignedIdentity {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-system_assigned_identity".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(SystemAssignedIdentityProperties::fixture()),
        }
    }
}
