# Integration Testing Methodology

Shared methodology for integration tests. Provider-specific details are in their respective testing guides.

## The Rule

**Integration tests MUST pass for every operation group BEFORE writing unit tests.** Unit tests encode proven behavior from real API calls, not assumptions from reading docs.

This is non-negotiable. No exceptions. No rationalizations.

## Why

API docs lie. Discovery docs lie. Only the real API tells the truth:

- Azure ARM may reject fields that docs say are optional
- API versions may behave differently than documented
- Response shapes may include undocumented fields

## The Fixture Rule — MANDATORY

**Every prerequisite resource MUST be created by the test itself.** If an operation requires resource X to exist, the test creates resource X. No exceptions.

- **If the library supports creating it** → use the library client
- **If the library does NOT support creating it** → use CLI (`az`) via `std::process::Command`
- **NEVER skip creating a prerequisite.** Not having a resource "in this account" is not a valid reason to skip — that's WHY you create fixtures.

A test that queries pre-existing account resources and silently passes when none exist is not a test — it's a lie.

## Structure: Setup → Test → Cleanup → Propagate

Every integration test follows this exact structure:

```
┌─────────────────────────────────┐
│  Pre-cleanup                    │  Delete leftovers from previous failed runs
│  (idempotent, ignores errors)   │
├─────────────────────────────────┤
│  Create fixtures                │  Via library client or CLI (see Fixture Rule)
│  (resources the test needs)     │
├─────────────────────────────────┤
│  let result = run_tests().await │  All test logic in a helper returning Result
├─────────────────────────────────┤
│  Cleanup                        │  Always runs, even on failure
│  (delete all fixtures)          │
├─────────────────────────────────┤
│  result?                        │  Propagate AFTER cleanup
└─────────────────────────────────┘
```

```rust
#[tokio::test]
#[ignore = "requires credentials"]
async fn test_api_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    // Pre-cleanup
    cleanup_fixture(&client);

    // Create fixtures
    create_fixture(&client);

    // Run tests (captures Result, does NOT early-return)
    let result = run_tests(&client).await;

    // ALWAYS cleanup
    cleanup_fixture(&client);

    // Propagate AFTER cleanup
    result
}
```

## Fixture Management: Library First, CLI as Fallback

**Prefer creating fixtures with the library client** when the library supports the operation. This tests real integration paths and is more natural.

**Fall back to CLI** (`az`) via `std::process::Command` when the library doesn't support the operation needed for test setup. This is not a workaround — it's the correct approach for operations the library can't do yet.

The point is: **the prerequisite MUST be created, one way or another.** Never rationalize skipping fixture creation.

### Azure Example

```rust
use std::process::Command;

fn az_create_resource_group(name: &str, location: &str) -> bool {
    let output = Command::new("az")
        .args(["group", "create", "--name", name, "--location", location])
        .output()
        .expect("az cli must be installed");
    output.status.success()
}

fn az_delete_resource_group(name: &str) {
    let _ = Command::new("az")
        .args(["group", "delete", "--name", name, "--yes", "--no-wait"])
        .output();
}
```

### Rules for CLI Helpers

- **Cleanup helpers return `()`** — silently ignore all errors (`let _ =`)
- **Create helpers return `Option<T>` or `bool`** — handle already-exists gracefully
- **Always pass `--yes` / suppress prompts**
- **Always pass explicit location/subscription** — never rely on defaults
- **Use `serde_json` to parse CLI JSON output** when you need return values

## What to Test (Comprehensive Checklist)

### Happy Paths

1. **Basic CRUD** — CREATE → GET → LIST → DELETE → VERIFY DELETED
2. **Multiple resource types** — don't just test one resource, test variety
3. **Different parameter combinations** — with and without optional params
4. **Nested/complex responses** — verify deep fields are parseable
5. **Aggregation/filtering** — WHERE, LIKE, GROUP BY, COUNT (if applicable)

### Pagination (REQUIRED if API supports it)

1. **Force pagination** — set limit=1 on a resource type with >1 item
2. **Verify page 2 is different** — `assert_ne!(page1.results[0], page2.results[0])`
3. **Stream helper** — verify the stream collects ALL items across pages
4. **Empty results** — query that returns 0 items with no pagination token

### Error Cases (REQUIRED for every API)

1. **Invalid input** — malformed request body, invalid fields, empty required fields
2. **Non-existent resource** — GET/DELETE on something that doesn't exist
3. **Boundary violations** — values outside allowed ranges
4. **Verify error types** — assert specific error codes, not just `is_err()`
5. **Exploratory** — try edge cases you're unsure about and document what happens

```rust
// Good: verifies specific error type
let err = result.unwrap_err();
assert!(format!("{err}").contains("ResourceNotFound"));

// Bad: just checks it failed
assert!(result.is_err());
```

### Edge Cases

- **SELECT \*** vs specific fields — may return different field sets
- **Special characters** in query strings
- **Empty optional params** — some APIs reject `?param=` vs omitting entirely

## Output: Step-Numbered Println

Every test step prints its progress. This is mandatory for debugging CI failures.

```rust
println!("\n=== API Name: Test Category ===");
println!("Subscription: {subscription_id}");

println!("\n[1/8] Creating resource...");
println!("  Created: {id}");

println!("\n[2/8] Querying resources...");
println!("  Results: {}", response.results.len());
println!("  First: {}", &result[..result.len().min(120)]);
```

- Step 0 is setup/fixture creation
- Print actual values, not just "ok"
- Truncate long values with `.min(N)`
- Prefix detail lines with two spaces

## Deterministic Resource Names

Use constants. Never generate random names. This enables reliable pre-cleanup.

```rust
const TEST_RG_NAME: &str = "cloud-lite-test-compute";
const TEST_VM_NAME: &str = "cloud-lite-test-compute-vm";
```

Pattern: `cloud-lite-test-{api}-{resource}`

## Eventual Consistency

Cloud APIs are eventually consistent. Add sleeps after mutations:

| Operation | Wait |
|-----------|------|
| Create resource | 2-5s before GET/LIST |
| Delete resource | 2-3s before verifying gone |
| IAM/permission change | 5-10s for propagation |

## Test File Structure

```
tests/integration/
├── main.rs              # mod declarations for each API
├── compute.rs           # Compute integration tests
├── storage.rs           # Storage integration tests
└── network.rs           # Network integration tests
```

Each file:
1. Module-level doc comment with run instructions
2. Imports + constants
3. CLI helper functions (create, delete, verify)
4. `#[tokio::test] #[ignore]` main test function (setup/cleanup wrapper)
5. `async fn run_*_tests()` helper (all test logic, returns Result)
6. Separate `#[tokio::test] #[ignore]` for error cases

## Running

```bash
# Single API
cargo test --test integration compute -- --ignored --test-threads=1 --nocapture

# All integration tests
cargo test --test integration -- --ignored --test-threads=1 --nocapture
```

Always `--test-threads=1` (shared cloud resources) and `--nocapture` (see println output).

## Anti-Patterns — NEVER Do These

These are real mistakes agents have made. Every one of them violates the Fixture Rule.

### 1. Rationalizing away prerequisite creation

> "DetachVolume requires a volume attached to an instance. Since we don't have instances in this test account, I'll test the API call path but expect it to fail gracefully."

**This is wrong.** If DetachVolume requires an attached instance, the test MUST create an instance (via library or CLI), attach a volume, then test detach. "We don't have X" is never a valid excuse — that's what fixture creation is for.

### 2. Querying pre-existing resources and silently passing

```rust
// BAD: silently passes when account has no resources
let result = client.list_resources().await?;
if result.items.is_empty() {
    println!("No resources found, skipping");
    return Ok(());  // THIS IS A LIE — nothing was tested
}
```

The test MUST create its own resources with deterministic names, exercise the operations on them, then clean up.

### 3. Testing only read operations without creating anything

A test that only calls `list_*` or `describe_*` on whatever happens to exist in the account is not an integration test. It's reading someone else's state. Proper tests create → read → verify → delete → verify-deleted.

### 4. Using non-deterministic names (timestamps, random)

```rust
// BAD: can't pre-cleanup, leaks resources on failure
let name = format!("test-resource-{}", Utc::now().timestamp());
```

Use deterministic constants: `const TEST_NAME: &str = "cloud-lite-test-api-resource";`. This enables pre-cleanup of leftovers from previous failed runs.

### 5. Skipping cleanup ("manual cleanup required")

If the library can't delete a resource, use CLI for cleanup. If CLI can't either, find a way or flag it — but never leave resources leaking with a TODO comment.

### 6. Weak error assertions

```rust
// BAD: just checks it failed, doesn't verify why
assert!(result.is_err());

// GOOD: verifies the specific error
let err = result.unwrap_err();
assert!(format!("{err}").contains("ResourceNotFound"),
    "Expected ResourceNotFound, got: {err}");
```
