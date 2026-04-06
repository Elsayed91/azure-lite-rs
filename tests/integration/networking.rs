//! Integration tests for Azure Networking operations.
//!
//! These tests create real Azure resources and MUST clean up after themselves.
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID.
//!
//! Run: cargo test -p azure-lite --test integration networking -- --ignored --test-threads=1 --nocapture

use azure_lite::types::networking::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const VNET_NAME: &str = "cloud-lite-test-ralph-net-vnet";
const SUBNET_NAME: &str = "default";
const NSG_NAME: &str = "cloud-lite-test-ralph-net-nsg";
const RULE_NAME: &str = "cloud-lite-test-ralph-net-rule";

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

async fn networking_cleanup() {
    // Delete security rule first (part of NSG)
    println!("  Cleaning up test security rule...");
    az_delete_ignore(&[
        "network",
        "nsg",
        "rule",
        "delete",
        "--resource-group",
        RG_NAME,
        "--nsg-name",
        NSG_NAME,
        "--name",
        RULE_NAME,
    ])
    .await;

    // Delete NSG
    println!("  Cleaning up test NSG...");
    az_delete_ignore(&[
        "network",
        "nsg",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        NSG_NAME,
    ])
    .await;

    // Delete VNet (and all subnets within it)
    println!("  Cleaning up test VNet...");
    az_delete_ignore(&[
        "network",
        "vnet",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        VNET_NAME,
    ])
    .await;
}

// ============================================================================
// VNet + NSG Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn networking_lifecycle() {
    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/9] Pre-cleanup: removing any leftover networking test resources...");
    networking_cleanup().await;

    let result = std::panic::AssertUnwindSafe(async {
        networking_lifecycle_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    // =========================================================================
    // Step 9: Always cleanup
    // =========================================================================
    println!("\n[9/9] Final networking cleanup...");
    networking_cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn networking_lifecycle_inner() {
    // =========================================================================
    // Step 2: Ensure resource group exists
    // =========================================================================
    println!("[2/9] Ensuring resource group '{RG_NAME}' exists in '{LOCATION}'...");
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
    println!("[3/9] Building AzureHttpClient...");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build client from env");
    let networking = client.networking();

    // =========================================================================
    // Step 4: List VNets (should not contain ours yet)
    // =========================================================================
    println!("[4/9] Listing VNets in resource group '{RG_NAME}'...");
    let list_result = networking
        .list_vnets(RG_NAME)
        .await
        .expect("list_vnets failed");
    println!(
        "  Found {} VNet(s) before create. next_link={}",
        list_result.value.len(),
        list_result.next_link.is_some(),
    );
    assert!(
        !list_result
            .value
            .iter()
            .any(|v| v.name.as_deref() == Some(VNET_NAME)),
        "Test VNet should not exist before create",
    );

    // Also test list_vnets_all (subscription-level)
    let all_vnets = networking
        .list_vnets_all()
        .await
        .expect("list_vnets_all failed");
    println!("  Subscription-level VNet count: {}", all_vnets.value.len());

    // =========================================================================
    // Step 5: Create VNet via library client
    // =========================================================================
    println!("[5/9] Creating VNet '{VNET_NAME}' with subnet '{SUBNET_NAME}'...");
    let create_request = VirtualNetworkCreateRequest {
        location: LOCATION.into(),
        properties: Some(VirtualNetworkPropertiesFormat {
            address_space: Some(AddressSpace {
                address_prefixes: vec!["10.2.0.0/16".into()],
            }),
            subnets: vec![Subnet {
                name: Some(SUBNET_NAME.into()),
                properties: Some(SubnetPropertiesFormat {
                    address_prefix: Some("10.2.0.0/24".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    let vnet = networking
        .create_vnet(RG_NAME, VNET_NAME, &create_request)
        .await
        .expect("create_vnet failed");
    println!(
        "  VNet created: name={:?}, location={:?}, provisioning_state={:?}",
        vnet.name,
        vnet.location,
        vnet.properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert_eq!(vnet.name.as_deref(), Some(VNET_NAME));
    assert!(vnet.id.is_some(), "VNet should have an ARM resource ID");

    // =========================================================================
    // Step 6: Get VNet + verify subnet
    // =========================================================================
    println!("[6/9] Getting VNet '{VNET_NAME}'...");
    let fetched = networking
        .get_vnet(RG_NAME, VNET_NAME)
        .await
        .expect("get_vnet failed");
    println!(
        "  Got VNet: name={:?}, type={:?}, subnets={}",
        fetched.name,
        fetched.r#type,
        fetched
            .properties
            .as_ref()
            .map(|p| p.subnets.len())
            .unwrap_or(0),
    );
    assert_eq!(fetched.name.as_deref(), Some(VNET_NAME));
    assert!(fetched.id.is_some(), "VNet should have an ARM resource ID");
    let subnets = fetched
        .properties
        .as_ref()
        .map(|p| &p.subnets)
        .expect("VNet should have properties");
    assert!(!subnets.is_empty(), "VNet should have at least one subnet");

    // List subnets via networking API
    println!("  Listing subnets in VNet '{VNET_NAME}'...");
    let subnet_list = networking
        .list_subnets(RG_NAME, VNET_NAME)
        .await
        .expect("list_subnets failed");
    println!("  Found {} subnet(s)", subnet_list.value.len());
    assert!(
        !subnet_list.value.is_empty(),
        "VNet should have at least one subnet after creation",
    );

    // Get subnet
    let subnet = networking
        .get_subnet(RG_NAME, VNET_NAME, SUBNET_NAME)
        .await
        .expect("get_subnet failed");
    println!(
        "  Got subnet: name={:?}, address_prefix={:?}",
        subnet.name,
        subnet
            .properties
            .as_ref()
            .and_then(|p| p.address_prefix.as_deref()),
    );
    assert_eq!(subnet.name.as_deref(), Some(SUBNET_NAME));
    assert_eq!(
        subnet
            .properties
            .as_ref()
            .and_then(|p| p.address_prefix.as_deref()),
        Some("10.2.0.0/24"),
    );

    // List VNets again — should include ours
    println!("  Listing VNets again (should include ours)...");
    let list_result2 = networking
        .list_vnets(RG_NAME)
        .await
        .expect("list_vnets failed");
    let found = list_result2
        .value
        .iter()
        .any(|v| v.name.as_deref() == Some(VNET_NAME));
    println!(
        "  Found {} VNet(s), ours present: {found}",
        list_result2.value.len()
    );
    assert!(found, "Our VNet should appear in the list");

    // =========================================================================
    // Step 7: NSG lifecycle (create, get, list, security rules)
    // =========================================================================
    println!("[7/9] Creating NSG '{NSG_NAME}'...");
    let nsg_request = NetworkSecurityGroupCreateRequest {
        location: LOCATION.into(),
        ..Default::default()
    };

    let nsg = networking
        .create_nsg(RG_NAME, NSG_NAME, &nsg_request)
        .await
        .expect("create_nsg failed");
    println!(
        "  NSG created: name={:?}, location={:?}, provisioning_state={:?}",
        nsg.name,
        nsg.location,
        nsg.properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert_eq!(nsg.name.as_deref(), Some(NSG_NAME));
    assert!(nsg.id.is_some(), "NSG should have an ARM resource ID");

    // Get NSG
    let fetched_nsg = networking
        .get_nsg(RG_NAME, NSG_NAME)
        .await
        .expect("get_nsg failed");
    println!(
        "  Got NSG: name={:?}, type={:?}, default_rules={}",
        fetched_nsg.name,
        fetched_nsg.r#type,
        fetched_nsg
            .properties
            .as_ref()
            .map(|p| p.default_security_rules.len())
            .unwrap_or(0),
    );
    assert_eq!(fetched_nsg.name.as_deref(), Some(NSG_NAME));

    // List NSGs in RG — should include ours
    let nsg_list = networking
        .list_nsgs(RG_NAME)
        .await
        .expect("list_nsgs failed");
    let found = nsg_list
        .value
        .iter()
        .any(|n| n.name.as_deref() == Some(NSG_NAME));
    println!(
        "  Found {} NSG(s), ours present: {found}",
        nsg_list.value.len()
    );
    assert!(found, "Our NSG should appear in the list");

    // List NSGs at subscription level
    let all_nsgs = networking
        .list_nsgs_all()
        .await
        .expect("list_nsgs_all failed");
    println!("  Subscription-level NSG count: {}", all_nsgs.value.len());
    assert!(
        all_nsgs
            .value
            .iter()
            .any(|n| n.name.as_deref() == Some(NSG_NAME)),
        "Our NSG should appear in subscription-level list",
    );

    // List security rules (initially empty custom rules, but default rules exist)
    let rules = networking
        .list_security_rules(RG_NAME, NSG_NAME)
        .await
        .expect("list_security_rules failed");
    println!("  Custom security rules count: {}", rules.value.len());

    // Create a custom security rule
    println!("  Creating security rule '{RULE_NAME}'...");
    let rule_body = SecurityRule {
        name: Some(RULE_NAME.into()),
        properties: Some(SecurityRulePropertiesFormat {
            protocol: Some("Tcp".into()),
            source_port_range: Some("*".into()),
            destination_port_range: Some("8080".into()),
            source_address_prefix: Some("*".into()),
            destination_address_prefix: Some("*".into()),
            access: Some("Allow".into()),
            priority: Some(200),
            direction: Some("Inbound".into()),
            description: Some("Allow HTTP on 8080 for testing".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let created_rule = networking
        .create_security_rule(RG_NAME, NSG_NAME, RULE_NAME, &rule_body)
        .await
        .expect("create_security_rule failed");
    println!(
        "  Security rule created: name={:?}, priority={:?}, access={:?}",
        created_rule.name,
        created_rule.properties.as_ref().and_then(|p| p.priority),
        created_rule
            .properties
            .as_ref()
            .and_then(|p| p.access.as_deref()),
    );
    assert_eq!(created_rule.name.as_deref(), Some(RULE_NAME));
    assert_eq!(
        created_rule.properties.as_ref().and_then(|p| p.priority),
        Some(200),
    );
    assert_eq!(
        created_rule
            .properties
            .as_ref()
            .and_then(|p| p.access.as_deref()),
        Some("Allow"),
    );

    // Get the security rule
    let fetched_rule = networking
        .get_security_rule(RG_NAME, NSG_NAME, RULE_NAME)
        .await
        .expect("get_security_rule failed");
    println!(
        "  Got rule: name={:?}, direction={:?}, dest_port={:?}",
        fetched_rule.name,
        fetched_rule
            .properties
            .as_ref()
            .and_then(|p| p.direction.as_deref()),
        fetched_rule
            .properties
            .as_ref()
            .and_then(|p| p.destination_port_range.as_deref()),
    );
    assert_eq!(fetched_rule.name.as_deref(), Some(RULE_NAME));
    assert_eq!(
        fetched_rule
            .properties
            .as_ref()
            .and_then(|p| p.destination_port_range.as_deref()),
        Some("8080"),
    );

    // List security rules again — should include ours
    let rules_after = networking
        .list_security_rules(RG_NAME, NSG_NAME)
        .await
        .expect("list_security_rules failed");
    let rule_found = rules_after
        .value
        .iter()
        .any(|r| r.name.as_deref() == Some(RULE_NAME));
    println!(
        "  Security rules after create: {}, our rule present: {rule_found}",
        rules_after.value.len(),
    );
    assert!(
        rule_found,
        "Our custom security rule should appear in the list"
    );

    // Delete the security rule
    println!("  Deleting security rule '{RULE_NAME}'...");
    networking
        .delete_security_rule(RG_NAME, NSG_NAME, RULE_NAME)
        .await
        .expect("delete_security_rule failed");
    println!("  Security rule deleted.");

    // =========================================================================
    // Step 8: Delete NSG + VNet
    // =========================================================================
    println!("[8/9] Deleting NSG '{NSG_NAME}'...");
    networking
        .delete_nsg(RG_NAME, NSG_NAME)
        .await
        .expect("delete_nsg failed");
    println!("  NSG deleted.");

    println!("  Deleting VNet '{VNET_NAME}'...");
    networking
        .delete_vnet(RG_NAME, VNET_NAME)
        .await
        .expect("delete_vnet failed");
    println!("  VNet deleted.");

    // Verify deletion: list VNets should no longer contain ours
    let final_list = networking
        .list_vnets(RG_NAME)
        .await
        .expect("list_vnets failed after delete");
    let still_exists = final_list
        .value
        .iter()
        .any(|v| v.name.as_deref() == Some(VNET_NAME));
    assert!(!still_exists, "VNet should be gone after delete");

    println!("\nAll networking integration tests passed!");
}

// ============================================================================
// Network Interface Delete Integration Test
// ============================================================================

const NIC_VNET_NAME: &str = "cloud-lite-test-delnic-vnet";
const NIC_SUBNET_NAME: &str = "default";
const NIC_NAME: &str = "cloud-lite-test-delnic-nic";

async fn nic_cleanup() {
    println!("  Cleaning up NIC test resources...");
    az_delete_ignore(&[
        "network",
        "nic",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        NIC_NAME,
    ])
    .await;
    az_delete_ignore(&[
        "network",
        "vnet",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        NIC_VNET_NAME,
    ])
    .await;
}

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn delete_network_interface_lifecycle() {
    // Step 1: Pre-cleanup
    println!("[1/6] Pre-cleanup: removing any leftover NIC test resources...");
    nic_cleanup().await;

    let result = std::panic::AssertUnwindSafe(async {
        delete_network_interface_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    // Step 6: Always cleanup
    println!("\n[6/6] Final NIC cleanup...");
    nic_cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn delete_network_interface_inner() {
    // Step 2: Ensure resource group
    println!("[2/6] Ensuring resource group '{RG_NAME}' exists...");
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

    // Step 3: Create VNet + subnet via CLI (NIC needs a subnet)
    println!("[3/6] Creating VNet '{NIC_VNET_NAME}' with subnet '{NIC_SUBNET_NAME}' via CLI...");
    az(&[
        "network",
        "vnet",
        "create",
        "--resource-group",
        RG_NAME,
        "--name",
        NIC_VNET_NAME,
        "--address-prefix",
        "10.3.0.0/16",
        "--subnet-name",
        NIC_SUBNET_NAME,
        "--subnet-prefix",
        "10.3.0.0/24",
        "--output",
        "json",
    ])
    .await;

    // Step 4: Create NIC via CLI
    println!("[4/6] Creating NIC '{NIC_NAME}' via CLI...");
    az(&[
        "network",
        "nic",
        "create",
        "--resource-group",
        RG_NAME,
        "--name",
        NIC_NAME,
        "--vnet-name",
        NIC_VNET_NAME,
        "--subnet",
        NIC_SUBNET_NAME,
        "--output",
        "json",
    ])
    .await;

    // Step 5: Delete NIC via library client
    println!("[5/6] Deleting NIC '{NIC_NAME}' via library client...");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build client from env");
    let networking = client.networking();

    networking
        .delete_network_interface(RG_NAME, NIC_NAME)
        .await
        .expect("delete_network_interface failed");
    println!("  NIC deleted successfully via library client.");

    // Verify: az CLI should report not found
    println!("  Verifying NIC is gone...");
    let check = az(&[
        "network",
        "nic",
        "show",
        "--resource-group",
        RG_NAME,
        "--name",
        NIC_NAME,
        "--output",
        "json",
    ])
    .await;
    assert!(check.is_empty(), "NIC should be gone after delete");
    println!("  Verified: NIC no longer exists.");
}

// ============================================================================
// NAT Gateway Delete Integration Test
// ============================================================================

const NATGW_NAME: &str = "cloud-lite-test-delnatgw-natgw";

async fn natgw_cleanup() {
    println!("  Cleaning up NAT Gateway test resources...");
    az_delete_ignore(&[
        "network",
        "nat",
        "gateway",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        NATGW_NAME,
    ])
    .await;
}

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn delete_nat_gateway_lifecycle() {
    // Step 1: Pre-cleanup
    println!("[1/5] Pre-cleanup: removing any leftover NAT Gateway test resources...");
    natgw_cleanup().await;

    let result = std::panic::AssertUnwindSafe(async {
        delete_nat_gateway_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    // Step 5: Always cleanup
    println!("\n[5/5] Final NAT Gateway cleanup...");
    natgw_cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn delete_nat_gateway_inner() {
    // Step 2: Ensure resource group
    println!("[2/5] Ensuring resource group '{RG_NAME}' exists...");
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

    // Step 3: Create NAT Gateway via CLI
    println!("[3/5] Creating NAT Gateway '{NATGW_NAME}' via CLI...");
    az(&[
        "network",
        "nat",
        "gateway",
        "create",
        "--resource-group",
        RG_NAME,
        "--name",
        NATGW_NAME,
        "--location",
        LOCATION,
        "--idle-timeout",
        "4",
        "--output",
        "json",
    ])
    .await;

    // Step 4: Delete NAT Gateway via library client
    println!("[4/5] Deleting NAT Gateway '{NATGW_NAME}' via library client...");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build client from env");
    let networking = client.networking();

    networking
        .delete_nat_gateway(RG_NAME, NATGW_NAME)
        .await
        .expect("delete_nat_gateway failed");
    println!("  NAT Gateway deleted successfully via library client.");

    // Verify: az CLI should report not found
    println!("  Verifying NAT Gateway is gone...");
    let check = az(&[
        "network",
        "nat",
        "gateway",
        "show",
        "--resource-group",
        RG_NAME,
        "--name",
        NATGW_NAME,
        "--output",
        "json",
    ])
    .await;
    assert!(check.is_empty(), "NAT Gateway should be gone after delete");
    println!("  Verified: NAT Gateway no longer exists.");
}
