//! Operation contracts for the Azure SQL API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/sql.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::sql::{
    Database, DatabaseCreateRequest, DatabaseListResult, EnableServerAuditingRequest, FirewallRule,
    FirewallRuleCreateRequest, FirewallRuleListResult, Server, ServerBlobAuditingPolicy,
    ServerCreateRequest, ServerListResult,
};
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure SQL API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::sql::SqlClient`] instead.
pub struct SqlOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> SqlOps<'a> {
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

    /// Gets a list of all servers in the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Sql/servers`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`ServerListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_servers(&self, subscription_id: &str) -> Result<ServerListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Sql/servers",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_servers response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_servers response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a server.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    ///
    /// # Response
    /// [`Server`]
    #[allow(dead_code)]
    pub(crate) async fn get_server(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<Server> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_server response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_server response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a server.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    ///
    /// # Request Body
    /// [`ServerCreateRequest`]
    ///
    /// # Response
    /// [`Server`]
    #[allow(dead_code)]
    pub(crate) async fn create_server(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        body: &ServerCreateRequest,
    ) -> Result<Server> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_server request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_server response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_server response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a server.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_server(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Gets a list of databases.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/databases`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    ///
    /// # Response
    /// [`DatabaseListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_databases(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<DatabaseListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/databases",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_databases response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_databases response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a database.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    /// - `databaseName` —  *(required)*
    ///
    /// # Response
    /// [`Database`]
    #[allow(dead_code)]
    pub(crate) async fn get_database(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> Result<Database> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/databases/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
            encode(database_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_database response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a database.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    /// - `databaseName` —  *(required)*
    ///
    /// # Request Body
    /// [`DatabaseCreateRequest`]
    ///
    /// # Response
    /// [`Database`]
    #[allow(dead_code)]
    pub(crate) async fn create_database(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
        body: &DatabaseCreateRequest,
    ) -> Result<Database> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/databases/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
            encode(database_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_database request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_database response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a database.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/databases/{databaseName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    /// - `databaseName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_database(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/databases/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
            encode(database_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Gets a list of firewall rules.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/firewallRules`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    ///
    /// # Response
    /// [`FirewallRuleListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_firewall_rules(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<FirewallRuleListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/firewallRules",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_firewall_rules response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_firewall_rules response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the server-level blob auditing policy (`auditingSettings/default`).
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/auditingSettings/default`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    ///
    /// # Response
    /// [`ServerBlobAuditingPolicy`]
    #[allow(dead_code)]
    pub(crate) async fn get_server_audit_policy(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<ServerBlobAuditingPolicy> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/auditingSettings/default",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2021-11-01-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_server_audit_policy response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_server_audit_policy response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Enables server-level blob auditing (`auditingSettings/default`).
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/auditingSettings/default`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    ///
    /// # Request Body
    /// [`EnableServerAuditingRequest`]
    ///
    /// # Response
    /// [`ServerBlobAuditingPolicy`]
    #[allow(dead_code)]
    pub(crate) async fn enable_server_auditing(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        body: &EnableServerAuditingRequest,
    ) -> Result<ServerBlobAuditingPolicy> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/auditingSettings/default",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2021-11-01-preview", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize enable_server_auditing request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read enable_server_auditing response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse enable_server_auditing response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a firewall rule.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Sql/servers/{serverName}/firewallRules/{firewallRuleName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `serverName` —  *(required)*
    /// - `firewallRuleName` —  *(required)*
    ///
    /// # Request Body
    /// [`FirewallRuleCreateRequest`]
    ///
    /// # Response
    /// [`FirewallRule`]
    #[allow(dead_code)]
    pub(crate) async fn create_firewall_rule(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        server_name: &str,
        firewall_rule_name: &str,
        body: &FirewallRuleCreateRequest,
    ) -> Result<FirewallRule> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/firewallRules/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(server_name),
            encode(firewall_rule_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2023-08-01-preview", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_firewall_rule request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_firewall_rule response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_firewall_rule response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_servers() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Sql/servers")
            .returning_json(serde_json::to_value(ServerListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let result = ops.list_servers("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_server() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName")
            .returning_json(serde_json::to_value(Server::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let result = ops
            .get_server(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_server() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName")
            .returning_json(serde_json::to_value(Server::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let body = ServerCreateRequest::fixture();
        let result = ops
            .create_server(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_server() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let result = ops
            .delete_server(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_databases() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName/databases")
            .returning_json(serde_json::to_value(DatabaseListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let result = ops
            .list_databases(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName/databases/test-databaseName")
            .returning_json(serde_json::to_value(Database::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let result = ops
            .get_database(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
                "test-databaseName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName/databases/test-databaseName")
            .returning_json(serde_json::to_value(Database::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let body = DatabaseCreateRequest::fixture();
        let result = ops
            .create_database(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
                "test-databaseName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName/databases/test-databaseName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let result = ops
            .delete_database(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
                "test-databaseName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_firewall_rules() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName/firewallRules")
            .returning_json(serde_json::to_value(FirewallRuleListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let result = ops
            .list_firewall_rules(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_firewall_rule() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Sql/servers/test-serverName/firewallRules/test-firewallRuleName")
            .returning_json(serde_json::to_value(FirewallRule::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SqlOps::new(&client);

        let body = FirewallRuleCreateRequest::fixture();
        let result = ops
            .create_firewall_rule(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-serverName",
                "test-firewallRuleName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }
}
