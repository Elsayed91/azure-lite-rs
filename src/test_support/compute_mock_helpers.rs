//! MockClient helpers for Azure Compute API.
//!
//! Auto-generated extension methods for ergonomic test setup.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[cfg(any(test, feature = "test-support"))]
use crate::mock_client::{ExpectationBuilder, MockClient};

/// Extension trait for MockClient with Azure Compute helpers.
#[cfg(any(test, feature = "test-support"))]
pub trait ComputeMockHelpers {
    /// Helper to expect `list_vms`: List virtual machines in a resource group.
    fn expect_list_vms(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_vm`: Get a virtual machine.
    fn expect_get_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
        expand: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_vm`: Create or update a virtual machine.
    fn expect_create_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_vm`: Delete a virtual machine.
    fn expect_delete_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `start_vm`: Start a virtual machine.
    fn expect_start_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `stop_vm`: Power off (stop) a virtual machine. The VM continues to be
    /// billed.
    fn expect_stop_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `deallocate_vm`: Deallocate a virtual machine. Stops billing.
    fn expect_deallocate_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `restart_vm`: Restart a virtual machine.
    fn expect_restart_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_instance_view`: Get the instance view of a virtual machine.
    fn expect_get_instance_view(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_vmss`: List virtual machine scale sets in a resource group.
    fn expect_list_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_vmss`: Get a virtual machine scale set.
    fn expect_get_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_vmss`: Create or update a virtual machine scale set.
    fn expect_create_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_vmss`: Delete a virtual machine scale set.
    fn expect_delete_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_vmss_instances`: List virtual machines in a VM scale set.
    fn expect_list_vmss_instances(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `start_vmss_instances`: Start one or more virtual machines in a VM scale
    /// set.
    fn expect_start_vmss_instances(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `stop_vmss_instances`: Power off one or more virtual machines in a VM scale
    /// set.
    fn expect_stop_vmss_instances(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_disks`: Lists all the disks under a resource group.
    fn expect_list_disks(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `list_disks_in_subscription`: Lists all the disks under a subscription.
    fn expect_list_disks_in_subscription(
        &mut self,
        subscription_id: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `get_disk`: Gets information about a disk.
    fn expect_get_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `create_disk`: Creates or updates a disk.
    fn expect_create_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_disk`: Deletes a disk.
    fn expect_delete_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `update_disk`: Updates (patches) a disk.
    fn expect_update_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `delete_snapshot`: Deletes a snapshot.
    fn expect_delete_snapshot(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        snapshot_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `grant_access`: Grants access to a disk.
    fn expect_grant_access(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> ExpectationBuilder<'_>;

    /// Helper to expect `revoke_access`: Revokes access to a disk.
    fn expect_revoke_access(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> ExpectationBuilder<'_>;
}

#[cfg(any(test, feature = "test-support"))]
impl ComputeMockHelpers for MockClient {
    /// Helper to expect `list_vms`: List virtual machines in a resource group.
    fn expect_list_vms(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_vm`: Get a virtual machine.
    fn expect_get_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
        expand: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let mut path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}"
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

    /// Helper to expect `create_vm`: Create or update a virtual machine.
    fn expect_create_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_vm`: Delete a virtual machine.
    fn expect_delete_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `start_vm`: Start a virtual machine.
    fn expect_start_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}/start"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `stop_vm`: Power off (stop) a virtual machine. The VM continues to be
    /// billed.
    fn expect_stop_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}/powerOff"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `deallocate_vm`: Deallocate a virtual machine. Stops billing.
    fn expect_deallocate_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}/deallocate"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `restart_vm`: Restart a virtual machine.
    fn expect_restart_vm(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}/restart"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `get_instance_view`: Get the instance view of a virtual machine.
    fn expect_get_instance_view(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachines/{vm_name}/instanceView"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_vmss`: List virtual machine scale sets in a resource group.
    fn expect_list_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachineScaleSets"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `get_vmss`: Get a virtual machine scale set.
    fn expect_get_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachineScaleSets/{vm_scale_set_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_vmss`: Create or update a virtual machine scale set.
    fn expect_create_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachineScaleSets/{vm_scale_set_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_vmss`: Delete a virtual machine scale set.
    fn expect_delete_vmss(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachineScaleSets/{vm_scale_set_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `list_vmss_instances`: List virtual machines in a VM scale set.
    fn expect_list_vmss_instances(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachineScaleSets/{vm_scale_set_name}/virtualMachines"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `start_vmss_instances`: Start one or more virtual machines in a VM scale
    /// set.
    fn expect_start_vmss_instances(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachineScaleSets/{vm_scale_set_name}/start"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `stop_vmss_instances`: Power off one or more virtual machines in a VM scale
    /// set.
    fn expect_stop_vmss_instances(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/virtualMachineScaleSets/{vm_scale_set_name}/poweroff"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `list_disks`: Lists all the disks under a resource group.
    fn expect_list_disks(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/disks"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `list_disks_in_subscription`: Lists all the disks under a subscription.
    fn expect_list_disks_in_subscription(
        &mut self,
        subscription_id: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!("/subscriptions/{subscription_id}/providers/Microsoft.Compute/disks");
        self.expect_get(&path)
    }

    /// Helper to expect `get_disk`: Gets information about a disk.
    fn expect_get_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/disks/{disk_name}"
        );
        self.expect_get(&path)
    }

    /// Helper to expect `create_disk`: Creates or updates a disk.
    fn expect_create_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/disks/{disk_name}"
        );
        self.expect_put(&path)
    }

    /// Helper to expect `delete_disk`: Deletes a disk.
    fn expect_delete_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/disks/{disk_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `update_disk`: Updates (patches) a disk.
    fn expect_update_disk(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/disks/{disk_name}"
        );
        self.expect_patch(&path)
    }

    /// Helper to expect `delete_snapshot`: Deletes a snapshot.
    fn expect_delete_snapshot(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        snapshot_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/snapshots/{snapshot_name}"
        );
        self.expect_delete(&path)
    }

    /// Helper to expect `grant_access`: Grants access to a disk.
    fn expect_grant_access(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/disks/{disk_name}/beginGetAccess"
        );
        self.expect_post(&path)
    }

    /// Helper to expect `revoke_access`: Revokes access to a disk.
    fn expect_revoke_access(
        &mut self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> crate::mock_client::ExpectationBuilder<'_> {
        let path = format!(
            "/subscriptions/{subscription_id}/resourceGroups/{resource_group_name}/providers/Microsoft.Compute/disks/{disk_name}/endGetAccess"
        );
        self.expect_post(&path)
    }
}
