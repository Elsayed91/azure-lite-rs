//! MockClient helpers for Azure Container Registry API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Container Registry helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait AcrMockHelpers {
    /// Helper to expect `list_registries`: Lists all the container registries under the specified
    /// subscription.
    fn expect_list_registries(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_registry`: Gets the properties of the specified container registry.
    fn expect_get_registry(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_registry`: Creates or updates a container registry.
    fn expect_create_registry(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_registry`: Deletes a container registry.
    fn expect_delete_registry(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl AcrMockHelpers for MockClient {
    /// Helper to expect `list_registries`: Lists all the container registries under the specified
    /// subscription.
    fn expect_list_registries(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.ContainerRegistry/registries"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_registry`: Gets the properties of the specified container registry.
    fn expect_get_registry(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerRegistry/registries/{registry_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_registry`: Creates or updates a container registry.
    fn expect_create_registry(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerRegistry/registries/{registry_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_registry`: Deletes a container registry.
    fn expect_delete_registry(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ContainerRegistry/registries/{registry_name}"
        );
        self.expect_delete(&path)
    }
}
