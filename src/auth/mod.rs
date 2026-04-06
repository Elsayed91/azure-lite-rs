//! Azure AD authentication.
//!
//! Credential chain (tried in order):
//! 1. **Service principal** — `AZURE_CLIENT_ID` + `AZURE_CLIENT_SECRET` + `AZURE_TENANT_ID`
//! 2. **Managed identity** — IMDS endpoint `http://169.254.169.254/metadata/identity/...`
//! 3. **Azure CLI** — `az account get-access-token`
//!
//! Tokens are cached and reused for up to 55 minutes (Azure tokens are valid for
//! 60–90 minutes; we refresh proactively to avoid mid-request expiry).

pub mod cli;
pub mod managed_identity;
pub mod service_principal;
pub mod token;

pub use token::{AccessToken, CachedToken};

use crate::error::AzureError;

/// Credential chain for Azure authentication.
///
/// Tries each provider in order and returns the first successful token.
pub enum AzureCredential {
    ServicePrincipal(service_principal::ServicePrincipalCredential),
    ManagedIdentity(managed_identity::ManagedIdentityCredential),
    AzureCli(cli::AzureCliCredential),
}

impl AzureCredential {
    /// Acquire an access token for the ARM management plane.
    pub async fn get_token(&self) -> Result<AccessToken, AzureError> {
        match self {
            AzureCredential::ServicePrincipal(sp) => sp.get_token().await,
            AzureCredential::ManagedIdentity(mi) => mi.get_token().await,
            AzureCredential::AzureCli(cli) => cli.get_token().await,
        }
    }

    /// Acquire an access token for an arbitrary OAuth2 scope.
    ///
    /// Use this for non-ARM services such as Microsoft Graph
    /// (`https://graph.microsoft.com/.default`).
    pub(crate) async fn get_token_for_scope(&self, scope: &str) -> Result<AccessToken, AzureError> {
        match self {
            AzureCredential::ServicePrincipal(sp) => sp.get_token_for_scope(scope).await,
            AzureCredential::ManagedIdentity(mi) => mi.get_token_for_scope(scope).await,
            AzureCredential::AzureCli(cli) => cli.get_token_for_scope(scope).await,
        }
    }
}

/// Resolve credentials from the default chain.
///
/// Order:
/// 1. Service principal env vars (AZURE_CLIENT_ID, AZURE_CLIENT_SECRET, AZURE_TENANT_ID)
/// 2. Managed identity IMDS endpoint
/// 3. Azure CLI (`az account get-access-token`)
///
/// Returns the first source that successfully acquires a token.
/// Errors only if all three sources fail.
pub async fn default_credential() -> Result<AzureCredential, AzureError> {
    // 1. Service principal via env vars
    if let Some(sp) = service_principal::ServicePrincipalCredential::from_env() {
        return Ok(AzureCredential::ServicePrincipal(sp));
    }

    // 2. Managed identity — probe the IMDS endpoint
    let mi = managed_identity::ManagedIdentityCredential::new();
    if mi.get_token().await.is_ok() {
        return Ok(AzureCredential::ManagedIdentity(mi));
    }

    // 3. Azure CLI
    let az = cli::AzureCliCredential::new();
    match az.get_token().await {
        Ok(_) => Ok(AzureCredential::AzureCli(az)),
        Err(e) => Err(AzureError::Auth {
            message: format!(
                "No Azure credentials found. \
                Set AZURE_CLIENT_ID/AZURE_CLIENT_SECRET/AZURE_TENANT_ID for service principal auth, \
                run on Azure for managed identity, or run 'az login' for CLI auth. \
                Last error: {e}"
            ),
        }),
    }
}
