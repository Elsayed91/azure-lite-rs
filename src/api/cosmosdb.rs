//! Azure CosmosDB API client.
//!
//! Wraps the ARM management plane operations for Azure CosmosDB: database
//! accounts, SQL databases, and SQL containers. All URL construction is in
//! `ops::cosmosdb::CosmosdbOps`. `subscription_id` is auto-injected from the
//! parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::cosmosdb::CosmosdbOps,
    types::cosmosdb::{
        DatabaseAccount, DatabaseAccountCreateRequest, DatabaseAccountListResult,
        SqlContainerGetResults, SqlContainerListResult, SqlDatabaseCreateRequest,
        SqlDatabaseGetResults, SqlDatabaseListResult,
    },
};

/// Client for the Azure CosmosDB ARM management plane.
///
/// Wraps [`CosmosdbOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct CosmosDbClient<'a> {
    ops: CosmosdbOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> CosmosDbClient<'a> {
    /// Create a new Azure CosmosDB API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: CosmosdbOps::new(client),
            client,
        }
    }

    // --- Account operations ---

    /// Lists all the Azure Cosmos DB database accounts available under the subscription.
    pub async fn list_accounts(&self) -> Result<DatabaseAccountListResult> {
        self.ops.list_accounts(self.client.subscription_id()).await
    }

    /// Retrieves the properties of an existing Azure Cosmos DB database account.
    pub async fn get_account(
        &self,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<DatabaseAccount> {
        self.ops
            .get_account(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
            )
            .await
    }

    /// Creates or updates an Azure Cosmos DB database account.
    pub async fn create_account(
        &self,
        resource_group_name: &str,
        account_name: &str,
        body: &DatabaseAccountCreateRequest,
    ) -> Result<DatabaseAccount> {
        self.ops
            .create_account(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
                body,
            )
            .await
    }

    /// Deletes an existing Azure Cosmos DB database account.
    pub async fn delete_account(
        &self,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_account(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
            )
            .await
    }

    // --- SQL Database operations ---

    /// Lists the SQL databases under an existing Azure Cosmos DB database account.
    pub async fn list_sql_databases(
        &self,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<SqlDatabaseListResult> {
        self.ops
            .list_sql_databases(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
            )
            .await
    }

    /// Gets the SQL database under an existing Azure Cosmos DB database account.
    pub async fn get_sql_database(
        &self,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> Result<SqlDatabaseGetResults> {
        self.ops
            .get_sql_database(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
                database_name,
            )
            .await
    }

    /// Creates or updates an Azure Cosmos DB SQL database.
    pub async fn create_sql_database(
        &self,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
        body: &SqlDatabaseCreateRequest,
    ) -> Result<SqlDatabaseGetResults> {
        self.ops
            .create_sql_database(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
                database_name,
                body,
            )
            .await
    }

    // --- SQL Container operations ---

    /// Lists the SQL containers under an existing Azure Cosmos DB SQL database.
    pub async fn list_sql_containers(
        &self,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
    ) -> Result<SqlContainerListResult> {
        self.ops
            .list_sql_containers(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
                database_name,
            )
            .await
    }

    /// Gets the SQL container under an existing Azure Cosmos DB SQL database.
    pub async fn get_sql_container(
        &self,
        resource_group_name: &str,
        account_name: &str,
        database_name: &str,
        container_name: &str,
    ) -> Result<SqlContainerGetResults> {
        self.ops
            .get_sql_container(
                self.client.subscription_id(),
                resource_group_name,
                account_name,
                database_name,
                container_name,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MockClient,
        types::cosmosdb::{
            ConsistencyPolicy, DatabaseAccountCreateUpdateProperties,
            SqlDatabaseCreateUpdateProperties, SqlDatabaseResource,
        },
    };

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const ACCOUNT: &str = "cloud-lite-test-cosmos";
    const DATABASE: &str = "cloud-lite-test-db";
    const CONTAINER: &str = "cloud-lite-test-container";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn account_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}"),
            "name": ACCOUNT,
            "type": "Microsoft.DocumentDB/databaseAccounts",
            "location": "eastus",
            "kind": "GlobalDocumentDB",
            "properties": {
                "documentEndpoint": format!("https://{ACCOUNT}.documents.azure.com:443/"),
                "provisioningState": "Succeeded",
                "databaseAccountOfferType": "Standard",
                "consistencyPolicy": {
                    "defaultConsistencyLevel": "Session",
                    "maxStalenessPrefix": 100,
                    "maxIntervalInSeconds": 5
                },
                "enableAutomaticFailover": false,
                "enableMultipleWriteLocations": false
            }
        })
    }

    fn sql_database_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}/sqlDatabases/{DATABASE}"),
            "name": DATABASE,
            "type": "Microsoft.DocumentDB/databaseAccounts/sqlDatabases",
            "location": "eastus",
            "properties": {
                "resource": {
                    "id": DATABASE,
                    "colls": "colls/",
                    "users": "users/"
                }
            }
        })
    }

    fn sql_container_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}/sqlDatabases/{DATABASE}/containers/{CONTAINER}"),
            "name": CONTAINER,
            "type": "Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers",
            "location": "eastus",
            "properties": {
                "resource": {
                    "id": CONTAINER,
                    "partitionKey": {
                        "kind": "Hash",
                        "version": 2
                    }
                }
            }
        })
    }

    #[tokio::test]
    async fn list_accounts_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.DocumentDB/databaseAccounts"
        ))
        .returning_json(serde_json::json!({ "value": [account_json()] }));
        let client = make_client(mock);
        let result = client
            .cosmosdb()
            .list_accounts()
            .await
            .expect("list_accounts failed");
        assert_eq!(result.value.len(), 1);
        let a = &result.value[0];
        assert_eq!(a.name.as_deref(), Some(ACCOUNT));
        assert_eq!(a.kind.as_deref(), Some("GlobalDocumentDB"));
    }

    #[tokio::test]
    async fn get_account_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}"),
        )
        .returning_json(account_json());
        let client = make_client(mock);
        let a = client
            .cosmosdb()
            .get_account(RG, ACCOUNT)
            .await
            .expect("get_account failed");
        assert_eq!(a.name.as_deref(), Some(ACCOUNT));
        let props = a.properties.as_ref().unwrap();
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
        assert_eq!(
            props.database_account_offer_type.as_deref(),
            Some("Standard")
        );
        assert!(props.document_endpoint.is_some());
        let cp = props.consistency_policy.as_ref().unwrap();
        assert_eq!(cp.default_consistency_level, "Session");
    }

    #[tokio::test]
    async fn create_account_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}"),
        )
        .returning_json(account_json());
        let client = make_client(mock);
        let body = DatabaseAccountCreateRequest {
            location: "eastus".into(),
            kind: Some("GlobalDocumentDB".into()),
            properties: DatabaseAccountCreateUpdateProperties {
                database_account_offer_type: "Standard".into(),
                consistency_policy: Some(ConsistencyPolicy {
                    default_consistency_level: "Session".into(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let a = client
            .cosmosdb()
            .create_account(RG, ACCOUNT, &body)
            .await
            .expect("create_account failed");
        assert_eq!(a.name.as_deref(), Some(ACCOUNT));
    }

    #[tokio::test]
    async fn delete_account_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .cosmosdb()
            .delete_account(RG, ACCOUNT)
            .await
            .expect("delete_account failed");
    }

    #[tokio::test]
    async fn list_sql_databases_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}/sqlDatabases"),
        )
        .returning_json(serde_json::json!({ "value": [sql_database_json()] }));
        let client = make_client(mock);
        let result = client
            .cosmosdb()
            .list_sql_databases(RG, ACCOUNT)
            .await
            .expect("list_sql_databases failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(DATABASE));
    }

    #[tokio::test]
    async fn get_sql_database_deserializes_resource() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}/sqlDatabases/{DATABASE}"),
        )
        .returning_json(sql_database_json());
        let client = make_client(mock);
        let db = client
            .cosmosdb()
            .get_sql_database(RG, ACCOUNT, DATABASE)
            .await
            .expect("get_sql_database failed");
        assert_eq!(db.name.as_deref(), Some(DATABASE));
        let props = db.properties.as_ref().unwrap();
        let resource = props.resource.as_ref().unwrap();
        assert_eq!(resource.id.as_deref(), Some(DATABASE));
        assert_eq!(resource.colls.as_deref(), Some("colls/"));
    }

    #[tokio::test]
    async fn create_sql_database_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}/sqlDatabases/{DATABASE}"),
        )
        .returning_json(sql_database_json());
        let client = make_client(mock);
        let body = SqlDatabaseCreateRequest {
            location: "eastus".into(),
            properties: SqlDatabaseCreateUpdateProperties {
                resource: SqlDatabaseResource {
                    id: DATABASE.into(),
                },
            },
            ..Default::default()
        };
        let db = client
            .cosmosdb()
            .create_sql_database(RG, ACCOUNT, DATABASE, &body)
            .await
            .expect("create_sql_database failed");
        assert_eq!(db.name.as_deref(), Some(DATABASE));
    }

    #[tokio::test]
    async fn list_sql_containers_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}/sqlDatabases/{DATABASE}/containers"),
        )
        .returning_json(serde_json::json!({ "value": [sql_container_json()] }));
        let client = make_client(mock);
        let result = client
            .cosmosdb()
            .list_sql_containers(RG, ACCOUNT, DATABASE)
            .await
            .expect("list_sql_containers failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(CONTAINER));
    }

    #[tokio::test]
    async fn get_sql_container_deserializes_partition_key() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.DocumentDB/databaseAccounts/{ACCOUNT}/sqlDatabases/{DATABASE}/containers/{CONTAINER}"),
        )
        .returning_json(sql_container_json());
        let client = make_client(mock);
        let c = client
            .cosmosdb()
            .get_sql_container(RG, ACCOUNT, DATABASE, CONTAINER)
            .await
            .expect("get_sql_container failed");
        assert_eq!(c.name.as_deref(), Some(CONTAINER));
        let props = c.properties.as_ref().unwrap();
        let resource = props.resource.as_ref().unwrap();
        assert_eq!(resource.id.as_deref(), Some(CONTAINER));
        let pk = resource.partition_key.as_ref().unwrap();
        assert_eq!(pk.kind.as_deref(), Some("Hash"));
        assert_eq!(pk.version, Some(2));
    }
}
