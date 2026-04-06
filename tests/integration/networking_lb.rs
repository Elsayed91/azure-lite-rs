//! Integration tests for Azure Load Balancer operations.
//!
//! These tests create real Azure resources and MUST clean up after themselves.
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID.
//!
//! Run: cargo test -p azure-lite --test integration lb_lifecycle -- --ignored --test-threads=1 --nocapture

use azure_lite::types::networking::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const LB_NAME: &str = "cloud-lite-test-ralph-lb";
const PIP_NAME: &str = "cloud-lite-test-ralph-lb-pip";

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
        {
            return String::new();
        }
        panic!("az {} failed: {}", args.join(" "), stderr);
    }
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Helper: delete a resource, ignoring "not found" errors.
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
        {
            eprintln!(
                "Warning: cleanup command failed: az {}: {}",
                args.join(" "),
                stderr,
            );
        }
    }
}

async fn lb_cleanup() {
    println!("  Cleaning up test load balancer '{LB_NAME}'...");
    az_delete_ignore(&[
        "network",
        "lb",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        LB_NAME,
    ])
    .await;

    println!("  Cleaning up test public IP '{PIP_NAME}'...");
    az_delete_ignore(&[
        "network",
        "public-ip",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        PIP_NAME,
    ])
    .await;
}

// ============================================================================
// Load Balancer Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn lb_lifecycle() {
    println!("[1/8] Pre-cleanup: removing any leftover LB test resources...");
    lb_cleanup().await;

    let result = std::panic::AssertUnwindSafe(async {
        lb_lifecycle_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    println!("\n[8/8] Final LB cleanup...");
    lb_cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn lb_lifecycle_inner() {
    // =========================================================================
    // Step 2: Ensure resource group exists
    // =========================================================================
    println!("[2/8] Ensuring resource group '{RG_NAME}' exists in '{LOCATION}'...");
    az(&[
        "group",
        "create",
        "--name",
        RG_NAME,
        "--location",
        LOCATION,
        "--output",
        "json",
    ])
    .await;
    println!("  Resource group ready.");

    // =========================================================================
    // Step 3: Build client
    // =========================================================================
    println!("[3/8] Building AzureHttpClient...");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build client from env");
    let networking = client.networking();

    // =========================================================================
    // Step 4: List LBs (should not contain ours yet)
    // =========================================================================
    println!("[4/8] Listing load balancers in '{RG_NAME}' (before create)...");
    let list_before = networking
        .list_load_balancers(RG_NAME)
        .await
        .expect("list_load_balancers failed");
    println!("  Found {} LB(s) before create.", list_before.value.len());
    assert!(
        !list_before
            .value
            .iter()
            .any(|lb| lb.name.as_deref() == Some(LB_NAME)),
        "Test LB should not exist before create",
    );

    // Also test subscription-level list
    let all_lbs = networking
        .list_load_balancers_all()
        .await
        .expect("list_load_balancers_all failed");
    println!("  Subscription-level LB count: {}", all_lbs.value.len());

    // =========================================================================
    // Step 5: Create prerequisite public IP via az CLI
    // =========================================================================
    println!("[5/8] Creating public IP '{PIP_NAME}' via az CLI...");
    let pip_json = az(&[
        "network",
        "public-ip",
        "create",
        "--resource-group",
        RG_NAME,
        "--name",
        PIP_NAME,
        "--location",
        LOCATION,
        "--sku",
        "Standard",
        "--allocation-method",
        "Static",
        "--output",
        "json",
    ])
    .await;
    let pip_value: serde_json::Value =
        serde_json::from_str(&pip_json).expect("Failed to parse public IP response");
    let pip_id = pip_value["publicIp"]["id"]
        .as_str()
        .expect("Public IP should have an ID")
        .to_string();
    println!("  Public IP created: id={pip_id}");

    // =========================================================================
    // Step 6: Create Load Balancer via library client
    // =========================================================================
    println!("[6/8] Creating load balancer '{LB_NAME}' via library client...");
    let lb_request = LoadBalancerCreateRequest {
        location: LOCATION.into(),
        sku: Some(LoadBalancerSku {
            name: Some("Standard".into()),
            tier: Some("Regional".into()),
        }),
        properties: Some(LoadBalancerPropertiesFormat {
            frontend_ip_configurations: vec![FrontendIPConfiguration {
                name: Some("frontend".into()),
                properties: Some(FrontendIPConfigurationPropertiesFormat {
                    public_ip_address: Some(SubResource { id: Some(pip_id) }),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    let lb = networking
        .create_load_balancer(RG_NAME, LB_NAME, &lb_request)
        .await
        .expect("create_load_balancer failed");
    println!(
        "  LB created: name={:?}, location={:?}, provisioning_state={:?}",
        lb.name,
        lb.location,
        lb.properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert_eq!(lb.name.as_deref(), Some(LB_NAME));
    assert!(lb.id.is_some(), "LB should have an ARM resource ID");

    // =========================================================================
    // Step 7: Get LB + verify + list
    // =========================================================================
    println!("[7/8] Getting load balancer '{LB_NAME}'...");
    let fetched_lb = networking
        .get_load_balancer(RG_NAME, LB_NAME)
        .await
        .expect("get_load_balancer failed");
    println!(
        "  Got LB: name={:?}, type={:?}, sku={:?}, frontend_ips={}",
        fetched_lb.name,
        fetched_lb.r#type,
        fetched_lb.sku.as_ref().and_then(|s| s.name.as_deref()),
        fetched_lb
            .properties
            .as_ref()
            .map(|p| p.frontend_ip_configurations.len())
            .unwrap_or(0),
    );
    assert_eq!(fetched_lb.name.as_deref(), Some(LB_NAME));
    assert!(fetched_lb.id.is_some(), "LB should have an ARM resource ID");
    assert_eq!(
        fetched_lb.sku.as_ref().and_then(|s| s.name.as_deref()),
        Some("Standard"),
    );
    let frontend_ips = fetched_lb
        .properties
        .as_ref()
        .map(|p| &p.frontend_ip_configurations)
        .expect("LB should have properties");
    assert!(
        !frontend_ips.is_empty(),
        "LB should have at least one frontend IP"
    );
    assert_eq!(frontend_ips[0].name.as_deref(), Some("frontend"));

    // List LBs in RG — should include ours
    let list_after = networking
        .list_load_balancers(RG_NAME)
        .await
        .expect("list_load_balancers failed");
    let found = list_after
        .value
        .iter()
        .any(|lb| lb.name.as_deref() == Some(LB_NAME));
    println!(
        "  Found {} LB(s), ours present: {found}",
        list_after.value.len()
    );
    assert!(found, "Our LB should appear in the list");

    // Delete LB via library client
    println!("  Deleting LB '{LB_NAME}'...");
    networking
        .delete_load_balancer(RG_NAME, LB_NAME)
        .await
        .expect("delete_load_balancer failed");
    println!("  LB deleted.");

    // Verify deletion
    let final_list = networking
        .list_load_balancers(RG_NAME)
        .await
        .expect("list_load_balancers failed after delete");
    assert!(
        !final_list
            .value
            .iter()
            .any(|lb| lb.name.as_deref() == Some(LB_NAME)),
        "LB should be gone after delete",
    );

    println!("\nAll LB integration tests passed!");
}
