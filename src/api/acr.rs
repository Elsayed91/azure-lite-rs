//! Azure Container Registry (ACR) API client.
//!
//! Two planes:
//! - ARM management plane (`management.azure.com`): registry CRUD via
//!   generated `ops::acr::AcrOps`.
//! - ACR data plane (`https://{loginServer}`): repository and tag operations
//!   using the OCI Distribution v2 and ACR-specific REST APIs.
//!
//! Both planes accept the same Azure AD bearer token.
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::acr::AcrOps,
    types::acr::{
        CatalogResult, Registry, RegistryCreateRequest, RegistryListResult, Repository,
        TagListResult,
    },
};

/// Client for the Azure Container Registry ARM management plane and data plane.
///
/// Wraps [`AcrOps`] for ARM operations and makes direct HTTP calls to the
/// ACR data plane (`https://{loginServer}`) for repository and tag operations.
pub struct AcrClient<'a> {
    ops: AcrOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> AcrClient<'a> {
    /// Create a new Azure Container Registry API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: AcrOps::new(client),
            client,
        }
    }

    // --- Registry (ARM) operations ---

    /// Lists all container registries in the subscription.
    pub async fn list_registries(&self) -> Result<RegistryListResult> {
        self.ops
            .list_registries(self.client.subscription_id())
            .await
    }

    /// Gets the properties of a container registry.
    pub async fn get_registry(
        &self,
        resource_group_name: &str,
        registry_name: &str,
    ) -> Result<Registry> {
        self.ops
            .get_registry(
                self.client.subscription_id(),
                resource_group_name,
                registry_name,
            )
            .await
    }

    /// Creates or updates a container registry.
    pub async fn create_registry(
        &self,
        resource_group_name: &str,
        registry_name: &str,
        body: &RegistryCreateRequest,
    ) -> Result<Registry> {
        self.ops
            .create_registry(
                self.client.subscription_id(),
                resource_group_name,
                registry_name,
                body,
            )
            .await
    }

    /// Deletes a container registry.
    pub async fn delete_registry(
        &self,
        resource_group_name: &str,
        registry_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_registry(
                self.client.subscription_id(),
                resource_group_name,
                registry_name,
            )
            .await
    }

    // --- Repository (data plane) operations ---
    //
    // NOTE: ACR data plane endpoints require ACR-specific OAuth tokens obtained
    // via the token exchange flow: POST {loginServer}/oauth2/exchange → refresh_token,
    // then POST {loginServer}/oauth2/token → access_token. ARM bearer tokens are
    // not accepted by the ACR data plane and will result in 401 Unauthorized.

    /// Lists all repositories in a registry (OCI v2 catalog API).
    ///
    /// Uses `https://{loginServer}/v2/_catalog`.
    ///
    /// **Note**: Requires an ACR-scoped token via the ACR OAuth exchange flow.
    pub async fn list_repositories(&self, login_server: &str) -> Result<CatalogResult> {
        let url = format!("https://{login_server}/v2/_catalog");
        let resp = self.client.get(&url).await?;
        let body = resp.bytes().await?;
        serde_json::from_slice(&body).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse CatalogResult: {e}"),
            body: None,
        })
    }

    /// Gets ACR-specific metadata for a repository.
    ///
    /// Uses `https://{loginServer}/acr/v1/{name}`.
    pub async fn get_repository(
        &self,
        login_server: &str,
        repository_name: &str,
    ) -> Result<Repository> {
        let url = format!("https://{login_server}/acr/v1/{repository_name}");
        let resp = self.client.get(&url).await?;
        let body = resp.bytes().await?;
        serde_json::from_slice(&body).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse Repository: {e}"),
            body: None,
        })
    }

    /// Deletes a repository and all its tags and manifests.
    ///
    /// Uses `DELETE https://{loginServer}/acr/v1/{name}`.
    pub async fn delete_repository(&self, login_server: &str, repository_name: &str) -> Result<()> {
        let url = format!("https://{login_server}/acr/v1/{repository_name}");
        self.client.delete(&url).await?;
        Ok(())
    }

    // --- Tag (data plane) operations ---

    /// Lists all tags for a repository (OCI v2 tag list API).
    ///
    /// Uses `https://{loginServer}/v2/{name}/tags/list`.
    pub async fn list_tags(
        &self,
        login_server: &str,
        repository_name: &str,
    ) -> Result<TagListResult> {
        let url = format!("https://{login_server}/v2/{repository_name}/tags/list");
        let resp = self.client.get(&url).await?;
        let body = resp.bytes().await?;
        serde_json::from_slice(&body).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse TagListResult: {e}"),
            body: None,
        })
    }

    /// Deletes a specific tag from a repository.
    ///
    /// Uses `DELETE https://{loginServer}/acr/v1/{name}/_tags/{tag}`.
    pub async fn delete_tag(
        &self,
        login_server: &str,
        repository_name: &str,
        tag: &str,
    ) -> Result<()> {
        let url = format!("https://{login_server}/acr/v1/{repository_name}/_tags/{tag}");
        self.client.delete(&url).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const REGISTRY: &str = "testregistry";
    const LOGIN_SERVER: &str = "testregistry.azurecr.io";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn registry_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerRegistry/registries/{REGISTRY}"),
            "name": REGISTRY,
            "type": "Microsoft.ContainerRegistry/registries",
            "location": "eastus",
            "sku": { "name": "Basic", "tier": "Basic" },
            "properties": {
                "loginServer": LOGIN_SERVER,
                "creationDate": "2026-02-19T07:00:00Z",
                "provisioningState": "Succeeded",
                "adminUserEnabled": false
            }
        })
    }

    #[tokio::test]
    async fn list_registries_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.ContainerRegistry/registries"
        ))
        .returning_json(serde_json::json!({ "value": [registry_json()] }));
        let client = make_client(mock);
        let result = client
            .acr()
            .list_registries()
            .await
            .expect("list_registries failed");
        assert_eq!(result.value.len(), 1);
        let r = &result.value[0];
        assert_eq!(r.name.as_deref(), Some(REGISTRY));
        let props = r.properties.as_ref().unwrap();
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
        assert_eq!(props.login_server.as_deref(), Some(LOGIN_SERVER));
    }

    #[tokio::test]
    async fn get_registry_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerRegistry/registries/{REGISTRY}"),
        )
        .returning_json(registry_json());
        let client = make_client(mock);
        let r = client
            .acr()
            .get_registry(RG, REGISTRY)
            .await
            .expect("get_registry failed");
        assert_eq!(r.name.as_deref(), Some(REGISTRY));
        let sku = r.sku.as_ref().unwrap();
        assert_eq!(sku.name, "Basic");
        let props = r.properties.as_ref().unwrap();
        assert_eq!(props.admin_user_enabled, Some(false));
        assert_eq!(props.login_server.as_deref(), Some(LOGIN_SERVER));
    }

    #[tokio::test]
    async fn create_registry_sends_body() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerRegistry/registries/{REGISTRY}"),
        )
        .returning_json(registry_json());
        let client = make_client(mock);
        let body = RegistryCreateRequest {
            location: "eastus".into(),
            sku: Some(crate::types::acr::RegistrySku {
                name: "Basic".into(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let r = client
            .acr()
            .create_registry(RG, REGISTRY, &body)
            .await
            .expect("create_registry failed");
        assert_eq!(r.name.as_deref(), Some(REGISTRY));
    }

    #[tokio::test]
    async fn delete_registry_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ContainerRegistry/registries/{REGISTRY}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .acr()
            .delete_registry(RG, REGISTRY)
            .await
            .expect("delete_registry failed");
    }

    #[tokio::test]
    async fn list_repositories_deserializes_catalog() {
        let mut mock = MockClient::new();
        // Mock strips the host and matches path only
        mock.expect_get("/v2/_catalog")
            .returning_json(serde_json::json!({ "repositories": ["hello-world", "myapp"] }));
        let client = make_client(mock);
        let result = client
            .acr()
            .list_repositories(LOGIN_SERVER)
            .await
            .expect("list_repositories failed");
        assert_eq!(result.repositories.len(), 2);
        assert!(result.repositories.contains(&"hello-world".to_string()));
    }

    #[tokio::test]
    async fn get_repository_deserializes_name() {
        let mut mock = MockClient::new();
        mock.expect_get("/acr/v1/hello-world")
            .returning_json(serde_json::json!({
                "name": "hello-world",
                "changeableAttributes": {
                    "deleteEnabled": true,
                    "writeEnabled": true,
                    "listEnabled": true,
                    "readEnabled": true
                }
            }));
        let client = make_client(mock);
        let repo = client
            .acr()
            .get_repository(LOGIN_SERVER, "hello-world")
            .await
            .expect("get_repository failed");
        assert_eq!(repo.name.as_deref(), Some("hello-world"));
        let attrs = repo.changeable_attributes.as_ref().unwrap();
        assert_eq!(attrs.delete_enabled, Some(true));
    }

    #[tokio::test]
    async fn list_tags_deserializes_tags() {
        let mut mock = MockClient::new();
        mock.expect_get("/v2/hello-world/tags/list")
            .returning_json(serde_json::json!({
                "name": "hello-world",
                "tags": ["latest", "v1.0", "v2.0"]
            }));
        let client = make_client(mock);
        let result = client
            .acr()
            .list_tags(LOGIN_SERVER, "hello-world")
            .await
            .expect("list_tags failed");
        assert_eq!(result.name.as_deref(), Some("hello-world"));
        assert_eq!(result.tags.len(), 3);
        assert!(result.tags.contains(&"latest".to_string()));
    }

    #[tokio::test]
    async fn delete_repository_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete("/acr/v1/hello-world")
            .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .acr()
            .delete_repository(LOGIN_SERVER, "hello-world")
            .await
            .expect("delete_repository failed");
    }

    #[tokio::test]
    async fn delete_tag_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete("/acr/v1/hello-world/_tags/latest")
            .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .acr()
            .delete_tag(LOGIN_SERVER, "hello-world", "latest")
            .await
            .expect("delete_tag failed");
    }
}
