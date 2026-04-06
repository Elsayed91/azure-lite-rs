//! Integration tests for Azure RBAC operations.
//!
//! These tests call real Azure ARM APIs — no resources are persisted
//! (role definition reads are read-only; the one role assignment we create
//! is deleted at the end of the test).
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//! (plus the service principal must have "Owner" or "User Access Administrator"
//!  on the subscription to create/delete role assignments).
//!
//! Run: cargo test -p azure-lite --test integration rbac_lifecycle -- --ignored --test-threads=1 --nocapture

use azure_lite::types::rbac::*;

// Reader built-in role definition GUID (stable across all Azure subscriptions)
const READER_ROLE_DEF_GUID: &str = "acdd72a7-3385-48ef-bd42-f606fba81ae7";
// Deterministic UUID used as the role assignment name (so cleanup is idempotent)
const TEST_ASSIGNMENT_NAME: &str = "c1a2b3c4-d5e6-7f80-9abc-def012345678";

// ============================================================================
// Helpers
// ============================================================================

/// Run `az` CLI, returning stdout.  Not-found errors return empty string.
async fn az(args: &[&str]) -> String {
    let output = tokio::process::Command::new("az")
        .args(args)
        .output()
        .await
        .expect("failed to run az CLI");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("ResourceNotFound")
            || stderr.contains("could not be found")
            || stderr.contains("does not exist")
            || stderr.contains("RoleAssignmentNotFound")
        {
            return String::new();
        }
        // Non-fatal: just log and continue
        eprintln!("az {} failed: {}", args.join(" "), stderr);
        return String::new();
    }
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Silently attempt to delete the test role assignment by its name/GUID.
async fn cleanup_test_assignment(subscription_id: &str) {
    az(&[
        "role", "assignment", "delete",
        "--ids",
        &format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleAssignments/{TEST_ASSIGNMENT_NAME}"
        ),
        "--yes",
    ]).await;
}

/// Get the object ID and type of the currently-signed-in principal (user or SP).
/// Returns `(principal_id, principal_type)` where type is "User" or "ServicePrincipal".
async fn get_current_principal() -> (String, &'static str) {
    // Try signed-in user first (interactive sessions)
    let uid = az(&["ad", "signed-in-user", "show", "--query", "id", "-o", "tsv"]).await;
    if !uid.is_empty() && !uid.contains("error") {
        return (uid, "User");
    }
    // Fall back to SP object ID via AZURE_CLIENT_ID env var
    let client_id = std::env::var("AZURE_CLIENT_ID").unwrap_or_default();
    if !client_id.is_empty() {
        let sp_id = az(&[
            "ad", "sp", "show", "--id", &client_id, "--query", "id", "-o", "tsv",
        ])
        .await;
        if !sp_id.is_empty() {
            return (sp_id, "ServicePrincipal");
        }
    }
    panic!(
        "Could not determine current principal ID. Ensure AZURE_CLIENT_ID is set or you are logged in with `az login`."
    );
}

// ============================================================================
// Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials"]
async fn rbac_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");
    let subscription_id = client.subscription_id().to_string();

    println!("[1/9] Pre-cleanup: removing any leftover test role assignment...");
    cleanup_test_assignment(&subscription_id).await;

    let sub_id_for_cleanup = subscription_id.clone();
    let result = std::panic::AssertUnwindSafe(async move {
        rbac_lifecycle_inner(&client, &subscription_id).await;
    });
    let outcome = tokio::task::spawn(result).await;

    println!("\n[9/9] Final cleanup: removing test role assignment...");
    cleanup_test_assignment(&sub_id_for_cleanup).await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn rbac_lifecycle_inner(client: &azure_lite::AzureHttpClient, subscription_id: &str) {
    let rbac = client.rbac();

    // =========================================================================
    // Step 2: List role definitions
    // =========================================================================
    println!("[2/9] Listing role definitions for subscription...");
    let role_defs = rbac
        .list_role_definitions()
        .await
        .expect("list_role_definitions failed");
    println!("  Found {} role definition(s).", role_defs.value.len());
    assert!(
        !role_defs.value.is_empty(),
        "Azure subscription should have built-in role definitions",
    );
    // Verify fields are populated on at least one definition
    let first = &role_defs.value[0];
    assert!(first.id.is_some(), "Role definition should have an ID");
    assert!(first.name.is_some(), "Role definition should have a name");
    println!(
        "  First role: id={:?}, name={:?}, role_name={:?}",
        first.id.as_deref(),
        first.name.as_deref(),
        first
            .properties
            .as_ref()
            .and_then(|p| p.role_name.as_deref()),
    );

    // =========================================================================
    // Step 3: Get "Reader" role definition by GUID
    // =========================================================================
    println!("[3/9] Getting 'Reader' built-in role definition...");
    let reader = rbac
        .get_role_definition(READER_ROLE_DEF_GUID)
        .await
        .expect("get_role_definition failed");
    println!(
        "  Reader role: name={:?}, role_name={:?}, permissions={}",
        reader.name.as_deref(),
        reader
            .properties
            .as_ref()
            .and_then(|p| p.role_name.as_deref()),
        reader
            .properties
            .as_ref()
            .map(|p| p.permissions.len())
            .unwrap_or(0),
    );
    assert_eq!(reader.name.as_deref(), Some(READER_ROLE_DEF_GUID));
    let props = reader
        .properties
        .as_ref()
        .expect("Reader should have properties");
    assert!(
        props
            .role_name
            .as_deref()
            .map(|n| n.to_lowercase().contains("reader"))
            .unwrap_or(false),
        "Role name should contain 'reader', got: {:?}",
        props.role_name,
    );
    assert!(
        !props.permissions.is_empty(),
        "Reader should have permissions"
    );
    assert!(
        props.permissions[0]
            .actions
            .iter()
            .any(|a| a == "*" || a.starts_with("*/read")),
        "Reader actions should contain read-all or wildcard",
    );

    // =========================================================================
    // Step 4: List role assignments
    // =========================================================================
    println!("[4/9] Listing role assignments for subscription...");
    let assignments = rbac
        .list_role_assignments()
        .await
        .expect("list_role_assignments failed");
    println!("  Found {} role assignment(s).", assignments.value.len());
    assert!(
        !assignments.value.is_empty(),
        "Azure subscription should have at least one role assignment",
    );
    // Spot-check the first assignment
    let first_assign = &assignments.value[0];
    assert!(
        first_assign.id.is_some(),
        "Role assignment should have an ID"
    );
    let a_props = first_assign
        .properties
        .as_ref()
        .expect("Role assignment should have properties");
    assert!(
        a_props.role_definition_id.is_some(),
        "Role assignment should have roleDefinitionId"
    );
    assert!(
        a_props.principal_id.is_some(),
        "Role assignment should have principalId"
    );

    // =========================================================================
    // Step 5: Get current principal ID + type for create/delete test
    // =========================================================================
    println!("[5/9] Getting current principal ID...");
    let (principal_id, principal_type) = get_current_principal().await;
    println!("  Principal ID: {principal_id}, type: {principal_type}");

    let full_role_def_id = format!(
        "/subscriptions/{subscription_id}/providers/Microsoft.Authorization/roleDefinitions/{READER_ROLE_DEF_GUID}"
    );

    // =========================================================================
    // Step 6: Create role assignment (Reader for current principal)
    // =========================================================================
    println!("[6/9] Creating test role assignment (Reader)...");
    let create_body = RoleAssignmentCreateRequest {
        properties: RoleAssignmentRequestProperties {
            role_definition_id: full_role_def_id.clone(),
            principal_id: principal_id.clone(),
            principal_type: Some(principal_type.into()),
            ..Default::default()
        },
    };
    let created = rbac
        .create_role_assignment(TEST_ASSIGNMENT_NAME, &create_body)
        .await
        .expect("create_role_assignment failed");
    println!(
        "  Created: name={:?}, role_def={:?}, principal={:?}",
        created.name.as_deref(),
        created
            .properties
            .as_ref()
            .and_then(|p| p.role_definition_id.as_deref()),
        created
            .properties
            .as_ref()
            .and_then(|p| p.principal_id.as_deref()),
    );
    assert_eq!(created.name.as_deref(), Some(TEST_ASSIGNMENT_NAME));
    assert!(
        created.id.is_some(),
        "Created role assignment should have an ARM ID"
    );
    let c_props = created
        .properties
        .as_ref()
        .expect("Created role assignment should have properties");
    assert_eq!(
        c_props.role_definition_id.as_deref(),
        Some(full_role_def_id.as_str())
    );
    assert_eq!(c_props.principal_id.as_deref(), Some(principal_id.as_str()));

    // =========================================================================
    // Step 7: Get role assignment we just created
    // =========================================================================
    println!("[7/9] Getting role assignment '{TEST_ASSIGNMENT_NAME}'...");
    let fetched = rbac
        .get_role_assignment(TEST_ASSIGNMENT_NAME)
        .await
        .expect("get_role_assignment failed");
    println!(
        "  Got: name={:?}, principal_type={:?}",
        fetched.name.as_deref(),
        fetched
            .properties
            .as_ref()
            .and_then(|p| p.principal_type.as_deref()),
    );
    assert_eq!(fetched.name.as_deref(), Some(TEST_ASSIGNMENT_NAME));

    // =========================================================================
    // Step 8: List role assignments — verify ours is present
    // =========================================================================
    println!("[8/9] Listing role assignments (post-create)...");
    let after_create = rbac
        .list_role_assignments()
        .await
        .expect("list_role_assignments failed");
    let found = after_create
        .value
        .iter()
        .any(|a| a.name.as_deref() == Some(TEST_ASSIGNMENT_NAME));
    println!(
        "  Total assignments: {}, ours present: {found}",
        after_create.value.len()
    );
    assert!(found, "Newly created assignment should appear in the list");

    // =========================================================================
    // Step 9 (inner): Delete role assignment
    // =========================================================================
    println!("  Deleting test role assignment...");
    rbac.delete_role_assignment(TEST_ASSIGNMENT_NAME)
        .await
        .expect("delete_role_assignment failed");
    println!("  Deleted.");

    // Verify it's gone
    let after_delete = rbac
        .list_role_assignments()
        .await
        .expect("list_role_assignments failed after delete");
    let still_there = after_delete
        .value
        .iter()
        .any(|a| a.name.as_deref() == Some(TEST_ASSIGNMENT_NAME));
    assert!(
        !still_there,
        "Deleted role assignment should not appear in list"
    );

    println!("\nAll RBAC integration tests passed!");
}
