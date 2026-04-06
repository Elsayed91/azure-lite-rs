//! MockClient helpers for Azure Storage API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Storage helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait StorageMockHelpers {
    /// Helper to expect `list_storage_accounts`: Lists all storage accounts in a subscription
    fn expect_list_storage_accounts(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_storage_accounts_by_resource_group`: Lists all storage accounts in a
    /// resource group
    fn expect_list_storage_accounts_by_resource_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_storage_account`: Returns the properties of a storage account
    fn expect_get_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_storage_account`: Creates a new storage account
    fn expect_create_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_storage_account`: Deletes a storage account
    fn expect_delete_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_keys`: Lists the access keys for a storage account
    fn expect_list_keys(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `regenerate_key`: Regenerates one of the access keys for a storage account
    fn expect_regenerate_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `patch_storage_account`: Updates properties of an existing storage account
    /// (partial PATCH — only provided fields are changed)
    fn expect_patch_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_management_policy`: Gets the blob lifecycle management policy for a
    /// storage account
    fn expect_get_management_policy(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl StorageMockHelpers for MockClient {
    /// Helper to expect `list_storage_accounts`: Lists all storage accounts in a subscription
    fn expect_list_storage_accounts(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts");
        self.expect_get(&path)
    }

    /// Helper to expect `list_storage_accounts_by_resource_group`: Lists all storage accounts in a
    /// resource group
    fn expect_list_storage_accounts_by_resource_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_storage_account`: Returns the properties of a storage account
    fn expect_get_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts/{account_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_storage_account`: Creates a new storage account
    fn expect_create_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts/{account_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_storage_account`: Deletes a storage account
    fn expect_delete_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts/{account_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_keys`: Lists the access keys for a storage account
    fn expect_list_keys(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts/{account_name}/listKeys"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `regenerate_key`: Regenerates one of the access keys for a storage account
    fn expect_regenerate_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts/{account_name}/regenerateKey"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `patch_storage_account`: Updates properties of an existing storage account
    /// (partial PATCH — only provided fields are changed)
    fn expect_patch_storage_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts/{account_name}"
        );
        self.expect_patch(&path)
    }

    /// Helper to expect `get_management_policy`: Gets the blob lifecycle management policy for a
    /// storage account
    fn expect_get_management_policy(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Storage/storageAccounts/{account_name}/managementPolicies/default"
        );
        self.expect_get(&path)
    }
}
