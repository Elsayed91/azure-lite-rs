//! Azure Key Vault API client.
//!
//! Wraps the ARM management plane operations for Key Vault vaults, secrets,
//! and keys. All URL construction is in `ops::keyvault::KeyvaultOps`.
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.
//!
//! NOTE: WrapKey / UnwrapKey are data-plane crypto operations that require
//! a vault-scoped endpoint (`https://{vault}.vault.azure.net`). They are
//! not included here; add them as custom methods if needed.

use crate::{
    AzureHttpClient, Result,
    ops::keyvault::KeyvaultOps,
    types::keyvault::{
        Key, KeyCreateRequest, KeyListResult, Secret, SecretCreateRequest, SecretListResult, Vault,
        VaultCreateRequest, VaultListResult,
    },
};

/// Client for the Azure Key Vault ARM management plane.
///
/// Wraps [`KeyvaultOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct KeyVaultClient<'a> {
    ops: KeyvaultOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> KeyVaultClient<'a> {
    /// Create a new Azure Key Vault API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: KeyvaultOps::new(client),
            client,
        }
    }

    // --- Vault operations ---

    /// Lists all key vaults in the subscription.
    pub async fn list_vaults(&self) -> Result<VaultListResult> {
        self.ops.list_vaults(self.client.subscription_id()).await
    }

    /// Lists key vaults within a resource group.
    pub async fn list_vaults_in_group(&self, resource_group_name: &str) -> Result<VaultListResult> {
        self.ops
            .list_vaults_in_group(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Gets the specified Azure key vault.
    pub async fn get_vault(&self, resource_group_name: &str, vault_name: &str) -> Result<Vault> {
        self.ops
            .get_vault(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
            )
            .await
    }

    /// Creates or updates a key vault.
    pub async fn create_vault(
        &self,
        resource_group_name: &str,
        vault_name: &str,
        body: &VaultCreateRequest,
    ) -> Result<Vault> {
        self.ops
            .create_vault(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
                body,
            )
            .await
    }

    /// Deletes the specified key vault.
    pub async fn delete_vault(&self, resource_group_name: &str, vault_name: &str) -> Result<()> {
        self.ops
            .delete_vault(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
            )
            .await
    }

    // --- Secret operations (ARM management plane — no secret values in list) ---

    /// Lists the secrets in a key vault (metadata only, no values).
    pub async fn list_secrets(
        &self,
        resource_group_name: &str,
        vault_name: &str,
    ) -> Result<SecretListResult> {
        self.ops
            .list_secrets(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
            )
            .await
    }

    /// Gets a secret's properties (no secret value).
    pub async fn get_secret(
        &self,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
    ) -> Result<Secret> {
        self.ops
            .get_secret(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
                secret_name,
            )
            .await
    }

    /// Creates or updates a secret in the key vault.
    pub async fn set_secret(
        &self,
        resource_group_name: &str,
        vault_name: &str,
        secret_name: &str,
        body: &SecretCreateRequest,
    ) -> Result<Secret> {
        self.ops
            .set_secret(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
                secret_name,
                body,
            )
            .await
    }

    // NOTE: delete_secret is not available via the ARM management plane (405 DeleteNotSupported).
    // Use the Key Vault data plane API (https://{vault}.vault.azure.net) to delete secrets.

    // --- Key operations (ARM management plane) ---

    /// Lists the keys in a key vault.
    pub async fn list_keys(
        &self,
        resource_group_name: &str,
        vault_name: &str,
    ) -> Result<KeyListResult> {
        self.ops
            .list_keys(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
            )
            .await
    }

    /// Gets the current version of a key.
    pub async fn get_key(
        &self,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
    ) -> Result<Key> {
        self.ops
            .get_key(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
                key_name,
            )
            .await
    }

    /// Creates a key (or returns existing if already present).
    pub async fn create_key(
        &self,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
        body: &KeyCreateRequest,
    ) -> Result<Key> {
        self.ops
            .create_key(
                self.client.subscription_id(),
                resource_group_name,
                vault_name,
                key_name,
                body,
            )
            .await
    }

    // NOTE: delete_key is not available via the ARM management plane (405 DeleteNotSupported).
    // Use the Key Vault data plane API (https://{vault}.vault.azure.net) to delete keys.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const RG: &str = "test-rg";
    const VAULT: &str = "test-vault";
    const SECRET: &str = "test-secret";
    const KEY: &str = "test-key";
    const TENANT_ID: &str = "12345678-1234-1234-1234-123456789abc";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn vault_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}"),
            "name": VAULT,
            "type": "Microsoft.KeyVault/vaults",
            "location": "eastus",
            "properties": {
                "tenantId": TENANT_ID,
                "sku": { "family": "A", "name": "standard" },
                "enabledForDeployment": false,
                "enableSoftDelete": true,
                "softDeleteRetentionInDays": 90,
                "enableRbacAuthorization": false,
                "vaultUri": "https://test-vault.vault.azure.net/",
                "provisioningState": "Succeeded",
                "accessPolicies": []
            }
        })
    }

    fn secret_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}/secrets/{SECRET}"),
            "name": SECRET,
            "type": "Microsoft.KeyVault/vaults/secrets",
            "location": "eastus",
            "properties": {
                "secretUri": "https://test-vault.vault.azure.net/secrets/test-secret",
                "secretUriWithVersion": "https://test-vault.vault.azure.net/secrets/test-secret/abc123"
            }
        })
    }

    fn key_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}/keys/{KEY}"),
            "name": KEY,
            "type": "Microsoft.KeyVault/vaults/keys",
            "location": "eastus",
            "properties": {
                "kty": "RSA",
                "keySize": 2048,
                "keyUri": "https://test-vault.vault.azure.net/keys/test-key",
                "keyUriWithVersion": "https://test-vault.vault.azure.net/keys/test-key/def456"
            }
        })
    }

    #[tokio::test]
    async fn list_vaults_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.KeyVault/vaults"
        ))
        .returning_json(serde_json::json!({ "value": [vault_json()] }));
        let client = make_client(mock);
        let result = client
            .keyvault()
            .list_vaults()
            .await
            .expect("list_vaults failed");
        assert_eq!(result.value.len(), 1);
        let v = &result.value[0];
        assert_eq!(v.name.as_deref(), Some(VAULT));
        let props = v.properties.as_ref().unwrap();
        assert_eq!(props.enable_soft_delete, Some(true));
        assert_eq!(props.enabled_for_deployment, Some(false));
        assert_eq!(props.provisioning_state.as_deref(), Some("Succeeded"));
    }

    #[tokio::test]
    async fn get_vault_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}"),
        )
        .returning_json(vault_json());
        let client = make_client(mock);
        let v = client
            .keyvault()
            .get_vault(RG, VAULT)
            .await
            .expect("get_vault failed");
        assert_eq!(v.name.as_deref(), Some(VAULT));
        let props = v.properties.as_ref().unwrap();
        assert_eq!(props.tenant_id.as_deref(), Some(TENANT_ID));
        assert_eq!(
            props.vault_uri.as_deref(),
            Some("https://test-vault.vault.azure.net/")
        );
        assert_eq!(props.soft_delete_retention_in_days, Some(90));
    }

    #[tokio::test]
    async fn create_vault_returns_vault() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}"),
        )
        .returning_json(vault_json());
        let client = make_client(mock);
        use crate::types::keyvault::{VaultCreateOrUpdateProperties, VaultSku};
        let body = VaultCreateRequest {
            location: "eastus".into(),
            properties: VaultCreateOrUpdateProperties {
                tenant_id: TENANT_ID.into(),
                sku: VaultSku {
                    family: Some("A".into()),
                    name: Some("standard".into()),
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let v = client
            .keyvault()
            .create_vault(RG, VAULT, &body)
            .await
            .expect("create_vault failed");
        assert_eq!(v.name.as_deref(), Some(VAULT));
        assert!(v.id.is_some());
    }

    #[tokio::test]
    async fn set_secret_returns_secret_without_value() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}/secrets/{SECRET}"),
        )
        .returning_json(secret_json());
        let client = make_client(mock);
        use crate::types::keyvault::SecretCreateOrUpdateProperties;
        let body = SecretCreateRequest {
            properties: SecretCreateOrUpdateProperties {
                value: Some("s3cr3t".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        let s = client
            .keyvault()
            .set_secret(RG, VAULT, SECRET, &body)
            .await
            .expect("set_secret failed");
        assert_eq!(s.name.as_deref(), Some(SECRET));
        // ARM management plane does NOT return the secret value
        assert!(
            s.properties
                .as_ref()
                .and_then(|p| p.value.as_deref())
                .is_none()
        );
        assert!(
            s.properties
                .as_ref()
                .and_then(|p| p.secret_uri.as_deref())
                .is_some()
        );
    }

    #[tokio::test]
    async fn create_key_returns_rsa_key() {
        let mut mock = MockClient::new();
        mock.expect_put(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}/keys/{KEY}"),
        )
        .returning_json(key_json());
        let client = make_client(mock);
        use crate::types::keyvault::KeyCreateProperties;
        let body = KeyCreateRequest {
            properties: KeyCreateProperties {
                kty: "RSA".into(),
                key_size: Some(2048),
                ..Default::default()
            },
            ..Default::default()
        };
        let k = client
            .keyvault()
            .create_key(RG, VAULT, KEY, &body)
            .await
            .expect("create_key failed");
        assert_eq!(k.name.as_deref(), Some(KEY));
        let props = k.properties.as_ref().unwrap();
        assert_eq!(props.kty.as_deref(), Some("RSA"));
        assert!(props.key_uri.is_some());
    }

    #[tokio::test]
    async fn delete_vault_succeeds() {
        let mut mock = MockClient::new();
        mock.expect_delete(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.KeyVault/vaults/{VAULT}"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .keyvault()
            .delete_vault(RG, VAULT)
            .await
            .expect("delete_vault failed");
    }
}
