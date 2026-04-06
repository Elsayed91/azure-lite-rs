//! Azure Managed Identities API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::identity::IdentityOps`. This layer adds:
//! - Ergonomic method signatures (auto-injects subscription_id from client)

use crate::{
    AzureHttpClient, Result,
    ops::identity::IdentityOps,
    types::identity::{
        SystemAssignedIdentity, UserAssignedIdentity, UserAssignedIdentityListResult,
        UserAssignedIdentityRequest,
    },
};

/// Client for the Azure Managed Identities API.
///
/// Wraps [`IdentityOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct IdentityClient<'a> {
    ops: IdentityOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> IdentityClient<'a> {
    /// Create a new Azure Managed Identities API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: IdentityOps::new(client),
            client,
        }
    }

    // --- User-Assigned Identity operations ---

    /// Lists all user-assigned identities available under the subscription.
    pub async fn list_user_assigned_identities(&self) -> Result<UserAssignedIdentityListResult> {
        self.ops
            .list_user_assigned_identities(self.client.subscription_id())
            .await
    }

    /// Lists all user-assigned identities within a resource group.
    pub async fn list_user_assigned_identities_in_group(
        &self,
        resource_group_name: &str,
    ) -> Result<UserAssignedIdentityListResult> {
        self.ops
            .list_user_assigned_identities_in_group(
                self.client.subscription_id(),
                resource_group_name,
            )
            .await
    }

    /// Gets a user-assigned identity.
    pub async fn get_identity(
        &self,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<UserAssignedIdentity> {
        self.ops
            .get_identity(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
            )
            .await
    }

    /// Creates or updates a user-assigned identity.
    pub async fn create_identity(
        &self,
        resource_group_name: &str,
        resource_name: &str,
        body: &UserAssignedIdentityRequest,
    ) -> Result<UserAssignedIdentity> {
        self.ops
            .create_identity(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
                body,
            )
            .await
    }

    /// Deletes a user-assigned identity.
    pub async fn delete_identity(
        &self,
        resource_group_name: &str,
        resource_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_identity(
                self.client.subscription_id(),
                resource_group_name,
                resource_name,
            )
            .await
    }

    // --- System-Assigned Identity operations ---

    /// Gets the system-assigned identity for an Azure resource (VM, App Service, etc.).
    ///
    /// `parent_resource_provider` e.g. `"Microsoft.Compute"`
    /// `parent_resource_type` e.g. `"virtualMachines"`
    /// `parent_resource_name` e.g. `"my-vm"`
    pub async fn get_system_assigned_identity(
        &self,
        resource_group_name: &str,
        parent_resource_provider: &str,
        parent_resource_type: &str,
        parent_resource_name: &str,
    ) -> Result<SystemAssignedIdentity> {
        self.ops
            .get_system_assigned_identity(
                self.client.subscription_id(),
                resource_group_name,
                parent_resource_provider,
                parent_resource_type,
                parent_resource_name,
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
    const IDENTITY_NAME: &str = "cloud-lite-test-ralph-identity";
    const PRINCIPAL_ID: &str = "3c9d25d5-26cf-4f4b-90e8-80cc050dc92b";
    const CLIENT_ID: &str = "da47ebbb-c00f-4c18-8c7d-14f2274094bf";
    const TENANT_ID: &str = "72f988bf-86f1-41af-91ab-2d7cd011db47";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn identity_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{IDENTITY_NAME}"),
            "name": IDENTITY_NAME,
            "type": "Microsoft.ManagedIdentity/userAssignedIdentities",
            "location": "eastus",
            "properties": {
                "tenantId": TENANT_ID,
                "principalId": PRINCIPAL_ID,
                "clientId": CLIENT_ID
            }
        })
    }

    #[tokio::test]
    async fn list_user_assigned_identities_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.ManagedIdentity/userAssignedIdentities"
        ))
        .returning_json(serde_json::json!({
            "value": [identity_json()]
        }));
        let client = make_client(mock);
        let result = client
            .identity()
            .list_user_assigned_identities()
            .await
            .expect("list_user_assigned_identities failed");
        assert_eq!(result.value.len(), 1);
        let i = &result.value[0];
        assert_eq!(i.name.as_deref(), Some(IDENTITY_NAME));
        let props = i.properties.as_ref().unwrap();
        assert_eq!(props.principal_id.as_deref(), Some(PRINCIPAL_ID));
        assert_eq!(props.client_id.as_deref(), Some(CLIENT_ID));
        assert_eq!(props.tenant_id.as_deref(), Some(TENANT_ID));
    }

    #[tokio::test]
    async fn list_user_assigned_identities_in_group_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ManagedIdentity/userAssignedIdentities"),
        )
        .returning_json(serde_json::json!({
            "value": [identity_json()]
        }));
        let client = make_client(mock);
        let result = client
            .identity()
            .list_user_assigned_identities_in_group(RG)
            .await
            .expect("list_user_assigned_identities_in_group failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some(IDENTITY_NAME));
    }

    #[tokio::test]
    async fn get_identity_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{IDENTITY_NAME}"),
        )
        .returning_json(identity_json());
        let client = make_client(mock);
        let i = client
            .identity()
            .get_identity(RG, IDENTITY_NAME)
            .await
            .expect("get_identity failed");
        assert_eq!(i.name.as_deref(), Some(IDENTITY_NAME));
        assert_eq!(i.location.as_deref(), Some("eastus"));
        assert_eq!(
            i.r#type.as_deref(),
            Some("Microsoft.ManagedIdentity/userAssignedIdentities")
        );
        let props = i.properties.as_ref().unwrap();
        assert_eq!(props.principal_id.as_deref(), Some(PRINCIPAL_ID));
        assert_eq!(props.client_id.as_deref(), Some(CLIENT_ID));
        assert_eq!(props.tenant_id.as_deref(), Some(TENANT_ID));
    }

    #[tokio::test]
    async fn create_identity_returns_identity() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{IDENTITY_NAME}"),
        )
        .returning_json(identity_json());
        let client = make_client(mock);
        use crate::types::identity::UserAssignedIdentityRequest;
        let body = UserAssignedIdentityRequest {
            location: "eastus".into(),
            ..Default::default()
        };
        let i = client
            .identity()
            .create_identity(RG, IDENTITY_NAME, &body)
            .await
            .expect("create_identity failed");
        assert_eq!(i.name.as_deref(), Some(IDENTITY_NAME));
        assert!(i.id.is_some());
        let props = i.properties.as_ref().unwrap();
        assert_eq!(props.principal_id.as_deref(), Some(PRINCIPAL_ID));
    }

    #[tokio::test]
    async fn delete_identity_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{IDENTITY_NAME}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .identity()
            .delete_identity(RG, IDENTITY_NAME)
            .await
            .expect("delete_identity failed");
    }

    #[tokio::test]
    async fn get_system_assigned_identity_constructs_url() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Compute/virtualMachines/my-vm/providers/Microsoft.ManagedIdentity/identities/default"),
        )
        .returning_json(serde_json::json!({
            "id": "/subscriptions/test-subscription-id/resourceGroups/test-rg/providers/Microsoft.Compute/virtualMachines/my-vm/providers/Microsoft.ManagedIdentity/identities/default",
            "name": "default",
            "type": "Microsoft.ManagedIdentity/identities",
            "location": "eastus",
            "properties": {
                "tenantId": TENANT_ID,
                "principalId": PRINCIPAL_ID,
                "clientId": CLIENT_ID,
                "clientSecretUrl": "https://control-eastus.identity.azure.net/subscriptions/test-subscription-id/resourcegroups/test-rg/providers/microsoft.compute/virtualmachines/my-vm/credentials?tid=72f988bf-86f1-41af-91ab-2d7cd011db47&oid=3c9d25d5-26cf-4f4b-90e8-80cc050dc92b&aid=da47ebbb-c00f-4c18-8c7d-14f2274094bf"
            }
        }));
        let client = make_client(mock);
        let sys_id = client
            .identity()
            .get_system_assigned_identity(RG, "Microsoft.Compute", "virtualMachines", "my-vm")
            .await
            .expect("get_system_assigned_identity failed");
        assert_eq!(sys_id.name.as_deref(), Some("default"));
        let props = sys_id.properties.as_ref().unwrap();
        assert_eq!(props.principal_id.as_deref(), Some(PRINCIPAL_ID));
        assert_eq!(props.client_id.as_deref(), Some(CLIENT_ID));
        assert!(props.client_secret_url.is_some());
    }
}
