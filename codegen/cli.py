"""Azure codegen CLI.

Usage:
    python3 -m codegen.cli apply
    python3 -m codegen.cli apply --api compute
    python3 -m codegen.cli apply --dry-run
    python3 -m codegen.cli generate
"""
from __future__ import annotations
import argparse, sys
from pathlib import Path
from cloud_lite_codegen.emitter import RustEmitter
from cloud_lite_codegen.apply import apply
from codegen.plugin import AzurePlugin

def _resolve_provider(plugin, api):
    if api:
        manifest_path = Path("codegen") / "manifests" / f"{api}.toml"
        if not manifest_path.exists():
            print(f"ERROR: Manifest not found: {manifest_path}", file=sys.stderr); sys.exit(1)
        api_def = plugin.resolve(str(manifest_path))
        full_provider = plugin.resolve_all()
        full_provider.apis = [api_def]
        return full_provider
    return plugin.resolve_all()

def cmd_generate(args):
    plugin = AzurePlugin()
    provider_def = _resolve_provider(plugin, args.api)
    output_dir = str(Path("codegen") / "generated")
    RustEmitter(output_dir).emit(provider_def)
    print(f"Generated to: {output_dir}")

def cmd_apply(args):
    plugin = AzurePlugin()
    provider_def = _resolve_provider(plugin, args.api)
    output_dir = str(Path("codegen") / "generated")
    RustEmitter(output_dir).emit(provider_def)
    actions = apply(output_dir, plugin.target_crate(), dry_run=args.dry_run)
    for action in actions: print(f"  {action}")
    print("Done!")

def main():
    parser = argparse.ArgumentParser(prog="azure-codegen")
    sub = parser.add_subparsers(dest="command")
    gen_p = sub.add_parser("generate"); gen_p.add_argument("--api")
    apply_p = sub.add_parser("apply"); apply_p.add_argument("--api"); apply_p.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    if args.command == "generate": cmd_generate(args)
    elif args.command == "apply": cmd_apply(args)
    else: parser.print_help(); sys.exit(1)

if __name__ == "__main__": main()
