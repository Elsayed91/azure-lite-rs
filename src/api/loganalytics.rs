//! Azure Log Analytics API client.
//!
//! Wraps the ARM management plane operations for Azure Log Analytics
//! (Microsoft.OperationalInsights): workspace management, KQL query execution,
//! and saved searches.
//!
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::loganalytics::LoganalyticsOps,
    types::loganalytics::{
        LogQueryBody, LogQueryResult, SavedSearchListResult, Workspace, WorkspaceCreateRequest,
        WorkspaceListResult,
    },
};

/// Client for Azure Log Analytics ARM management plane.
pub struct LogAnalyticsClient<'a> {
    ops: LoganalyticsOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> LogAnalyticsClient<'a> {
    /// Create a new Log Analytics API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: LoganalyticsOps::new(client),
            client,
        }
    }

    /// Gets the workspaces in a subscription.
    pub async fn list_workspaces(&self) -> Result<WorkspaceListResult> {
        self.ops
            .list_workspaces(self.client.subscription_id())
            .await
    }

    /// Gets a workspace instance.
    pub async fn get_workspace(
        &self,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> Result<Workspace> {
        self.ops
            .get_workspace(
                self.client.subscription_id(),
                resource_group_name,
                workspace_name,
            )
            .await
    }

    /// Create or update a workspace.
    pub async fn create_workspace(
        &self,
        resource_group_name: &str,
        workspace_name: &str,
        body: &WorkspaceCreateRequest,
    ) -> Result<Workspace> {
        self.ops
            .create_workspace(
                self.client.subscription_id(),
                resource_group_name,
                workspace_name,
                body,
            )
            .await
    }

    /// Deletes a workspace resource instance.
    pub async fn delete_workspace(
        &self,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_workspace(
                self.client.subscription_id(),
                resource_group_name,
                workspace_name,
            )
            .await
    }

    /// Execute a KQL query against a Log Analytics workspace.
    pub async fn query_logs(
        &self,
        resource_group_name: &str,
        workspace_name: &str,
        body: &LogQueryBody,
    ) -> Result<LogQueryResult> {
        self.ops
            .query_logs(
                self.client.subscription_id(),
                resource_group_name,
                workspace_name,
                body,
            )
            .await
    }

    /// Gets the saved searches for a given Log Analytics workspace.
    pub async fn list_saved_searches(
        &self,
        resource_group_name: &str,
        workspace_name: &str,
    ) -> Result<SavedSearchListResult> {
        self.ops
            .list_saved_searches(
                self.client.subscription_id(),
                resource_group_name,
                workspace_name,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;
    use crate::types::loganalytics::WorkspaceCreateRequest;

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "cloud-lite-test-rg";
    const WS_NAME: &str = "cloud-lite-test-ralph-workspace";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn workspace_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.OperationalInsights/workspaces/{WS_NAME}"),
            "name": WS_NAME,
            "type": "Microsoft.OperationalInsights/workspaces",
            "location": "eastus",
            "properties": {
                "customerId": "aae30729-30f7-4237-aec7-59447782acbb",
                "provisioningState": "Succeeded",
                "retentionInDays": 30,
                "sku": { "name": "PerGB2018" }
            }
        })
    }

    #[tokio::test]
    async fn list_workspaces_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.OperationalInsights/workspaces"
        ))
        .returning_json(serde_json::json!({ "value": [workspace_json()] }));
        let client = make_client(mock);
        let result = client
            .log_analytics()
            .list_workspaces()
            .await
            .expect("list_workspaces failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(WS_NAME));
    }

    #[tokio::test]
    async fn get_workspace_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourcegroups/{RG}/providers/Microsoft.OperationalInsights/workspaces/{WS_NAME}"),
        )
        .returning_json(workspace_json());
        let client = make_client(mock);
        let ws = client
            .log_analytics()
            .get_workspace(RG, WS_NAME)
            .await
            .expect("get_workspace failed");
        assert_eq!(ws.name.as_deref(), Some(WS_NAME));
        let props = ws.properties.as_ref().unwrap();
        assert_eq!(
            props.customer_id.as_deref(),
            Some("aae30729-30f7-4237-aec7-59447782acbb")
        );
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
        assert_eq!(props.retention_in_days, Some(30));
    }

    #[tokio::test]
    async fn create_workspace_returns_workspace() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourcegroups/{RG}/providers/Microsoft.OperationalInsights/workspaces/{WS_NAME}"),
        )
        .returning_json(workspace_json());
        let client = make_client(mock);
        let body = WorkspaceCreateRequest {
            location: "eastus".into(),
            ..Default::default()
        };
        let ws = client
            .log_analytics()
            .create_workspace(RG, WS_NAME, &body)
            .await
            .expect("create_workspace failed");
        assert_eq!(ws.name.as_deref(), Some(WS_NAME));
    }

    #[tokio::test]
    async fn delete_workspace_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourcegroups/{RG}/providers/Microsoft.OperationalInsights/workspaces/{WS_NAME}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .log_analytics()
            .delete_workspace(RG, WS_NAME)
            .await
            .expect("delete_workspace failed");
    }

    #[tokio::test]
    async fn query_logs_returns_tables() {
        let mut mock = MockClient::new();
        mock.expect_post(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.OperationalInsights/workspaces/{WS_NAME}/query"),
        )
        .returning_json(serde_json::json!({
            "tables": [
                {
                    "name": "PrimaryResult",
                    "columns": [
                        { "name": "TimeGenerated", "type": "datetime" },
                        { "name": "Category", "type": "string" }
                    ],
                    "rows": [
                        ["2024-01-01T00:00:00Z", "Administrative"]
                    ]
                }
            ]
        }));
        let client = make_client(mock);
        let body = LogQueryBody {
            query: "AzureActivity | limit 5".into(),
            timespan: Some("PT1H".into()),
            ..Default::default()
        };
        let result = client
            .log_analytics()
            .query_logs(RG, WS_NAME, &body)
            .await
            .expect("query_logs failed");
        assert_eq!(result.tables.len(), 1);
        assert_eq!(result.tables[0].name.as_deref(), Some("PrimaryResult"));
        assert_eq!(result.tables[0].columns.len(), 2);
        assert_eq!(
            result.tables[0].columns[0].name.as_deref(),
            Some("TimeGenerated")
        );
        assert_eq!(result.tables[0].rows.len(), 1);
    }

    #[tokio::test]
    async fn list_saved_searches_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.OperationalInsights/workspaces/{WS_NAME}/savedSearches"),
        )
        .returning_json(serde_json::json!({ "value": [] }));
        let client = make_client(mock);
        let result = client
            .log_analytics()
            .list_saved_searches(RG, WS_NAME)
            .await
            .expect("list_saved_searches failed");
        assert_eq!(result.value.len(), 0);
    }
}
