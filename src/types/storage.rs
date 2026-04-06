//! Types for the Azure Storage API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// The SKU of a storage account.
///
/// **Azure API**: `storage.v1.StorageAccountSku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountSku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountSku {
    /// SKU name (e.g. Standard_LRS, Standard_GRS, Premium_LRS)
    pub name: String,

    /// SKU tier (Standard or Premium)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl StorageAccountSku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-storage_account_sku".into(),
            tier: Some("test-tier".into()),
        }
    }
}

/// Properties of a storage account.
///
/// **Azure API**: `storage.v1.StorageAccountProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountProperties {
    /// Provisioning state of the storage account (e.g. Succeeded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The primary region for the storage account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_location: Option<String>,

    /// The secondary region for geo-redundant storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_location: Option<String>,

    /// Status of the primary endpoint (available or unavailable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_of_primary: Option<String>,

    /// Status of the secondary endpoint (available or unavailable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_of_secondary: Option<String>,

    /// Timestamp when the storage account was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,

    /// Primary service endpoints for the storage account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_endpoints: Option<serde_json::Value>,

    /// Whether only HTTPS traffic is permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,

    /// Whether hierarchical namespace is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hns_enabled: Option<bool>,

    /// Minimum TLS version (TLS1_0, TLS1_1, TLS1_2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<String>,

    /// Whether blob public access is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_blob_public_access: Option<bool>,
}

impl StorageAccountProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provisioning_state: Some("test-provisioning_state".into()),
            primary_location: Some("test-primary_location".into()),
            secondary_location: Some("test-secondary_location".into()),
            status_of_primary: Some("test-status_of_primary".into()),
            status_of_secondary: Some("test-status_of_secondary".into()),
            creation_time: Some("test-creation_time".into()),
            supports_https_traffic_only: Some(false),
            is_hns_enabled: Some(false),
            minimum_tls_version: Some("test-minimum_tls_version".into()),
            allow_blob_public_access: Some(false),
            ..Default::default()
        }
    }
}

/// An Azure storage account.
///
/// **Azure API**: `storage.v1.StorageAccount`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccount>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccount {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type (Microsoft.Storage/storageAccounts)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Azure region where the storage account is located
    pub location: String,

    /// Kind of storage account (Storage, StorageV2, BlobStorage, FileStorage, BlockBlobStorage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// SKU of the storage account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<serde_json::Value>,

    /// Resource tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    /// Storage account properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl StorageAccount {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-storage_account".into()),
            r#type: Some("test-type".into()),
            location: "test-location".into(),
            kind: Some("test-kind".into()),
            ..Default::default()
        }
    }
}

/// Response from list storage accounts operations.
///
/// **Azure API**: `storage.v1.StorageAccountListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountListResult {
    /// List of storage accounts
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,

    /// URL to retrieve next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl StorageAccountListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating a storage account.
///
/// **Azure API**: `storage.v1.StorageAccountCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountCreateRequest {
    /// Azure region where the storage account should be created
    pub location: String,

    /// Kind of storage account (StorageV2, BlobStorage, etc.)
    pub kind: String,

    /// SKU for the storage account
    pub sku: serde_json::Value,

    /// Resource tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,

    /// Additional storage account properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl StorageAccountCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            kind: "test-kind".into(),
            ..Default::default()
        }
    }
}

/// An access key for a storage account.
///
/// **Azure API**: `storage.v1.StorageAccountKey`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountKey>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountKey {
    /// Name of the key (key1, key2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,

    /// Base64-encoded key value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Key permissions (Full or Read)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,

    /// Timestamp when the key was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
}

impl StorageAccountKey {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_name: Some("test-key_name".into()),
            value: Some("test-value".into()),
            permissions: Some("test-permissions".into()),
            creation_time: Some("test-creation_time".into()),
        }
    }
}

/// Response from the ListKeys and RegenerateKey operations.
///
/// **Azure API**: `storage.v1.StorageAccountListKeysResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountListKeysResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountListKeysResult {
    /// List of storage account access keys
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<serde_json::Value>,
}

impl StorageAccountListKeysResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { keys: vec![] }
    }
}

/// Request body for regenerating a storage account key.
///
/// **Azure API**: `storage.v1.StorageAccountRegenerateKeyRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountRegenerateKeyRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountRegenerateKeyRequest {
    /// Name of the key to regenerate (key1 or key2)
    pub key_name: String,
}

impl StorageAccountRegenerateKeyRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_name: "test-key_name".into(),
        }
    }
}

/// Properties to update on a storage account (partial PATCH — only set fields are sent).
///
/// **Azure API**: `storage.v1.StorageAccountUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountUpdateProperties {
    /// Whether blob public access is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_blob_public_access: Option<bool>,

    /// Whether only HTTPS traffic is permitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_https_traffic_only: Option<bool>,

    /// Minimum TLS version (TLS1_0, TLS1_1, TLS1_2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<String>,
}

impl StorageAccountUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allow_blob_public_access: Some(false),
            supports_https_traffic_only: Some(false),
            minimum_tls_version: Some("test-minimum_tls_version".into()),
        }
    }
}

/// Request body for updating a storage account (PATCH — partial update, ARM merges with
/// existing state).
///
/// **Azure API**: `storage.v1.StorageAccountUpdateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/StorageAccountUpdateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAccountUpdateRequest {
    /// Properties to update (partial — only set fields are sent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountUpdateProperties>,

    /// Tags to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}

impl StorageAccountUpdateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            properties: Some(StorageAccountUpdateProperties::fixture()),
            ..Default::default()
        }
    }
}

/// A rule in a blob lifecycle management policy.
///
/// **Azure API**: `storage.v1.ManagementPolicyRule`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/ManagementPolicyRule>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementPolicyRule {
    /// Whether the rule is enabled
    #[serde(default)]
    pub enabled: bool,

    /// The name of the rule
    pub name: String,

    /// The type of the rule (e.g. Lifecycle)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ManagementPolicyRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enabled: false,
            name: "test-management_policy_rule".into(),
            r#type: Some("test-type".into()),
        }
    }
}

/// The Storage Account ManagementPolicy schema containing lifecycle rules.
///
/// **Azure API**: `storage.v1.ManagementPolicySchema`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/ManagementPolicySchema>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementPolicySchema {
    /// The list of lifecycle management policy rules
    #[serde(default)]
    pub rules: Vec<ManagementPolicyRule>,
}

impl ManagementPolicySchema {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { rules: vec![] }
    }
}

/// Properties of a blob lifecycle management policy.
///
/// **Azure API**: `storage.v1.ManagementPolicyProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/ManagementPolicyProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementPolicyProperties {
    /// The date and time the policy was last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,

    /// The lifecycle management policy schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ManagementPolicySchema>,
}

impl ManagementPolicyProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            last_modified_time: Some("test-last_modified_time".into()),
            policy: Some(ManagementPolicySchema::fixture()),
        }
    }
}

/// The Get Storage Account ManagementPolicies operation response.
///
/// **Azure API**: `storage.v1.ManagementPolicy`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/storagerp/storage-accounts/ManagementPolicy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementPolicy {
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

    /// The management policy properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementPolicyProperties>,
}

impl ManagementPolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-management_policy".into()),
            r#type: Some("test-type".into()),
            properties: Some(ManagementPolicyProperties::fixture()),
        }
    }
}
