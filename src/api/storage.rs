//! Azure Storage API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::storage::StorageOps`. This layer adds:
//! - Ergonomic method signatures
//! - Polling for async operations (CreateStorageAccount returns 202)

use crate::{
    AzureHttpClient, Result,
    ops::storage::StorageOps,
    types::storage::{
        ManagementPolicy, StorageAccount, StorageAccountCreateRequest,
        StorageAccountListKeysResult, StorageAccountListResult, StorageAccountRegenerateKeyRequest,
        StorageAccountUpdateRequest,
    },
};

/// Client for the Azure Storage API
pub struct StorageClient<'a> {
    ops: StorageOps<'a>,
}

impl<'a> StorageClient<'a> {
    /// Create a new Azure Storage API client
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: StorageOps::new(client),
        }
    }

    /// Lists all storage accounts in a subscription
    pub async fn list_storage_accounts(
        &self,
        subscription_id: &str,
    ) -> Result<StorageAccountListResult> {
        self.ops.list_storage_accounts(subscription_id).await
    }

    /// Lists all storage accounts in a resource group
    pub async fn list_storage_accounts_by_resource_group(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<StorageAccountListResult> {
        self.ops
            .list_storage_accounts_by_resource_group(subscription_id, resource_group_name)
            .await
    }

    /// Returns the properties of a storage account
    pub async fn get_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<StorageAccount> {
        self.ops
            .get_storage_account(subscription_id, resource_group_name, account_name)
            .await
    }

    /// Creates a new storage account.
    ///
    /// Azure Storage creates accounts asynchronously — this method polls
    /// until provisioningState is "Succeeded" (or up to ~60s) when the
    /// initial PUT returns 202 with an empty body.
    pub async fn create_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        body: &StorageAccountCreateRequest,
    ) -> Result<StorageAccount> {
        let result = self
            .ops
            .create_storage_account(subscription_id, resource_group_name, account_name, body)
            .await;

        match result {
            Ok(account) => Ok(account),
            // 202 Accepted returns an empty body — poll via GET until provisioned
            Err(crate::AzureError::InvalidResponse { message, .. })
                if message.contains("EOF while parsing") || message.contains("empty") =>
            {
                self.poll_until_provisioned(subscription_id, resource_group_name, account_name)
                    .await
            }
            Err(e) => Err(e),
        }
    }

    /// Poll GET until provisioningState == "Succeeded" (max 60 attempts * 5s = 5 min).
    async fn poll_until_provisioned(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<StorageAccount> {
        for attempt in 1..=60 {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let account = self
                .ops
                .get_storage_account(subscription_id, resource_group_name, account_name)
                .await?;
            let state = account
                .properties
                .as_ref()
                .and_then(|p| p.get("provisioningState"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if state == "Succeeded" {
                return Ok(account);
            }
            if attempt % 3 == 0 {
                eprintln!("  Polling create_storage_account: attempt {attempt}, state={state}");
            }
        }
        Err(crate::AzureError::InvalidResponse {
            message: format!(
                "Storage account '{account_name}' did not reach Succeeded state after polling"
            ),
            body: None,
        })
    }

    /// Deletes a storage account
    pub async fn delete_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<StorageAccount> {
        self.ops
            .delete_storage_account(subscription_id, resource_group_name, account_name)
            .await
    }

    /// Lists the access keys for a storage account
    pub async fn list_keys(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<StorageAccountListKeysResult> {
        self.ops
            .list_keys(subscription_id, resource_group_name, account_name)
            .await
    }

    /// Regenerates one of the access keys for a storage account
    pub async fn regenerate_key(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        body: &StorageAccountRegenerateKeyRequest,
    ) -> Result<StorageAccountListKeysResult> {
        self.ops
            .regenerate_key(subscription_id, resource_group_name, account_name, body)
            .await
    }

    /// Updates properties of an existing storage account (partial PATCH).
    ///
    /// ARM PATCH semantics — only fields set in `body` are changed; all other
    /// properties remain unchanged. Useful for toggling security settings such
    /// as `allow_blob_public_access`, `supports_https_traffic_only`, and
    /// `minimum_tls_version` without touching the rest of the account.
    pub async fn update_storage_account(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
        body: &StorageAccountUpdateRequest,
    ) -> Result<StorageAccount> {
        self.ops
            .patch_storage_account(subscription_id, resource_group_name, account_name, body)
            .await
    }

    /// Fetch the blob lifecycle management policy for a storage account.
    ///
    /// Returns `Ok(None)` if no policy is configured (HTTP 404 — valid state).
    /// Returns `Ok(Some(policy))` on HTTP 200 with the policy data.
    /// Returns `Err(...)` on other errors (401, 403, 5xx, etc.).
    pub async fn get_management_policy(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        account_name: &str,
    ) -> Result<Option<ManagementPolicy>> {
        match self
            .ops
            .get_management_policy(subscription_id, resource_group_name, account_name)
            .await
        {
            Ok(policy) => Ok(Some(policy)),
            Err(crate::AzureError::NotFound { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AzureHttpClient;
    use crate::types::storage::StorageAccountUpdateProperties;

    const SUB: &str = "test-subscription-id";
    const RG: &str = "my-rg";

    #[tokio::test]
    async fn list_storage_accounts_returns_empty_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/subscriptions/test-subscription-id/providers/Microsoft.Storage/storageAccounts",
        )
        .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage
            .list_storage_accounts(SUB)
            .await
            .expect("list failed");
        assert_eq!(result.value.len(), 0);
    }

    #[tokio::test]
    async fn list_storage_accounts_returns_accounts_with_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/providers/Microsoft.Storage/storageAccounts")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct",
                    "name": "myacct",
                    "type": "Microsoft.Storage/storageAccounts",
                    "location": "eastus",
                    "kind": "StorageV2",
                    "sku": { "name": "Standard_LRS", "tier": "Standard" }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage
            .list_storage_accounts(SUB)
            .await
            .expect("list failed");
        assert_eq!(result.value.len(), 1);
        let acct = &result.value[0];
        assert_eq!(acct.get("name").and_then(|v| v.as_str()), Some("myacct"));
        assert_eq!(
            acct.get("location").and_then(|v| v.as_str()),
            Some("eastus")
        );
        assert_eq!(acct.get("kind").and_then(|v| v.as_str()), Some("StorageV2"));
    }

    #[tokio::test]
    async fn list_storage_accounts_by_resource_group_injects_rg() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage
            .list_storage_accounts_by_resource_group(SUB, RG)
            .await
            .expect("list by rg failed");
        assert_eq!(result.value.len(), 0);
    }

    #[tokio::test]
    async fn get_storage_account_returns_account_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct",
                "name": "myacct",
                "type": "Microsoft.Storage/storageAccounts",
                "location": "eastus",
                "kind": "StorageV2",
                "sku": { "name": "Standard_LRS", "tier": "Standard" },
                "properties": {
                    "provisioningState": "Succeeded",
                    "primaryLocation": "eastus"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let account = storage
            .get_storage_account(SUB, RG, "myacct")
            .await
            .expect("get failed");
        assert_eq!(account.name.as_deref(), Some("myacct"));
        assert_eq!(account.location, "eastus");
        assert_eq!(account.kind.as_deref(), Some("StorageV2"));
        let ps = account
            .properties
            .as_ref()
            .and_then(|p| p.get("provisioningState"))
            .and_then(|v| v.as_str());
        assert_eq!(ps, Some("Succeeded"));
    }

    #[tokio::test]
    async fn create_storage_account_sends_put_and_returns_account() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/newacct")
            .returning_json(serde_json::json!({
                "name": "newacct",
                "location": "eastus",
                "kind": "StorageV2",
                "properties": { "provisioningState": "Succeeded" }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let body = StorageAccountCreateRequest {
            location: "eastus".into(),
            kind: "StorageV2".into(),
            sku: serde_json::json!({ "name": "Standard_LRS" }),
            ..Default::default()
        };
        let account = storage
            .create_storage_account(SUB, RG, "newacct", &body)
            .await
            .expect("create failed");
        assert_eq!(account.name.as_deref(), Some("newacct"));
        assert_eq!(account.location, "eastus");
        assert_eq!(account.kind.as_deref(), Some("StorageV2"));
    }

    #[tokio::test]
    async fn delete_storage_account_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct")
            .returning_json(serde_json::json!({
                "name": "myacct",
                "location": "eastus"
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage
            .delete_storage_account(SUB, RG, "myacct")
            .await
            .expect("delete failed");
        assert_eq!(result.name.as_deref(), Some("myacct"));
    }

    #[tokio::test]
    async fn list_keys_returns_two_keys() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct/listKeys")
            .returning_json(serde_json::json!({
                "keys": [
                    { "keyName": "key1", "permissions": "Full", "value": "base64val1==" },
                    { "keyName": "key2", "permissions": "Full", "value": "base64val2==" }
                ]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage
            .list_keys(SUB, RG, "myacct")
            .await
            .expect("list_keys failed");
        assert_eq!(result.keys.len(), 2);
        assert_eq!(
            result.keys[0].get("keyName").and_then(|v| v.as_str()),
            Some("key1")
        );
        assert_eq!(
            result.keys[1].get("keyName").and_then(|v| v.as_str()),
            Some("key2")
        );
    }

    #[tokio::test]
    async fn regenerate_key_sends_post_with_key_name() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct/regenerateKey")
            .returning_json(serde_json::json!({
                "keys": [
                    { "keyName": "key1", "permissions": "Full", "value": "newbase64val1==" },
                    { "keyName": "key2", "permissions": "Full", "value": "base64val2==" }
                ]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let body = StorageAccountRegenerateKeyRequest {
            key_name: "key1".into(),
        };
        let result = storage
            .regenerate_key(SUB, RG, "myacct", &body)
            .await
            .expect("regenerate_key failed");
        assert_eq!(result.keys.len(), 2);
        assert_eq!(
            result.keys[0].get("keyName").and_then(|v| v.as_str()),
            Some("key1")
        );
        assert_eq!(
            result.keys[0].get("value").and_then(|v| v.as_str()),
            Some("newbase64val1==")
        );
    }

    #[tokio::test]
    async fn update_storage_account_sends_patch_and_returns_account() {
        let mut mock = crate::MockClient::new();
        mock.expect_patch("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct")
            .returning_json(serde_json::json!({
                "name": "myacct",
                "location": "eastus",
                "kind": "StorageV2",
                "properties": {
                    "provisioningState": "Succeeded",
                    "supportsHttpsTrafficOnly": true,
                    "minimumTlsVersion": "TLS1_2",
                    "allowBlobPublicAccess": false
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let body = StorageAccountUpdateRequest {
            properties: Some(StorageAccountUpdateProperties {
                allow_blob_public_access: Some(false),
                supports_https_traffic_only: Some(true),
                minimum_tls_version: Some("TLS1_2".into()),
            }),
            ..Default::default()
        };
        let account = storage
            .update_storage_account(SUB, RG, "myacct", &body)
            .await
            .expect("update_storage_account failed");
        assert_eq!(account.name.as_deref(), Some("myacct"));
        assert_eq!(account.location, "eastus");
        let props = account.properties.as_ref().expect("properties missing");
        assert_eq!(
            props
                .get("supportsHttpsTrafficOnly")
                .and_then(|v| v.as_bool()),
            Some(true)
        );
        assert_eq!(
            props.get("minimumTlsVersion").and_then(|v| v.as_str()),
            Some("TLS1_2")
        );
        assert_eq!(
            props.get("allowBlobPublicAccess").and_then(|v| v.as_bool()),
            Some(false)
        );
    }

    #[tokio::test]
    async fn update_storage_account_partial_body_omits_unset_fields() {
        // Verify that setting only one field doesn't include others in the serialized body
        let body = StorageAccountUpdateRequest {
            properties: Some(StorageAccountUpdateProperties {
                supports_https_traffic_only: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };
        let serialized = serde_json::to_value(&body).expect("serialize failed");
        // Only supportsHttpsTrafficOnly should be present — allowBlobPublicAccess and
        // minimumTlsVersion should be omitted (skip_serializing_if = "Option::is_none")
        let props = serialized
            .get("properties")
            .expect("properties key missing");
        assert!(props.get("supportsHttpsTrafficOnly").is_some());
        assert!(
            props.get("allowBlobPublicAccess").is_none(),
            "unset field must be omitted"
        );
        assert!(
            props.get("minimumTlsVersion").is_none(),
            "unset field must be omitted"
        );
    }

    #[tokio::test]
    async fn get_management_policy_returns_some_with_rules() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct/managementPolicies/default")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct/managementPolicies/default",
                "name": "DefaultManagementPolicy",
                "type": "Microsoft.Storage/storageAccounts/managementPolicies",
                "properties": {
                    "lastModifiedTime": "2024-01-15T10:30:00Z",
                    "policy": {
                        "rules": [
                            { "enabled": true, "name": "move-to-cool", "type": "Lifecycle" },
                            { "enabled": false, "name": "delete-old-blobs", "type": "Lifecycle" }
                        ]
                    }
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage
            .get_management_policy(SUB, RG, "myacct")
            .await
            .expect("get_management_policy failed");
        let policy = result.expect("expected Some(policy)");
        assert_eq!(policy.name.as_deref(), Some("DefaultManagementPolicy"));

        let props = policy.properties.expect("properties missing");
        assert_eq!(
            props.last_modified_time.as_deref(),
            Some("2024-01-15T10:30:00Z")
        );

        let schema = props.policy.expect("policy schema missing");
        assert_eq!(schema.rules.len(), 2);
        assert_eq!(schema.rules[0].name, "move-to-cool");
        assert!(schema.rules[0].enabled);
        assert_eq!(schema.rules[1].name, "delete-old-blobs");
        assert!(!schema.rules[1].enabled);
    }

    #[tokio::test]
    async fn get_management_policy_returns_none_for_404() {
        // Integration-proven: storage accounts without a lifecycle policy return 404,
        // which becomes AzureError::NotFound. get_management_policy must return Ok(None).
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct/managementPolicies/default")
            .returning_error(crate::AzureError::NotFound {
                resource: "ManagementPolicy not found".into(),
            });

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage.get_management_policy(SUB, RG, "myacct").await;
        assert!(result.is_ok(), "NotFound should become Ok(None), not Err");
        assert!(result.unwrap().is_none(), "should return None for 404");
    }

    #[tokio::test]
    async fn get_management_policy_propagates_auth_error() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Storage/storageAccounts/myacct/managementPolicies/default")
            .returning_error(crate::AzureError::Auth {
                message: "Token expired".into(),
            });

        let client = AzureHttpClient::from_mock(mock);
        let storage = client.storage();
        let result = storage.get_management_policy(SUB, RG, "myacct").await;
        assert!(result.is_err(), "Auth errors must propagate as Err");
        assert!(matches!(
            result.unwrap_err(),
            crate::AzureError::Auth { .. }
        ));
    }
}
