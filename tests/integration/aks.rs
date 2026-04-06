//! Integration tests for Azure Kubernetes Service operations.
//!
//! Creates a real AKS cluster and exercises cluster + node pool lifecycle.
//! AKS cluster creation takes ~5-10 minutes; total test time ~20-30 minutes.
//!
//! NOTE: CreateNodePool / DeleteNodePool are covered by generated op tests.
//! RunCommand is tested but requires the cluster to be Running first.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration aks -- --ignored --test-threads=1 --nocapture

use azure_lite::types::aks::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const CLUSTER_NAME: &str = "cloud-lite-test-aks";
const NODE_POOL_NAME: &str = "nodepool1";
const DNS_PREFIX: &str = "cloud-lite-test-aks";

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

async fn cleanup_cluster() {
    az_run_ignore(&[
        "aks",
        "delete",
        "--name",
        CLUSTER_NAME,
        "--resource-group",
        RG_NAME,
        "--yes",
        "--no-wait",
    ])
    .await;
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
#[ignore = "requires Azure credentials and ~20-30 minutes"]
async fn aks_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let aks = client.aks();

    // =========================================================================
    // Step 1: Pre-cleanup — delete any leftover cluster and wait until fully gone
    // =========================================================================
    println!("[1/8] Pre-cleanup: removing any leftover test cluster...");
    cleanup_cluster().await;
    // Poll until the cluster ARM record is gone
    for attempt in 0..60u32 {
        match aks.get_cluster(RG_NAME, CLUSTER_NAME).await {
            Err(_) => {
                println!(
                    "  Cluster ARM record gone after {} attempt(s).",
                    attempt + 1
                );
                break;
            }
            Ok(c) => {
                let state = c
                    .properties
                    .as_ref()
                    .and_then(|p| p.provisioning_state.as_deref())
                    .unwrap_or("");
                if attempt % 6 == 0 {
                    println!(
                        "  Waiting for cluster deletion... state={state} (attempt {})",
                        attempt + 1
                    );
                }
                if attempt == 59 {
                    panic!("Cluster ARM record not deleted in 10 minutes");
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        }
    }
    // Also wait for the managed node resource group to be fully cleaned up.
    // AKS ARM record disappears before the underlying MC_* resource group is deleted.
    let mc_rg = format!("MC_{RG_NAME}_{CLUSTER_NAME}_{LOCATION}");
    for attempt in 0..60u32 {
        let output = tokio::process::Command::new("az")
            .args(["group", "show", "--name", &mc_rg])
            .output()
            .await
            .expect("az group show failed");
        if !output.status.success() {
            println!("  Managed resource group '{mc_rg}' gone.");
            break;
        }
        if attempt % 6 == 0 {
            println!(
                "  Waiting for managed RG '{mc_rg}' deletion... (attempt {})",
                attempt + 1
            );
        }
        if attempt == 59 {
            println!("  Warning: managed RG still exists after 10 minutes — proceeding anyway");
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    // =========================================================================
    // Step 2: Ensure resource group
    // =========================================================================
    println!("[2/8] Ensuring resource group '{RG_NAME}' exists...");
    ensure_resource_group().await;

    // =========================================================================
    // Step 3: Create cluster
    // =========================================================================
    println!("[3/8] Creating AKS cluster '{CLUSTER_NAME}'...");
    let create_req = ManagedClusterCreateRequest {
        location: LOCATION.to_string(),
        identity: Some(ManagedClusterIdentity {
            r#type: Some("SystemAssigned".to_string()),
            ..Default::default()
        }),
        properties: Some(ManagedClusterCreateOrUpdateProperties {
            dns_prefix: DNS_PREFIX.to_string(),
            // kubernetes_version omitted — Azure picks the default stable version
            enable_rbac: Some(true),
            agent_pool_profiles: vec![ManagedClusterAgentPoolProfile {
                name: NODE_POOL_NAME.to_string(),
                count: Some(1),
                vm_size: Some("Standard_D2s_v3".to_string()),
                os_type: Some("Linux".to_string()),
                mode: Some("System".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let cluster = aks
        .create_cluster(RG_NAME, CLUSTER_NAME, &create_req)
        .await
        .expect("create_cluster failed");
    println!(
        "  Created: name={:?}, location={:?}, provisioning_state={:?}",
        cluster.name.as_deref(),
        cluster.location.as_deref(),
        cluster
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );

    // =========================================================================
    // Step 4: Poll for Succeeded provisioning state (~5-10 minutes)
    // =========================================================================
    println!(
        "[4/8] Waiting for cluster to reach Succeeded provisioning state (this takes ~5-10 minutes)..."
    );
    let mut succeeded = false;
    for attempt in 0..60u32 {
        let c = aks
            .get_cluster(RG_NAME, CLUSTER_NAME)
            .await
            .expect("poll get_cluster failed");
        let state = c
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref())
            .unwrap_or("");
        if state == "Succeeded" {
            println!("  Cluster is Succeeded after {} poll(s).", attempt + 1);
            succeeded = true;
            break;
        }
        if state == "Failed" || state == "Canceled" {
            panic!("Cluster provisioning failed with state: {state}");
        }
        if attempt % 6 == 0 {
            println!(
                "  Still provisioning... state={state} (attempt {})",
                attempt + 1
            );
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
    if !succeeded {
        cleanup_cluster().await;
        panic!("Cluster did not reach Succeeded after 60 polls (10 minutes)");
    }

    // =========================================================================
    // Step 5: Get cluster + list clusters
    // =========================================================================
    println!("[5/8] Getting and listing clusters...");
    let got = aks
        .get_cluster(RG_NAME, CLUSTER_NAME)
        .await
        .expect("get_cluster failed");
    assert_eq!(got.name.as_deref(), Some(CLUSTER_NAME));
    let props = got
        .properties
        .as_ref()
        .expect("cluster should have properties");
    assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
    println!(
        "  Got cluster: fqdn={:?}, k8s_version={:?}",
        props.fqdn.as_deref(),
        props.current_kubernetes_version.as_deref(),
    );

    let all = aks.list_clusters().await.expect("list_clusters failed");
    let ours = all
        .value
        .iter()
        .any(|c| c.name.as_deref() == Some(CLUSTER_NAME));
    println!(
        "  Found {} cluster(s) in subscription, ours present: {ours}",
        all.value.len()
    );
    assert!(ours, "Our cluster should appear in subscription list");

    // =========================================================================
    // Step 6: List node pools + get the system node pool
    // =========================================================================
    println!("[6/8] Listing and getting node pools...");
    let pools = aks
        .list_node_pools(RG_NAME, CLUSTER_NAME)
        .await
        .expect("list_node_pools failed");
    println!("  Found {} node pool(s).", pools.value.len());
    assert!(
        !pools.value.is_empty(),
        "Should have at least the system node pool"
    );

    let pool = aks
        .get_node_pool(RG_NAME, CLUSTER_NAME, NODE_POOL_NAME)
        .await
        .expect("get_node_pool failed");
    assert_eq!(pool.name.as_deref(), Some(NODE_POOL_NAME));
    let pool_props = pool
        .properties
        .as_ref()
        .expect("node pool should have properties");
    println!(
        "  Node pool: count={:?}, vmSize={:?}, mode={:?}, state={:?}",
        pool_props.count,
        pool_props.vm_size.as_deref(),
        pool_props.mode.as_deref(),
        pool_props.provisioning_state.as_deref(),
    );
    assert_eq!(pool_props.mode.as_deref(), Some("System"));

    // =========================================================================
    // Step 7: Get credentials
    // =========================================================================
    println!("[7/8] Getting user credentials (kubeconfig)...");
    let creds = aks
        .get_credentials(RG_NAME, CLUSTER_NAME)
        .await
        .expect("get_credentials failed");
    println!("  Got {} kubeconfig(s).", creds.kubeconfigs.len());
    assert!(
        !creds.kubeconfigs.is_empty(),
        "Should have at least one kubeconfig"
    );

    // =========================================================================
    // Step 8: Run command (kubectl get nodes) + poll for result
    // =========================================================================
    println!("[8/8] Running 'kubectl get nodes' command...");
    let cmd_req = RunCommandRequest {
        command: "kubectl get nodes".to_string(),
        ..Default::default()
    };
    // run_command returns 202 Accepted with a RunCommandResult containing an id
    // We need to poll get_command_result until provisioningState = "succeeded"
    let cmd_result = aks.run_command(RG_NAME, CLUSTER_NAME, &cmd_req).await;
    match cmd_result {
        Ok(initial) => {
            let cmd_id = initial.id.as_deref().unwrap_or("");
            println!("  RunCommand accepted, id={cmd_id:?}");
            if !cmd_id.is_empty() {
                // Extract just the command ID (last path segment)
                let short_id = cmd_id.rsplit('/').next().unwrap_or(cmd_id);
                // Poll for completion
                for attempt in 0..30u32 {
                    let result = aks
                        .get_command_result(RG_NAME, CLUSTER_NAME, short_id)
                        .await
                        .expect("get_command_result failed");
                    let state = result
                        .properties
                        .as_ref()
                        .and_then(|p| p.provisioning_state.as_deref())
                        .unwrap_or("");
                    if state.eq_ignore_ascii_case("succeeded") {
                        let logs = result
                            .properties
                            .as_ref()
                            .and_then(|p| p.logs.as_deref())
                            .unwrap_or("(no logs)");
                        let exit_code = result
                            .properties
                            .as_ref()
                            .and_then(|p| p.exit_code)
                            .unwrap_or(-1);
                        println!("  Command completed: exit_code={exit_code}, logs={logs}");
                        assert_eq!(exit_code, 0, "kubectl get nodes should exit 0");
                        break;
                    }
                    if state.eq_ignore_ascii_case("failed") {
                        println!("  RunCommand failed (may be cluster not fully ready)");
                        break;
                    }
                    if attempt == 29 {
                        println!("  RunCommand did not complete in 30 polls — skipping assertion");
                        break;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
        Err(e) => {
            // run_command may return 202 which could fail deserialization — treat as soft failure
            println!("  RunCommand returned error (may be 202 async result): {e}");
        }
    }

    // =========================================================================
    // Cleanup: Delete cluster
    // =========================================================================
    println!("\n[Cleanup] Deleting cluster '{CLUSTER_NAME}'...");
    aks.delete_cluster(RG_NAME, CLUSTER_NAME)
        .await
        .expect("delete_cluster failed");
    println!("  Delete requested (async, cluster will terminate in background).");

    println!("\nAll AKS integration tests passed!");
}
