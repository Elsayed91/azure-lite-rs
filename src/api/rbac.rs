//! Azure RBAC API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::rbac::RbacOps`. This layer adds:
//! - Ergonomic method signatures (auto-injects subscription_id from client)

use crate::{
    AzureHttpClient, Result,
    ops::rbac::RbacOps,
    types::rbac::{
        RoleAssignment, RoleAssignmentCreateRequest, RoleAssignmentListResult, RoleDefinition,
        RoleDefinitionListResult,
    },
};

/// Client for the Azure RBAC API.
///
/// Wraps the raw [`RbacOps`] with ergonomic signatures that
/// auto-inject `subscription_id` from the parent [`AzureHttpClient`].
pub struct RbacClient<'a> {
    ops: RbacOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> RbacClient<'a> {
    /// Create a new Azure RBAC API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: RbacOps::new(client),
            client,
        }
    }

    // --- Role Definition operations ---

    /// Lists all role definitions applicable at the subscription scope.
    pub async fn list_role_definitions(&self) -> Result<RoleDefinitionListResult> {
        self.ops
            .list_role_definitions(self.client.subscription_id())
            .await
    }

    /// Gets a role definition by ID.
    pub async fn get_role_definition(&self, role_definition_id: &str) -> Result<RoleDefinition> {
        self.ops
            .get_role_definition(self.client.subscription_id(), role_definition_id)
            .await
    }

    // --- Role Assignment operations ---

    /// Lists all role assignments for the subscription.
    pub async fn list_role_assignments(&self) -> Result<RoleAssignmentListResult> {
        self.ops
            .list_role_assignments(self.client.subscription_id())
            .await
    }

    /// Gets a role assignment by ID.
    pub async fn get_role_assignment(&self, role_assignment_id: &str) -> Result<RoleAssignment> {
        self.ops
            .get_role_assignment(self.client.subscription_id(), role_assignment_id)
            .await
    }

    /// Creates a role assignment at the subscription scope.
    pub async fn create_role_assignment(
        &self,
        role_assignment_name: &str,
        body: &RoleAssignmentCreateRequest,
    ) -> Result<RoleAssignment> {
        self.ops
            .create_role_assignment(self.client.subscription_id(), role_assignment_name, body)
            .await
    }

    /// Deletes a role assignment.
    pub async fn delete_role_assignment(&self, role_assignment_id: &str) -> Result<()> {
        self.ops
            .delete_role_assignment(self.client.subscription_id(), role_assignment_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const READER_GUID: &str = "acdd72a7-3385-48ef-bd42-f606fba81ae7";
    const ASSIGNMENT_NAME: &str = "c1a2b3c4-d5e6-7f80-9abc-def012345678";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    #[tokio::test]
    async fn list_role_definitions_returns_roles() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/providers/Microsoft.Authorization/roleDefinitions"),
        )
        .returning_json(serde_json::json!({
            "value": [
                {
                    "id": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleDefinitions/acdd72a7-3385-48ef-bd42-f606fba81ae7",
                    "name": "acdd72a7-3385-48ef-bd42-f606fba81ae7",
                    "type": "Microsoft.Authorization/roleDefinitions",
                    "properties": {
                        "roleName": "Reader",
                        "description": "View all resources",
                        "type": "BuiltInRole",
                        "permissions": [{"actions": ["*/read"], "notActions": []}],
                        "assignableScopes": ["/subscriptions/test-subscription-id"]
                    }
                }
            ]
        }));
        let client = make_client(mock);
        let rbac = client.rbac();
        let result = rbac
            .list_role_definitions()
            .await
            .expect("list_role_definitions failed");
        assert_eq!(result.value.len(), 1);
        let rd = &result.value[0];
        assert_eq!(rd.name.as_deref(), Some(READER_GUID));
        let props = rd.properties.as_ref().unwrap();
        assert_eq!(props.role_name.as_deref(), Some("Reader"));
        assert_eq!(props.r#type.as_deref(), Some("BuiltInRole"));
        assert_eq!(props.permissions[0].actions, vec!["*/read"]);
        assert_eq!(props.assignable_scopes.len(), 1);
    }

    #[tokio::test]
    async fn get_role_definition_deserializes_permissions() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/providers/Microsoft.Authorization/roleDefinitions/{READER_GUID}"),
        )
        .returning_json(serde_json::json!({
            "id": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleDefinitions/acdd72a7-3385-48ef-bd42-f606fba81ae7",
            "name": "acdd72a7-3385-48ef-bd42-f606fba81ae7",
            "type": "Microsoft.Authorization/roleDefinitions",
            "properties": {
                "roleName": "Reader",
                "permissions": [
                    {"actions": ["*/read"], "notActions": [], "dataActions": [], "notDataActions": []}
                ],
                "assignableScopes": ["/"]
            }
        }));
        let client = make_client(mock);
        let rd = client
            .rbac()
            .get_role_definition(READER_GUID)
            .await
            .expect("get_role_definition failed");
        assert_eq!(rd.name.as_deref(), Some(READER_GUID));
        let props = rd.properties.as_ref().unwrap();
        assert_eq!(props.role_name.as_deref(), Some("Reader"));
        assert_eq!(props.permissions.len(), 1);
        assert_eq!(props.permissions[0].actions, vec!["*/read"]);
        assert!(props.permissions[0].not_actions.is_empty());
    }

    #[tokio::test]
    async fn list_role_assignments_returns_assignments() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/providers/Microsoft.Authorization/roleAssignments"),
        )
        .returning_json(serde_json::json!({
            "value": [
                {
                    "id": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleAssignments/00000000-0000-0000-0000-000000000001",
                    "name": "00000000-0000-0000-0000-000000000001",
                    "type": "Microsoft.Authorization/roleAssignments",
                    "properties": {
                        "roleDefinitionId": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleDefinitions/acdd72a7-3385-48ef-bd42-f606fba81ae7",
                        "principalId": "fa8b3842-840e-40cb-ae98-86f7d631e6c8",
                        "principalType": "User",
                        "scope": "/subscriptions/test-subscription-id"
                    }
                }
            ]
        }));
        let client = make_client(mock);
        let result = client
            .rbac()
            .list_role_assignments()
            .await
            .expect("list_role_assignments failed");
        assert_eq!(result.value.len(), 1);
        let a = &result.value[0];
        assert_eq!(
            a.name.as_deref(),
            Some("00000000-0000-0000-0000-000000000001")
        );
        let props = a.properties.as_ref().unwrap();
        assert_eq!(props.principal_type.as_deref(), Some("User"));
        assert!(props.role_definition_id.is_some());
    }

    #[tokio::test]
    async fn get_role_assignment_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/providers/Microsoft.Authorization/roleAssignments/{ASSIGNMENT_NAME}"),
        )
        .returning_json(serde_json::json!({
            "id": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleAssignments/c1a2b3c4-d5e6-7f80-9abc-def012345678",
            "name": "c1a2b3c4-d5e6-7f80-9abc-def012345678",
            "type": "Microsoft.Authorization/roleAssignments",
            "properties": {
                "roleDefinitionId": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleDefinitions/acdd72a7-3385-48ef-bd42-f606fba81ae7",
                "principalId": "fa8b3842-840e-40cb-ae98-86f7d631e6c8",
                "principalType": "User",
                "scope": "/subscriptions/test-subscription-id"
            }
        }));
        let client = make_client(mock);
        let a = client
            .rbac()
            .get_role_assignment(ASSIGNMENT_NAME)
            .await
            .expect("get_role_assignment failed");
        assert_eq!(a.name.as_deref(), Some(ASSIGNMENT_NAME));
        let props = a.properties.as_ref().unwrap();
        assert_eq!(
            props.principal_id.as_deref(),
            Some("fa8b3842-840e-40cb-ae98-86f7d631e6c8")
        );
        assert_eq!(props.principal_type.as_deref(), Some("User"));
        assert!(props.scope.is_some());
    }

    #[tokio::test]
    async fn create_role_assignment_returns_assignment() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/providers/Microsoft.Authorization/roleAssignments/{ASSIGNMENT_NAME}"),
        )
        .returning_json(serde_json::json!({
            "id": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleAssignments/c1a2b3c4-d5e6-7f80-9abc-def012345678",
            "name": "c1a2b3c4-d5e6-7f80-9abc-def012345678",
            "type": "Microsoft.Authorization/roleAssignments",
            "properties": {
                "roleDefinitionId": "/subscriptions/test-subscription-id/providers/Microsoft.Authorization/roleDefinitions/acdd72a7-3385-48ef-bd42-f606fba81ae7",
                "principalId": "fa8b3842-840e-40cb-ae98-86f7d631e6c8",
                "principalType": "User",
                "scope": "/subscriptions/test-subscription-id"
            }
        }));
        let client = make_client(mock);
        use crate::types::rbac::{RoleAssignmentCreateRequest, RoleAssignmentRequestProperties};
        let body = RoleAssignmentCreateRequest {
            properties: RoleAssignmentRequestProperties {
                role_definition_id: format!(
                    "/subscriptions/{SUB_ID}/providers/Microsoft.Authorization/roleDefinitions/{READER_GUID}"
                ),
                principal_id: "fa8b3842-840e-40cb-ae98-86f7d631e6c8".into(),
                principal_type: Some("User".into()),
                ..Default::default()
            },
        };
        let a = client
            .rbac()
            .create_role_assignment(ASSIGNMENT_NAME, &body)
            .await
            .expect("create_role_assignment failed");
        assert_eq!(a.name.as_deref(), Some(ASSIGNMENT_NAME));
        assert!(a.id.is_some());
        let props = a.properties.as_ref().unwrap();
        assert_eq!(
            props.principal_id.as_deref(),
            Some("fa8b3842-840e-40cb-ae98-86f7d631e6c8")
        );
    }

    #[tokio::test]
    async fn delete_role_assignment_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/providers/Microsoft.Authorization/roleAssignments/{ASSIGNMENT_NAME}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .rbac()
            .delete_role_assignment(ASSIGNMENT_NAME)
            .await
            .expect("delete_role_assignment failed");
    }
}
