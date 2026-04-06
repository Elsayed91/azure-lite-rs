//! Azure Compute API client.
//!
//! Thin wrapper over generated ops. All URL construction and HTTP methods
//! are in `ops::compute::ComputeOps`. This layer adds:
//! - Ergonomic method signatures (auto-injects subscription_id from client)

use crate::{
    AzureHttpClient, Result,
    ops::compute::ComputeOps,
    types::compute::{
        AccessUri, Disk, DiskCreateRequest, DiskListResult, DiskSku, DiskUpdateRequest,
        GrantAccessData, VirtualMachine, VirtualMachineCreateRequest,
        VirtualMachineInstanceViewResult, VirtualMachineListResult, VirtualMachineScaleSet,
        VirtualMachineScaleSetCreateRequest, VirtualMachineScaleSetListResult,
        VirtualMachineScaleSetVMInstanceIDs, VirtualMachineScaleSetVMListResult,
    },
};

/// Client for the Azure Compute API.
///
/// Wraps the raw [`ComputeOps`] with ergonomic signatures that
/// auto-inject `subscription_id` from the parent [`AzureHttpClient`].
pub struct ComputeClient<'a> {
    ops: ComputeOps<'a>,
    client: &'a AzureHttpClient,
}

impl<'a> ComputeClient<'a> {
    /// Create a new Azure Compute API client.
    pub(crate) fn new(client: &'a AzureHttpClient) -> Self {
        Self {
            ops: ComputeOps::new(client),
            client,
        }
    }

    /// List virtual machines in a resource group.
    pub async fn list_vms(&self, resource_group_name: &str) -> Result<VirtualMachineListResult> {
        self.ops
            .list_vms(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Get a virtual machine.
    pub async fn get_vm(&self, resource_group_name: &str, vm_name: &str) -> Result<VirtualMachine> {
        self.ops
            .get_vm(
                self.client.subscription_id(),
                resource_group_name,
                vm_name,
                "",
            )
            .await
    }

    /// Get a virtual machine with expanded properties.
    pub async fn get_vm_expanded(
        &self,
        resource_group_name: &str,
        vm_name: &str,
        expand: &str,
    ) -> Result<VirtualMachine> {
        self.ops
            .get_vm(
                self.client.subscription_id(),
                resource_group_name,
                vm_name,
                expand,
            )
            .await
    }

    /// Create or update a virtual machine.
    pub async fn create_vm(
        &self,
        resource_group_name: &str,
        vm_name: &str,
        body: &VirtualMachineCreateRequest,
    ) -> Result<VirtualMachine> {
        self.ops
            .create_vm(
                self.client.subscription_id(),
                resource_group_name,
                vm_name,
                body,
            )
            .await
    }

    /// Delete a virtual machine.
    pub async fn delete_vm(&self, resource_group_name: &str, vm_name: &str) -> Result<()> {
        self.ops
            .delete_vm(self.client.subscription_id(), resource_group_name, vm_name)
            .await
    }

    /// Start a virtual machine.
    pub async fn start_vm(&self, resource_group_name: &str, vm_name: &str) -> Result<()> {
        self.ops
            .start_vm(self.client.subscription_id(), resource_group_name, vm_name)
            .await
    }

    /// Power off (stop) a virtual machine. The VM continues to be billed.
    pub async fn stop_vm(&self, resource_group_name: &str, vm_name: &str) -> Result<()> {
        self.ops
            .stop_vm(self.client.subscription_id(), resource_group_name, vm_name)
            .await
    }

    /// Deallocate a virtual machine. Stops billing.
    pub async fn deallocate_vm(&self, resource_group_name: &str, vm_name: &str) -> Result<()> {
        self.ops
            .deallocate_vm(self.client.subscription_id(), resource_group_name, vm_name)
            .await
    }

    /// Restart a virtual machine.
    pub async fn restart_vm(&self, resource_group_name: &str, vm_name: &str) -> Result<()> {
        self.ops
            .restart_vm(self.client.subscription_id(), resource_group_name, vm_name)
            .await
    }

    /// Get the instance view of a virtual machine.
    pub async fn get_instance_view(
        &self,
        resource_group_name: &str,
        vm_name: &str,
    ) -> Result<VirtualMachineInstanceViewResult> {
        self.ops
            .get_instance_view(self.client.subscription_id(), resource_group_name, vm_name)
            .await
    }

    // --- VMSS operations ---

    /// List virtual machine scale sets in a resource group.
    pub async fn list_vmss(
        &self,
        resource_group_name: &str,
    ) -> Result<VirtualMachineScaleSetListResult> {
        self.ops
            .list_vmss(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// Get a virtual machine scale set.
    pub async fn get_vmss(
        &self,
        resource_group_name: &str,
        vmss_name: &str,
    ) -> Result<VirtualMachineScaleSet> {
        self.ops
            .get_vmss(
                self.client.subscription_id(),
                resource_group_name,
                vmss_name,
            )
            .await
    }

    /// Create or update a virtual machine scale set.
    pub async fn create_vmss(
        &self,
        resource_group_name: &str,
        vmss_name: &str,
        body: &VirtualMachineScaleSetCreateRequest,
    ) -> Result<VirtualMachineScaleSet> {
        self.ops
            .create_vmss(
                self.client.subscription_id(),
                resource_group_name,
                vmss_name,
                body,
            )
            .await
    }

    /// Delete a virtual machine scale set.
    pub async fn delete_vmss(&self, resource_group_name: &str, vmss_name: &str) -> Result<()> {
        self.ops
            .delete_vmss(
                self.client.subscription_id(),
                resource_group_name,
                vmss_name,
            )
            .await
    }

    /// List virtual machines in a VM scale set.
    pub async fn list_vmss_instances(
        &self,
        resource_group_name: &str,
        vmss_name: &str,
    ) -> Result<VirtualMachineScaleSetVMListResult> {
        self.ops
            .list_vmss_instances(
                self.client.subscription_id(),
                resource_group_name,
                vmss_name,
            )
            .await
    }

    /// Start one or more virtual machines in a VM scale set.
    pub async fn start_vmss_instances(
        &self,
        resource_group_name: &str,
        vmss_name: &str,
        instance_ids: &VirtualMachineScaleSetVMInstanceIDs,
    ) -> Result<()> {
        self.ops
            .start_vmss_instances(
                self.client.subscription_id(),
                resource_group_name,
                vmss_name,
                instance_ids,
            )
            .await
    }

    /// Power off one or more virtual machines in a VM scale set.
    pub async fn stop_vmss_instances(
        &self,
        resource_group_name: &str,
        vmss_name: &str,
        instance_ids: &VirtualMachineScaleSetVMInstanceIDs,
    ) -> Result<()> {
        self.ops
            .stop_vmss_instances(
                self.client.subscription_id(),
                resource_group_name,
                vmss_name,
                instance_ids,
            )
            .await
    }

    // --- Managed Disk operations ---

    /// List all managed disks in a resource group.
    pub async fn list_disks(&self, resource_group_name: &str) -> Result<DiskListResult> {
        self.ops
            .list_disks(self.client.subscription_id(), resource_group_name)
            .await
    }

    /// List all managed disks in the subscription.
    pub async fn list_disks_in_subscription(&self) -> Result<DiskListResult> {
        self.ops
            .list_disks_in_subscription(self.client.subscription_id())
            .await
    }

    /// Get information about a managed disk.
    pub async fn get_disk(&self, resource_group_name: &str, disk_name: &str) -> Result<Disk> {
        self.ops
            .get_disk(
                self.client.subscription_id(),
                resource_group_name,
                disk_name,
            )
            .await
    }

    /// Create or update a managed disk.
    pub async fn create_disk(
        &self,
        resource_group_name: &str,
        disk_name: &str,
        body: &DiskCreateRequest,
    ) -> Result<Disk> {
        self.ops
            .create_disk(
                self.client.subscription_id(),
                resource_group_name,
                disk_name,
                body,
            )
            .await
    }

    /// Delete a managed disk.
    pub async fn delete_disk(&self, resource_group_name: &str, disk_name: &str) -> Result<()> {
        self.ops
            .delete_disk(
                self.client.subscription_id(),
                resource_group_name,
                disk_name,
            )
            .await
    }

    /// Change the SKU (performance tier) of a managed disk.
    ///
    /// The disk must be unattached OR the attached VM must be fully deallocated
    /// before this call will succeed. Returns an error if the VM is still running.
    ///
    /// # Arguments
    /// * `resource_group_name` - Resource group containing the disk
    /// * `disk_name` - Name of the disk to update
    /// * `sku_name` - Target SKU name: "StandardSSD_LRS", "StandardSSD_ZRS",
    ///   "Standard_LRS", "Premium_LRS", "Premium_ZRS"
    pub async fn update_disk_sku(
        &self,
        resource_group_name: &str,
        disk_name: &str,
        sku_name: &str,
    ) -> Result<Disk> {
        let body = DiskUpdateRequest {
            sku: Some(DiskSku {
                name: Some(sku_name.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        self.ops
            .update_disk(
                self.client.subscription_id(),
                resource_group_name,
                disk_name,
                &body,
            )
            .await
    }

    /// Delete a managed disk snapshot.
    pub async fn delete_snapshot(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        snapshot_name: &str,
    ) -> Result<()> {
        self.ops
            .delete_snapshot(subscription_id, resource_group_name, snapshot_name)
            .await
    }

    /// Grant SAS access to a managed disk.
    ///
    /// The Azure `beginGetAccess` endpoint returns HTTP 202 with a `Location` header
    /// pointing to an async operation URL. This method polls that URL until the
    /// operation completes and returns the SAS URI.
    pub async fn grant_access(
        &self,
        resource_group_name: &str,
        disk_name: &str,
        body: &GrantAccessData,
    ) -> Result<AccessUri> {
        let sub = self.client.subscription_id();
        let url = format!(
            "https://management.azure.com/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Compute/disks/{disk}/beginGetAccess?api-version=2024-07-01",
            rg = urlencoding::encode(resource_group_name),
            disk = urlencoding::encode(disk_name),
        );

        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize grant_access request: {e}"),
                body: None,
            })?;

        let response = self.client.post(&url, &body_bytes).await?;
        let location = response.location();
        let response_bytes = response.error_for_status().await?.bytes().await?;

        // If the response body is non-empty, parse it directly (rare synchronous response)
        if !response_bytes.is_empty() {
            return serde_json::from_slice(&response_bytes).map_err(|e| {
                crate::AzureError::InvalidResponse {
                    message: format!("Failed to parse grant_access response: {e}"),
                    body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
                }
            });
        }

        // 202 Accepted: poll the Location URL until the operation completes
        let poll_url = location.ok_or_else(|| crate::AzureError::InvalidResponse {
            message: "grant_access returned 202 with no Location header".into(),
            body: None,
        })?;

        for _ in 0..30 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let poll_response = self.client.get(&poll_url).await?;
            let poll_bytes = poll_response.error_for_status().await?.bytes().await?;
            if !poll_bytes.is_empty() {
                return serde_json::from_slice(&poll_bytes).map_err(|e| {
                    crate::AzureError::InvalidResponse {
                        message: format!("Failed to parse grant_access poll response: {e}"),
                        body: Some(String::from_utf8_lossy(&poll_bytes).to_string()),
                    }
                });
            }
        }

        Err(crate::AzureError::InvalidResponse {
            message: "grant_access polling timed out after 30 attempts".into(),
            body: None,
        })
    }

    /// Revoke SAS access to a managed disk.
    pub async fn revoke_access(&self, resource_group_name: &str, disk_name: &str) -> Result<()> {
        self.ops
            .revoke_access(
                self.client.subscription_id(),
                resource_group_name,
                disk_name,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::compute::VirtualMachineProperties;

    #[tokio::test]
    async fn list_vms_returns_empty_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Compute/virtualMachines")
            .returning_json(serde_json::json!({
                "value": []
            }));

        let client = AzureHttpClient::from_mock(mock);
        let compute = client.compute();
        let result = compute.list_vms("my-rg").await.expect("list_vms failed");
        assert!(result.value.is_empty());
        assert!(result.next_link.is_none());
    }

    #[tokio::test]
    async fn list_vms_returns_vm_with_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Compute/virtualMachines")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/my-rg/providers/Microsoft.Compute/virtualMachines/vm1",
                    "name": "vm1",
                    "type": "Microsoft.Compute/virtualMachines",
                    "location": "eastus",
                    "properties": {
                        "vmId": "abc-123",
                        "provisioningState": "Succeeded",
                        "hardwareProfile": { "vmSize": "Standard_B1s" }
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let compute = client.compute();
        let result = compute.list_vms("my-rg").await.expect("list_vms failed");
        assert_eq!(result.value.len(), 1);
        let vm = &result.value[0];
        assert_eq!(vm.name.as_deref(), Some("vm1"));
        assert_eq!(vm.location.as_deref(), Some("eastus"));
        assert_eq!(
            vm.properties.as_ref().and_then(|p| p.vm_id.as_deref()),
            Some("abc-123"),
        );
        assert_eq!(
            vm.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
    }

    #[tokio::test]
    async fn get_vm_injects_subscription_id() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/my-vm")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/my-vm",
                "name": "my-vm",
                "type": "Microsoft.Compute/virtualMachines",
                "location": "westus2",
                "properties": {
                    "vmId": "vm-uuid",
                    "provisioningState": "Succeeded"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let compute = client.compute();
        let vm = compute.get_vm("rg1", "my-vm").await.expect("get_vm failed");
        assert_eq!(vm.name.as_deref(), Some("my-vm"));
        assert_eq!(vm.location.as_deref(), Some("westus2"));
        assert_eq!(
            vm.properties.as_ref().and_then(|p| p.vm_id.as_deref()),
            Some("vm-uuid"),
        );
    }

    #[tokio::test]
    async fn create_vm_sends_put_and_returns_vm() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/new-vm")
            .returning_json(serde_json::json!({
                "name": "new-vm",
                "location": "eastus",
                "properties": {
                    "provisioningState": "Creating",
                    "hardwareProfile": { "vmSize": "Standard_B1s" }
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let compute = client.compute();
        let request = VirtualMachineCreateRequest {
            location: "eastus".into(),
            properties: Some(VirtualMachineProperties {
                hardware_profile: Some(crate::types::compute::HardwareProfile {
                    vm_size: Some("Standard_B1s".into()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let vm = compute
            .create_vm("rg1", "new-vm", &request)
            .await
            .expect("create_vm failed");
        assert_eq!(vm.name.as_deref(), Some("new-vm"));
        assert_eq!(
            vm.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Creating"),
        );
    }

    #[tokio::test]
    async fn delete_vm_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/dead-vm")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        let compute = client.compute();
        compute
            .delete_vm("rg1", "dead-vm")
            .await
            .expect("delete_vm failed");
    }

    #[tokio::test]
    async fn deallocate_vm_sends_post() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/my-vm/deallocate")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        let compute = client.compute();
        compute
            .deallocate_vm("rg1", "my-vm")
            .await
            .expect("deallocate_vm failed");
    }

    #[tokio::test]
    async fn get_instance_view_returns_statuses() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachines/my-vm/instanceView")
            .returning_json(serde_json::json!({
                "statuses": [
                    {
                        "code": "ProvisioningState/succeeded",
                        "displayStatus": "Provisioning succeeded",
                        "level": "Info"
                    },
                    {
                        "code": "PowerState/running",
                        "displayStatus": "VM running",
                        "level": "Info"
                    }
                ]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let compute = client.compute();
        let iv = compute
            .get_instance_view("rg1", "my-vm")
            .await
            .expect("get_instance_view failed");
        assert_eq!(iv.statuses.len(), 2);
        assert_eq!(
            iv.statuses[0].code.as_deref(),
            Some("ProvisioningState/succeeded")
        );
        assert_eq!(
            iv.statuses[0].display_status.as_deref(),
            Some("Provisioning succeeded")
        );
        assert_eq!(iv.statuses[1].code.as_deref(), Some("PowerState/running"));
    }

    // --- VMSS unit tests ---

    #[tokio::test]
    async fn list_vmss_returns_empty_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Compute/virtualMachineScaleSets")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .compute()
            .list_vmss("my-rg")
            .await
            .expect("list_vmss failed");
        assert!(result.value.is_empty());
        assert!(result.next_link.is_none());
    }

    #[tokio::test]
    async fn list_vmss_returns_vmss_with_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Compute/virtualMachineScaleSets")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/my-rg/providers/Microsoft.Compute/virtualMachineScaleSets/my-vmss",
                    "name": "my-vmss",
                    "type": "Microsoft.Compute/virtualMachineScaleSets",
                    "location": "eastus",
                    "sku": { "name": "Standard_B1s", "tier": "Standard", "capacity": 1 },
                    "properties": {
                        "provisioningState": "Succeeded",
                        "uniqueId": "abc-123"
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .compute()
            .list_vmss("my-rg")
            .await
            .expect("list_vmss failed");
        assert_eq!(result.value.len(), 1);
        let vmss = &result.value[0];
        assert_eq!(vmss.name.as_deref(), Some("my-vmss"));
        assert_eq!(vmss.location.as_deref(), Some("eastus"));
        assert_eq!(
            vmss.sku.as_ref().and_then(|s| s.name.as_deref()),
            Some("Standard_B1s")
        );
        assert_eq!(vmss.sku.as_ref().and_then(|s| s.capacity), Some(1));
        assert_eq!(
            vmss.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
        assert_eq!(
            vmss.properties
                .as_ref()
                .and_then(|p| p.unique_id.as_deref()),
            Some("abc-123"),
        );
    }

    #[tokio::test]
    async fn get_vmss_injects_subscription_id() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachineScaleSets/my-vmss")
            .returning_json(serde_json::json!({
                "name": "my-vmss",
                "type": "Microsoft.Compute/virtualMachineScaleSets",
                "location": "eastus",
                "sku": { "name": "Standard_B1s", "tier": "Standard", "capacity": 2 },
                "properties": {
                    "provisioningState": "Succeeded",
                    "uniqueId": "vmss-uuid"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let vmss = client
            .compute()
            .get_vmss("rg1", "my-vmss")
            .await
            .expect("get_vmss failed");
        assert_eq!(vmss.name.as_deref(), Some("my-vmss"));
        assert_eq!(vmss.sku.as_ref().and_then(|s| s.capacity), Some(2));
        assert_eq!(
            vmss.properties
                .as_ref()
                .and_then(|p| p.unique_id.as_deref()),
            Some("vmss-uuid"),
        );
    }

    #[tokio::test]
    async fn create_vmss_sends_put_and_returns_vmss() {
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachineScaleSets/new-vmss")
            .returning_json(serde_json::json!({
                "name": "new-vmss",
                "location": "eastus",
                "sku": { "name": "Standard_B1s", "tier": "Standard", "capacity": 1 },
                "properties": {
                    "provisioningState": "Creating"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let request = VirtualMachineScaleSetCreateRequest {
            location: "eastus".into(),
            sku: Some(crate::types::compute::Sku {
                name: Some("Standard_B1s".into()),
                capacity: Some(1),
                ..Default::default()
            }),
            ..Default::default()
        };
        let vmss = client
            .compute()
            .create_vmss("rg1", "new-vmss", &request)
            .await
            .expect("create_vmss failed");
        assert_eq!(vmss.name.as_deref(), Some("new-vmss"));
        assert_eq!(
            vmss.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Creating"),
        );
    }

    #[tokio::test]
    async fn delete_vmss_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachineScaleSets/dead-vmss")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .compute()
            .delete_vmss("rg1", "dead-vmss")
            .await
            .expect("delete_vmss failed");
    }

    #[tokio::test]
    async fn list_vmss_instances_returns_instances() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachineScaleSets/my-vmss/virtualMachines")
            .returning_json(serde_json::json!({
                "value": [{
                    "name": "my-vmss_0",
                    "instanceId": "0",
                    "location": "eastus",
                    "properties": {
                        "latestModelApplied": true,
                        "provisioningState": "Succeeded",
                        "vmId": "instance-uuid"
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .compute()
            .list_vmss_instances("rg1", "my-vmss")
            .await
            .expect("list_vmss_instances failed");
        assert_eq!(result.value.len(), 1);
        let inst = &result.value[0];
        assert_eq!(inst.name.as_deref(), Some("my-vmss_0"));
        assert_eq!(inst.instance_id.as_deref(), Some("0"));
        assert_eq!(
            inst.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
        assert_eq!(
            inst.properties
                .as_ref()
                .and_then(|p| p.latest_model_applied),
            Some(true),
        );
    }

    #[tokio::test]
    async fn stop_vmss_instances_sends_post() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachineScaleSets/my-vmss/poweroff")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        let ids = VirtualMachineScaleSetVMInstanceIDs {
            instance_ids: vec!["0".into()],
        };
        client
            .compute()
            .stop_vmss_instances("rg1", "my-vmss", &ids)
            .await
            .expect("stop_vmss_instances failed");
    }

    #[tokio::test]
    async fn start_vmss_instances_sends_post() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/virtualMachineScaleSets/my-vmss/start")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        let ids = VirtualMachineScaleSetVMInstanceIDs {
            instance_ids: vec!["0".into(), "1".into()],
        };
        client
            .compute()
            .start_vmss_instances("rg1", "my-vmss", &ids)
            .await
            .expect("start_vmss_instances failed");
    }

    // --- Managed Disk unit tests ---

    #[tokio::test]
    async fn list_disks_returns_empty_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Compute/disks")
            .returning_json(serde_json::json!({ "value": [] }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .compute()
            .list_disks("my-rg")
            .await
            .expect("list_disks failed");
        assert!(result.value.is_empty());
        assert!(result.next_link.is_none());
    }

    #[tokio::test]
    async fn list_disks_returns_disk_with_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/my-rg/providers/Microsoft.Compute/disks")
            .returning_json(serde_json::json!({
                "value": [{
                    "id": "/subscriptions/sub/resourceGroups/my-rg/providers/Microsoft.Compute/disks/my-disk",
                    "name": "my-disk",
                    "type": "Microsoft.Compute/disks",
                    "location": "eastus",
                    "sku": { "name": "Standard_LRS" },
                    "properties": {
                        "diskSizeGB": 32,
                        "diskState": "Unattached",
                        "provisioningState": "Succeeded"
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .compute()
            .list_disks("my-rg")
            .await
            .expect("list_disks failed");
        assert_eq!(result.value.len(), 1);
        let disk = &result.value[0];
        assert_eq!(disk.name.as_deref(), Some("my-disk"));
        assert_eq!(disk.location.as_str(), "eastus");
        assert_eq!(
            disk.sku.as_ref().and_then(|s| s.name.as_deref()),
            Some("Standard_LRS")
        );
        assert_eq!(
            disk.properties.as_ref().and_then(|p| p.disk_size_gb),
            Some(32)
        );
        assert_eq!(
            disk.properties
                .as_ref()
                .and_then(|p| p.disk_state.as_deref()),
            Some("Unattached"),
        );
    }

    #[tokio::test]
    async fn list_disks_in_subscription_returns_list() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/providers/Microsoft.Compute/disks")
            .returning_json(serde_json::json!({
                "value": [{
                    "name": "disk-a",
                    "location": "eastus",
                    "properties": {
                        "diskSizeGB": 64,
                        "diskState": "Unattached"
                    }
                }]
            }));

        let client = AzureHttpClient::from_mock(mock);
        let result = client
            .compute()
            .list_disks_in_subscription()
            .await
            .expect("list_disks_in_subscription failed");
        assert_eq!(result.value.len(), 1);
        assert_eq!(result.value[0].name.as_deref(), Some("disk-a"));
    }

    #[tokio::test]
    async fn get_disk_returns_disk_fields() {
        let mut mock = crate::MockClient::new();
        mock.expect_get("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/disks/my-disk")
            .returning_json(serde_json::json!({
                "id": "/subscriptions/sub/resourceGroups/rg1/providers/Microsoft.Compute/disks/my-disk",
                "name": "my-disk",
                "location": "westus2",
                "sku": { "name": "Premium_LRS", "tier": "Premium" },
                "properties": {
                    "diskSizeGB": 128,
                    "diskState": "Unattached",
                    "provisioningState": "Succeeded",
                    "uniqueId": "disk-uuid-abc"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let disk = client
            .compute()
            .get_disk("rg1", "my-disk")
            .await
            .expect("get_disk failed");
        assert_eq!(disk.name.as_deref(), Some("my-disk"));
        assert_eq!(disk.location.as_str(), "westus2");
        assert_eq!(
            disk.sku.as_ref().and_then(|s| s.name.as_deref()),
            Some("Premium_LRS")
        );
        assert_eq!(
            disk.properties.as_ref().and_then(|p| p.disk_size_gb),
            Some(128)
        );
        assert_eq!(
            disk.properties
                .as_ref()
                .and_then(|p| p.unique_id.as_deref()),
            Some("disk-uuid-abc"),
        );
    }

    #[tokio::test]
    async fn create_disk_sends_put_and_returns_disk() {
        use crate::types::compute::{DiskCreateRequest, DiskCreationData, DiskProperties, DiskSku};
        let mut mock = crate::MockClient::new();
        mock.expect_put("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/disks/new-disk")
            .returning_json(serde_json::json!({
                "name": "new-disk",
                "location": "eastus",
                "sku": { "name": "Standard_LRS" },
                "properties": {
                    "diskSizeGB": 4,
                    "diskState": "Unattached",
                    "provisioningState": "Updating"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let request = DiskCreateRequest {
            location: "eastus".into(),
            sku: Some(DiskSku {
                name: Some("Standard_LRS".into()),
                ..Default::default()
            }),
            properties: Some(DiskProperties {
                disk_size_gb: Some(4),
                creation_data: Some(DiskCreationData {
                    create_option: "Empty".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let disk = client
            .compute()
            .create_disk("rg1", "new-disk", &request)
            .await
            .expect("create_disk failed");
        assert_eq!(disk.name.as_deref(), Some("new-disk"));
        assert_eq!(
            disk.properties.as_ref().and_then(|p| p.disk_size_gb),
            Some(4)
        );
        assert_eq!(
            disk.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Updating"),
        );
    }

    #[tokio::test]
    async fn delete_disk_sends_delete() {
        let mut mock = crate::MockClient::new();
        mock.expect_delete("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/disks/old-disk")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .compute()
            .delete_disk("rg1", "old-disk")
            .await
            .expect("delete_disk failed");
    }

    #[tokio::test]
    async fn update_disk_sku_sends_patch_and_returns_disk() {
        let mut mock = crate::MockClient::new();
        mock.expect_patch("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/disks/my-disk")
            .returning_json(serde_json::json!({
                "name": "my-disk",
                "location": "eastus",
                "sku": { "name": "StandardSSD_LRS", "tier": "StandardSSD" },
                "properties": {
                    "diskSizeGB": 128,
                    "diskState": "Unattached",
                    "provisioningState": "Succeeded"
                }
            }));

        let client = AzureHttpClient::from_mock(mock);
        let disk = client
            .compute()
            .update_disk_sku("rg1", "my-disk", "StandardSSD_LRS")
            .await
            .expect("update_disk_sku failed");
        assert_eq!(disk.name.as_deref(), Some("my-disk"));
        assert_eq!(
            disk.sku.as_ref().and_then(|s| s.name.as_deref()),
            Some("StandardSSD_LRS")
        );
        assert_eq!(
            disk.properties.as_ref().and_then(|p| p.disk_size_gb),
            Some(128)
        );
        assert_eq!(
            disk.properties
                .as_ref()
                .and_then(|p| p.disk_state.as_deref()),
            Some("Unattached"),
        );
        assert_eq!(
            disk.properties
                .as_ref()
                .and_then(|p| p.provisioning_state.as_deref()),
            Some("Succeeded"),
        );
    }

    #[tokio::test]
    async fn revoke_access_sends_post() {
        let mut mock = crate::MockClient::new();
        mock.expect_post("/subscriptions/test-subscription-id/resourceGroups/rg1/providers/Microsoft.Compute/disks/my-disk/endGetAccess")
            .returning_json(serde_json::json!(null));

        let client = AzureHttpClient::from_mock(mock);
        client
            .compute()
            .revoke_access("rg1", "my-disk")
            .await
            .expect("revoke_access failed");
    }
}
