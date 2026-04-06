//! Operation contracts for the Azure DNS API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/dns.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::dns::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure DNS API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::dns::DnsClient`] instead.
pub struct DnsOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> DnsOps<'a> {
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

    /// Lists the DNS zones within a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`ZoneListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_dns_zones(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<ZoneListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_dns_zones response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_dns_zones response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists the DNS zones in all resource groups in a subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Network/dnsZones`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`ZoneListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_dns_zones_all(&self, subscription_id: &str) -> Result<ZoneListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Network/dnsZones",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_dns_zones_all response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_dns_zones_all response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a DNS zone.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones/{zoneName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `zoneName` —  *(required)*
    ///
    /// # Response
    /// [`Zone`]
    #[allow(dead_code)]
    pub(crate) async fn get_dns_zone(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> Result<Zone> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(zone_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_dns_zone response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_dns_zone response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a DNS zone.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones/{zoneName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `zoneName` —  *(required)*
    ///
    /// # Request Body
    /// [`ZoneCreateRequest`]
    ///
    /// # Response
    /// [`Zone`]
    #[allow(dead_code)]
    pub(crate) async fn create_dns_zone(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        body: &ZoneCreateRequest,
    ) -> Result<Zone> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(zone_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_dns_zone request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_dns_zone response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_dns_zone response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a DNS zone. WARNING: All DNS records in the zone will also be deleted.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones/{zoneName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `zoneName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_dns_zone(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(zone_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Lists all record sets in a DNS zone.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones/{zoneName}/recordsets`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `zoneName` —  *(required)*
    ///
    /// # Response
    /// [`RecordSetListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_record_sets(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> Result<RecordSetListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}/recordsets",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(zone_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_record_sets response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_record_sets response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets a record set.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones/{zoneName}/{recordType}/{relativeRecordSetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `zoneName` —  *(required)*
    /// - `recordType` —  *(required)*
    /// - `relativeRecordSetName` —  *(required)*
    ///
    /// # Response
    /// [`RecordSet`]
    #[allow(dead_code)]
    pub(crate) async fn get_record_set(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> Result<RecordSet> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}/{}/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(zone_name),
            encode(record_type),
            encode(relative_record_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_record_set response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_record_set response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a record set within a DNS zone.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones/{zoneName}/{recordType}/{relativeRecordSetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `zoneName` —  *(required)*
    /// - `recordType` —  *(required)*
    /// - `relativeRecordSetName` —  *(required)*
    ///
    /// # Request Body
    /// [`RecordSetCreateRequest`]
    ///
    /// # Response
    /// [`RecordSet`]
    #[allow(dead_code)]
    pub(crate) async fn create_record_set(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
        body: &RecordSetCreateRequest,
    ) -> Result<RecordSet> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}/{}/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(zone_name),
            encode(record_type),
            encode(relative_record_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_record_set request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_record_set response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_record_set response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a record set from a DNS zone.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/dnsZones/{zoneName}/{recordType}/{relativeRecordSetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `zoneName` —  *(required)*
    /// - `recordType` —  *(required)*
    /// - `relativeRecordSetName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_record_set(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}/{}/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(zone_name),
            encode(record_type),
            encode(relative_record_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2018-05-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_dns_zones() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones")
            .returning_json(serde_json::to_value(ZoneListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .list_dns_zones("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_dns_zones_all() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Network/dnsZones")
            .returning_json(serde_json::to_value(ZoneListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops.list_dns_zones_all("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_dns_zone() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones/test-zoneName")
            .returning_json(serde_json::to_value(Zone::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .get_dns_zone(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-zoneName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_dns_zone() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones/test-zoneName")
            .returning_json(serde_json::to_value(Zone::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let body = ZoneCreateRequest::fixture();
        let result = ops
            .create_dns_zone(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-zoneName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_dns_zone() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones/test-zoneName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .delete_dns_zone(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-zoneName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_record_sets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones/test-zoneName/recordsets")
            .returning_json(serde_json::to_value(RecordSetListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .list_record_sets(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-zoneName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_record_set() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones/test-zoneName/test-recordType/test-relativeRecordSetName")
            .returning_json(serde_json::to_value(RecordSet::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .get_record_set(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-zoneName",
                "test-recordType",
                "test-relativeRecordSetName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_record_set() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones/test-zoneName/test-recordType/test-relativeRecordSetName")
            .returning_json(serde_json::to_value(RecordSet::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let body = RecordSetCreateRequest::fixture();
        let result = ops
            .create_record_set(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-zoneName",
                "test-recordType",
                "test-relativeRecordSetName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_record_set() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/dnsZones/test-zoneName/test-recordType/test-relativeRecordSetName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = DnsOps::new(&client);

        let result = ops
            .delete_record_set(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-zoneName",
                "test-recordType",
                "test-relativeRecordSetName",
            )
            .await;
        assert!(result.is_ok());
    }
}
