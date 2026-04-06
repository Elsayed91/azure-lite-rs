#!/usr/bin/env python3
"""Bootstrap a new Azure API manifest from an ARM REST spec or scratch.

Usage:
    python3 -m codegen.cli apply <service-name>
    python3 -m codegen.cli apply compute
    python3 -m codegen.cli apply storage

The manifest is written to codegen/providers/azure/manifests/<service-name>.toml.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

MANIFESTS_DIR = Path("codegen/manifests")


def _to_snake(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower()


def bootstrap(service_name: str, display_name: str = "", api_version: str = "2024-01-01") -> str:
    """Generate a draft Azure manifest for a service."""
    if not display_name:
        display_name = service_name.replace("_", " ").replace("-", " ").title()

    client_struct = "".join(w.capitalize() for w in service_name.replace("-", "_").split("_"))

    lines = [
        f"# {display_name} Azure API manifest",
        f"# ARM REST API version: {api_version}",
        "",
        "[api]",
        f'name = "{service_name}"',
        f'display_name = "{display_name}"',
        f'version = "v1"',
        f'api_version = "{api_version}"',
        f'service_name = "{service_name}"',
        f'wire_format = "rest_json"',
        f'base_url = "https://management.azure.com"',
        f'doc_url = "https://learn.microsoft.com/en-us/rest/api/azure/"',
        "",
        "[api.client]",
        f'accessor_name = "{service_name}"',
        f'client_struct = "{client_struct}Client"',
        "",
        "# === Types ===",
        "# Uncomment and fill in the types you need.",
        "# Each type maps to a Rust struct.",
        "#",
        "# [[types]]",
        '# name = "MyResource"',
        '# description = "A resource in Azure"',
        "#",
        "# [[types.fields]]",
        '# name = "id"',
        '# type = "string"',
        '# description = "Resource ID"',
        "#",
        "# [[types.fields]]",
        '# name = "name"',
        '# type = "string"',
        '# required = true',
        '# description = "Resource name"',
        "",
        "# === Operations ===",
        "# Uncomment and fill in the operations you need.",
        "#",
        "# [[operations]]",
        '# name = "ListResources"',
        '# rust_name = "list_resources"',
        '# method = "GET"',
        '# url_template = "/subscriptions/{subscriptionId}/providers/Microsoft.Foo/resources"',
        '# response_type = "ListResourcesResponse"',
        '# description = "List all resources in a subscription"',
        "",
    ]

    return "\n".join(lines)


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 -m codegen.cli apply <service-name> [display-name] [api-version]")
        sys.exit(1)

    service_name = sys.argv[1]
    display_name = sys.argv[2] if len(sys.argv) > 2 else ""
    api_version = sys.argv[3] if len(sys.argv) > 3 else "2024-01-01"

    MANIFESTS_DIR.mkdir(parents=True, exist_ok=True)
    out_path = MANIFESTS_DIR / f"{service_name}.toml"

    if out_path.exists():
        print(f"Manifest already exists: {out_path}")
        print("Use extend.py to add types/operations to an existing manifest.")
        sys.exit(1)

    output = bootstrap(service_name, display_name, api_version)
    out_path.write_text(output)
    print(f"Created manifest: {out_path}")


if __name__ == "__main__":
    main()
