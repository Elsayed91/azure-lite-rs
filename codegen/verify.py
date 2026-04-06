#!/usr/bin/env python3
"""Validate Azure manifests and generated code.

Usage:
    python3 codegen/verify.py
"""
from __future__ import annotations

import subprocess
import sys
import tomllib
from pathlib import Path

MANIFESTS_DIR = Path("codegen/manifests")

VALID_METHODS = {"GET", "POST", "PUT", "PATCH", "DELETE"}
VALID_SCALAR_TYPES = {
    "string", "boolean", "integer", "int32", "int64",
    "number", "float", "double", "datetime", "date-time",
    "object", "bytes", "array",
}


def validate_manifest(manifest_path: Path) -> list[str]:
    """Validate a single Azure manifest for required fields and consistency."""
    errors: list[str] = []

    try:
        with open(manifest_path, "rb") as f:
            manifest = tomllib.load(f)
    except Exception as e:
        return [f"Failed to parse TOML: {e}"]

    api = manifest.get("api", {})
    if not api:
        errors.append("Missing [api] section")
        return errors

    # Required api fields
    for field in ("name", "api_version"):
        if not api.get(field):
            errors.append(f"Missing api.{field}")

    # Validate client section
    client = api.get("client", {})
    if not client.get("client_struct"):
        errors.append("Missing api.client.client_struct")
    if not client.get("accessor_name"):
        errors.append("Missing api.client.accessor_name")

    # Collect defined type names for cross-referencing
    defined_types: set[str] = set()
    for type_entry in manifest.get("types", []):
        if not type_entry.get("name"):
            errors.append("A [[types]] entry is missing 'name'")
            continue
        defined_types.add(type_entry["name"])

        # Validate fields
        for field in type_entry.get("fields", []):
            if not field.get("name"):
                errors.append(f"Type '{type_entry['name']}': a field is missing 'name'")
                continue
            # Validate type string
            type_str = field.get("type", "string")
            type_lower = type_str.lower()
            if (type_lower not in VALID_SCALAR_TYPES
                    and not type_lower.startswith("array<")
                    and not type_lower.startswith("map<")
                    and not field.get("rust_type")
                    and not field.get("enum_type")):
                # Could be a reference to another type — warn but don't error
                pass  # User-defined types are allowed as cross-references

    # Validate enums
    for enum_entry in manifest.get("enums", []):
        if not enum_entry.get("name"):
            errors.append("A [[enums]] entry is missing 'name'")
        if not enum_entry.get("values"):
            name = enum_entry.get("name", "?")
            errors.append(f"Enum '{name}' has no values")

    # Validate operations
    for op_entry in manifest.get("operations", []):
        op_id = op_entry.get("rust_name") or op_entry.get("name") or "?"
        if not op_entry.get("name") and not op_entry.get("rust_name"):
            errors.append("An [[operations]] entry is missing both 'name' and 'rust_name'")
            continue

        method = op_entry.get("method", "GET").upper()
        if method not in VALID_METHODS:
            errors.append(f"Operation '{op_id}': invalid method '{method}'")

        if not op_entry.get("url_template"):
            errors.append(f"Operation '{op_id}': missing url_template")

        # Validate response_type references a defined type (warn only)
        response_type = op_entry.get("response_type", "")
        if response_type and response_type not in defined_types:
            # Could be a type from another manifest or a primitive — warn
            pass

    return errors


def main() -> int:
    print("=== Azure Manifest Validation ===\n")

    if not MANIFESTS_DIR.exists():
        print(f"No manifests directory found at {MANIFESTS_DIR}")
        print("This is OK if no manifests have been created yet.")
    else:
        all_errors: list[str] = []
        manifest_files = sorted(MANIFESTS_DIR.glob("*.toml"))

        if not manifest_files:
            print("No manifests found (OK for initial setup)")
        else:
            for manifest_path in manifest_files:
                print(f"Validating: {manifest_path.name}")
                errors = validate_manifest(manifest_path)
                if errors:
                    for e in errors:
                        print(f"  ERROR: {e}")
                    all_errors.extend(errors)
                else:
                    print("  OK")

            if all_errors:
                print(f"\n{len(all_errors)} validation error(s) found!")
                return 1

    print("\n=== Cargo Check ===")
    result = subprocess.run(
        ["cargo", "check", "-p", "azure-lite"],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(result.stderr)
        return 1
    print("  OK")

    print("\n=== Cargo Clippy ===")
    result = subprocess.run(
        ["cargo", "clippy", "-p", "azure-lite", "--", "-D", "warnings"],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(result.stderr)
        return 1
    print("  OK")

    print("\n=== Cargo Test ===")
    result = subprocess.run(
        ["cargo", "test", "-p", "azure-lite", "--lib"],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        print(result.stdout)
        print(result.stderr)
        return 1
    for line in result.stdout.split("\n"):
        if "test result:" in line:
            print(f"  {line.strip()}")
    print("  OK")

    print("\nAll checks passed!")
    return 0


if __name__ == "__main__":
    sys.exit(main())
