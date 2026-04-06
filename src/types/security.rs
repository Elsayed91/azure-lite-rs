//! Types for the Azure Defender for Cloud API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// Describes the properties of a security alert.
///
/// **Azure API**: `security.v1.AlertProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//AlertProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertProperties {
    /// The display name of the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,

    /// Unique identifier for the detection logic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,

    /// The display name of the resource most related to the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compromised_entity: Option<String>,

    /// Description of the suspected vulnerability and what it means
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The risk level of the threat detected (High, Medium, Low, Informational)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    /// The lifecycle status of the alert (Active, Resolved, Dismissed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The UTC time of the first event or activity included in the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_utc: Option<String>,

    /// The UTC time the alert was generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_generated_utc: Option<String>,

    /// The name of the vendor that raises the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,

    /// Manual action items to take to remediate the alert
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub remediation_steps: Vec<String>,

    /// The kill chain related intent behind the alert (e.g. Initial Access, Defense Evasion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_source: Option<String>,

    /// The kill chain related intent behind the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
}

impl AlertProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            alert_display_name: Some("test-alert_display_name".into()),
            alert_type: Some("test-alert_type".into()),
            compromised_entity: Some("test-compromised_entity".into()),
            description: Some("test-description".into()),
            severity: Some("test-severity".into()),
            status: Some("test-status".into()),
            start_time_utc: Some("test-start_time_utc".into()),
            time_generated_utc: Some("test-time_generated_utc".into()),
            vendor_name: Some("test-vendor_name".into()),
            remediation_steps: vec![],
            system_source: Some("test-system_source".into()),
            intent: Some("test-intent".into()),
        }
    }
}

/// Security alert.
///
/// **Azure API**: `security.v1.Alert`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//Alert>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Alert properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}

impl Alert {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-alert".into()),
            r#type: Some("test-type".into()),
            properties: Some(AlertProperties::fixture()),
        }
    }
}

/// List of security alerts.
///
/// **Azure API**: `security.v1.AlertListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//AlertListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertListResult {
    /// The list of security alerts
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,

    /// The URI to fetch the next page of alerts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl AlertListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Calculation result data.
///
/// **Azure API**: `security.v1.ScoreDetails`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//ScoreDetails>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreDetails {
    /// Maximum score available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,

    /// Current score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,

    /// Ratio of the current score divided by the maximum (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl ScoreDetails {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            max: Some(100),
            ..Default::default()
        }
    }
}

/// Describes the properties of a security score.
///
/// **Azure API**: `security.v1.SecureScoreProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//SecureScoreProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureScoreProperties {
    /// The initiative's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Calculation result data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreDetails>,

    /// The relative weight for each subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

impl SecureScoreProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            display_name: Some("test-display_name".into()),
            score: Some(ScoreDetails::fixture()),
            weight: Some(100),
        }
    }
}

/// Microsoft Defender for Cloud secure score.
///
/// **Azure API**: `security.v1.SecureScore`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//SecureScore>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureScore {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Secure score properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecureScoreProperties>,
}

impl SecureScore {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-secure_score".into()),
            r#type: Some("test-type".into()),
            properties: Some(SecureScoreProperties::fixture()),
        }
    }
}

/// List of secure scores.
///
/// **Azure API**: `security.v1.SecureScoreListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//SecureScoreListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureScoreListResult {
    /// The collection of security scores
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SecureScore>,

    /// The URI to fetch the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl SecureScoreListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// The result of the assessment.
///
/// **Azure API**: `security.v1.AssessmentStatus`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//AssessmentStatus>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentStatus {
    /// Programmatic code for the status of the assessment (Healthy, Unhealthy, NotApplicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Programmatic code for the cause of the assessment status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,

    /// Human readable description of the assessment status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AssessmentStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            cause: Some("test-cause".into()),
            description: Some("test-description".into()),
        }
    }
}

/// Describes the properties of an assessment.
///
/// **Azure API**: `security.v1.AssessmentProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//AssessmentProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentProperties {
    /// User-friendly display name of the assessment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The result of the assessment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssessmentStatus>,

    /// Human readable description of the assessment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Human readable description of what you should do to mitigate this security issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_description: Option<String>,
}

impl AssessmentProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            display_name: Some("test-display_name".into()),
            status: Some(AssessmentStatus::fixture()),
            description: Some("test-description".into()),
            remediation_description: Some("test-remediation_description".into()),
        }
    }
}

/// Security assessment on a resource.
///
/// **Azure API**: `security.v1.Assessment`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//Assessment>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assessment {
    /// Fully qualified resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of the resource
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Assessment properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssessmentProperties>,
}

impl Assessment {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-assessment".into()),
            r#type: Some("test-type".into()),
            properties: Some(AssessmentProperties::fixture()),
        }
    }
}

/// Page of a list of security assessments.
///
/// **Azure API**: `security.v1.AssessmentListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/defenderforcloud//AssessmentListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentListResult {
    /// Collection of security assessments in this page
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Assessment>,

    /// The URI to fetch the next page of assessments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl AssessmentListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}
