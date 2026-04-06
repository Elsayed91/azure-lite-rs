# Azure Codegen Reference

## Tools

### `codegen/cli.py` — Main CLI

```bash
cd codegen
uv run python -m codegen.cli apply              # Generate + apply all APIs
uv run python -m codegen.cli apply --api compute # Single API only
uv run python -m codegen.cli apply --dry-run     # Preview changes
uv run python -m codegen.cli generate            # Generate only (no apply)
```

### `codegen/bootstrap.py` — Draft a new manifest

```bash
uv run python codegen/bootstrap.py <service-name>
```

Creates a starter TOML manifest at `codegen/manifests/<service>.toml` from Azure ARM spec structure.

### `codegen/extend.py` — Explore and extend a manifest

```bash
uv run python codegen/extend.py <service>                    # Show current types/ops
uv run python codegen/extend.py <service> --available-types  # List available types
```

### `codegen/fetch_specs.py` — Download Azure REST specs

```bash
uv run python codegen/fetch_specs.py <service>
```

### `codegen/verify.py` — Validate manifests

```bash
uv run python codegen/verify.py
```

Checks all manifests for:
- Valid TOML syntax
- Required fields present
- Valid HTTP methods (GET, POST, PUT, PATCH, DELETE)
- Valid scalar types
- Type references resolve correctly

## TOML Manifest Format

Manifests live in `codegen/manifests/*.toml`. Each file defines one Azure service.

### Header

```toml
[meta]
api_name = "compute"
display_name = "Compute"
api_version = "2024-07-01"
resource_provider = "Microsoft.Compute"
```

### Types

```toml
[[types]]
name = "VirtualMachine"
description = "An Azure virtual machine."

[[types.fields]]
name = "location"
type = "string"
required = true

[[types.fields]]
name = "properties"
type = "VirtualMachineProperties"

[[types.fields]]
name = "tags"
type = "map<string, string>"
```

### Field Overrides

```toml
[[types.fields]]
name = "sku"
type = "Sku"
flatten = true          # Flatten nested struct into parent

[[types.fields]]
name = "id"
type = "string"
read_only = true        # Skip in serialization (server-assigned)
```

### Operations

```toml
[[operations]]
name = "virtual_machines_get"
display_name = "Get Virtual Machine"
method = "GET"
path = "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}"
response_type = "VirtualMachine"

[[operations.params]]
name = "resourceGroupName"
location = "path"
type = "string"
required = true

[[operations.params]]
name = "vmName"
location = "path"
type = "string"
required = true

[[operations]]
name = "virtual_machines_create_or_update"
display_name = "Create or Update Virtual Machine"
method = "PUT"
path = "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines/{vmName}"
request_type = "VirtualMachine"
response_type = "VirtualMachine"
```

### List Operations (Pagination)

```toml
[[operations]]
name = "virtual_machines_list"
method = "GET"
path = "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/virtualMachines"
response_type = "VirtualMachineListResult"
paginated = true
page_item_field = "value"
next_link_field = "nextLink"
```

### Operations with Query Parameters

```toml
[[operations.params]]
name = "$filter"
location = "query"
type = "string"

[[operations.params]]
name = "$expand"
location = "query"
type = "string"
```

## Generated Code Patterns

### Types (`src/types/<service>.rs`)

```rust
//! Types for the Compute API (2024-07-01).
//!
//! Auto-generated from the Azure ARM REST spec.
//! **Do not edit manually** — modify the manifest and re-run codegen.

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualMachine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, String>,
}
```

### Ops (`src/ops/<service>.rs`)

```rust
pub(crate) struct ComputeOps<'a> {
    client: &'a AzureHttpClient,
}

impl<'a> ComputeOps<'a> {
    pub(crate) async fn virtual_machines_get(
        &self, resource_group: &str, vm_name: &str,
    ) -> Result<VirtualMachine> {
        let url = format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}?api-version=2024-07-01",
            self.client.base_url(), self.client.subscription_id(), resource_group, vm_name
        );
        let resp = self.client.get(&url).await?;
        resp.error_for_status().await?.json().await
    }
}
```

## Currently Configured APIs

20 services with manifests in `codegen/manifests/`:

acr, aks, compute, cosmosdb, cost, dns, functions, graph, identity, keyvault, loganalytics, monitor, networking, rbac, redis, resource_graph, security, sql, storage, subscriptions
