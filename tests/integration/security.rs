//! Integration tests for Azure Defender for Cloud operations.
//!
//! All operations are read-only (no resources created or modified).
//! Verifies that list/get operations return well-formed data.
//! UpdateAlertStatus is not tested in integration — it would modify real
//! security state and cannot be deterministically set up.
//!
//! Requires: Azure CLI login + AZURE_SUBSCRIPTION_ID
//!
//! Run: cargo test -p azure-lite --test integration security -- --ignored --test-threads=1 --nocapture

// ============================================================================
// Helpers
// ============================================================================

/// Parse the ascLocation and alertName from an ARM alert resource ID.
/// ID format: /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Security/locations/{location}/alerts/{name}
fn parse_alert_id(id: &str) -> Option<(&str, &str, &str)> {
    let parts: Vec<&str> = id.split('/').collect();
    // Expected: ["", "subscriptions", sub, "resourceGroups", rg, "providers",
    //            "Microsoft.Security", "locations", location, "alerts", name]
    if parts.len() >= 11 {
        let rg = parts.get(4)?;
        let location = parts.get(8)?;
        let name = parts.get(10)?;
        Some((rg, location, name))
    } else {
        None
    }
}

// ============================================================================
// Integration Test
// ============================================================================

#[tokio::test]
#[ignore = "requires Azure credentials"]
async fn security_read_operations() {
    let client = azure_lite::AzureHttpClient::from_env()
        .await
        .expect("Failed to build AzureHttpClient from env");

    let sec = client.security();

    // =========================================================================
    // Step 1: List alerts
    // =========================================================================
    println!("[1/5] Listing security alerts...");
    let alerts = sec.list_alerts().await.expect("list_alerts failed");
    println!("  Found {} alert(s).", alerts.value.len());
    // Validate structure of any alerts present
    for alert in &alerts.value {
        if let Some(props) = &alert.properties {
            assert!(
                props.severity.is_some() || props.alert_type.is_some(),
                "Alert should have at least severity or alertType",
            );
        }
    }

    // =========================================================================
    // Step 2: Get alert (if any exist)
    // =========================================================================
    if let Some(alert) = alerts.value.first() {
        if let Some(id) = alert.id.as_deref() {
            println!("[2/5] Getting first alert by ID: {id}");
            if let Some((rg, location, name)) = parse_alert_id(id) {
                let got = sec
                    .get_alert(rg, location, name)
                    .await
                    .expect("get_alert failed");
                assert_eq!(
                    got.name.as_deref(),
                    alert.name.as_deref(),
                    "name should match list"
                );
                let props = got
                    .properties
                    .as_ref()
                    .expect("alert should have properties");
                println!(
                    "  Got alert: displayName={:?}, severity={:?}, status={:?}",
                    props.alert_display_name.as_deref(),
                    props.severity.as_deref(),
                    props.status.as_deref(),
                );
            } else {
                println!("[2/5] Skipped get_alert — could not parse alert ID.");
            }
        } else {
            println!("[2/5] Skipped get_alert — first alert has no ID.");
        }
    } else {
        println!("[2/5] Skipped get_alert — no alerts in subscription.");
    }

    // =========================================================================
    // Step 3: List secure scores
    // =========================================================================
    println!("[3/5] Listing secure scores...");
    let scores = sec
        .list_secure_scores()
        .await
        .expect("list_secure_scores failed");
    println!("  Found {} secure score(s).", scores.value.len());
    assert!(
        !scores.value.is_empty(),
        "Should have at least one secure score (ascScore)"
    );
    for score in &scores.value {
        if let Some(props) = &score.properties {
            println!(
                "  Score: name={:?}, displayName={:?}, current={:?}",
                score.name.as_deref(),
                props.display_name.as_deref(),
                props.score.as_ref().and_then(|s| s.current),
            );
        }
    }

    // =========================================================================
    // Step 4: Get the built-in 'ascScore' secure score
    // =========================================================================
    println!("[4/5] Getting 'ascScore' secure score...");
    let asc_score = sec
        .get_secure_score("ascScore")
        .await
        .expect("get_secure_score(ascScore) failed");
    assert_eq!(asc_score.name.as_deref(), Some("ascScore"));
    let props = asc_score
        .properties
        .as_ref()
        .expect("ascScore should have properties");
    assert!(props.score.is_some(), "ascScore should have score details");
    let score_details = props.score.as_ref().unwrap();
    println!(
        "  ascScore: max={:?}, current={:?}, percentage={:?}",
        score_details.max, score_details.current, score_details.percentage,
    );
    assert!(score_details.max.is_some(), "score should have max");
    assert!(
        score_details.current.is_some(),
        "score should have current value"
    );

    // =========================================================================
    // Step 5: List assessments (individual get requires api-version 2021-06-01
    //         which conflicts with secureScores needing 2020-01-01 — skipped)
    // =========================================================================
    println!("[5/5] Listing security assessments...");
    let assessments = sec
        .list_assessments()
        .await
        .expect("list_assessments failed");
    println!("  Found {} assessment(s).", assessments.value.len());
    assert!(
        !assessments.value.is_empty(),
        "Should have security assessments in subscription"
    );

    // Validate structure of the first few assessments
    for a in assessments.value.iter().take(3) {
        let name = a.name.as_deref().unwrap_or("(no name)");
        let status_code = a
            .properties
            .as_ref()
            .and_then(|p| p.status.as_ref())
            .and_then(|s| s.code.as_deref())
            .unwrap_or("(no status)");
        println!("  Assessment: name={name}, status={status_code}");
    }
    // NOTE: get_assessment(name) uses api-version 2020-01-01 (shared manifest api_version)
    // which returns 405. Individual GET requires api-version 2021-06-01. Covered by unit tests.

    println!("\nAll Defender for Cloud read operations passed!");
}
