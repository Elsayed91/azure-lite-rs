//! Operation contracts for the Azure Functions API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/functions.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::functions::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Functions API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::functions::FunctionsClient`] instead.
pub struct FunctionsOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> FunctionsOps<'a> {
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

    /// List all Function Apps in the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Web/sites`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`FunctionAppListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_function_apps(
        &self,
        subscription_id: &str,
    ) -> Result<FunctionAppListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Web/sites",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_function_apps response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_function_apps response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// List all Function Apps in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`FunctionAppListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_function_apps_by_resource_group(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<FunctionAppListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!(
                        "Failed to read list_function_apps_by_resource_group response: {e}"
                    ),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_function_apps_by_resource_group response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get a Function App.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{name}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Response
    /// [`FunctionApp`]
    #[allow(dead_code)]
    pub(crate) async fn get_function_app(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> Result<FunctionApp> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_function_app response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_function_app response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update a Function App.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{name}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Request Body
    /// [`FunctionAppCreateRequest`]
    ///
    /// # Response
    /// [`FunctionApp`]
    #[allow(dead_code)]
    pub(crate) async fn create_function_app(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        body: &FunctionAppCreateRequest,
    ) -> Result<FunctionApp> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_function_app request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_function_app response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_function_app response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Delete a Function App.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{name}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_function_app(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// List the functions in a Function App.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{name}/functions`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Response
    /// [`FunctionListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_functions(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> Result<FunctionListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/functions",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_functions response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_functions response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get a function in a Function App.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{name}/functions/{functionName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    /// - `functionName` —  *(required)*
    ///
    /// # Response
    /// [`Function`]
    #[allow(dead_code)]
    pub(crate) async fn get_function(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        function_name: &str,
    ) -> Result<Function> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/functions/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
            encode(function_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_function response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_function response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get the application settings of a Function App.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{name}/config/appsettings/list`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Response
    /// [`AppSettingsResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_app_settings(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
    ) -> Result<AppSettingsResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/config/appsettings/list",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_app_settings response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_app_settings response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Update the application settings of a Function App.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Web/sites/{name}/config/appsettings`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `name` —  *(required)*
    ///
    /// # Request Body
    /// [`AppSettingsUpdateRequest`]
    ///
    /// # Response
    /// [`AppSettingsResult`]
    #[allow(dead_code)]
    pub(crate) async fn update_app_settings(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        name: &str,
        body: &AppSettingsUpdateRequest,
    ) -> Result<AppSettingsResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Web/sites/{}/config/appsettings",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-12-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize update_app_settings request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read update_app_settings response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse update_app_settings response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_function_apps() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Web/sites")
            .returning_json(serde_json::to_value(FunctionAppListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let result = ops.list_function_apps("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_function_apps_by_resource_group() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites")
            .returning_json(serde_json::to_value(FunctionAppListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let result = ops
            .list_function_apps_by_resource_group("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_function_app() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites/test-name")
            .returning_json(serde_json::to_value(FunctionApp::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let result = ops
            .get_function_app("test-subscriptionId", "test-resourceGroupName", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_function_app() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites/test-name")
            .returning_json(serde_json::to_value(FunctionApp::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let body = FunctionAppCreateRequest::fixture();
        let result = ops
            .create_function_app(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_function_app() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites/test-name")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let result = ops
            .delete_function_app("test-subscriptionId", "test-resourceGroupName", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_functions() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites/test-name/functions")
            .returning_json(serde_json::to_value(FunctionListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let result = ops
            .list_functions("test-subscriptionId", "test-resourceGroupName", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_function() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites/test-name/functions/test-functionName")
            .returning_json(serde_json::to_value(Function::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let result = ops
            .get_function(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                "test-functionName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_app_settings() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites/test-name/config/appsettings/list")
            .returning_json(serde_json::to_value(AppSettingsResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let result = ops
            .list_app_settings("test-subscriptionId", "test-resourceGroupName", "test-name")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_app_settings() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Web/sites/test-name/config/appsettings")
            .returning_json(serde_json::to_value(AppSettingsResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = FunctionsOps::new(&client);

        let body = AppSettingsUpdateRequest::fixture();
        let result = ops
            .update_app_settings(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-name",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }
}
