---
name: azure-api
description: Add a new Azure API or extend an existing one. Auto-detects whether to bootstrap a new API or add operations to an existing one.
argument-hint: "[service_name]: [operations needed]"
---

# Azure API Workflow

You are adding or extending an Azure API in azure-lite. The user's request: "$ARGUMENTS"

This workflow uses **integration-first development**: each operation group is validated against the real Azure ARM API before unit tests are written.

## IMMUTABLE RULES

0. **Integration tests MUST pass for an operation group BEFORE writing unit tests.** Unit tests encode proven behavior, not assumptions.
1. **NEVER edit generated files** (`src/types/`, `src/ops/`, `src/test_support/`) — a hook blocks this. Flag issues → manifest fix → wait for approval.
2. **NEVER modify codegen scripts** without explicit user approval.
3. **NEVER skip or work around authentication failures** — ask user to re-authenticate (`az login` + `az account set --subscription {id}`).
4. **NEVER overwrite existing `src/api/*.rs` files** — only create new or add methods.
5. **NEVER choose the simplest fix by default** — explore solutions, choose the one that generalizes and avoids technical debt.

## Prerequisites

Read these docs:
1. `.claude/docs/azure/architecture.md` — 3-layer pattern and ARM REST conventions
2. `.claude/docs/azure/codegen-reference.md` — manifest format and field overrides
3. `.claude/docs/azure/testing-guide.md` — test patterns

## Phase 1: Discovery

1. Parse the user's request: service name, requested operations.

2. **Detect add vs extend**:
   - Check for `codegen/manifests/{service_name}.toml` and `src/api/{service_name}.rs`
   - **Both exist** → EXTEND mode
   - **Neither exists** → ADD mode
   - **Manifest only** → resume interrupted add (skip to Phase 3)

3. **ADD mode**: Bootstrap to scaffold a draft manifest:
   ```
   python3 codegen/bootstrap.py {service_name}
   ```
   Read draft manifest. Wire format is always `rest_json` for ARM APIs.
   Note the `api_version` — all ARM requests require `?api-version=` as a query param.

4. **EXTEND mode**: Read existing manifest and API client. Discover available additions:
   ```
   python3 codegen/extend.py {service_name} --available-types
   ```

5. Present available operations (AskUserQuestion). Group by resource type, let user select.

6. Plan operation groups for incremental development (CRUD first, async/LRO last).

## Phase 2: Manifest

### ADD mode
Edit `codegen/manifests/{service_name}.toml`:
- `[api]` metadata: `name`, `display_name`, `version`, `api_version`, `service_name`, `wire_format = "rest_json"`, `base_url`, `doc_url`
- `[api.client]`: `accessor_name`, `client_struct`
- `[[types]]`: `name`, `description`, `[[types.fields]]` with `name`, `type`, `required`, `description`
  - Field types: `string`, `boolean`, `integer`, `int64`, `number`, `datetime`, `object`, `array`
  - Use `required = true` for mandatory fields (name, location, etc.)
  - ARM IDs: always `type = "string"`, never required in request bodies
- `[[operations]]`: `name` (PascalCase), `rust_name` (snake_case), `method`, `url_template`, `response_type`, `description`
  - URL template variables: `{subscriptionId}`, `{resourceGroupName}`, `{name}`, etc.
  - `api_version` is injected automatically — do NOT put it in url_template

### EXTEND mode
Add new `[[types]]` and/or `[[operations]]` following existing manifest conventions.

**COMMIT**: `feat: {add|extend} {service_name} manifest`

## Phase 3: Generation

```
cd codegen && uv run python -m codegen.cli apply
cargo check
```

Fix manifest on failure (wrong type name, missing required fields, undefined response_type).

**COMMIT**: `feat: generate types/ops for {service_name}`

## Phase 4: Registration & Scaffolding (ADD mode only)

Skip for EXTEND mode.

1. Verify codegen auto-registered in `src/api/mod.rs` and `src/client.rs`
2. Create `src/api/{service_name}.rs` with struct shell
3. Create `tests/integration/{service_name}.rs` with module header
4. `cargo check`

**COMMIT**: `feat: scaffold {service_name} API client`

## Phase 5: Incremental Development

For each operation group:

### Step A: Write API Methods
Add methods to `src/api/{service_name}.rs` for THIS GROUP ONLY:
- Ergonomic signatures: `subscription_id`, `resource_group`, `name` — not raw URL paths
- Construct ARM URLs: `format!("https://management.azure.com/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.{Provider}/{type}/{name}")`
- Append `api-version` query param via `append_query_params`
- Thin wrappers delegating to generated ops

### Step B: Write Integration Test
Add test function in `tests/integration/{service_name}.rs`:
- Real create → get → list → update → delete lifecycle
- Use `AZURE_SUBSCRIPTION_ID` and `AZURE_RESOURCE_GROUP` env vars (or defaults)
- Always-cleanup pattern (cleanup in both success and failure paths), step-numbered `println!`
- Deterministic resource names: `cloud-lite-test-{feature-slug}-{resource}`

### Step C: Run Integration Test
```
cargo test --test integration {service_name} -- --ignored --test-threads=1 --nocapture
```
Fix failures (wrong api-version, missing required fields, wrong URL structure). Re-run until passing.

### Step D: Write Unit Tests
Encode proven behavior with MockClient. Every test verifies actual data — never just `is_ok()`.

### Step E: Run Full Test Suite
```
cargo test --lib
```

### Step F: Commit
```
feat: add {service_name} {group_name} operations
```

**Repeat Steps A-F for each operation group.**

## Phase 6: Documentation

- **ADD**: Create `docs/{service_name}/` with api.md, operations.md, usage.md (see `.claude/docs/api-doc-template.md`)
- **EXTEND**: Update existing `docs/{service_name}/operations.md` and `usage.md`

**COMMIT**: `docs: {add|update} {service_name} documentation`

## Phase 7: Quality Gate

```
cargo check
cargo clippy -- -D warnings
cargo test --lib
```

Report results. Fix and re-run if anything fails.
