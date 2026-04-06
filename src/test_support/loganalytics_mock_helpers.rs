//! MockClient helpers for Azure Log Analytics API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Log Analytics helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait LoganalyticsMockHelpers {
    /// Helper to expect `list_workspaces`: Gets the workspaces in a subscription.
    fn expect_list_workspaces(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_workspace`: Gets a workspace instance.
    fn expect_get_workspace(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_workspace`: Create or update a workspace.
    fn expect_create_workspace(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_workspace`: Deletes a workspace resource instance.
    fn expect_delete_workspace(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `query_logs`: Execute a KQL query against a Log Analytics workspace.
    fn expect_query_logs(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_saved_searches`: Gets the saved searches for a given Log Analytics
    /// workspace.
    fn expect_list_saved_searches(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl LoganalyticsMockHelpers for MockClient {
    /// Helper to expect `list_workspaces`: Gets the workspaces in a subscription.
    fn expect_list_workspaces(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.OperationalInsights/workspaces"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_workspace`: Gets a workspace instance.
    fn expect_get_workspace(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/Microsoft.OperationalInsights/workspaces/{workspace_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_workspace`: Create or update a workspace.
    fn expect_create_workspace(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/Microsoft.OperationalInsights/workspaces/{workspace_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_workspace`: Deletes a workspace resource instance.
    fn expect_delete_workspace(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourcegroups/{resource_group_name}/providers/Microsoft.OperationalInsights/workspaces/{workspace_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `query_logs`: Execute a KQL query against a Log Analytics workspace.
    fn expect_query_logs(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.OperationalInsights/workspaces/{workspace_name}/query"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `list_saved_searches`: Gets the saved searches for a given Log Analytics
    /// workspace.
    fn expect_list_saved_searches(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.OperationalInsights/workspaces/{workspace_name}/savedSearches"
        );
        self.expect_get(&path)
    }
}
