//! MockClient helpers for Azure Key Vault API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Key Vault helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait KeyvaultMockHelpers {
    /// Helper to expect `list_vaults`: The List operation gets information about the vaults
    /// associated with the subscription.
    fn expect_list_vaults(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_vaults_in_group`: The List operation gets information about the
    /// vaults associated with the subscription and within the specified resource group.
    fn expect_list_vaults_in_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_vault`: Gets the specified Azure key vault.
    fn expect_get_vault(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_vault`: Create or update a key vault in the specified subscription.
    fn expect_create_vault(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_vault`: Deletes the specified Azure key vault.
    fn expect_delete_vault(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_secrets`: The List operation gets information about the secrets in a
    /// vault. NOTE: This operation does not return secret values.
    fn expect_list_secrets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_secret`: Gets the specified secret. NOTE: This operation does not
    /// return the secret value.
    fn expect_get_secret(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `set_secret`: Create or update a secret in a key vault in the specified
    /// subscription.
    fn expect_set_secret(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_keys`: Lists the keys in the specified key vault.
    fn expect_list_keys(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_key`: Gets the current version of the specified key from the specified
    /// key vault.
    fn expect_get_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_key`: Creates the first version of a new key if it does not exist.
    /// If it already exists, then the existing key is returned without any write operations being
    /// performed.
    fn expect_create_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl KeyvaultMockHelpers for MockClient {
    /// Helper to expect `list_vaults`: The List operation gets information about the vaults
    /// associated with the subscription.
    fn expect_list_vaults(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/subscriptions/{subscription_id}/providers/Microsoft.KeyVault/vaults");
        self.expect_get(&path)
    }

    /// Helper to expect `list_vaults_in_group`: The List operation gets information about the
    /// vaults associated with the subscription and within the specified resource group.
    fn expect_list_vaults_in_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_vault`: Gets the specified Azure key vault.
    fn expect_get_vault(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_vault`: Create or update a key vault in the specified subscription.
    fn expect_create_vault(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_vault`: Deletes the specified Azure key vault.
    fn expect_delete_vault(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_secrets`: The List operation gets information about the secrets in a
    /// vault. NOTE: This operation does not return secret values.
    fn expect_list_secrets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}/secrets"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_secret`: Gets the specified secret. NOTE: This operation does not
    /// return the secret value.
    fn expect_get_secret(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}/secrets/{secret_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `set_secret`: Create or update a secret in a key vault in the specified
    /// subscription.
    fn expect_set_secret(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}/secrets/{secret_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `list_keys`: Lists the keys in the specified key vault.
    fn expect_list_keys(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}/keys"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_key`: Gets the current version of the specified key from the specified
    /// key vault.
    fn expect_get_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}/keys/{key_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_key`: Creates the first version of a new key if it does not exist.
    /// If it already exists, then the existing key is returned without any write operations being
    /// performed.
    fn expect_create_key(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.KeyVault/vaults/{vault_name}/keys/{key_name}"
        );
        self.expect_put(&path)
    }
}
