#!/usr/bin/env python3
"""Adapt Ruby's generated naming/type metadata to the shared reference renderer."""

import html
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ruby = json.loads((ROOT / "ruby/generated/reference.json").read_text())
model = json.loads((ROOT / "spec/model.json").read_text())
version_source = (ROOT / "ruby/lib/slackblocks/version.rb").read_text()
version = re.search(r'VERSION = "([^"]+)"', version_source).group(1)
spec_version = re.search(r'SPEC_VERSION = "([^"]+)"', version_source).group(1)


def prose(value):
    return (
        html.escape(value, quote=False)
        .replace("{", "&#123;")
        .replace("}", "&#125;")
        .replace("|", "\\|")
        .replace("\n", " ")
    )


def member(name, signature, doc, params=None, returns=None, throws=None):
    return {
        "name": name,
        "signature": signature,
        "parameterTypes": "",
        "doc": prose(doc),
        "params": [
            {"name": parameter["name"], "doc": prose(parameter["description"])}
            for parameter in (params or [])
        ],
        "returns": prose(returns or ""),
        "throws": throws or [],
    }


def type_entry(name, package, doc, members=None, constants=None, see=None):
    return {
        "name": name,
        "package": package,
        "doc": prose(doc),
        "members": members or [],
        "constants": constants or [],
        "see": see or [],
    }


types = []
for entry in ruby["types"]:
    name = entry["name"]
    fields = entry["fields"]
    parameters = [
        field["name"] + ": " + field["type"]
        if field["required"]
        else "?" + field["name"] + ": (" + field["type"] + ")?"
        for field in fields
    ]
    parameters.append("?additional_fields: Hash[String | Symbol, untyped]?")
    required = [field["name"] for field in fields if field["required"]]
    note = " Required keywords: " + ", ".join(required) + "." if required else ""
    constructor = "(" + ", ".join(parameters) + ") -> " + name
    members = [
        member(
            ".new",
            "def self.new: " + constructor,
            "Construct an immutable, validated value with keyword arguments." + note,
            fields
            + [
                {
                    "name": "additional_fields",
                    "description": "JSON-compatible extension fields that do not replace modeled wire keys.",
                }
            ],
            name,
            [
                {
                    "type": "ValidationError",
                    "doc": "A field or cross-field Slack rule fails.",
                }
            ],
        ),
        member(
            ".[]",
            "def self.[]: " + constructor,
            "Keyword-only alias for .new.",
            returns=name,
        ),
    ]
    for field in fields:
        read_type = (
            field["readerType"]
            if field["required"]
            else "(" + field["readerType"] + ")?"
        )
        members.append(
            member(
                field["name"],
                "def " + field["name"] + ": () -> " + read_type,
                field["description"] + " Wire key: " + field["wire"] + ".",
                returns=read_type,
            )
        )
    members.extend(
        [
            member(
                "additional_fields",
                "def additional_fields: () -> Hash[String, untyped]",
                "Return frozen extension fields with normalized string keys.",
            ),
            member(
                "with",
                "def with: (**untyped changes) -> " + name,
                "Return a newly validated value with changed keywords.",
                returns=name,
            ),
            member(
                "to_h",
                "def to_h: () -> Hash[Symbol, untyped]",
                "Return structural Ruby members, including additional_fields. Nested members remain frozen.",
            ),
            member(
                "as_json",
                "def as_json: (?untyped options) -> Hash[String, untyped]",
                "Return a fresh, detached Slack wire Hash.",
            ),
            member(
                "to_json",
                "def to_json: (?untyped options) -> String",
                "Encode the Slack wire payload as JSON.",
            ),
        ]
    )
    types.append(
        type_entry(
            name,
            entry["package"],
            entry["description"],
            members,
            see=[{"label": "Slack reference", "url": entry["docUrl"]}]
            if entry["docUrl"]
            else [],
        )
    )

for interface in model["interfaces"]:
    description = (
        "A validated Slack layout block accepted by payload block arrays."
        if interface["name"] == "Block"
        else interface["description"]
    )
    types.append(type_entry(interface["name"], interface["package"], description))

for enum in model["enums"]:
    types.append(
        type_entry(
            enum["name"],
            enum["package"],
            enum["description"] + " Pass the wire symbol to constructor keywords.",
            constants=[
                {
                    "name": constant["name"],
                    "wire": ":" + constant["wire"],
                    "doc": prose(constant["description"]),
                }
                for constant in enum["constants"]
            ],
        )
    )

flags = sorted(
    {
        flag
        for entry in model["types"]
        for field in entry["fields"]
        if field["kind"] == "style"
        for flag in field["flags"]
    }
)
types.append(
    type_entry(
        "RichTextStyle",
        "object",
        "Immutable rich text style flags. The owning rich text element limits which flags may be set.",
        [
            member(
                ".new",
                "def self.new: ("
                + ", ".join("?" + flag + ": bool?" for flag in flags)
                + ", ?additional_fields: Hash[String | Symbol, untyped]?) -> RichTextStyle",
                "Create a style with permitted boolean flags.",
                [
                    {"name": flag, "description": "Optional style flag."}
                    for flag in flags
                ],
                "RichTextStyle",
            ),
            member(
                ".[]",
                "def self.[]: ("
                + ", ".join("?" + flag + ": bool?" for flag in flags)
                + ", ?additional_fields: Hash[String | Symbol, untyped]?) -> RichTextStyle",
                "Keyword-only alias for .new.",
                returns="RichTextStyle",
            ),
            *[
                member(
                    flag,
                    "def " + flag + ": () -> bool?",
                    "Read the optional style flag.",
                    returns="bool?",
                )
                for flag in flags
            ],
            member(
                "additional_fields",
                "def additional_fields: () -> Hash[String, untyped]",
                "Return frozen extension style fields.",
            ),
            member(
                "with",
                "def with: (**untyped changes) -> RichTextStyle",
                "Return a validated style with changed keywords.",
            ),
            member(
                "to_h",
                "def to_h: () -> Hash[Symbol, untyped]",
                "Return structural style members.",
            ),
            member(
                "as_json",
                "def as_json: (?untyped options) -> Hash[String, untyped]",
                "Return a detached wire Hash.",
            ),
            member(
                "to_json",
                "def to_json: (?untyped options) -> String",
                "Return JSON for this style.",
            ),
        ],
    )
)

types.extend(
    [
        type_entry(
            "AccordionSection",
            "component",
            "Create one Slack-native collapsible container.",
            [
                member(
                    ".create",
                    "def self.create: (title: String | PlainText, blocks: Array[Block], ?subtitle: String | Text?, ?icon: ImageElement?, ?expanded: bool, ?width: :narrow | :standard | :wide | :full, ?has_header_divider: bool, ?block_id: String?) -> ContainerBlock",
                    "Return a validated collapsible container. The title remains visible when collapsed.",
                    returns="ContainerBlock",
                )
            ],
        ),
        type_entry(
            "Accordion",
            "component",
            "Combine collapsible sections in display order.",
            [
                member(
                    ".create",
                    "def self.create: (sections: Array[ContainerBlock]) -> Array[Block]",
                    "Return an immutable block collection. Every section must be a collapsible container.",
                    returns="Array[Block]",
                )
            ],
        ),
        type_entry(
            "Paginator",
            "component",
            "Select one page of blocks and append navigation controls when multiple pages exist.",
            [
                member(
                    ".create",
                    "def self.create: (action_id_prefix: String, blocks: Array[Block], ?page: Integer, ?page_size: Integer, ?previous_text: String, ?next_text: String, ?show_page_indicator: bool, ?block_id: String?) -> Array[Block]",
                    "Pages are one-based. Previous and next button action IDs append .previous and .next to the prefix.",
                    returns="Array[Block]",
                )
            ],
        ),
        type_entry(
            "Value",
            "core",
            "Shared immutable value protocol used by every Slackblocks value.",
            [
                member(
                    "additional_fields",
                    "def additional_fields: () -> Hash[String, untyped]",
                    "Return frozen extension fields.",
                ),
                member(
                    "with",
                    "def with: (**untyped changes) -> self",
                    "Return a validated value with changed keywords.",
                ),
                member(
                    "==",
                    "def ==: (untyped other) -> bool",
                    "Compare values structurally.",
                ),
                member(
                    "eql?",
                    "def eql?: (untyped other) -> bool",
                    "Compare values structurally for Hash keys.",
                ),
                member(
                    "hash",
                    "def hash: () -> Integer",
                    "Return a structural hash for Hash keys.",
                ),
                member(
                    "to_h",
                    "def to_h: () -> Hash[Symbol, untyped]",
                    "Return structural Ruby members.",
                ),
                member(
                    "as_json",
                    "def as_json: (?untyped options) -> Hash[String, untyped]",
                    "Return a detached wire Hash.",
                ),
                member(
                    "to_json",
                    "def to_json: (?untyped options) -> String",
                    "Return Slack JSON.",
                ),
            ],
        ),
        type_entry(
            "Slackblocks",
            "core",
            "The gem namespace and its version constants.",
            constants=[
                {
                    "name": "VERSION",
                    "wire": version,
                    "doc": "Coordinated package version.",
                },
                {
                    "name": "SPEC_VERSION",
                    "wire": spec_version,
                    "doc": "Shared contract version.",
                },
            ],
        ),
        type_entry(
            "ValidationError",
            "error",
            "Raised when a Slack rule fails. category is one of six stable wire names; path identifies the offending value.",
            [
                member(
                    "category",
                    "def category: () -> String",
                    "Validation category, such as length-exceeded or type-mismatch.",
                ),
                member(
                    "path",
                    "def path: () -> String",
                    "Root value type followed by wire field names and array indexes.",
                ),
            ],
        ),
    ]
)

print(json.dumps({"types": types}, ensure_ascii=False))
