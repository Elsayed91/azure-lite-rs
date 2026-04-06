//! Operation contracts for the Azure Monitor API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/monitor.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::monitor::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Monitor API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::monitor::MonitorClient`] instead.
pub struct MonitorOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> MonitorOps<'a> {
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://management.azure.com"
    }

    /// Lists the metric definitions for the resource.
    ///
    /// **Azure API**: `GET /{resourceUri}/providers/microsoft.insights/metricDefinitions`
    ///
    /// # Path Parameters
    /// - `resourceUri` —  *(required)*
    ///
    /// # Response
    /// [`MetricDefinitionCollection`]
    #[allow(dead_code)]
    pub(crate) async fn list_metric_definitions(
        &self,
        resource_uri: &str,
    ) -> Result<MetricDefinitionCollection> {
        let url = format!(
            "{}/{}/providers/microsoft.insights/metricDefinitions",
            self.base_url(),
            encode(resource_uri),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-10-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_metric_definitions response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_metric_definitions response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the metric values for a resource.
    ///
    /// **Azure API**: `GET /{resourceUri}/providers/microsoft.insights/metrics`
    ///
    /// # Path Parameters
    /// - `resourceUri` —  *(required)*
    ///
    /// # Response
    /// [`MetricsResponse`]
    #[allow(dead_code)]
    pub(crate) async fn get_metrics(&self, resource_uri: &str) -> Result<MetricsResponse> {
        let url = format!(
            "{}/{}/providers/microsoft.insights/metrics",
            self.base_url(),
            encode(resource_uri),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-10-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_metrics response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_metrics response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Retrieve alert rule definitions in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Insights/metricAlerts`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`MetricAlertResourceCollection`]
    #[allow(dead_code)]
    pub(crate) async fn list_alert_rules(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<MetricAlertResourceCollection> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-10-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_alert_rules response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_alert_rules response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Retrieve an alert rule definition.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Insights/metricAlerts/{ruleName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `ruleName` —  *(required)*
    ///
    /// # Response
    /// [`MetricAlertResource`]
    #[allow(dead_code)]
    pub(crate) async fn get_alert_rule(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> Result<MetricAlertResource> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(rule_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-10-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_alert_rule response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_alert_rule response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update an alert rule definition.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Insights/metricAlerts/{ruleName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `ruleName` —  *(required)*
    ///
    /// # Request Body
    /// [`MetricAlertCreateRequest`]
    ///
    /// # Response
    /// [`MetricAlertResource`]
    #[allow(dead_code)]
    pub(crate) async fn create_alert_rule(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
        body: &MetricAlertCreateRequest,
    ) -> Result<MetricAlertResource> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(rule_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-10-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_alert_rule request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_alert_rule response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_alert_rule response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Delete an alert rule definition.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Insights/metricAlerts/{ruleName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `ruleName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_alert_rule(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        rule_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/metricAlerts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(rule_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-10-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Provides the list of records from the activity logs.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/microsoft.insights/eventtypes/management/values`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`EventDataCollection`]
    #[allow(dead_code)]
    pub(crate) async fn list_activity_logs(
        &self,
        subscription_id: &str,
    ) -> Result<EventDataCollection> {
        let url = format!(
            "{}/subscriptions/{}/providers/microsoft.insights/eventtypes/management/values",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-10-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_activity_logs response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_activity_logs response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_metric_definitions() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-resourceUri/providers/microsoft.insights/metricDefinitions")
            .returning_json(serde_json::to_value(MetricDefinitionCollection::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = MonitorOps::new(&client);

        let result = ops.list_metric_definitions("test-resourceUri").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_metrics() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/test-resourceUri/providers/microsoft.insights/metrics")
            .returning_json(serde_json::to_value(MetricsResponse::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = MonitorOps::new(&client);

        let result = ops.get_metrics("test-resourceUri").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_alert_rules() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Insights/metricAlerts")
            .returning_json(serde_json::to_value(MetricAlertResourceCollection::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = MonitorOps::new(&client);

        let result = ops
            .list_alert_rules("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_alert_rule() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Insights/metricAlerts/test-ruleName")
            .returning_json(serde_json::to_value(MetricAlertResource::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = MonitorOps::new(&client);

        let result = ops
            .get_alert_rule(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-ruleName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_alert_rule() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Insights/metricAlerts/test-ruleName")
            .returning_json(serde_json::to_value(MetricAlertResource::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = MonitorOps::new(&client);

        let body = MetricAlertCreateRequest::fixture();
        let result = ops
            .create_alert_rule(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-ruleName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_alert_rule() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Insights/metricAlerts/test-ruleName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = MonitorOps::new(&client);

        let result = ops
            .delete_alert_rule(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-ruleName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_activity_logs() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/microsoft.insights/eventtypes/management/values")
            .returning_json(serde_json::to_value(EventDataCollection::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = MonitorOps::new(&client);

        let result = ops.list_activity_logs("test-subscriptionId").await;
        assert!(result.is_ok());
    }
}
