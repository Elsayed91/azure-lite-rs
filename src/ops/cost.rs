//! Operation contracts for the Azure Cost Management API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cost.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cost::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Cost Management API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cost::CostClient`] instead.
pub struct CostOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> CostOps<'a> {
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

    /// Lists all budgets for the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Consumption/budgets`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`BudgetListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_budgets(&self, subscription_id: &str) -> Result<BudgetListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Consumption/budgets",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-11-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_budgets response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_budgets response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the budget for the subscription by budget name.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Consumption/budgets/{budgetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `budgetName` —  *(required)*
    ///
    /// # Response
    /// [`Budget`]
    #[allow(dead_code)]
    pub(crate) async fn get_budget(
        &self,
        subscription_id: &str,
        budget_name: &str,
    ) -> Result<Budget> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Consumption/budgets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(budget_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-11-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_budget response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_budget response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// The operation to create or update a budget.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/providers/Microsoft.Consumption/budgets/{budgetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `budgetName` —  *(required)*
    ///
    /// # Request Body
    /// [`BudgetCreateRequest`]
    ///
    /// # Response
    /// [`Budget`]
    #[allow(dead_code)]
    pub(crate) async fn create_budget(
        &self,
        subscription_id: &str,
        budget_name: &str,
        body: &BudgetCreateRequest,
    ) -> Result<Budget> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Consumption/budgets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(budget_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-11-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_budget request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_budget response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_budget response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// The operation to delete a budget.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/providers/Microsoft.Consumption/budgets/{budgetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `budgetName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_budget(
        &self,
        subscription_id: &str,
        budget_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Consumption/budgets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(budget_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-11-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Query the usage data for subscription scope.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/providers/Microsoft.CostManagement/query`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Request Body
    /// [`QueryDefinition`]
    ///
    /// # Response
    /// [`QueryResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_cost_by_resource(
        &self,
        subscription_id: &str,
        body: &QueryDefinition,
    ) -> Result<QueryResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.CostManagement/query",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-11-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize list_cost_by_resource request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_cost_by_resource response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_cost_by_resource response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the forecast charges for subscription scope.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/providers/Microsoft.CostManagement/forecast`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Request Body
    /// [`ForecastDefinition`]
    ///
    /// # Response
    /// [`QueryResult`]
    #[allow(dead_code)]
    pub(crate) async fn get_forecast(
        &self,
        subscription_id: &str,
        body: &ForecastDefinition,
    ) -> Result<QueryResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.CostManagement/forecast",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-11-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize get_forecast request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_forecast response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_forecast response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the usage details for the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Consumption/usageDetails`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`UsageDetailsListResult`]
    #[allow(dead_code)]
    pub(crate) async fn get_usage_details(
        &self,
        subscription_id: &str,
    ) -> Result<UsageDetailsListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Consumption/usageDetails",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-11-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_usage_details response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_usage_details response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_budgets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Consumption/budgets",
        )
        .returning_json(serde_json::to_value(BudgetListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CostOps::new(&client);

        let result = ops.list_budgets("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_budget() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Consumption/budgets/test-budgetName")
            .returning_json(serde_json::to_value(Budget::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CostOps::new(&client);

        let result = ops
            .get_budget("test-subscriptionId", "test-budgetName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_budget() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/providers/Microsoft.Consumption/budgets/test-budgetName")
            .returning_json(serde_json::to_value(Budget::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CostOps::new(&client);

        let body = BudgetCreateRequest::fixture();
        let result = ops
            .create_budget("test-subscriptionId", "test-budgetName", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_budget() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/providers/Microsoft.Consumption/budgets/test-budgetName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CostOps::new(&client);

        let result = ops
            .delete_budget("test-subscriptionId", "test-budgetName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_cost_by_resource() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/subscriptions/test-subscriptionId/providers/Microsoft.CostManagement/query",
        )
        .returning_json(serde_json::to_value(QueryResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CostOps::new(&client);

        let body = QueryDefinition::fixture();
        let result = ops
            .list_cost_by_resource("test-subscriptionId", &body)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_forecast() {
        let mut mock = crate::MockClient::new();

        mock.expect_post(
            "/subscriptions/test-subscriptionId/providers/Microsoft.CostManagement/forecast",
        )
        .returning_json(serde_json::to_value(QueryResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CostOps::new(&client);

        let body = ForecastDefinition::fixture();
        let result = ops.get_forecast("test-subscriptionId", &body).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_usage_details() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Consumption/usageDetails",
        )
        .returning_json(serde_json::to_value(UsageDetailsListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CostOps::new(&client);

        let result = ops.get_usage_details("test-subscriptionId").await;
        assert!(result.is_ok());
    }
}
