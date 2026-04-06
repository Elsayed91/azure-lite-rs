//! Integration tests for Azure Resource Graph operations.
//!
//! Tests KQL query execution and pagination against the real Resource Graph API.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration resource_graph -- --ignored --test-threads=1 --nocapture

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials"]
async fn resource_graph_query_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let rg = client.resource_graph();

    // =========================================================================
    // Step 1: Basic query — all resources in subscription (up to 1000)
    // =========================================================================
    println!("[1/4] Querying all resources in subscription (first page)...");
    let page = rg
        .query_page(
            "Resources | project id, name, type, location",
            &[],
            None,
            None,
        )
        .await
        .expect("query_page failed");
    println!(
        "  total_records={:?}, count={:?}, result_truncated={:?}",
        page.total_records, page.count, page.result_truncated
    );
    assert!(page.count.unwrap_or(0) >= 0, "count should be non-negative");
    println!("  Page returned {} record(s).", page.data.len());

    // =========================================================================
    // Step 2: Type-filtered query — disks
    // =========================================================================
    println!("[2/4] Querying compute disks...");
    let disks = rg
        .query(
            "Resources | where type =~ 'microsoft.compute/disks' | project id, name, type, location, resourceGroup, subscriptionId",
            &[],
        )
        .await
        .expect("query disks failed");
    println!("  Found {} disk(s).", disks.len());
    for disk in disks.iter().take(3) {
        println!(
            "    - {} ({})",
            disk["name"].as_str().unwrap_or("?"),
            disk["location"].as_str().unwrap_or("?")
        );
    }

    // =========================================================================
    // Step 3: Multi-type query
    // =========================================================================
    println!("[3/4] Querying multiple resource types...");
    let multi = rg
        .query(
            "Resources | where type in~ ('microsoft.compute/disks', 'microsoft.storage/storageaccounts', 'microsoft.compute/virtualmachines') | project id, name, type, location",
            &[],
        )
        .await
        .expect("multi-type query failed");
    println!("  Found {} resource(s) across queried types.", multi.len());

    // =========================================================================
    // Step 4: Pagination — query with top=1 to force multi-page
    // =========================================================================
    println!("[4/4] Testing pagination with top=1...");
    use azure_lite::types::resource_graph::QueryOptions;

    let first = rg
        .query_page(
            "Resources | project id, name, type",
            &[],
            Some(QueryOptions {
                top: Some(1),
                result_format: Some("objectArray".to_string()),
                ..Default::default()
            }),
            None,
        )
        .await
        .expect("first page failed");
    println!(
        "  First page: count={:?}, truncated={:?}, skip_token={}",
        first.count,
        first.result_truncated,
        if first.skip_token.is_some() {
            "present"
        } else {
            "absent"
        }
    );

    if first.skip_token.is_some() {
        let second = rg
            .query_page(
                "Resources | project id, name, type",
                &[],
                Some(QueryOptions {
                    top: Some(1),
                    result_format: Some("objectArray".to_string()),
                    ..Default::default()
                }),
                first.skip_token.as_deref(),
            )
            .await
            .expect("second page failed");
        println!("  Second page: count={:?}", second.count);
        assert_eq!(second.count, Some(1), "second page should have 1 record");
    } else {
        println!("  Only 0 or 1 resources in subscription — pagination not exercised.");
    }

    println!("\nAll Azure Resource Graph integration tests passed!");
}
