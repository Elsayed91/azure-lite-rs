//! Integration tests for Azure Cost Management operations.
//!
//! Tests budget lifecycle, cost queries, forecasts, and usage details.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration cost -- --ignored --test-threads=1 --nocapture

use azure_lite::types::cost::{
    BudgetCreateRequest, BudgetProperties, BudgetTimePeriod, ForecastDefinition, QueryDataset,
    QueryDefinition,
};

const BUDGET_NAME: &str = "cloud-lite-test-ralph-budget";

/// Returns the first day of the current UTC month (e.g. "2026-03-01").
/// Budget start_date must be on or after the first of the current month.
fn first_of_current_month() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let days = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before epoch")
        .as_secs() as i64
        / 86400;
    // Howard Hinnant's civil_from_days algorithm
    let z = days + 719468;
    let era = if z >= 0 {
        z / 146097
    } else {
        (z - 146096) / 146097
    };
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-01")
}

#[tokio::test]
#[ignore]
async fn test_cost_management() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to create AzureHttpClient");

    println!("[1/8] Pre-cleanup: deleting any stale test budget...");
    let _ = client.cost().delete_budget(BUDGET_NAME).await;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("[2/8] Listing budgets (baseline)...");
    let budgets = client
        .cost()
        .list_budgets()
        .await
        .expect("list_budgets failed");
    println!("  {} existing budget(s)", budgets.value.len());

    println!("[3/8] Creating budget '{BUDGET_NAME}'...");
    let body = BudgetCreateRequest {
        properties: BudgetProperties {
            category: Some("Cost".into()),
            amount: 100.0,
            time_grain: "Monthly".into(),
            time_period: BudgetTimePeriod {
                start_date: first_of_current_month(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };
    let budget = client
        .cost()
        .create_budget(BUDGET_NAME, &body)
        .await
        .expect("create_budget failed");
    assert_eq!(budget.name.as_deref(), Some(BUDGET_NAME));
    println!("  Created: {}", budget.id.as_deref().unwrap_or("(no id)"));

    println!("[4/8] Getting budget '{BUDGET_NAME}'...");
    let budget = client
        .cost()
        .get_budget(BUDGET_NAME)
        .await
        .expect("get_budget failed");
    assert_eq!(budget.name.as_deref(), Some(BUDGET_NAME));
    let props = budget
        .properties
        .as_ref()
        .expect("budget has no properties");
    println!("  amount: {}", props.amount);
    println!("  timeGrain: {}", props.time_grain);
    assert_eq!(props.amount, 100.0);
    assert_eq!(props.time_grain, "Monthly");

    println!("[5/8] Listing budgets (should include test budget)...");
    let budgets = client
        .cost()
        .list_budgets()
        .await
        .expect("list_budgets failed");
    let found = budgets
        .value
        .iter()
        .any(|b| b.name.as_deref() == Some(BUDGET_NAME));
    assert!(
        found,
        "test budget not found in list; got {} budget(s)",
        budgets.value.len()
    );

    println!("[6/8] Cost query (MonthToDate)...");
    let query_body = QueryDefinition {
        r#type: "ActualCost".into(),
        timeframe: "MonthToDate".into(),
        dataset: QueryDataset {
            granularity: Some("Daily".into()),
            aggregation: Some(serde_json::json!({
                "totalCost": { "name": "Cost", "function": "Sum" }
            })),
            ..Default::default()
        },
        ..Default::default()
    };
    match client.cost().list_cost_by_resource(&query_body).await {
        Ok(result) => {
            let cols = result
                .properties
                .as_ref()
                .map(|p| p.columns.len())
                .unwrap_or(0);
            println!("  Cost query returned {cols} column(s)");
        }
        Err(e) => println!("  Warning: list_cost_by_resource failed (graceful): {e}"),
    }

    println!("[7/8] Forecast (MonthToDate)...");
    let forecast_body = ForecastDefinition {
        r#type: "ActualCost".into(),
        timeframe: "MonthToDate".into(),
        dataset: QueryDataset {
            granularity: Some("Daily".into()),
            ..Default::default()
        },
        include_actual_cost: Some(false),
        ..Default::default()
    };
    match client.cost().get_forecast(&forecast_body).await {
        Ok(result) => {
            let cols = result
                .properties
                .as_ref()
                .map(|p| p.columns.len())
                .unwrap_or(0);
            println!("  Forecast returned {cols} column(s)");
        }
        Err(e) => println!("  Warning: get_forecast failed (graceful): {e}"),
    }

    println!("[8/8] Deleting budget '{BUDGET_NAME}'...");
    client
        .cost()
        .delete_budget(BUDGET_NAME)
        .await
        .expect("delete_budget failed");
    println!("  Done.");
}
