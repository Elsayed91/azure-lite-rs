# Blocked: Azure Blob Data Plane requires XML wire format support

**Task:** 2.3 — Extend storage manifest with blob data plane operations
**Blocked at:** Manifest authoring / codegen
**Date:** 2026-02-18

## Problem

The Azure Blob Storage **data plane** (`https://{account}.blob.core.windows.net`) is fundamentally different from the ARM management plane (`https://management.azure.com`) used by all existing azure-lite operations:

| Dimension | ARM (management plane) | Blob Data Plane |
|-----------|----------------------|-----------------|
| Base URL | `https://management.azure.com` | `https://{accountName}.blob.core.windows.net` |
| Wire format | JSON (`application/json`) | **XML** (`application/xml`) |
| Auth header | `Authorization: Bearer {token}` | `Authorization: Bearer {token}` + `x-ms-version` header |
| URL routing | REST paths | Query params (`?comp=list`, `?restype=container&comp=list`) |
| API version | `?api-version=2023-05-01` | `x-ms-version: 2020-10-02` (header, not query param) |

## What's Missing

The codegen infrastructure needs:

1. **`azure_blob_xml` wire format** in `codegen/core/emitter.py`
   - Must use azure-lite's `self.client.get(url)` interface
   - Must parse XML responses using `quick_xml`
   - Must inject `x-ms-version` header
   - Must support query-param-based operation routing (e.g., `?comp=list`)

2. **Account-scoped URL handling** in the azure plugin (`codegen/plugin.py`)
   - Need a way to express `https://{accountName}.blob.core.windows.net/{containerName}` as a URL template
   - The `{accountName}` comes from a constructor parameter, not a path segment

3. **`quick_xml` dependency** in `Cargo.toml`

4. **XML parsing helper** in `src/` (similar to aws-lite's `xml.rs`)

## Impact

- Tasks 2.3 and 2.4 are both blocked on this.
- Tasks 2.5+ (Managed Disks — ARM JSON) are NOT affected and can proceed.

## Recommended Fix

Before implementing 2.3/2.4, complete the following codegen extension (estimated medium effort):

1. Add `quick_xml` to `Cargo.toml`
2. Add `src/xml.rs` to azure-lite with blob XML response parsers
3. Add `azure_blob_xml` wire format to `codegen/core/emitter.py`
4. Update `codegen/plugin.py` to support account-scoped URL templates
5. Full safety protocol: codegen apply + `cargo check` + full test suite

## Workaround

None — the XML wire format is required by the Azure Blob Storage REST API specification. Mock/stub implementations would not satisfy the integration-first testing requirement.
