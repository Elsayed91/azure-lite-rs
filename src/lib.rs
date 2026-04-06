//! Lightweight HTTP client for Microsoft Azure APIs.
//!
//! Provides REST API access with automatic Bearer token injection,
//! retry, and rate limiting.

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
#[cfg(any(test, feature = "test-support"))]
pub mod mock_client;
pub mod ops;
#[cfg(any(test, feature = "test-support"))]
pub mod test_support;
pub mod types;

pub use auth::{AzureCredential, default_credential};
pub use client::{AzureHttpClient, AzureHttpClientBuilder, AzureResponse};
pub use error::{AzureError, Result};
#[cfg(any(test, feature = "test-support"))]
pub use mock_client::MockClient;

/// Build a URL query string from key-value pairs, omitting any with empty values.
#[allow(dead_code)]
pub(crate) fn append_query_params(url: String, params: &[(&str, &str)]) -> String {
    let qs: Vec<String> = params
        .iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect();
    if qs.is_empty() {
        url
    } else {
        format!("{}?{}", url, qs.join("&"))
    }
}
