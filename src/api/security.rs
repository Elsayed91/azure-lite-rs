//! Azure Defender for Cloud API client.
//!
//! Wraps the ARM management plane read operations for Defender for Cloud:
//! alerts, secure scores, and assessments. All URL construction is in
//! `ops::security::SecurityOps`.
//! `subscription_id` is auto-injected from the parent `AzureHttpClient`.

use crate::{
    AzureHttpClient, Result,
    ops::security::SecurityOps,
    types::security::{
        Alert, AlertListResult, Assessment, AssessmentListResult, SecureScore,
        SecureScoreListResult,
    },
};

/// Client for the Azure Defender for Cloud ARM management plane.
///
/// Wraps [`SecurityOps`] with ergonomic signatures that auto-inject
/// `subscription_id` from the parent [`AzureHttpClient`].
pub struct SecurityClient<'a> {
    ops: SecurityOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> SecurityClient<'a> {
    /// Create a new Azure Defender for Cloud API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: SecurityOps::new(client),
            client,
        }
    }

    // --- Alert operations ---

    /// Lists all alerts associated with the subscription.
    pub async fn list_alerts(&self) -> Result<AlertListResult> {
        self.ops.list_alerts(self.client.subscription_id()).await
    }

    /// Gets an alert within a resource group.
    ///
    /// `asc_location` is the ASC location (e.g. `"eastus"`).
    /// Use `az security location list` to discover the location for your tenant.
    pub async fn get_alert(
        &self,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
    ) -> Result<Alert> {
        self.ops
            .get_alert(
                self.client.subscription_id(),
                resource_group_name,
                asc_location,
                alert_name,
            )
            .await
    }

    /// Updates the status of an alert.
    ///
    /// `alert_update_action_type` is one of: `activate`, `dismiss`, `resolve`,
    /// `inProgress`, `close`.
    pub async fn update_alert_status(
        &self,
        resource_group_name: &str,
        asc_location: &str,
        alert_name: &str,
        alert_update_action_type: &str,
    ) -> Result<()> {
        self.ops
            .update_alert_status(
                self.client.subscription_id(),
                resource_group_name,
                asc_location,
                alert_name,
                alert_update_action_type,
            )
            .await
    }

    // --- Secure Score operations ---

    /// Lists all secure scores for the subscription.
    pub async fn list_secure_scores(&self) -> Result<SecureScoreListResult> {
        self.ops
            .list_secure_scores(self.client.subscription_id())
            .await
    }

    /// Gets the secure score for a specific Defender for Cloud initiative.
    ///
    /// Use `"ascScore"` for the built-in Microsoft Defender for Cloud score.
    pub async fn get_secure_score(&self, secure_score_name: &str) -> Result<SecureScore> {
        self.ops
            .get_secure_score(self.client.subscription_id(), secure_score_name)
            .await
    }

    // --- Assessment operations ---

    /// Lists all security assessments for the subscription.
    pub async fn list_assessments(&self) -> Result<AssessmentListResult> {
        self.ops
            .list_assessments(self.client.subscription_id())
            .await
    }

    /// Gets a specific security assessment.
    ///
    /// `assessment_name` is the UUID of the assessment definition
    /// (e.g. `"4fb67663-9ab9-475d-b026-8c544cced439"` for OS vulnerabilities).
    ///
    /// NOTE: This endpoint requires `api-version=2021-06-01` on the live API,
    /// while the service manifest uses `2020-01-01` (required by secureScores).
    /// As a result this will return 405 against the live API. Use the unit test
    /// mock to verify the URL and deserialization logic.
    pub async fn get_assessment(&self, assessment_name: &str) -> Result<Assessment> {
        self.ops
            .get_assessment(self.client.subscription_id(), assessment_name)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    const SUB_ID: &str = "test-subscription-id";
    const ALERT_NAME: &str = "2517376_cloud-lite-test-alert";
    const ASC_LOCATION: &str = "eastus";
    const RG: &str = "test-rg";
    const ASSESSMENT_NAME: &str = "050ac097-3dda-4d24-ab6d-82568e7a50cf";

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn alert_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Security/locations/{ASC_LOCATION}/alerts/{ALERT_NAME}"),
            "name": ALERT_NAME,
            "type": "Microsoft.Security/alerts",
            "properties": {
                "alertDisplayName": "Suspicious PowerShell activity",
                "alertType": "VM_SuspiciousPowerShellActivity",
                "severity": "High",
                "status": "Active",
                "description": "A suspicious PowerShell command was executed.",
                "remediationSteps": ["Investigate the activity", "Revoke credentials"],
                "intent": "Execution"
            }
        })
    }

    fn secure_score_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/providers/Microsoft.Security/secureScores/ascScore"),
            "name": "ascScore",
            "type": "Microsoft.Security/secureScores",
            "properties": {
                "displayName": "ASC score",
                "score": {
                    "max": 21,
                    "current": 12.55,
                    "percentage": 0.5976
                },
                "weight": 1
            }
        })
    }

    fn assessment_json() -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/{SUB_ID}/providers/Microsoft.Security/assessments/{ASSESSMENT_NAME}"),
            "name": ASSESSMENT_NAME,
            "type": "Microsoft.Security/assessments",
            "properties": {
                "displayName": "Disabled accounts with owner permissions should be removed",
                "status": {
                    "code": "Healthy",
                    "cause": "OffByPolicy",
                    "description": "All accounts are enabled"
                }
            }
        })
    }

    #[tokio::test]
    async fn list_alerts_returns_list() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Security/alerts"
        ))
        .returning_json(serde_json::json!({ "value": [alert_json()] }));
        let client = make_client(mock);
        let result = client
            .security()
            .list_alerts()
            .await
            .expect("list_alerts failed");
        assert_eq!(result.value.len(), 1);
        let a = &result.value[0];
        assert_eq!(a.name.as_deref(), Some(ALERT_NAME));
        let props = a.properties.as_ref().unwrap();
        assert_eq!(props.severity.as_deref(), Some("High"));
        assert_eq!(props.status.as_deref(), Some("Active"));
        assert_eq!(props.remediation_steps.len(), 2);
    }

    #[tokio::test]
    async fn get_alert_deserializes_properties() {
        let mut mock = MockClient::new();
        mock.expect_get(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Security/locations/{ASC_LOCATION}/alerts/{ALERT_NAME}"),
        )
        .returning_json(alert_json());
        let client = make_client(mock);
        let a = client
            .security()
            .get_alert(RG, ASC_LOCATION, ALERT_NAME)
            .await
            .expect("get_alert failed");
        assert_eq!(a.name.as_deref(), Some(ALERT_NAME));
        let props = a.properties.as_ref().unwrap();
        assert_eq!(
            props.alert_type.as_deref(),
            Some("VM_SuspiciousPowerShellActivity")
        );
        assert_eq!(props.intent.as_deref(), Some("Execution"));
    }

    #[tokio::test]
    async fn update_alert_status_constructs_url() {
        let mut mock = MockClient::new();
        mock.expect_post(
            &format!("/subscriptions/{SUB_ID}/resourceGroups/{RG}/providers/Microsoft.Security/locations/{ASC_LOCATION}/alerts/{ALERT_NAME}/dismiss"),
        )
        .returning_json(serde_json::json!({}));
        let client = make_client(mock);
        client
            .security()
            .update_alert_status(RG, ASC_LOCATION, ALERT_NAME, "dismiss")
            .await
            .expect("update_alert_status failed");
    }

    #[tokio::test]
    async fn list_secure_scores_returns_asc_score() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Security/secureScores"
        ))
        .returning_json(serde_json::json!({ "value": [secure_score_json()] }));
        let client = make_client(mock);
        let result = client
            .security()
            .list_secure_scores()
            .await
            .expect("list_secure_scores failed");
        assert_eq!(result.value.len(), 1);
        let score = &result.value[0];
        assert_eq!(score.name.as_deref(), Some("ascScore"));
        let props = score.properties.as_ref().unwrap();
        let details = props.score.as_ref().unwrap();
        assert_eq!(details.max, Some(21));
        assert_eq!(details.current, Some(12.55));
    }

    #[tokio::test]
    async fn get_assessment_deserializes_status() {
        let mut mock = MockClient::new();
        mock.expect_get(&format!(
            "/subscriptions/{SUB_ID}/providers/Microsoft.Security/assessments/{ASSESSMENT_NAME}"
        ))
        .returning_json(assessment_json());
        let client = make_client(mock);
        let a = client
            .security()
            .get_assessment(ASSESSMENT_NAME)
            .await
            .expect("get_assessment failed");
        assert_eq!(a.name.as_deref(), Some(ASSESSMENT_NAME));
        let props = a.properties.as_ref().unwrap();
        let status = props.status.as_ref().unwrap();
        assert_eq!(status.code.as_deref(), Some("Healthy"));
        assert_eq!(status.cause.as_deref(), Some("OffByPolicy"));
    }
}
