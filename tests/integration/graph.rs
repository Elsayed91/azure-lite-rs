//! Integration tests for Microsoft Graph user lookups.
//!
//! Tests that Graph-scoped tokens are acquired and user objects are
//! correctly deserialized, including `userType` (Member/Guest).
//!
//! Requires: Azure CLI login (or service principal env vars) + AZURE_SUBSCRIPTION_ID
//! The test principal must have at least one RBAC role assignment in the subscription
//! so we can get a real principalId to look up.
//!
//! Run: cargo test -p azure-lite --test integration graph -- --ignored --test-threads=1 --nocapture

// ============================================================================
// Helpers
// ============================================================================

async fn az(args: &[&str]) -> String {
    let output = tokio::process::Command::new("az")
        .args(args)
        .output()
        .await
        .expect("failed to run az CLI");
    if !output.status.success() {
        return String::new();
    }
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Get the object ID of the currently signed-in user or service principal.
async fn get_current_user_id() -> Option<String> {
    // Try signed-in user first (interactive sessions)
    let uid = az(&["ad", "signed-in-user", "show", "--query", "id", "-o", "tsv"]).await;
    if !uid.is_empty() && !uid.contains("error") {
        return Some(uid);
    }
    None
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and Graph API access"]
async fn graph_get_user_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");
    let graph = client.graph();

    // =========================================================================
    // Step 1: Get the current user's object ID via the CLI
    // =========================================================================
    println!("[1/5] Resolving current user's object ID via az CLI...");
    let current_user_id = get_current_user_id()
        .await
        .expect("Could not resolve current user ID. Run 'az login' to authenticate.");
    println!("  Current user ID: {current_user_id}");

    // =========================================================================
    // Step 2: get_user — single user lookup
    // =========================================================================
    println!("[2/5] Getting user object for current user...");
    let user = graph
        .get_user(&current_user_id)
        .await
        .expect("get_user failed")
        .expect("current user should be found");

    println!(
        "  id={:?}, displayName={:?}, userPrincipalName={:?}, userType={:?}",
        user.id.as_deref(),
        user.display_name.as_deref(),
        user.user_principal_name.as_deref(),
        user.user_type.as_deref(),
    );

    assert_eq!(
        user.id.as_deref(),
        Some(current_user_id.as_str()),
        "returned id should match requested principal ID"
    );
    assert!(
        user.display_name.is_some(),
        "user should have a displayName"
    );
    assert!(
        user.user_principal_name.is_some(),
        "user should have a userPrincipalName"
    );
    assert!(
        user.user_type.as_deref() == Some("Member") || user.user_type.as_deref() == Some("Guest"),
        "userType should be 'Member' or 'Guest', got: {:?}",
        user.user_type,
    );
    println!("  ✓ userType = {:?}", user.user_type.as_deref());

    // =========================================================================
    // Step 3: get_user with non-existent ID → None
    // =========================================================================
    println!("[3/5] Getting user with a non-existent ID (expect None)...");
    let not_found = graph
        .get_user("00000000-0000-0000-0000-000000000000")
        .await
        .expect("get_user should not error on 404");
    println!(
        "  Result: {}",
        if not_found.is_none() {
            "None (correct)"
        } else {
            "Some (unexpected)"
        }
    );
    assert!(
        not_found.is_none(),
        "all-zeros UUID should not resolve to a user"
    );

    // =========================================================================
    // Step 4: batch_get_users — single-user batch
    // =========================================================================
    println!("[4/5] Batch-fetching current user via $batch...");
    let map = graph
        .batch_get_users(&[&current_user_id])
        .await
        .expect("batch_get_users failed");

    println!("  Batch result: {} user(s) returned", map.len());
    assert_eq!(map.len(), 1, "batch should return exactly 1 user");
    let batch_user = map
        .get(&current_user_id)
        .expect("user should be in batch map");
    assert_eq!(
        batch_user.user_type.as_deref(),
        user.user_type.as_deref(),
        "batch userType should match single-get userType"
    );
    println!("  ✓ Batch user matches single-get result");

    // =========================================================================
    // Step 5: batch_get_users — mix of valid and not-found IDs
    // =========================================================================
    println!("[5/5] Batch-fetching with a mix of valid and not-found IDs...");
    let invalid_id = "00000000-0000-0000-0000-000000000001";
    let mixed_map = graph
        .batch_get_users(&[&current_user_id, invalid_id])
        .await
        .expect("batch_get_users with mixed IDs failed");

    println!(
        "  Mixed batch: {} user(s) returned (expected 1)",
        mixed_map.len()
    );
    assert_eq!(
        mixed_map.len(),
        1,
        "only the valid user should be in the map"
    );
    assert!(
        mixed_map.contains_key(&current_user_id),
        "valid user should be present"
    );
    assert!(
        !mixed_map.contains_key(invalid_id),
        "not-found user should be absent"
    );

    println!("\nAll Microsoft Graph user integration tests passed!");
}

#[tokio::test]
#[ignore = "requires Azure credentials and Graph API access"]
async fn graph_list_service_principals() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");
    let graph = client.graph();

    // =========================================================================
    // Step 1: List service principals with a known display name filter
    // =========================================================================
    println!("[1/3] Listing service principals with filter for Databricks SCIM connector...");
    let results = graph
        .list_service_principals(
            "displayName eq 'Azure Databricks SCIM Provisioning Connector'",
        )
        .await
        .expect("list_service_principals failed");
    println!("  Found {} service principal(s)", results.len());
    for sp in &results {
        println!(
            "  id={:?}, appId={:?}, displayName={:?}, type={:?}, enabled={:?}",
            sp.id.as_deref(),
            sp.app_id.as_deref(),
            sp.display_name.as_deref(),
            sp.service_principal_type.as_deref(),
            sp.account_enabled,
        );
    }

    // =========================================================================
    // Step 2: List with a filter that should return no results
    // =========================================================================
    println!("[2/3] Listing service principals with a non-existent name...");
    let empty = graph
        .list_service_principals("displayName eq 'cloud-lite-nonexistent-sp-test-12345'")
        .await
        .expect("list_service_principals failed for non-existent name");
    println!("  Found {} service principal(s) (expected 0)", empty.len());
    assert!(
        empty.is_empty(),
        "non-existent display name should return empty list"
    );

    // =========================================================================
    // Step 3: List with a startsWith filter to exercise partial matching
    // =========================================================================
    println!("[3/3] Listing service principals with startsWith filter...");
    let starts_with = graph
        .list_service_principals("startswith(displayName, 'Azure Databricks')")
        .await
        .expect("list_service_principals with startsWith failed");
    println!(
        "  Found {} service principal(s) matching startsWith('Azure Databricks')",
        starts_with.len()
    );
    for sp in &starts_with {
        println!("    - {:?}", sp.display_name.as_deref());
    }

    println!("\nAll Microsoft Graph service principal integration tests passed!");
}
