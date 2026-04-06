//! MockClient helpers for Azure Cost Management API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Cost Management helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait CostMockHelpers {
    /// Helper to expect `list_budgets`: Lists all budgets for the subscription.
    fn expect_list_budgets(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_budget`: Gets the budget for the subscription by budget name.
    fn expect_get_budget(
        &mut self,
        subscription_id: &str,
        budget_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_budget`: The operation to create or update a budget.
    fn expect_create_budget(
        &mut self,
        subscription_id: &str,
        budget_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_budget`: The operation to delete a budget.
    fn expect_delete_budget(
        &mut self,
        subscription_id: &str,
        budget_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_cost_by_resource`: Query the usage data for subscription scope.
    fn expect_list_cost_by_resource(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_forecast`: Lists the forecast charges for subscription scope.
    fn expect_get_forecast(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_usage_details`: Lists the usage details for the subscription.
    fn expect_get_usage_details(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl CostMockHelpers for MockClient {
    /// Helper to expect `list_budgets`: Lists all budgets for the subscription.
    fn expect_list_budgets(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.Consumption/budgets");
        self.expect_get(&path)
    }

    /// Helper to expect `get_budget`: Gets the budget for the subscription by budget name.
    fn expect_get_budget(
        &mut self,
        subscription_id: &str,
        budget_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Consumption/budgets/{budget_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_budget`: The operation to create or update a budget.
    fn expect_create_budget(
        &mut self,
        subscription_id: &str,
        budget_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Consumption/budgets/{budget_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_budget`: The operation to delete a budget.
    fn expect_delete_budget(
        &mut self,
        subscription_id: &str,
        budget_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Consumption/budgets/{budget_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_cost_by_resource`: Query the usage data for subscription scope.
    fn expect_list_cost_by_resource(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.CostManagement/query");
        self.expect_post(&path)
    }

    /// Helper to expect `get_forecast`: Lists the forecast charges for subscription scope.
    fn expect_get_forecast(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.CostManagement/forecast");
        self.expect_post(&path)
    }

    /// Helper to expect `get_usage_details`: Lists the usage details for the subscription.
    fn expect_get_usage_details(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Consumption/usageDetails"
        );
        self.expect_get(&path)
    }
}
