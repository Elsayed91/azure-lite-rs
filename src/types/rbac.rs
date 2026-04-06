//! Types for the Azure RBAC API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Role definition permissions.
///
/// **Azure API**: `rbac.v1.Permission`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//Permission>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    /// Allowed actions
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,

    /// Denied actions
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,

    /// Allowed data actions
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,

    /// Denied data actions
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}

impl Permission {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            actions: vec![],
            not_actions: vec![],
            data_actions: vec![],
            not_data_actions: vec![],
        }
    }
}

/// Role definition properties.
///
/// **Azure API**: `rbac.v1.RoleDefinitionProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleDefinitionProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleDefinitionProperties {
    /// The role name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,

    /// The role definition description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The role type (BuiltInRole or CustomRole)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Role definition permissions
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,

    /// Role definition assignable scopes
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assignable_scopes: Vec<String>,
}

impl RoleDefinitionProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_name: Some("test-role_name".into()),
            description: Some("test-description".into()),
            r#type: Some("test-type".into()),
            permissions: vec![],
            assignable_scopes: vec![],
        }
    }
}

/// Role definition.
///
/// **Azure API**: `rbac.v1.RoleDefinition`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleDefinition>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleDefinition {
    /// The role definition ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The role definition name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The role definition type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Role definition properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleDefinitionProperties>,
}

impl RoleDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-role_definition".into()),
            r#type: Some("test-type".into()),
            properties: Some(RoleDefinitionProperties::fixture()),
        }
    }
}

/// Role definition list operation result.
///
/// **Azure API**: `rbac.v1.RoleDefinitionListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleDefinitionListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleDefinitionListResult {
    /// Role definition list
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleDefinition>,

    /// The URL to use for getting the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl RoleDefinitionListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Role assignment properties.
///
/// **Azure API**: `rbac.v1.RoleAssignmentProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleAssignmentProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignmentProperties {
    /// The role definition ID used in the role assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,

    /// The principal ID assigned to the role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,

    /// The principal type of the assigned principal ID (User, Group, ServicePrincipal, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,

    /// The role assignment scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// The description of role assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The conditions on the role assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// Version of the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,

    /// Time it was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,

    /// Time it was updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on: Option<String>,

    /// ID of the user who created the assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    /// ID of the user who updated the assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

impl RoleAssignmentProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_definition_id: Some("test-role_definition_id".into()),
            principal_id: Some("test-principal_id".into()),
            principal_type: Some("test-principal_type".into()),
            scope: Some("test-scope".into()),
            description: Some("test-description".into()),
            condition: Some("test-condition".into()),
            condition_version: Some("test-condition_version".into()),
            created_on: Some("test-created_on".into()),
            updated_on: Some("test-updated_on".into()),
            created_by: Some("test-created_by".into()),
            updated_by: Some("test-updated_by".into()),
        }
    }
}

/// Role assignments.
///
/// **Azure API**: `rbac.v1.RoleAssignment`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleAssignment>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignment {
    /// The role assignment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The role assignment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The role assignment type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Role assignment properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentProperties>,
}

impl RoleAssignment {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-role_assignment".into()),
            r#type: Some("test-type".into()),
            properties: Some(RoleAssignmentProperties::fixture()),
        }
    }
}

/// Role assignment list operation result.
///
/// **Azure API**: `rbac.v1.RoleAssignmentListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleAssignmentListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignmentListResult {
    /// Role assignment list
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignment>,

    /// The URL to use for getting the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl RoleAssignmentListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Role assignment request properties.
///
/// **Azure API**: `rbac.v1.RoleAssignmentRequestProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleAssignmentRequestProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignmentRequestProperties {
    /// The role definition ID used in the role assignment
    pub role_definition_id: String,

    /// The principal ID assigned to the role
    pub principal_id: String,

    /// The principal type of the assigned principal ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,

    /// The description of role assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The conditions on the role assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// Version of the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
}

impl RoleAssignmentRequestProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            role_definition_id: "test-role_definition_id".into(),
            principal_id: "test-principal_id".into(),
            principal_type: Some("test-principal_type".into()),
            description: Some("test-description".into()),
            condition: Some("test-condition".into()),
            condition_version: Some("test-condition_version".into()),
        }
    }
}

/// Role assignment create parameters.
///
/// **Azure API**: `rbac.v1.RoleAssignmentCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/authorization//RoleAssignmentCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAssignmentCreateRequest {
    /// Role assignment properties
    pub properties: RoleAssignmentRequestProperties,
}

impl RoleAssignmentCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            properties: RoleAssignmentRequestProperties::fixture(),
        }
    }
}
