//! MockClient helpers for Azure DNS API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure DNS helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait DnsMockHelpers {
    /// Helper to expect `list_dns_zones`: Lists the DNS zones within a resource group.
    fn expect_list_dns_zones(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_dns_zones_all`: Lists the DNS zones in all resource groups in a
    /// subscription.
    fn expect_list_dns_zones_all(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_dns_zone`: Gets a DNS zone.
    fn expect_get_dns_zone(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_dns_zone`: Creates or updates a DNS zone.
    fn expect_create_dns_zone(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_dns_zone`: Deletes a DNS zone. WARNING: All DNS records in the zone
    /// will also be deleted.
    fn expect_delete_dns_zone(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_record_sets`: Lists all record sets in a DNS zone.
    fn expect_list_record_sets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_record_set`: Gets a record set.
    fn expect_get_record_set(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_record_set`: Creates or updates a record set within a DNS zone.
    fn expect_create_record_set(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_record_set`: Deletes a record set from a DNS zone.
    fn expect_delete_record_set(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl DnsMockHelpers for MockClient {
    /// Helper to expect `list_dns_zones`: Lists the DNS zones within a resource group.
    fn expect_list_dns_zones(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_dns_zones_all`: Lists the DNS zones in all resource groups in a
    /// subscription.
    fn expect_list_dns_zones_all(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/subscriptions/{subscription_id}/providers/Microsoft.Network/dnsZones");
        self.expect_get(&path)
    }

    /// Helper to expect `get_dns_zone`: Gets a DNS zone.
    fn expect_get_dns_zone(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones/{zone_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_dns_zone`: Creates or updates a DNS zone.
    fn expect_create_dns_zone(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones/{zone_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_dns_zone`: Deletes a DNS zone. WARNING: All DNS records in the zone
    /// will also be deleted.
    fn expect_delete_dns_zone(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones/{zone_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_record_sets`: Lists all record sets in a DNS zone.
    fn expect_list_record_sets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones/{zone_name}/recordsets"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_record_set`: Gets a record set.
    fn expect_get_record_set(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones/{zone_name}/{record_type}/{relative_record_set_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_record_set`: Creates or updates a record set within a DNS zone.
    fn expect_create_record_set(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones/{zone_name}/{record_type}/{relative_record_set_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_record_set`: Deletes a record set from a DNS zone.
    fn expect_delete_record_set(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        zone_name: &str,
        record_type: &str,
        relative_record_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/dnsZones/{zone_name}/{record_type}/{relative_record_set_name}"
        );
        self.expect_delete(&path)
    }
}
