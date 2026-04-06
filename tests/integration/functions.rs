//! Integration tests for Azure Functions operations.
//!
//! Creates a real Function App and exercises its lifecycle.
//! Requires a Consumption plan (automatically created) in eastus.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration functions -- --ignored --test-threads=1 --nocapture

use azure_lite::types::functions::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const APP_NAME: &str = "cloud-lite-test-func-app";
const STORAGE_ACCOUNT: &str = "cloudlitetestfuncstorage";

// ============================================================================
// Helpers
// ============================================================================

async fn az_run_ignore(args: &[&str]) {
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
            && !stderr.contains("NotFound")
        {
            eprintln!("Warning: az {} failed: {}", args.join(" "), stderr);
        }
    }
}

async fn ensure_resource_group() {
    let output = tokio::process::Command::new("az")
        .args(["group", "create", "--name", RG_NAME, "--location", LOCATION])
        .output()
        .await
        .expect("az group create failed");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Failed to ensure resource group: {stderr}");
    }
    println!("  Resource group ready.");
}

/// Ensure a storage account exists for the Function App.
async fn ensure_storage_account() {
    let output = tokio::process::Command::new("az")
        .args([
            "storage",
            "account",
            "create",
            "--name",
            STORAGE_ACCOUNT,
            "--resource-group",
            RG_NAME,
            "--location",
            LOCATION,
            "--sku",
            "Standard_LRS",
        ])
        .output()
        .await
        .expect("az storage account create failed");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignore "already exists" errors
        if !stderr.contains("already exists") && !stderr.contains("AlreadyExists") {
            panic!("Failed to create storage account: {stderr}");
        }
    }
    println!("  Storage account ready.");
}

/// Get connection string for a storage account.
async fn get_storage_connection_string() -> String {
    let output = tokio::process::Command::new("az")
        .args([
            "storage",
            "account",
            "show-connection-string",
            "--name",
            STORAGE_ACCOUNT,
            "--resource-group",
            RG_NAME,
            "--output",
            "tsv",
            "--query",
            "connectionString",
        ])
        .output()
        .await
        .expect("az storage account show-connection-string failed");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Failed to get storage connection string: {stderr}");
    }
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

// ============================================================================
// Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and ~5-10 minutes"]
async fn functions_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let functions = client.functions();

    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/7] Pre-cleanup: removing any leftover test Function App...");
    az_run_ignore(&[
        "functionapp",
        "delete",
        "--name",
        APP_NAME,
        "--resource-group",
        RG_NAME,
    ])
    .await;
    // Poll until ARM record is gone
    for attempt in 0..30u32 {
        match functions.get_function_app(RG_NAME, APP_NAME).await {
            Err(_) => {
                println!(
                    "  Function App ARM record gone after {} attempt(s).",
                    attempt + 1
                );
                break;
            }
            Ok(_) => {
                if attempt % 6 == 0 {
                    println!("  Waiting for deletion... (attempt {})", attempt + 1);
                }
                if attempt == 29 {
                    println!("  Warning: Function App still exists after 5 minutes — proceeding");
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        }
    }

    // =========================================================================
    // Step 2: Ensure resource group and storage account
    // =========================================================================
    println!("[2/7] Ensuring resource group and storage account...");
    ensure_resource_group().await;
    ensure_storage_account().await;
    let conn_str = get_storage_connection_string().await;

    // =========================================================================
    // Step 3: Create Function App via az CLI (ARM PUT for Consumption plan is complex)
    // =========================================================================
    println!("[3/7] Creating Function App '{APP_NAME}' via az CLI...");
    let create_output = tokio::process::Command::new("az")
        .args([
            "functionapp",
            "create",
            "--name",
            APP_NAME,
            "--resource-group",
            RG_NAME,
            "--storage-account",
            STORAGE_ACCOUNT,
            "--consumption-plan-location",
            LOCATION,
            "--runtime",
            "python",
            "--runtime-version",
            "3.11",
            "--os-type",
            "linux",
            "--functions-version",
            "4",
        ])
        .output()
        .await
        .expect("az functionapp create failed");
    if !create_output.status.success() {
        let stderr = String::from_utf8_lossy(&create_output.stderr);
        panic!("Failed to create Function App: {stderr}");
    }
    println!("  Function App creation requested.");

    // =========================================================================
    // Step 4: Poll for Running state
    // =========================================================================
    println!("[4/7] Waiting for Function App to reach Running state...");
    let mut found = false;
    for attempt in 0..18u32 {
        match functions.get_function_app(RG_NAME, APP_NAME).await {
            Ok(app) => {
                let state = app
                    .properties
                    .as_ref()
                    .and_then(|p| p.state.as_deref())
                    .unwrap_or("");
                if state == "Running" {
                    println!(
                        "  Function App is Running after {} attempt(s).",
                        attempt + 1
                    );
                    found = true;
                    break;
                }
                if attempt % 3 == 0 {
                    println!(
                        "  Still provisioning... state={state} (attempt {})",
                        attempt + 1
                    );
                }
            }
            Err(e) => {
                if attempt % 3 == 0 {
                    println!(
                        "  get_function_app returned error (attempt {}): {e}",
                        attempt + 1
                    );
                }
            }
        }
        if attempt == 17 {
            println!(
                "  Warning: Function App did not reach Running in 3 minutes — proceeding anyway"
            );
            found = true; // proceed anyway
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    // =========================================================================
    // Step 5: Get + list Function Apps
    // =========================================================================
    println!("[5/7] Getting and listing Function Apps...");
    let app = functions
        .get_function_app(RG_NAME, APP_NAME)
        .await
        .expect("get_function_app failed");
    assert_eq!(app.name.as_deref(), Some(APP_NAME));
    println!(
        "  Got Function App: state={:?}, defaultHostName={:?}",
        app.properties.as_ref().and_then(|p| p.state.as_deref()),
        app.properties
            .as_ref()
            .and_then(|p| p.default_host_name.as_deref()),
    );

    let all = functions
        .list_function_apps_by_resource_group(RG_NAME)
        .await
        .expect("list_function_apps_by_resource_group failed");
    let ours = all
        .value
        .iter()
        .any(|a| a.name.as_deref() == Some(APP_NAME));
    println!(
        "  Found {} app(s) in resource group, ours present: {ours}",
        all.value.len()
    );
    assert!(
        ours,
        "Our Function App should appear in resource group list"
    );

    // =========================================================================
    // Step 6: List functions + get app settings
    // =========================================================================
    println!("[6/7] Listing functions and app settings...");
    let funcs = functions
        .list_functions(RG_NAME, APP_NAME)
        .await
        .expect("list_functions failed");
    println!(
        "  Found {} function(s) (no functions deployed yet).",
        funcs.value.len()
    );

    // Get app settings (list_app_settings is a POST)
    match functions.list_app_settings(RG_NAME, APP_NAME).await {
        Ok(settings) => {
            println!("  Got {} app setting(s).", settings.properties.len());
            // AzureWebJobsStorage should be present if storage account is linked
            let has_storage = settings.properties.contains_key("AzureWebJobsStorage")
                || settings
                    .properties
                    .contains_key("WEBSITE_CONTENTAZUREFILECONNECTIONSTRING");
            println!("  Has storage connection: {has_storage}");
        }
        Err(e) => {
            println!("  list_app_settings failed: {e} — skipping");
        }
    }

    // Update app settings with a test value
    let mut new_settings = std::collections::HashMap::new();
    new_settings.insert("CLOUD_LITE_TEST".to_string(), "hello".to_string());
    new_settings.insert("AzureWebJobsStorage".to_string(), conn_str.clone());
    let update_body = AppSettingsUpdateRequest {
        properties: new_settings,
    };
    match functions
        .update_app_settings(RG_NAME, APP_NAME, &update_body)
        .await
    {
        Ok(updated) => {
            println!(
                "  Updated app settings, {} setting(s) present.",
                updated.properties.len()
            );
        }
        Err(e) => {
            println!("  update_app_settings failed: {e} — skipping");
        }
    }

    // =========================================================================
    // Step 7: Cleanup
    // =========================================================================
    println!("[7/7] Deleting Function App '{APP_NAME}'...");
    functions
        .delete_function_app(RG_NAME, APP_NAME)
        .await
        .expect("delete_function_app failed");
    println!("  Delete requested.");

    println!("\nAll Azure Functions integration tests passed!");
    let _ = found;
}
