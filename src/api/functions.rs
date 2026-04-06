//! Azure Functions API client.
//!
//! Wraps the ARM management plane operations for Azure Functions (App Service
//! Web Sites API). All URL construction is in `ops::functions::FunctionsOps`.
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::functions::FunctionsOps,
    types::functions::{
        AppSettingsResult, AppSettingsUpdateRequest, Function, FunctionApp,
        FunctionAppCreateRequest, FunctionAppListResult, FunctionListResult,
    },
};

/// Client for the Azure Functions ARM management plane.
///
/// Wraps [`FunctionsOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct FunctionsClient<'a> {
    ops: FunctionsOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> FunctionsClient<'a> {
    /// Create a new Azure Functions API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: FunctionsOps::new(client),
            client,
        }
    }

    // --- Function App operations ---

    /// Lists all Function Apps in the subscription.
    pub async fn list_function_apps(&self) -> Result<FunctionAppListResult> {
        self.ops
            .list_function_apps(self.client.subscription_id())
            .await
    }

    /// Lists all Function Apps in a resource group.
    pub async fn list_function_apps_by_resource_group(
        &self,
        resource_group_name: &str,
    ) -> Result<FunctionAppListResult> {
        self.ops
            .list_function_apps_by_resource_group(
                self.client.subscription_id(),
                resource_group_name,
            )
            .await
    }

    /// Gets a Function App.
    pub async fn get_function_app(
        &self,
        resource_group_name: &str,
        name: &str,
    ) -> Result<FunctionApp> {
        self.ops
            .get_function_app(self.client.subscription_id(), resource_group_name, name)
            .await
    }

    /// Creates or updates a Function App.
    pub async fn create_function_app(
        &self,
        resource_group_name: &str,
        name: &str,
        body: &FunctionAppCreateRequest,
    ) -> Result<FunctionApp> {
        self.ops
            .create_function_app(
                self.client.subscription_id(),
                resource_group_name,
                name,
                body,
            )
            .await
    }

    /// Deletes a Function App.
    pub async fn delete_function_app(&self, resource_group_name: &str, name: &str) -> Result<()> {
        self.ops
            .delete_function_app(self.client.subscription_id(), resource_group_name, name)
            .await
    }

    // --- Function operations ---

    /// Lists all functions in a Function App.
    pub async fn list_functions(
        &self,
        resource_group_name: &str,
        name: &str,
    ) -> Result<FunctionListResult> {
        self.ops
            .list_functions(self.client.subscription_id(), resource_group_name, name)
            .await
    }

    /// Gets a specific function in a Function App.
    pub async fn get_function(
        &self,
        resource_group_name: &str,
        name: &str,
        function_name: &str,
    ) -> Result<Function> {
        self.ops
            .get_function(
                self.client.subscription_id(),
                resource_group_name,
                name,
                function_name,
            )
            .await
    }

    // --- App Settings operations ---

    /// Gets the application settings of a Function App.
    pub async fn list_app_settings(
        &self,
        resource_group_name: &str,
        name: &str,
    ) -> Result<AppSettingsResult> {
        self.ops
            .list_app_settings(self.client.subscription_id(), resource_group_name, name)
            .await
    }

    /// Updates the application settings of a Function App.
    pub async fn update_app_settings(
        &self,
        resource_group_name: &str,
        name: &str,
        body: &AppSettingsUpdateRequest,
    ) -> Result<AppSettingsResult> {
        self.ops
            .update_app_settings(
                self.client.subscription_id(),
                resource_group_name,
                name,
                body,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const APP: &str = "cloud-lite-test-func-app";
    const FUNC: &str = "MyFunction";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn app_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}"),
            "name": APP,
            "type": "Microsoft.Web/sites",
            "kind": "functionapp,linux",
            "location": "eastus",
            "properties": {
                "state": "Running",
                "hostNames": ["cloud-lite-test-func-app.azurewebsites.net"],
                "defaultHostName": "cloud-lite-test-func-app.azurewebsites.net",
                "resourceGroup": RG,
                "serverFarmId": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/serverfarms/EastUSLinuxDynamicPlan"),
                "siteConfig": {
                    "numberOfWorkers": 1,
                    "linuxFxVersion": "Python|3.11"
                }
            }
        })
    }

    fn function_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}/functions/{FUNC}"),
            "name": format!("{APP}/{FUNC}"),
            "type": "Microsoft.Web/sites/functions",
            "properties": {
                "name": FUNC,
                "functionAppId": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}"),
                "isDisabled": false
            }
        })
    }

    fn settings_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}/config/appsettings"),
            "name": "appsettings",
            "type": "Microsoft.Web/sites/config",
            "properties": {
                "AzureWebJobsStorage": "DefaultEndpointsProtocol=https;...",
                "FUNCTIONS_WORKER_RUNTIME": "python"
            }
        })
    }

    #[tokio::test]
    async fn list_function_apps_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Web/sites"
        ))
        .returning_json(serde_json::json!({ "value": [app_json()] }));
        let client = make_client(mock);
        let result = client
            .functions()
            .list_function_apps()
            .await
            .expect("list_function_apps failed");
        assert_eq!(result.value.len(), 1);
        let app = &result.value[0];
        assert_eq!(app.name.as_deref(), Some(APP));
        let props = app.properties.as_ref().unwrap();
        assert_eq!(props.state.as_deref(), Some("Running"));
    }

    #[tokio::test]
    async fn list_function_apps_by_resource_group_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites"
        ))
        .returning_json(serde_json::json!({ "value": [app_json()] }));
        let client = make_client(mock);
        let result = client
            .functions()
            .list_function_apps_by_resource_group(RG)
            .await
            .expect("list_function_apps_by_resource_group failed");
        assert_eq!(result.value.len(), 1);
    }

    #[tokio::test]
    async fn get_function_app_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}"
        ))
        .returning_json(app_json());
        let client = make_client(mock);
        let app = client
            .functions()
            .get_function_app(RG, APP)
            .await
            .expect("get_function_app failed");
        assert_eq!(app.name.as_deref(), Some(APP));
        let props = app.properties.as_ref().unwrap();
        assert_eq!(
            props.default_host_name.as_deref(),
            Some("cloud-lite-test-func-app.azurewebsites.net")
        );
        let config = props.site_config.as_ref().unwrap();
        assert_eq!(config.linux_fx_version.as_deref(), Some("Python|3.11"));
    }

    #[tokio::test]
    async fn create_function_app_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}"
        ))
        .returning_json(app_json());
        let client = make_client(mock);
        let body = FunctionAppCreateRequest {
            location: "eastus".into(),
            kind: Some("functionapp,linux".into()),
            ..Default::default()
        };
        let app = client
            .functions()
            .create_function_app(RG, APP, &body)
            .await
            .expect("create_function_app failed");
        assert_eq!(app.name.as_deref(), Some(APP));
    }

    #[tokio::test]
    async fn delete_function_app_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}"
        ))
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .functions()
            .delete_function_app(RG, APP)
            .await
            .expect("delete_function_app failed");
    }

    #[tokio::test]
    async fn list_functions_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}/functions"),
        )
        .returning_json(serde_json::json!({ "value": [function_json()] }));
        let client = make_client(mock);
        let result = client
            .functions()
            .list_functions(RG, APP)
            .await
            .expect("list_functions failed");
        assert_eq!(result.value.len(), 1);
        let f = &result.value[0];
        let props = f.properties.as_ref().unwrap();
        assert_eq!(props.is_disabled, Some(false));
    }

    #[tokio::test]
    async fn get_function_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}/functions/{FUNC}"),
        )
        .returning_json(function_json());
        let client = make_client(mock);
        let f = client
            .functions()
            .get_function(RG, APP, FUNC)
            .await
            .expect("get_function failed");
        let props = f.properties.as_ref().unwrap();
        assert_eq!(props.name.as_deref(), Some(FUNC));
    }

    #[tokio::test]
    async fn list_app_settings_returns_settings() {
        let mut mock = MockClient::new();
        mock.expect_post(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}/config/appsettings/list"),
        )
        .returning_json(settings_json());
        let client = make_client(mock);
        let settings = client
            .functions()
            .list_app_settings(RG, APP)
            .await
            .expect("list_app_settings failed");
        assert!(settings.properties.contains_key("AzureWebJobsStorage"));
        assert!(settings.properties.contains_key("FUNCTIONS_WORKER_RUNTIME"));
    }

    #[tokio::test]
    async fn update_app_settings_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Web/sites/{APP}/config/appsettings"),
        )
        .returning_json(settings_json());
        let client = make_client(mock);
        let mut props = std::collections::HashMap::new();
        props.insert("CLOUD_LITE_TEST".to_string(), "hello".to_string());
        let body = AppSettingsUpdateRequest { properties: props };
        let result = client
            .functions()
            .update_app_settings(RG, APP, &body)
            .await
            .expect("update_app_settings failed");
        assert_eq!(result.name.as_deref(), Some("appsettings"));
    }
}
