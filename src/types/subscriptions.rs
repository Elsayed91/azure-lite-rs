//! Types for the Azure Subscriptions API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Summary of a single Azure subscription.
///
/// **Azure API**: `subscriptions.v1.SubscriptionInfo`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/resources/subscriptions/list/SubscriptionInfo>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionInfo {
    /// The subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    /// The display name of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The tenant ID the subscription belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    /// The subscription state: Enabled, Disabled, Warned, Deleted, or PastDue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl SubscriptionInfo {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            subscription_id: Some("test-subscription_id".into()),
            display_name: Some("test-display_name".into()),
            tenant_id: Some("test-tenant_id".into()),
            state: Some("test-state".into()),
        }
    }
}

/// Paginated list of Azure subscriptions.
///
/// **Azure API**: `subscriptions.v1.SubscriptionListResponse`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/resources/subscriptions/list/SubscriptionListResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionListResponse {
    /// The list of subscriptions.
    #[serde(default)]
    pub value: Vec<SubscriptionInfo>,

    /// URL to the next page of results, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl SubscriptionListResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}
