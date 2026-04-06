//! Operation contracts for the Azure Defender for Cloud API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/security.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::security::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Defender for Cloud API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::security::SecurityClient`] instead.
pub struct SecurityOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> SecurityOps<'a> {
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self { client }
    }

    fn base_url(&self) -> &str {
        #[cfg(any(test, feature = "test-support"))]
        {
            if let Some(ref base) = self.client.base_url {
                return base.trim_end_matches('/');
            }
        }
        "https://management.azure.com"
    }

    /// List all the alerts that are associated with the subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Security/alerts`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`AlertListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_alerts(&self, subscription_id: &str) -> Result<AlertListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Security/alerts",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2020-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_alerts response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_alerts response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get an alert that is associated with a resource group or a resource in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Security/locations/{ascLocation}/alerts/{alertName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `ascLocation` —  *(required)*
    /// - `alertName` —  *(required)*
    ///
    /// # Response
    /// [`Alert`]
    #[allow(dead_code)]
    pub(crate) async fn get_alert(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
    ) -> Result<Alert> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Security/locations/{}/alerts/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(asc_location),
            encode(alert_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2020-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_alert response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_alert response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Update the alert's state (activate, dismiss, resolve, inProgress, close).
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Security/locations/{ascLocation}/alerts/{alertName}/{alertUpdateActionType}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `ascLocation` —  *(required)*
    /// - `alertName` —  *(required)*
    /// - `alertUpdateActionType` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn update_alert_status(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
        alert_update_action_type: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Security/locations/{}/alerts/{}/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(asc_location),
            encode(alert_name),
            encode(alert_update_action_type),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2020-01-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// List secure scores for all your Microsoft Defender for Cloud initiatives within a
    /// subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Security/secureScores`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`SecureScoreListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_secure_scores(
        &self,
        subscription_id: &str,
    ) -> Result<SecureScoreListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Security/secureScores",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2020-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_secure_scores response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_secure_scores response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get secure score for a specific Microsoft Defender for Cloud initiative within the
    /// current scope.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Security/secureScores/{secureScoreName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `secureScoreName` —  *(required)*
    ///
    /// # Response
    /// [`SecureScore`]
    #[allow(dead_code)]
    pub(crate) async fn get_secure_score(
        &self,
        subscription_id: &str,
        secure_score_name: &str,
    ) -> Result<SecureScore> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Security/secureScores/{}",
            self.base_url(),
            encode(subscription_id),
            encode(secure_score_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2020-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_secure_score response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_secure_score response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get security assessments on all your scanned resources inside a subscription scope.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Security/assessments`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`AssessmentListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_assessments(
        &self,
        subscription_id: &str,
    ) -> Result<AssessmentListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Security/assessments",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2020-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_assessments response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_assessments response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get a security assessment on your scanned resource.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Security/assessments/{assessmentName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `assessmentName` —  *(required)*
    ///
    /// # Response
    /// [`Assessment`]
    #[allow(dead_code)]
    pub(crate) async fn get_assessment(
        &self,
        subscription_id: &str,
        assessment_name: &str,
    ) -> Result<Assessment> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Security/assessments/{}",
            self.base_url(),
            encode(subscription_id),
            encode(assessment_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2020-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_assessment response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_assessment response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_alerts() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Security/alerts")
            .returning_json(serde_json::to_value(AlertListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SecurityOps::new(&client);

        let result = ops.list_alerts("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_alert() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Security/locations/test-ascLocation/alerts/test-alertName")
            .returning_json(serde_json::to_value(Alert::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SecurityOps::new(&client);

        let result = ops
            .get_alert(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-ascLocation",
                "test-alertName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_alert_status() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Security/locations/test-ascLocation/alerts/test-alertName/test-alertUpdateActionType")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SecurityOps::new(&client);

        let result = ops
            .update_alert_status(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-ascLocation",
                "test-alertName",
                "test-alertUpdateActionType",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_secure_scores() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Security/secureScores",
        )
        .returning_json(serde_json::to_value(SecureScoreListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SecurityOps::new(&client);

        let result = ops.list_secure_scores("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_secure_score() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Security/secureScores/test-secureScoreName")
            .returning_json(serde_json::to_value(SecureScore::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SecurityOps::new(&client);

        let result = ops
            .get_secure_score("test-subscriptionId", "test-secureScoreName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_assessments() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Security/assessments",
        )
        .returning_json(serde_json::to_value(AssessmentListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SecurityOps::new(&client);

        let result = ops.list_assessments("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_assessment() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Security/assessments/test-assessmentName")
            .returning_json(serde_json::to_value(Assessment::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = SecurityOps::new(&client);

        let result = ops
            .get_assessment("test-subscriptionId", "test-assessmentName")
            .await;
        assert!(result.is_ok());
    }
}
