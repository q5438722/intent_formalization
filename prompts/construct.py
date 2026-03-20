"""
Construct a prompt from a YAML config file.

Usage:
    python construct.py <config.yml> [-o output_file]

The config YAML should have:
  - template: path to a markdown template with {{placeholder}} markers
  - components: dict of placeholder_name -> file_path or inline value
All file paths are resolved relative to the workspace root (spec/).
"""

import argparse
import os
import re
import yaml


def load_config(config_path: str) -> dict:
    with open(config_path, "r") as f:
        return yaml.safe_load(f)


def resolve_path(rel_path: str, base_dir: str) -> str:
    """Resolve a path relative to the base directory."""
    return os.path.normpath(os.path.join(base_dir, rel_path))


def load_component(value: str, base_dir: str) -> str:
    """Load a component: if it looks like a file path and exists, read it; otherwise return as-is."""
    if isinstance(value, str) and ("/" in value or value.endswith((".md", ".rs", ".txt"))):
        path = resolve_path(value, base_dir)
        if os.path.isfile(path):
            with open(path, "r") as f:
                return f.read().strip()
    return str(value)


def construct_prompt(config_path: str) -> str:
    config = load_config(config_path)
    # Paths in the YAML are relative to the spec/ folder (parent of prompts/).
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    # Load template
    template_path = resolve_path(config["template"], base_dir)
    with open(template_path, "r") as f:
        template = f.read()

    # Find which placeholders are actually used in the template
    used_keys = set(re.findall(r"\{\{(\w+)\}\}", template))

    # Only load components that are referenced in the template
    components = config.get("components", {})
    resolved = {}
    for key, value in components.items():
        if key in used_keys:
            resolved[key] = load_component(value, base_dir)

    # Substitute {{key}} placeholders in the template
    def replacer(match):
        key = match.group(1).strip()
        if key in resolved:
            return resolved[key]
        return match.group(0)  # leave unresolved placeholders as-is

    prompt = re.sub(r"\{\{(\w+)\}\}", replacer, template)
    return prompt


def main():
    parser = argparse.ArgumentParser(description="Construct a prompt from a YAML config.")
    parser.add_argument("config", help="Path to the YAML config file")
    parser.add_argument("-o", "--output", help="Output file (default: stdout)", default=None)
    args = parser.parse_args()

    prompt = construct_prompt(args.config)

    if args.output:
        with open(args.output, "w") as f:
            f.write(prompt)
        print(f"Prompt written to {args.output}")
    else:
        print(prompt)


if __name__ == "__main__":
    main()
