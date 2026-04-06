---
name: quality-standards
description: used to review implemented tasks and enusre they match project standards.
user-invokable: true
disable-model-invocation: false
---

# azure-lite Quality Standards

## Review Flow

Execute these steps in order:

**1. Generated code integrity** (BLOCKING if violated)
```bash
git diff --name-only src/types/ src/ops/ src/test_support/
```
Any changes = CRITICAL. Also check `src/api/` for copy-pasted generated code.
**2. Understand changes** — `git diff --name-only` + `git diff`
**3. Quality gates**
```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo nextest run --lib
```
**4. Code review** — check against Architecture, Serde, and Review Checklist sections below.
**5. Test evaluation** — check against Testing Standards below. Flag tests that only assert `is_ok()`.
**6. Edge cases** — empty lists, missing optional fields, unknown enum variants, error responses, resource-not-found after deletion.
**7. Report** — organize by severity, each with file:line, issue, and fix suggestion.


### Severity
- **CRITICAL** — Bugs, generated code tampering, incorrect behavior. Blocks completion.
- **WARNING** — Missing test coverage, potential edge cases, trivial tests.
- **INFO** — Suggestions, not blocking.

## Architecture Rules

### Shared
- Never overwrite existing `src/api/*.rs` — only create new or extend
- Manifests are single source of truth
- No builders — `Default` derive + struct literal pattern

### Azure
- ARM REST: JSON wire format, Bearer token auth
- URL pattern: `https://management.azure.com/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.{Provider}/{type}/{name}`
- API version as query param: `?api-version=YYYY-MM-DD`

## Serde Conventions

- `Option<T>`: `skip_serializing_if = "Option::is_none"`
- `Vec<T>`: `#[serde(default)]` + `skip_serializing_if = "Vec::is_empty"`
- Enums: `#[serde(other)] Unknown` variant

## Testing Standards

**Integration-first**: integration tests MUST pass before writing unit tests. Unit tests encode proven behavior, not assumptions.

**Unit tests**: written after integration tests pass. Never just `is_ok()` — verify field values, list lengths, resource names.

**Integration tests**: library clients for cleanup (not CLI), deterministic `cloud-lite-test-` names, pre-cleanup -> test -> always-cleanup, step-numbered `println!` with `[N/total]`, never skip/mock auth.

## Review Checklist

### Azure
- Serde round-trip correctness, field naming matches API
- Error handling — no panics in library code
- Test quality — verify behavior, not just success
- ARM URL format correctness
- API version correctness
- Ergonomic method signatures (friendly params, not raw paths)

## References
- Shared: `.claude/docs/integration-testing-methodology.md`
