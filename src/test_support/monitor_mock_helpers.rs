//! MockClient helpers for Azure Monitor API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Monitor helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait MonitorMockHelpers {
    /// Helper to expect `list_metric_definitions`: Lists the metric definitions for the resource.
    fn expect_list_metric_definitions(&mut self, resource_uri: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_metrics`: Lists the metric values for a resource.
    fn expect_get_metrics(&mut self, resource_uri: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_alert_rules`: Retrieve alert rule definitions in a resource group.
    fn expect_list_alert_rules(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_alert_rule`: Retrieve an alert rule definition.
    fn expect_get_alert_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_alert_rule`: Create or update an alert rule definition.
    fn expect_create_alert_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_alert_rule`: Delete an alert rule definition.
    fn expect_delete_alert_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_activity_logs`: Provides the list of records from the activity logs.
    fn expect_list_activity_logs(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl MonitorMockHelpers for MockClient {
    /// Helper to expect `list_metric_definitions`: Lists the metric definitions for the resource.
    fn expect_list_metric_definitions(
        &mut self,
        resource_uri: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{resource_uri}/providers/microsoft.insights/metricDefinitions");
        self.expect_get(&path)
    }

    /// Helper to expect `get_metrics`: Lists the metric values for a resource.
    fn expect_get_metrics(
        &mut self,
        resource_uri: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/{resource_uri}/providers/microsoft.insights/metrics");
        self.expect_get(&path)
    }

    /// Helper to expect `list_alert_rules`: Retrieve alert rule definitions in a resource group.
    fn expect_list_alert_rules(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Insights/metricAlerts"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_alert_rule`: Retrieve an alert rule definition.
    fn expect_get_alert_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Insights/metricAlerts/{rule_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_alert_rule`: Create or update an alert rule definition.
    fn expect_create_alert_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Insights/metricAlerts/{rule_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_alert_rule`: Delete an alert rule definition.
    fn expect_delete_alert_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Insights/metricAlerts/{rule_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_activity_logs`: Provides the list of records from the activity logs.
    fn expect_list_activity_logs(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/microsoft.insights/eventtypes/management/values"
        );
        self.expect_get(&path)
    }
}
