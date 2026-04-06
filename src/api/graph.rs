//! Microsoft Graph API client.
//!
//! Provides access to Entra ID (Azure AD) user objects via the Graph API.
//! Unlike ARM APIs, Graph uses a different OAuth2 scope and base URL:
//!   - Base URL: `https://graph.microsoft.com/v1.0`
//!   - Scope:    `https://graph.microsoft.com/.default`
//!
//! HTTP is handled via `AzureHttpClient::graph_get` / `graph_post`, which
//! acquire Graph-scoped tokens automatically.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    AzureHttpClient, Result,
    types::graph::{GraphBatchRequest, GraphBatchRequestItem, GraphBatchResponse, GraphUser},
};

/// An Entra ID service principal (enterprise application) returned by Microsoft Graph.
///
/// Used to detect provisioning connectors (e.g. Databricks SCIM) by querying
/// `GET /v1.0/servicePrincipals?$filter=displayName eq '...'`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphServicePrincipal {
    /// The unique object ID of the service principal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The application (client) ID associated with this service principal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    /// Display name of the service principal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The type of service principal: "Application", "ManagedIdentity", or "SocialIdp".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal_type: Option<String>,

    /// Whether the service principal is enabled for sign-in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
}

/// Response wrapper for Graph API list queries (`{ "value": [...] }`).
#[derive(Debug, Clone, Default, Deserialize)]
struct GraphListResponse<T> {
    #[serde(default)]
    value: Vec<T>,
}

const GRAPH_BASE: &str = "https://graph.microsoft.com/v1.0";

/// Fields to request from Graph user objects.
///
/// `userType` and `accountEnabled` are not returned by default — they must be explicitly selected.
const USER_SELECT: &str = "$select=id,displayName,userPrincipalName,userType,accountEnabled";

/// Maximum requests per `$batch` call (Graph API limit).
const BATCH_MAX: usize = 20;

/// Client for the Microsoft Graph API.
///
/// Provides Entra ID user lookups needed to distinguish internal users
/// ("Member") from external/guest users ("Guest") in RBAC policy evaluation.
pub struct GraphClient<'a> {
    client: &'a AzureHttpClient,
}

impl<'a> GraphClient<'a> {
    /// Create a new Microsoft Graph API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self { client }
    }

    /// Get a single user by their object ID (principal ID).
    ///
    /// Returns `None` if the user is not found (404).
    pub async fn get_user(&self, principal_id: &str) -> Result<Option<GraphUser>> {
        let url = format!("{GRAPH_BASE}/users/{principal_id}?{USER_SELECT}");
        let response = match self.client.graph_get(&url).await {
            Ok(r) => r,
            Err(crate::AzureError::NotFound { .. }) => return Ok(None),
            Err(e) => return Err(e),
        };
        let response = response.error_for_status().await?;
        let bytes = response.bytes().await?;
        let user =
            serde_json::from_slice(&bytes).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to parse GraphUser: {e}"),
                body: Some(String::from_utf8_lossy(&bytes).to_string()),
            })?;
        Ok(Some(user))
    }

    /// List service principals matching an OData `$filter` expression.
    ///
    /// Example: find the Databricks SCIM provisioning connector:
    /// ```ignore
    /// let results = graph
    ///     .list_service_principals("displayName eq 'Azure Databricks SCIM Provisioning Connector'")
    ///     .await?;
    /// ```
    pub async fn list_service_principals(
        &self,
        filter: &str,
    ) -> Result<Vec<GraphServicePrincipal>> {
        let encoded_filter = urlencoding::encode(filter);
        let url = format!(
            "{GRAPH_BASE}/servicePrincipals?$filter={encoded_filter}&$select=id,appId,displayName,servicePrincipalType,accountEnabled"
        );
        let response = self.client.graph_get(&url).await?;
        let response = response.error_for_status().await?;
        let bytes = response.bytes().await?;
        let list: GraphListResponse<GraphServicePrincipal> =
            serde_json::from_slice(&bytes).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to parse servicePrincipals response: {e}"),
                body: Some(String::from_utf8_lossy(&bytes).to_string()),
            })?;
        Ok(list.value)
    }

    /// Batch-fetch multiple users by their object IDs.
    ///
    /// Returns a map of `principal_id → GraphUser` for users that were found.
    /// Not-found (404) entries are silently omitted from the result.
    ///
    /// Internally uses `$batch` to resolve up to 20 IDs per HTTP call,
    /// handling chunking automatically for larger sets.
    pub async fn batch_get_users(
        &self,
        principal_ids: &[&str],
    ) -> Result<HashMap<String, GraphUser>> {
        let mut result: HashMap<String, GraphUser> = HashMap::new();

        for chunk in principal_ids.chunks(BATCH_MAX) {
            let requests: Vec<GraphBatchRequestItem> = chunk
                .iter()
                .enumerate()
                .map(|(i, id)| GraphBatchRequestItem {
                    id: i.to_string(),
                    method: "GET".to_string(),
                    url: format!("/users/{id}?{USER_SELECT}"),
                })
                .collect();

            // Map batch request index → principalId for result correlation
            let index_to_id: Vec<&str> = chunk.to_vec();

            let body = GraphBatchRequest { requests };
            let body_bytes =
                serde_json::to_vec(&body).map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to serialize GraphBatchRequest: {e}"),
                    body: None,
                })?;

            let url = format!("{GRAPH_BASE}/$batch");
            let response = self.client.graph_post(&url, &body_bytes).await?;
            let response = response.error_for_status().await?;
            let bytes = response.bytes().await?;

            let batch_resp: GraphBatchResponse =
                serde_json::from_slice(&bytes).map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to parse GraphBatchResponse: {e}"),
                    body: Some(String::from_utf8_lossy(&bytes).to_string()),
                })?;

            for item in batch_resp.responses {
                let status = item.status.unwrap_or(0);
                if status == 404 {
                    continue;
                }
                if !(200..300).contains(&status) {
                    continue;
                }
                let idx: usize = item
                    .id
                    .as_deref()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(usize::MAX);
                if idx >= index_to_id.len() {
                    continue;
                }
                let principal_id = index_to_id[idx];

                if let Some(body_val) = item.body
                    && let Ok(user) = serde_json::from_value::<GraphUser>(body_val)
                {
                    result.insert(principal_id.to_string(), user);
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn user_json(id: &str, display_name: &str, user_type: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "displayName": display_name,
            "userPrincipalName": format!("{}@contoso.com", display_name.to_lowercase()),
            "userType": user_type
        })
    }

    // NOTE: The Graph API requires `$select=id,displayName,userPrincipalName,userType` to return
    // userType — it is NOT included in default responses. This is proven by integration tests.
    // Mock expectations use URL path matching, so the ?$select= suffix is matched by StartsWith.

    #[tokio::test]
    async fn get_user_returns_member_user() {
        let mut mock = MockClient::new();
        // Mock matches on path prefix — includes $select suffix in the real URL
        mock.expect_get("/v1.0/users/abc-123")
            .returning_json(user_json("abc-123", "Alice", "Member"));
        let client = make_client(mock);
        let user = client
            .graph()
            .get_user("abc-123")
            .await
            .expect("get_user failed")
            .expect("user should be found");
        assert_eq!(user.id.as_deref(), Some("abc-123"));
        assert_eq!(user.display_name.as_deref(), Some("Alice"));
        assert_eq!(user.user_type.as_deref(), Some("Member"));
    }

    #[tokio::test]
    async fn get_user_returns_guest_user() {
        let mut mock = MockClient::new();
        mock.expect_get("/v1.0/users/ext-456")
            .returning_json(serde_json::json!({
                "id": "ext-456",
                "displayName": "Bob External",
                "userPrincipalName": "bob_external#EXT#@contoso.onmicrosoft.com",
                "userType": "Guest"
            }));
        let client = make_client(mock);
        let user = client
            .graph()
            .get_user("ext-456")
            .await
            .expect("get_user failed")
            .expect("user should be found");
        assert_eq!(user.user_type.as_deref(), Some("Guest"));
        assert!(
            user.user_principal_name
                .as_deref()
                .unwrap_or("")
                .contains("#EXT#"),
            "guest UPN should contain #EXT#"
        );
    }

    #[tokio::test]
    async fn get_user_returns_none_for_404() {
        // Integration-proven: Graph 404 is returned as AzureError::NotFound (not a 200 with status).
        // get_user must match on NotFound and return Ok(None).
        let mut mock = MockClient::new();
        mock.expect_get("/v1.0/users/not-found")
            .returning_error(crate::AzureError::NotFound {
                resource: "User not-found".into(),
            });
        let client = make_client(mock);
        let result = client.graph().get_user("not-found").await;
        assert!(result.is_ok(), "NotFound should become Ok(None), not Err");
        assert!(result.unwrap().is_none(), "should return None for 404");
    }

    #[tokio::test]
    async fn batch_get_users_returns_map() {
        let mut mock = MockClient::new();
        mock.expect_post("/v1.0/$batch")
            .returning_json(serde_json::json!({
                "responses": [
                    {
                        "id": "0",
                        "status": 200,
                        "body": user_json("user-a", "Alice", "Member")
                    },
                    {
                        "id": "1",
                        "status": 200,
                        "body": user_json("user-b", "Bob", "Guest")
                    }
                ]
            }));
        let client = make_client(mock);
        let map = client
            .graph()
            .batch_get_users(&["user-a", "user-b"])
            .await
            .expect("batch_get_users failed");
        assert_eq!(map.len(), 2);
        assert_eq!(map["user-a"].user_type.as_deref(), Some("Member"));
        assert_eq!(map["user-b"].user_type.as_deref(), Some("Guest"));
    }

    #[tokio::test]
    async fn batch_get_users_omits_not_found() {
        let mut mock = MockClient::new();
        mock.expect_post("/v1.0/$batch")
            .returning_json(serde_json::json!({
                "responses": [
                    { "id": "0", "status": 200, "body": user_json("user-a", "Alice", "Member") },
                    { "id": "1", "status": 404, "body": null }
                ]
            }));
        let client = make_client(mock);
        let map = client
            .graph()
            .batch_get_users(&["user-a", "user-b"])
            .await
            .expect("batch_get_users failed");
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("user-a"));
        assert!(!map.contains_key("user-b"));
    }

    #[tokio::test]
    async fn list_service_principals_returns_results() {
        let mut mock = MockClient::new();
        mock.expect_get("/v1.0/servicePrincipals")
            .returning_json(serde_json::json!({
                "value": [
                    {
                        "id": "sp-001",
                        "appId": "app-001",
                        "displayName": "Azure Databricks SCIM Provisioning Connector",
                        "servicePrincipalType": "Application",
                        "accountEnabled": true
                    }
                ]
            }));
        let client = make_client(mock);
        let results = client
            .graph()
            .list_service_principals("displayName eq 'Azure Databricks SCIM Provisioning Connector'")
            .await
            .expect("list_service_principals failed");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id.as_deref(), Some("sp-001"));
        assert_eq!(
            results[0].display_name.as_deref(),
            Some("Azure Databricks SCIM Provisioning Connector")
        );
        assert_eq!(results[0].account_enabled, Some(true));
        assert_eq!(
            results[0].service_principal_type.as_deref(),
            Some("Application")
        );
    }

    #[tokio::test]
    async fn list_service_principals_returns_empty() {
        let mut mock = MockClient::new();
        mock.expect_get("/v1.0/servicePrincipals")
            .returning_json(serde_json::json!({ "value": [] }));
        let client = make_client(mock);
        let results = client
            .graph()
            .list_service_principals("displayName eq 'nonexistent'")
            .await
            .expect("list_service_principals failed");
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn batch_get_users_handles_chunking() {
        // 21 users → 2 batch calls (20 + 1)
        let ids: Vec<String> = (0..21).map(|i| format!("user-{i}")).collect();
        let id_refs: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();

        let first_batch_response = serde_json::json!({
            "responses": (0..20_usize).map(|i| serde_json::json!({
                "id": i.to_string(),
                "status": 200,
                "body": user_json(&format!("user-{i}"), &format!("User {i}"), "Member")
            })).collect::<Vec<_>>()
        });
        let second_batch_response = serde_json::json!({
            "responses": [
                { "id": "0", "status": 200, "body": user_json("user-20", "User 20", "Member") }
            ]
        });

        let mut mock = MockClient::new();
        mock.expect_post("/v1.0/$batch")
            .returning_json_sequence(vec![first_batch_response, second_batch_response])
            .times(2);

        let client = make_client(mock);
        let map = client
            .graph()
            .batch_get_users(&id_refs)
            .await
            .expect("batch_get_users failed");
        assert_eq!(map.len(), 21, "all 21 users should be in result");
    }
}
