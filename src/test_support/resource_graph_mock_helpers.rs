//! MockClient helpers for Azure Resource Graph API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Resource Graph helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ResourceGraphMockHelpers {
    /// Helper to expect `query_resources`: Execute a KQL query across one or more subscriptions.
    fn expect_query_resources(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ResourceGraphMockHelpers for MockClient {
    /// Helper to expect `query_resources`: Execute a KQL query across one or more subscriptions.
    fn expect_query_resources(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/providers/Microsoft.ResourceGraph/resources".to_string();
        self.expect_post(&path)
    }
}
