#!/usr/bin/env python3
"""Generate Ruby value types, limits, vocabularies, signatures, and reference data."""

from __future__ import annotations

import argparse
import json
import keyword
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MODEL = ROOT / "spec/model.json"
LIMITS = ROOT / "spec/limits.json"
VOCABULARY = ROOT / "spec/vocabulary.json"
RUBY = ROOT / "ruby"
OUTPUTS = {
    RUBY / "lib/slackblocks/generated/models.rb",
    RUBY / "lib/slackblocks/generated/limits.rb",
    RUBY / "lib/slackblocks/generated/vocabulary.rb",
    RUBY / "lib/slackblocks/generated/style_flags.rb",
    RUBY / "sig/slackblocks/generated.rbs",
    RUBY / "generated/reference.json",
}
FIELD_KINDS = {
    "string", "boolean", "int", "long", "double", "number", "enum", "map",
    "style", "object", "list", "rows", "text", "textList", "stringList",
}
RESERVED = {
    "additional_fields", "as_json", "to_json", "to_h", "with", "members",
    "deconstruct", "deconstruct_keys", "hash", "class", "new", "type",
}


def ruby_literal(value: object) -> str:
    if value is None:
        return "nil"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    if isinstance(value, (int, float)):
        return repr(value)
    if isinstance(value, list):
        return "[" + ", ".join(ruby_literal(item) for item in value) + "]"
    if isinstance(value, dict):
        return "{" + ", ".join(
            f"{ruby_literal(key)} => {ruby_literal(item)}" for key, item in value.items()
        ) + "}"
    raise ValueError(f"Unsupported literal: {value!r}")


def ruby_name(method: str) -> str:
    name = re.sub(r"(?<!^)(?=[A-Z])", "_", method).lower()
    if not name.isidentifier() or keyword.iskeyword(name) or name in RESERVED:
        raise ValueError(f"Ruby field name collision: {method} -> {name}")
    return name


def limit_leaves(node: dict, prefix: str = "") -> list[tuple[str, int]]:
    output = []
    for key, value in node.items():
        path = f"{prefix}.{key}" if prefix else key
        if isinstance(value, dict):
            output.extend(limit_leaves(value, path))
        elif isinstance(value, int) and not isinstance(value, bool):
            output.append((path, value))
        else:
            raise ValueError(f"Unsupported limit at {path}: {value!r}")
    return sorted(output)


def field_type(field: dict, enum_values: dict[str, list[str]]) -> str:
    kind, target = field["kind"], field.get("type")
    if kind == "string":
        return "String"
    if kind == "boolean":
        return "bool"
    if kind in ("int", "long"):
        return "Integer"
    if kind in ("double", "number"):
        return "Integer | Float"
    if kind == "enum":
        return " | ".join(f":{wire}" for wire in enum_values[target])
    if kind == "map":
        return "Hash[String | Symbol, untyped]"
    if kind == "style":
        return "RichTextStyle"
    if kind == "object":
        return target
    if kind == "list":
        return f"Array[{target}]"
    if kind == "rows":
        return f"Array[Array[{target}]]"
    if kind == "text":
        return f"String | {target}"
    if kind == "textList":
        return f"Array[String | {target}]"
    if kind == "stringList":
        return "Array[String]"
    raise ValueError(f"Unsupported field kind: {kind}")


def reader_type(field: dict, enum_values: dict[str, list[str]]) -> str:
    kind = field["kind"]
    if kind == "text":
        return field["type"]
    if kind == "textList":
        return f"Array[{field['type']}]"
    if kind == "map":
        return "Hash[String, untyped]"
    return field_type(field, enum_values)


def model_outputs(model: dict) -> tuple[str, str, str]:
    interfaces = {item["name"] for item in model["interfaces"]}
    enum_values = {
        item["name"]: [constant["wire"] for constant in item["constants"]]
        for item in model["enums"]
    }
    source = [
        "# Generated from spec/model.json. Do not edit by hand.",
        "module Slackblocks",
    ]
    signatures = [
        "# Generated from spec/model.json. Do not edit by hand.",
        "module Slackblocks",
    ]
    for item in model["interfaces"]:
        name = item["name"]
        source.extend([f"  module {name}", "  end", ""])
        signatures.extend([f"  module {name}", "  end", ""])
    for item in model["enums"]:
        name = item["name"]
        source.append(f"  module {name}")
        signatures.append(f"  module {name}")
        for constant in item["constants"]:
            wire = constant["wire"]
            source.append(f"    {constant['name']} = :{wire}")
            signatures.append(f"    {constant['name']}: :{wire}")
        source.extend(["  end", ""])
        signatures.extend(["  end", ""])

    style_flags = sorted({
        flag
        for item in model["types"]
        for field in item["fields"]
        if field["kind"] == "style"
        for flag in field["flags"]
    })
    if not style_flags:
        raise ValueError("No rich text style flags found")
    reference = []
    for item in model["types"]:
        name = item["name"]
        fields = []
        names = set()
        for field in item["fields"]:
            if field["kind"] not in FIELD_KINDS:
                raise ValueError(f"Unsupported field kind: {name}.{field['method']}")
            field_name = ruby_name(field["method"])
            if field_name in names:
                raise ValueError(f"Duplicate Ruby field: {name}.{field_name}")
            names.add(field_name)
            if field.get("type") and field["type"] not in interfaces | enum_values.keys() | {
                item["name"] for item in model["types"]
            }:
                raise ValueError(f"Unknown target: {name}.{field_name}")
            fields.append({
                "name": field_name, "wire": field["wire"], "kind": field["kind"],
                "target": field.get("type"), "coerce": field.get("coerce"),
                "flags": field.get("flags"), "limits": field.get("limits"),
                "required": field.get("required", False),
            })
        defaults = {
            ruby_name(next(field["method"] for field in item["fields"] if field["wire"] == wire)): value
            for wire, value in item["defaults"].items()
        }
        for field in fields:
            if field["kind"] == "enum" and field["name"] in defaults:
                if defaults[field["name"]] not in enum_values[field["target"]]:
                    raise ValueError(f"Unknown enum default: {name}.{field['name']}")
        memberships = set(item["implements"])
        if item["package"] == "block":
            memberships.add("Block")
        if item["package"] == "element":
            memberships.add("Element")
        if "InputElement" in memberships:
            memberships.add("Element")
        if not memberships <= interfaces:
            raise ValueError(f"Unknown interface for {name}: {memberships - interfaces}")

        field_names = [field["name"] for field in fields]
        source.append(f"  class {name}")
        source.append("    include Value")
        for interface in sorted(memberships):
            source.append(f"    include {interface}")
        source.extend([
            f"    attr_reader {', '.join(':' + n for n in field_names + ['additional_fields'])}",
            "  end",
            f"  {name}.const_set(:WIRE_TYPE, {ruby_literal(item['wireType'])})",
            f"  {name}.const_set(:FIELD_SPECS, Value.deep_freeze({ruby_literal(fields)}))",
            f"  {name}.const_set(:DEFAULTS, Value.deep_freeze({ruby_literal(defaults)}))",
            f"  {name}.singleton_class.prepend(KeywordOnly)",
            "",
        ])

        signatures.append(f"  class {name}")
        signatures.append("    include Value")
        for interface in sorted(memberships):
            signatures.append(f"    include {interface}")
        required, optional = [], []
        for original, field in zip(item["fields"], fields):
            declared = field_type(original, enum_values)
            normalized = reader_type(original, enum_values)
            if field["required"] and field["name"] not in defaults:
                required.append(f"{field['name']}: {declared}")
                read_type = normalized
            else:
                optional.append(f"?{field['name']}: ({declared})?")
                read_type = f"({normalized})?"
            signatures.append(f"    attr_reader {field['name']}: {read_type}")
        signatures.append(
            "    attr_reader additional_fields: Hash[String, untyped]"
        )
        parameters = required + optional + ["?additional_fields: Hash[String | Symbol, untyped]?"]
        for method in ("initialize", "self.new", "self.[]"):
            result = "void" if method == "initialize" else name
            signatures.append(f"    def {method}: ({', '.join(parameters)}) -> {result}")
        signatures.extend(["  end", ""])
        reference.append({
            "name": name, "package": item["package"], "description": item["description"],
            "docUrl": item["docUrl"], "wireType": item["wireType"],
            "fields": [
                {"name": f["name"], "wire": f["wire"], "type": field_type(original, enum_values),
                 "readerType": reader_type(original, enum_values), "required": f["required"] and f["name"] not in defaults,
                 "description": original["description"]}
                for original, f in zip(item["fields"], fields)
            ],
        })
    source.append("end")
    signatures.append("  module StyleFlags")
    signatures.append("    NAMES: Array[String]")
    signatures.append("  end")
    signatures.append("")
    signatures.append("  class RichTextStyle")
    signatures.append("    include Value")
    for flag in style_flags:
        signatures.append(f"    attr_reader {flag}: bool?")
    signatures.append("    attr_reader additional_fields: Hash[String, untyped]")
    style_parameters = ", ".join(f"?{flag}: bool?" for flag in style_flags)
    style_parameters += ", ?additional_fields: Hash[String | Symbol, untyped]?"
    for method in ("initialize", "self.new", "self.[]"):
        result = "void" if method == "initialize" else "RichTextStyle"
        signatures.append(f"    def {method}: ({style_parameters}) -> {result}")
    signatures.extend(["  end", "end"])
    return "\n".join(source) + "\n", "\n".join(signatures) + "\n", json.dumps(
        {"types": reference}, indent=2, ensure_ascii=False
    ) + "\n"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    model = json.loads(MODEL.read_text())
    limits = json.loads(LIMITS.read_text())
    vocabulary = json.loads(VOCABULARY.read_text())
    models, signatures, reference = model_outputs(model)
    outputs = {
        RUBY / "lib/slackblocks/generated/models.rb": models,
        RUBY / "sig/slackblocks/generated.rbs": signatures,
        RUBY / "generated/reference.json": reference,
        RUBY / "lib/slackblocks/generated/limits.rb": (
            "# Generated from spec/limits.json. Do not edit by hand.\n"
            "module Slackblocks\n  module Limits\n"
            f"    VALUES = {ruby_literal(dict(limit_leaves(limits)))}.freeze\n"
            "    def self.[](path)\n      VALUES.fetch(path)\n    end\n"
            "  end\nend\n"
        ),
        RUBY / "lib/slackblocks/generated/style_flags.rb": (
            "# Generated from spec/model.json. Do not edit by hand.\n"
            "module Slackblocks\n  module StyleFlags\n"
            f"    NAMES = Value.deep_freeze({ruby_literal(sorted({flag for item in model['types'] for field in item['fields'] if field['kind'] == 'style' for flag in field['flags']}))})\n"
            "  end\nend\n"
        ),
        RUBY / "lib/slackblocks/generated/vocabulary.rb": (
            "# Generated from spec/vocabulary.json. Do not edit by hand.\n"
            "module Slackblocks\n  module Vocabulary\n"
            f"    ICON_NAMES = Value.deep_freeze({ruby_literal(vocabulary['slack_icon_names'])})\n"
            f"    SURFACE_BLOCK_TYPES = Value.deep_freeze({ruby_literal(vocabulary['surface_block_types'])})\n"
            "  end\nend\n"
        ),
    }
    assert set(outputs) == OUTPUTS
    generated_dirs = {path.parent for path in OUTPUTS}
    stale = {
        path for directory in generated_dirs for path in directory.iterdir()
        if path.is_file() and path not in OUTPUTS
    }
    if stale:
        raise SystemExit("Stale generated files: " + ", ".join(map(str, sorted(stale))))
    if args.check:
        differences = [path for path, content in outputs.items() if not path.exists() or path.read_text() != content]
        if differences:
            raise SystemExit("Generated output differs: " + ", ".join(map(str, differences)))
    else:
        for path, content in outputs.items():
            path.write_text(content)


if __name__ == "__main__":
    main()
