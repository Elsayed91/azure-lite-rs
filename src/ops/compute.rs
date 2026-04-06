//! Operation contracts for the Azure Compute API (v1).
//!
//! Auto-generated from the Azure ARM REST Specification.
//! **Do not edit manually** — modify the manifest and re-run codegen.
//!
//! These are the raw HTTP operations with correct URLs, methods,
//! and parameter ordering. The hand-written `api/compute.rs` wraps
//! these with ergonomic builders, operation polling, etc.

use crate::types::compute::*;
use crate::{AzureHttpClient, Result};
use urlencoding::encode;

/// Raw HTTP operations for the Azure Compute API.
///
/// These methods encode the correct URL paths, HTTP methods, and
/// parameter ordering from the Azure ARM REST Specification.
/// They are `pub(crate)` — use the ergonomic wrappers in
/// [`super::compute::ComputeClient`] instead.
pub struct ComputeOps<'a> {
    pub(crate) client: &'a AzureHttpClient,
}

impl<'a> ComputeOps<'a> {
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

    /// List virtual machines in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`VirtualMachineListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_vms(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<VirtualMachineListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_vms response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_vms response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get a virtual machine.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    ///
    /// # Query Parameters
    /// - `$expand` — Expand expression (e.g. instanceView)
    ///
    /// # Response
    /// [`VirtualMachine`]
    #[allow(dead_code)]
    pub(crate) async fn get_vm(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
        expand: &str,
    ) -> Result<VirtualMachine> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let url = crate::append_query_params(url, &[("$expand", expand)]);
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_vm response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_vm response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update a virtual machine.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    ///
    /// # Request Body
    /// [`VirtualMachineCreateRequest`]
    ///
    /// # Response
    /// [`VirtualMachine`]
    #[allow(dead_code)]
    pub(crate) async fn create_vm(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
        body: &VirtualMachineCreateRequest,
    ) -> Result<VirtualMachine> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_vm request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_vm response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_vm response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Delete a virtual machine.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_vm(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Start a virtual machine.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/start`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn start_vm(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/start",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Power off (stop) a virtual machine. The VM continues to be billed.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/powerOff`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn stop_vm(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/powerOff",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Deallocate a virtual machine. Stops billing.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/deallocate`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn deallocate_vm(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/deallocate",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Restart a virtual machine.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/restart`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn restart_vm(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/restart",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Get the instance view of a virtual machine.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}/instanceView`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmName` —  *(required)*
    ///
    /// # Response
    /// [`VirtualMachineInstanceViewResult`]
    #[allow(dead_code)]
    pub(crate) async fn get_instance_view(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_name: &str,
    ) -> Result<VirtualMachineInstanceViewResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/instanceView",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_instance_view response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_instance_view response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// List virtual machine scale sets in a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachineScaleSets`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`VirtualMachineScaleSetListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_vmss(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<VirtualMachineScaleSetListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_vmss response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_vmss response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Get a virtual machine scale set.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachineScaleSets/{vmScaleSetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmScaleSetName` —  *(required)*
    ///
    /// # Response
    /// [`VirtualMachineScaleSet`]
    #[allow(dead_code)]
    pub(crate) async fn get_vmss(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> Result<VirtualMachineScaleSet> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_scale_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_vmss response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_vmss response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Create or update a virtual machine scale set.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachineScaleSets/{vmScaleSetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmScaleSetName` —  *(required)*
    ///
    /// # Request Body
    /// [`VirtualMachineScaleSetCreateRequest`]
    ///
    /// # Response
    /// [`VirtualMachineScaleSet`]
    #[allow(dead_code)]
    pub(crate) async fn create_vmss(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
        body: &VirtualMachineScaleSetCreateRequest,
    ) -> Result<VirtualMachineScaleSet> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_scale_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_vmss request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_vmss response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_vmss response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Delete a virtual machine scale set.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachineScaleSets/{vmScaleSetName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmScaleSetName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_vmss(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_scale_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// List virtual machines in a VM scale set.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachineScaleSets/{vmScaleSetName}/virtualMachines`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmScaleSetName` —  *(required)*
    ///
    /// # Response
    /// [`VirtualMachineScaleSetVMListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_vmss_instances(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
    ) -> Result<VirtualMachineScaleSetVMListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}/virtualMachines",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_scale_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_vmss_instances response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_vmss_instances response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Start one or more virtual machines in a VM scale set.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachineScaleSets/{vmScaleSetName}/start`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmScaleSetName` —  *(required)*
    ///
    /// # Request Body
    /// [`VirtualMachineScaleSetVMInstanceIDs`]
    #[allow(dead_code)]
    pub(crate) async fn start_vmss_instances(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
        body: &VirtualMachineScaleSetVMInstanceIDs,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}/start",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_scale_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize start_vmss_instances request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Power off one or more virtual machines in a VM scale set.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachineScaleSets/{vmScaleSetName}/poweroff`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `vmScaleSetName` —  *(required)*
    ///
    /// # Request Body
    /// [`VirtualMachineScaleSetVMInstanceIDs`]
    #[allow(dead_code)]
    pub(crate) async fn stop_vmss_instances(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        vm_scale_set_name: &str,
        body: &VirtualMachineScaleSetVMInstanceIDs,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachineScaleSets/{}/poweroff",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(vm_scale_set_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize stop_vmss_instances request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Lists all the disks under a resource group.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/disks`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    ///
    /// # Response
    /// [`DiskListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_disks(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> Result<DiskListResult> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/disks",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_disks response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_disks response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Lists all the disks under a subscription.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/providers/Microsoft.Compute/disks`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    ///
    /// # Response
    /// [`DiskListResult`]
    #[allow(dead_code)]
    pub(crate) async fn list_disks_in_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<DiskListResult> {
        let url = format!(
            "{}/subscriptions/{}/providers/Microsoft.Compute/disks",
            self.base_url(),
            encode(subscription_id),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read list_disks_in_subscription response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse list_disks_in_subscription response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Gets information about a disk.
    ///
    /// **Azure API**: `GET /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/disks/{diskName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `diskName` —  *(required)*
    ///
    /// # Response
    /// [`Disk`]
    #[allow(dead_code)]
    pub(crate) async fn get_disk(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> Result<Disk> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/disks/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(disk_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.get(&url).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read get_disk response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse get_disk response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Creates or updates a disk.
    ///
    /// **Azure API**: `PUT /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/disks/{diskName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `diskName` —  *(required)*
    ///
    /// # Request Body
    /// [`DiskCreateRequest`]
    ///
    /// # Response
    /// [`Disk`]
    #[allow(dead_code)]
    pub(crate) async fn create_disk(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
        body: &DiskCreateRequest,
    ) -> Result<Disk> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/disks/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(disk_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize create_disk request: {e}"),
                body: None,
            })?;
        let response = self.client.put(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read create_disk response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse create_disk response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a disk.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/disks/{diskName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `diskName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_disk(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/disks/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(disk_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Updates (patches) a disk.
    ///
    /// **Azure API**: `PATCH /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/disks/{diskName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `diskName` —  *(required)*
    ///
    /// # Request Body
    /// [`DiskUpdateRequest`]
    ///
    /// # Response
    /// [`Disk`]
    #[allow(dead_code)]
    pub(crate) async fn update_disk(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
        body: &DiskUpdateRequest,
    ) -> Result<Disk> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/disks/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(disk_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize update_disk request: {e}"),
                body: None,
            })?;
        let response = self.client.patch(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read update_disk response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse update_disk response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Deletes a snapshot.
    ///
    /// **Azure API**: `DELETE /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/snapshots/{snapshotName}`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `snapshotName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn delete_snapshot(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        snapshot_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/snapshots/{}",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(snapshot_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.delete(&url).await?;
        response.error_for_status().await?;
        Ok(())
    }

    /// Grants access to a disk.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/disks/{diskName}/beginGetAccess`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `diskName` —  *(required)*
    ///
    /// # Request Body
    /// [`GrantAccessData`]
    ///
    /// # Response
    /// [`AccessUri`]
    #[allow(dead_code)]
    pub(crate) async fn grant_access(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
        body: &GrantAccessData,
    ) -> Result<AccessUri> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/disks/{}/beginGetAccess",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(disk_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let body_bytes =
            serde_json::to_vec(body).map_err(|e| crate::AzureError::InvalidResponse {
                message: format!("Failed to serialize grant_access request: {e}"),
                body: None,
            })?;
        let response = self.client.post(&url, &body_bytes).await?;
        let response = response.error_for_status().await?;
        let response_bytes =
            response
                .bytes()
                .await
                .map_err(|e| crate::AzureError::InvalidResponse {
                    message: format!("Failed to read grant_access response: {e}"),
                    body: None,
                })?;
        serde_json::from_slice(&response_bytes).map_err(|e| crate::AzureError::InvalidResponse {
            message: format!("Failed to parse grant_access response: {e}"),
            body: Some(String::from_utf8_lossy(&response_bytes).to_string()),
        })
    }

    /// Revokes access to a disk.
    ///
    /// **Azure API**: `POST /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/disks/{diskName}/endGetAccess`
    ///
    /// # Path Parameters
    /// - `subscriptionId` —  *(required)*
    /// - `resourceGroupName` —  *(required)*
    /// - `diskName` —  *(required)*
    #[allow(dead_code)]
    pub(crate) async fn revoke_access(
        &self,
        subscription_id: &str,
        resource_group_name: &str,
        disk_name: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/disks/{}/endGetAccess",
            self.base_url(),
            encode(subscription_id),
            encode(resource_group_name),
            encode(disk_name),
        );
        let sep = if url.contains('?') { "&" } else { "?" };
        let url = format!("{}{}api-version=2024-07-01", url, sep);
        let response = self.client.post(&url, &[]).await?;
        response.error_for_status().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_vms() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines")
            .returning_json(serde_json::to_value(VirtualMachineListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_vms("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_vm() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName?$expand=test-%24expand")
            .returning_json(serde_json::to_value(VirtualMachine::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_vm(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
                "test-$expand",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_vm() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName")
            .returning_json(serde_json::to_value(VirtualMachine::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = VirtualMachineCreateRequest::fixture();
        let result = ops
            .create_vm(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_vm() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_vm(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_vm() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName/start")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .start_vm(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_vm() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName/powerOff")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .stop_vm(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_deallocate_vm() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName/deallocate")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .deallocate_vm(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_restart_vm() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName/restart")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .restart_vm(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_instance_view() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachines/test-vmName/instanceView")
            .returning_json(serde_json::to_value(VirtualMachineInstanceViewResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_instance_view(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_vmss() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachineScaleSets")
            .returning_json(serde_json::to_value(VirtualMachineScaleSetListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_vmss("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_vmss() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachineScaleSets/test-vmScaleSetName")
            .returning_json(serde_json::to_value(VirtualMachineScaleSet::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_vmss(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmScaleSetName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_vmss() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachineScaleSets/test-vmScaleSetName")
            .returning_json(serde_json::to_value(VirtualMachineScaleSet::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = VirtualMachineScaleSetCreateRequest::fixture();
        let result = ops
            .create_vmss(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmScaleSetName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_vmss() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachineScaleSets/test-vmScaleSetName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_vmss(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmScaleSetName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_vmss_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachineScaleSets/test-vmScaleSetName/virtualMachines")
            .returning_json(serde_json::to_value(VirtualMachineScaleSetVMListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_vmss_instances(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmScaleSetName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_vmss_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachineScaleSets/test-vmScaleSetName/start")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = VirtualMachineScaleSetVMInstanceIDs::fixture();
        let result = ops
            .start_vmss_instances(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmScaleSetName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_vmss_instances() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/virtualMachineScaleSets/test-vmScaleSetName/poweroff")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = VirtualMachineScaleSetVMInstanceIDs::fixture();
        let result = ops
            .stop_vmss_instances(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-vmScaleSetName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_disks() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/disks")
            .returning_json(serde_json::to_value(DiskListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .list_disks("test-subscriptionId", "test-resourceGroupName")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_disks_in_subscription() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/providers/Microsoft.Compute/disks")
            .returning_json(serde_json::to_value(DiskListResult::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops.list_disks_in_subscription("test-subscriptionId").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_get("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/disks/test-diskName")
            .returning_json(serde_json::to_value(Disk::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .get_disk(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-diskName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_put("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/disks/test-diskName")
            .returning_json(serde_json::to_value(Disk::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = DiskCreateRequest::fixture();
        let result = ops
            .create_disk(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-diskName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/disks/test-diskName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_disk(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-diskName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_disk() {
        let mut mock = crate::MockClient::new();

        mock.expect_patch("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/disks/test-diskName")
            .returning_json(serde_json::to_value(Disk::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = DiskUpdateRequest::fixture();
        let result = ops
            .update_disk(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-diskName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_snapshot() {
        let mut mock = crate::MockClient::new();

        mock.expect_delete("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/snapshots/test-snapshotName")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .delete_snapshot(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-snapshotName",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_grant_access() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/disks/test-diskName/beginGetAccess")
            .returning_json(serde_json::to_value(AccessUri::fixture()).unwrap());

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let body = GrantAccessData::fixture();
        let result = ops
            .grant_access(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-diskName",
                &body,
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_revoke_access() {
        let mut mock = crate::MockClient::new();

        mock.expect_post("/subscriptions/test-subscriptionId/resourceGroups/test-resourceGroupName/providers/Microsoft.Compute/disks/test-diskName/endGetAccess")
            .returning_json(serde_json::json!({}));

        let client = crate::AzureHttpClient::from_mock(mock);
        let ops = ComputeOps::new(&client);

        let result = ops
            .revoke_access(
                "test-subscriptionId",
                "test-resourceGroupName",
                "test-diskName",
            )
            .await;
        assert!(result.is_ok());
    }
}
