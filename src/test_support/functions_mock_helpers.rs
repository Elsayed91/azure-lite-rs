//! MockClient helpers for Azure Functions API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Functions helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait FunctionsMockHelpers {
    /// Helper to expect `list_function_apps`: List all Function Apps in the subscription.
    fn expect_list_function_apps(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_function_apps_by_resource_group`: List all Function Apps in a
    /// resource group.
    fn expect_list_function_apps_by_resource_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_function_app`: Get a Function App.
    fn expect_get_function_app(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_function_app`: Create or update a Function App.
    fn expect_create_function_app(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_function_app`: Delete a Function App.
    fn expect_delete_function_app(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_functions`: List the functions in a Function App.
    fn expect_list_functions(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_function`: Get a function in a Function App.
    fn expect_get_function(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        function_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_app_settings`: Get the application settings of a Function App.
    fn expect_list_app_settings(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_app_settings`: Update the application settings of a Function App.
    fn expect_update_app_settings(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl FunctionsMockHelpers for MockClient {
    /// Helper to expect `list_function_apps`: List all Function Apps in the subscription.
    fn expect_list_function_apps(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/subscriptions/{subscription_id}/providers/Microsoft.Web/sites");
        self.expect_get(&path)
    }

    /// Helper to expect `list_function_apps_by_resource_group`: List all Function Apps in a
    /// resource group.
    fn expect_list_function_apps_by_resource_group(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_function_app`: Get a Function App.
    fn expect_get_function_app(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites/{name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_function_app`: Create or update a Function App.
    fn expect_create_function_app(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites/{name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_function_app`: Delete a Function App.
    fn expect_delete_function_app(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites/{name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_functions`: List the functions in a Function App.
    fn expect_list_functions(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites/{name}/functions"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_function`: Get a function in a Function App.
    fn expect_get_function(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        function_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites/{name}/functions/{function_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_app_settings`: Get the application settings of a Function App.
    fn expect_list_app_settings(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites/{name}/config/appsettings/list"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `update_app_settings`: Update the application settings of a Function App.
    fn expect_update_app_settings(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Web/sites/{name}/config/appsettings"
        );
        self.expect_put(&path)
    }
}
