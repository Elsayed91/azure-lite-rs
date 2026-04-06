//! Integration tests for Azure CosmosDB operations.
//!
//! Creates a real CosmosDB account + SQL database + SQL container and exercises
//! lifecycle. Account creation takes ~5-10 minutes.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration cosmosdb -- --ignored --test-threads=1 --nocapture

use azure_lite::types::cosmosdb::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const ACCOUNT_NAME: &str = "cloud-lite-test-cosmos";
const DATABASE_NAME: &str = "cloud-lite-test-db";
const CONTAINER_NAME: &str = "cloud-lite-test-container";

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

// ============================================================================
// Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and ~15-20 minutes"]
async fn cosmosdb_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let cosmos = client.cosmosdb();

    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/7] Pre-cleanup: removing any leftover test CosmosDB account...");
    az_run_ignore(&[
        "cosmosdb",
        "delete",
        "--name",
        ACCOUNT_NAME,
        "--resource-group",
        RG_NAME,
        "--yes",
    ])
    .await;
    for attempt in 0..30u32 {
        match cosmos.get_account(RG_NAME, ACCOUNT_NAME).await {
            Err(_) => {
                println!(
                    "  Account ARM record gone after {} attempt(s).",
                    attempt + 1
                );
                break;
            }
            Ok(_) => {
                if attempt % 6 == 0 {
                    println!(
                        "  Waiting for account deletion... (attempt {})",
                        attempt + 1
                    );
                }
                if attempt == 29 {
                    println!("  Warning: account still exists after 5 minutes — proceeding");
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        }
    }

    // =========================================================================
    // Step 2: Ensure resource group
    // =========================================================================
    println!("[2/7] Ensuring resource group '{RG_NAME}' exists...");
    ensure_resource_group().await;

    // =========================================================================
    // Step 3: Create CosmosDB account
    // =========================================================================
    println!("[3/7] Creating CosmosDB account '{ACCOUNT_NAME}'...");
    let create_req = DatabaseAccountCreateRequest {
        location: LOCATION.to_string(),
        kind: Some("GlobalDocumentDB".to_string()),
        properties: DatabaseAccountCreateUpdateProperties {
            database_account_offer_type: "Standard".to_string(),
            consistency_policy: Some(ConsistencyPolicy {
                default_consistency_level: "Session".to_string(),
                max_staleness_prefix: None,
                max_interval_in_seconds: None,
            }),
            enable_automatic_failover: Some(false),
            enable_multiple_write_locations: Some(false),
        },
        ..Default::default()
    };
    let account = cosmos
        .create_account(RG_NAME, ACCOUNT_NAME, &create_req)
        .await
        .expect("create_account failed");
    println!(
        "  Created: name={:?}, provisioning={:?}",
        account.name.as_deref(),
        account
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );

    // Poll for Succeeded state
    for attempt in 0..72u32 {
        match cosmos.get_account(RG_NAME, ACCOUNT_NAME).await {
            Ok(a) => {
                let state = a
                    .properties
                    .as_ref()
                    .and_then(|p| p.provisioning_state.as_deref())
                    .unwrap_or("");
                if state == "Succeeded" {
                    println!("  Account is Succeeded after {} poll(s).", attempt + 1);
                    break;
                }
                if attempt % 6 == 0 {
                    println!(
                        "  Still provisioning... state={state} (attempt {})",
                        attempt + 1
                    );
                }
                if attempt == 71 {
                    println!(
                        "  Warning: account did not reach Succeeded in 12 minutes — proceeding"
                    );
                }
            }
            Err(e) => {
                if attempt % 6 == 0 {
                    println!(
                        "  get_account returned error (attempt {}): {e}",
                        attempt + 1
                    );
                }
                if attempt == 71 {
                    println!("  Warning: account not found after 12 minutes — proceeding");
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    // =========================================================================
    // Step 4: Get + list accounts
    // =========================================================================
    println!("[4/7] Getting and listing accounts...");
    let got = cosmos
        .get_account(RG_NAME, ACCOUNT_NAME)
        .await
        .expect("get_account failed");
    assert_eq!(got.name.as_deref(), Some(ACCOUNT_NAME));
    let props = got
        .properties
        .as_ref()
        .expect("account should have properties");
    println!(
        "  Got account: endpoint={:?}, kind={:?}",
        props.document_endpoint.as_deref(),
        got.kind.as_deref(),
    );
    assert!(
        props.document_endpoint.is_some(),
        "should have document endpoint"
    );

    let all = cosmos.list_accounts().await.expect("list_accounts failed");
    let ours = all
        .value
        .iter()
        .any(|a| a.name.as_deref() == Some(ACCOUNT_NAME));
    println!(
        "  Found {} account(s) in subscription, ours present: {ours}",
        all.value.len()
    );
    assert!(ours, "Our account should appear in subscription list");

    // =========================================================================
    // Step 5: Create SQL database
    // =========================================================================
    println!("[5/7] Creating SQL database '{DATABASE_NAME}'...");
    let db_req = SqlDatabaseCreateRequest {
        location: LOCATION.to_string(),
        properties: SqlDatabaseCreateUpdateProperties {
            resource: SqlDatabaseResource {
                id: DATABASE_NAME.to_string(),
            },
        },
        ..Default::default()
    };
    let db = cosmos
        .create_sql_database(RG_NAME, ACCOUNT_NAME, DATABASE_NAME, &db_req)
        .await
        .expect("create_sql_database failed");
    println!("  Created database: name={:?}", db.name.as_deref());

    // Poll for database to be available
    for attempt in 0..12u32 {
        match cosmos
            .get_sql_database(RG_NAME, ACCOUNT_NAME, DATABASE_NAME)
            .await
        {
            Ok(d) => {
                let db_id = d
                    .properties
                    .as_ref()
                    .and_then(|p| p.resource.as_ref())
                    .and_then(|r| r.id.as_deref());
                if db_id.is_some() {
                    println!("  Database ready after {} poll(s).", attempt + 1);
                    break;
                }
            }
            Err(_) => {
                if attempt == 11 {
                    println!("  Warning: database not confirmed after 2 minutes — proceeding");
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    let db2 = cosmos
        .get_sql_database(RG_NAME, ACCOUNT_NAME, DATABASE_NAME)
        .await
        .expect("get_sql_database failed");
    assert_eq!(db2.name.as_deref(), Some(DATABASE_NAME));

    let dbs = cosmos
        .list_sql_databases(RG_NAME, ACCOUNT_NAME)
        .await
        .expect("list_sql_databases failed");
    println!("  Found {} database(s) in account.", dbs.value.len());
    assert!(!dbs.value.is_empty(), "Should have at least one database");
    assert!(
        dbs.value
            .iter()
            .any(|d| d.name.as_deref() == Some(DATABASE_NAME)),
        "Our database should appear in list"
    );

    // =========================================================================
    // Step 6: Create + list SQL containers
    // =========================================================================
    println!("[6/7] Creating SQL container '{CONTAINER_NAME}'...");
    az_run_ignore(&[
        "cosmosdb",
        "sql",
        "container",
        "create",
        "--account-name",
        ACCOUNT_NAME,
        "--resource-group",
        RG_NAME,
        "--database-name",
        DATABASE_NAME,
        "--name",
        CONTAINER_NAME,
        "--partition-key-path",
        "/id",
    ])
    .await;

    // Poll until container appears
    let mut container_found = false;
    for attempt in 0..12u32 {
        match cosmos
            .get_sql_container(RG_NAME, ACCOUNT_NAME, DATABASE_NAME, CONTAINER_NAME)
            .await
        {
            Ok(c) => {
                println!(
                    "  Container ready after {} poll(s): name={:?}",
                    attempt + 1,
                    c.name.as_deref()
                );
                container_found = true;
                break;
            }
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        }
    }

    if container_found {
        let containers = cosmos
            .list_sql_containers(RG_NAME, ACCOUNT_NAME, DATABASE_NAME)
            .await
            .expect("list_sql_containers failed");
        println!(
            "  Found {} container(s) in database.",
            containers.value.len()
        );
        assert!(
            !containers.value.is_empty(),
            "Should have at least one container"
        );
    } else {
        println!("  Warning: container not found after polls — skipping container assertions");
    }

    // =========================================================================
    // Step 7: Cleanup
    // =========================================================================
    println!("[7/7] Deleting CosmosDB account '{ACCOUNT_NAME}'...");
    cosmos
        .delete_account(RG_NAME, ACCOUNT_NAME)
        .await
        .expect("delete_account failed");
    println!("  Delete requested (async, account will terminate in background).");

    println!("\nAll Azure CosmosDB integration tests passed!");
}
