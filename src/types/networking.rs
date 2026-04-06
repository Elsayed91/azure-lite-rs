//! Types for the Azure Networking API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AddressSpace contains an array of IP address ranges.
///
/// **Azure API**: `networking.v1.AddressSpace`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//AddressSpace>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressSpace {
    /// A list of address blocks reserved for this virtual network in CIDR notation
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address_prefixes: Vec<String>,
}

impl AddressSpace {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            address_prefixes: vec![],
        }
    }
}

/// DhcpOptions contains an array of DNS servers available to VMs.
///
/// **Azure API**: `networking.v1.DhcpOptions`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//DhcpOptions>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhcpOptions {
    /// The list of DNS servers IP addresses
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
}

impl DhcpOptions {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            dns_servers: vec![],
        }
    }
}

/// Properties of the subnet.
///
/// **Azure API**: `networking.v1.SubnetPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//SubnetPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubnetPropertiesFormat {
    /// The address prefix for the subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_prefix: Option<String>,

    /// List of address prefixes for the subnet
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address_prefixes: Vec<String>,

    /// The provisioning state of the subnet resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Enable or Disable apply network policies on private endpoint in the subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_endpoint_network_policies: Option<String>,

    /// Enable or Disable apply network policies on private link service in the subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_service_network_policies: Option<String>,

    /// The reference to the NetworkSecurityGroup resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<SubResource>,
}

impl SubnetPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            address_prefix: Some("test-address_prefix".into()),
            address_prefixes: vec![],
            provisioning_state: Some("test-provisioning_state".into()),
            private_endpoint_network_policies: Some(
                "test-private_endpoint_network_policies".into(),
            ),
            private_link_service_network_policies: Some(
                "test-private_link_service_network_policies".into(),
            ),
            network_security_group: Some(SubResource::fixture()),
        }
    }
}

/// Subnet in a virtual network resource.
///
/// **Azure API**: `networking.v1.Subnet`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//Subnet>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subnet {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource that is unique within a resource group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// A unique read-only string that changes whenever the resource is updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Properties of the subnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubnetPropertiesFormat>,
}

impl Subnet {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-subnet".into()),
            r#type: Some("test-type".into()),
            etag: Some("test-etag".into()),
            properties: Some(SubnetPropertiesFormat::fixture()),
        }
    }
}

/// Reference to another sub resource.
///
/// **Azure API**: `networking.v1.SubResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//SubResource>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubResource {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl SubResource {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
        }
    }
}

/// Properties of the virtual network.
///
/// **Azure API**: `networking.v1.VirtualNetworkPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//VirtualNetworkPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetworkPropertiesFormat {
    /// The provisioning state of the virtual network resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The AddressSpace that contains an array of IP address ranges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_space: Option<AddressSpace>,

    /// The dhcpOptions that contains an array of DNS servers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_options: Option<DhcpOptions>,

    /// A list of subnets in a Virtual Network
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<Subnet>,

    /// A list of peerings in a Virtual Network
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_peerings: Vec<VirtualNetworkPeering>,

    /// Indicates if DDoS protection is enabled for all the protected resources in the virtual
    /// network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ddos_protection: Option<bool>,

    /// Indicates if VM protection is enabled for all the subnets in the virtual network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_vm_protection: Option<bool>,
}

impl VirtualNetworkPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provisioning_state: Some("test-provisioning_state".into()),
            address_space: Some(AddressSpace::fixture()),
            dhcp_options: Some(DhcpOptions::fixture()),
            subnets: vec![],
            virtual_network_peerings: vec![],
            enable_ddos_protection: Some(false),
            enable_vm_protection: Some(false),
        }
    }
}

/// Peerings in a virtual network resource.
///
/// **Azure API**: `networking.v1.VirtualNetworkPeering`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//VirtualNetworkPeering>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetworkPeering {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource that is unique within a resource group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Properties of the virtual network peering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkPeeringPropertiesFormat>,
}

impl VirtualNetworkPeering {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-virtual_network_peering".into()),
            properties: Some(VirtualNetworkPeeringPropertiesFormat::fixture()),
        }
    }
}

/// Properties of the virtual network peering.
///
/// **Azure API**: `networking.v1.VirtualNetworkPeeringPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//VirtualNetworkPeeringPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetworkPeeringPropertiesFormat {
    /// Whether the VMs in the local virtual network space would be able to access the VMs in
    /// remote virtual network space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_virtual_network_access: Option<bool>,

    /// Whether the forwarded traffic from the VMs in the local virtual network will be
    /// allowed/disallowed in remote virtual network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_forwarded_traffic: Option<bool>,

    /// The status of the virtual network peering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_state: Option<String>,

    /// The provisioning state of the virtual network peering resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The reference to the remote virtual network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_virtual_network: Option<SubResource>,
}

impl VirtualNetworkPeeringPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            allow_virtual_network_access: Some(false),
            allow_forwarded_traffic: Some(false),
            peering_state: Some("test-peering_state".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            remote_virtual_network: Some(SubResource::fixture()),
        }
    }
}

/// Virtual Network resource.
///
/// **Azure API**: `networking.v1.VirtualNetwork`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//VirtualNetwork>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetwork {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// A unique read-only string that changes whenever the resource is updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Properties of the virtual network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkPropertiesFormat>,
}

impl VirtualNetwork {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-virtual_network".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            etag: Some("test-etag".into()),
            properties: Some(VirtualNetworkPropertiesFormat::fixture()),
        }
    }
}

/// Response for the ListVirtualNetworks API service call.
///
/// **Azure API**: `networking.v1.VirtualNetworkListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//VirtualNetworkListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetworkListResult {
    /// A list of VirtualNetwork resources in a resource group
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetwork>,

    /// The URL to get the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl VirtualNetworkListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a virtual network.
///
/// **Azure API**: `networking.v1.VirtualNetworkCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//VirtualNetworkCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetworkCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties of the virtual network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkPropertiesFormat>,
}

impl VirtualNetworkCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(VirtualNetworkPropertiesFormat::fixture()),
        }
    }
}

/// Response for ListSubnets API service call.
///
/// **Azure API**: `networking.v1.SubnetListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//SubnetListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubnetListResult {
    /// The subnets in a virtual network
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Subnet>,

    /// The URL to get the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl SubnetListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Security rule resource.
///
/// **Azure API**: `networking.v1.SecurityRulePropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//SecurityRulePropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRulePropertiesFormat {
    /// A description for this rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Network protocol this rule applies to (Tcp, Udp, Icmp, Esp, Ah, *)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// The source port or range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<String>,

    /// The destination port or range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port_range: Option<String>,

    /// The CIDR or source IP range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address_prefix: Option<String>,

    /// The destination address prefix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address_prefix: Option<String>,

    /// The source port ranges
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_port_ranges: Vec<String>,

    /// The destination port ranges
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub destination_port_ranges: Vec<String>,

    /// The CIDR or source IP ranges
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub source_address_prefixes: Vec<String>,

    /// The destination address prefixes
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub destination_address_prefixes: Vec<String>,

    /// The network traffic is allowed or denied (Allow, Deny)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,

    /// The priority of the rule (100–4096)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    /// The direction of the rule (Inbound, Outbound)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    /// The provisioning state of the security rule resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}

impl SecurityRulePropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            description: Some("test-description".into()),
            protocol: Some("test-protocol".into()),
            source_port_range: Some("test-source_port_range".into()),
            destination_port_range: Some("test-destination_port_range".into()),
            source_address_prefix: Some("test-source_address_prefix".into()),
            destination_address_prefix: Some("test-destination_address_prefix".into()),
            source_port_ranges: vec![],
            destination_port_ranges: vec![],
            source_address_prefixes: vec![],
            destination_address_prefixes: vec![],
            access: Some("test-access".into()),
            priority: Some(100),
            direction: Some("test-direction".into()),
            provisioning_state: Some("test-provisioning_state".into()),
        }
    }
}

/// Network security rule.
///
/// **Azure API**: `networking.v1.SecurityRule`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//SecurityRule>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRule {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource that is unique within a resource group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// A unique read-only string that changes whenever the resource is updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Properties of the security rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityRulePropertiesFormat>,
}

impl SecurityRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-security_rule".into()),
            r#type: Some("test-type".into()),
            etag: Some("test-etag".into()),
            properties: Some(SecurityRulePropertiesFormat::fixture()),
        }
    }
}

/// Response for ListSecurityRule API service call.
///
/// **Azure API**: `networking.v1.SecurityRuleListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//SecurityRuleListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRuleListResult {
    /// The security rules in a network security group
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SecurityRule>,

    /// The URL to get the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl SecurityRuleListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Network Security Group resource.
///
/// **Azure API**: `networking.v1.NetworkSecurityGroupPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//NetworkSecurityGroupPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSecurityGroupPropertiesFormat {
    /// A collection of security rules of the network security group
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_rules: Vec<SecurityRule>,

    /// The default security rules of network security group
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_security_rules: Vec<SecurityRule>,

    /// A collection of references to network interfaces
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<SubResource>,

    /// A collection of references to subnets
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<SubResource>,

    /// The provisioning state of the network security group resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The resource GUID property of the network security group resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
}

impl NetworkSecurityGroupPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            security_rules: vec![],
            default_security_rules: vec![],
            network_interfaces: vec![],
            subnets: vec![],
            provisioning_state: Some("test-provisioning_state".into()),
            resource_guid: Some("test-resource_guid".into()),
        }
    }
}

/// NetworkSecurityGroup resource.
///
/// **Azure API**: `networking.v1.NetworkSecurityGroup`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//NetworkSecurityGroup>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSecurityGroup {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// A unique read-only string that changes whenever the resource is updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Properties of the network security group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkSecurityGroupPropertiesFormat>,
}

impl NetworkSecurityGroup {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-network_security_group".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            etag: Some("test-etag".into()),
            properties: Some(NetworkSecurityGroupPropertiesFormat::fixture()),
        }
    }
}

/// Response for ListNetworkSecurityGroups API service call.
///
/// **Azure API**: `networking.v1.NetworkSecurityGroupListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//NetworkSecurityGroupListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSecurityGroupListResult {
    /// A list of NetworkSecurityGroup resources
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkSecurityGroup>,

    /// The URL to get the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl NetworkSecurityGroupListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a network security group.
///
/// **Azure API**: `networking.v1.NetworkSecurityGroupCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//NetworkSecurityGroupCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSecurityGroupCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Properties of the network security group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkSecurityGroupPropertiesFormat>,
}

impl NetworkSecurityGroupCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            properties: Some(NetworkSecurityGroupPropertiesFormat::fixture()),
        }
    }
}

/// Frontend IP address of the load balancer.
///
/// **Azure API**: `networking.v1.FrontendIPConfiguration`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//FrontendIPConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendIPConfiguration {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource that is unique within a resource group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Properties of the frontend IP configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<FrontendIPConfigurationPropertiesFormat>,
}

impl FrontendIPConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-frontend_ip_configuration".into()),
            properties: Some(FrontendIPConfigurationPropertiesFormat::fixture()),
        }
    }
}

/// Properties of Frontend IP Configuration of the load balancer.
///
/// **Azure API**: `networking.v1.FrontendIPConfigurationPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//FrontendIPConfigurationPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendIPConfigurationPropertiesFormat {
    /// The private IP address of the IP configuration
    #[serde(rename = "privateIPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,

    /// The Private IP allocation method (Dynamic or Static)
    #[serde(rename = "privateIPAllocationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_allocation_method: Option<String>,

    /// The reference to the subnet resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,

    /// The reference to the Public IP resource
    #[serde(rename = "publicIPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<SubResource>,

    /// The provisioning state of the frontend IP configuration resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}

impl FrontendIPConfigurationPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            private_ip_address: Some("test-private_ip_address".into()),
            private_ip_allocation_method: Some("test-private_ip_allocation_method".into()),
            subnet: Some(SubResource::fixture()),
            public_ip_address: Some(SubResource::fixture()),
            provisioning_state: Some("test-provisioning_state".into()),
        }
    }
}

/// Pool of backend IP addresses.
///
/// **Azure API**: `networking.v1.BackendAddressPool`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//BackendAddressPool>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendAddressPool {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource that is unique within the set of backend address pools used by
    /// the load balancer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Properties of load balancer backend address pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendAddressPoolPropertiesFormat>,
}

impl BackendAddressPool {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-backend_address_pool".into()),
            properties: Some(BackendAddressPoolPropertiesFormat::fixture()),
        }
    }
}

/// Properties of the backend address pool.
///
/// **Azure API**: `networking.v1.BackendAddressPoolPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//BackendAddressPoolPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendAddressPoolPropertiesFormat {
    /// The provisioning state of the backend address pool resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// An array of backend addresses
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_backend_addresses: Vec<LoadBalancerBackendAddress>,
}

impl BackendAddressPoolPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provisioning_state: Some("test-provisioning_state".into()),
            load_balancer_backend_addresses: vec![],
        }
    }
}

/// Load balancer backend addresses.
///
/// **Azure API**: `networking.v1.LoadBalancerBackendAddress`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancerBackendAddress>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerBackendAddress {
    /// Name of the backend address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Properties of load balancer backend address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancerBackendAddressPropertiesFormat>,
}

impl LoadBalancerBackendAddress {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-load_balancer_backend_address".into()),
            properties: Some(LoadBalancerBackendAddressPropertiesFormat::fixture()),
        }
    }
}

/// Properties of the load balancer backend addresses.
///
/// **Azure API**: `networking.v1.LoadBalancerBackendAddressPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancerBackendAddressPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerBackendAddressPropertiesFormat {
    /// IP Address belonging to the referenced virtual network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// Reference to an existing virtual network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_network: Option<SubResource>,
}

impl LoadBalancerBackendAddressPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            ip_address: Some("test-ip_address".into()),
            virtual_network: Some(SubResource::fixture()),
        }
    }
}

/// A load balancer probe.
///
/// **Azure API**: `networking.v1.Probe`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//Probe>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Probe {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource that is unique within the set of probes used by the load
    /// balancer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Properties of load balancer probe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProbePropertiesFormat>,
}

impl Probe {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-probe".into()),
            properties: Some(ProbePropertiesFormat::fixture()),
        }
    }
}

/// Load balancer probe resource.
///
/// **Azure API**: `networking.v1.ProbePropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//ProbePropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbePropertiesFormat {
    /// The protocol of the end point (Http, Https, Tcp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// The port for communicating the probe
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// The interval, in seconds, for how frequently to probe the endpoint for health status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,

    /// The number of probes where if no response, will result in stopping further traffic from
    /// being delivered to the endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_probes: Option<i32>,

    /// The URI used for requesting health status from the VM (Http/Https only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_path: Option<String>,

    /// The provisioning state of the probe resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}

impl ProbePropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            protocol: Some("test-protocol".into()),
            port: Some(100),
            interval_in_seconds: Some(100),
            number_of_probes: Some(100),
            request_path: Some("test-request_path".into()),
            provisioning_state: Some("test-provisioning_state".into()),
        }
    }
}

/// A load balancing rule for a load balancer.
///
/// **Azure API**: `networking.v1.LoadBalancingRule`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancingRule>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancingRule {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the resource that is unique within the set of load balancing rules used by
    /// the load balancer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Properties of load balancer load balancing rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancingRulePropertiesFormat>,
}

impl LoadBalancingRule {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-load_balancing_rule".into()),
            properties: Some(LoadBalancingRulePropertiesFormat::fixture()),
        }
    }
}

/// Properties of the load balancer.
///
/// **Azure API**: `networking.v1.LoadBalancingRulePropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancingRulePropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancingRulePropertiesFormat {
    /// The reference to the transport protocol used by the load balancing rule (Tcp, Udp, All)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    /// The port for the external endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,

    /// The port used for internal connections on the endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,

    /// A reference to frontend IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_ip_configuration: Option<SubResource>,

    /// A reference to a pool of DIPs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_address_pool: Option<SubResource>,

    /// The reference to the load balancer probe used by the load balancing rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probe: Option<SubResource>,

    /// Configures a virtual machine's endpoint for the floating IP capability required to
    /// configure a SQL AlwaysOn Availability Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_floating_ip: Option<bool>,

    /// The timeout for the TCP idle connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,

    /// The load distribution policy for this rule (Default, SourceIP, SourceIPProtocol)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_distribution: Option<String>,

    /// The provisioning state of the load balancing rule resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}

impl LoadBalancingRulePropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            protocol: Some("test-protocol".into()),
            frontend_port: Some(100),
            backend_port: Some(100),
            frontend_ip_configuration: Some(SubResource::fixture()),
            backend_address_pool: Some(SubResource::fixture()),
            probe: Some(SubResource::fixture()),
            enable_floating_ip: Some(false),
            idle_timeout_in_minutes: Some(100),
            load_distribution: Some("test-load_distribution".into()),
            provisioning_state: Some("test-provisioning_state".into()),
        }
    }
}

/// SKU of a load balancer.
///
/// **Azure API**: `networking.v1.LoadBalancerSku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancerSku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerSku {
    /// Name of a load balancer SKU (Basic, Standard, Gateway)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tier of a load balancer SKU (Regional, Global)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl LoadBalancerSku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-load_balancer_sku".into()),
            tier: Some("test-tier".into()),
        }
    }
}

/// Properties of the load balancer.
///
/// **Azure API**: `networking.v1.LoadBalancerPropertiesFormat`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancerPropertiesFormat>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerPropertiesFormat {
    /// Object representing the frontend IPs to be used for the load balancer
    #[serde(rename = "frontendIPConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub frontend_ip_configurations: Vec<FrontendIPConfiguration>,

    /// Collection of backend address pools used by a load balancer
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub backend_address_pools: Vec<BackendAddressPool>,

    /// Collection of probe objects used in the load balancer
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub probes: Vec<Probe>,

    /// Object collection representing the load balancing rules Gets the provisioning
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub load_balancing_rules: Vec<LoadBalancingRule>,

    /// The provisioning state of the load balancer resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The resource GUID property of the load balancer resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
}

impl LoadBalancerPropertiesFormat {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            frontend_ip_configurations: vec![],
            backend_address_pools: vec![],
            probes: vec![],
            load_balancing_rules: vec![],
            provisioning_state: Some("test-provisioning_state".into()),
            resource_guid: Some("test-resource_guid".into()),
        }
    }
}

/// LoadBalancer resource.
///
/// **Azure API**: `networking.v1.LoadBalancer`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancer>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancer {
    /// Resource ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Resource name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Resource type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// A unique read-only string that changes whenever the resource is updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// The load balancer SKU
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<LoadBalancerSku>,

    /// Properties of load balancer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancerPropertiesFormat>,
}

impl LoadBalancer {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-load_balancer".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            etag: Some("test-etag".into()),
            sku: Some(LoadBalancerSku::fixture()),
            properties: Some(LoadBalancerPropertiesFormat::fixture()),
        }
    }
}

/// Response for ListLoadBalancers API service call.
///
/// **Azure API**: `networking.v1.LoadBalancerListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancerListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerListResult {
    /// A list of load balancers in a resource group
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LoadBalancer>,

    /// The URL to get the next set of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl LoadBalancerListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a load balancer.
///
/// **Azure API**: `networking.v1.LoadBalancerCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/virtualnetwork//LoadBalancerCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The load balancer SKU
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<LoadBalancerSku>,

    /// Properties of load balancer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoadBalancerPropertiesFormat>,
}

impl LoadBalancerCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            sku: Some(LoadBalancerSku::fixture()),
            properties: Some(LoadBalancerPropertiesFormat::fixture()),
        }
    }
}
