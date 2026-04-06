//! Azure Monitor API client.
//!
//! Wraps the ARM management plane operations for Azure Monitor: metric
//! definitions, metric values, alert rules, and activity logs.
//!
//! Different Azure Monitor operation groups require different API versions:
//! - Metrics / MetricDefinitions: `2023-10-01` (via generated ops)
//! - MetricAlerts: `2018-03-01` (direct HTTP calls — codegen version is wrong)
//! - Activity Logs: `2015-04-01` (direct HTTP calls — codegen version is wrong)
//!
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureError, AzureHttpClient, Result,
    ops::monitor::MonitorOps,
    types::monitor::{
        EventDataCollection, MetricAlertCreateRequest, MetricAlertResource,
        MetricAlertResourceCollection, MetricDefinitionCollection, MetricsResponse,
    },
};

const ALERTS_API_VERSION: &str = "2018-03-01";
const ACTIVITY_LOGS_API_VERSION: &str = "2015-04-01";

/// Client for the Azure Monitor ARM management plane.
///
/// All methods use direct HTTP calls with operation-appropriate API versions:
/// - Metrics / MetricDefinitions: `2023-10-01`
/// - MetricAlerts: `2018-03-01`
/// - Activity Logs: `2015-04-01`
pub struct MonitorClient<'a> {
    client: &'a AzureHttpClient,
    // Kept to silence dead_code on the generated MonitorOps::new().
    // Direct HTTP calls are used instead of the ops methods because
    // different Monitor operations require different api-versions.
    _ops: MonitorOps<'a>,
}

impl<'a> MonitorClient<'a> {
    /// Create a new Azure Monitor API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            client,
            _ops: MonitorOps::new(client),
        }
    }

    fn base_url(&self) -> &str {
        "https://management.azure.com"
    }

    fn parse_json<T: serde::de::DeserializeOwned>(&self, bytes: &[u8], op: &str) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| AzureError::InvalidResponse {
            message: format!("Failed to parse {op} response: {e}"),
            body: Some(String::from_utf8_lossy(bytes).to_string()),
        })
    }

    // --- Metric operations (api-version 2023-10-01 via generated ops) ---

    /// Lists the metric definitions for a resource.
    ///
    /// `resource_uri` is the ARM resource path without the leading `/`,
    /// e.g. `subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Compute/virtualMachines/{vm}`.
    ///
    /// `metric_namespace` is required by the Azure API, e.g. `Microsoft.Compute/virtualMachines`.
    pub async fn list_metric_definitions(
        &self,
        resource_uri: &str,
        metric_namespace: &str,
    ) -> Result<MetricDefinitionCollection> {
        // Append metricnamespace — the generated op URL is missing it
        let sub_url = format!(
            "/{}/providers/microsoft.insights/metricDefinitions?api-version=2023-10-01&metricnamespace={}",
            resource_uri,
            urlencoding::encode(metric_namespace),
        );
        let url = format!("{}{}", self.base_url(), sub_url);
        let resp = self.client.get(&url).await?;
        let resp = resp.error_for_status().await?;
        let bytes = resp.bytes().await?;
        self.parse_json(&bytes, "list_metric_definitions")
    }

    /// Lists the metric values for a resource.
    ///
    /// `resource_uri` is the ARM resource path without the leading `/`.
    /// `metric_names` is a comma-separated list of metric names.
    /// `timespan` is ISO 8601 duration, e.g. `PT1H` for last hour.
    pub async fn get_metrics(
        &self,
        resource_uri: &str,
        metric_names: &str,
        timespan: &str,
    ) -> Result<MetricsResponse> {
        let url = format!(
            "{}/{}/providers/microsoft.insights/metrics?api-version=2023-10-01&metricnames={}&timespan={}",
            self.base_url(),
            resource_uri,
            urlencoding::encode(metric_names),
            urlencoding::encode(timespan),
        );
        let resp = self.client.get(&url).await?;
        let resp = resp.error_for_status().await?;
        let bytes = resp.bytes().await?;
        self.parse_json(&bytes, "get_metrics")
    }

    // --- Alert rule operations (api-version 2018-03-01 — direct HTTP) ---

    /// Retrieve alert rule definitions in a resource group.
    pub async fn list_alert_rules(
        &self,
        resource_group_name: &str,
    ) -> Result<MetricAlertResourceCollection> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts?api-version={}",
            self.base_url(),
            urlencoding::encode(self.client.subscription_id()),
            urlencoding::encode(resource_group_name),
            ALERTS_API_VERSION,
        );
        let resp = self.client.get(&url).await?;
        let resp = resp.error_for_status().await?;
        let bytes = resp.bytes().await?;
        self.parse_json(&bytes, "list_alert_rules")
    }

    /// Retrieve an alert rule definition.
    pub async fn get_alert_rule(
        &self,
        resource_group_name: &str,
        rule_name: &str,
    ) -> Result<MetricAlertResource> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts/{}?api-version={}",
            self.base_url(),
            urlencoding::encode(self.client.subscription_id()),
            urlencoding::encode(resource_group_name),
            urlencoding::encode(rule_name),
            ALERTS_API_VERSION,
        );
        let resp = self.client.get(&url).await?;
        let resp = resp.error_for_status().await?;
        let bytes = resp.bytes().await?;
        self.parse_json(&bytes, "get_alert_rule")
    }

    /// Create or update an alert rule definition.
    pub async fn create_alert_rule(
        &self,
        resource_group_name: &str,
        rule_name: &str,
        body: &MetricAlertCreateRequest,
    ) -> Result<MetricAlertResource> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts/{}?api-version={}",
            self.base_url(),
            urlencoding::encode(self.client.subscription_id()),
            urlencoding::encode(resource_group_name),
            urlencoding::encode(rule_name),
            ALERTS_API_VERSION,
        );
        let body_bytes = serde_json::to_vec(body).map_err(|e| AzureError::InvalidResponse {
            message: format!("Failed to serialize create_alert_rule body: {e}"),
            body: None,
        })?;
        let resp = self.client.put(&url, &body_bytes).await?;
        let resp = resp.error_for_status().await?;
        let bytes = resp.bytes().await?;
        self.parse_json(&bytes, "create_alert_rule")
    }

    /// Delete an alert rule definition.
    pub async fn delete_alert_rule(
        &self,
        resource_group_name: &str,
        rule_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts/{}?api-version={}",
            self.base_url(),
            urlencoding::encode(self.client.subscription_id()),
            urlencoding::encode(resource_group_name),
            urlencoding::encode(rule_name),
            ALERTS_API_VERSION,
        );
        self.client.delete(&url).await?;
        Ok(())
    }

    // --- Activity log operations (api-version 2015-04-01 — direct HTTP) ---

    /// Provides the list of records from the activity logs.
    ///
    /// `filter` is an OData filter string, e.g. `"eventTimestamp ge '2024-01-01T00:00:00Z'"`.
    pub async fn list_activity_logs(&self, filter: &str) -> Result<EventDataCollection> {
        let url = format!(
            "{}/subscriptions/{}/providers/microsoft.insights/eventtypes/management/values?api-version={}&$filter={}",
            self.base_url(),
            urlencoding::encode(self.client.subscription_id()),
            ACTIVITY_LOGS_API_VERSION,
            urlencoding::encode(filter),
        );
        let resp = self.client.get(&url).await?;
        let resp = resp.error_for_status().await?;
        let bytes = resp.bytes().await?;
        self.parse_json(&bytes, "list_activity_logs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const RULE_NAME: &str = "cloud-lite-test-alert-rule";
    const RESOURCE_URI: &str = "subscriptions/test-subscription-id/resourceGroups/test-rg/providers/Microsoft.Compute/virtualMachines/test-vm";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn alert_rule_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Insights/metricAlerts/{RULE_NAME}"),
            "name": RULE_NAME,
            "type": "Microsoft.Insights/metricAlerts",
            "location": "global",
            "properties": {
                "description": "Test alert rule",
                "severity": 3,
                "enabled": true,
                "evaluationFrequency": "PT1M",
                "windowSize": "PT5M",
                "provisioningState": "Succeeded"
            }
        })
    }

    #[tokio::test]
    async fn list_alert_rules_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Insights/metricAlerts"
        ))
        .returning_json(serde_json::json!({ "value": [alert_rule_json()] }));
        let client = make_client(mock);
        let result = client
            .monitor()
            .list_alert_rules(RG)
            .await
            .expect("list_alert_rules failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(RULE_NAME));
    }

    #[tokio::test]
    async fn get_alert_rule_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Insights/metricAlerts/{RULE_NAME}"),
        )
        .returning_json(alert_rule_json());
        let client = make_client(mock);
        let rule = client
            .monitor()
            .get_alert_rule(RG, RULE_NAME)
            .await
            .expect("get_alert_rule failed");
        assert_eq!(rule.name.as_deref(), Some(RULE_NAME));
        let props = rule.properties.as_ref().unwrap();
        assert_eq!(props.severity, Some(3));
        assert_eq!(props.enabled, Some(true));
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
    }

    #[tokio::test]
    async fn create_alert_rule_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Insights/metricAlerts/{RULE_NAME}"),
        )
        .returning_json(alert_rule_json());
        let client = make_client(mock);
        let body = MetricAlertCreateRequest {
            location: "global".into(),
            ..Default::default()
        };
        let rule = client
            .monitor()
            .create_alert_rule(RG, RULE_NAME, &body)
            .await
            .expect("create_alert_rule failed");
        assert_eq!(rule.name.as_deref(), Some(RULE_NAME));
    }

    #[tokio::test]
    async fn delete_alert_rule_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Insights/metricAlerts/{RULE_NAME}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .monitor()
            .delete_alert_rule(RG, RULE_NAME)
            .await
            .expect("delete_alert_rule failed");
    }

    #[tokio::test]
    async fn list_activity_logs_returns_events() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/providers/microsoft.insights/eventtypes/management/values"),
        )
        .returning_json(serde_json::json!({
            "value": [
                {
                    "id": "/subscriptions/test-subscription-id/resourceGroups/test-rg/providers/microsoft.insights/eventtypes/management/values/event1",
                    "resourceGroupName": "test-rg",
                    "level": "Informational",
                    "caller": "user@example.com",
                    "description": "A test event"
                }
            ]
        }));
        let client = make_client(mock);
        let result = client
            .monitor()
            .list_activity_logs("eventTimestamp ge '2024-01-01T00:00:00Z'")
            .await
            .expect("list_activity_logs failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].level.as_deref(), Some("Informational"));
        assert_eq!(result.value[0].caller.as_deref(), Some("user@example.com"));
    }

    #[tokio::test]
    async fn list_metric_definitions_returns_list() {
        let mut mock = MockClient::new();
        // Mock strips query params and just matches path
        mock.expect_get(&format!(
            "/{RESOURCE_URI}/providers/microsoft.insights/metricDefinitions"
        ))
        .returning_json(serde_json::json!({
            "value": [
                {
                    "id": "metric-def-id",
                    "name": "Percentage CPU",
                    "namespace": "Microsoft.Compute/virtualMachines",
                    "unit": "Percent",
                    "primaryAggregationType": "Average"
                }
            ]
        }));
        let client = make_client(mock);
        let result = client
            .monitor()
            .list_metric_definitions(RESOURCE_URI, "Microsoft.Compute/virtualMachines")
            .await
            .expect("list_metric_definitions failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some("Percentage CPU"));
        assert_eq!(result.value[0].unit.as_deref(), Some("Percent"));
    }

    #[tokio::test]
    async fn get_metrics_returns_response() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/{RESOURCE_URI}/providers/microsoft.insights/metrics"
        ))
        .returning_json(serde_json::json!({
            "cost": 0,
            "timespan": "2024-01-01T00:00:00Z/2024-01-01T01:00:00Z",
            "interval": "PT1M",
            "namespace": "Microsoft.Compute/virtualMachines",
            "value": [
                {
                    "id": "/metric/id",
                    "type": "Microsoft.Insights/metrics",
                    "name": "Percentage CPU",
                    "unit": "Percent",
                    "timeseries": [
                        {
                            "data": [
                                { "timeStamp": "2024-01-01T00:00:00Z", "average": 5.0 }
                            ]
                        }
                    ]
                }
            ]
        }));
        let client = make_client(mock);
        let result = client
            .monitor()
            .get_metrics(RESOURCE_URI, "Percentage CPU", "PT1H")
            .await
            .expect("get_metrics failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].unit, "Percent");
        let ts = &result.value[0].timeseries;
        assert_eq!(ts.len(), 1);
        let data = &ts[0].data;
        assert_eq!(data[0].average, Some(5.0));
    }
}
