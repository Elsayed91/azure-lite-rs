//! Integration tests for Azure Redis Cache operations.
//!
//! Creates a real Redis cache (Basic C0) and exercises lifecycle.
//! Cache creation takes ~10-15 minutes.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration redis -- --ignored --test-threads=1 --nocapture

use azure_lite::types::redis::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const CACHE_NAME: &str = "cloud-lite-test-redis";

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
async fn redis_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let redis = client.redis();

    // =========================================================================
    // Step 1: Pre-cleanup
    // =========================================================================
    println!("[1/7] Pre-cleanup: removing any leftover test Redis cache...");
    az_run_ignore(&[
        "redis",
        "delete",
        "--name",
        CACHE_NAME,
        "--resource-group",
        RG_NAME,
        "--yes",
    ])
    .await;
    for attempt in 0..30u32 {
        match redis.get_cache(RG_NAME, CACHE_NAME).await {
            Err(_) => {
                println!("  Cache ARM record gone after {} attempt(s).", attempt + 1);
                break;
            }
            Ok(_) => {
                if attempt % 6 == 0 {
                    println!("  Waiting for cache deletion... (attempt {})", attempt + 1);
                }
                if attempt == 29 {
                    println!("  Warning: cache still exists after 5 minutes — proceeding");
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
    // Step 3: Create Redis cache
    // =========================================================================
    println!("[3/7] Creating Redis cache '{CACHE_NAME}' (Basic C0)...");
    let create_req = RedisCreateRequest {
        location: LOCATION.to_string(),
        properties: RedisCreateProperties {
            sku: RedisSku {
                name: "Basic".to_string(),
                family: "C".to_string(),
                capacity: 0,
            },
            enable_non_ssl_port: Some(false),
            redis_version: Some("6".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let cache = redis
        .create_cache(RG_NAME, CACHE_NAME, &create_req)
        .await
        .expect("create_cache failed");
    println!(
        "  Created: name={:?}, provisioning={:?}",
        cache.name.as_deref(),
        cache
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );

    // Poll for Succeeded state — Redis Basic C0 can take up to 20 minutes
    for attempt in 0..120u32 {
        match redis.get_cache(RG_NAME, CACHE_NAME).await {
            Ok(c) => {
                let state = c
                    .properties
                    .as_ref()
                    .and_then(|p| p.provisioning_state.as_deref())
                    .unwrap_or("");
                if state == "Succeeded" {
                    println!("  Cache is Succeeded after {} poll(s).", attempt + 1);
                    break;
                }
                if attempt % 6 == 0 {
                    println!(
                        "  Still provisioning... state={state} (attempt {})",
                        attempt + 1
                    );
                }
                if attempt == 119 {
                    panic!("Cache did not reach Succeeded state in 20 minutes");
                }
            }
            Err(e) => {
                if attempt % 6 == 0 {
                    println!("  get_cache error (attempt {}): {e}", attempt + 1);
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    // =========================================================================
    // Step 4: Get + list caches
    // =========================================================================
    println!("[4/7] Getting and listing caches...");
    let got = redis
        .get_cache(RG_NAME, CACHE_NAME)
        .await
        .expect("get_cache failed");
    assert_eq!(got.name.as_deref(), Some(CACHE_NAME));
    let props = got
        .properties
        .as_ref()
        .expect("cache should have properties");
    println!(
        "  Got cache: hostname={:?}, sslPort={:?}, version={:?}",
        props.host_name.as_deref(),
        props.ssl_port,
        props.redis_version.as_deref(),
    );
    assert!(props.host_name.is_some(), "should have hostname");

    let all = redis.list_caches().await.expect("list_caches failed");
    let ours = all
        .value
        .iter()
        .any(|c| c.name.as_deref() == Some(CACHE_NAME));
    println!(
        "  Found {} cache(s) in subscription, ours present: {ours}",
        all.value.len()
    );
    assert!(ours, "Our cache should appear in subscription list");

    let by_rg = redis
        .list_caches_by_resource_group(RG_NAME)
        .await
        .expect("list_caches_by_resource_group failed");
    println!("  Found {} cache(s) in resource group.", by_rg.value.len());
    assert!(
        by_rg
            .value
            .iter()
            .any(|c| c.name.as_deref() == Some(CACHE_NAME)),
        "Our cache should appear in resource group list"
    );

    // =========================================================================
    // Step 5: List + regenerate keys
    // =========================================================================
    println!("[5/7] Listing and regenerating keys...");
    let keys = redis
        .list_keys(RG_NAME, CACHE_NAME)
        .await
        .expect("list_keys failed");
    println!("  Primary key present: {}", keys.primary_key.is_some());
    assert!(keys.primary_key.is_some(), "should have primary key");

    // Retry on ResourceConflict — cache may still be finishing background tasks
    let mut new_keys = None;
    for attempt in 0..12u32 {
        match redis
            .regenerate_key(
                RG_NAME,
                CACHE_NAME,
                &RedisRegenerateKeyParameters {
                    key_type: "Secondary".to_string(),
                },
            )
            .await
        {
            Ok(k) => {
                println!(
                    "  Regenerated secondary key after {} attempt(s).",
                    attempt + 1
                );
                new_keys = Some(k);
                break;
            }
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("busy") || msg.contains("Conflict") || msg.contains("conflict") {
                    if attempt % 3 == 0 {
                        println!(
                            "  Cache busy, retrying regenerate_key (attempt {})...",
                            attempt + 1
                        );
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                } else {
                    panic!("regenerate_key failed: {e}");
                }
            }
        }
    }
    let new_keys = new_keys.expect("regenerate_key never succeeded");
    println!(
        "  New secondary key present: {}",
        new_keys.secondary_key.is_some()
    );
    assert!(
        new_keys.secondary_key.is_some(),
        "should have secondary key after regeneration"
    );

    // =========================================================================
    // Step 6: Force reboot
    // =========================================================================
    println!("[6/7] Force rebooting AllNodes...");
    // Wait for cache to be ready again after key regeneration
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    let reboot_result = redis
        .force_reboot(
            RG_NAME,
            CACHE_NAME,
            &RedisRebootParameters {
                reboot_type: "AllNodes".to_string(),
                shard_id: None,
            },
        )
        .await
        .expect("force_reboot failed");
    println!(
        "  Reboot result message: {:?}",
        reboot_result.message.as_deref()
    );

    // =========================================================================
    // Step 7: Cleanup
    // =========================================================================
    println!("[7/7] Deleting Redis cache '{CACHE_NAME}'...");
    redis
        .delete_cache(RG_NAME, CACHE_NAME)
        .await
        .expect("delete_cache failed");
    println!("  Delete requested (async, cache will terminate in background).");

    println!("\nAll Azure Redis Cache integration tests passed!");
}
