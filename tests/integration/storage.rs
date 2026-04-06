//! Integration tests for Azure Storage management operations.
//!
//! These tests create real Azure resources and MUST clean up after themselves.
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID env var.
//!
//! Run: cargo test -p azure-lite --test integration storage -- --ignored --test-threads=1 --nocapture

use azure_lite::types::storage::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
// Storage account names must be 3-24 chars, lowercase alphanumeric only
const STORAGE_ACCOUNT_NAME: &str = "cltralphstorage";

/// Helper: run az CLI command, return stdout. Panics on failure unless it's a "not found" error.
async fn az(args: &[&str]) -> String {
    let output = tokio::process::Command::new("az")
        .args(args)
        .output()
        .await
        .expect("failed to run az CLI");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("ResourceNotFound")
            || stderr.contains("ResourceGroupNotFound")
            || stderr.contains("could not be found")
            || stderr.contains("StorageAccountNotFound")
        {
            return String::new();
        }
        panic!("az {} failed: {}", args.join(" "), stderr);
    }
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Clean up the test storage account (ignore errors if already gone).
async fn storage_cleanup(subscription_id: &str) {
    println!("  Cleaning up test storage account...");
    az(&[
        "storage",
        "account",
        "delete",
        "--subscription",
        subscription_id,
        "--resource-group",
        RG_NAME,
        "--name",
        STORAGE_ACCOUNT_NAME,
        "--yes",
    ])
    .await;
}

/// Ensure the test resource group exists.
async fn ensure_rg(subscription_id: &str) {
    az(&[
        "group",
        "create",
        "--subscription",
        subscription_id,
        "--name",
        RG_NAME,
        "--location",
        LOCATION,
        "--output",
        "none",
    ])
    .await;
}

#[tokio::test]
#[ignore]
async fn storage_account_lifecycle() {
    let subscription_id =
        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID must be set");

    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to create AzureHttpClient");

    // ---- Pre-cleanup: remove any leftover from previous test run ----
    println!("[pre] Cleaning up any leftover resources...");
    storage_cleanup(&subscription_id).await;

    // Always-cleanup wrapper
    let result = run_storage_lifecycle(&client, &subscription_id).await;

    println!("[cleanup] Cleaning up test resources...");
    storage_cleanup(&subscription_id).await;

    result.expect("Storage lifecycle test failed");
}

async fn run_storage_lifecycle(
    client: &azure_lite::AzureHttpClient,
    subscription_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let storage = client.storage();

    // Step 1: Ensure resource group exists
    println!("[1/10] Ensuring resource group '{RG_NAME}' exists...");
    ensure_rg(subscription_id).await;

    // Step 2: List storage accounts in subscription (may be empty)
    println!("[2/10] Listing storage accounts in subscription...");
    let list_result = storage
        .list_storage_accounts(subscription_id)
        .await
        .expect("list_storage_accounts failed");
    println!(
        "  Found {} accounts in subscription",
        list_result.value.len()
    );

    // Step 3: List storage accounts by resource group (should be empty before creation)
    println!("[3/10] Listing storage accounts in resource group '{RG_NAME}'...");
    let rg_list = storage
        .list_storage_accounts_by_resource_group(subscription_id, RG_NAME)
        .await
        .expect("list_storage_accounts_by_resource_group failed");
    let initial_count = rg_list.value.len();
    println!("  Found {initial_count} accounts in resource group before creation");

    // Step 4: Create storage account
    println!("[4/10] Creating storage account '{STORAGE_ACCOUNT_NAME}'...");
    let create_body = StorageAccountCreateRequest {
        location: LOCATION.to_string(),
        kind: "StorageV2".to_string(),
        sku: serde_json::json!({ "name": "Standard_LRS" }),
        properties: Some(serde_json::json!({
            "minimumTlsVersion": "TLS1_2",
            "supportsHttpsTrafficOnly": true,
            "allowBlobPublicAccess": false,
        })),
        ..Default::default()
    };
    let created = storage
        .create_storage_account(subscription_id, RG_NAME, STORAGE_ACCOUNT_NAME, &create_body)
        .await
        .expect("create_storage_account failed");

    let account_name = created.name.as_deref().unwrap_or("");
    println!(
        "  Created: name={account_name}, location={}",
        created.location
    );
    assert_eq!(account_name, STORAGE_ACCOUNT_NAME, "account name mismatch");
    assert_eq!(created.location, LOCATION, "location mismatch");

    // Extract provisioning state from the `properties` JSON field
    let provisioning_state = created
        .properties
        .as_ref()
        .and_then(|p| p.get("provisioningState"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    println!("  provisioningState={provisioning_state}");
    assert!(
        provisioning_state == "Succeeded" || provisioning_state == "Creating",
        "unexpected provisioning state: {provisioning_state}"
    );

    // Step 5: Get storage account
    println!("[5/10] Getting storage account '{STORAGE_ACCOUNT_NAME}'...");
    let got = storage
        .get_storage_account(subscription_id, RG_NAME, STORAGE_ACCOUNT_NAME)
        .await
        .expect("get_storage_account failed");
    assert_eq!(got.name.as_deref(), Some(STORAGE_ACCOUNT_NAME));
    assert_eq!(got.location, LOCATION);
    println!(
        "  Got: name={}, kind={}",
        got.name.as_deref().unwrap_or(""),
        got.kind.as_deref().unwrap_or("")
    );

    // Step 6: Update storage account (PATCH) — enforce security settings
    println!(
        "[6/10] Updating storage account (PATCH) — enforcing HTTPS-only + TLS1_2 + no public blob access..."
    );
    let update_body = StorageAccountUpdateRequest {
        properties: Some(StorageAccountUpdateProperties {
            allow_blob_public_access: Some(false),
            supports_https_traffic_only: Some(true),
            minimum_tls_version: Some("TLS1_2".to_string()),
        }),
        ..Default::default()
    };
    let updated = storage
        .update_storage_account(subscription_id, RG_NAME, STORAGE_ACCOUNT_NAME, &update_body)
        .await
        .expect("update_storage_account failed");
    assert_eq!(updated.name.as_deref(), Some(STORAGE_ACCOUNT_NAME));
    let updated_props = updated.properties.as_ref();
    let https_only = updated_props
        .and_then(|p| p.get("supportsHttpsTrafficOnly"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let min_tls = updated_props
        .and_then(|p| p.get("minimumTlsVersion"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let allow_public = updated_props
        .and_then(|p| p.get("allowBlobPublicAccess"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    println!(
        "  supportsHttpsTrafficOnly={https_only}, minimumTlsVersion={min_tls}, allowBlobPublicAccess={allow_public}"
    );
    assert!(
        https_only,
        "expected supportsHttpsTrafficOnly=true after update"
    );
    assert_eq!(
        min_tls, "TLS1_2",
        "expected minimumTlsVersion=TLS1_2 after update"
    );
    assert!(
        !allow_public,
        "expected allowBlobPublicAccess=false after update"
    );

    // Step 7: List by resource group — should include our account
    println!("[7/10] Listing storage accounts in resource group after creation...");
    let rg_list_after = storage
        .list_storage_accounts_by_resource_group(subscription_id, RG_NAME)
        .await
        .expect("list_storage_accounts_by_resource_group after create failed");
    println!(
        "  Found {} accounts in resource group",
        rg_list_after.value.len()
    );
    assert!(
        rg_list_after.value.len() > initial_count,
        "expected at least one more account after creation"
    );
    let found = rg_list_after
        .value
        .iter()
        .any(|v| v.get("name").and_then(|n| n.as_str()) == Some(STORAGE_ACCOUNT_NAME));
    assert!(found, "new account not found in resource group list");

    // Step 8: List keys
    println!("[8/10] Listing access keys...");
    let keys_result = storage
        .list_keys(subscription_id, RG_NAME, STORAGE_ACCOUNT_NAME)
        .await
        .expect("list_keys failed");
    println!("  Found {} keys", keys_result.keys.len());
    assert!(!keys_result.keys.is_empty(), "expected at least one key");
    let first_key_name = keys_result.keys[0]
        .get("keyName")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    println!("  First key name: {first_key_name}");

    // Step 9: Regenerate key1
    println!("[9/10] Regenerating key1...");
    let regen_body = StorageAccountRegenerateKeyRequest {
        key_name: "key1".to_string(),
    };
    let regen_result = storage
        .regenerate_key(subscription_id, RG_NAME, STORAGE_ACCOUNT_NAME, &regen_body)
        .await
        .expect("regenerate_key failed");
    println!("  Regenerated keys: {} returned", regen_result.keys.len());
    assert!(
        !regen_result.keys.is_empty(),
        "expected keys after regeneration"
    );

    // Step 10: Get management policy — should return None (no policy configured)
    println!("[10/10] Getting management policy (should be None for new account)...");
    let policy = storage
        .get_management_policy(subscription_id, RG_NAME, STORAGE_ACCOUNT_NAME)
        .await
        .expect("get_management_policy failed");
    println!("  Management policy: {:?}", policy.is_some());
    assert!(
        policy.is_none(),
        "expected None for new account without lifecycle policy"
    );

    Ok(())
}
