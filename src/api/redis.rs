//! Azure Redis Cache API client.
//!
//! Wraps the ARM management plane operations for Azure Redis Cache. All URL
//! construction is in `ops::redis::RedisOps`. `subscription_id` is
//! auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::redis::RedisOps,
    types::redis::{
        ExportRDBParameters, ImportRDBParameters, RedisAccessKeys, RedisCreateRequest,
        RedisForceRebootResponse, RedisListResult, RedisRebootParameters,
        RedisRegenerateKeyParameters, RedisResource,
    },
};

/// Client for the Azure Redis Cache ARM management plane.
///
/// Wraps [`RedisOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct RedisClient<'a> {
    ops: RedisOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> RedisClient<'a> {
    /// Create a new Azure Redis Cache API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: RedisOps::new(client),
            client,
        }
    }

    // --- Cache operations ---

    /// Gets all Redis caches in the specified subscription.
    pub async fn list_caches(&self) -> Result<RedisListResult> {
        self.ops.list_caches(self.client.subscription_id()).await
    }

    /// Lists all Redis caches in a resource group.
    pub async fn list_caches_by_resource_group(
        &self,
        resource_group_name: &str,
    ) -> Result<RedisListResult> {
        self.ops
            .list_caches_by_resource_group(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Gets a Redis cache (resource description).
    pub async fn get_cache(&self, resource_group_name: &str, name: &str) -> Result<RedisResource> {
        self.ops
            .get_cache(self.client.subscription_id(), resource_group_name, name)
            .await
    }

    /// Create or replace an existing Redis cache.
    pub async fn create_cache(
        &self,
        resource_group_name: &str,
        name: &str,
        body: &RedisCreateRequest,
    ) -> Result<RedisResource> {
        self.ops
            .create_cache(
                self.client.subscription_id(),
                resource_group_name,
                name,
                body,
            )
            .await
    }

    /// Deletes a Redis cache.
    pub async fn delete_cache(&self, resource_group_name: &str, name: &str) -> Result<()> {
        self.ops
            .delete_cache(self.client.subscription_id(), resource_group_name, name)
            .await
    }

    // --- Key operations ---

    /// Retrieve a Redis cache's access keys.
    pub async fn list_keys(
        &self,
        resource_group_name: &str,
        name: &str,
    ) -> Result<RedisAccessKeys> {
        self.ops
            .list_keys(self.client.subscription_id(), resource_group_name, name)
            .await
    }

    /// Regenerate Redis cache's access keys.
    pub async fn regenerate_key(
        &self,
        resource_group_name: &str,
        name: &str,
        body: &RedisRegenerateKeyParameters,
    ) -> Result<RedisAccessKeys> {
        self.ops
            .regenerate_key(
                self.client.subscription_id(),
                resource_group_name,
                name,
                body,
            )
            .await
    }

    // --- Maintenance operations ---

    /// Reboot specified Redis node(s).
    pub async fn force_reboot(
        &self,
        resource_group_name: &str,
        name: &str,
        body: &RedisRebootParameters,
    ) -> Result<RedisForceRebootResponse> {
        self.ops
            .force_reboot(
                self.client.subscription_id(),
                resource_group_name,
                name,
                body,
            )
            .await
    }

    /// Import data into Redis cache.
    pub async fn import_data(
        &self,
        resource_group_name: &str,
        name: &str,
        body: &ImportRDBParameters,
    ) -> Result<()> {
        self.ops
            .import_data(
                self.client.subscription_id(),
                resource_group_name,
                name,
                body,
            )
            .await
    }

    /// Export data from the redis cache to blobs in a container.
    pub async fn export_data(
        &self,
        resource_group_name: &str,
        name: &str,
        body: &ExportRDBParameters,
    ) -> Result<()> {
        self.ops
            .export_data(
                self.client.subscription_id(),
                resource_group_name,
                name,
                body,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MockClient,
        types::redis::{RedisCreateProperties, RedisSku},
    };

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const CACHE: &str = "cloud-lite-test-redis";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn cache_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/Redis/{CACHE}"),
            "name": CACHE,
            "type": "Microsoft.Cache/Redis",
            "location": "eastus",
            "properties": {
                "hostName": format!("{CACHE}.redis.cache.windows.net"),
                "port": 6379,
                "sslPort": 6380,
                "provisioningState": "Succeeded",
                "enableNonSslPort": false,
                "redisVersion": "6.0",
                "sku": { "name": "Basic", "family": "C", "capacity": 0 }
            }
        })
    }

    fn keys_json() -> serde_json::Value {
        serde_json::json!({
            "primaryKey": "primary-key-value",
            "secondaryKey": "secondary-key-value"
        })
    }

    #[tokio::test]
    async fn list_caches_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Cache/redis"
        ))
        .returning_json(serde_json::json!({ "value": [cache_json()] }));
        let client = make_client(mock);
        let result = client
            .redis()
            .list_caches()
            .await
            .expect("list_caches failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(CACHE));
    }

    #[tokio::test]
    async fn list_caches_by_resource_group_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/redis"
        ))
        .returning_json(serde_json::json!({ "value": [cache_json()] }));
        let client = make_client(mock);
        let result = client
            .redis()
            .list_caches_by_resource_group(RG)
            .await
            .expect("list_caches_by_resource_group failed");
        assert_eq!(result.value.len(), 1);
    }

    #[tokio::test]
    async fn get_cache_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/redis/{CACHE}"
        ))
        .returning_json(cache_json());
        let client = make_client(mock);
        let c = client
            .redis()
            .get_cache(RG, CACHE)
            .await
            .expect("get_cache failed");
        assert_eq!(c.name.as_deref(), Some(CACHE));
        let props = c.properties.as_ref().unwrap();
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
        assert_eq!(props.ssl_port, Some(6380));
        assert_eq!(props.redis_version.as_deref(), Some("6.0"));
        let sku = props.sku.as_ref().unwrap();
        assert_eq!(sku.name, "Basic");
        assert_eq!(sku.family, "C");
        assert_eq!(sku.capacity, 0);
    }

    #[tokio::test]
    async fn create_cache_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/redis/{CACHE}"
        ))
        .returning_json(cache_json());
        let client = make_client(mock);
        let body = RedisCreateRequest {
            location: "eastus".into(),
            properties: RedisCreateProperties {
                sku: RedisSku {
                    name: "Basic".into(),
                    family: "C".into(),
                    capacity: 0,
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let c = client
            .redis()
            .create_cache(RG, CACHE, &body)
            .await
            .expect("create_cache failed");
        assert_eq!(c.name.as_deref(), Some(CACHE));
    }

    #[tokio::test]
    async fn delete_cache_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/redis/{CACHE}"
        ))
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .redis()
            .delete_cache(RG, CACHE)
            .await
            .expect("delete_cache failed");
    }

    #[tokio::test]
    async fn list_keys_returns_both_keys() {
        let mut mock = MockClient::new();
        mock.expect_post(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/redis/{CACHE}/listKeys"),
        )
        .returning_json(keys_json());
        let client = make_client(mock);
        let keys = client
            .redis()
            .list_keys(RG, CACHE)
            .await
            .expect("list_keys failed");
        assert_eq!(keys.primary_key.as_deref(), Some("primary-key-value"));
        assert_eq!(keys.secondary_key.as_deref(), Some("secondary-key-value"));
    }

    #[tokio::test]
    async fn regenerate_key_returns_new_keys() {
        let mut mock = MockClient::new();
        mock.expect_post(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/redis/{CACHE}/regenerateKey"),
        )
        .returning_json(keys_json());
        let client = make_client(mock);
        let keys = client
            .redis()
            .regenerate_key(
                RG,
                CACHE,
                &RedisRegenerateKeyParameters {
                    key_type: "Secondary".into(),
                },
            )
            .await
            .expect("regenerate_key failed");
        assert_eq!(keys.secondary_key.as_deref(), Some("secondary-key-value"));
    }

    #[tokio::test]
    async fn force_reboot_returns_message() {
        let mut mock = MockClient::new();
        mock.expect_post(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Cache/redis/{CACHE}/forceReboot"),
        )
        .returning_json(serde_json::json!({
            "message": "The requested reboot operation has been successfully scheduled."
        }));
        let client = make_client(mock);
        let result = client
            .redis()
            .force_reboot(
                RG,
                CACHE,
                &RedisRebootParameters {
                    reboot_type: "AllNodes".into(),
                    shard_id: None,
                },
            )
            .await
            .expect("force_reboot failed");
        assert!(
            result
                .message
                .as_deref()
                .unwrap_or("")
                .contains("scheduled")
        );
    }
}
