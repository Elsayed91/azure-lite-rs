//! Operation contracts for the Azure Networking API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/networking.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::networking::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Networking API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::networking::NetworkingClient`] instead.
pub struct NetworkingOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> NetworkingOps<'a> {
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

    /// Gets all virtual networks in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`VirtualNetworkListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_vnets(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<VirtualNetworkListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_vnets response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_vnets response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets all virtual networks in a subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Network/virtualNetworks`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`VirtualNetworkListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_vnets_all(
        &self,
        subscription_id: &str,
    ) -> Result<VirtualNetworkListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Network/virtualNetworks",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_vnets_all response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_vnets_all response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the specified virtual network by resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `virtualNetworkName` —  *(required)*
    ///
    /// # Query Parameters
    /// - `$expand` — Expands referenced resources
    ///
    /// # Response
    /// [`VirtualNetwork`]
    #[allow(dead_code)]
    pub(crate) async fn get_vnet(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
        expand: &str,
    ) -> Result<VirtualNetwork> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(virtual_network_name),
        );
        let url = crate::append_query_params(url, &[("$expand", expand)]);
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_vnet response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_vnet response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a virtual network in the specified resource group.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `virtualNetworkName` —  *(required)*
    ///
    /// # Request Body
    /// [`VirtualNetworkCreateRequest`]
    ///
    /// # Response
    /// [`VirtualNetwork`]
    #[allow(dead_code)]
    pub(crate) async fn create_vnet(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
        body: &VirtualNetworkCreateRequest,
    ) -> Result<VirtualNetwork> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(virtual_network_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_vnet request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_vnet response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_vnet response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes the specified virtual network.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `virtualNetworkName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_vnet(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(virtual_network_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Gets all subnets in a virtual network.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `virtualNetworkName` —  *(required)*
    ///
    /// # Response
    /// [`SubnetListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_subnets(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
    ) -> Result<SubnetListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}/subnets",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(virtual_network_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_subnets response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_subnets response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the specified subnet by virtual network and resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `virtualNetworkName` —  *(required)*
    /// - `subnetName` —  *(required)*
    ///
    /// # Query Parameters
    /// - `$expand` — Expands referenced resources
    ///
    /// # Response
    /// [`Subnet`]
    #[allow(dead_code)]
    pub(crate) async fn get_subnet(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        virtual_network_name: &str,
        subnet_name: &str,
        expand: &str,
    ) -> Result<Subnet> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}/subnets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(virtual_network_name),
            encode(subnet_name),
        );
        let url = crate::append_query_params(url, &[("$expand", expand)]);
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_subnet response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_subnet response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets all network security groups in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`NetworkSecurityGroupListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_nsgs(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<NetworkSecurityGroupListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_nsgs response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_nsgs response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets all network security groups in a subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Network/networkSecurityGroups`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`NetworkSecurityGroupListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_nsgs_all(
        &self,
        subscription_id: &str,
    ) -> Result<NetworkSecurityGroupListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Network/networkSecurityGroups",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_nsgs_all response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_nsgs_all response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the specified network security group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{networkSecurityGroupName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkSecurityGroupName` —  *(required)*
    ///
    /// # Query Parameters
    /// - `$expand` — Expands referenced resources
    ///
    /// # Response
    /// [`NetworkSecurityGroup`]
    #[allow(dead_code)]
    pub(crate) async fn get_nsg(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        expand: &str,
    ) -> Result<NetworkSecurityGroup> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_security_group_name),
        );
        let url = crate::append_query_params(url, &[("$expand", expand)]);
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_nsg response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_nsg response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a network security group in the specified resource group.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{networkSecurityGroupName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkSecurityGroupName` —  *(required)*
    ///
    /// # Request Body
    /// [`NetworkSecurityGroupCreateRequest`]
    ///
    /// # Response
    /// [`NetworkSecurityGroup`]
    #[allow(dead_code)]
    pub(crate) async fn create_nsg(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        body: &NetworkSecurityGroupCreateRequest,
    ) -> Result<NetworkSecurityGroup> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_security_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_nsg request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_nsg response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_nsg response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes the specified network security group.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{networkSecurityGroupName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkSecurityGroupName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_nsg(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_security_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Gets all security rules in a network security group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{networkSecurityGroupName}/securityRules`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkSecurityGroupName` —  *(required)*
    ///
    /// # Response
    /// [`SecurityRuleListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_security_rules(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
    ) -> Result<SecurityRuleListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}/securityRules",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_security_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_security_rules response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_security_rules response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get the specified network security rule.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{networkSecurityGroupName}/securityRules/{securityRuleName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkSecurityGroupName` —  *(required)*
    /// - `securityRuleName` —  *(required)*
    ///
    /// # Response
    /// [`SecurityRule`]
    #[allow(dead_code)]
    pub(crate) async fn get_security_rule(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> Result<SecurityRule> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}/securityRules/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_security_group_name),
            encode(security_rule_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_security_rule response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_security_rule response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a security rule in the specified network security group.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{networkSecurityGroupName}/securityRules/{securityRuleName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkSecurityGroupName` —  *(required)*
    /// - `securityRuleName` —  *(required)*
    ///
    /// # Request Body
    /// [`SecurityRule`]
    ///
    /// # Response
    /// [`SecurityRule`]
    #[allow(dead_code)]
    pub(crate) async fn create_security_rule(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
        body: &SecurityRule,
    ) -> Result<SecurityRule> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}/securityRules/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_security_group_name),
            encode(security_rule_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_security_rule request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_security_rule response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_security_rule response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes the specified network security rule.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkSecurityGroups/{networkSecurityGroupName}/securityRules/{securityRuleName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkSecurityGroupName` —  *(required)*
    /// - `securityRuleName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_security_rule(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_security_group_name: &str,
        security_rule_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkSecurityGroups/{}/securityRules/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_security_group_name),
            encode(security_rule_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Gets all the load balancers in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/loadBalancers`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`LoadBalancerListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_load_balancers(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<LoadBalancerListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_load_balancers response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_load_balancers response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets all the load balancers in a subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Network/loadBalancers`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`LoadBalancerListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_load_balancers_all(
        &self,
        subscription_id: &str,
    ) -> Result<LoadBalancerListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Network/loadBalancers",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_load_balancers_all response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_load_balancers_all response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets the specified load balancer.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/loadBalancers/{loadBalancerName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `loadBalancerName` —  *(required)*
    ///
    /// # Query Parameters
    /// - `$expand` — Expands referenced resources
    ///
    /// # Response
    /// [`LoadBalancer`]
    #[allow(dead_code)]
    pub(crate) async fn get_load_balancer(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
        expand: &str,
    ) -> Result<LoadBalancer> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(load_balancer_name),
        );
        let url = crate::append_query_params(url, &[("$expand", expand)]);
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_load_balancer response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_load_balancer response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a load balancer.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/loadBalancers/{loadBalancerName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `loadBalancerName` —  *(required)*
    ///
    /// # Request Body
    /// [`LoadBalancerCreateRequest`]
    ///
    /// # Response
    /// [`LoadBalancer`]
    #[allow(dead_code)]
    pub(crate) async fn create_load_balancer(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
        body: &LoadBalancerCreateRequest,
    ) -> Result<LoadBalancer> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(load_balancer_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_load_balancer request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_load_balancer response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_load_balancer response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes the specified load balancer.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/loadBalancers/{loadBalancerName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `loadBalancerName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_load_balancer(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        load_balancer_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/loadBalancers/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(load_balancer_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Deletes the specified network interface.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/networkInterfaces/{networkInterfaceName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `networkInterfaceName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_network_interface(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        network_interface_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkInterfaces/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(network_interface_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Deletes the specified NAT gateway.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Network/natGateways/{natGatewayName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `natGatewayName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_nat_gateway(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        nat_gateway_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/natGateways/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(nat_gateway_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-01-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_vnets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/virtualNetworks")
            .returning_json(serde_json::to_value(VirtualNetworkListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .list_vnets("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_vnets_all() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Network/virtualNetworks",
        )
        .returning_json(serde_json::to_value(VirtualNetworkListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops.list_vnets_all("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_vnet() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/virtualNetworks/test-virtualNetworkName?$expand=test-%24expand")
            .returning_json(serde_json::to_value(VirtualNetwork::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .get_vnet(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-virtualNetworkName",
                "test-$expand",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_vnet() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/virtualNetworks/test-virtualNetworkName")
            .returning_json(serde_json::to_value(VirtualNetwork::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let body = VirtualNetworkCreateRequest::fixture();
        let result = ops
            .create_vnet(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-virtualNetworkName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_vnet() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/virtualNetworks/test-virtualNetworkName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .delete_vnet(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-virtualNetworkName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_subnets() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/virtualNetworks/test-virtualNetworkName/subnets")
            .returning_json(serde_json::to_value(SubnetListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .list_subnets(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-virtualNetworkName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_subnet() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/virtualNetworks/test-virtualNetworkName/subnets/test-subnetName?$expand=test-%24expand")
            .returning_json(serde_json::to_value(Subnet::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .get_subnet(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-virtualNetworkName",
                "test-subnetName",
                "test-$expand",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_nsgs() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups")
            .returning_json(serde_json::to_value(NetworkSecurityGroupListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .list_nsgs("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_nsgs_all() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Network/networkSecurityGroups",
        )
        .returning_json(serde_json::to_value(NetworkSecurityGroupListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops.list_nsgs_all("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_nsg() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups/test-networkSecurityGroupName?$expand=test-%24expand")
            .returning_json(serde_json::to_value(NetworkSecurityGroup::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .get_nsg(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkSecurityGroupName",
                "test-$expand",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_nsg() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups/test-networkSecurityGroupName")
            .returning_json(serde_json::to_value(NetworkSecurityGroup::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let body = NetworkSecurityGroupCreateRequest::fixture();
        let result = ops
            .create_nsg(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkSecurityGroupName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_nsg() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups/test-networkSecurityGroupName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .delete_nsg(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkSecurityGroupName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_security_rules() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups/test-networkSecurityGroupName/securityRules")
            .returning_json(serde_json::to_value(SecurityRuleListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .list_security_rules(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkSecurityGroupName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_security_rule() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups/test-networkSecurityGroupName/securityRules/test-securityRuleName")
            .returning_json(serde_json::to_value(SecurityRule::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .get_security_rule(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkSecurityGroupName",
                "test-securityRuleName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_security_rule() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups/test-networkSecurityGroupName/securityRules/test-securityRuleName")
            .returning_json(serde_json::to_value(SecurityRule::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let body = SecurityRule::fixture();
        let result = ops
            .create_security_rule(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkSecurityGroupName",
                "test-securityRuleName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_security_rule() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkSecurityGroups/test-networkSecurityGroupName/securityRules/test-securityRuleName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .delete_security_rule(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkSecurityGroupName",
                "test-securityRuleName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_load_balancers() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/loadBalancers")
            .returning_json(serde_json::to_value(LoadBalancerListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .list_load_balancers("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_load_balancers_all() {
        let mut mock = crate::MockClient::new();

        mock.expect_get(
            "/subscriptions/test-subscriptionId/providers/Microsoft.Network/loadBalancers",
        )
        .returning_json(serde_json::to_value(LoadBalancerListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops.list_load_balancers_all("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_load_balancer() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/loadBalancers/test-loadBalancerName?$expand=test-%24expand")
            .returning_json(serde_json::to_value(LoadBalancer::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .get_load_balancer(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-loadBalancerName",
                "test-$expand",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_load_balancer() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/loadBalancers/test-loadBalancerName")
            .returning_json(serde_json::to_value(LoadBalancer::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let body = LoadBalancerCreateRequest::fixture();
        let result = ops
            .create_load_balancer(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-loadBalancerName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_load_balancer() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/loadBalancers/test-loadBalancerName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .delete_load_balancer(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-loadBalancerName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_network_interface() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/networkInterfaces/test-networkInterfaceName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .delete_network_interface(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-networkInterfaceName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_nat_gateway() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Network/natGateways/test-natGatewayName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = NetworkingOps::new(&client);

        let result = ops
            .delete_nat_gateway(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-natGatewayName",
            )
            .await;
        assert!(result.is_ok());
    }
}
