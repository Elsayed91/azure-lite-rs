//! MockClient helpers for Azure Subscriptions API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Subscriptions helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SubscriptionsMockHelpers {
    /// Helper to expect `list_subscriptions`: List all subscriptions accessible to the
    /// authenticated principal.
    fn expect_list_subscriptions(&mut self) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SubscriptionsMockHelpers for MockClient {
    /// Helper to expect `list_subscriptions`: List all subscriptions accessible to the
    /// authenticated principal.
    fn expect_list_subscriptions(&mut self) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = "/subscriptions".to_string();
        self.expect_get(&path)
    }
}
