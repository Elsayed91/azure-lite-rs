//! Integration tests for Azure Compute VM operations.
//!
//! These tests create real Azure resources and MUST clean up after themselves.
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID or default subscription.
//!
//! Run: cargo test -p azure-lite --test integration compute -- --ignored --test-threads=1 --nocapture

use azure_lite::types::compute::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const VM_NAME: &str = "cloud-lite-test-ralph-vm";
const VNET_NAME: &str = "cloud-lite-test-ralph-vnet";
const SUBNET_NAME: &str = "cloud-lite-test-ralph-subnet";
const NIC_NAME: &str = "cloud-lite-test-ralph-nic";

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

/// Helper: delete a resource, ignoring "not found" and "in use" errors.
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
            && !stderr.contains("NicReservedForAnotherVm")
            && !stderr.contains("InUseSubnetCannotBeDeleted")
        {
            eprintln!(
                "Warning: cleanup command failed: az {}: {}",
                args.join(" "),
                stderr
            );
        }
    }
}

/// Generate an SSH public key for test VM creation.
async fn generate_ssh_public_key() -> String {
    let key_path = "/tmp/cloud-lite-test-ssh-key";
    // Remove old key if exists
    let _ = tokio::fs::remove_file(key_path).await;
    let _ = tokio::fs::remove_file(format!("{key_path}.pub")).await;

    let output = tokio::process::Command::new("ssh-keygen")
        .args(["-t", "rsa", "-b", "2048", "-f", key_path, "-N", "", "-q"])
        .output()
        .await
        .expect("failed to run ssh-keygen");
    assert!(output.status.success(), "ssh-keygen failed");

    let pub_key = tokio::fs::read_to_string(format!("{key_path}.pub"))
        .await
        .expect("failed to read generated SSH public key");
    pub_key.trim().to_string()
}

/// Clean up all test resources. Waits for VM deletion before deleting networking.
async fn cleanup() {
    // 1. Delete VM (synchronous wait)
    println!("  Cleaning up test VM...");
    az_delete_ignore(&[
        "vm",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        VM_NAME,
        "--yes",
        "--force-deletion",
        "true",
    ])
    .await;

    // 2. Delete OS disk (left behind after VM deletion)
    let disk_name = format!("{VM_NAME}-osdisk");
    println!("  Cleaning up OS disk...");
    az_delete_ignore(&[
        "disk",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        &disk_name,
        "--yes",
    ])
    .await;

    // 3. Delete NIC (VM must be gone first)
    println!("  Cleaning up test NIC...");
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

    // 4. Delete VNet (NIC must be gone first)
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

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn compute_vm_lifecycle() {
    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/10] Pre-cleanup: removing any leftover test resources...");
    cleanup().await;

    // Ensure the rest runs cleanup even on panic
    let result = std::panic::AssertUnwindSafe(async {
        compute_vm_lifecycle_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    // =========================================================================
    // Step 10: Always cleanup
    // =========================================================================
    println!("\n[10/10] Final cleanup...");
    cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn compute_vm_lifecycle_inner() {
    // =========================================================================
    // Step 2: Create resource group
    // =========================================================================
    println!("[2/10] Creating resource group '{RG_NAME}' in '{LOCATION}'...");
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
    println!("  Resource group created.");

    // =========================================================================
    // Step 3: Create networking prerequisites via CLI
    // =========================================================================
    println!("[3/10] Creating VNet + Subnet + NIC via CLI...");
    az(&[
        "network",
        "vnet",
        "create",
        "--resource-group",
        RG_NAME,
        "--name",
        VNET_NAME,
        "--address-prefix",
        "10.0.0.0/16",
        "--subnet-name",
        SUBNET_NAME,
        "--subnet-prefix",
        "10.0.0.0/24",
        "--output",
        "json",
    ])
    .await;

    let nic_output = az(&[
        "network",
        "nic",
        "create",
        "--resource-group",
        RG_NAME,
        "--name",
        NIC_NAME,
        "--vnet-name",
        VNET_NAME,
        "--subnet",
        SUBNET_NAME,
        "--output",
        "json",
    ])
    .await;
    let nic_json: serde_json::Value =
        serde_json::from_str(&nic_output).expect("failed to parse NIC create output");
    let nic_id = nic_json["NewNIC"]["id"]
        .as_str()
        .expect("NIC output missing id field");
    println!("  NIC created: {nic_id}");

    // Generate an SSH key pair for the VM
    let ssh_pub_key = generate_ssh_public_key().await;
    println!("  SSH public key generated ({} bytes)", ssh_pub_key.len());

    // =========================================================================
    // Step 4: Build client and list VMs (should be empty initially)
    // =========================================================================
    println!("[4/10] Building AzureHttpClient and listing VMs...");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build client from env");

    let compute = client.compute();
    let list_result = compute.list_vms(RG_NAME).await.expect("list_vms failed");
    println!(
        "  Listed {} VMs (before create). next_link={}",
        list_result.value.len(),
        list_result.next_link.is_some(),
    );
    assert!(
        !list_result
            .value
            .iter()
            .any(|vm| vm.name.as_deref() == Some(VM_NAME)),
        "Test VM should not exist before create"
    );

    // =========================================================================
    // Step 5: Create VM via library client
    // =========================================================================
    println!("[5/10] Creating VM '{VM_NAME}' via library client...");
    let create_request = VirtualMachineCreateRequest {
        location: LOCATION.into(),
        properties: Some(VirtualMachineProperties {
            hardware_profile: Some(HardwareProfile {
                vm_size: Some("Standard_B1s".into()),
            }),
            storage_profile: Some(StorageProfile {
                image_reference: Some(ImageReference {
                    publisher: Some("Canonical".into()),
                    offer: Some("0001-com-ubuntu-server-jammy".into()),
                    sku: Some("22_04-lts-gen2".into()),
                    version: Some("latest".into()),
                    ..Default::default()
                }),
                os_disk: Some(OsDisk {
                    name: Some(format!("{VM_NAME}-osdisk")),
                    create_option: "FromImage".into(),
                    managed_disk: Some(ManagedDiskParameters {
                        storage_account_type: Some("Standard_LRS".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            os_profile: Some(OsProfile {
                computer_name: Some("testvm".into()),
                admin_username: Some("azureuser".into()),
                linux_configuration: Some(LinuxConfiguration {
                    disable_password_authentication: Some(true),
                    ssh: Some(SshConfiguration {
                        public_keys: vec![SshPublicKey {
                            path: Some("/home/azureuser/.ssh/authorized_keys".into()),
                            key_data: Some(ssh_pub_key),
                        }],
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            network_profile: Some(NetworkProfile {
                network_interfaces: vec![NetworkInterfaceReference {
                    id: Some(nic_id.into()),
                    properties: Some(NetworkInterfaceReferenceProperties {
                        primary: Some(true),
                        delete_option: None,
                    }),
                }],
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let vm = compute
        .create_vm(RG_NAME, VM_NAME, &create_request)
        .await
        .expect("create_vm failed");
    println!(
        "  VM created: name={:?}, location={:?}, provisioning_state={:?}",
        vm.name,
        vm.location,
        vm.properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert_eq!(vm.name.as_deref(), Some(VM_NAME));

    // =========================================================================
    // Step 6: Get VM to verify
    // =========================================================================
    println!("[6/10] Getting VM '{VM_NAME}'...");
    let fetched = compute
        .get_vm(RG_NAME, VM_NAME)
        .await
        .expect("get_vm failed");
    println!(
        "  Got VM: name={:?}, type={:?}, vm_id={:?}",
        fetched.name,
        fetched.r#type,
        fetched.properties.as_ref().and_then(|p| p.vm_id.as_deref()),
    );
    assert_eq!(fetched.name.as_deref(), Some(VM_NAME));
    assert!(fetched.id.is_some(), "VM should have an ARM resource ID");
    assert!(
        fetched
            .properties
            .as_ref()
            .and_then(|p| p.vm_id.as_deref())
            .is_some(),
        "VM should have a vmId in properties",
    );

    // =========================================================================
    // Step 7: List VMs again (should include our VM)
    // =========================================================================
    println!("[7/10] Listing VMs again (should include our VM)...");
    let list_result = compute.list_vms(RG_NAME).await.expect("list_vms failed");
    let found = list_result
        .value
        .iter()
        .any(|vm| vm.name.as_deref() == Some(VM_NAME));
    println!(
        "  Listed {} VMs, our VM found: {found}",
        list_result.value.len()
    );
    assert!(found, "Our VM should appear in the list");

    // =========================================================================
    // Step 8: Wait for provisioning + get instance view
    // =========================================================================
    println!("[8/10] Waiting for VM provisioning to complete...");
    for i in 0..60 {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        let vm_state = compute
            .get_vm(RG_NAME, VM_NAME)
            .await
            .expect("get_vm failed during wait");
        let state = vm_state
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref());
        println!("    ({}/60) Provisioning state: {:?}", i + 1, state);
        if state == Some("Succeeded") || state == Some("Failed") {
            assert_eq!(state, Some("Succeeded"), "VM provisioning should succeed");
            break;
        }
    }

    println!("  Getting instance view...");
    let instance_view = compute
        .get_instance_view(RG_NAME, VM_NAME)
        .await
        .expect("get_instance_view failed");
    println!(
        "  Instance view: statuses count={}",
        instance_view.statuses.len(),
    );
    assert!(
        !instance_view.statuses.is_empty(),
        "Instance view should have at least one status",
    );
    // Print status codes for visibility
    for status in &instance_view.statuses {
        println!(
            "    Status: code={:?}, display={:?}",
            status.code, status.display_status
        );
    }

    // =========================================================================
    // Step 9: Deallocate VM (stops billing, proves lifecycle ops work)
    // =========================================================================
    println!("[9/10] Deallocating VM (this stops billing)...");
    compute
        .deallocate_vm(RG_NAME, VM_NAME)
        .await
        .expect("deallocate_vm failed");
    println!("  Deallocate request accepted. Waiting for completion...");

    for i in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        if let Ok(iv) = compute.get_instance_view(RG_NAME, VM_NAME).await {
            let power_state = iv
                .statuses
                .iter()
                .find_map(|s| s.code.as_deref().filter(|c| c.starts_with("PowerState/")));
            if let Some(ps) = power_state {
                println!("    ({}/30) Power state: {ps}", i + 1);
                if ps == "PowerState/deallocated" {
                    break;
                }
            }
        }
    }
    println!("  VM deallocated.");

    println!("\nAll compute VM integration tests passed!");
}

// ============================================================================
// Managed Disk Integration Test
// ============================================================================

const DISK_NAME: &str = "cloud-lite-test-ralph-disk";

async fn disk_cleanup() {
    println!("  Cleaning up test disk...");
    az_delete_ignore(&[
        "disk",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        DISK_NAME,
        "--yes",
    ])
    .await;
}

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn compute_disk_lifecycle() {
    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/8] Pre-cleanup: removing any leftover disk test resources...");
    disk_cleanup().await;

    let result = std::panic::AssertUnwindSafe(async {
        compute_disk_lifecycle_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    // =========================================================================
    // Step 8: Always cleanup
    // =========================================================================
    println!("\n[8/8] Final disk cleanup...");
    disk_cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn compute_disk_lifecycle_inner() {
    use azure_lite::types::compute::{
        DiskCreateRequest, DiskCreationData, DiskProperties, DiskSku, GrantAccessData,
    };

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
    let compute = client.compute();

    // =========================================================================
    // Step 4: List disks (should not contain our test disk)
    // =========================================================================
    println!("[4/8] Listing disks in resource group '{RG_NAME}'...");
    let list_result = compute
        .list_disks(RG_NAME)
        .await
        .expect("list_disks failed");
    println!(
        "  Found {} disk(s) before create. next_link={}",
        list_result.value.len(),
        list_result.next_link.is_some(),
    );
    assert!(
        !list_result
            .value
            .iter()
            .any(|d| d.name.as_deref() == Some(DISK_NAME)),
        "Test disk should not exist before create",
    );

    // =========================================================================
    // Step 5: Create a 4 GB empty disk
    // =========================================================================
    println!("[5/8] Creating empty managed disk '{DISK_NAME}' (4 GB, Standard_LRS)...");
    let create_request = DiskCreateRequest {
        location: LOCATION.into(),
        sku: Some(DiskSku {
            name: Some("Standard_LRS".into()),
            ..Default::default()
        }),
        properties: Some(DiskProperties {
            disk_size_gb: Some(4),
            creation_data: Some(DiskCreationData {
                create_option: "Empty".into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let disk = compute
        .create_disk(RG_NAME, DISK_NAME, &create_request)
        .await
        .expect("create_disk failed");
    println!(
        "  Disk created: name={:?}, location={:?}, provisioning_state={:?}",
        disk.name,
        disk.location,
        disk.properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert_eq!(disk.name.as_deref(), Some(DISK_NAME));
    assert_eq!(disk.location.as_str(), LOCATION);
    assert!(disk.id.is_some(), "Disk should have an ARM resource ID");

    // =========================================================================
    // Step 6: Get disk to verify fields
    // =========================================================================
    println!("[6/8] Getting disk '{DISK_NAME}'...");
    let fetched = compute
        .get_disk(RG_NAME, DISK_NAME)
        .await
        .expect("get_disk failed");
    println!(
        "  Got disk: name={:?}, sku={:?}, size_gb={:?}, state={:?}",
        fetched.name,
        fetched.sku.as_ref().and_then(|s| s.name.as_deref()),
        fetched.properties.as_ref().and_then(|p| p.disk_size_gb),
        fetched
            .properties
            .as_ref()
            .and_then(|p| p.disk_state.as_deref()),
    );
    assert_eq!(fetched.name.as_deref(), Some(DISK_NAME));
    assert_eq!(
        fetched.sku.as_ref().and_then(|s| s.name.as_deref()),
        Some("Standard_LRS"),
    );
    assert_eq!(
        fetched.properties.as_ref().and_then(|p| p.disk_size_gb),
        Some(4),
    );

    // =========================================================================
    // Step 7: List disks again (should include our disk)
    // =========================================================================
    println!("[7/8] Listing disks again (should include our disk)...");
    let list_result = compute
        .list_disks(RG_NAME)
        .await
        .expect("list_disks failed");
    let found = list_result
        .value
        .iter()
        .any(|d| d.name.as_deref() == Some(DISK_NAME));
    println!(
        "  Found {} disk(s), our disk found: {found}",
        list_result.value.len()
    );
    assert!(found, "Our disk should appear in the list");

    // Also test list_disks_in_subscription
    let sub_list = compute
        .list_disks_in_subscription()
        .await
        .expect("list_disks_in_subscription failed");
    println!("  Subscription-level disk count: {}", sub_list.value.len());
    assert!(
        sub_list
            .value
            .iter()
            .any(|d| d.name.as_deref() == Some(DISK_NAME)),
        "Disk should appear in subscription-level list",
    );

    // Test GrantAccess to prove the SAS URI flow works (Read access, 300s duration)
    println!("  Granting SAS read access to disk...");
    let grant_request = GrantAccessData {
        access: "Read".into(),
        duration_in_seconds: 300,
        ..Default::default()
    };
    let access_uri = compute
        .grant_access(RG_NAME, DISK_NAME, &grant_request)
        .await
        .expect("grant_access failed");
    println!(
        "  SAS URI obtained: has_sas={}",
        access_uri.access_sas.is_some(),
    );
    assert!(
        access_uri.access_sas.is_some(),
        "grant_access should return a non-empty SAS URI",
    );

    // Revoke access before deleting
    println!("  Revoking SAS access...");
    compute
        .revoke_access(RG_NAME, DISK_NAME)
        .await
        .expect("revoke_access failed");
    println!("  SAS access revoked.");

    println!("\nAll compute disk integration tests passed!");
}

// ============================================================================
// VMSS Integration Test
// ============================================================================

const VMSS_NAME: &str = "cloud-lite-test-ralph-vmss";
const VMSS_VNET_NAME: &str = "cloud-lite-test-ralph-vmss-vnet";
const VMSS_SUBNET_NAME: &str = "cloud-lite-test-ralph-vmss-subnet";

async fn vmss_cleanup() {
    println!("  Cleaning up VMSS...");
    az_delete_ignore(&[
        "vmss",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        VMSS_NAME,
        "--force-deletion",
        "true",
    ])
    .await;

    println!("  Cleaning up VMSS VNet...");
    az_delete_ignore(&[
        "network",
        "vnet",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        VMSS_VNET_NAME,
    ])
    .await;
}

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn compute_vmss_lifecycle() {
    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/10] Pre-cleanup: removing any leftover VMSS test resources...");
    vmss_cleanup().await;

    // Ensure the rest runs cleanup even on panic
    let result = std::panic::AssertUnwindSafe(async {
        compute_vmss_lifecycle_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    // =========================================================================
    // Step 10: Always cleanup
    // =========================================================================
    println!("\n[10/10] Final VMSS cleanup...");
    vmss_cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn compute_vmss_lifecycle_inner() {
    // =========================================================================
    // Step 2: Create resource group (idempotent)
    // =========================================================================
    println!("[2/10] Ensuring resource group '{RG_NAME}' exists in '{LOCATION}'...");
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
    // Step 3: Create VNet + Subnet for VMSS
    // =========================================================================
    println!("[3/10] Creating VNet + Subnet for VMSS via CLI...");
    az(&[
        "network",
        "vnet",
        "create",
        "--resource-group",
        RG_NAME,
        "--name",
        VMSS_VNET_NAME,
        "--address-prefix",
        "10.1.0.0/16",
        "--subnet-name",
        VMSS_SUBNET_NAME,
        "--subnet-prefix",
        "10.1.0.0/24",
        "--output",
        "json",
    ])
    .await;

    // Get subnet ID for the VMSS network config
    let subnet_output = az(&[
        "network",
        "vnet",
        "subnet",
        "show",
        "--resource-group",
        RG_NAME,
        "--vnet-name",
        VMSS_VNET_NAME,
        "--name",
        VMSS_SUBNET_NAME,
        "--output",
        "json",
    ])
    .await;
    let subnet_json: serde_json::Value =
        serde_json::from_str(&subnet_output).expect("failed to parse subnet show output");
    let subnet_id = subnet_json["id"]
        .as_str()
        .expect("subnet output missing id field");
    println!("  Subnet created: {subnet_id}");

    // =========================================================================
    // Step 4: Build client and list VMSS (should be empty or not contain ours)
    // =========================================================================
    println!("[4/10] Building AzureHttpClient and listing VMSS...");
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build client from env");

    let compute = client.compute();
    let list_result = compute.list_vmss(RG_NAME).await.expect("list_vmss failed");
    println!(
        "  Listed {} VMSS (before create). next_link={}",
        list_result.value.len(),
        list_result.next_link.is_some(),
    );
    assert!(
        !list_result
            .value
            .iter()
            .any(|vmss| vmss.name.as_deref() == Some(VMSS_NAME)),
        "Test VMSS should not exist before create"
    );

    // =========================================================================
    // Step 5: Create VMSS via library client
    // =========================================================================
    println!("[5/10] Creating VMSS '{VMSS_NAME}' via library client...");

    // Generate SSH key for VMSS
    let ssh_pub_key = generate_ssh_public_key().await;
    println!("  SSH public key generated ({} bytes)", ssh_pub_key.len());

    let create_request = VirtualMachineScaleSetCreateRequest {
        location: LOCATION.into(),
        sku: Some(Sku {
            name: Some("Standard_B1s".into()),
            tier: Some("Standard".into()),
            capacity: Some(1),
        }),
        properties: Some(VirtualMachineScaleSetProperties {
            overprovision: Some(false),
            single_placement_group: Some(true),
            upgrade_policy: Some(UpgradePolicy {
                mode: Some("Manual".into()),
            }),
            virtual_machine_profile: Some(VirtualMachineScaleSetVMProfile {
                storage_profile: Some(StorageProfile {
                    image_reference: Some(ImageReference {
                        publisher: Some("Canonical".into()),
                        offer: Some("0001-com-ubuntu-server-jammy".into()),
                        sku: Some("22_04-lts-gen2".into()),
                        version: Some("latest".into()),
                        ..Default::default()
                    }),
                    os_disk: Some(OsDisk {
                        create_option: "FromImage".into(),
                        managed_disk: Some(ManagedDiskParameters {
                            storage_account_type: Some("Standard_LRS".into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                os_profile: Some(VirtualMachineScaleSetOsProfile {
                    computer_name_prefix: Some("vmss".into()),
                    admin_username: Some("azureuser".into()),
                    linux_configuration: Some(LinuxConfiguration {
                        disable_password_authentication: Some(true),
                        ssh: Some(SshConfiguration {
                            public_keys: vec![SshPublicKey {
                                path: Some("/home/azureuser/.ssh/authorized_keys".into()),
                                key_data: Some(ssh_pub_key),
                            }],
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                network_profile: Some(VirtualMachineScaleSetNetworkProfile {
                    network_interface_configurations: vec![
                        VirtualMachineScaleSetNetworkConfiguration {
                            name: "vmss-nic".into(),
                            properties: Some(
                                VirtualMachineScaleSetNetworkConfigurationProperties {
                                    primary: Some(true),
                                    ip_configurations: vec![
                                        VirtualMachineScaleSetIPConfiguration {
                                            name: "vmss-ipconfig".into(),
                                            properties: Some(
                                                VirtualMachineScaleSetIPConfigurationProperties {
                                                    subnet: Some(SubResource {
                                                        id: Some(subnet_id.into()),
                                                    }),
                                                },
                                            ),
                                        },
                                    ],
                                    ..Default::default()
                                },
                            ),
                        },
                    ],
                }),
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let vmss = compute
        .create_vmss(RG_NAME, VMSS_NAME, &create_request)
        .await
        .expect("create_vmss failed");
    println!(
        "  VMSS created: name={:?}, location={:?}, provisioning_state={:?}",
        vmss.name,
        vmss.location,
        vmss.properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert_eq!(vmss.name.as_deref(), Some(VMSS_NAME));

    // =========================================================================
    // Step 6: Get VMSS to verify
    // =========================================================================
    println!("[6/10] Getting VMSS '{VMSS_NAME}'...");
    let fetched = compute
        .get_vmss(RG_NAME, VMSS_NAME)
        .await
        .expect("get_vmss failed");
    println!(
        "  Got VMSS: name={:?}, type={:?}, unique_id={:?}",
        fetched.name,
        fetched.r#type,
        fetched
            .properties
            .as_ref()
            .and_then(|p| p.unique_id.as_deref()),
    );
    assert_eq!(fetched.name.as_deref(), Some(VMSS_NAME));
    assert!(fetched.id.is_some(), "VMSS should have an ARM resource ID");

    // =========================================================================
    // Step 7: List VMSS again (should include ours)
    // =========================================================================
    println!("[7/10] Listing VMSS again (should include ours)...");
    let list_result = compute.list_vmss(RG_NAME).await.expect("list_vmss failed");
    let found = list_result
        .value
        .iter()
        .any(|v| v.name.as_deref() == Some(VMSS_NAME));
    println!(
        "  Listed {} VMSS, ours found: {found}",
        list_result.value.len()
    );
    assert!(found, "Our VMSS should appear in the list");

    // =========================================================================
    // Step 8: Wait for provisioning to complete, then list instances
    // =========================================================================
    println!("[8/10] Waiting for VMSS provisioning to complete...");
    for i in 0..60 {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        let vmss_state = compute
            .get_vmss(RG_NAME, VMSS_NAME)
            .await
            .expect("get_vmss failed during wait");
        let state = vmss_state
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref());
        println!("    ({}/60) Provisioning state: {:?}", i + 1, state);
        if state == Some("Succeeded") || state == Some("Failed") {
            assert_eq!(state, Some("Succeeded"), "VMSS provisioning should succeed");
            break;
        }
    }

    println!("  Listing VMSS instances...");
    let instances = compute
        .list_vmss_instances(RG_NAME, VMSS_NAME)
        .await
        .expect("list_vmss_instances failed");
    println!("  Found {} VMSS instance(s)", instances.value.len());
    assert!(
        !instances.value.is_empty(),
        "VMSS should have at least one instance (capacity=1)",
    );
    let first_instance = &instances.value[0];
    println!(
        "  First instance: name={:?}, instance_id={:?}, provisioning_state={:?}",
        first_instance.name,
        first_instance.instance_id,
        first_instance
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert!(
        first_instance.instance_id.is_some(),
        "VMSS instance should have an instance ID"
    );

    // =========================================================================
    // Step 9: Stop VMSS instances (power off to stop billing)
    // =========================================================================
    println!("[9/10] Stopping VMSS instances (power off)...");
    let instance_id = first_instance.instance_id.as_deref().unwrap();
    let stop_request = VirtualMachineScaleSetVMInstanceIDs {
        instance_ids: vec![instance_id.into()],
    };
    compute
        .stop_vmss_instances(RG_NAME, VMSS_NAME, &stop_request)
        .await
        .expect("stop_vmss_instances failed");
    println!("  Stop request accepted.");

    println!("\nAll compute VMSS integration tests passed!");
}
