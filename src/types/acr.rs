//! Types for the Azure Container Registry API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The SKU of a container registry.
///
/// **Azure API**: `acr.v1.RegistrySku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//RegistrySku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrySku {
    /// The SKU name of the container registry (Basic, Standard, Premium)
    pub name: String,

    /// The SKU tier based on the SKU name (read-only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl RegistrySku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-registry_sku".into(),
            tier: Some("test-tier".into()),
        }
    }
}

/// The properties of a container registry.
///
/// **Azure API**: `acr.v1.RegistryProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//RegistryProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryProperties {
    /// The URL that can be used to log into the container registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_server: Option<String>,

    /// The creation date of the container registry (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,

    /// The provisioning state of the container registry (Succeeded, Failed, Creating, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Whether the admin user is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
}

impl RegistryProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            login_server: Some("test-login_server".into()),
            creation_date: Some("test-creation_date".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            admin_user_enabled: Some(false),
        }
    }
}

/// An Azure container registry.
///
/// **Azure API**: `acr.v1.Registry`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//Registry>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
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

    /// The SKU of the container registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<RegistrySku>,

    /// The properties of the container registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryProperties>,
}

impl Registry {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-registry".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            sku: Some(RegistrySku::fixture()),
            properties: Some(RegistryProperties::fixture()),
        }
    }
}

/// The result of a request to list container registries.
///
/// **Azure API**: `acr.v1.RegistryListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//RegistryListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryListResult {
    /// The list of container registries
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Registry>,

    /// The URI that can be used to request the next list of container registries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl RegistryListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties for creating a container registry.
///
/// **Azure API**: `acr.v1.RegistryCreateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//RegistryCreateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryCreateProperties {
    /// Whether the admin user is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_user_enabled: Option<bool>,
}

impl RegistryCreateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            admin_user_enabled: Some(false),
        }
    }
}

/// Request body for creating or updating a container registry.
///
/// **Azure API**: `acr.v1.RegistryCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//RegistryCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The SKU of the container registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<RegistrySku>,

    /// The properties of the container registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistryCreateProperties>,
}

impl RegistryCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            sku: Some(RegistrySku::fixture()),
            properties: Some(RegistryCreateProperties::fixture()),
        }
    }
}

/// Changeable attributes for a repository.
///
/// **Azure API**: `acr.v1.RepositoryChangeableAttributes`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//RepositoryChangeableAttributes>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryChangeableAttributes {
    /// Whether delete is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_enabled: Option<bool>,

    /// Whether write is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,

    /// Whether list is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_enabled: Option<bool>,

    /// Whether read is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_enabled: Option<bool>,
}

impl RepositoryChangeableAttributes {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            delete_enabled: Some(false),
            write_enabled: Some(false),
            list_enabled: Some(false),
            read_enabled: Some(false),
        }
    }
}

/// ACR repository information.
///
/// **Azure API**: `acr.v1.Repository`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//Repository>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    /// The name of the repository
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Modifiable attributes of the repository
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeable_attributes: Option<RepositoryChangeableAttributes>,
}

impl Repository {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-repository".into()),
            changeable_attributes: Some(RepositoryChangeableAttributes::fixture()),
        }
    }
}

/// OCI distribution catalog result (list of repository names).
///
/// **Azure API**: `acr.v1.CatalogResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//CatalogResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogResult {
    /// List of repository names
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
}

impl CatalogResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            repositories: vec![],
        }
    }
}

/// OCI distribution tag list result.
///
/// **Azure API**: `acr.v1.TagListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/containerregistry//TagListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagListResult {
    /// Repository name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// List of tag names
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl TagListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-tag_list_result".into()),
            tags: vec![],
        }
    }
}
