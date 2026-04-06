# API Documentation Template

When adding a new API, create `docs/{api_name}/` with three files.

## docs/{api_name}/api.md

```markdown
# {Display Name} API

## Overview

Brief description of what this API does and when you'd use it.

## Client Access

```rust
let client = AzureHttpClient::from_cli().await?;
let {accessor} = client.{accessor_name}();
```

## Features

- List the key features/capabilities this client provides
- Note any convenience methods
- Note any async/LRO support

## Types

| Type | Description |
|------|-------------|
| `{Type}` | Main resource type |
| `{ListType}` | List response wrapper |

## Error Handling

Common errors for this API:
- `AzureError::NotFound` — resource doesn't exist
- `AzureError::PermissionDenied` — insufficient permissions
- `AzureError::AuthenticationFailed` — token expired or invalid
```

## docs/{api_name}/operations.md

```markdown
# {Display Name} Operations

## {Resource Group 1}

### {method_name}

**Signature**: `pub async fn {method}({params}) -> Result<{ReturnType}>`

{Brief description}

| Parameter | Type | Description |
|-----------|------|-------------|
| `subscription_id` | `&str` | Azure subscription ID |
| `resource_group` | `&str` | Resource group name |
| ... | ... | ... |

**Returns**: `Result<{Type}>`

---
```

## docs/{api_name}/usage.md

```markdown
# {Display Name} Usage Examples

## Basic CRUD

### Create a resource

```rust
use azure_lite::AzureHttpClient;
use azure_lite::types::{api}::{Type};

let client = AzureHttpClient::from_cli().await?;

let resource = {Type} {
    name: Some("my-resource".to_string()),
    location: Some("eastus".to_string()),
    ..Default::default()
};

client.{api}().create_{resource}(subscription_id, resource_group, &resource).await?;
```

### Get a resource

```rust
let resource = client.{api}().get_{resource}(subscription_id, resource_group, "my-resource").await?;
println!("Status: {:?}", resource.properties);
```

### List resources

```rust
let list = client.{api}().list_{resources}(subscription_id, resource_group).await?;
for item in &list.value {
    println!("{}", item.name.as_deref().unwrap_or("unnamed"));
}
```

### Delete a resource

```rust
client.{api}().delete_{resource}(subscription_id, resource_group, "my-resource").await?;
```

## Testing

```rust
use azure_lite::{AzureHttpClient, MockClient};
use azure_lite::test_support::{ApiMockHelpers};
use azure_lite::types::{api}::*;

#[tokio::test]
async fn test_example() {
    let mut mock = MockClient::new();
    mock.expect_get_{resource}(subscription_id, resource_group, "name")
        .returning_json(serde_json::to_value({Type}::fixture()).unwrap());

    let client = AzureHttpClient::from_mock(mock);
    let result = client.{api}().get_{resource}(subscription_id, resource_group, "name").await;
    assert!(result.is_ok());
}
```
```
