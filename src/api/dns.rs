//! Azure DNS API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::dns::DnsOps`. This layer adds:
//! - Ergonomic method signatures (auto-injects subscription_id from client)

use crate::{
    AzureHttpClient, Result,
    ops::dns::DnsOps,
    types::dns::{
        RecordSet, RecordSetCreateRequest, RecordSetListResult, Zone, ZoneCreateRequest,
        ZoneListResult,
    },
};

/// Client for the Azure DNS API.
///
/// Wraps the raw [`DnsOps`] with ergonomic signatures that
/// auto-inject `subscription_id` from the parent [`AzureHttpClient`].
pub struct DnsClient<'a> {
    ops: DnsOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> DnsClient<'a> {
    /// Create a new Azure DNS API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: DnsOps::new(client),
            client,
        }
    }

    // --- DNS Zone operations ---

    /// Lists the DNS zones within a resource group.
    pub async fn list_dns_zones(&self, resource_group_name: &str) -> Result<ZoneListResult> {
        self.ops
            .list_dns_zones(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Lists the DNS zones in all resource groups in the subscription.
    pub async fn list_dns_zones_all(&self) -> Result<ZoneListResult> {
        self.ops
            .list_dns_zones_all(self.client.subscription_id())
            .await
    }

    /// Gets a DNS zone.
    pub async fn get_dns_zone(&self, resource_group_name: &str, zone_name: &str) -> Result<Zone> {
        self.ops
            .get_dns_zone(
                self.client.subscription_id(),
                resource_group_name,
                zone_name,
            )
            .await
    }

    /// Creates or updates a DNS zone.
    pub async fn create_dns_zone(
        &self,
        resource_group_name: &str,
        zone_name: &str,
        body: &ZoneCreateRequest,
    ) -> Result<Zone> {
        self.ops
            .create_dns_zone(
                self.client.subscription_id(),
                resource_group_name,
                zone_name,
                body,
            )
            .await
    }

    /// Deletes a DNS zone. WARNING: All DNS records in the zone will also be deleted.
    pub async fn delete_dns_zone(&self, resource_group_name: &str, zone_name: &str) -> Result<()> {
        self.ops
            .delete_dns_zone(
                self.client.subscription_id(),
                resource_group_name,
                zone_name,
            )
            .await
    }

    // --- DNS Record Set operations ---

    /// Lists all record sets in a DNS zone.
    pub async fn list_record_sets(
        &self,
        resource_group_name: &str,
        zone_name: &str,
    ) -> Result<RecordSetListResult> {
        self.ops
            .list_record_sets(
                self.client.subscription_id(),
                resource_group_name,
                zone_name,
            )
            .await
    }

    /// Gets a record set.
    pub async fn get_record_set(
        &self,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> Result<RecordSet> {
        self.ops
            .get_record_set(
                self.client.subscription_id(),
                resource_group_name,
                zone_name,
                record_type,
                relative_record_set_name,
            )
            .await
    }

    /// Creates or updates a record set within a DNS zone.
    pub async fn create_record_set(
        &self,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
        body: &RecordSetCreateRequest,
    ) -> Result<RecordSet> {
        self.ops
            .create_record_set(
                self.client.subscription_id(),
                resource_group_name,
                zone_name,
                record_type,
                relative_record_set_name,
                body,
            )
            .await
    }

    /// Deletes a record set from a DNS zone.
    pub async fn delete_record_set(
        &self,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_record_set(
                self.client.subscription_id(),
                resource_group_name,
                zone_name,
                record_type,
                relative_record_set_name,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- DNS Zone tests ----

    #[tokio::test]
    async fn list_dns_zones_returns_empty_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .dns()
            .list_dns_zones("rg1")
            .await
            .expect("list_dns_zones failed");
        assert!(result.value.is_empty());
        assert!(result.next_link.is_none());
    }

    #[tokio::test]
    async fn list_dns_zones_returns_zone_with_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com",
                    "name": "example.com",
                    "type": "Microsoft.Network/dnszones",
                    "location": "global",
                    "etag": "abc123",
                    "properties": {
                        "zoneType": "Public",
                        "numberOfRecordSets": 2,
                        "nameServers": ["ns1-08.azure-dns.com.", "ns2-08.azure-dns.net."]
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .dns()
            .list_dns_zones("rg1")
            .await
            .expect("list_dns_zones failed");
        assert_eq!(result.value.len(), 1);
        let zone = &result.value[0];
        assert_eq!(zone.name.as_deref(), Some("example.com"));
        assert_eq!(zone.etag.as_deref(), Some("abc123"));
        assert_eq!(
            zone.properties
                .as_ref()
                .and_then(|p| p.zone_type.as_deref()),
            Some("Public"),
        );
        assert_eq!(
            zone.properties.as_ref().map(|p| p.name_servers.len()),
            Some(2),
        );
    }

    #[tokio::test]
    async fn list_dns_zones_all_uses_subscription_scope() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/providers/Microsoft.Network/dnsZones")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .dns()
            .list_dns_zones_all()
            .await
            .expect("list_dns_zones_all failed");
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn get_dns_zone_returns_zone() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com",
                "name": "example.com",
                "location": "global",
                "etag": "xyz789",
                "properties": {
                    "zoneType": "Public",
                    "nameServers": ["ns1-08.azure-dns.com."]
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let zone = client
            .dns()
            .get_dns_zone("rg1", "example.com")
            .await
            .expect("get_dns_zone failed");
        assert_eq!(zone.name.as_deref(), Some("example.com"));
        assert!(zone.id.is_some());
        assert_eq!(zone.etag.as_deref(), Some("xyz789"));
    }

    #[tokio::test]
    async fn create_dns_zone_sends_put_and_returns_zone() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/new.example.com")
            .returning_json(serde_json::json!({
                "name": "new.example.com",
                "location": "global",
                "etag": "newetag",
                "properties": {
                    "zoneType": "Public",
                    "nameServers": ["ns1-08.azure-dns.com."]
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let request = ZoneCreateRequest {
            location: "global".into(),
            ..Default::default()
        };
        let zone = client
            .dns()
            .create_dns_zone("rg1", "new.example.com", &request)
            .await
            .expect("create_dns_zone failed");
        assert_eq!(zone.name.as_deref(), Some("new.example.com"));
        assert_eq!(
            zone.properties
                .as_ref()
                .and_then(|p| p.zone_type.as_deref()),
            Some("Public"),
        );
    }

    #[tokio::test]
    async fn delete_dns_zone_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/old.example.com")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .dns()
            .delete_dns_zone("rg1", "old.example.com")
            .await
            .expect("delete_dns_zone failed");
    }

    // ---- Record Set tests ----

    #[tokio::test]
    async fn list_record_sets_returns_ns_and_soa_by_default() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com/recordsets")
            .returning_json(serde_json::json!({
                "value": [
                    {
                        "name": "@",
                        "type": "Microsoft.Network/dnszones/NS",
                        "etag": "ns-etag",
                        "properties": {
                            "TTL": 172800,
                            "NSRecords": [{"nsdname": "ns1-08.azure-dns.com."}],
                            "fqdn": "example.com.",
                            "provisioningState": "Succeeded"
                        }
                    },
                    {
                        "name": "@",
                        "type": "Microsoft.Network/dnszones/SOA",
                        "etag": "soa-etag",
                        "properties": {
                            "TTL": 3600,
                            "fqdn": "example.com.",
                            "provisioningState": "Succeeded"
                        }
                    }
                ]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .dns()
            .list_record_sets("rg1", "example.com")
            .await
            .expect("list_record_sets failed");
        assert_eq!(result.value.len(), 2);
        assert_eq!(
            result.value[0].r#type.as_deref(),
            Some("Microsoft.Network/dnszones/NS")
        );
        assert_eq!(
            result.value[0].properties.as_ref().and_then(|p| p.ttl),
            Some(172800),
        );
        let ns_records = &result.value[0].properties.as_ref().unwrap().ns_records;
        assert_eq!(ns_records.len(), 1);
        assert_eq!(
            ns_records[0].nsdname.as_deref(),
            Some("ns1-08.azure-dns.com.")
        );
    }

    #[tokio::test]
    async fn get_record_set_returns_a_record_with_ip() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com/A/www")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com/A/www",
                "name": "www",
                "type": "Microsoft.Network/dnszones/A",
                "etag": "a-etag",
                "properties": {
                    "TTL": 300,
                    "ARecords": [{"ipv4Address": "1.2.3.4"}],
                    "fqdn": "www.example.com.",
                    "provisioningState": "Succeeded"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let rs = client
            .dns()
            .get_record_set("rg1", "example.com", "A", "www")
            .await
            .expect("get_record_set failed");
        assert_eq!(rs.name.as_deref(), Some("www"));
        assert!(rs.id.is_some());
        assert_eq!(rs.properties.as_ref().and_then(|p| p.ttl), Some(300));
        let a_records = &rs.properties.as_ref().unwrap().a_records;
        assert_eq!(a_records.len(), 1);
        assert_eq!(a_records[0].ipv4_address.as_deref(), Some("1.2.3.4"));
        assert_eq!(
            rs.properties.as_ref().and_then(|p| p.fqdn.as_deref()),
            Some("www.example.com."),
        );
    }

    #[tokio::test]
    async fn create_record_set_sends_put_and_returns_record_set() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com/A/new-record")
            .returning_json(serde_json::json!({
                "name": "new-record",
                "type": "Microsoft.Network/dnszones/A",
                "etag": "new-etag",
                "properties": {
                    "TTL": 300,
                    "ARecords": [{"ipv4Address": "5.6.7.8"}],
                    "fqdn": "new-record.example.com.",
                    "provisioningState": "Succeeded"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let request = RecordSetCreateRequest {
            properties: Some(crate::types::dns::RecordSetProperties {
                ttl: Some(300),
                a_records: vec![crate::types::dns::ARecord {
                    ipv4_address: Some("5.6.7.8".into()),
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let rs = client
            .dns()
            .create_record_set("rg1", "example.com", "A", "new-record", &request)
            .await
            .expect("create_record_set failed");
        assert_eq!(rs.name.as_deref(), Some("new-record"));
        assert_eq!(rs.properties.as_ref().and_then(|p| p.ttl), Some(300));
        assert_eq!(
            rs.properties
                .as_ref()
                .and_then(|p| p.a_records.first())
                .and_then(|a| a.ipv4_address.as_deref()),
            Some("5.6.7.8"),
        );
    }

    #[tokio::test]
    async fn delete_record_set_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/dnsZones/example.com/A/old-record")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .dns()
            .delete_record_set("rg1", "example.com", "A", "old-record")
            .await
            .expect("delete_record_set failed");
    }
}
