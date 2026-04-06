//! MockClient helpers for Azure CosmosDB API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure CosmosDB helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CosmosdbMockHelpers {
    /// Helper to expect `list_accounts`: Lists all the Azure Cosmos DB database accounts available
    /// under the subscription.
    fn expect_list_accounts(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_account`: Retrieves the properties of an existing Azure Cosmos DB
    /// database account.
    fn expect_get_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_account`: Creates or updates an Azure Cosmos DB database account.
    fn expect_create_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_account`: Deletes an existing Azure Cosmos DB database account.
    fn expect_delete_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_sql_databases`: Lists the SQL databases under an existing Azure
    /// Cosmos DB database account.
    fn expect_list_sql_databases(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_sql_database`: Gets the SQL database under an existing Azure Cosmos DB
    /// database account.
    fn expect_get_sql_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_sql_database`: Create or update an Azure Cosmos DB SQL database.
    fn expect_create_sql_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_sql_containers`: Lists the SQL container under an existing Azure
    /// Cosmos DB database account.
    fn expect_list_sql_containers(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_sql_container`: Gets the SQL container under an existing Azure Cosmos
    /// DB database account.
    fn expect_get_sql_container(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
        container_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CosmosdbMockHelpers for MockClient {
    /// Helper to expect `list_accounts`: Lists all the Azure Cosmos DB database accounts available
    /// under the subscription.
    fn expect_list_accounts(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.DocumentDB/databaseAccounts"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_account`: Retrieves the properties of an existing Azure Cosmos DB
    /// database account.
    fn expect_get_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_account`: Creates or updates an Azure Cosmos DB database account.
    fn expect_create_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_account`: Deletes an existing Azure Cosmos DB database account.
    fn expect_delete_account(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_sql_databases`: Lists the SQL databases under an existing Azure
    /// Cosmos DB database account.
    fn expect_list_sql_databases(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}/sqlDatabases"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_sql_database`: Gets the SQL database under an existing Azure Cosmos DB
    /// database account.
    fn expect_get_sql_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}/sqlDatabases/{database_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_sql_database`: Create or update an Azure Cosmos DB SQL database.
    fn expect_create_sql_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}/sqlDatabases/{database_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `list_sql_containers`: Lists the SQL container under an existing Azure
    /// Cosmos DB database account.
    fn expect_list_sql_containers(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}/sqlDatabases/{database_name}/containers"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_sql_container`: Gets the SQL container under an existing Azure Cosmos
    /// DB database account.
    fn expect_get_sql_container(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
        container_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}/sqlDatabases/{database_name}/containers/{container_name}"
        );
        self.expect_get(&path)
    }
}
