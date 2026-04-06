//! MockClient helpers for Azure Defender for Cloud API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Defender for Cloud helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait SecurityMockHelpers {
    /// Helper to expect `list_alerts`: List all the alerts that are associated with the
    /// subscription.
    fn expect_list_alerts(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_alert`: Get an alert that is associated with a resource group or a
    /// resource in a resource group.
    fn expect_get_alert(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_alert_status`: Update the alert's state (activate, dismiss,
    /// resolve, inProgress, close).
    fn expect_update_alert_status(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
        alert_update_action_type: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_secure_scores`: List secure scores for all your Microsoft Defender
    /// for Cloud initiatives within a subscription.
    fn expect_list_secure_scores(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_secure_score`: Get secure score for a specific Microsoft Defender for
    /// Cloud initiative within the current scope.
    fn expect_get_secure_score(
        &mut self,
        subscription_id: &str,
        secure_score_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_assessments`: Get security assessments on all your scanned resources
    /// inside a subscription scope.
    fn expect_list_assessments(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_assessment`: Get a security assessment on your scanned resource.
    fn expect_get_assessment(
        &mut self,
        subscription_id: &str,
        assessment_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl SecurityMockHelpers for MockClient {
    /// Helper to expect `list_alerts`: List all the alerts that are associated with the
    /// subscription.
    fn expect_list_alerts(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/subscriptions/{subscription_id}/providers/Microsoft.Security/alerts");
        self.expect_get(&path)
    }

    /// Helper to expect `get_alert`: Get an alert that is associated with a resource group or a
    /// resource in a resource group.
    fn expect_get_alert(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Security/locations/{asc_location}/alerts/{alert_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `update_alert_status`: Update the alert's state (activate, dismiss,
    /// resolve, inProgress, close).
    fn expect_update_alert_status(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
        alert_update_action_type: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Security/locations/{asc_location}/alerts/{alert_name}/{alert_update_action_type}"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `list_secure_scores`: List secure scores for all your Microsoft Defender
    /// for Cloud initiatives within a subscription.
    fn expect_list_secure_scores(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.Security/secureScores");
        self.expect_get(&path)
    }

    /// Helper to expect `get_secure_score`: Get secure score for a specific Microsoft Defender for
    /// Cloud initiative within the current scope.
    fn expect_get_secure_score(
        &mut self,
        subscription_id: &str,
        secure_score_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Security/secureScores/{secure_score_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_assessments`: Get security assessments on all your scanned resources
    /// inside a subscription scope.
    fn expect_list_assessments(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.Security/assessments");
        self.expect_get(&path)
    }

    /// Helper to expect `get_assessment`: Get a security assessment on your scanned resource.
    fn expect_get_assessment(
        &mut self,
        subscription_id: &str,
        assessment_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Security/assessments/{assessment_name}"
        );
        self.expect_get(&path)
    }
}
