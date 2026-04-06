#!/usr/bin/env python3
"""Extend an existing Azure manifest with new types and operations.

Usage:
    python3 codegen/extend.py <service>
    python3 codegen/extend.py compute --available-types
    python3 codegen/extend.py storage
"""
from __future__ import annotations

import sys
import tomllib
from pathlib import Path

MANIFESTS_DIR = Path("codegen/manifests")


def show_available_types(service_name: str) -> None:
    """Show current types and operations in a manifest."""
    manifest_path = MANIFESTS_DIR / f"{service_name}.toml"
    if not manifest_path.exists():
        print(f"Manifest not found: {manifest_path}")
        print(f"Run: python3 codegen/bootstrap.py {service_name}")
        sys.exit(1)

    with open(manifest_path, "rb") as f:
        manifest = tomllib.load(f)

    api = manifest.get("api", {})
    print(f"=== {api.get('display_name', service_name)} manifest ===")
    print(f"API version: {api.get('api_version', 'unknown')}")
    print()

    types = manifest.get("types", [])
    print(f"Types ({len(types)}):")
    for t in types:
        fields = t.get("fields", [])
        print(f"  {t['name']} ({len(fields)} fields)")
    print()

    operations = manifest.get("operations", [])
    print(f"Operations ({len(operations)}):")
    for op in operations:
        rust_name = op.get("rust_name", op.get("name", "?"))
        method = op.get("method", "GET")
        url = op.get("url_template", "")
        print(f"  {rust_name}: {method} {url}")


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 codegen/extend.py <service> [--available-types]")
        sys.exit(1)

    service_name = sys.argv[1]
    show_types = "--available-types" in sys.argv

    if show_types:
        show_available_types(service_name)
        return

    manifest_path = MANIFESTS_DIR / f"{service_name}.toml"
    if not manifest_path.exists():
        print(f"Manifest not found: {manifest_path}")
        print(f"Run: python3 codegen/bootstrap.py {service_name}")
        sys.exit(1)

    print(f"To extend the {service_name} manifest, edit:")
    print(f"  {manifest_path}")
    print()
    print("Add new [[types]] and [[operations]] sections to the TOML file.")
    print("Then run: python3 -m codegen.cli apply")


if __name__ == "__main__":
    main()
