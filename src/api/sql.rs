//! Azure SQL API client.
//!
//! Wraps the ARM management plane operations for Azure SQL: servers,
//! databases, and firewall rules. All URL construction is in
//! `ops::sql::SqlOps`. `subscription_id` is auto-injected from the
//! parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::sql::SqlOps,
    types::sql::{
        Database, DatabaseCreateRequest, DatabaseListResult, EnableServerAuditingRequest,
        FirewallRule, FirewallRuleCreateRequest, FirewallRuleListResult, Server,
        ServerBlobAuditingPolicy, ServerCreateRequest, ServerListResult,
    },
};

/// Client for the Azure SQL ARM management plane.
///
/// Wraps [`SqlOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct SqlClient<'a> {
    ops: SqlOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> SqlClient<'a> {
    /// Create a new Azure SQL API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: SqlOps::new(client),
            client,
        }
    }

    // --- Server operations ---

    /// Gets a list of all servers in the subscription.
    pub async fn list_servers(&self) -> Result<ServerListResult> {
        self.ops.list_servers(self.client.subscription_id()).await
    }

    /// Gets a server.
    pub async fn get_server(&self, resource_group_name: &str, server_name: &str) -> Result<Server> {
        self.ops
            .get_server(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
            )
            .await
    }

    /// Creates or updates a server.
    pub async fn create_server(
        &self,
        resource_group_name: &str,
        server_name: &str,
        body: &ServerCreateRequest,
    ) -> Result<Server> {
        self.ops
            .create_server(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
                body,
            )
            .await
    }

    /// Deletes a server.
    pub async fn delete_server(&self, resource_group_name: &str, server_name: &str) -> Result<()> {
        self.ops
            .delete_server(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
            )
            .await
    }

    // --- Database operations ---

    /// Gets a list of databases in a server.
    pub async fn list_databases(
        &self,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<DatabaseListResult> {
        self.ops
            .list_databases(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
            )
            .await
    }

    /// Gets a database.
    pub async fn get_database(
        &self,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> Result<Database> {
        self.ops
            .get_database(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
                database_name,
            )
            .await
    }

    /// Creates or updates a database.
    pub async fn create_database(
        &self,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
        body: &DatabaseCreateRequest,
    ) -> Result<Database> {
        self.ops
            .create_database(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
                database_name,
                body,
            )
            .await
    }

    /// Deletes a database.
    pub async fn delete_database(
        &self,
        resource_group_name: &str,
        server_name: &str,
        database_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_database(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
                database_name,
            )
            .await
    }

    // --- Auditing operations ---

    /// Gets the server-level blob auditing policy (`auditingSettings/default`).
    pub async fn get_server_audit_policy(
        &self,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<ServerBlobAuditingPolicy> {
        self.ops
            .get_server_audit_policy(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
            )
            .await
    }

    /// Enables server-level auditing (sets `state: "Enabled"` on `auditingSettings/default`).
    pub async fn enable_server_auditing(
        &self,
        resource_group_name: &str,
        server_name: &str,
        body: &EnableServerAuditingRequest,
    ) -> Result<ServerBlobAuditingPolicy> {
        self.ops
            .enable_server_auditing(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
                body,
            )
            .await
    }

    // --- Firewall rule operations ---

    /// Gets a list of firewall rules for a server.
    pub async fn list_firewall_rules(
        &self,
        resource_group_name: &str,
        server_name: &str,
    ) -> Result<FirewallRuleListResult> {
        self.ops
            .list_firewall_rules(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
            )
            .await
    }

    /// Creates or updates a firewall rule.
    pub async fn create_firewall_rule(
        &self,
        resource_group_name: &str,
        server_name: &str,
        firewall_rule_name: &str,
        body: &FirewallRuleCreateRequest,
    ) -> Result<FirewallRule> {
        self.ops
            .create_firewall_rule(
                self.client.subscription_id(),
                resource_group_name,
                server_name,
                firewall_rule_name,
                body,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MockClient,
        types::sql::{DatabaseSku, FirewallRuleProperties, ServerCreateOrUpdateProperties},
    };

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const SERVER: &str = "cloud-lite-test-sql-srv";
    const DB: &str = "cloud-lite-test-db";
    const FW_RULE: &str = "cloud-lite-test-fw-rule";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn server_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}"),
            "name": SERVER,
            "type": "Microsoft.Sql/servers",
            "location": "eastus",
            "properties": {
                "administratorLogin": "cloudliteadmin",
                "fullyQualifiedDomainName": format!("{SERVER}.database.windows.net"),
                "state": "Ready",
                "version": "12.0"
            }
        })
    }

    fn database_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/databases/{DB}"),
            "name": DB,
            "type": "Microsoft.Sql/servers/databases",
            "location": "eastus",
            "sku": { "name": "Basic", "tier": "Basic" },
            "properties": {
                "status": "Online",
                "collation": "SQL_Latin1_General_CP1_CI_AS",
                "requestedServiceObjectiveName": "Basic",
                "currentServiceObjectiveName": "Basic"
            }
        })
    }

    fn firewall_rule_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/firewallRules/{FW_RULE}"),
            "name": FW_RULE,
            "type": "Microsoft.Sql/servers/firewallRules",
            "properties": {
                "startIpAddress": "0.0.0.0",
                "endIpAddress": "0.0.0.0"
            }
        })
    }

    #[tokio::test]
    async fn list_servers_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Sql/servers"
        ))
        .returning_json(serde_json::json!({ "value": [server_json()] }));
        let client = make_client(mock);
        let result = client
            .sql()
            .list_servers()
            .await
            .expect("list_servers failed");
        assert_eq!(result.value.len(), 1);
        let s = &result.value[0];
        assert_eq!(s.name.as_deref(), Some(SERVER));
        let props = s.properties.as_ref().unwrap();
        assert_eq!(props.state.as_deref(), Some("Ready"));
    }

    #[tokio::test]
    async fn get_server_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}"
        ))
        .returning_json(server_json());
        let client = make_client(mock);
        let s = client
            .sql()
            .get_server(RG, SERVER)
            .await
            .expect("get_server failed");
        assert_eq!(s.name.as_deref(), Some(SERVER));
        let props = s.properties.as_ref().unwrap();
        assert_eq!(props.administrator_login.as_deref(), Some("cloudliteadmin"));
        assert_eq!(
            props.fully_qualified_domain_name.as_deref(),
            Some(&format!("{SERVER}.database.windows.net") as &str)
        );
        assert_eq!(props.version.as_deref(), Some("12.0"));
    }

    #[tokio::test]
    async fn create_server_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}"
        ))
        .returning_json(server_json());
        let client = make_client(mock);
        let body = ServerCreateRequest {
            location: "eastus".into(),
            properties: Some(ServerCreateOrUpdateProperties {
                administrator_login: "cloudliteadmin".into(),
                administrator_login_password: Some("Password123!".into()),
                version: Some("12.0".into()),
            }),
            ..Default::default()
        };
        let s = client
            .sql()
            .create_server(RG, SERVER, &body)
            .await
            .expect("create_server failed");
        assert_eq!(s.name.as_deref(), Some(SERVER));
    }

    #[tokio::test]
    async fn delete_server_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(&format!(
            "/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}"
        ))
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .sql()
            .delete_server(RG, SERVER)
            .await
            .expect("delete_server failed");
    }

    #[tokio::test]
    async fn list_databases_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/databases"),
        )
        .returning_json(serde_json::json!({ "value": [database_json()] }));
        let client = make_client(mock);
        let result = client
            .sql()
            .list_databases(RG, SERVER)
            .await
            .expect("list_databases failed");
        assert_eq!(result.value.len(), 1);
        let db = &result.value[0];
        let props = db.properties.as_ref().unwrap();
        assert_eq!(props.status.as_deref(), Some("Online"));
    }

    #[tokio::test]
    async fn get_database_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/databases/{DB}"),
        )
        .returning_json(database_json());
        let client = make_client(mock);
        let db = client
            .sql()
            .get_database(RG, SERVER, DB)
            .await
            .expect("get_database failed");
        assert_eq!(db.name.as_deref(), Some(DB));
        let props = db.properties.as_ref().unwrap();
        assert_eq!(
            props.current_service_objective_name.as_deref(),
            Some("Basic")
        );
        assert_eq!(
            props.collation.as_deref(),
            Some("SQL_Latin1_General_CP1_CI_AS")
        );
        let sku = db.sku.as_ref().unwrap();
        assert_eq!(sku.name, "Basic");
    }

    #[tokio::test]
    async fn create_database_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/databases/{DB}"),
        )
        .returning_json(database_json());
        let client = make_client(mock);
        let body = DatabaseCreateRequest {
            location: "eastus".into(),
            sku: Some(DatabaseSku {
                name: "Basic".into(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let db = client
            .sql()
            .create_database(RG, SERVER, DB, &body)
            .await
            .expect("create_database failed");
        assert_eq!(db.name.as_deref(), Some(DB));
    }

    #[tokio::test]
    async fn delete_database_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/databases/{DB}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .sql()
            .delete_database(RG, SERVER, DB)
            .await
            .expect("delete_database failed");
    }

    #[tokio::test]
    async fn list_firewall_rules_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/firewallRules"),
        )
        .returning_json(serde_json::json!({ "value": [firewall_rule_json()] }));
        let client = make_client(mock);
        let result = client
            .sql()
            .list_firewall_rules(RG, SERVER)
            .await
            .expect("list_firewall_rules failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(FW_RULE));
    }

    #[tokio::test]
    async fn create_firewall_rule_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Sql/servers/{SERVER}/firewallRules/{FW_RULE}"),
        )
        .returning_json(firewall_rule_json());
        let client = make_client(mock);
        let body = FirewallRuleCreateRequest {
            properties: Some(FirewallRuleProperties {
                start_ip_address: "0.0.0.0".into(),
                end_ip_address: "0.0.0.0".into(),
            }),
        };
        let fw = client
            .sql()
            .create_firewall_rule(RG, SERVER, FW_RULE, &body)
            .await
            .expect("create_firewall_rule failed");
        assert_eq!(fw.name.as_deref(), Some(FW_RULE));
        let props = fw.properties.as_ref().unwrap();
        assert_eq!(props.start_ip_address, "0.0.0.0");
    }
}
