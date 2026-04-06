//! Integration tests for Azure Subscriptions operations.
//!
//! Tests listing all subscriptions accessible to the authenticated principal.
//! This is a tenant-level call — no subscription ID in the path.
//!
//! Requires: Azure CLI login (`az login`)
//!
//! Run: cargo test -p azure-lite --test integration subscriptions -- --ignored --test-threads=1 --nocapture

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials"]
async fn subscriptions_list() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let subs_client = client.subscriptions();

    // =========================================================================
    // Step 1: List all subscriptions
    // =========================================================================
    println!("[1/2] Listing all subscriptions accessible to this principal...");
    let subs = subs_client
        .list()
        .await
        .expect("subscriptions().list() failed");

    println!("  Found {} subscription(s).", subs.len());
    assert!(
        !subs.is_empty(),
        "Expected at least one subscription to be returned"
    );

    for sub in &subs {
        println!(
            "    - {} | {} | state={} | tenant={}",
            sub.subscription_id.as_deref().unwrap_or("?"),
            sub.display_name.as_deref().unwrap_or("?"),
            sub.state.as_deref().unwrap_or("?"),
            sub.tenant_id.as_deref().unwrap_or("?"),
        );
    }

    // =========================================================================
    // Step 2: Validate fields are present
    // =========================================================================
    println!("[2/2] Validating subscription fields...");
    let first = &subs[0];
    assert!(
        first.subscription_id.as_deref().unwrap_or("").len() > 0,
        "subscriptionId should be non-empty"
    );
    assert!(
        first.display_name.as_deref().unwrap_or("").len() > 0,
        "displayName should be non-empty"
    );
    assert!(
        first.tenant_id.as_deref().unwrap_or("").len() > 0,
        "tenantId should be non-empty"
    );
    let state = first.state.as_deref().unwrap_or("");
    assert!(
        ["Enabled", "Disabled", "Warned", "Deleted", "PastDue"].contains(&state),
        "state '{}' should be a known value",
        state
    );

    println!("\nAll Azure Subscriptions integration tests passed!");
}
