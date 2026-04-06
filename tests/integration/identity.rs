//! Integration tests for Azure Managed Identities operations.
//!
//! Creates a real user-assigned managed identity and verifies all CRUD operations.
//! The identity is deleted at the end of the test.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration identity_lifecycle -- --ignored --test-threads=1 --nocapture

use azure_lite::types::identity::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const IDENTITY_NAME: &str = "cloud-lite-test-ralph-identity";

// ============================================================================
// Helpers
// ============================================================================

async fn az_delete_ignore(args: &[&str]) {
    let output = tokio::process::Command::new("az")
        .args(args)
        .output()
        .await
        .expect("failed to run az CLI");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("ResourceNotFound")
            && !stderr.contains("could not be found")
            && !stderr.contains("ResourceGroupNotFound")
            && !stderr.contains("was not found")
        {
            eprintln!("Warning: cleanup az {} failed: {}", args.join(" "), stderr);
        }
    }
}

async fn cleanup_identity() {
    az_delete_ignore(&[
        "identity",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        IDENTITY_NAME,
    ])
    .await;
}

// ============================================================================
// Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn identity_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    println!("[1/8] Pre-cleanup: removing any leftover test identity...");
    cleanup_identity().await;

    let result = std::panic::AssertUnwindSafe(async move {
        identity_lifecycle_inner(&client).await;
    });
    let outcome = tokio::task::spawn(result).await;

    println!("\n[8/8] Final cleanup: removing test identity...");
    cleanup_identity().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn identity_lifecycle_inner(client: &azure_lite::AzureHttpClient) {
    let identity = client.identity();

    // =========================================================================
    // Step 2: Ensure resource group exists
    // =========================================================================
    println!("[2/8] Ensuring resource group '{RG_NAME}' exists...");
    tokio::process::Command::new("az")
        .args([
            "group",
            "create",
            "--name",
            RG_NAME,
            "--location",
            "eastus",
            "--output",
            "json",
        ])
        .output()
        .await
        .expect("failed to run az group create");
    println!("  Resource group ready.");

    // =========================================================================
    // Step 3: List identities before create (verify ours is absent)
    // =========================================================================
    println!("[3/8] Listing user-assigned identities in subscription (before create)...");
    let before_sub = identity
        .list_user_assigned_identities()
        .await
        .expect("list_user_assigned_identities failed");
    println!(
        "  Found {} identity(ies) subscription-wide.",
        before_sub.value.len()
    );
    assert!(
        !before_sub
            .value
            .iter()
            .any(|i| i.name.as_deref() == Some(IDENTITY_NAME)),
        "Test identity should not exist before create",
    );

    let before_rg = identity
        .list_user_assigned_identities_in_group(RG_NAME)
        .await
        .expect("list_user_assigned_identities_in_group failed");
    println!(
        "  Found {} identity(ies) in resource group.",
        before_rg.value.len()
    );

    // =========================================================================
    // Step 4: Create identity via library client
    // =========================================================================
    println!("[4/8] Creating user-assigned identity '{IDENTITY_NAME}'...");
    let create_req = UserAssignedIdentityRequest {
        location: LOCATION.into(),
        ..Default::default()
    };
    let created = identity
        .create_identity(RG_NAME, IDENTITY_NAME, &create_req)
        .await
        .expect("create_identity failed");
    println!(
        "  Created: name={:?}, location={:?}, principal_id={:?}, client_id={:?}",
        created.name.as_deref(),
        created.location.as_deref(),
        created
            .properties
            .as_ref()
            .and_then(|p| p.principal_id.as_deref()),
        created
            .properties
            .as_ref()
            .and_then(|p| p.client_id.as_deref()),
    );
    assert_eq!(created.name.as_deref(), Some(IDENTITY_NAME));
    assert!(
        created.id.is_some(),
        "Created identity should have an ARM resource ID"
    );
    let props = created
        .properties
        .as_ref()
        .expect("Created identity should have properties");
    assert!(
        props.principal_id.is_some(),
        "Identity should have a principalId"
    );
    assert!(props.client_id.is_some(), "Identity should have a clientId");
    assert!(props.tenant_id.is_some(), "Identity should have a tenantId");

    // =========================================================================
    // Step 5: Get identity
    // =========================================================================
    println!("[5/8] Getting identity '{IDENTITY_NAME}'...");
    let fetched = identity
        .get_identity(RG_NAME, IDENTITY_NAME)
        .await
        .expect("get_identity failed");
    println!(
        "  Got: name={:?}, location={:?}, type={:?}",
        fetched.name.as_deref(),
        fetched.location.as_deref(),
        fetched.r#type.as_deref(),
    );
    assert_eq!(fetched.name.as_deref(), Some(IDENTITY_NAME));
    assert_eq!(
        fetched
            .properties
            .as_ref()
            .and_then(|p| p.principal_id.as_deref()),
        props.principal_id.as_deref(),
        "principalId should match between create and get",
    );

    // =========================================================================
    // Step 6: List identities after create
    // =========================================================================
    println!("[6/8] Listing user-assigned identities in RG (after create)...");
    let after_rg = identity
        .list_user_assigned_identities_in_group(RG_NAME)
        .await
        .expect("list_user_assigned_identities_in_group failed");
    let found_in_rg = after_rg
        .value
        .iter()
        .any(|i| i.name.as_deref() == Some(IDENTITY_NAME));
    println!(
        "  Found {} identity(ies) in RG, ours present: {found_in_rg}",
        after_rg.value.len(),
    );
    assert!(found_in_rg, "Created identity should appear in RG list");

    let after_sub = identity
        .list_user_assigned_identities()
        .await
        .expect("list_user_assigned_identities failed");
    let found_sub = after_sub
        .value
        .iter()
        .any(|i| i.name.as_deref() == Some(IDENTITY_NAME));
    println!(
        "  Found {} identity(ies) subscription-wide, ours present: {found_sub}",
        after_sub.value.len(),
    );
    assert!(
        found_sub,
        "Created identity should appear in subscription list"
    );

    // =========================================================================
    // Step 7: Delete identity via library client
    // =========================================================================
    println!("[7/8] Deleting identity '{IDENTITY_NAME}'...");
    identity
        .delete_identity(RG_NAME, IDENTITY_NAME)
        .await
        .expect("delete_identity failed");
    println!("  Deleted.");

    // Verify deletion
    let after_delete = identity
        .list_user_assigned_identities_in_group(RG_NAME)
        .await
        .expect("list after delete failed");
    let still_there = after_delete
        .value
        .iter()
        .any(|i| i.name.as_deref() == Some(IDENTITY_NAME));
    assert!(
        !still_there,
        "Deleted identity should not appear in RG list"
    );
    println!("  Confirmed: identity no longer in list.");

    println!("\nAll Managed Identity integration tests passed!");
}
