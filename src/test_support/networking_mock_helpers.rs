//! MockClient helpers for Azure Networking API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Networking helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait NetworkingMockHelpers {
    /// Helper to expect `list_vnets`: Gets all virtual networks in a resource group.
    fn expect_list_vnets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_vnets_all`: Gets all virtual networks in a subscription.
    fn expect_list_vnets_all(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_vnet`: Gets the specified virtual network by resource group.
    fn expect_get_vnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
        expand: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_vnet`: Creates or updates a virtual network in the specified
    /// resource group.
    fn expect_create_vnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_vnet`: Deletes the specified virtual network.
    fn expect_delete_vnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_subnets`: Gets all subnets in a virtual network.
    fn expect_list_subnets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_subnet`: Gets the specified subnet by virtual network and resource
    /// group.
    fn expect_get_subnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
        subnet_name: &str,
        expand: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_nsgs`: Gets all network security groups in a resource group.
    fn expect_list_nsgs(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_nsgs_all`: Gets all network security groups in a subscription.
    fn expect_list_nsgs_all(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_nsg`: Gets the specified network security group.
    fn expect_get_nsg(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        expand: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_nsg`: Creates or updates a network security group in the specified
    /// resource group.
    fn expect_create_nsg(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_nsg`: Deletes the specified network security group.
    fn expect_delete_nsg(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_security_rules`: Gets all security rules in a network security group.
    fn expect_list_security_rules(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_security_rule`: Get the specified network security rule.
    fn expect_get_security_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_security_rule`: Creates or updates a security rule in the specified
    /// network security group.
    fn expect_create_security_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_security_rule`: Deletes the specified network security rule.
    fn expect_delete_security_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_load_balancers`: Gets all the load balancers in a resource group.
    fn expect_list_load_balancers(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_load_balancers_all`: Gets all the load balancers in a subscription.
    fn expect_list_load_balancers_all(&mut self, subscription_id: &str) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_load_balancer`: Gets the specified load balancer.
    fn expect_get_load_balancer(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
        expand: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_load_balancer`: Creates or updates a load balancer.
    fn expect_create_load_balancer(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_load_balancer`: Deletes the specified load balancer.
    fn expect_delete_load_balancer(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_network_interface`: Deletes the specified network interface.
    fn expect_delete_network_interface(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_interface_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_nat_gateway`: Deletes the specified NAT gateway.
    fn expect_delete_nat_gateway(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        nat_gateway_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl NetworkingMockHelpers for MockClient {
    /// Helper to expect `list_vnets`: Gets all virtual networks in a resource group.
    fn expect_list_vnets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_vnets_all`: Gets all virtual networks in a subscription.
    fn expect_list_vnets_all(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.Network/virtualNetworks");
        self.expect_get(&path)
    }

    /// Helper to expect `get_vnet`: Gets the specified virtual network by resource group.
    fn expect_get_vnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
        expand: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks/{virtual_network_name}"
        );
        let mut __qp: Vec<String> = Vec::new();
        if !expand.is_empty() {
            __qp.push(format!("$expand={}", expand));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_vnet`: Creates or updates a virtual network in the specified
    /// resource group.
    fn expect_create_vnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks/{virtual_network_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_vnet`: Deletes the specified virtual network.
    fn expect_delete_vnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks/{virtual_network_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_subnets`: Gets all subnets in a virtual network.
    fn expect_list_subnets(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks/{virtual_network_name}/subnets"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_subnet`: Gets the specified subnet by virtual network and resource
    /// group.
    fn expect_get_subnet(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
        subnet_name: &str,
        expand: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/virtualNetworks/{virtual_network_name}/subnets/{subnet_name}"
        );
        let mut __qp: Vec<String> = Vec::new();
        if !expand.is_empty() {
            __qp.push(format!("$expand={}", expand));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `list_nsgs`: Gets all network security groups in a resource group.
    fn expect_list_nsgs(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_nsgs_all`: Gets all network security groups in a subscription.
    fn expect_list_nsgs_all(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/providers/Microsoft.Network/networkSecurityGroups"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_nsg`: Gets the specified network security group.
    fn expect_get_nsg(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        expand: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups/{network_security_group_name}"
        );
        let mut __qp: Vec<String> = Vec::new();
        if !expand.is_empty() {
            __qp.push(format!("$expand={}", expand));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_nsg`: Creates or updates a network security group in the specified
    /// resource group.
    fn expect_create_nsg(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups/{network_security_group_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_nsg`: Deletes the specified network security group.
    fn expect_delete_nsg(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups/{network_security_group_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_security_rules`: Gets all security rules in a network security group.
    fn expect_list_security_rules(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups/{network_security_group_name}/securityRules"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_security_rule`: Get the specified network security rule.
    fn expect_get_security_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups/{network_security_group_name}/securityRules/{security_rule_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_security_rule`: Creates or updates a security rule in the specified
    /// network security group.
    fn expect_create_security_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups/{network_security_group_name}/securityRules/{security_rule_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_security_rule`: Deletes the specified network security rule.
    fn expect_delete_security_rule(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkSecurityGroups/{network_security_group_name}/securityRules/{security_rule_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_load_balancers`: Gets all the load balancers in a resource group.
    fn expect_list_load_balancers(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/loadBalancers"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_load_balancers_all`: Gets all the load balancers in a subscription.
    fn expect_list_load_balancers_all(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path =
            format!("/subscriptions/{subscription_id}/providers/Microsoft.Network/loadBalancers");
        self.expect_get(&path)
    }

    /// Helper to expect `get_load_balancer`: Gets the specified load balancer.
    fn expect_get_load_balancer(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
        expand: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/loadBalancers/{load_balancer_name}"
        );
        let mut __qp: Vec<String> = Vec::new();
        if !expand.is_empty() {
            __qp.push(format!("$expand={}", expand));
        }
        if !__qp.is_empty() {
            path = format!("{}?{}", path, __qp.join("&"));
        }
        self.expect_get(&path)
    }

    /// Helper to expect `create_load_balancer`: Creates or updates a load balancer.
    fn expect_create_load_balancer(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/loadBalancers/{load_balancer_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_load_balancer`: Deletes the specified load balancer.
    fn expect_delete_load_balancer(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/loadBalancers/{load_balancer_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `delete_network_interface`: Deletes the specified network interface.
    fn expect_delete_network_interface(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        network_interface_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/networkInterfaces/{network_interface_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `delete_nat_gateway`: Deletes the specified NAT gateway.
    fn expect_delete_nat_gateway(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        nat_gateway_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Network/natGateways/{nat_gateway_name}"
        );
        self.expect_delete(&path)
    }
}
