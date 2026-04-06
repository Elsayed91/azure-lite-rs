//! Azure Resource Graph API client.
//!
//! Provides KQL query execution against the Azure Resource Graph, which indexes
//! all ARM resources across subscriptions. Supports single-page and
//! auto-paginating queries.
//!
//! Auth: standard Azure Bearer token (handled by `AzureHttpClient`).
//! Rate limit: 15 requests/5 s per tenant — the existing `RateLimiter` applies.

use crate::{
    AzureHttpClient, Result,
    ops::resource_graph::ResourceGraphOps,
    types::resource_graph::{QueryOptions, ResourceGraphRequest, ResourceGraphResponse},
};

/// Client for the Azure Resource Graph API.
///
/// Wraps [`ResourceGraphOps`] with ergonomic, pagination-aware methods.
pub struct ResourceGraphClient<'a> {
    ops: ResourceGraphOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> ResourceGraphClient<'a> {
    /// Create a new Azure Resource Graph API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: ResourceGraphOps::new(client),
            client,
        }
    }

    /// Execute a KQL query across subscriptions, auto-injecting the client's
    /// subscription ID. Handles pagination internally — returns **all** results.
    ///
    /// # Example
    /// ```no_run
    /// # async fn run(client: &azure_lite::AzureHttpClient) -> azure_lite::Result<()> {
    /// let results = client.resource_graph()
    ///     .query("Resources | where type =~ 'microsoft.compute/disks' | project id, name", &[])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Pass an empty slice for `extra_subscriptions` to query only the client's
    /// own subscription. Pass additional IDs to query across multiple subscriptions.
    pub async fn query(
        &self,
        query: &str,
        extra_subscriptions: &[&str],
    ) -> Result<Vec<serde_json::Value>> {
        let mut subscriptions = vec![self.client.subscription_id().to_string()];
        subscriptions.extend(extra_subscriptions.iter().map(|s| s.to_string()));

        let mut all_results: Vec<serde_json::Value> = Vec::new();
        let mut skip_token: Option<String> = None;

        loop {
            let options = QueryOptions {
                top: Some(1000),
                result_format: Some("objectArray".to_string()),
                skip_token: skip_token.clone(),
            };
            let req = ResourceGraphRequest {
                subscriptions: subscriptions.clone(),
                query: query.to_string(),
                options: Some(options),
            };
            let page = self.ops.query_resources(&req).await?;
            all_results.extend(page.data);

            let truncated = page
                .result_truncated
                .as_deref()
                .map(|s| s.eq_ignore_ascii_case("true"))
                .unwrap_or(false);
            skip_token = page.skip_token;

            if !truncated || skip_token.is_none() {
                break;
            }
        }

        Ok(all_results)
    }

    /// Execute a KQL query and return a **single page** of results.
    ///
    /// Use `skip_token` from a previous [`ResourceGraphResponse`] to fetch
    /// the next page. Pass `None` to start from the beginning.
    ///
    /// The client's subscription ID is always included; `extra_subscriptions`
    /// adds more.
    pub async fn query_page(
        &self,
        query: &str,
        extra_subscriptions: &[&str],
        options: Option<QueryOptions>,
        skip_token: Option<&str>,
    ) -> Result<ResourceGraphResponse> {
        let mut subscriptions = vec![self.client.subscription_id().to_string()];
        subscriptions.extend(extra_subscriptions.iter().map(|s| s.to_string()));

        let opts = match (options, skip_token) {
            (Some(mut o), Some(tok)) => {
                o.skip_token = Some(tok.to_string());
                Some(o)
            }
            (Some(o), None) => Some(o),
            (None, Some(tok)) => Some(QueryOptions {
                skip_token: Some(tok.to_string()),
                ..Default::default()
            }),
            (None, None) => None,
        };

        let req = ResourceGraphRequest {
            subscriptions,
            query: query.to_string(),
            options: opts,
        };
        self.ops.query_resources(&req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockClient;

    fn make_client(mock: MockClient) -> AzureHttpClient {
        AzureHttpClient::from_mock(mock)
    }

    fn disk_json(name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": format!("/subscriptions/test-subscription-id/resourceGroups/test-rg/providers/Microsoft.Compute/disks/{name}"),
            "name": name,
            "type": "microsoft.compute/disks",
            "location": "eastus",
            "resourceGroup": "test-rg",
            "subscriptionId": "test-subscription-id"
        })
    }

    #[tokio::test]
    async fn query_returns_all_results_single_page() {
        let mut mock = MockClient::new();
        mock.expect_post("/providers/Microsoft.ResourceGraph/resources")
            .returning_json(serde_json::json!({
                "totalRecords": 2,
                "count": 2,
                "resultTruncated": "false",
                "data": [disk_json("disk-1"), disk_json("disk-2")]
            }));
        let client = make_client(mock);
        let results = client
            .resource_graph()
            .query("Resources | where type =~ 'microsoft.compute/disks'", &[])
            .await
            .expect("query failed");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], "disk-1");
        assert_eq!(results[1]["name"], "disk-2");
    }

    #[tokio::test]
    async fn query_paginates_across_multiple_pages() {
        let mut mock = MockClient::new();
        // Two sequential responses for the same endpoint: page 1 then page 2
        mock.expect_post("/providers/Microsoft.ResourceGraph/resources")
            .returning_json_sequence(vec![
                serde_json::json!({
                    "totalRecords": 2,
                    "count": 1,
                    "resultTruncated": "true",
                    "$skipToken": "page2-token",
                    "data": [disk_json("disk-1")]
                }),
                serde_json::json!({
                    "totalRecords": 2,
                    "count": 1,
                    "resultTruncated": "false",
                    "data": [disk_json("disk-2")]
                }),
            ])
            .times(2);
        let client = make_client(mock);
        let results = client
            .resource_graph()
            .query("Resources | where type =~ 'microsoft.compute/disks'", &[])
            .await
            .expect("query failed");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], "disk-1");
        assert_eq!(results[1]["name"], "disk-2");
    }

    #[tokio::test]
    async fn query_page_returns_single_page_with_skip_token() {
        let mut mock = MockClient::new();
        mock.expect_post("/providers/Microsoft.ResourceGraph/resources")
            .returning_json(serde_json::json!({
                "totalRecords": 1000,
                "count": 1,
                "resultTruncated": "true",
                "$skipToken": "next-page-token",
                "data": [disk_json("disk-1")]
            }));
        let client = make_client(mock);
        let page = client
            .resource_graph()
            .query_page(
                "Resources | where type =~ 'microsoft.compute/disks'",
                &[],
                None,
                None,
            )
            .await
            .expect("query_page failed");
        assert_eq!(page.count, Some(1));
        assert_eq!(page.total_records, Some(1000));
        assert_eq!(page.result_truncated.as_deref(), Some("true"));
        assert_eq!(page.skip_token.as_deref(), Some("next-page-token"));
        assert_eq!(page.data.len(), 1);
        assert_eq!(page.data[0]["name"], "disk-1");
    }

    #[tokio::test]
    async fn query_page_merges_skip_token_into_options() {
        let mut mock = MockClient::new();
        mock.expect_post("/providers/Microsoft.ResourceGraph/resources")
            .returning_json(serde_json::json!({
                "totalRecords": 1000,
                "count": 1,
                "resultTruncated": "false",
                "data": [disk_json("disk-2")]
            }));
        let client = make_client(mock);
        let page = client
            .resource_graph()
            .query_page(
                "Resources | where type =~ 'microsoft.compute/disks'",
                &[],
                None,
                Some("page2-token"),
            )
            .await
            .expect("query_page failed");
        assert_eq!(page.data.len(), 1);
        assert_eq!(page.data[0]["name"], "disk-2");
    }

    #[tokio::test]
    async fn query_returns_empty_for_no_results() {
        let mut mock = MockClient::new();
        mock.expect_post("/providers/Microsoft.ResourceGraph/resources")
            .returning_json(serde_json::json!({
                "totalRecords": 0,
                "count": 0,
                "resultTruncated": "false",
                "data": []
            }));
        let client = make_client(mock);
        let results = client
            .resource_graph()
            .query("Resources | where 1 == 0", &[])
            .await
            .expect("query failed");
        assert_eq!(results.len(), 0);
    }
}
