//! Types for the Azure Redis Cache API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SKU parameters supplied to the create Redis operation.
///
/// **Azure API**: `redis.v1.RedisSku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisSku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisSku {
    /// The type of Redis cache to deploy (Basic, Standard, Premium)
    pub name: String,

    /// The SKU family to use (C for Basic/Standard, P for Premium)
    pub family: String,

    /// The size of the Redis cache to deploy (0-6 for C family, 1-5 for P family)
    pub capacity: i32,
}

impl RedisSku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-redis_sku".into(),
            family: "test-family".into(),
            capacity: 100,
        }
    }
}

/// All Redis Settings. Few possible keys: rdb-backup-enabled, rdb-storage-connection-string,
/// etc.
///
/// **Azure API**: `redis.v1.RedisConfiguration`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisConfiguration {
    /// How Redis will select what to remove when maxmemory is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmemory_policy: Option<String>,

    /// Value in megabytes reserved for non-cache usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmemory_reserved: Option<String>,
}

impl RedisConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            maxmemory_policy: Some("test-maxmemory_policy".into()),
            maxmemory_reserved: Some("test-maxmemory_reserved".into()),
        }
    }
}

/// Properties of the redis cache.
///
/// **Azure API**: `redis.v1.RedisProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisProperties {
    /// The SKU of the Redis cache to deploy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<RedisSku>,

    /// Redis host name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,

    /// Redis non-SSL port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Redis SSL port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_port: Option<i32>,

    /// Redis instance provisioning status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Specifies whether the non-ssl Redis server port (6379) is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_non_ssl_port: Option<bool>,

    /// Redis version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,

    /// All Redis Settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_configuration: Option<RedisConfiguration>,
}

impl RedisProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            sku: Some(RedisSku::fixture()),
            host_name: Some("test-host_name".into()),
            port: Some(100),
            ssl_port: Some(100),
            provisioning_state: Some("test-provisioning_state".into()),
            enable_non_ssl_port: Some(false),
            redis_version: Some("test-redis_version".into()),
            redis_configuration: Some(RedisConfiguration::fixture()),
        }
    }
}

/// A single Redis item in List or Get Operation.
///
/// **Azure API**: `redis.v1.RedisResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisResource {
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

    /// Redis cache properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RedisProperties>,
}

impl RedisResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-redis_resource".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            properties: Some(RedisProperties::fixture()),
        }
    }
}

/// The response of list Redis operation.
///
/// **Azure API**: `redis.v1.RedisListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisListResult {
    /// List of Redis cache instances
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RedisResource>,

    /// Link to retrieve next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl RedisListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Properties supplied to Create Redis operation.
///
/// **Azure API**: `redis.v1.RedisCreateProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisCreateProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisCreateProperties {
    /// The SKU of the Redis cache to deploy
    pub sku: RedisSku,

    /// Specifies whether the non-ssl Redis server port (6379) is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_non_ssl_port: Option<bool>,

    /// Redis version. Only major version will be used in PUT/PATCH request with current valid
    /// values: (4, 6)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,

    /// All Redis Settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_configuration: Option<RedisConfiguration>,

    /// Optional: requires clients to use a specified TLS version (or higher)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<String>,
}

impl RedisCreateProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            sku: RedisSku::fixture(),
            enable_non_ssl_port: Some(false),
            redis_version: Some("test-redis_version".into()),
            redis_configuration: Some(RedisConfiguration::fixture()),
            minimum_tls_version: Some("test-minimum_tls_version".into()),
        }
    }
}

/// Parameters supplied to the Create Redis operation.
///
/// **Azure API**: `redis.v1.RedisCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisCreateRequest {
    /// The geo-location where the resource lives
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Redis cache properties
    pub properties: RedisCreateProperties,
}

impl RedisCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: RedisCreateProperties::fixture(),
        }
    }
}

/// Redis cache access keys.
///
/// **Azure API**: `redis.v1.RedisAccessKeys`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisAccessKeys>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisAccessKeys {
    /// The current primary key that clients can use to authenticate with Redis cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,

    /// The current secondary key that clients can use to authenticate with Redis cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}

impl RedisAccessKeys {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            primary_key: Some("test-primary_key".into()),
            secondary_key: Some("test-secondary_key".into()),
        }
    }
}

/// Specifies which Redis access keys to reset.
///
/// **Azure API**: `redis.v1.RedisRegenerateKeyParameters`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisRegenerateKeyParameters>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisRegenerateKeyParameters {
    /// The Redis access key to regenerate (Primary or Secondary)
    pub key_type: String,
}

impl RedisRegenerateKeyParameters {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            key_type: "test-key_type".into(),
        }
    }
}

/// Specifies which Redis node(s) to reboot.
///
/// **Azure API**: `redis.v1.RedisRebootParameters`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisRebootParameters>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisRebootParameters {
    /// Which Redis node(s) to reboot (PrimaryNode, SecondaryNode, AllNodes)
    pub reboot_type: String,

    /// If clustering is enabled, the ID of the shard to be rebooted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<i32>,
}

impl RedisRebootParameters {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            reboot_type: "test-reboot_type".into(),
            shard_id: Some(100),
        }
    }
}

/// Response to force reboot for Redis cache.
///
/// **Azure API**: `redis.v1.RedisForceRebootResponse`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//RedisForceRebootResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisForceRebootResponse {
    /// Status message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RedisForceRebootResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            message: Some("test-message".into()),
        }
    }
}

/// Parameters for Redis export operation.
///
/// **Azure API**: `redis.v1.ExportRDBParameters`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//ExportRDBParameters>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRDBParameters {
    /// Prefix to use for exported files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// Container name to export to
    pub container: String,

    /// File format (RDB is the only supported value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl ExportRDBParameters {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            prefix: Some("test-prefix".into()),
            container: "test-container".into(),
            format: Some("test-format".into()),
        }
    }
}

/// Parameters for Redis import operation.
///
/// **Azure API**: `redis.v1.ImportRDBParameters`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/redis//ImportRDBParameters>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportRDBParameters {
    /// File format (RDB is the only supported value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// files to import
    #[serde(default)]
    pub file_uris: Vec<String>,
}

impl ImportRDBParameters {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            format: Some("test-format".into()),
            file_uris: vec![],
        }
    }
}
