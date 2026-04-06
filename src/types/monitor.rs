//! Types for the Azure Monitor API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metric availability specifies the time grain.
///
/// **Azure API**: `monitor.v1.MetricAvailability`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricAvailability>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricAvailability {
    /// The time grain specifies the aggregation interval for the metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,

    /// The retention period for the metric at the specified timegrain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}

impl MetricAvailability {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            time_grain: Some("test-time_grain".into()),
            retention: Some("test-retention".into()),
        }
    }
}

/// Metric definition class specifies the metadata for a metric.
///
/// **Azure API**: `monitor.v1.MetricDefinition`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricDefinition>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricDefinition {
    /// The resource identifier of the metric definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name and the display name of the metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The namespace the metric belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The unit of the metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// The primary aggregation type (None, Average, Count, Minimum, Maximum, Total)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<String>,
}

impl MetricDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-metric_definition".into()),
            namespace: Some("test-namespace".into()),
            unit: Some("test-unit".into()),
            primary_aggregation_type: Some("test-primary_aggregation_type".into()),
        }
    }
}

/// Represents collection of metric definitions.
///
/// **Azure API**: `monitor.v1.MetricDefinitionCollection`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricDefinitionCollection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricDefinitionCollection {
    /// The values for the metric definitions
    #[serde(default)]
    pub value: Vec<MetricDefinition>,
}

impl MetricDefinitionCollection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}

/// Represents a metric value.
///
/// **Azure API**: `monitor.v1.MetricValue`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricValue>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricValue {
    /// The timestamp for the metric value
    pub time_stamp: String,

    /// The average value in the time range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,

    /// The minimum value in the time range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    /// The maximum value in the time range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    /// The sum of all values in the time range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,

    /// The number of samples in the time range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
}

impl MetricValue {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            time_stamp: "test-time_stamp".into(),
            ..Default::default()
        }
    }
}

/// A time series result for a metric.
///
/// **Azure API**: `monitor.v1.TimeSeriesElement`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//TimeSeriesElement>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesElement {
    /// An array of data points representing the metric values
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<MetricValue>,
}

impl TimeSeriesElement {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { data: vec![] }
    }
}

/// The result data of a query.
///
/// **Azure API**: `monitor.v1.Metric`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//Metric>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    /// The metric Id
    pub id: String,

    /// The resource type of the metric resource
    #[serde(rename = "type")]
    pub r#type: String,

    /// The name and the display name of the metric
    pub name: String,

    /// The unit of the metric
    pub unit: String,

    /// The time series returned when a data query is performed
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub timeseries: Vec<TimeSeriesElement>,
}

impl Metric {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: "test-id".into(),
            r#type: "test-type".into(),
            name: "test-metric".into(),
            unit: "test-unit".into(),
            timeseries: vec![],
        }
    }
}

/// The response to a metrics query.
///
/// **Azure API**: `monitor.v1.MetricsResponse`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricsResponse>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsResponse {
    /// The value of the collection
    #[serde(default)]
    pub value: Vec<Metric>,

    /// The integer value representing the relative cost of the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i32>,

    /// The timespan for which the data was retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timespan: Option<String>,

    /// The interval (window size) for which the metric data was returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    /// The namespace of the metrics been queried
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl MetricsResponse {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            cost: Some(100),
            timespan: Some("test-timespan".into()),
            interval: Some("test-interval".into()),
            namespace: Some("test-namespace".into()),
        }
    }
}

/// An alert rule.
///
/// **Azure API**: `monitor.v1.MetricAlertProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricAlertProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricAlertProperties {
    /// The description of the alert rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Alert severity (0-4, where 0 is critical)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,

    /// Whether the alert rule is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// How often the alert rule is evaluated in ISO 8601 duration format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_frequency: Option<String>,

    /// The period of time in ISO 8601 duration format used to monitor alert activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<String>,

    /// The provisioning state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The resource type of the target resource(s) on which the alert is created/updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,

    /// The region of the target resource(s) on which the alert is created/updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_region: Option<String>,
}

impl MetricAlertProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            description: Some("test-description".into()),
            severity: Some(100),
            enabled: Some(false),
            evaluation_frequency: Some("test-evaluation_frequency".into()),
            window_size: Some("test-window_size".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            target_resource_type: Some("test-target_resource_type".into()),
            target_resource_region: Some("test-target_resource_region".into()),
        }
    }
}

/// The metric alert resource.
///
/// **Azure API**: `monitor.v1.MetricAlertResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricAlertResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricAlertResource {
    /// Azure resource Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Azure resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Azure resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The alert rule properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertProperties>,
}

impl MetricAlertResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-metric_alert_resource".into()),
            r#type: Some("test-type".into()),
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(MetricAlertProperties::fixture()),
        }
    }
}

/// Represents a collection of alert rule resources.
///
/// **Azure API**: `monitor.v1.MetricAlertResourceCollection`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricAlertResourceCollection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricAlertResourceCollection {
    /// The values for the alert rule resources
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetricAlertResource>,
}

impl MetricAlertResourceCollection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self { value: vec![] }
    }
}

/// The metric alert resource for patch operations.
///
/// **Azure API**: `monitor.v1.MetricAlertResourcePatch`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricAlertResourcePatch>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricAlertResourcePatch {
    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The alert rule properties for an update operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertProperties>,
}

impl MetricAlertResourcePatch {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tags: Default::default(),
            properties: Some(MetricAlertProperties::fixture()),
        }
    }
}

/// The metric alert resource.
///
/// **Azure API**: `monitor.v1.MetricAlertCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//MetricAlertCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricAlertCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The alert rule properties of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetricAlertProperties>,
}

impl MetricAlertCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(MetricAlertProperties::fixture()),
        }
    }
}

/// The Azure event log entries are of type EventData.
///
/// **Azure API**: `monitor.v1.EventData`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//EventData>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    /// The Id of this event as required by ARM for RBAC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The timestamp of when the event was generated by the Azure service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<String>,

    /// The operation name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,

    /// The Resource Group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,

    /// The resource type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,

    /// The resource URI that has generated the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// A string describing the status of the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The event sub status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<String>,

    /// The event level (Critical, Error, Warning, Informational, Verbose)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    /// The email address of the user who performed the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,

    /// The description of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl EventData {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            event_timestamp: Some("test-event_timestamp".into()),
            operation_name: Some("test-operation_name".into()),
            resource_group_name: Some("test-resource_group_name".into()),
            resource_type: Some("test-resource_type".into()),
            resource_id: Some("test-resource_id".into()),
            status: Some("test-status".into()),
            sub_status: Some("test-sub_status".into()),
            level: Some("test-level".into()),
            caller: Some("test-caller".into()),
            description: Some("test-description".into()),
        }
    }
}

/// Represents collection of events.
///
/// **Azure API**: `monitor.v1.EventDataCollection`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/monitor//EventDataCollection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDataCollection {
    /// This lists the events
    #[serde(default)]
    pub value: Vec<EventData>,

    /// Provides the link to retrieve the next set of events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl EventDataCollection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}
