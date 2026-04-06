//! MockClient helpers for Azure RBAC API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure RBAC helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait RbacMockHelpers {
    /// Helper to expect `list_role_definitions`: Lists all role definitions applicable at the
    /// subscription scope.
    fn expect_list_role_definitions(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_role_definition`: Gets a role definition by ID.
    fn expect_get_role_definition(
        &mut self,
        subscription_id: &str,
        role_definition_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_role_assignments`: Lists all role assignments for the subscription.
    fn expect_list_role_assignments(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_role_assignment`: Gets a role assignment by ID.
    fn expect_get_role_assignment(
        &mut self,
        subscription_id: &str,
        role_assignment_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_role_assignment`: Creates a role assignment at the subscription
    /// scope.
    fn expect_create_role_assignment(
        &mut self,
        subscription_id: &str,
        role_assignment_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_role_assignment`: Deletes a role assignment.
    fn expect_delete_role_assignment(
        &mut self,
        subscription_id: &str,
        role_assignment_id: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl RbacMockHelpers for MockClient {
    /// Helper to expect `list_role_definitions`: Lists all role definitions applicable at the
    /// subscription scope.
    fn expect_list_role_definitions(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleDefinitions"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_role_definition`: Gets a role definition by ID.
    fn expect_get_role_definition(
        &mut self,
        subscription_id: &str,
        role_definition_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleDefinitions/{role_definition_id}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_role_assignments`: Lists all role assignments for the subscription.
    fn expect_list_role_assignments(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleAssignments"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_role_assignment`: Gets a role assignment by ID.
    fn expect_get_role_assignment(
        &mut self,
        subscription_id: &str,
        role_assignment_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleAssignments/{role_assignment_id}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_role_assignment`: Creates a role assignment at the subscription
    /// scope.
    fn expect_create_role_assignment(
        &mut self,
        subscription_id: &str,
        role_assignment_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleAssignments/{role_assignment_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_role_assignment`: Deletes a role assignment.
    fn expect_delete_role_assignment(
        &mut self,
        subscription_id: &str,
        role_assignment_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleAssignments/{role_assignment_id}"
        );
        self.expect_delete(&path)
    }
}
