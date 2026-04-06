//! Integration tests for Azure SQL operations.
//!
//! Creates a real Azure SQL server + database and exercises lifecycle.
//! SQL server creation takes ~2-5 minutes.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration sql -- --ignored --test-threads=1 --nocapture

use azure_lite::types::sql::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const SERVER_NAME: &str = "cloud-lite-test-sql-srv";
const DATABASE_NAME: &str = "cloud-lite-test-db";
const FIREWALL_RULE_NAME: &str = "cloud-lite-test-fw-rule";
const ADMIN_LOGIN: &str = "cloudliteadmin";
const ADMIN_PASSWORD: &str = "Cloud!Lite#2026Test";

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
#[ignore = "requires Azure credentials and ~10-15 minutes"]
async fn sql_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let sql = client.sql();

    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/7] Pre-cleanup: removing any leftover test SQL server...");
    az_run_ignore(&[
        "sql",
        "server",
        "delete",
        "--name",
        SERVER_NAME,
        "--resource-group",
        RG_NAME,
        "--yes",
    ])
    .await;
    for attempt in 0..30u32 {
        match sql.get_server(RG_NAME, SERVER_NAME).await {
            Err(_) => {
                println!("  Server ARM record gone after {} attempt(s).", attempt + 1);
                break;
            }
            Ok(_) => {
                if attempt % 6 == 0 {
                    println!("  Waiting for server deletion... (attempt {})", attempt + 1);
                }
                if attempt == 29 {
                    println!("  Warning: server still exists after 5 minutes — proceeding");
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
    // Step 3: Create server
    // =========================================================================
    println!("[3/7] Creating SQL server '{SERVER_NAME}'...");
    let create_req = ServerCreateRequest {
        location: LOCATION.to_string(),
        properties: Some(ServerCreateOrUpdateProperties {
            administrator_login: ADMIN_LOGIN.to_string(),
            administrator_login_password: Some(ADMIN_PASSWORD.to_string()),
            version: Some("12.0".to_string()),
        }),
        ..Default::default()
    };
    let server = sql
        .create_server(RG_NAME, SERVER_NAME, &create_req)
        .await
        .expect("create_server failed");
    println!(
        "  Created: name={:?}, state={:?}, fqdn={:?}",
        server.name.as_deref(),
        server.properties.as_ref().and_then(|p| p.state.as_deref()),
        server
            .properties
            .as_ref()
            .and_then(|p| p.fully_qualified_domain_name.as_deref()),
    );

    // Poll for Ready state — handle 404 during async provisioning
    for attempt in 0..36u32 {
        match sql.get_server(RG_NAME, SERVER_NAME).await {
            Ok(s) => {
                let state = s
                    .properties
                    .as_ref()
                    .and_then(|p| p.state.as_deref())
                    .unwrap_or("");
                if state == "Ready" {
                    println!("  Server is Ready after {} poll(s).", attempt + 1);
                    break;
                }
                if attempt % 3 == 0 {
                    println!(
                        "  Still provisioning... state={state} (attempt {})",
                        attempt + 1
                    );
                }
                if attempt == 35 {
                    println!("  Warning: server did not reach Ready in 6 minutes — proceeding");
                }
            }
            Err(e) => {
                // NotFound is expected during async provisioning
                if attempt % 6 == 0 {
                    println!(
                        "  get_server returned error (expected during provisioning, attempt {}): {e}",
                        attempt + 1
                    );
                }
                if attempt == 35 {
                    println!("  Warning: server not found after 6 minutes — proceeding");
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    // =========================================================================
    // Step 4: Add firewall rule (allow Azure services)
    // =========================================================================
    println!("[4/7] Creating firewall rule '{FIREWALL_RULE_NAME}'...");
    let fw_body = FirewallRuleCreateRequest {
        properties: Some(FirewallRuleProperties {
            start_ip_address: "0.0.0.0".to_string(),
            end_ip_address: "0.0.0.0".to_string(),
        }),
    };
    let fw = sql
        .create_firewall_rule(RG_NAME, SERVER_NAME, FIREWALL_RULE_NAME, &fw_body)
        .await
        .expect("create_firewall_rule failed");
    println!("  Firewall rule: name={:?}", fw.name.as_deref());

    let rules = sql
        .list_firewall_rules(RG_NAME, SERVER_NAME)
        .await
        .expect("list_firewall_rules failed");
    println!("  Found {} firewall rule(s).", rules.value.len());
    assert!(
        !rules.value.is_empty(),
        "Should have at least one firewall rule"
    );

    // =========================================================================
    // Step 5: Get + list servers
    // =========================================================================
    println!("[5/7] Getting and listing servers...");
    let got = sql
        .get_server(RG_NAME, SERVER_NAME)
        .await
        .expect("get_server failed");
    assert_eq!(got.name.as_deref(), Some(SERVER_NAME));
    let props = got
        .properties
        .as_ref()
        .expect("server should have properties");
    println!(
        "  Got server: fqdn={:?}, state={:?}, version={:?}",
        props.fully_qualified_domain_name.as_deref(),
        props.state.as_deref(),
        props.version.as_deref(),
    );
    assert_eq!(props.administrator_login.as_deref(), Some(ADMIN_LOGIN));

    let all = sql.list_servers().await.expect("list_servers failed");
    let ours = all
        .value
        .iter()
        .any(|s| s.name.as_deref() == Some(SERVER_NAME));
    println!(
        "  Found {} server(s) in subscription, ours present: {ours}",
        all.value.len()
    );
    assert!(ours, "Our server should appear in subscription list");

    // =========================================================================
    // Step 6: Create database + get + list
    // =========================================================================
    println!("[6/7] Creating database '{DATABASE_NAME}'...");
    let db_req = DatabaseCreateRequest {
        location: LOCATION.to_string(),
        sku: Some(DatabaseSku {
            name: "Basic".to_string(),
            tier: Some("Basic".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let db = sql
        .create_database(RG_NAME, SERVER_NAME, DATABASE_NAME, &db_req)
        .await
        .expect("create_database failed");
    println!(
        "  Created: name={:?}, status={:?}",
        db.name.as_deref(),
        db.properties.as_ref().and_then(|p| p.status.as_deref()),
    );

    // Poll for Online state — handle 404 during async provisioning
    for attempt in 0..24u32 {
        match sql.get_database(RG_NAME, SERVER_NAME, DATABASE_NAME).await {
            Ok(d) => {
                let status = d
                    .properties
                    .as_ref()
                    .and_then(|p| p.status.as_deref())
                    .unwrap_or("");
                if status == "Online" {
                    println!("  Database is Online after {} poll(s).", attempt + 1);
                    break;
                }
                if attempt % 3 == 0 {
                    println!(
                        "  Still creating... status={status} (attempt {})",
                        attempt + 1
                    );
                }
                if attempt == 23 {
                    println!("  Warning: database did not reach Online in 4 minutes");
                }
            }
            Err(e) => {
                if attempt % 6 == 0 {
                    println!(
                        "  get_database returned error (attempt {}): {e}",
                        attempt + 1
                    );
                }
                if attempt == 23 {
                    println!("  Warning: database not found after 4 minutes — proceeding");
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    let db2 = sql
        .get_database(RG_NAME, SERVER_NAME, DATABASE_NAME)
        .await
        .expect("get_database failed");
    assert_eq!(db2.name.as_deref(), Some(DATABASE_NAME));
    let db_props = db2
        .properties
        .as_ref()
        .expect("database should have properties");
    println!(
        "  Database: status={:?}, slo={:?}",
        db_props.status.as_deref(),
        db_props.current_service_objective_name.as_deref(),
    );

    let dbs = sql
        .list_databases(RG_NAME, SERVER_NAME)
        .await
        .expect("list_databases failed");
    println!("  Found {} database(s) in server.", dbs.value.len());
    // Azure SQL always has at least 'master' database
    assert!(!dbs.value.is_empty(), "Should have at least one database");

    // Delete test database
    sql.delete_database(RG_NAME, SERVER_NAME, DATABASE_NAME)
        .await
        .expect("delete_database failed");
    println!("  Database deleted.");

    // =========================================================================
    // Step 7: Cleanup — delete server
    // =========================================================================
    println!("[7/7] Deleting server '{SERVER_NAME}'...");
    sql.delete_server(RG_NAME, SERVER_NAME)
        .await
        .expect("delete_server failed");
    println!("  Delete requested (async, server will terminate in background).");

    println!("\nAll Azure SQL integration tests passed!");
}
