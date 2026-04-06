//! Azure Cost Management API client.
//!
//! Wraps the ARM management plane operations for Azure Cost Management:
//! budgets, cost queries, forecasts, and usage details.
//!
//! Operation API versions:
//! - Budgets: `2023-11-01` (Microsoft.Consumption)
//! - Cost Query: `2023-11-01` (Microsoft.CostManagement)
//! - Forecast: `2023-11-01` (Microsoft.CostManagement)
//! - Usage Details: `2023-03-01` (Microsoft.Consumption) — direct HTTP
//!
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureError, AzureHttpClient, Result,
    ops::cost::CostOps,
    types::cost::{
        Budget, BudgetCreateRequest, BudgetListResult, ForecastDefinition, QueryDefinition,
        QueryResult, UsageDetailsListResult,
    },
};

/// Client for the Azure Cost Management ARM management plane.
pub struct CostClient<'a> {
    ops: CostOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> CostClient<'a> {
    /// Create a new Azure Cost Management API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: CostOps::new(client),
            client,
        }
    }

    fn sub_id(&self) -> &str {
        self.client.subscription_id()
    }

    // -------------------------------------------------------------------------
    // Budgets (Microsoft.Consumption, api-version 2023-11-01)
    // -------------------------------------------------------------------------

    /// Lists all budgets for the subscription.
    pub async fn list_budgets(&self) -> Result<BudgetListResult> {
        self.ops.list_budgets(self.sub_id()).await
    }

    /// Gets the budget for the subscription by budget name.
    pub async fn get_budget(&self, budget_name: &str) -> Result<Budget> {
        self.ops.get_budget(self.sub_id(), budget_name).await
    }

    /// The operation to create or update a budget.
    pub async fn create_budget(
        &self,
        budget_name: &str,
        body: &BudgetCreateRequest,
    ) -> Result<Budget> {
        self.ops
            .create_budget(self.sub_id(), budget_name, body)
            .await
    }

    /// The operation to delete a budget.
    pub async fn delete_budget(&self, budget_name: &str) -> Result<()> {
        self.ops.delete_budget(self.sub_id(), budget_name).await
    }

    // -------------------------------------------------------------------------
    // Cost Query (Microsoft.CostManagement, api-version 2023-11-01)
    // -------------------------------------------------------------------------

    /// Query usage data for subscription scope grouped by resource.
    pub async fn list_cost_by_resource(&self, body: &QueryDefinition) -> Result<QueryResult> {
        self.ops.list_cost_by_resource(self.sub_id(), body).await
    }

    // -------------------------------------------------------------------------
    // Forecast (Microsoft.CostManagement, api-version 2023-11-01)
    // -------------------------------------------------------------------------

    /// Lists the forecast charges for subscription scope.
    pub async fn get_forecast(&self, body: &ForecastDefinition) -> Result<QueryResult> {
        self.ops.get_forecast(self.sub_id(), body).await
    }

    // -------------------------------------------------------------------------
    // Usage Details (Microsoft.Consumption, api-version 2023-03-01)
    // Uses direct HTTP because 2023-11-01 is not supported for usageDetails.
    // -------------------------------------------------------------------------

    /// Lists the usage details for the subscription.
    pub async fn get_usage_details(&self) -> Result<UsageDetailsListResult> {
        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Consumption/usageDetails?api-version=2023-03-01",
            urlencoding::encode(self.sub_id()),
        );
        let resp = self.client.get(&url).await?;
        let resp = resp.error_for_status().await?;
        let bytes = resp.bytes().await?;
        serde_json::from_slice(&bytes).map_err(|e| AzureError::InvalidResponse {
            message: format!("Failed to parse get_usage_details response: {e}"),
            body: Some(String::from_utf8_lossy(&bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const BUDGET_NAME: &str = "cloud-lite-test-ralph-budget";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn budget_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/providers/Microsoft.Consumption/budgets/{BUDGET_NAME}"),
            "name": BUDGET_NAME,
            "type": "Microsoft.Consumption/budgets",
            "properties": {
                "category": "Cost",
                "amount": 100.0,
                "timeGrain": "Monthly",
                "timePeriod": { "startDate": "2026-02-01" },
                "currentSpend": { "amount": 12.5, "unit": "USD" }
            }
        })
    }

    fn query_result_json() -> serde_json::Value {
        serde_json::json!({
            "id": "/subscriptions/test/providers/Microsoft.CostManagement/query/test",
            "name": "test",
            "type": "Microsoft.CostManagement/query",
            "properties": {
                "columns": [
                    { "name": "Cost", "type": "Number" },
                    { "name": "ResourceType", "type": "String" },
                    { "name": "Currency", "type": "String" }
                ],
                "rows": []
            }
        })
    }

    #[tokio::test]
    async fn list_budgets_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Consumption/budgets"
        ))
        .returning_json(serde_json::json!({ "value": [budget_json()] }));
        let client = make_client(mock);
        let result = client
            .cost()
            .list_budgets()
            .await
            .expect("list_budgets failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(BUDGET_NAME));
    }

    #[tokio::test]
    async fn get_budget_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Consumption/budgets/{BUDGET_NAME}"
        ))
        .returning_json(budget_json());
        let client = make_client(mock);
        let budget = client
            .cost()
            .get_budget(BUDGET_NAME)
            .await
            .expect("get_budget failed");
        assert_eq!(budget.name.as_deref(), Some(BUDGET_NAME));
        let props = budget
            .properties
            .as_ref()
            .expect("budget has no properties");
        assert_eq!(props.amount, 100.0);
        assert_eq!(props.time_grain, "Monthly");
        assert_eq!(props.category.as_deref(), Some("Cost"));
        let spend = props.current_spend.as_ref().expect("no currentSpend");
        assert_eq!(spend.unit.as_deref(), Some("USD"));
    }

    #[tokio::test]
    async fn create_budget_returns_budget() {
        let mut mock = MockClient::new();
        mock.expect_put(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Consumption/budgets/{BUDGET_NAME}"
        ))
        .returning_json(budget_json());
        let client = make_client(mock);
        let body = BudgetCreateRequest::fixture();
        let budget = client
            .cost()
            .create_budget(BUDGET_NAME, &body)
            .await
            .expect("create_budget failed");
        assert_eq!(budget.name.as_deref(), Some(BUDGET_NAME));
    }

    #[tokio::test]
    async fn delete_budget_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Consumption/budgets/{BUDGET_NAME}"
        ))
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .cost()
            .delete_budget(BUDGET_NAME)
            .await
            .expect("delete_budget failed");
    }

    #[tokio::test]
    async fn list_cost_by_resource_returns_columns() {
        let mut mock = MockClient::new();
        mock.expect_post(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.CostManagement/query"
        ))
        .returning_json(query_result_json());
        let client = make_client(mock);
        let body = QueryDefinition::fixture();
        let result = client
            .cost()
            .list_cost_by_resource(&body)
            .await
            .expect("list_cost_by_resource failed");
        let props = result.properties.as_ref().expect("no properties");
        assert_eq!(props.columns.len(), 3);
        assert_eq!(props.columns[0].name.as_deref(), Some("Cost"));
    }

    #[tokio::test]
    async fn get_usage_details_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Consumption/usageDetails"
        ))
        .returning_json(serde_json::json!({ "value": [] }));
        let client = make_client(mock);
        let result = client
            .cost()
            .get_usage_details()
            .await
            .expect("get_usage_details failed");
        assert_eq!(result.value.len(), 0);
    }
}
