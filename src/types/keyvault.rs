//! Types for the Azure Key Vault API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SKU details for a Key Vault.
///
/// **Azure API**: `keyvault.v1.VaultSku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//VaultSku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultSku {
    /// SKU family name (e.g., 'A')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,

    /// SKU name (Standard or Premium)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl VaultSku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            family: Some("test-family".into()),
            name: Some("test-vault_sku".into()),
        }
    }
}

/// An identity that has access to the key vault.
///
/// **Azure API**: `keyvault.v1.AccessPolicyEntry`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//AccessPolicyEntry>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicyEntry {
    /// The Azure Active Directory tenant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    /// The object ID of the user, service principal, or security group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Permissions the identity has for keys, secrets, and certificates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AccessPermissions>,
}

impl AccessPolicyEntry {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tenant_id: Some("test-tenant_id".into()),
            object_id: Some("test-object_id".into()),
            permissions: Some(AccessPermissions::fixture()),
        }
    }
}

/// Permissions the identity has for keys, secrets, and certificates.
///
/// **Azure API**: `keyvault.v1.AccessPermissions`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//AccessPermissions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPermissions {
    /// Permissions to keys
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,

    /// Permissions to secrets
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,

    /// Permissions to certificates
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<String>,

    /// Permissions to storage accounts
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<String>,
}

impl AccessPermissions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            keys: vec![],
            secrets: vec![],
            certificates: vec![],
            storage: vec![],
        }
    }
}

/// A rule governing the accessibility from a specific IP address or IP range.
///
/// **Azure API**: `keyvault.v1.IpRule`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//IpRule>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpRule {
    /// IPv4 address range in CIDR notation (e.g., '124.56.78.91' or '124.56.78.0/24')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl IpRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: Some("test-value".into()),
        }
    }
}

/// A rule governing the accessibility from a specific virtual network.
///
/// **Azure API**: `keyvault.v1.VirtualNetworkRule`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//VirtualNetworkRule>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetworkRule {
    /// Resource ID of a subnet (e.g., '/subscriptions/.../subnets/subnet1')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Property to specify whether NRP will ignore the check if parent subnet has
    /// serviceEndpoints configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}

impl VirtualNetworkRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            ignore_missing_vnet_service_endpoint: Some(false),
        }
    }
}

/// A set of rules governing the network accessibility of a vault.
///
/// **Azure API**: `keyvault.v1.NetworkRuleSet`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//NetworkRuleSet>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkRuleSet {
    /// Specifies which traffic can bypass network rules (AzureServices or None)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass: Option<String>,

    /// The default action when no rule matches (Allow or Deny)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,

    /// The list of IP address rules
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,

    /// The list of virtual network rules
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}

impl NetworkRuleSet {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            bypass: Some("test-bypass".into()),
            default_action: Some("test-default_action".into()),
            ip_rules: vec![],
            virtual_network_rules: vec![],
        }
    }
}

/// Properties of the vault.
///
/// **Azure API**: `keyvault.v1.VaultProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//VaultProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultProperties {
    /// The Azure Active Directory tenant ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    /// SKU details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<VaultSku>,

    /// An array of 0 to 1024 identities that have access to the key vault
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub access_policies: Vec<AccessPolicyEntry>,

    /// The URI of the vault for performing operations on keys and secrets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,

    /// Property to specify whether Azure VMs can retrieve certificates stored as secrets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,

    /// Property to specify whether Azure Resource Manager is permitted to retrieve secrets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_for_template_deployment: Option<bool>,

    /// Property specifying whether recoverable deletion is enabled for this vault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,

    /// softDelete data retention days (7-90)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_delete_retention_in_days: Option<i32>,

    /// Property specifying whether protection against purge is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,

    /// Property that controls how data actions are authorized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,

    /// Rules governing the accessibility of the key vault from specific network locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,

    /// Property to specify whether the vault will accept traffic from public internet (Enabled
    /// or Disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,

    /// Provisioning state of the vault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}

impl VaultProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tenant_id: Some("test-tenant_id".into()),
            sku: Some(VaultSku::fixture()),
            access_policies: vec![],
            vault_uri: Some("test-vault_uri".into()),
            enabled_for_deployment: Some(false),
            enabled_for_template_deployment: Some(false),
            enable_soft_delete: Some(false),
            soft_delete_retention_in_days: Some(100),
            enable_purge_protection: Some(false),
            enable_rbac_authorization: Some(false),
            network_acls: Some(NetworkRuleSet::fixture()),
            public_network_access: Some("test-public_network_access".into()),
            provisioning_state: Some("test-provisioning_state".into()),
        }
    }
}

/// Resource information with extended details.
///
/// **Azure API**: `keyvault.v1.Vault`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//Vault>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vault {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the key vault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The supported Azure location where the key vault should be created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties of the vault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultProperties>,
}

impl Vault {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-vault".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(VaultProperties::fixture()),
        }
    }
}

/// List of vaults.
///
/// **Azure API**: `keyvault.v1.VaultListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//VaultListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultListResult {
    /// The list of key vaults
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vault>,

    /// The URL to get the next set of vaults
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl VaultListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties for creating or updating a vault.
///
/// **Azure API**: `keyvault.v1.VaultCreateOrUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//VaultCreateOrUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultCreateOrUpdateProperties {
    /// The Azure Active Directory tenant ID
    pub tenant_id: String,

    /// SKU details
    pub sku: VaultSku,

    /// An array of 0 to 1024 identities that have access to the key vault.
    /// Azure PUT API requires this field to always be present (even if empty).
    #[serde(default)]
    pub access_policies: Vec<AccessPolicyEntry>,

    /// Property to specify whether Azure VMs can retrieve certificates stored as secrets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_for_deployment: Option<bool>,

    /// Property specifying whether recoverable deletion is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_soft_delete: Option<bool>,

    /// Property specifying whether protection against purge is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_purge_protection: Option<bool>,

    /// Property that controls how data actions are authorized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_rbac_authorization: Option<bool>,

    /// Rules governing the accessibility of the key vault from specific network locations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
}

impl VaultCreateOrUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tenant_id: "test-tenant_id".into(),
            sku: VaultSku::fixture(),
            access_policies: vec![],
            enabled_for_deployment: Some(false),
            enable_soft_delete: Some(false),
            enable_purge_protection: Some(false),
            enable_rbac_authorization: Some(false),
            network_acls: Some(NetworkRuleSet::fixture()),
        }
    }
}

/// Parameters for creating or updating a vault.
///
/// **Azure API**: `keyvault.v1.VaultCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//VaultCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultCreateRequest {
    /// The supported Azure location where the key vault should be created
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties of the vault
    pub properties: VaultCreateOrUpdateProperties,
}

impl VaultCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: VaultCreateOrUpdateProperties::fixture(),
        }
    }
}

/// The secret management attributes.
///
/// **Azure API**: `keyvault.v1.SecretAttributes`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//SecretAttributes>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretAttributes {
    /// Determines whether the object is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Not before date in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,

    /// Expiry date in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,

    /// Creation time in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,

    /// Last updated time in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}

impl SecretAttributes {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enabled: Some(false),
            nbf: Some(100),
            exp: Some(100),
            created: Some(100),
            updated: Some(100),
        }
    }
}

/// Properties of the secret.
///
/// **Azure API**: `keyvault.v1.SecretProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//SecretProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretProperties {
    /// The value of the secret (only present when creating/updating)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The content type of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The secret management attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,

    /// The URI to retrieve the current version of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_uri: Option<String>,

    /// The URI to retrieve the specific version of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_uri_with_version: Option<String>,
}

impl SecretProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: Some("test-value".into()),
            content_type: Some("test-content_type".into()),
            attributes: Some(SecretAttributes::fixture()),
            secret_uri: Some("test-secret_uri".into()),
            secret_uri_with_version: Some("test-secret_uri_with_version".into()),
        }
    }
}

/// Resource information with extended details.
///
/// **Azure API**: `keyvault.v1.Secret`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//Secret>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Azure location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecretProperties>,
}

impl Secret {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-secret".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(SecretProperties::fixture()),
        }
    }
}

/// List of secrets.
///
/// **Azure API**: `keyvault.v1.SecretListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//SecretListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretListResult {
    /// The list of secrets
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Secret>,

    /// The URL to get the next set of secrets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl SecretListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties for creating or updating a secret.
///
/// **Azure API**: `keyvault.v1.SecretCreateOrUpdateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//SecretCreateOrUpdateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretCreateOrUpdateProperties {
    /// The value of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The content type of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The attributes of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,
}

impl SecretCreateOrUpdateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: Some("test-value".into()),
            content_type: Some("test-content_type".into()),
            attributes: Some(SecretAttributes::fixture()),
        }
    }
}

/// Parameters for creating or updating a secret.
///
/// **Azure API**: `keyvault.v1.SecretCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//SecretCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretCreateRequest {
    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties of the secret
    pub properties: SecretCreateOrUpdateProperties,
}

impl SecretCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tags: Default::default(),
            properties: SecretCreateOrUpdateProperties::fixture(),
        }
    }
}

/// The object attributes managed by the Azure Key Vault service.
///
/// **Azure API**: `keyvault.v1.KeyAttributes`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//KeyAttributes>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAttributes {
    /// Determines whether the object is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Not before date in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,

    /// Expiry date in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,

    /// Creation time in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,

    /// Last updated time in UTC (Unix epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,

    /// Indicates if the private key can be exported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exportable: Option<bool>,

    /// Reflects the deletion recovery level currently in effect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<String>,
}

impl KeyAttributes {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enabled: Some(false),
            nbf: Some(100),
            exp: Some(100),
            created: Some(100),
            updated: Some(100),
            exportable: Some(false),
            recovery_level: Some("test-recovery_level".into()),
        }
    }
}

/// The properties of the key.
///
/// **Azure API**: `keyvault.v1.KeyProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//KeyProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyProperties {
    /// The attributes of the key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,

    /// The type of the key (RSA, EC, oct-HSM, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,

    /// The permitted operations on the key
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub key_ops: Vec<String>,

    /// The key size in bits (for RSA)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,

    /// The elliptic curve name (for EC keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_name: Option<String>,

    /// The URI to retrieve the current version of the key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_uri: Option<String>,

    /// The URI to retrieve the specific version of the key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_uri_with_version: Option<String>,
}

impl KeyProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            attributes: Some(KeyAttributes::fixture()),
            kty: Some("test-kty".into()),
            key_ops: vec![],
            key_size: Some(100),
            curve_name: Some("test-curve_name".into()),
            key_uri: Some("test-key_uri".into()),
            key_uri_with_version: Some("test-key_uri_with_version".into()),
        }
    }
}

/// The key resource.
///
/// **Azure API**: `keyvault.v1.Key`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//Key>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Azure location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The properties of the key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<KeyProperties>,
}

impl Key {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-key".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(KeyProperties::fixture()),
        }
    }
}

/// The page of keys.
///
/// **Azure API**: `keyvault.v1.KeyListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//KeyListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyListResult {
    /// The list of keys
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Key>,

    /// The URL to get the next set of keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl KeyListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties for creating a key.
///
/// **Azure API**: `keyvault.v1.KeyCreateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//KeyCreateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyCreateProperties {
    /// The type of the key to create (RSA, EC, oct-HSM, etc.)
    pub kty: String,

    /// The key size in bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i32>,

    /// Elliptic curve name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_name: Option<String>,

    /// The permitted operations on the key
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub key_ops: Vec<String>,

    /// Key attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<KeyAttributes>,
}

impl KeyCreateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            kty: "test-kty".into(),
            key_size: Some(100),
            curve_name: Some("test-curve_name".into()),
            key_ops: vec![],
            attributes: Some(KeyAttributes::fixture()),
        }
    }
}

/// Parameters for creating a key.
///
/// **Azure API**: `keyvault.v1.KeyCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/keyvault//KeyCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyCreateRequest {
    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties to create the key
    pub properties: KeyCreateProperties,
}

impl KeyCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tags: Default::default(),
            properties: KeyCreateProperties::fixture(),
        }
    }
}
