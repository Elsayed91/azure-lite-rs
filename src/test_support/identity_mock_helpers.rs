//! MockClient helpers for Azure Managed Identities API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Managed Identities helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait IdentityMockHelpers {
    /// Helper to expect `list_user_assigned_identities`: Lists all the userAssignedIdentities
    /// available under the specified subscription.
    fn expect_list_user_assigned_identities(
        &mut self,
        subscription_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_user_assigned_identities_in_group`: Lists all the
    /// userAssignedIdentities available under the specified ResourceGroup.
    fn expect_list_user_assigned_identities_in_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_identity`: Gets the identity.
    fn expect_get_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_identity`: Create or update an identity in the specified
    /// subscription and resource group.
    fn expect_create_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_identity`: Deletes the identity.
    fn expect_delete_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_system_assigned_identity`: Gets the systemAssignedIdentity available
    /// under the specified SubscriptionId, ResourceGroupName, and resource name.
    fn expect_get_system_assigned_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        parent_resource_provider: &str,
        parent_resource_type: &str,
        parent_resource_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl IdentityMockHelpers for MockClient {
    /// Helper to expect `list_user_assigned_identities`: Lists all the userAssignedIdentities
    /// available under the specified subscription.
    fn expect_list_user_assigned_identities(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.ManagedIdentity/userAssignedIdentities"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_user_assigned_identities_in_group`: Lists all the
    /// userAssignedIdentities available under the specified ResourceGroup.
    fn expect_list_user_assigned_identities_in_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ManagedIdentity/userAssignedIdentities"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_identity`: Gets the identity.
    fn expect_get_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{resource_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_identity`: Create or update an identity in the specified
    /// subscription and resource group.
    fn expect_create_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{resource_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_identity`: Deletes the identity.
    fn expect_delete_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{resource_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `get_system_assigned_identity`: Gets the systemAssignedIdentity available
    /// under the specified SubscriptionId, ResourceGroupName, and resource name.
    fn expect_get_system_assigned_identity(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        parent_resource_provider: &str,
        parent_resource_type: &str,
        parent_resource_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/{parent_resource_provider}/{parent_resource_type}/{parent_resource_name}/providers/Microsoft.ManagedIdentity/identities/default"
        );
        self.expect_get(&path)
    }
}
