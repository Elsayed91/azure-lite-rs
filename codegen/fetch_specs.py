#!/usr/bin/env python3
"""Fetch Azure ARM REST API specs for a service.

Usage:
    python3 codegen/fetch_specs.py <service>

This script is a placeholder — Azure manifests are hand-crafted from the
ARM REST API documentation at:
  https://learn.microsoft.com/en-us/rest/api/azure/

For automated spec fetching, the Azure REST API specs are available at:
  https://github.com/Azure/azure-rest-api-specs

To use the GitHub repo specs:
  git clone https://github.com/Azure/azure-rest-api-specs
  # Then reference the OpenAPI specs in specification/<service>/
"""
from __future__ import annotations

import sys


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 codegen/fetch_specs.py <service>")
        sys.exit(1)

    service = sys.argv[1]
    print(f"Azure REST specs for '{service}' are available at:")
    print(f"  https://learn.microsoft.com/en-us/rest/api/{service}/")
    print("")
    print("ARM REST specs on GitHub:")
    print(f"  https://github.com/Azure/azure-rest-api-specs/tree/main/specification/{service}")
    print("")
    print("For cloud-lite, manifests are hand-crafted from the documentation.")
    print(f"Run: python3 codegen/bootstrap.py {service}")


if __name__ == "__main__":
    main()
