//! Operation contracts for the Azure CosmosDB API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/cosmosdb.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::cosmosdb::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure CosmosDB API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::cosmosdb::CosmosdbClient`] instead.
pub struct CosmosdbOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> CosmosdbOps<'a> {
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

    /// Lists all the Azure Cosmos DB database accounts available under the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.DocumentDB/databaseAccounts`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`DatabaseAccountListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_accounts(
        &self,
        subscription_id: &str,
    ) -> Result<DatabaseAccountListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.DocumentDB/databaseAccounts",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_accounts response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_accounts response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Retrieves the properties of an existing Azure Cosmos DB database account.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Response
    /// [`DatabaseAccount`]
    #[allow(dead_code)]
    pub(crate) async fn get_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<DatabaseAccount> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_account response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates an Azure Cosmos DB database account.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Request Body
    /// [`DatabaseAccountCreateRequest`]
    ///
    /// # Response
    /// [`DatabaseAccount`]
    #[allow(dead_code)]
    pub(crate) async fn create_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        body: &DatabaseAccountCreateRequest,
    ) -> Result<DatabaseAccount> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_account request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_account response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_account response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes an existing Azure Cosmos DB database account.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Lists the SQL databases under an existing Azure Cosmos DB database account.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}/sqlDatabases`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    ///
    /// # Response
    /// [`SqlDatabaseListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_sql_databases(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<SqlDatabaseListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}/sqlDatabases",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_sql_databases response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_sql_databases response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the SQL database under an existing Azure Cosmos DB database account.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}/sqlDatabases/{databaseName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    /// - `databaseName` —  *(required)*
    ///
    /// # Response
    /// [`SqlDatabaseGetResults`]
    #[allow(dead_code)]
    pub(crate) async fn get_sql_database(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> Result<SqlDatabaseGetResults> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}/sqlDatabases/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
            encode(database_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_sql_database response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_sql_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update an Azure Cosmos DB SQL database.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}/sqlDatabases/{databaseName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    /// - `databaseName` —  *(required)*
    ///
    /// # Request Body
    /// [`SqlDatabaseCreateRequest`]
    ///
    /// # Response
    /// [`SqlDatabaseGetResults`]
    #[allow(dead_code)]
    pub(crate) async fn create_sql_database(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
        body: &SqlDatabaseCreateRequest,
    ) -> Result<SqlDatabaseGetResults> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}/sqlDatabases/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
            encode(database_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_sql_database request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_sql_database response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_sql_database response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the SQL container under an existing Azure Cosmos DB database account.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}/sqlDatabases/{databaseName}/containers`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    /// - `databaseName` —  *(required)*
    ///
    /// # Response
    /// [`SqlContainerListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_sql_containers(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> Result<SqlContainerListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}/sqlDatabases/{}/containers",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
            encode(database_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_sql_containers response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_sql_containers response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the SQL container under an existing Azure Cosmos DB database account.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.DocumentDB/databaseAccounts/{accountName}/sqlDatabases/{databaseName}/containers/{containerName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `accountName` —  *(required)*
    /// - `databaseName` —  *(required)*
    /// - `containerName` —  *(required)*
    ///
    /// # Response
    /// [`SqlContainerGetResults`]
    #[allow(dead_code)]
    pub(crate) async fn get_sql_container(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<SqlContainerGetResults> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}/sqlDatabases/{}/containers/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(account_name),
            encode(database_name),
            encode(container_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-02-15-preview", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_sql_container response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_sql_container response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_accounts() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.DocumentDB/databaseAccounts",
        )
        .returning_json(serde_json::to_value(DatabaseAccountListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let result = ops.list_accounts("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName")
            .returning_json(serde_json::to_value(DatabaseAccount::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let result = ops
            .get_account(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName")
            .returning_json(serde_json::to_value(DatabaseAccount::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let body = DatabaseAccountCreateRequest::fixture();
        let result = ops
            .create_account(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_account() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let result = ops
            .delete_account(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_sql_databases() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName/sqlDatabases")
            .returning_json(serde_json::to_value(SqlDatabaseListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let result = ops
            .list_sql_databases(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_sql_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName/sqlDatabases/test-databaseName")
            .returning_json(serde_json::to_value(SqlDatabaseGetResults::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let result = ops
            .get_sql_database(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                "test-databaseName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_sql_database() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName/sqlDatabases/test-databaseName")
            .returning_json(serde_json::to_value(SqlDatabaseGetResults::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let body = SqlDatabaseCreateRequest::fixture();
        let result = ops
            .create_sql_database(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                "test-databaseName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_sql_containers() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName/sqlDatabases/test-databaseName/containers")
            .returning_json(serde_json::to_value(SqlContainerListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let result = ops
            .list_sql_containers(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                "test-databaseName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_sql_container() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.DocumentDB/databaseAccounts/test-accountName/sqlDatabases/test-databaseName/containers/test-containerName")
            .returning_json(serde_json::to_value(SqlContainerGetResults::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = CosmosdbOps::new(&client);

        let result = ops
            .get_sql_container(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-accountName",
                "test-databaseName",
                "test-containerName",
            )
            .await;
        assert!(result.is_ok());
    }
}
