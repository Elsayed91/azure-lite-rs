//! Test support utilities and MockClient helpers.
//!
//! This module provides extension traits for `MockClient` that make test setup more ergonomic.
//! Each API has its own helper trait with `expect_*` methods for ergonomic test setup.
//!
//! # Example
//!
//! ```no_run
//! use azure_lite::MockClient;
//! use azure_lite::test_support::AcrMockHelpers;
//!
//! let mut mock = MockClient::new();
//! ```

#[cfg(any(test, feature = "test-support"))]
pub mod acr_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod aks_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod compute_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cosmosdb_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod cost_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod dns_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod functions_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod identity_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod keyvault_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod loganalytics_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod monitor_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod networking_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod rbac_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod redis_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod resource_graph_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod security_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod sql_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod storage_mock_helpers;
#[cfg(any(test, feature = "test-support"))]
pub mod subscriptions_mock_helpers;

// Re-export traits for convenience
#[cfg(any(test, feature = "test-support"))]
pub use acr_mock_helpers::AcrMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use aks_mock_helpers::AksMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use compute_mock_helpers::ComputeMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cosmosdb_mock_helpers::CosmosdbMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use cost_mock_helpers::CostMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use dns_mock_helpers::DnsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use functions_mock_helpers::FunctionsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use identity_mock_helpers::IdentityMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use keyvault_mock_helpers::KeyvaultMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use loganalytics_mock_helpers::LoganalyticsMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use monitor_mock_helpers::MonitorMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use networking_mock_helpers::NetworkingMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use rbac_mock_helpers::RbacMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use redis_mock_helpers::RedisMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use resource_graph_mock_helpers::ResourceGraphMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use security_mock_helpers::SecurityMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use sql_mock_helpers::SqlMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use storage_mock_helpers::StorageMockHelpers;
#[cfg(any(test, feature = "test-support"))]
pub use subscriptions_mock_helpers::SubscriptionsMockHelpers;
