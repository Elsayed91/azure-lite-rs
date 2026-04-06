//! Integration tests for Azure DNS operations.
//!
//! These tests create real Azure resources and MUST clean up after themselves.
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID.
//!
//! Run: cargo test -p azure-lite --test integration dns_lifecycle -- --ignored --test-threads=1 --nocapture

use azure_lite::types::dns::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "global";
const ZONE_NAME: &str = "cloud-lite-test-ralph-dns.example.com";
const RECORD_SET_NAME: &str = "test-a";
const RECORD_TYPE: &str = "A";

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

async fn dns_cleanup() {
    println!("  Cleaning up test record set '{RECORD_SET_NAME}'...");
    az_delete_ignore(&[
        "network",
        "dns",
        "record-set",
        "a",
        "delete",
        "--resource-group",
        RG_NAME,
        "--zone-name",
        ZONE_NAME,
        "--name",
        RECORD_SET_NAME,
        "--yes",
    ])
    .await;

    println!("  Cleaning up test DNS zone '{ZONE_NAME}'...");
    az_delete_ignore(&[
        "network",
        "dns",
        "zone",
        "delete",
        "--resource-group",
        RG_NAME,
        "--name",
        ZONE_NAME,
        "--yes",
    ])
    .await;
}

// ============================================================================
// DNS Zone + Record Set Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn dns_lifecycle() {
    println!("[1/8] Pre-cleanup: removing any leftover DNS test resources...");
    dns_cleanup().await;

    let result = std::panic::AssertUnwindSafe(async {
        dns_lifecycle_inner().await;
    });
    let outcome = tokio::task::spawn(result).await;

    println!("\n[8/8] Final DNS cleanup...");
    dns_cleanup().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn dns_lifecycle_inner() {
    // =========================================================================
    // Step 2: Ensure resource group exists
    // =========================================================================
    println!("[2/8] Ensuring resource group '{RG_NAME}' exists...");
    az(&[
        "group",
        "create",
        "--name",
        RG_NAME,
        "--location",
        "eastus",
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
    let dns = client.dns();

    // =========================================================================
    // Step 4: List DNS zones (should not contain ours yet)
    // =========================================================================
    println!("[4/8] Listing DNS zones in '{RG_NAME}' (before create)...");
    let list_before = dns
        .list_dns_zones(RG_NAME)
        .await
        .expect("list_dns_zones failed");
    println!("  Found {} zone(s) before create.", list_before.value.len());
    assert!(
        !list_before
            .value
            .iter()
            .any(|z| z.name.as_deref() == Some(ZONE_NAME)),
        "Test zone should not exist before create",
    );

    // Also test subscription-level list
    let all_zones = dns
        .list_dns_zones_all()
        .await
        .expect("list_dns_zones_all failed");
    println!("  Subscription-level zone count: {}", all_zones.value.len());

    // =========================================================================
    // Step 5: Create DNS zone via library client
    // =========================================================================
    println!("[5/8] Creating DNS zone '{ZONE_NAME}'...");
    let zone_request = ZoneCreateRequest {
        location: LOCATION.into(),
        ..Default::default()
    };

    let zone = dns
        .create_dns_zone(RG_NAME, ZONE_NAME, &zone_request)
        .await
        .expect("create_dns_zone failed");
    println!(
        "  Zone created: name={:?}, type={:?}, zone_type={:?}, name_servers={}",
        zone.name,
        zone.r#type,
        zone.properties
            .as_ref()
            .and_then(|p| p.zone_type.as_deref()),
        zone.properties
            .as_ref()
            .map(|p| p.name_servers.len())
            .unwrap_or(0),
    );
    assert_eq!(zone.name.as_deref(), Some(ZONE_NAME));
    assert!(zone.id.is_some(), "Zone should have an ARM resource ID");
    // DNS zones have name servers populated after creation
    let name_servers = zone
        .properties
        .as_ref()
        .map(|p| &p.name_servers)
        .expect("Zone should have properties");
    assert!(
        !name_servers.is_empty(),
        "DNS zone should have name servers"
    );

    // =========================================================================
    // Step 6: Get DNS zone + list record sets (default SOA + NS)
    // =========================================================================
    println!("[6/8] Getting DNS zone '{ZONE_NAME}'...");
    let fetched_zone = dns
        .get_dns_zone(RG_NAME, ZONE_NAME)
        .await
        .expect("get_dns_zone failed");
    println!(
        "  Got zone: name={:?}, etag={:?}",
        fetched_zone.name,
        fetched_zone.etag.as_deref().map(|e| &e[..8.min(e.len())]),
    );
    assert_eq!(fetched_zone.name.as_deref(), Some(ZONE_NAME));
    assert!(fetched_zone.id.is_some());

    // List zones again — should include ours
    let list_after = dns
        .list_dns_zones(RG_NAME)
        .await
        .expect("list_dns_zones failed");
    let found = list_after
        .value
        .iter()
        .any(|z| z.name.as_deref() == Some(ZONE_NAME));
    println!(
        "  Found {} zone(s), ours present: {found}",
        list_after.value.len()
    );
    assert!(found, "Our zone should appear in the list");

    // List record sets — should have default SOA and NS records
    println!("  Listing record sets in zone '{ZONE_NAME}'...");
    let record_sets = dns
        .list_record_sets(RG_NAME, ZONE_NAME)
        .await
        .expect("list_record_sets failed");
    println!(
        "  Found {} record set(s) (includes default SOA + NS).",
        record_sets.value.len()
    );
    assert!(
        record_sets.value.len() >= 2,
        "DNS zone should have at least SOA and NS record sets by default",
    );

    // =========================================================================
    // Step 7: Create A record set, get it, then delete it
    // =========================================================================
    println!("[7/8] Creating A record set '{RECORD_SET_NAME}' in zone '{ZONE_NAME}'...");
    let record_request = RecordSetCreateRequest {
        properties: Some(RecordSetProperties {
            ttl: Some(300),
            a_records: vec![ARecord {
                ipv4_address: Some("1.2.3.4".into()),
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    let record_set = dns
        .create_record_set(
            RG_NAME,
            ZONE_NAME,
            RECORD_TYPE,
            RECORD_SET_NAME,
            &record_request,
        )
        .await
        .expect("create_record_set failed");
    println!(
        "  Record set created: name={:?}, type={:?}, ttl={:?}, a_records={}",
        record_set.name,
        record_set.r#type,
        record_set.properties.as_ref().and_then(|p| p.ttl),
        record_set
            .properties
            .as_ref()
            .map(|p| p.a_records.len())
            .unwrap_or(0),
    );
    assert_eq!(record_set.name.as_deref(), Some(RECORD_SET_NAME));
    assert!(
        record_set.id.is_some(),
        "Record set should have an ARM resource ID"
    );
    assert_eq!(
        record_set.properties.as_ref().and_then(|p| p.ttl),
        Some(300),
    );
    let a_records = &record_set.properties.as_ref().unwrap().a_records;
    assert_eq!(a_records.len(), 1);
    assert_eq!(a_records[0].ipv4_address.as_deref(), Some("1.2.3.4"));

    // Get the record set
    let fetched_rs = dns
        .get_record_set(RG_NAME, ZONE_NAME, RECORD_TYPE, RECORD_SET_NAME)
        .await
        .expect("get_record_set failed");
    println!(
        "  Got record set: name={:?}, fqdn={:?}",
        fetched_rs.name,
        fetched_rs
            .properties
            .as_ref()
            .and_then(|p| p.fqdn.as_deref()),
    );
    assert_eq!(fetched_rs.name.as_deref(), Some(RECORD_SET_NAME));
    assert_eq!(
        fetched_rs.properties.as_ref().and_then(|p| p.ttl),
        Some(300),
    );

    // List record sets again — should include our A record
    let record_sets_after = dns
        .list_record_sets(RG_NAME, ZONE_NAME)
        .await
        .expect("list_record_sets failed");
    let rs_found = record_sets_after
        .value
        .iter()
        .any(|rs| rs.name.as_deref() == Some(RECORD_SET_NAME));
    println!(
        "  Record sets after create: {}, our A record present: {rs_found}",
        record_sets_after.value.len(),
    );
    assert!(rs_found, "Our A record set should appear in the list");

    // Delete the record set
    println!("  Deleting record set '{RECORD_SET_NAME}'...");
    dns.delete_record_set(RG_NAME, ZONE_NAME, RECORD_TYPE, RECORD_SET_NAME)
        .await
        .expect("delete_record_set failed");
    println!("  Record set deleted.");

    // Delete DNS zone via library client
    println!("  Deleting DNS zone '{ZONE_NAME}'...");
    dns.delete_dns_zone(RG_NAME, ZONE_NAME)
        .await
        .expect("delete_dns_zone failed");
    println!("  DNS zone deleted.");

    // Verify deletion
    let final_list = dns
        .list_dns_zones(RG_NAME)
        .await
        .expect("list_dns_zones failed after delete");
    assert!(
        !final_list
            .value
            .iter()
            .any(|z| z.name.as_deref() == Some(ZONE_NAME)),
        "DNS zone should be gone after delete",
    );

    println!("\nAll DNS integration tests passed!");
}
