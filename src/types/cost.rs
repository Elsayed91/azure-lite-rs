//! Types for the Azure Cost Management API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};

/// The start and end date for a budget.
///
/// **Azure API**: `cost.v1.BudgetTimePeriod`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//BudgetTimePeriod>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetTimePeriod {
    /// The start date for the budget (ISO 8601 date)
    pub start_date: String,

    /// The end date for the budget (ISO 8601 date)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl BudgetTimePeriod {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            start_date: "test-start_date".into(),
            end_date: Some("test-end_date".into()),
        }
    }
}

/// The current amount of cost which is being tracked for a budget.
///
/// **Azure API**: `cost.v1.CurrentSpend`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//CurrentSpend>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentSpend {
    /// The total amount of cost which is being tracked by the budget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    /// The unit of measure for the budget amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl CurrentSpend {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            unit: Some("test-unit".into()),
            ..Default::default()
        }
    }
}

/// The properties of a budget.
///
/// **Azure API**: `cost.v1.BudgetProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//BudgetProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetProperties {
    /// The category of the budget (Cost or Usage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// The total amount of cost to track with the budget
    pub amount: f64,

    /// The time covered by a budget (Monthly, Quarterly, Annually, BillingMonth,
    /// BillingQuarter, BillingAnnual)
    pub time_grain: String,

    /// Has start and end date of the budget
    pub time_period: BudgetTimePeriod,

    /// The current amount of cost which is being tracked for a budget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,

    /// May be used to filter budgets by resource group, resource, or meter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl BudgetProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            category: Some("test-category".into()),
            time_grain: "test-time_grain".into(),
            time_period: BudgetTimePeriod::fixture(),
            current_spend: Some(CurrentSpend::fixture()),
            ..Default::default()
        }
    }
}

/// A budget resource.
///
/// **Azure API**: `cost.v1.Budget`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//Budget>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    /// Resource identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// eTag of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,

    /// The properties of the budget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BudgetProperties>,
}

impl Budget {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-budget".into()),
            r#type: Some("test-type".into()),
            e_tag: Some("test-e_tag".into()),
            properties: Some(BudgetProperties::fixture()),
        }
    }
}

/// Result of listing budgets.
///
/// **Azure API**: `cost.v1.BudgetListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//BudgetListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetListResult {
    /// The list of budgets
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Budget>,

    /// The link (URL) to the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl BudgetListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// The request body for creating or updating a budget.
///
/// **Azure API**: `cost.v1.BudgetCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//BudgetCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetCreateRequest {
    /// eTag of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,

    /// The properties of the budget
    pub properties: BudgetProperties,
}

impl BudgetCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            e_tag: Some("test-e_tag".into()),
            properties: BudgetProperties::fixture(),
        }
    }
}

/// QueryColumn.
///
/// **Azure API**: `cost.v1.QueryColumn`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//QueryColumn>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryColumn {
    /// The name of column
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of column
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl QueryColumn {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-query_column".into()),
            r#type: Some("test-type".into()),
        }
    }
}

/// QueryResult.
///
/// **Azure API**: `cost.v1.QueryProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//QueryProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryProperties {
    /// Array of columns
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<QueryColumn>,

    /// Array of rows
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<serde_json::Value>,

    /// The link to the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl QueryProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            columns: vec![],
            rows: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Result of query. It contains all columns listed under groupings and aggregation.
///
/// **Azure API**: `cost.v1.QueryResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//QueryResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    /// Resource id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Query result properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}

impl QueryResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-query_result".into()),
            r#type: Some("test-type".into()),
            properties: Some(QueryProperties::fixture()),
        }
    }
}

/// The start and end date for pulling data for the query.
///
/// **Azure API**: `cost.v1.QueryTimePeriod`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//QueryTimePeriod>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTimePeriod {
    /// The start date to pull data from
    #[serde(rename = "from")]
    pub from: String,

    /// The end date to pull data to
    #[serde(rename = "to")]
    pub to: String,
}

impl QueryTimePeriod {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            from: "test-from".into(),
            to: "test-to".into(),
        }
    }
}

/// The definition of data present in the query.
///
/// **Azure API**: `cost.v1.QueryDataset`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//QueryDataset>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDataset {
    /// The granularity of rows in the query (None, Daily)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,

    /// Dictionary of aggregation expression to use in the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,

    /// Array of group by expression to use in the query
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<serde_json::Value>,

    /// Has filter expression to use in the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl QueryDataset {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            granularity: Some("test-granularity".into()),
            grouping: vec![],
            ..Default::default()
        }
    }
}

/// The definition of a query.
///
/// **Azure API**: `cost.v1.QueryDefinition`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//QueryDefinition>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDefinition {
    /// The type of the query (Usage, ActualCost, AmortizedCost)
    #[serde(rename = "type")]
    pub r#type: String,

    /// The time frame for pulling data for the query (WeekToDate, MonthToDate,
    /// BillingMonthToDate, TheLastBillingMonth, TheLastMonth, Custom)
    pub timeframe: String,

    /// Has time period for pulling data for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,

    /// Has definition for data in this query
    pub dataset: QueryDataset,
}

impl QueryDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            r#type: "test-type".into(),
            timeframe: "test-timeframe".into(),
            time_period: Some(QueryTimePeriod::fixture()),
            dataset: QueryDataset::fixture(),
        }
    }
}

/// The definition of a forecast.
///
/// **Azure API**: `cost.v1.ForecastDefinition`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//ForecastDefinition>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastDefinition {
    /// The type of the forecast (Usage, ActualCost, AmortizedCost)
    #[serde(rename = "type")]
    pub r#type: String,

    /// The time frame for pulling data for the forecast (WeekToDate, MonthToDate,
    /// BillingMonthToDate, TheLastBillingMonth, TheLastMonth, Custom)
    pub timeframe: String,

    /// Has time period for pulling data for the forecast
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,

    /// Has definition for data in this forecast
    pub dataset: QueryDataset,

    /// a flag which indicates whether actuals cost will be included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_actual_cost: Option<bool>,

    /// a flag which indicates whether fresh partial data will be included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fresh_partial_cost: Option<bool>,
}

impl ForecastDefinition {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            r#type: "test-type".into(),
            timeframe: "test-timeframe".into(),
            time_period: Some(QueryTimePeriod::fixture()),
            dataset: QueryDataset::fixture(),
            include_actual_cost: Some(false),
            include_fresh_partial_cost: Some(false),
        }
    }
}

/// The properties of a legacy usage detail.
///
/// **Azure API**: `cost.v1.UsageDetailProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//UsageDetailProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageDetailProperties {
    /// The id of the billing period resource that the usage belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,

    /// The start of the date time range covered by the usage detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,

    /// The end of the date time range covered by the usage detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,

    /// The uri of the resource instance that the usage is about
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// The name of the resource instance that the usage is about
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,

    /// The resource group name of the usage detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,

    /// The name of the product that the consumption is about
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// The ID of the meter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,

    /// The pretax charged amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretax_cost: Option<f64>,

    /// The ISO currency in which the meter is charged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// The quantity of usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_quantity: Option<f64>,

    /// The unit of measure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
}

impl UsageDetailProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            billing_period_id: Some("test-billing_period_id".into()),
            usage_start: Some("test-usage_start".into()),
            usage_end: Some("test-usage_end".into()),
            instance_id: Some("test-instance_id".into()),
            instance_name: Some("test-instance_name".into()),
            resource_group: Some("test-resource_group".into()),
            product: Some("test-product".into()),
            meter_id: Some("test-meter_id".into()),
            currency: Some("test-currency".into()),
            unit_of_measure: Some("test-unit_of_measure".into()),
            ..Default::default()
        }
    }
}

/// An usage detail resource.
///
/// **Azure API**: `cost.v1.UsageDetail`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//UsageDetail>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageDetail {
    /// Resource id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The etag for the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The properties of the usage detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsageDetailProperties>,
}

impl UsageDetail {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-usage_detail".into()),
            r#type: Some("test-type".into()),
            etag: Some("test-etag".into()),
            properties: Some(UsageDetailProperties::fixture()),
        }
    }
}

/// Result of listing usage details.
///
/// **Azure API**: `cost.v1.UsageDetailsListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/cost-management//UsageDetailsListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageDetailsListResult {
    /// The list of usage details
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageDetail>,

    /// The link to the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl UsageDetailsListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}
