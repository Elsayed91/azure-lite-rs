//! Azure Networking API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::networking::NetworkingOps`. This layer adds:
//! - Ergonomic method signatures (auto-injects subscription_id from client)

use crate::{
    AzureHttpClient, Result,
    ops::networking::NetworkingOps,
    types::networking::{
        LoadBalancer, LoadBalancerCreateRequest, LoadBalancerListResult, NetworkSecurityGroup,
        NetworkSecurityGroupCreateRequest, NetworkSecurityGroupListResult, SecurityRule,
        SecurityRuleListResult, Subnet, SubnetListResult, VirtualNetwork,
        VirtualNetworkCreateRequest, VirtualNetworkListResult,
    },
};

/// Client for the Azure Networking API.
///
/// Wraps the raw [`NetworkingOps`] with ergonomic signatures that
/// auto-inject `subscription_id` from the parent [`AzureHttpClient`].
pub struct NetworkingClient<'a> {
    ops: NetworkingOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> NetworkingClient<'a> {
    /// Create a new Azure Networking API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: NetworkingOps::new(client),
            client,
        }
    }

    // --- VNet operations ---

    /// Gets all virtual networks in a resource group.
    pub async fn list_vnets(&self, resource_group_name: &str) -> Result<VirtualNetworkListResult> {
        self.ops
            .list_vnets(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Gets all virtual networks in the subscription.
    pub async fn list_vnets_all(&self) -> Result<VirtualNetworkListResult> {
        self.ops.list_vnets_all(self.client.subscription_id()).await
    }

    /// Gets the specified virtual network.
    pub async fn get_vnet(
        &self,
        resource_group_name: &str,
        vnet_name: &str,
    ) -> Result<VirtualNetwork> {
        self.ops
            .get_vnet(
                self.client.subscription_id(),
                resource_group_name,
                vnet_name,
                "",
            )
            .await
    }

    /// Creates or updates a virtual network.
    pub async fn create_vnet(
        &self,
        resource_group_name: &str,
        vnet_name: &str,
        body: &VirtualNetworkCreateRequest,
    ) -> Result<VirtualNetwork> {
        self.ops
            .create_vnet(
                self.client.subscription_id(),
                resource_group_name,
                vnet_name,
                body,
            )
            .await
    }

    /// Deletes the specified virtual network.
    pub async fn delete_vnet(&self, resource_group_name: &str, vnet_name: &str) -> Result<()> {
        self.ops
            .delete_vnet(
                self.client.subscription_id(),
                resource_group_name,
                vnet_name,
            )
            .await
    }

    /// Gets all subnets in a virtual network.
    pub async fn list_subnets(
        &self,
        resource_group_name: &str,
        vnet_name: &str,
    ) -> Result<SubnetListResult> {
        self.ops
            .list_subnets(
                self.client.subscription_id(),
                resource_group_name,
                vnet_name,
            )
            .await
    }

    /// Gets the specified subnet.
    pub async fn get_subnet(
        &self,
        resource_group_name: &str,
        vnet_name: &str,
        subnet_name: &str,
    ) -> Result<Subnet> {
        self.ops
            .get_subnet(
                self.client.subscription_id(),
                resource_group_name,
                vnet_name,
                subnet_name,
                "",
            )
            .await
    }

    // --- NSG operations ---

    /// Gets all network security groups in a resource group.
    pub async fn list_nsgs(
        &self,
        resource_group_name: &str,
    ) -> Result<NetworkSecurityGroupListResult> {
        self.ops
            .list_nsgs(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Gets all network security groups in the subscription.
    pub async fn list_nsgs_all(&self) -> Result<NetworkSecurityGroupListResult> {
        self.ops.list_nsgs_all(self.client.subscription_id()).await
    }

    /// Gets the specified network security group.
    pub async fn get_nsg(
        &self,
        resource_group_name: &str,
        nsg_name: &str,
    ) -> Result<NetworkSecurityGroup> {
        self.ops
            .get_nsg(
                self.client.subscription_id(),
                resource_group_name,
                nsg_name,
                "",
            )
            .await
    }

    /// Creates or updates a network security group.
    pub async fn create_nsg(
        &self,
        resource_group_name: &str,
        nsg_name: &str,
        body: &NetworkSecurityGroupCreateRequest,
    ) -> Result<NetworkSecurityGroup> {
        self.ops
            .create_nsg(
                self.client.subscription_id(),
                resource_group_name,
                nsg_name,
                body,
            )
            .await
    }

    /// Deletes the specified network security group.
    pub async fn delete_nsg(&self, resource_group_name: &str, nsg_name: &str) -> Result<()> {
        self.ops
            .delete_nsg(self.client.subscription_id(), resource_group_name, nsg_name)
            .await
    }

    /// Gets all security rules in a network security group.
    pub async fn list_security_rules(
        &self,
        resource_group_name: &str,
        nsg_name: &str,
    ) -> Result<SecurityRuleListResult> {
        self.ops
            .list_security_rules(self.client.subscription_id(), resource_group_name, nsg_name)
            .await
    }

    /// Gets the specified network security rule.
    pub async fn get_security_rule(
        &self,
        resource_group_name: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> Result<SecurityRule> {
        self.ops
            .get_security_rule(
                self.client.subscription_id(),
                resource_group_name,
                nsg_name,
                rule_name,
            )
            .await
    }

    /// Creates or updates a security rule in a network security group.
    pub async fn create_security_rule(
        &self,
        resource_group_name: &str,
        nsg_name: &str,
        rule_name: &str,
        body: &SecurityRule,
    ) -> Result<SecurityRule> {
        self.ops
            .create_security_rule(
                self.client.subscription_id(),
                resource_group_name,
                nsg_name,
                rule_name,
                body,
            )
            .await
    }

    /// Deletes the specified network security rule.
    pub async fn delete_security_rule(
        &self,
        resource_group_name: &str,
        nsg_name: &str,
        rule_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_security_rule(
                self.client.subscription_id(),
                resource_group_name,
                nsg_name,
                rule_name,
            )
            .await
    }

    // --- Load Balancer operations ---

    /// Gets all the load balancers in a resource group.
    pub async fn list_load_balancers(
        &self,
        resource_group_name: &str,
    ) -> Result<LoadBalancerListResult> {
        self.ops
            .list_load_balancers(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Gets all the load balancers in the subscription.
    pub async fn list_load_balancers_all(&self) -> Result<LoadBalancerListResult> {
        self.ops
            .list_load_balancers_all(self.client.subscription_id())
            .await
    }

    /// Gets the specified load balancer.
    pub async fn get_load_balancer(
        &self,
        resource_group_name: &str,
        lb_name: &str,
    ) -> Result<LoadBalancer> {
        self.ops
            .get_load_balancer(
                self.client.subscription_id(),
                resource_group_name,
                lb_name,
                "",
            )
            .await
    }

    /// Creates or updates a load balancer.
    pub async fn create_load_balancer(
        &self,
        resource_group_name: &str,
        lb_name: &str,
        body: &LoadBalancerCreateRequest,
    ) -> Result<LoadBalancer> {
        self.ops
            .create_load_balancer(
                self.client.subscription_id(),
                resource_group_name,
                lb_name,
                body,
            )
            .await
    }

    /// Deletes the specified load balancer.
    pub async fn delete_load_balancer(
        &self,
        resource_group_name: &str,
        lb_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_load_balancer(self.client.subscription_id(), resource_group_name, lb_name)
            .await
    }

    // --- Network Interface operations ---

    /// Deletes the specified network interface.
    pub async fn delete_network_interface(
        &self,
        resource_group_name: &str,
        nic_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_network_interface(self.client.subscription_id(), resource_group_name, nic_name)
            .await
    }

    // --- NAT Gateway operations ---

    /// Deletes the specified NAT gateway.
    pub async fn delete_nat_gateway(
        &self,
        resource_group_name: &str,
        nat_gateway_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_nat_gateway(
                self.client.subscription_id(),
                resource_group_name,
                nat_gateway_name,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- VNet tests ----

    #[tokio::test]
    async fn list_vnets_returns_empty_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Network/virtualNetworks")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_vnets("my-rg")
            .await
            .expect("list_vnets failed");
        assert!(result.value.is_empty());
        assert!(result.next_link.is_none());
    }

    #[tokio::test]
    async fn list_vnets_returns_vnet_with_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Network/virtualNetworks")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/my-rg/providers/Microsoft.Network/virtualNetworks/vnet1",
                    "name": "vnet1",
                    "type": "Microsoft.Network/virtualNetworks",
                    "location": "eastus",
                    "properties": {
                        "provisioningState": "Succeeded",
                        "addressSpace": { "addressPrefixes": ["10.0.0.0/16"] }
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_vnets("my-rg")
            .await
            .expect("list_vnets failed");
        assert_eq!(result.value.len(), 1);
        let vnet = &result.value[0];
        assert_eq!(vnet.name.as_deref(), Some("vnet1"));
        assert_eq!(vnet.location.as_deref(), Some("eastus"));
        assert_eq!(
            vnet.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
        assert_eq!(
            vnet.properties
                .as_ref()
                .and_then(|p| p.address_space.as_ref())
                .map(|a| a.address_prefixes.as_slice()),
            Some(["10.0.0.0/16".to_string()].as_slice()),
        );
    }

    #[tokio::test]
    async fn list_vnets_all_uses_subscription_scope() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/subscriptions/test-subscription-id/providers/Microsoft.Network/virtualNetworks",
        )
        .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_vnets_all()
            .await
            .expect("list_vnets_all failed");
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn get_vnet_injects_subscription_id() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/my-vnet")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/my-vnet",
                "name": "my-vnet",
                "type": "Microsoft.Network/virtualNetworks",
                "location": "westus2",
                "properties": {
                    "provisioningState": "Succeeded",
                    "addressSpace": { "addressPrefixes": ["10.1.0.0/16"] }
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let vnet = client
            .networking()
            .get_vnet("rg1", "my-vnet")
            .await
            .expect("get_vnet failed");
        assert_eq!(vnet.name.as_deref(), Some("my-vnet"));
        assert_eq!(vnet.location.as_deref(), Some("westus2"));
        assert!(vnet.id.is_some());
    }

    #[tokio::test]
    async fn create_vnet_sends_put_and_returns_vnet() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/new-vnet")
            .returning_json(serde_json::json!({
                "name": "new-vnet",
                "location": "eastus",
                "properties": {
                    "provisioningState": "Updating",
                    "addressSpace": { "addressPrefixes": ["10.2.0.0/16"] }
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let request = VirtualNetworkCreateRequest {
            location: "eastus".into(),
            properties: Some(crate::types::networking::VirtualNetworkPropertiesFormat {
                address_space: Some(crate::types::networking::AddressSpace {
                    address_prefixes: vec!["10.2.0.0/16".into()],
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let vnet = client
            .networking()
            .create_vnet("rg1", "new-vnet", &request)
            .await
            .expect("create_vnet failed");
        assert_eq!(vnet.name.as_deref(), Some("new-vnet"));
        assert_eq!(
            vnet.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Updating"),
        );
    }

    #[tokio::test]
    async fn delete_vnet_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/old-vnet")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .networking()
            .delete_vnet("rg1", "old-vnet")
            .await
            .expect("delete_vnet failed");
    }

    #[tokio::test]
    async fn list_subnets_returns_subnet_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/my-vnet/subnets")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/my-vnet/subnets/default",
                    "name": "default",
                    "properties": { "addressPrefix": "10.0.0.0/24", "provisioningState": "Succeeded" }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_subnets("rg1", "my-vnet")
            .await
            .expect("list_subnets failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some("default"));
        assert_eq!(
            result.value[0]
                .properties
                .as_ref()
                .and_then(|p| p.address_prefix.as_deref()),
            Some("10.0.0.0/24"),
        );
    }

    #[tokio::test]
    async fn get_subnet_returns_address_prefix() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/my-vnet/subnets/default")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/my-vnet/subnets/default",
                "name": "default",
                "properties": { "addressPrefix": "10.0.0.0/24", "provisioningState": "Succeeded" }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let subnet = client
            .networking()
            .get_subnet("rg1", "my-vnet", "default")
            .await
            .expect("get_subnet failed");
        assert_eq!(subnet.name.as_deref(), Some("default"));
        assert_eq!(
            subnet
                .properties
                .as_ref()
                .and_then(|p| p.address_prefix.as_deref()),
            Some("10.0.0.0/24"),
        );
    }

    // ---- NSG tests ----

    #[tokio::test]
    async fn list_nsgs_returns_nsg_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg",
                    "name": "my-nsg",
                    "type": "Microsoft.Network/networkSecurityGroups",
                    "location": "eastus",
                    "properties": {
                        "provisioningState": "Succeeded",
                        "securityRules": [],
                        "defaultSecurityRules": []
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_nsgs("rg1")
            .await
            .expect("list_nsgs failed");
        assert_eq!(result.value.len(), 1);
        let nsg = &result.value[0];
        assert_eq!(nsg.name.as_deref(), Some("my-nsg"));
        assert_eq!(nsg.location.as_deref(), Some("eastus"));
        assert_eq!(
            nsg.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
    }

    #[tokio::test]
    async fn list_nsgs_all_uses_subscription_scope() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/subscriptions/test-subscription-id/providers/Microsoft.Network/networkSecurityGroups",
        )
        .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_nsgs_all()
            .await
            .expect("list_nsgs_all failed");
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn get_nsg_returns_nsg_with_default_rules() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg",
                "name": "my-nsg",
                "location": "eastus",
                "properties": {
                    "provisioningState": "Succeeded",
                    "securityRules": [],
                    "defaultSecurityRules": [
                        {
                            "name": "AllowVnetInBound",
                            "properties": {
                                "priority": 65000,
                                "access": "Allow",
                                "direction": "Inbound",
                                "protocol": "*"
                            }
                        }
                    ]
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let nsg = client
            .networking()
            .get_nsg("rg1", "my-nsg")
            .await
            .expect("get_nsg failed");
        assert_eq!(nsg.name.as_deref(), Some("my-nsg"));
        assert!(nsg.id.is_some());
        let default_rules = &nsg.properties.as_ref().unwrap().default_security_rules;
        assert_eq!(default_rules.len(), 1);
        assert_eq!(default_rules[0].name.as_deref(), Some("AllowVnetInBound"));
    }

    #[tokio::test]
    async fn create_nsg_sends_put_and_returns_nsg() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/new-nsg")
            .returning_json(serde_json::json!({
                "name": "new-nsg",
                "location": "eastus",
                "properties": {
                    "provisioningState": "Updating",
                    "securityRules": [],
                    "defaultSecurityRules": []
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let request = NetworkSecurityGroupCreateRequest {
            location: "eastus".into(),
            ..Default::default()
        };
        let nsg = client
            .networking()
            .create_nsg("rg1", "new-nsg", &request)
            .await
            .expect("create_nsg failed");
        assert_eq!(nsg.name.as_deref(), Some("new-nsg"));
        assert_eq!(
            nsg.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Updating"),
        );
    }

    #[tokio::test]
    async fn delete_nsg_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/old-nsg")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .networking()
            .delete_nsg("rg1", "old-nsg")
            .await
            .expect("delete_nsg failed");
    }

    // ---- Security Rule tests ----

    #[tokio::test]
    async fn list_security_rules_returns_empty_initially() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg/securityRules")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_security_rules("rg1", "my-nsg")
            .await
            .expect("list_security_rules failed");
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn list_security_rules_returns_rule_with_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg/securityRules")
            .returning_json(serde_json::json!({
                "value": [{
                    "name": "allow-http",
                    "properties": {
                        "priority": 100,
                        "protocol": "Tcp",
                        "access": "Allow",
                        "direction": "Inbound",
                        "sourcePortRange": "*",
                        "destinationPortRange": "80",
                        "sourceAddressPrefix": "*",
                        "destinationAddressPrefix": "*",
                        "provisioningState": "Succeeded"
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_security_rules("rg1", "my-nsg")
            .await
            .expect("list_security_rules failed");
        assert_eq!(result.value.len(), 1);
        let rule = &result.value[0];
        assert_eq!(rule.name.as_deref(), Some("allow-http"));
        assert_eq!(rule.properties.as_ref().and_then(|p| p.priority), Some(100),);
        assert_eq!(
            rule.properties.as_ref().and_then(|p| p.access.as_deref()),
            Some("Allow"),
        );
        assert_eq!(
            rule.properties
                .as_ref()
                .and_then(|p| p.destination_port_range.as_deref()),
            Some("80"),
        );
    }

    #[tokio::test]
    async fn get_security_rule_returns_rule() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg/securityRules/my-rule")
            .returning_json(serde_json::json!({
                "name": "my-rule",
                "properties": {
                    "priority": 200,
                    "protocol": "Tcp",
                    "access": "Deny",
                    "direction": "Outbound",
                    "sourcePortRange": "*",
                    "destinationPortRange": "443",
                    "sourceAddressPrefix": "*",
                    "destinationAddressPrefix": "Internet"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let rule = client
            .networking()
            .get_security_rule("rg1", "my-nsg", "my-rule")
            .await
            .expect("get_security_rule failed");
        assert_eq!(rule.name.as_deref(), Some("my-rule"));
        assert_eq!(
            rule.properties
                .as_ref()
                .and_then(|p| p.direction.as_deref()),
            Some("Outbound"),
        );
        assert_eq!(
            rule.properties.as_ref().and_then(|p| p.access.as_deref()),
            Some("Deny"),
        );
        assert_eq!(
            rule.properties
                .as_ref()
                .and_then(|p| p.destination_port_range.as_deref()),
            Some("443"),
        );
    }

    #[tokio::test]
    async fn create_security_rule_sends_put_and_returns_rule() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg/securityRules/new-rule")
            .returning_json(serde_json::json!({
                "name": "new-rule",
                "properties": {
                    "priority": 300,
                    "protocol": "Tcp",
                    "access": "Allow",
                    "direction": "Inbound",
                    "sourcePortRange": "*",
                    "destinationPortRange": "8080",
                    "sourceAddressPrefix": "*",
                    "destinationAddressPrefix": "*",
                    "provisioningState": "Succeeded"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let body = SecurityRule {
            name: Some("new-rule".into()),
            properties: Some(crate::types::networking::SecurityRulePropertiesFormat {
                protocol: Some("Tcp".into()),
                source_port_range: Some("*".into()),
                destination_port_range: Some("8080".into()),
                source_address_prefix: Some("*".into()),
                destination_address_prefix: Some("*".into()),
                access: Some("Allow".into()),
                priority: Some(300),
                direction: Some("Inbound".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let rule = client
            .networking()
            .create_security_rule("rg1", "my-nsg", "new-rule", &body)
            .await
            .expect("create_security_rule failed");
        assert_eq!(rule.name.as_deref(), Some("new-rule"));
        assert_eq!(rule.properties.as_ref().and_then(|p| p.priority), Some(300),);
        assert_eq!(
            rule.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
    }

    #[tokio::test]
    async fn delete_security_rule_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkSecurityGroups/my-nsg/securityRules/old-rule")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .networking()
            .delete_security_rule("rg1", "my-nsg", "old-rule")
            .await
            .expect("delete_security_rule failed");
    }

    // ---- Load Balancer tests ----

    #[tokio::test]
    async fn list_load_balancers_returns_empty_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/loadBalancers")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_load_balancers("rg1")
            .await
            .expect("list_load_balancers failed");
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn list_load_balancers_returns_lb_with_standard_sku() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/loadBalancers")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/loadBalancers/my-lb",
                    "name": "my-lb",
                    "type": "Microsoft.Network/loadBalancers",
                    "location": "eastus",
                    "sku": { "name": "Standard", "tier": "Regional" },
                    "properties": {
                        "provisioningState": "Succeeded",
                        "frontendIPConfigurations": [],
                        "backendAddressPools": []
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_load_balancers("rg1")
            .await
            .expect("list_load_balancers failed");
        assert_eq!(result.value.len(), 1);
        let lb = &result.value[0];
        assert_eq!(lb.name.as_deref(), Some("my-lb"));
        assert_eq!(lb.location.as_deref(), Some("eastus"));
        assert_eq!(
            lb.sku.as_ref().and_then(|s| s.name.as_deref()),
            Some("Standard")
        );
        assert_eq!(
            lb.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
    }

    #[tokio::test]
    async fn list_load_balancers_all_uses_subscription_scope() {
        let mut mock = crate::MockClient::new();
        mock.expect_get(
            "/subscriptions/test-subscription-id/providers/Microsoft.Network/loadBalancers",
        )
        .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .networking()
            .list_load_balancers_all()
            .await
            .expect("list_load_balancers_all failed");
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn get_load_balancer_deserializes_frontend_ips() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/loadBalancers/my-lb")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Network/loadBalancers/my-lb",
                "name": "my-lb",
                "location": "eastus",
                "sku": { "name": "Standard" },
                "properties": {
                    "provisioningState": "Succeeded",
                    "frontendIPConfigurations": [{
                        "name": "frontend",
                        "properties": {
                            "privateIPAllocationMethod": "Dynamic",
                            "provisioningState": "Succeeded"
                        }
                    }]
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let lb = client
            .networking()
            .get_load_balancer("rg1", "my-lb")
            .await
            .expect("get_load_balancer failed");
        assert_eq!(lb.name.as_deref(), Some("my-lb"));
        assert!(lb.id.is_some());
        let frontend_ips = &lb.properties.as_ref().unwrap().frontend_ip_configurations;
        assert_eq!(frontend_ips.len(), 1);
        assert_eq!(frontend_ips[0].name.as_deref(), Some("frontend"));
        assert_eq!(
            frontend_ips[0]
                .properties
                .as_ref()
                .and_then(|p| p.private_ip_allocation_method.as_deref()),
            Some("Dynamic"),
        );
    }

    #[tokio::test]
    async fn create_load_balancer_sends_put_and_returns_lb() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/loadBalancers/new-lb")
            .returning_json(serde_json::json!({
                "name": "new-lb",
                "location": "eastus",
                "sku": { "name": "Standard" },
                "properties": {
                    "provisioningState": "Succeeded",
                    "frontendIPConfigurations": []
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let request = LoadBalancerCreateRequest {
            location: "eastus".into(),
            sku: Some(crate::types::networking::LoadBalancerSku {
                name: Some("Standard".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let lb = client
            .networking()
            .create_load_balancer("rg1", "new-lb", &request)
            .await
            .expect("create_load_balancer failed");
        assert_eq!(lb.name.as_deref(), Some("new-lb"));
        assert_eq!(
            lb.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
    }

    #[tokio::test]
    async fn delete_load_balancer_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/loadBalancers/old-lb")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .networking()
            .delete_load_balancer("rg1", "old-lb")
            .await
            .expect("delete_load_balancer failed");
    }

    // ---- Network Interface tests ----

    #[tokio::test]
    async fn delete_network_interface_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/networkInterfaces/old-nic")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .networking()
            .delete_network_interface("rg1", "old-nic")
            .await
            .expect("delete_network_interface failed");
    }

    // ---- NAT Gateway tests ----

    #[tokio::test]
    async fn delete_nat_gateway_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Network/natGateways/old-natgw")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .networking()
            .delete_nat_gateway("rg1", "old-natgw")
            .await
            .expect("delete_nat_gateway failed");
    }
}
