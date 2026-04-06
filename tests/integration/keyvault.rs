//! Integration tests for Azure Key Vault operations.
//!
//! Creates a real Key Vault and exercises vault, secret, and key CRUD via the
//! ARM management plane. The vault is purged at the end of the test to avoid
//! soft-delete name conflicts on re-runs.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration keyvault_lifecycle -- --ignored --test-threads=1 --nocapture

use azure_lite::types::keyvault::*;

const RG_NAME: &str = "cloud-lite-test-rg";
const LOCATION: &str = "eastus";
const VAULT_NAME: &str = "cloud-lite-ralph-kv";
const SECRET_NAME: &str = "ralph-test-secret";
const KEY_NAME: &str = "ralph-test-key";

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
            && !stderr.contains("VaultNotFound")
            && !stderr.contains("NotFound")
        {
            eprintln!("Warning: az {} failed: {}", args.join(" "), stderr);
        }
    }
}

async fn cleanup_vault() {
    // Soft-delete the vault (may already be deleted)
    az_run_ignore(&[
        "keyvault",
        "delete",
        "--name",
        VAULT_NAME,
        "--resource-group",
        RG_NAME,
    ])
    .await;
    // Purge the soft-deleted vault so the name can be reused
    az_run_ignore(&[
        "keyvault",
        "purge",
        "--name",
        VAULT_NAME,
        "--location",
        LOCATION,
    ])
    .await;
}

/// Returns (tenant_id, user_object_id).
async fn get_account_info() -> (String, String) {
    let tenant_output = tokio::process::Command::new("az")
        .args(["account", "show", "--query", "tenantId", "-o", "tsv"])
        .output()
        .await
        .expect("failed to run az account show");
    let tenant_id = String::from_utf8_lossy(&tenant_output.stdout)
        .trim()
        .to_string();

    let oid_output = tokio::process::Command::new("az")
        .args(["ad", "signed-in-user", "show", "--query", "id", "-o", "tsv"])
        .output()
        .await
        .expect("failed to run az ad signed-in-user show");
    let object_id = String::from_utf8_lossy(&oid_output.stdout)
        .trim()
        .to_string();

    (tenant_id, object_id)
}

// ============================================================================
// Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials and creates real resources"]
async fn keyvault_lifecycle() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    println!("[1/9] Pre-cleanup: removing any leftover test vault (including purge)...");
    cleanup_vault().await;

    let result = std::panic::AssertUnwindSafe(async move {
        keyvault_lifecycle_inner(&client).await;
    });
    let outcome = tokio::task::spawn(result).await;

    println!("\n[9/9] Final cleanup: purging test vault...");
    cleanup_vault().await;

    if let Err(e) = outcome {
        std::panic::resume_unwind(e.into_panic());
    }
}

async fn keyvault_lifecycle_inner(client: &azure_lite::AzureHttpClient) {
    let kv = client.keyvault();

    // =========================================================================
    // Step 2: Get account info
    // =========================================================================
    println!("[2/9] Getting tenant ID and user object ID...");
    let (tenant_id, object_id) = get_account_info().await;
    println!("  tenant_id={tenant_id}, object_id={object_id}");
    assert!(!tenant_id.is_empty(), "tenant_id must not be empty");
    assert!(!object_id.is_empty(), "object_id must not be empty");

    // =========================================================================
    // Step 3: Ensure resource group exists
    // =========================================================================
    println!("[3/9] Ensuring resource group '{RG_NAME}' exists...");
    tokio::process::Command::new("az")
        .args([
            "group",
            "create",
            "--name",
            RG_NAME,
            "--location",
            LOCATION,
            "--output",
            "json",
        ])
        .output()
        .await
        .expect("failed to run az group create");
    println!("  Resource group ready.");

    // =========================================================================
    // Step 4: Create vault
    // =========================================================================
    println!("[4/9] Creating key vault '{VAULT_NAME}'...");
    let create_req = VaultCreateRequest {
        location: LOCATION.into(),
        properties: VaultCreateOrUpdateProperties {
            tenant_id: tenant_id.clone(),
            sku: VaultSku {
                family: Some("A".into()),
                name: Some("standard".into()),
            },
            access_policies: vec![AccessPolicyEntry {
                tenant_id: Some(tenant_id.clone()),
                object_id: Some(object_id.clone()),
                permissions: Some(AccessPermissions {
                    keys: vec![
                        "get".into(),
                        "create".into(),
                        "delete".into(),
                        "list".into(),
                    ],
                    secrets: vec!["get".into(), "set".into(), "delete".into(), "list".into()],
                    ..Default::default()
                }),
            }],
            ..Default::default()
        },
        ..Default::default()
    };
    let created = kv
        .create_vault(RG_NAME, VAULT_NAME, &create_req)
        .await
        .expect("create_vault failed");
    println!(
        "  Created: name={:?}, location={:?}, provisioning_state={:?}",
        created.name.as_deref(),
        created.location.as_deref(),
        created
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref()),
    );
    assert_eq!(created.name.as_deref(), Some(VAULT_NAME));
    assert!(created.id.is_some(), "vault should have ARM resource ID");
    let vault_props = created
        .properties
        .as_ref()
        .expect("vault should have properties");
    assert_eq!(vault_props.tenant_id.as_deref(), Some(tenant_id.as_str()));
    assert!(vault_props.vault_uri.is_some(), "vault should have a URI");

    // Wait for vault to reach Succeeded provisioning state before using it
    println!("  Waiting for vault to reach Succeeded provisioning state...");
    for attempt in 0..30u32 {
        let v = kv
            .get_vault(RG_NAME, VAULT_NAME)
            .await
            .expect("poll get_vault failed");
        let state = v
            .properties
            .as_ref()
            .and_then(|p| p.provisioning_state.as_deref())
            .unwrap_or("");
        if state == "Succeeded" {
            println!("  Vault is Succeeded after {} poll(s).", attempt + 1);
            break;
        }
        if attempt == 29 {
            panic!("Vault did not reach Succeeded after 30 polls: state={state}");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    // =========================================================================
    // Step 5: Get vault
    // =========================================================================
    println!("[5/9] Getting vault '{VAULT_NAME}'...");
    let fetched = kv
        .get_vault(RG_NAME, VAULT_NAME)
        .await
        .expect("get_vault failed");
    assert_eq!(fetched.name.as_deref(), Some(VAULT_NAME));
    assert_eq!(
        fetched
            .properties
            .as_ref()
            .and_then(|p| p.tenant_id.as_deref()),
        Some(tenant_id.as_str()),
    );
    println!(
        "  Got vault, URI={:?}",
        fetched
            .properties
            .as_ref()
            .and_then(|p| p.vault_uri.as_deref())
    );

    // =========================================================================
    // Step 6: List vaults in subscription and RG
    // =========================================================================
    println!("[6/9] Listing vaults in RG (after create)...");
    let list_rg = kv
        .list_vaults_in_group(RG_NAME)
        .await
        .expect("list_vaults_in_group failed");
    let found_rg = list_rg
        .value
        .iter()
        .any(|v| v.name.as_deref() == Some(VAULT_NAME));
    println!(
        "  Found {} vault(s) in RG, ours present: {found_rg}",
        list_rg.value.len()
    );
    assert!(found_rg, "vault should appear in RG list");

    let list_sub = kv.list_vaults().await.expect("list_vaults failed");
    let found_sub = list_sub
        .value
        .iter()
        .any(|v| v.name.as_deref() == Some(VAULT_NAME));
    println!(
        "  Found {} vault(s) subscription-wide, ours present: {found_sub}",
        list_sub.value.len()
    );
    assert!(found_sub, "vault should appear in subscription list");

    // =========================================================================
    // Step 7: Set and manage a secret
    // =========================================================================
    println!("[7/9] Setting secret '{SECRET_NAME}'...");
    let secret_req = SecretCreateRequest {
        properties: SecretCreateOrUpdateProperties {
            value: Some("hunter2".into()),
            content_type: Some("text/plain".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    let secret = kv
        .set_secret(RG_NAME, VAULT_NAME, SECRET_NAME, &secret_req)
        .await
        .expect("set_secret failed");
    println!("  Set secret: name={:?}", secret.name.as_deref());
    assert_eq!(secret.name.as_deref(), Some(SECRET_NAME));
    assert!(secret.id.is_some(), "secret should have ARM resource ID");

    println!("  Getting secret '{SECRET_NAME}'...");
    let got_secret = kv
        .get_secret(RG_NAME, VAULT_NAME, SECRET_NAME)
        .await
        .expect("get_secret failed");
    assert_eq!(got_secret.name.as_deref(), Some(SECRET_NAME));
    // ARM management plane does NOT return the secret value — only URI
    let sp = got_secret
        .properties
        .as_ref()
        .expect("secret should have properties");
    assert!(sp.secret_uri.is_some(), "secret should have a URI");
    println!("  Secret URI: {:?}", sp.secret_uri.as_deref());

    println!("  Listing secrets in vault...");
    let secrets_list = kv
        .list_secrets(RG_NAME, VAULT_NAME)
        .await
        .expect("list_secrets failed");
    let found_secret = secrets_list
        .value
        .iter()
        .any(|s| s.name.as_deref() == Some(SECRET_NAME));
    println!(
        "  Found {} secret(s), ours present: {found_secret}",
        secrets_list.value.len()
    );
    assert!(found_secret, "secret should appear in list");
    // NOTE: delete_secret is not available via the ARM management plane (405 DeleteNotSupported).
    // Vault deletion at the end will remove all secrets.

    // =========================================================================
    // Step 8: Create and manage a key
    // =========================================================================
    println!("[8/9] Creating key '{KEY_NAME}' (RSA-2048)...");
    let key_req = KeyCreateRequest {
        properties: KeyCreateProperties {
            kty: "RSA".into(),
            key_size: Some(2048),
            ..Default::default()
        },
        ..Default::default()
    };
    let key = kv
        .create_key(RG_NAME, VAULT_NAME, KEY_NAME, &key_req)
        .await
        .expect("create_key failed");
    println!(
        "  Created key: name={:?}, kty={:?}",
        key.name.as_deref(),
        key.properties.as_ref().and_then(|p| p.kty.as_deref())
    );
    assert_eq!(key.name.as_deref(), Some(KEY_NAME));
    assert!(key.id.is_some(), "key should have ARM resource ID");
    let kp = key.properties.as_ref().expect("key should have properties");
    assert_eq!(kp.kty.as_deref(), Some("RSA"), "key type should be RSA");
    assert!(kp.key_uri.is_some(), "key should have a URI");

    println!("  Getting key '{KEY_NAME}'...");
    let got_key = kv
        .get_key(RG_NAME, VAULT_NAME, KEY_NAME)
        .await
        .expect("get_key failed");
    assert_eq!(got_key.name.as_deref(), Some(KEY_NAME));
    assert_eq!(
        got_key.properties.as_ref().and_then(|p| p.kty.as_deref()),
        Some("RSA")
    );

    println!("  Listing keys in vault...");
    let keys_list = kv
        .list_keys(RG_NAME, VAULT_NAME)
        .await
        .expect("list_keys failed");
    let found_key = keys_list
        .value
        .iter()
        .any(|k| k.name.as_deref() == Some(KEY_NAME));
    println!(
        "  Found {} key(s), ours present: {found_key}",
        keys_list.value.len()
    );
    assert!(found_key, "key should appear in list");
    // NOTE: delete_key is not available via the ARM management plane (405 DeleteNotSupported).
    // Vault deletion at the end will remove all keys.

    // =========================================================================
    // Step 9: Delete vault
    // =========================================================================
    println!("[9/9] Deleting vault '{VAULT_NAME}'...");
    kv.delete_vault(RG_NAME, VAULT_NAME)
        .await
        .expect("delete_vault failed");
    let after_delete_vaults = kv
        .list_vaults_in_group(RG_NAME)
        .await
        .expect("list_vaults_in_group after delete failed");
    let still_there = after_delete_vaults
        .value
        .iter()
        .any(|v| v.name.as_deref() == Some(VAULT_NAME));
    assert!(!still_there, "deleted vault should not appear in RG list");
    println!("  Vault deleted and confirmed gone from ARM list.");

    println!("\nAll Key Vault integration tests passed!");
}
