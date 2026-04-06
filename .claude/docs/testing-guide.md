# Azure Testing Guide

## Integration-First Development

**Integration tests MUST pass BEFORE writing unit tests.** Every prerequisite resource MUST be created by the test itself.

See `.claude/docs/integration-testing-methodology.md` for the shared methodology.

## Authentication Setup

```bash
# Login
az login
az account set --subscription <subscription-id>

# Verify
az account show

# Set env var
export AZURE_SUBSCRIPTION_ID="<subscription-id>"
```

Integration tests use `default_credential()` which tries service principal → managed identity → CLI in order.

## Test Categories

### 1. Generated Ops Tests (codegen)

Location: `src/ops/<service>.rs` (inside `#[cfg(test)]` module)

Auto-generated round-trip serialization tests. **Never edit.**

### 2. Unit Tests

Location: `src/api/<service>.rs` (inside `#[cfg(test)]` module)

Test API layer logic using `MockClient`:

```rust
#[tokio::test]
async fn test_get_virtual_machine() {
    let mock = MockClient::new();
    mock.mock_virtual_machines_get(VirtualMachine {
        location: "eastus".into(),
        ..Default::default()
    });
    let client = mock.client();
    let vm = client.compute().get_virtual_machine("my-rg", "my-vm").await.unwrap();
    assert_eq!(vm.location, "eastus");
}
```

### 3. Integration Tests

Location: `tests/integration/<service>.rs`

Test against real Azure ARM APIs. Run locally only:

```bash
cargo test --test integration <service> -- --ignored --test-threads=1 --nocapture
```

### Integration Test Structure

```rust
#[tokio::test]
#[ignore] // requires Azure credentials
async fn test_virtual_machine_lifecycle() {
    let client = test_client().await;
    let rg = "cloud-lite-test-compute";
    let vm_name = "cloud-lite-test-vm";

    // Setup: ensure resource group exists
    client.subscriptions().create_or_update_resource_group(rg, "eastus").await.unwrap();

    // Create
    let vm = client.compute().create_or_update_virtual_machine(rg, vm_name, &body).await.unwrap();
    assert_eq!(vm.location.as_deref(), Some("eastus"));

    // Read
    let fetched = client.compute().get_virtual_machine(rg, vm_name).await.unwrap();
    assert!(fetched.id.is_some());

    // List
    let vms = client.compute().list_virtual_machines(rg).await.unwrap();
    assert!(vms.iter().any(|v| v.name.as_deref() == Some(vm_name)));

    // Delete
    client.compute().delete_virtual_machine(rg, vm_name).await.unwrap();

    // Verify deleted
    let err = client.compute().get_virtual_machine(rg, vm_name).await.unwrap_err();
    assert!(matches!(err, AzureError::NotFound { .. }));

    // Cleanup resource group
    client.subscriptions().delete_resource_group(rg).await.unwrap();
}
```

### Resource Group Management

Azure resources live in resource groups. Test conventions:
- **Name prefix**: `cloud-lite-test-<service>` to avoid collisions
- **Location**: Use `eastus` (cheapest, most services available)
- **Cleanup**: Delete the resource group at test end (cascading delete)
- **Idempotent setup**: Use `create_or_update_resource_group` (PUT is idempotent)

### Deterministic Naming

```
cloud-lite-test-<service>-<resource>
```

Examples:
- `cloud-lite-test-compute-vm`
- `cloud-lite-test-storage-account`
- `cloud-lite-test-keyvault-vault`

## Edge Cases to Test

### Per-Operation Group (minimum)

1. **CRUD lifecycle** — create → get → list → update → delete
2. **Error: non-existent** — GET on resource that doesn't exist → `AzureError::NotFound`
3. **Error: deleted** — GET after delete → `AzureError::NotFound`
4. **List: no filter** — List all in resource group
5. **List: with filter** — List with `$filter` query parameter

### Azure-Specific Patterns

- **Long-running operations**: Some PUT/DELETE return 202 with `Azure-AsyncOperation` header. Poll until complete.
- **Eventual consistency**: After DELETE, GET may still return 200 briefly. Add small delay or retry loop.
- **Resource provider registration**: Some APIs require `az provider register --namespace Microsoft.X` first.

## Running Tests

```bash
# All integration tests for a service
cargo test --test integration compute -- --ignored --test-threads=1 --nocapture

# Single test
cargo test --test integration test_virtual_machine_lifecycle -- --ignored --nocapture

# Unit tests only (CI)
cargo test --lib

# All unit + generated tests
cargo test --lib --all-features
```

## Quality Checklist

Before marking work complete:

- [ ] `cargo check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo test --lib` passes
- [ ] `cargo fmt --check` passes
- [ ] Integration tests pass for new/changed APIs
- [ ] `uv run python codegen/verify.py` passes (if manifests changed)
