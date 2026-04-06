//! Core HTTP client for Azure API access.

use crate::auth::AzureCredential;
use crate::error::AzureError;
use cloud_lite_core::rate_limit::{RateLimitConfig, RateLimiter};
use cloud_lite_core::retry::RetryConfig;

/// HTTP client for Azure API operations.
///
/// Provides automatic Bearer token injection, retry, and rate limiting.
pub struct AzureHttpClient {
    http: reqwest::Client,
    credential: AzureCredential,
    subscription_id: String,
    retry_config: RetryConfig,
    rate_limiter: RateLimiter,
    /// Override base URL for testing (e.g. mock server).
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) base_url: Option<String>,
    /// Mock client for testing.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) mock: Option<std::sync::Arc<crate::mock_client::MockClient>>,
}

/// Response wrapper that abstracts over real and mock responses.
pub struct AzureResponse {
    data: ResponseData,
}

enum ResponseData {
    Real(reqwest::Response),
    #[cfg(any(test, feature = "test-support"))]
    Mock(Vec<u8>),
}

impl AzureResponse {
    /// Get the HTTP status code of the response.
    pub fn status(&self) -> u16 {
        match &self.data {
            ResponseData::Real(response) => response.status().as_u16(),
            #[cfg(any(test, feature = "test-support"))]
            ResponseData::Mock(_) => 200,
        }
    }

    /// Check for HTTP error status and parse the error body.
    pub async fn error_for_status(self) -> Result<Self, AzureError> {
        let status = self.status();
        if status < 400 {
            return Ok(self);
        }

        let body_bytes = self
            .bytes()
            .await
            .unwrap_or_else(|_| bytes::Bytes::from_static(b""));
        let body_text = std::str::from_utf8(&body_bytes).unwrap_or("");

        Err(crate::error::parse_json_error(status, body_text))
    }

    /// Get the `Location` response header value, if present.
    pub fn location(&self) -> Option<String> {
        match &self.data {
            ResponseData::Real(response) => response
                .headers()
                .get("location")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string()),
            #[cfg(any(test, feature = "test-support"))]
            ResponseData::Mock(_) => None,
        }
    }

    /// Read the response body as bytes.
    pub async fn bytes(self) -> Result<bytes::Bytes, AzureError> {
        match self.data {
            ResponseData::Real(response) => response
                .bytes()
                .await
                .map_err(|e| AzureError::Network(e.to_string())),
            #[cfg(any(test, feature = "test-support"))]
            ResponseData::Mock(data) => Ok(bytes::Bytes::from(data)),
        }
    }
}

/// Builder for [`AzureHttpClient`].
pub struct AzureHttpClientBuilder {
    pub(crate) subscription_id: Option<String>,
    pub(crate) retry_config: RetryConfig,
    pub(crate) rate_limit: RateLimitConfig,
    credential: Option<AzureCredential>,
}

impl Default for AzureHttpClientBuilder {
    fn default() -> Self {
        Self {
            subscription_id: None,
            retry_config: RetryConfig::default(),
            rate_limit: RateLimitConfig::new(20),
            credential: None,
        }
    }
}

impl AzureHttpClientBuilder {
    /// Set the Azure subscription ID.
    pub fn subscription_id(mut self, id: impl Into<String>) -> Self {
        self.subscription_id = Some(id.into());
        self
    }

    /// Set an explicit credential (overrides default chain).
    pub fn credential(mut self, cred: AzureCredential) -> Self {
        self.credential = Some(cred);
        self
    }

    /// Set retry configuration.
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Set rate limiting configuration.
    pub fn rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.rate_limit = config;
        self
    }

    /// Build the client (sync — requires credential to be supplied explicitly).
    ///
    /// For default credential chain resolution, use [`AzureHttpClient::from_env()`] which is async.
    pub fn build(self) -> Result<AzureHttpClient, AzureError> {
        let subscription_id = self
            .subscription_id
            .or_else(|| std::env::var("AZURE_SUBSCRIPTION_ID").ok())
            .ok_or_else(|| AzureError::Auth {
                message:
                    "subscription_id required (set AZURE_SUBSCRIPTION_ID or call .subscription_id())"
                        .into(),
            })?;

        let credential = self.credential.ok_or_else(|| AzureError::Auth {
            message:
                "credential required — use AzureHttpClient::from_env().await for the default chain"
                    .into(),
        })?;

        let http = reqwest::Client::builder()
            .build()
            .map_err(|e| AzureError::Network(e.to_string()))?;

        Ok(AzureHttpClient {
            http,
            credential,
            subscription_id,
            retry_config: self.retry_config,
            rate_limiter: RateLimiter::new(self.rate_limit),
            #[cfg(any(test, feature = "test-support"))]
            base_url: None,
            #[cfg(any(test, feature = "test-support"))]
            mock: None,
        })
    }
}

impl AzureHttpClient {
    /// Create a new builder.
    pub fn builder() -> AzureHttpClientBuilder {
        AzureHttpClientBuilder::default()
    }

    /// Create a client using the default credential chain.
    ///
    /// Resolves credentials in order:
    /// 1. Service principal env vars
    /// 2. Managed identity IMDS
    /// 3. Azure CLI
    pub async fn from_env() -> Result<Self, AzureError> {
        let credential = crate::auth::default_credential().await?;
        let subscription_id =
            std::env::var("AZURE_SUBSCRIPTION_ID").map_err(|_| AzureError::Auth {
                message: "AZURE_SUBSCRIPTION_ID environment variable not set".into(),
            })?;

        let http = reqwest::Client::builder()
            .build()
            .map_err(|e| AzureError::Network(e.to_string()))?;

        Ok(Self {
            http,
            credential,
            subscription_id,
            retry_config: RetryConfig::default(),
            rate_limiter: RateLimiter::new(RateLimitConfig::new(20)),
            #[cfg(any(test, feature = "test-support"))]
            base_url: None,
            #[cfg(any(test, feature = "test-support"))]
            mock: None,
        })
    }

    /// Create a client from a mock for testing.
    #[cfg(any(test, feature = "test-support"))]
    pub fn from_mock(mock: crate::mock_client::MockClient) -> Self {
        use crate::auth::cli::AzureCliCredential;
        Self {
            http: reqwest::Client::new(),
            credential: AzureCredential::AzureCli(AzureCliCredential::new()),
            subscription_id: "test-subscription-id".into(),
            retry_config: RetryConfig::default(),
            rate_limiter: RateLimiter::new(RateLimitConfig::disabled()),
            base_url: None,
            mock: Some(std::sync::Arc::new(mock)),
        }
    }

    /// Get the configured subscription ID.
    pub fn subscription_id(&self) -> &str {
        &self.subscription_id
    }

    /// Acquire a token from the credential chain.
    pub async fn token(&self) -> Result<String, AzureError> {
        Ok(self.credential.get_token().await?.token)
    }

    // === Generated API Accessors (do not edit) ===

    /// Access the Azure Container Registry API
    pub fn acr(&self) -> crate::api::AcrClient<'_> {
        crate::api::AcrClient::new(self)
    }

    /// Access the Azure Kubernetes Service API
    pub fn aks(&self) -> crate::api::AksClient<'_> {
        crate::api::AksClient::new(self)
    }

    /// Access the Azure Compute API
    pub fn compute(&self) -> crate::api::ComputeClient<'_> {
        crate::api::ComputeClient::new(self)
    }

    /// Access the Azure CosmosDB API
    pub fn cosmosdb(&self) -> crate::api::CosmosDbClient<'_> {
        crate::api::CosmosDbClient::new(self)
    }

    /// Access the Azure Cost Management API
    pub fn cost(&self) -> crate::api::CostClient<'_> {
        crate::api::CostClient::new(self)
    }

    /// Access the Azure DNS API
    pub fn dns(&self) -> crate::api::DnsClient<'_> {
        crate::api::DnsClient::new(self)
    }

    /// Access the Azure Functions API
    pub fn functions(&self) -> crate::api::FunctionsClient<'_> {
        crate::api::FunctionsClient::new(self)
    }

    /// Access the Microsoft Graph API
    pub fn graph(&self) -> crate::api::GraphClient<'_> {
        crate::api::GraphClient::new(self)
    }

    /// Access the Azure Managed Identities API
    pub fn identity(&self) -> crate::api::IdentityClient<'_> {
        crate::api::IdentityClient::new(self)
    }

    /// Access the Azure Key Vault API
    pub fn keyvault(&self) -> crate::api::KeyVaultClient<'_> {
        crate::api::KeyVaultClient::new(self)
    }

    /// Access the Azure Log Analytics API
    pub fn log_analytics(&self) -> crate::api::LogAnalyticsClient<'_> {
        crate::api::LogAnalyticsClient::new(self)
    }

    /// Access the Azure Monitor API
    pub fn monitor(&self) -> crate::api::MonitorClient<'_> {
        crate::api::MonitorClient::new(self)
    }

    /// Access the Azure Networking API
    pub fn networking(&self) -> crate::api::NetworkingClient<'_> {
        crate::api::NetworkingClient::new(self)
    }

    /// Access the Azure RBAC API
    pub fn rbac(&self) -> crate::api::RbacClient<'_> {
        crate::api::RbacClient::new(self)
    }

    /// Access the Azure Redis Cache API
    pub fn redis(&self) -> crate::api::RedisClient<'_> {
        crate::api::RedisClient::new(self)
    }

    /// Access the Azure Resource Graph API
    pub fn resource_graph(&self) -> crate::api::ResourceGraphClient<'_> {
        crate::api::ResourceGraphClient::new(self)
    }

    /// Access the Azure Defender for Cloud API
    pub fn security(&self) -> crate::api::SecurityClient<'_> {
        crate::api::SecurityClient::new(self)
    }

    /// Access the Azure SQL API
    pub fn sql(&self) -> crate::api::SqlClient<'_> {
        crate::api::SqlClient::new(self)
    }

    /// Access the Azure Storage API
    pub fn storage(&self) -> crate::api::StorageClient<'_> {
        crate::api::StorageClient::new(self)
    }

    /// Access the Azure Subscriptions API
    pub fn subscriptions(&self) -> crate::api::SubscriptionsClient<'_> {
        crate::api::SubscriptionsClient::new(self)
    }
    // === End Generated API Accessors ===

    // =========================================================================
    // Microsoft Graph API — uses a separate OAuth2 scope
    // =========================================================================

    const GRAPH_SCOPE: &str = "https://graph.microsoft.com/.default";

    /// Make a GET request to the Microsoft Graph API.
    ///
    /// Acquires a Graph-scoped token (`https://graph.microsoft.com/.default`)
    /// rather than the ARM scope used by the standard `get()` method.
    pub(crate) async fn graph_get(&self, url: &str) -> Result<AzureResponse, AzureError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("GET", url, None).await?;
            return Ok(AzureResponse {
                data: ResponseData::Mock(result),
            });
        }

        let token = self
            .credential
            .get_token_for_scope(Self::GRAPH_SCOPE)
            .await?
            .token;
        let response = self.bearer_request("GET", url, &token, b"", None).await?;
        Ok(AzureResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a POST request to the Microsoft Graph API with a JSON body.
    ///
    /// Acquires a Graph-scoped token (`https://graph.microsoft.com/.default`)
    /// rather than the ARM scope used by the standard `post()` method.
    pub(crate) async fn graph_post(
        &self,
        url: &str,
        body: &[u8],
    ) -> Result<AzureResponse, AzureError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("POST", url, None).await?;
            return Ok(AzureResponse {
                data: ResponseData::Mock(result),
            });
        }

        let token = self
            .credential
            .get_token_for_scope(Self::GRAPH_SCOPE)
            .await?
            .token;
        let response = self
            .bearer_request("POST", url, &token, body, Some("application/json"))
            .await?;
        Ok(AzureResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a GET request with automatic Bearer token injection and retry.
    pub async fn get(&self, url: &str) -> Result<AzureResponse, AzureError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("GET", url, None).await?;
            return Ok(AzureResponse {
                data: ResponseData::Mock(result),
            });
        }

        let token = self.credential.get_token().await?.token;
        let response = self.bearer_request("GET", url, &token, b"", None).await?;
        Ok(AzureResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a PUT request with a JSON body.
    pub async fn put(&self, url: &str, body: &[u8]) -> Result<AzureResponse, AzureError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("PUT", url, None).await?;
            return Ok(AzureResponse {
                data: ResponseData::Mock(result),
            });
        }

        let token = self.credential.get_token().await?.token;
        let response = self
            .bearer_request("PUT", url, &token, body, Some("application/json"))
            .await?;
        Ok(AzureResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a POST request with a JSON body.
    pub async fn post(&self, url: &str, body: &[u8]) -> Result<AzureResponse, AzureError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("POST", url, None).await?;
            return Ok(AzureResponse {
                data: ResponseData::Mock(result),
            });
        }

        let token = self.credential.get_token().await?.token;
        let response = self
            .bearer_request("POST", url, &token, body, Some("application/json"))
            .await?;
        Ok(AzureResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a DELETE request.
    pub async fn delete(&self, url: &str) -> Result<AzureResponse, AzureError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("DELETE", url, None).await?;
            return Ok(AzureResponse {
                data: ResponseData::Mock(result),
            });
        }

        let token = self.credential.get_token().await?.token;
        let response = self
            .bearer_request("DELETE", url, &token, b"", None)
            .await?;
        Ok(AzureResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Make a PATCH request with a JSON body.
    pub async fn patch(&self, url: &str, body: &[u8]) -> Result<AzureResponse, AzureError> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(ref mock) = self.mock {
            let result = mock.execute("PATCH", url, None).await?;
            return Ok(AzureResponse {
                data: ResponseData::Mock(result),
            });
        }

        let token = self.credential.get_token().await?.token;
        let response = self
            .bearer_request("PATCH", url, &token, body, Some("application/json"))
            .await?;
        Ok(AzureResponse {
            data: ResponseData::Real(response),
        })
    }

    /// Internal: request with Bearer auth, content-type, and retry.
    async fn bearer_request(
        &self,
        method: &str,
        url: &str,
        token: &str,
        body: &[u8],
        content_type: Option<&str>,
    ) -> Result<reqwest::Response, AzureError> {
        let _permit = self.rate_limiter.acquire(url).await;

        let mut attempt = 0u32;
        let mut backoff = self.retry_config.initial_backoff;
        let body_bytes = if body.is_empty() {
            None
        } else {
            Some(bytes::Bytes::copy_from_slice(body))
        };

        loop {
            let mut request = self
                .http
                .request(method.parse().expect("invalid HTTP method"), url)
                .header("Authorization", format!("Bearer {token}"));

            if let Some(ct) = content_type {
                request = request.header("Content-Type", ct);
            }
            if let Some(ref b) = body_bytes {
                request = request.body(b.clone());
            } else {
                // Azure ARM requires Content-Length: 0 for POST with no body (returns 411 otherwise)
                request = request.header("Content-Length", "0");
            }

            let result = match request.send().await {
                Ok(response) => Self::classify_response(response).await,
                Err(e) => Err(AzureError::from(e)),
            };

            match result {
                Ok(response) => return Ok(response),
                Err(azure_err) => {
                    if azure_err.is_retryable() && attempt < self.retry_config.max_retries {
                        let delay = self
                            .retry_config
                            .compute_backoff(backoff, azure_err.retry_after());
                        tokio::time::sleep(delay).await;
                        backoff = std::time::Duration::from_secs_f64(
                            backoff.as_secs_f64() * self.retry_config.backoff_multiplier,
                        );
                        attempt += 1;
                        continue;
                    }
                    return Err(azure_err);
                }
            }
        }
    }

    /// Classify an HTTP response: return Ok for 2xx, parse and return Err for 4xx/5xx.
    async fn classify_response(
        response: reqwest::Response,
    ) -> Result<reqwest::Response, AzureError> {
        let status = response.status().as_u16();
        if status < 400 {
            return Ok(response);
        }

        // Extract Retry-After header before consuming body
        let retry_after_secs: Option<u64> = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok());

        let body_text = response.text().await.unwrap_or_default();

        let mut err = crate::error::parse_json_error(status, &body_text);

        if let Some(secs) = retry_after_secs
            && let AzureError::Throttled { retry_after, .. } = &mut err
        {
            *retry_after = Some(std::time::Duration::from_secs(secs));
        }

        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_succeeds_with_explicit_subscription_id_and_credential() {
        use crate::auth::cli::AzureCliCredential;
        let cred = AzureCredential::AzureCli(AzureCliCredential::new());
        let client = AzureHttpClient::builder()
            .subscription_id("test-sub-123")
            .credential(cred)
            .build();
        assert!(client.is_ok());
        assert_eq!(client.unwrap().subscription_id(), "test-sub-123");
    }

    #[test]
    fn builder_requires_subscription_id_without_env() {
        use crate::auth::cli::AzureCliCredential;
        let cred = AzureCredential::AzureCli(AzureCliCredential::new());
        let result = AzureHttpClientBuilder {
            subscription_id: None,
            retry_config: RetryConfig::default(),
            rate_limit: RateLimitConfig::disabled(),
            credential: Some(cred),
        }
        .build();
        if std::env::var("AZURE_SUBSCRIPTION_ID").is_err() {
            assert!(
                matches!(result, Err(AzureError::Auth { .. })),
                "expected Auth error, got: {:?}",
                result.err()
            );
        }
    }
}
