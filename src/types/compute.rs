//! Types for the Azure Compute API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An Azure virtual machine resource.
///
/// **Azure API**: `compute.v1.VirtualMachine`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachine>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachine {
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

    /// Availability zones
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,

    /// VM properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
}

impl VirtualMachine {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-virtual_machine".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            zones: vec![],
            properties: Some(VirtualMachineProperties::fixture()),
        }
    }
}

/// Properties of a virtual machine.
///
/// **Azure API**: `compute.v1.VirtualMachineProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineProperties {
    /// Unique VM identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,

    /// Provisioning state (Succeeded, Failed, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Hardware profile (VM size)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,

    /// Storage profile (disks, image)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,

    /// OS profile (admin user, computer name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,

    /// Network profile (NICs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,

    /// Diagnostics profile (boot diagnostics)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<DiagnosticsProfile>,

    /// License type for hybrid benefit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,

    /// VM priority (Regular, Low, Spot)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    /// Eviction policy for Spot VMs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<String>,

    /// Time when the VM was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,

    /// Instance view status (when $expand=instanceView)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<VirtualMachineInstanceView>,
}

impl VirtualMachineProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vm_id: Some("test-vm_id".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            hardware_profile: Some(HardwareProfile::fixture()),
            storage_profile: Some(StorageProfile::fixture()),
            os_profile: Some(OsProfile::fixture()),
            network_profile: Some(NetworkProfile::fixture()),
            diagnostics_profile: Some(DiagnosticsProfile::fixture()),
            license_type: Some("test-license_type".into()),
            priority: Some("test-priority".into()),
            eviction_policy: Some("test-eviction_policy".into()),
            time_created: Some("test-time_created".into()),
            instance_view: Some(VirtualMachineInstanceView::fixture()),
        }
    }
}

/// Specifies the hardware settings for the virtual machine.
///
/// **Azure API**: `compute.v1.HardwareProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//HardwareProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareProfile {
    /// VM size name (e.g. Standard_D2s_v3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}

impl HardwareProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vm_size: Some("test-vm_size".into()),
        }
    }
}

/// Storage profile for a VM.
///
/// **Azure API**: `compute.v1.StorageProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//StorageProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageProfile {
    /// Image reference for the OS disk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,

    /// OS disk settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,

    /// Data disks attached to the VM
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
}

impl StorageProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            image_reference: Some(ImageReference::fixture()),
            os_disk: Some(OsDisk::fixture()),
            data_disks: vec![],
        }
    }
}

/// Reference to a platform or marketplace image.
///
/// **Azure API**: `compute.v1.ImageReference`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//ImageReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageReference {
    /// Resource ID of a shared gallery image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Image publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    /// Image offer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,

    /// Image SKU
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,

    /// Image version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Exact image version in use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
}

impl ImageReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            publisher: Some("test-publisher".into()),
            offer: Some("test-offer".into()),
            sku: Some("test-sku".into()),
            version: Some("test-version".into()),
            exact_version: Some("test-exact_version".into()),
        }
    }
}

/// OS disk settings.
///
/// **Azure API**: `compute.v1.OsDisk`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//OsDisk>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsDisk {
    /// Operating system type (Windows or Linux)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,

    /// Disk name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// How the disk was created (FromImage, Attach, Empty)
    pub create_option: String,

    /// Host caching mode (None, ReadOnly, ReadWrite)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,

    /// Disk size in GB
    #[serde(rename = "diskSizeGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,

    /// Managed disk parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDiskParameters>,

    /// Delete option on VM delete (Delete, Detach)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<String>,
}

impl OsDisk {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            os_type: Some("test-os_type".into()),
            name: Some("test-os_disk".into()),
            create_option: "test-create_option".into(),
            caching: Some("test-caching".into()),
            disk_size_gb: Some(100),
            managed_disk: Some(ManagedDiskParameters::fixture()),
            delete_option: Some("test-delete_option".into()),
        }
    }
}

/// A data disk attached to a VM.
///
/// **Azure API**: `compute.v1.DataDisk`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DataDisk>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataDisk {
    /// Logical unit number
    pub lun: i32,

    /// Disk name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// How the disk was created
    pub create_option: String,

    /// Host caching mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,

    /// Disk size in GB
    #[serde(rename = "diskSizeGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,

    /// Managed disk parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDiskParameters>,

    /// Delete option on VM delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<String>,
}

impl DataDisk {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            lun: 100,
            name: Some("test-data_disk".into()),
            create_option: "test-create_option".into(),
            caching: Some("test-caching".into()),
            disk_size_gb: Some(100),
            managed_disk: Some(ManagedDiskParameters::fixture()),
            delete_option: Some("test-delete_option".into()),
        }
    }
}

/// Managed disk parameters.
///
/// **Azure API**: `compute.v1.ManagedDiskParameters`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//ManagedDiskParameters>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedDiskParameters {
    /// Resource ID of the managed disk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Storage account type (Standard_LRS, Premium_LRS, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<String>,
}

impl ManagedDiskParameters {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            storage_account_type: Some("test-storage_account_type".into()),
        }
    }
}

/// OS profile for a VM.
///
/// **Azure API**: `compute.v1.OsProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//OsProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsProfile {
    /// Computer host name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// Admin username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,

    /// Admin password (write-only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,

    /// Linux-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<LinuxConfiguration>,

    /// Windows-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
}

impl OsProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            computer_name: Some("test-computer_name".into()),
            admin_username: Some("test-admin_username".into()),
            admin_password: Some("test-admin_password".into()),
            linux_configuration: Some(LinuxConfiguration::fixture()),
            windows_configuration: Some(WindowsConfiguration::fixture()),
        }
    }
}

/// Linux-specific OS configuration.
///
/// **Azure API**: `compute.v1.LinuxConfiguration`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//LinuxConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxConfiguration {
    /// Disable password auth (use SSH keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_password_authentication: Option<bool>,

    /// Whether to provision the VM agent
    #[serde(rename = "provisionVMAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_vm_agent: Option<bool>,

    /// SSH configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SshConfiguration>,
}

impl LinuxConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            disable_password_authentication: Some(false),
            provision_vm_agent: Some(false),
            ssh: Some(SshConfiguration::fixture()),
        }
    }
}

/// Windows-specific OS configuration.
///
/// **Azure API**: `compute.v1.WindowsConfiguration`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//WindowsConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsConfiguration {
    /// Whether to provision the VM agent
    #[serde(rename = "provisionVMAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_vm_agent: Option<bool>,

    /// Enable automatic Windows updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_automatic_updates: Option<bool>,
}

impl WindowsConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provision_vm_agent: Some(false),
            enable_automatic_updates: Some(false),
        }
    }
}

/// SSH configuration.
///
/// **Azure API**: `compute.v1.SshConfiguration`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//SshConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshConfiguration {
    /// SSH public keys
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<SshPublicKey>,
}

impl SshConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            public_keys: vec![],
        }
    }
}

/// SSH public key for Linux VMs.
///
/// **Azure API**: `compute.v1.SshPublicKey`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//SshPublicKey>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshPublicKey {
    /// Path where the key is placed (e.g. /home/user/.ssh/authorized_keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// SSH public key data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}

impl SshPublicKey {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            path: Some("test-path".into()),
            key_data: Some("test-key_data".into()),
        }
    }
}

/// Network profile for a VM.
///
/// **Azure API**: `compute.v1.NetworkProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//NetworkProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkProfile {
    /// Network interfaces attached to the VM
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterfaceReference>,
}

impl NetworkProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            network_interfaces: vec![],
        }
    }
}

/// Reference to a network interface.
///
/// **Azure API**: `compute.v1.NetworkInterfaceReference`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//NetworkInterfaceReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterfaceReference {
    /// Resource ID of the NIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// NIC reference properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceReferenceProperties>,
}

impl NetworkInterfaceReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            properties: Some(NetworkInterfaceReferenceProperties::fixture()),
        }
    }
}

/// Properties of a NIC reference.
///
/// **Azure API**: `compute.v1.NetworkInterfaceReferenceProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//NetworkInterfaceReferenceProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterfaceReferenceProperties {
    /// Whether this is the primary NIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,

    /// Delete option on VM delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<String>,
}

impl NetworkInterfaceReferenceProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            primary: Some(false),
            delete_option: Some("test-delete_option".into()),
        }
    }
}

/// Diagnostics profile.
///
/// **Azure API**: `compute.v1.DiagnosticsProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DiagnosticsProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsProfile {
    /// Boot diagnostics settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_diagnostics: Option<BootDiagnostics>,
}

impl DiagnosticsProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            boot_diagnostics: Some(BootDiagnostics::fixture()),
        }
    }
}

/// Boot diagnostics configuration.
///
/// **Azure API**: `compute.v1.BootDiagnostics`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//BootDiagnostics>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootDiagnostics {
    /// Whether boot diagnostics is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Storage account URI for boot diagnostics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}

impl BootDiagnostics {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enabled: Some(false),
            storage_uri: Some("test-storage_uri".into()),
        }
    }
}

/// Instance view of a virtual machine.
///
/// **Azure API**: `compute.v1.VirtualMachineInstanceView`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineInstanceView>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineInstanceView {
    /// Update domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<i32>,

    /// Fault domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<i32>,

    /// Computer name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// Operating system name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,

    /// Operating system version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,

    /// VM agent instance view
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_agent: Option<VirtualMachineAgentInstanceView>,

    /// Resource status information
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}

impl VirtualMachineInstanceView {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            platform_update_domain: Some(100),
            platform_fault_domain: Some(100),
            computer_name: Some("test-computer_name".into()),
            os_name: Some("test-os_name".into()),
            os_version: Some("test-os_version".into()),
            vm_agent: Some(VirtualMachineAgentInstanceView::fixture()),
            statuses: vec![],
        }
    }
}

/// Instance view of the VM agent.
///
/// **Azure API**: `compute.v1.VirtualMachineAgentInstanceView`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineAgentInstanceView>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineAgentInstanceView {
    /// VM agent version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_agent_version: Option<String>,

    /// Agent status information
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}

impl VirtualMachineAgentInstanceView {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            vm_agent_version: Some("test-vm_agent_version".into()),
            statuses: vec![],
        }
    }
}

/// Instance view status.
///
/// **Azure API**: `compute.v1.InstanceViewStatus`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//InstanceViewStatus>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceViewStatus {
    /// Status code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Level (Info, Warning, Error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,

    /// Display status text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_status: Option<String>,

    /// Detailed status message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Status time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl InstanceViewStatus {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            code: Some("test-code".into()),
            level: Some("test-level".into()),
            display_status: Some("test-display_status".into()),
            message: Some("test-message".into()),
            time: Some("test-time".into()),
        }
    }
}

/// Response for listing virtual machines.
///
/// **Azure API**: `compute.v1.VirtualMachineListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineListResult {
    /// List of virtual machines
    #[serde(default)]
    pub value: Vec<VirtualMachine>,

    /// URL to get the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl VirtualMachineListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a virtual machine.
///
/// **Azure API**: `compute.v1.VirtualMachineCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// Availability zones
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,

    /// VM properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
}

impl VirtualMachineCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            zones: vec![],
            properties: Some(VirtualMachineProperties::fixture()),
        }
    }
}

/// Response for getting a VM instance view.
///
/// **Azure API**: `compute.v1.VirtualMachineInstanceViewResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineInstanceViewResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineInstanceViewResult {
    /// Update domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<i32>,

    /// Fault domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<i32>,

    /// Computer name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,

    /// Operating system name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,

    /// Operating system version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,

    /// VM agent instance view
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_agent: Option<VirtualMachineAgentInstanceView>,

    /// Resource status information
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}

impl VirtualMachineInstanceViewResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            platform_update_domain: Some(100),
            platform_fault_domain: Some(100),
            computer_name: Some("test-computer_name".into()),
            os_name: Some("test-os_name".into()),
            os_version: Some("test-os_version".into()),
            vm_agent: Some(VirtualMachineAgentInstanceView::fixture()),
            statuses: vec![],
        }
    }
}

/// Describes a virtual machine scale set sku.
///
/// **Azure API**: `compute.v1.Sku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//Sku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sku {
    /// The sku name (e.g. Standard_D2s_v3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The sku tier (Standard, Basic)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,

    /// Number of instances in the scale set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}

impl Sku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-sku".into()),
            tier: Some("test-tier".into()),
            capacity: Some(100),
        }
    }
}

/// Describes an upgrade policy.
///
/// **Azure API**: `compute.v1.UpgradePolicy`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//UpgradePolicy>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradePolicy {
    /// Upgrade mode: Manual, Rolling, or Automatic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl UpgradePolicy {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            mode: Some("test-mode".into()),
        }
    }
}

/// Describes a virtual machine scale set VM profile.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetVMProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetVMProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetVMProfile {
    /// OS profile for VMSS VMs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<VirtualMachineScaleSetOsProfile>,

    /// Storage profile (reuses VM StorageProfile)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,

    /// Network profile for VMSS VMs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<VirtualMachineScaleSetNetworkProfile>,
}

impl VirtualMachineScaleSetVMProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            os_profile: Some(VirtualMachineScaleSetOsProfile::fixture()),
            storage_profile: Some(StorageProfile::fixture()),
            network_profile: Some(VirtualMachineScaleSetNetworkProfile::fixture()),
        }
    }
}

/// OS profile for VMSS VMs.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetOsProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetOsProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetOsProfile {
    /// Computer name prefix for VM instances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name_prefix: Option<String>,

    /// Admin username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,

    /// Admin password (write-only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,

    /// Linux-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<LinuxConfiguration>,

    /// Windows-specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
}

impl VirtualMachineScaleSetOsProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            computer_name_prefix: Some("test-computer_name_prefix".into()),
            admin_username: Some("test-admin_username".into()),
            admin_password: Some("test-admin_password".into()),
            linux_configuration: Some(LinuxConfiguration::fixture()),
            windows_configuration: Some(WindowsConfiguration::fixture()),
        }
    }
}

/// Network profile for a VMSS.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetNetworkProfile`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetNetworkProfile>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetNetworkProfile {
    /// Network interface configurations
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_interface_configurations: Vec<VirtualMachineScaleSetNetworkConfiguration>,
}

impl VirtualMachineScaleSetNetworkProfile {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            network_interface_configurations: vec![],
        }
    }
}

/// Network configuration for VMSS instances.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetNetworkConfiguration`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetNetworkConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetNetworkConfiguration {
    /// Network configuration name
    pub name: String,

    /// Network configuration properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetNetworkConfigurationProperties>,
}

impl VirtualMachineScaleSetNetworkConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-virtual_machine_scale_set_network_configuration".into(),
            properties: Some(VirtualMachineScaleSetNetworkConfigurationProperties::fixture()),
        }
    }
}

/// Properties of a VMSS network configuration.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetNetworkConfigurationProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetNetworkConfigurationProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetNetworkConfigurationProperties {
    /// Whether this is the primary NIC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,

    /// Whether IP forwarding is enabled
    #[serde(rename = "enableIPForwarding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ip_forwarding: Option<bool>,

    /// IP configurations
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<VirtualMachineScaleSetIPConfiguration>,
}

impl VirtualMachineScaleSetNetworkConfigurationProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            primary: Some(false),
            enable_ip_forwarding: Some(false),
            ip_configurations: vec![],
        }
    }
}

/// IP configuration for a VMSS NIC.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetIPConfiguration`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetIPConfiguration>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetIPConfiguration {
    /// IP configuration name
    pub name: String,

    /// IP configuration properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetIPConfigurationProperties>,
}

impl VirtualMachineScaleSetIPConfiguration {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: "test-virtual_machine_scale_set_ip_configuration".into(),
            properties: Some(VirtualMachineScaleSetIPConfigurationProperties::fixture()),
        }
    }
}

/// Properties of a VMSS IP configuration.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetIPConfigurationProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetIPConfigurationProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetIPConfigurationProperties {
    /// Subnet reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
}

impl VirtualMachineScaleSetIPConfigurationProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            subnet: Some(SubResource::fixture()),
        }
    }
}

/// Generic ARM sub-resource reference with ID.
///
/// **Azure API**: `compute.v1.SubResource`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//SubResource>
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

/// Properties of a virtual machine scale set.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetProperties {
    /// Provisioning state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Unique ID of the scale set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Whether to overprovision VMs during scaling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overprovision: Option<bool>,

    /// Limits to single placement group (max 100 VMs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_placement_group: Option<bool>,

    /// Fault domain count per placement group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain_count: Option<i32>,

    /// Upgrade policy configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_policy: Option<UpgradePolicy>,

    /// VM profile template for instances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machine_profile: Option<VirtualMachineScaleSetVMProfile>,
}

impl VirtualMachineScaleSetProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provisioning_state: Some("test-provisioning_state".into()),
            unique_id: Some("test-unique_id".into()),
            overprovision: Some(false),
            single_placement_group: Some(false),
            platform_fault_domain_count: Some(100),
            upgrade_policy: Some(UpgradePolicy::fixture()),
            virtual_machine_profile: Some(VirtualMachineScaleSetVMProfile::fixture()),
        }
    }
}

/// An Azure virtual machine scale set resource.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSet`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSet>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSet {
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

    /// VMSS SKU (VM size + capacity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,

    /// VMSS properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetProperties>,
}

impl VirtualMachineScaleSet {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-virtual_machine_scale_set".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            tags: Default::default(),
            sku: Some(Sku::fixture()),
            properties: Some(VirtualMachineScaleSetProperties::fixture()),
        }
    }
}

/// Response for listing virtual machine scale sets.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetListResult {
    /// List of virtual machine scale sets
    #[serde(default)]
    pub value: Vec<VirtualMachineScaleSet>,

    /// URL to get the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl VirtualMachineScaleSetListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a virtual machine scale set.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// VMSS SKU (VM size + capacity)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,

    /// VMSS properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetProperties>,
}

impl VirtualMachineScaleSetCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            sku: Some(Sku::fixture()),
            properties: Some(VirtualMachineScaleSetProperties::fixture()),
        }
    }
}

/// A virtual machine in a VM scale set.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetVM`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetVM>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetVM {
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

    /// Instance ID within the scale set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,

    /// VM SKU information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,

    /// VMSS VM properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetVMProperties>,
}

impl VirtualMachineScaleSetVM {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-virtual_machine_scale_set_vm".into()),
            r#type: Some("test-type".into()),
            location: Some("test-location".into()),
            instance_id: Some("test-instance_id".into()),
            sku: Some(Sku::fixture()),
            properties: Some(VirtualMachineScaleSetVMProperties::fixture()),
        }
    }
}

/// Properties of a VMSS VM instance.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetVMProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetVMProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetVMProperties {
    /// Whether the latest VMSS model is applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_model_applied: Option<bool>,

    /// Unique VM identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,

    /// Provisioning state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// Hardware profile (VM size)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,

    /// Storage profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,

    /// OS profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,

    /// Network profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
}

impl VirtualMachineScaleSetVMProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            latest_model_applied: Some(false),
            vm_id: Some("test-vm_id".into()),
            provisioning_state: Some("test-provisioning_state".into()),
            hardware_profile: Some(HardwareProfile::fixture()),
            storage_profile: Some(StorageProfile::fixture()),
            os_profile: Some(OsProfile::fixture()),
            network_profile: Some(NetworkProfile::fixture()),
        }
    }
}

/// Response for listing VMSS VM instances.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetVMListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetVMListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetVMListResult {
    /// List of VMSS VM instances
    #[serde(default)]
    pub value: Vec<VirtualMachineScaleSetVM>,

    /// URL to get the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl VirtualMachineScaleSetVMListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Instance IDs for VMSS instance operations.
///
/// **Azure API**: `compute.v1.VirtualMachineScaleSetVMInstanceIDs`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//VirtualMachineScaleSetVMInstanceIDs>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachineScaleSetVMInstanceIDs {
    /// VM instance IDs (omit to target all instances)
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_ids: Vec<String>,
}

impl VirtualMachineScaleSetVMInstanceIDs {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            instance_ids: vec![],
        }
    }
}

/// The disks sku name.
///
/// **Azure API**: `compute.v1.DiskSku`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DiskSku>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskSku {
    /// The sku name (Standard_LRS, Premium_LRS, StandardSSD_LRS, UltraSSD_LRS, Premium_ZRS,
    /// StandardSSD_ZRS, PremiumV2_LRS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The sku tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl DiskSku {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            name: Some("test-disk_sku".into()),
            tier: Some("test-tier".into()),
        }
    }
}

/// Data used when creating a disk.
///
/// **Azure API**: `compute.v1.DiskCreationData`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DiskCreationData>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskCreationData {
    /// Disk source type: Empty, Copy, Import, Restore, FromImage, etc.
    pub create_option: String,

    /// Storage account ID (required for Import)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,

    /// URI of source VHD (for Import)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,

    /// ARM ID of source snapshot or disk (for Copy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,

    /// Disk image reference (for FromImage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageDiskReference>,

    /// Gallery image reference (for FromImage)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<ImageDiskReference>,
}

impl DiskCreationData {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            create_option: "test-create_option".into(),
            storage_account_id: Some("test-storage_account_id".into()),
            source_uri: Some("test-source_uri".into()),
            source_resource_id: Some("test-source_resource_id".into()),
            image_reference: Some(ImageDiskReference::fixture()),
            gallery_image_reference: Some(ImageDiskReference::fixture()),
        }
    }
}

/// The source image used for creating the disk.
///
/// **Azure API**: `compute.v1.ImageDiskReference`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//ImageDiskReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageDiskReference {
    /// ARM ID of the image or gallery image version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// If the disk is created from an image's data disk, this is an index into the data disk
    /// array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}

impl ImageDiskReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            lun: Some(100),
        }
    }
}

/// Disk resource properties.
///
/// **Azure API**: `compute.v1.DiskProperties`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DiskProperties>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskProperties {
    /// The disk provisioning state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,

    /// The state of the disk (Unattached, Attached, Reserved, Frozen, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<String>,

    /// Disk size in GB
    #[serde(rename = "diskSizeGB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,

    /// The size of the disk in bytes
    #[serde(rename = "diskSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_bytes: Option<i64>,

    /// The number of IOPS allowed for this disk (UltraSSD only)
    #[serde(rename = "diskIOPSReadWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,

    /// The bandwidth allowed for this disk in MB per second (UltraSSD only)
    #[serde(rename = "diskMBpsReadWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,

    /// The operating system type (Windows or Linux)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,

    /// The hypervisor generation of the VM (V1 or V2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<String>,

    /// Disk source information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_data: Option<DiskCreationData>,

    /// Encryption settings collection for Azure Disk Encryption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,

    /// Encryption property to encrypt data using customer managed or platform managed keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,

    /// Policy for accessing the disk via network (AllowAll, AllowPrivate, DenyAll)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<String>,

    /// Policy for controlling export on the disk (Enabled, Disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<String>,

    /// The time when the disk was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,

    /// Unique GUID identifying the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl DiskProperties {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            provisioning_state: Some("test-provisioning_state".into()),
            disk_state: Some("test-disk_state".into()),
            disk_size_gb: Some(100),
            disk_size_bytes: Some(100),
            disk_iops_read_write: Some(100),
            disk_m_bps_read_write: Some(100),
            os_type: Some("test-os_type".into()),
            hyper_v_generation: Some("test-hyper_v_generation".into()),
            creation_data: Some(DiskCreationData::fixture()),
            encryption_settings_collection: Some(EncryptionSettingsCollection::fixture()),
            encryption: Some(Encryption::fixture()),
            network_access_policy: Some("test-network_access_policy".into()),
            public_network_access: Some("test-public_network_access".into()),
            time_created: Some("test-time_created".into()),
            unique_id: Some("test-unique_id".into()),
        }
    }
}

/// Encryption settings for disk or snapshot.
///
/// **Azure API**: `compute.v1.EncryptionSettingsCollection`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//EncryptionSettingsCollection>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionSettingsCollection {
    /// Whether encryption settings are enabled
    #[serde(default)]
    pub enabled: bool,

    /// A collection of encryption settings, one for each disk volume
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encryption_settings: Vec<EncryptionSettingsElement>,
}

impl EncryptionSettingsCollection {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            enabled: false,
            encryption_settings: vec![],
        }
    }
}

/// Encryption settings for one disk volume.
///
/// **Azure API**: `compute.v1.EncryptionSettingsElement`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//EncryptionSettingsElement>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionSettingsElement {
    /// Key Vault Secret URL and vault id of the disk encryption key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<KeyVaultAndSecretReference>,

    /// Key Vault Key URL and vault id of the key encryption key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultAndKeyReference>,
}

impl EncryptionSettingsElement {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            disk_encryption_key: Some(KeyVaultAndSecretReference::fixture()),
            key_encryption_key: Some(KeyVaultAndKeyReference::fixture()),
        }
    }
}

/// Key Vault Secret URL and vault id of the encryption key.
///
/// **Azure API**: `compute.v1.KeyVaultAndSecretReference`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//KeyVaultAndSecretReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyVaultAndSecretReference {
    /// Resource id of the KeyVault containing the key or secret
    pub source_vault: SubResource,

    /// Url pointing to a key or secret in KeyVault
    pub secret_url: String,
}

impl KeyVaultAndSecretReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            source_vault: SubResource::fixture(),
            secret_url: "test-secret_url".into(),
        }
    }
}

/// Key Vault Key URL and vault id of the encryption key.
///
/// **Azure API**: `compute.v1.KeyVaultAndKeyReference`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//KeyVaultAndKeyReference>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyVaultAndKeyReference {
    /// Resource id of the KeyVault containing the key or secret
    pub source_vault: SubResource,

    /// Url pointing to a key or secret in KeyVault
    pub key_url: String,
}

impl KeyVaultAndKeyReference {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            source_vault: SubResource::fixture(),
            key_url: "test-key_url".into(),
        }
    }
}

/// Encryption at rest settings for disk or snapshot.
///
/// **Azure API**: `compute.v1.Encryption`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//Encryption>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    /// ResourceId of the disk encryption set to use for enabling encryption at rest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,

    /// The type of key used to encrypt the data of the disk (EncryptionAtRestWithPlatformKey,
    /// EncryptionAtRestWithCustomerKey, etc.)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl Encryption {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            disk_encryption_set_id: Some("test-disk_encryption_set_id".into()),
            r#type: Some("test-type".into()),
        }
    }
}

/// Disk resource.
///
/// **Azure API**: `compute.v1.Disk`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//Disk>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disk {
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
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The logical zone list for Disk
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,

    /// The disks sku name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,

    /// Disk resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}

impl Disk {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            id: Some("test-id".into()),
            name: Some("test-disk".into()),
            r#type: Some("test-type".into()),
            location: "test-location".into(),
            tags: Default::default(),
            zones: vec![],
            sku: Some(DiskSku::fixture()),
            properties: Some(DiskProperties::fixture()),
        }
    }
}

/// The List Disks operation response.
///
/// **Azure API**: `compute.v1.DiskListResult`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DiskListResult>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskListResult {
    /// A list of disks
    #[serde(default)]
    pub value: Vec<Disk>,

    /// The uri to fetch the next page of disks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl DiskListResult {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            value: vec![],
            next_link: Some("test-next_link".into()),
        }
    }
}

/// Request body for creating or updating a disk.
///
/// **Azure API**: `compute.v1.DiskCreateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DiskCreateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskCreateRequest {
    /// Resource location
    pub location: String,

    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The logical zone list for Disk
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,

    /// The disks sku name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,

    /// Disk resource properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}

impl DiskCreateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            location: "test-location".into(),
            tags: Default::default(),
            zones: vec![],
            sku: Some(DiskSku::fixture()),
            properties: Some(DiskProperties::fixture()),
        }
    }
}

/// Request body for updating a disk (PATCH).
///
/// **Azure API**: `compute.v1.DiskUpdateRequest`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//DiskUpdateRequest>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskUpdateRequest {
    /// Resource tags
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,

    /// The disk SKU (name and tier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
}

impl DiskUpdateRequest {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            tags: Default::default(),
            sku: Some(DiskSku::fixture()),
        }
    }
}

/// Data used for requesting a SAS URI for a managed disk or snapshot.
///
/// **Azure API**: `compute.v1.GrantAccessData`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//GrantAccessData>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantAccessData {
    /// The Access Level (None, Read, Write)
    pub access: String,

    /// Time duration in seconds until the SAS access expires
    pub duration_in_seconds: i32,

    /// Used to specify the file format when downloading a VHD snapshot (VHDX or VHD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
}

impl GrantAccessData {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            access: "test-access".into(),
            duration_in_seconds: 100,
            file_format: Some("test-file_format".into()),
        }
    }
}

/// A disk access SAS uri.
///
/// **Azure API**: `compute.v1.AccessUri`
/// **Reference**: <https://learn.microsoft.com/en-us/rest/api/compute//AccessUri>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessUri {
    /// A SAS uri for accessing a disk
    #[serde(rename = "accessSAS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_sas: Option<String>,

    /// A SAS uri for accessing a VM guest state
    #[serde(rename = "securityDataAccessSAS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_data_access_sas: Option<String>,
}

impl AccessUri {
    #[cfg(any(test, feature = "test-support"))]
    /// Create a fixture instance for testing.
    pub fn fixture() -> Self {
        Self {
            access_sas: Some("test-access_sas".into()),
            security_data_access_sas: Some("test-security_data_access_sas".into()),
        }
    }
}
