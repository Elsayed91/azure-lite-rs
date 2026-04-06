//! MockClient helpers for Azure Redis Cache API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Redis Cache helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait RedisMockHelpers {
    /// Helper to expect `list_caches`: Gets all Redis caches in the specified subscription.
    fn expect_list_caches(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_caches_by_resource_group`: Lists all Redis caches in a resource
    /// group.
    fn expect_list_caches_by_resource_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_cache`: Gets a Redis cache (resource description).
    fn expect_get_cache(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_cache`: Create or replace (overwrite/recreate, with potential
    /// downtime) an existing Redis cache.
    fn expect_create_cache(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_cache`: Deletes a Redis cache.
    fn expect_delete_cache(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_keys`: Retrieve a Redis cache's access keys.
    fn expect_list_keys(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `regenerate_key`: Regenerate Redis cache's access keys.
    fn expect_regenerate_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `force_reboot`: Reboot specified Redis node(s).
    fn expect_force_reboot(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `import_data`: Import data into Redis cache.
    fn expect_import_data(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `export_data`: Export data from the redis cache to blobs in a container.
    fn expect_export_data(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl RedisMockHelpers for MockClient {
    /// Helper to expect `list_caches`: Gets all Redis caches in the specified subscription.
    fn expect_list_caches(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/subscriptions/{subscription_id}/providers/Microsoft.Cache/redis");
        self.expect_get(&path)
    }

    /// Helper to expect `list_caches_by_resource_group`: Lists all Redis caches in a resource
    /// group.
    fn expect_list_caches_by_resource_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_cache`: Gets a Redis cache (resource description).
    fn expect_get_cache(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_cache`: Create or replace (overwrite/recreate, with potential
    /// downtime) an existing Redis cache.
    fn expect_create_cache(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_cache`: Deletes a Redis cache.
    fn expect_delete_cache(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_keys`: Retrieve a Redis cache's access keys.
    fn expect_list_keys(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}/listKeys"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `regenerate_key`: Regenerate Redis cache's access keys.
    fn expect_regenerate_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}/regenerateKey"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `force_reboot`: Reboot specified Redis node(s).
    fn expect_force_reboot(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}/forceReboot"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `import_data`: Import data into Redis cache.
    fn expect_import_data(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}/import"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `export_data`: Export data from the redis cache to blobs in a container.
    fn expect_export_data(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Cache/redis/{name}/export"
        );
        self.expect_post(&path)
    }
}
