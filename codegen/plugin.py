"""Azure provider plugin for the codegen pipeline.

Reads TOML manifests (hand-crafted from ARM REST specs) and produces
fully-resolved IR dataclasses for the shared Rust emitter.

Wire format: rest_json
  - Base URL: https://management.azure.com{path}?api-version={version}
  - camelCase JSON body/response
  - Authorization: Bearer {token} header
"""

from __future__ import annotations

import re
import tomllib
from pathlib import Path

from cloud_lite_codegen.ir import (
    ApiDef,
    ClientConfig,
    EnumDef,
    EnumVariant,
    FieldDef,
    FieldFormat,
    HttpMethod,
    OperationDef,
    PathParam,
    ProviderDef,
    QueryParam,
    TypeDef,
)
from cloud_lite_codegen.plugin import ProviderPlugin


def _to_snake_case(name: str) -> str:
    """Convert camelCase or PascalCase to snake_case."""
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def _resolve_rust_type(type_str: str) -> tuple[str, FieldFormat]:
    """Resolve an ARM JSON schema type string to (Rust type, FieldFormat)."""
    type_map = {
        "string": ("String", FieldFormat.NONE),
        "boolean": ("bool", FieldFormat.NONE),
        "integer": ("i32", FieldFormat.NONE),
        "int32": ("i32", FieldFormat.NONE),
        "int64": ("i64", FieldFormat.INT64),
        "number": ("f64", FieldFormat.NONE),
        "float": ("f64", FieldFormat.NONE),
        "double": ("f64", FieldFormat.NONE),
        "datetime": ("String", FieldFormat.DATE_TIME),
        "date-time": ("String", FieldFormat.DATE_TIME),
        "object": ("serde_json::Value", FieldFormat.NONE),
        "bytes": ("String", FieldFormat.BYTES),
    }
    normalized = type_str.lower()
    return type_map.get(normalized, ("String", FieldFormat.NONE))


def _resolve_field(field_entry: dict) -> FieldDef:
    """Resolve a fields entry in a [[types]] manifest entry into a FieldDef."""
    name = field_entry["name"]
    rust_name = field_entry.get("rust_name", _to_snake_case(name))
    rust_type_override = field_entry.get("rust_type", "")
    type_str = field_entry.get("type", "string")
    required = field_entry.get("required", False)
    description = field_entry.get("description", "")
    serde_rename = field_entry.get("serde_rename", "")
    enum_type = field_entry.get("enum_type", "")

    if rust_type_override:
        rust_type = rust_type_override
        field_format = FieldFormat.NONE
        is_repeated = rust_type.startswith("Vec<")
        is_map = rust_type.startswith("HashMap<")
    elif enum_type:
        rust_type = enum_type
        field_format = FieldFormat.NONE
        is_repeated = False
        is_map = False
    else:
        # Handle array types: "array<T>"
        type_lower = type_str.lower()
        if type_lower.startswith("array<") and type_lower.endswith(">"):
            inner = type_str[6:-1]
            inner_rust, _ = _resolve_rust_type(inner)
            rust_type = f"Vec<{inner_rust}>"
            field_format = FieldFormat.NONE
            is_repeated = True
            is_map = False
        elif type_lower == "array":
            rust_type = "Vec<serde_json::Value>"
            field_format = FieldFormat.NONE
            is_repeated = True
            is_map = False
        elif type_lower.startswith("map<") and type_lower.endswith(">"):
            rust_type = type_str  # pass through as-is
            field_format = FieldFormat.NONE
            is_repeated = False
            is_map = True
        else:
            rust_type, field_format = _resolve_rust_type(type_str)
            is_repeated = rust_type.startswith("Vec<")
            is_map = rust_type.startswith("HashMap<")

    return FieldDef(
        name=name,
        rust_name=rust_name,
        rust_type=rust_type,
        required=required,
        repeated=is_repeated,
        is_map=is_map,
        format=field_format,
        enum_type=enum_type,
        serde_rename=serde_rename,
        description=description,
    )


def _resolve_type(type_entry: dict) -> TypeDef:
    """Resolve a [[types]] manifest entry into a TypeDef."""
    name = type_entry["name"]
    schema_name = type_entry.get("schema_name", name)
    description = type_entry.get("description", "")
    fields_raw = type_entry.get("fields", [])
    fields = [_resolve_field(f) for f in fields_raw]

    return TypeDef(
        name=name,
        schema_name=schema_name,
        fields=fields,
        description=description,
        total_fields=len(fields),
        included_fields=len(fields),
    )


def _resolve_enum(enum_entry: dict) -> EnumDef:
    """Resolve a [[enums]] manifest entry into an EnumDef."""
    name = enum_entry["name"]
    values = enum_entry.get("values", [])
    variants = []
    for val in values:
        if isinstance(val, dict):
            api_name = val["value"]
            rust_name = val.get("rust_name", api_name)
            desc = val.get("description", "")
        else:
            api_name = str(val)
            # Convert SCREAMING_SNAKE to PascalCase
            if "_" in api_name and api_name == api_name.upper():
                rust_name = "".join(w.capitalize() for w in api_name.split("_"))
            else:
                rust_name = api_name[0].upper() + api_name[1:] if api_name else api_name
            desc = ""
        variants.append(EnumVariant(api_name=api_name, rust_name=rust_name, description=desc))

    all_screaming = bool(variants) and all(
        re.fullmatch(r"[A-Z0-9_]+", v.api_name) is not None
        for v in variants
        if v.api_name
    )

    return EnumDef(
        name=name,
        variants=variants,
        all_screaming=all_screaming,
        has_unknown=True,
        api_path=enum_entry.get("api_path", ""),
    )


def _resolve_operation(op_entry: dict) -> OperationDef:
    """Resolve a [[operations]] manifest entry into an OperationDef."""
    original_name = op_entry.get("name", "")
    name = op_entry.get("rust_name", _to_snake_case(original_name) if original_name else "")
    http_method = HttpMethod(op_entry.get("method", "GET").upper())
    url_template = op_entry.get("url_template", "/")
    description = op_entry.get("description", "")
    request_body_type = op_entry.get("request_body_type", "")
    response_type = op_entry.get("response_type", "")

    # Parse path params from URL template (e.g. {subscriptionId}, {resourceGroupName})
    path_params: list[PathParam] = []
    for match in re.finditer(r"\{(\w+)\}", url_template):
        param_name = match.group(1)
        path_params.append(PathParam(
            name=param_name,
            rust_name=_to_snake_case(param_name),
        ))

    # Query params declared in manifest
    query_params: list[QueryParam] = []
    for qp in op_entry.get("query_params", []):
        if isinstance(qp, str):
            query_params.append(QueryParam(
                name=qp,
                rust_name=_to_snake_case(qp),
            ))
        else:
            query_params.append(QueryParam(
                name=qp["name"],
                rust_name=qp.get("rust_name", _to_snake_case(qp["name"])),
                required=qp.get("required", False),
                description=qp.get("description", ""),
            ))

    return OperationDef(
        name=name,
        http_method=http_method,
        url_template=url_template,
        path_params=path_params,
        query_params=query_params,
        request_body_type=request_body_type,
        response_type=response_type,
        description=description,
        original_name=original_name,
    )


class AzurePlugin(ProviderPlugin):
    """Azure provider plugin using hand-crafted TOML manifests from ARM REST specs."""

    def __init__(self) -> None:
        self._base_dir = Path("codegen")
        self._manifests_dir = self._base_dir / "manifests"

    def name(self) -> str:
        return "azure"

    def target_crate(self) -> str:
        return "."

    def resolve(self, manifest_path: str) -> ApiDef:
        """Resolve a single Azure manifest into an ApiDef."""
        with open(manifest_path, "rb") as f:
            manifest = tomllib.load(f)

        api_section = manifest["api"]
        api_name = api_section["name"]
        version = api_section.get("version", "v1")
        api_version = api_section.get("api_version", "2024-01-01")
        display_name = api_section.get("display_name", api_name)
        doc_url = api_section.get("doc_url", "")

        # ARM management plane base URL
        base_url = api_section.get("base_url", "https://management.azure.com")

        # Resolve types
        type_defs = [_resolve_type(t) for t in manifest.get("types", [])]

        # Resolve enums
        enums = [_resolve_enum(e) for e in manifest.get("enums", [])]

        # Resolve operations
        op_defs = [_resolve_operation(op) for op in manifest.get("operations", [])]

        client_section = api_section.get("client", {})

        return ApiDef(
            name=api_name,
            display_name=display_name,
            version=version,
            base_url=base_url,
            doc_url=doc_url,
            wire_format=api_section.get("wire_format", "azure_rest_json"),
            rename_all="camelCase",
            api_version=api_version,
            service_name=api_section.get("service_name", api_name),
            client=ClientConfig(
                client_struct=client_section.get("client_struct", ""),
                accessor_name=client_section.get("accessor_name", api_name),
            ),
            types=type_defs,
            enums=enums,
            operations=op_defs,
        )

    def resolve_all(self) -> ProviderDef:
        """Resolve all Azure manifests into a ProviderDef."""
        apis: list[ApiDef] = []
        if self._manifests_dir.exists():
            for manifest_path in sorted(self._manifests_dir.glob("*.toml")):
                apis.append(self.resolve(str(manifest_path)))
        return ProviderDef(
            provider="azure",
            target_crate=".",
            client_struct="AzureHttpClient",
            apis=apis,
            rename_all="camelCase",
            wire_format="azure_rest_json",
            spec_source_name="the Azure ARM REST Specification",
            api_doc_label="Azure API",
            error_invalid_response="crate::AzureError::InvalidResponse",
            error_type="crate::AzureError",
            result_type="crate::Result",
        )
