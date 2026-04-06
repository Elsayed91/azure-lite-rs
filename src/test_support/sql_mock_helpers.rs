//! MockClient helpers for Azure SQL API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure SQL helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SqlMockHelpers {
    /// Helper to expect `list_servers`: Gets a list of all servers in the subscription.
    fn expect_list_servers(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_server`: Gets a server.
    fn expect_get_server(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_server`: Creates or updates a server.
    fn expect_create_server(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_server`: Deletes a server.
    fn expect_delete_server(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_databases`: Gets a list of databases.
    fn expect_list_databases(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_database`: Gets a database.
    fn expect_get_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_database`: Creates or updates a database.
    fn expect_create_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_database`: Deletes a database.
    fn expect_delete_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_firewall_rules`: Gets a list of firewall rules.
    fn expect_list_firewall_rules(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_firewall_rule`: Creates or updates a firewall rule.
    fn expect_create_firewall_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        firewall_rule_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SqlMockHelpers for MockClient {
    /// Helper to expect `list_servers`: Gets a list of all servers in the subscription.
    fn expect_list_servers(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/subscriptions/{subscription_id}/providers/Microsoft.Sql/servers");
        self.expect_get(&path)
    }

    /// Helper to expect `get_server`: Gets a server.
    fn expect_get_server(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_server`: Creates or updates a server.
    fn expect_create_server(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_server`: Deletes a server.
    fn expect_delete_server(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_databases`: Gets a list of databases.
    fn expect_list_databases(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}/databases"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_database`: Gets a database.
    fn expect_get_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}/databases/{database_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_database`: Creates or updates a database.
    fn expect_create_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}/databases/{database_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_database`: Deletes a database.
    fn expect_delete_database(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}/databases/{database_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_firewall_rules`: Gets a list of firewall rules.
    fn expect_list_firewall_rules(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}/firewallRules"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_firewall_rule`: Creates or updates a firewall rule.
    fn expect_create_firewall_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        firewall_rule_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Sql/servers/{server_name}/firewallRules/{firewall_rule_name}"
        );
        self.expect_put(&path)
    }
}
