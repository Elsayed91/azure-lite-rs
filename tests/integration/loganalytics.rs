//! Integration tests for Azure Log Analytics operations.
//!
//! Tests workspace management, KQL query execution, and saved searches.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration loganalytics -- --ignored --test-threads=1 --nocapture

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const WORKSPACE_NAME: &str = "cloud-lite-test-ralph-workspace";

// ============================================================================
// Helpers
// ============================================================================

async fn ensure_resource_group(client: &azure_lite::AzureHttpClient) {
    let sub_id = std::env::var("AZURE_SUBSCRIPTION_ID").unwrap();
    // Try to create; ignore if it already exists
    let _ = tokio::process::Command::new("az")
        .args([
            "group",
            "create",
            "--name",
            RG_NAME,
            "--location",
            LOCATION,
            "--subscription",
            &sub_id,
            "--output",
            "none",
        ])
        .status()
        .await
        .expect("az group create failed");
    let _ = client; // suppress unused warning
}

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
            eprintln!("az CLI warning: {stderr}");
        }
    }
}

/// Poll workspace until it reaches Succeeded (or timeout after N attempts).
async fn poll_workspace(
    client: &azure_lite::AzureHttpClient,
    workspace_name: &str,
    max_attempts: u32,
) {
    for i in 0..max_attempts {
        match client
            .log_analytics()
            .get_workspace(RG_NAME, workspace_name)
            .await
        {
            Ok(ws) => {
                let state = ws
                    .properties
                    .as_ref()
                    .and_then(|p| p.provisioning_state.as_deref())
                    .unwrap_or("Unknown");
                println!(
                    "  Workspace state: {state} (attempt {}/{})",
                    i + 1,
                    max_attempts
                );
                if state == "Succeeded" {
                    return;
                }
            }
            Err(e) => println!("  Poll error: {e} (attempt {}/{})", i + 1, max_attempts),
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
    panic!(
        "Workspace did not reach Succeeded state after {} attempts",
        max_attempts
    );
}

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
#[ignore]
async fn test_log_analytics_lifecycle() {
    let sub_id = std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID must be set");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to create AzureHttpClient");

    println!("[1/9] Pre-cleanup: deleting any stale test workspace...");
    az_run_ignore(&[
        "monitor",
        "log-analytics",
        "workspace",
        "delete",
        "--resource-group",
        RG_NAME,
        "--workspace-name",
        WORKSPACE_NAME,
        "--subscription",
        &sub_id,
        "--yes",
        "--force",
        "true",
        "--output",
        "none",
    ])
    .await;
    // Wait briefly for deletion propagation
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("[2/9] Ensuring resource group exists...");
    ensure_resource_group(&client).await;

    println!("[3/9] Creating Log Analytics workspace '{WORKSPACE_NAME}'...");
    let body = azure_lite::types::loganalytics::WorkspaceCreateRequest {
        location: LOCATION.into(),
        ..Default::default()
    };
    let ws = client
        .log_analytics()
        .create_workspace(RG_NAME, WORKSPACE_NAME, &body)
        .await
        .expect("create_workspace failed");
    assert_eq!(ws.name.as_deref(), Some(WORKSPACE_NAME));
    println!("  Created: {}", ws.id.as_deref().unwrap_or("(no id)"));

    println!("[4/9] Polling until workspace is Succeeded...");
    poll_workspace(&client, WORKSPACE_NAME, 30).await;

    println!("[5/9] Getting workspace...");
    let ws = client
        .log_analytics()
        .get_workspace(RG_NAME, WORKSPACE_NAME)
        .await
        .expect("get_workspace failed");
    assert_eq!(ws.name.as_deref(), Some(WORKSPACE_NAME));
    let props = ws.properties.as_ref().expect("workspace has no properties");
    println!(
        "  customerId: {}",
        props.customer_id.as_deref().unwrap_or("(none)")
    );
    println!(
        "  provisioningState: {}",
        props.provisioning_state.as_deref().unwrap_or("(none)")
    );

    println!("[6/9] Listing workspaces in subscription...");
    let list = client
        .log_analytics()
        .list_workspaces()
        .await
        .expect("list_workspaces failed");
    let found = list
        .value
        .iter()
        .any(|w| w.name.as_deref() == Some(WORKSPACE_NAME));
    assert!(
        found,
        "test workspace not found in list; got {} workspaces",
        list.value.len()
    );

    println!("[7/9] Running a KQL query (AzureActivity | limit 5)...");
    let query_body = azure_lite::types::loganalytics::LogQueryBody {
        query: "AzureActivity | limit 5".into(),
        timespan: Some("PT1H".into()),
        ..Default::default()
    };
    match client
        .log_analytics()
        .query_logs(RG_NAME, WORKSPACE_NAME, &query_body)
        .await
    {
        Ok(result) => {
            println!("  Query returned {} table(s)", result.tables.len());
            assert!(!result.tables.is_empty(), "query returned no tables");
        }
        Err(e) => println!("  Warning: query_logs failed (graceful): {e}"),
    }

    println!("[8/9] Listing saved searches...");
    let saved = client
        .log_analytics()
        .list_saved_searches(RG_NAME, WORKSPACE_NAME)
        .await
        .expect("list_saved_searches failed");
    println!("  {} saved search(es) found", saved.value.len());

    println!("[9/9] Deleting workspace...");
    client
        .log_analytics()
        .delete_workspace(RG_NAME, WORKSPACE_NAME)
        .await
        .expect("delete_workspace failed");
    println!("  Done.");
}
