#!/usr/bin/env python3
"""Generate Elixir structs directly from the shared model; format with Elixir 1.16.3."""

import argparse
import json
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
PACKAGE = ROOT / "elixir"

KINDS = {
    "boolean",
    "double",
    "enum",
    "int",
    "list",
    "long",
    "map",
    "number",
    "object",
    "rows",
    "string",
    "stringList",
    "style",
    "text",
    "textList",
}


class Expr(str):
    pass


def literal(value):
    if isinstance(value, Expr):
        return value
    if value is None:
        return "nil"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False).replace("#{", r"\#{")
    if isinstance(value, list):
        return "[" + ", ".join(map(literal, value)) + "]"
    if isinstance(value, dict):
        return (
            "%{"
            + ", ".join(literal(k) + " => " + literal(v) for k, v in value.items())
            + "}"
        )
    return f"{value:_}" if isinstance(value, int) else str(value)


def snake(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def atom(name):
    return Expr(":" + name)


def meta(**kwargs):
    return {atom(k): v for k, v in kwargs.items()}


def flat(data, prefix=""):
    out = {}
    for key, value in data.items():
        path = f"{prefix}.{key}" if prefix else key
        if isinstance(value, dict):
            out.update(flat(value, path))
        elif not key.startswith("$"):
            out[path] = value
    return out


def generate():
    model = json.loads((ROOT / "spec/model.json").read_text())
    inventory = json.loads((PACKAGE / "design/parity.json").read_text())
    names = inventory["models"]
    if set(names) != {t["name"] for t in model["types"]}:
        raise ValueError(
            "Native naming map differs from model types; review the API first"
        )
    enums = {
        e["name"]: {atom(c["wire"]): c["wire"] for c in e["constants"]}
        for e in model["enums"]
    }
    roles = {r["name"]: [] for r in model["interfaces"]}
    selected = model["types"]
    known = set(names) | set(roles) | set(enums)
    limits = flat(json.loads((ROOT / "spec/limits.json").read_text()))
    schema, files = {}, {}
    for t in selected:
        module = names[t["name"]]
        memberships = list(t["implements"])
        if t["package"] in ("block", "element"):
            memberships.append(t["package"].title())
        for role in memberships:
            roles[role].append(Expr(module))
        fields, defaults, required, specs = [], [], [], []
        docs = [
            t["description"],
            "",
            f"Slack reference: {t['docUrl']}",
            "",
            "## Fields",
            "",
        ]
        for f in t["fields"]:
            kind, key = f["kind"], f["wire"]
            if kind not in KINDS or f.get("type", "Text") not in known:
                raise ValueError(f"Unknown field kind/type: {t['name']}.{key}")
            if f.get("limits") and not any(
                k.startswith(f["limits"] + ".") for k in limits
            ):
                raise ValueError(f"Unknown limit: {f['limits']}")
            default = t["defaults"].get(key)
            if default is not None and kind == "enum":
                default = atom(default)
            fields.append(
                meta(
                    key=atom(key),
                    wire=key,
                    kind=atom(kind),
                    type=f.get("type"),
                    required=f.get("required", False),
                    default=default,
                    has_default=key in t["defaults"],
                    coerce=f.get("coerce"),
                    limits=f.get("limits"),
                    flags=f.get("flags", []),
                )
            )
            defaults.append(f"{key}: {literal(default)}")
            if f.get("required") and key not in t["defaults"]:
                required.append(atom(key))
            typ = {
                "string": "binary()",
                "boolean": "boolean()",
                "int": "integer()",
                "long": "integer()",
                "double": "number()",
                "number": "number()",
                "map": "%{optional(binary()) => Slackblocks.Types.json()}",
                "style": "Slackblocks.RichText.Style.t()",
                "stringList": "[binary()]",
            }.get(kind)
            if typ is None:
                ref = f["type"]
                typ = (
                    f"Slackblocks.Types.{snake(ref)}()"
                    if ref in roles or ref in enums
                    else f"{names[ref]}.t()"
                )
                if kind in ("list", "textList"):
                    typ = f"[{typ}]"
                elif kind == "rows":
                    typ = f"[[{typ}]]"
            if not f.get("required"):
                typ += " | nil"
            specs.append(f"{key}: {typ}")
            docs.append(
                f"* `{key}` — {f['description']}"
                + (f" Default: `{literal(default)}`." if key in t["defaults"] else "")
                + (
                    " Required."
                    if f.get("required")
                    else " Optional; `nil` omits the field."
                )
            )
        schema[Expr(module)] = meta(
            name=t["name"], wire=t["wireType"], fields=fields, roles=memberships
        )
        noargs = not required and t["name"] not in {
            "SectionBlock",
            "CardBlock",
            "ContainerBlock",
            "ImageBlock",
            "ImageElement",
            "SlackFile",
            "ConversationFilter",
            "DispatchActionConfiguration",
        }
        arg = "options \\\\ []" if noargs else "options"
        doc = "\n".join(docs).replace('"""', '\\"\\"\\"')
        content = f'''# Generated by generator/generate.py. Edit spec/model.json or the generator.
defmodule {module} do
  @moduledoc """
  {doc}
  """
  alias Slackblocks.Internal.Value
  @enforce_keys {literal(required)}
  defstruct [{", ".join(defaults)}]
  @type t :: %__MODULE__{{{", ".join(specs)}}}

  @doc "Constructs and validates a value. Text conveniences apply to keyword input."
  @spec new(keyword()) :: {{:ok, t()}} | {{:error, Slackblocks.ValidationError.t()}}
  def new({arg}), do: Value.new(__MODULE__, options)

  @doc "Constructs a value or raises `Slackblocks.ValidationError`."
  @spec new!(keyword()) :: t()
  def new!({arg}), do: Value.unwrap(new(options))

  @doc "Atomically replaces supplied fields; omitted fields retain their values."
  @spec change(t(), keyword()) :: {{:ok, t()}} | {{:error, Slackblocks.ValidationError.t()}}
  def change(value, options), do: Value.change(__MODULE__, value, options)

  @doc "Replaces fields or raises `Slackblocks.ValidationError`."
  @spec change!(t(), keyword()) :: t()
  def change!(value, options), do: Value.unwrap(change(value, options))
end
'''
        if t["name"] == "Workflow":
            extra = (PACKAGE / "generator/extra/workflow.ex.fragment").read_text()
            content = content.removesuffix("end\n") + extra + "end\n"
        path = "lib/" + "/".join(snake(x) for x in module.split(".")) + ".ex"
        if path.casefold() in {p.casefold() for p in files}:
            raise ValueError(f"Case-insensitive path collision: {path}")
        files[path] = content
    vocab = json.loads((ROOT / "spec/vocabulary.json").read_text())
    vocab.pop("$comment", None)
    files[
        "lib/slackblocks/internal/schema.ex"
    ] = f"""# Generated by generator/generate.py.
defmodule Slackblocks.Internal.Schema do
  @moduledoc false
  @models {literal(schema)}
  @roles {literal(roles)}
  @enums {literal(enums)}
  @limits {literal(limits)}
  @vocabulary {literal(vocab)}
  def models, do: @models
  def model(module), do: Map.get(@models, module)
  def roles, do: @roles
  def enums, do: @enums
  def limits, do: @limits
  def vocabulary, do: @vocabulary
end
"""
    types = [
        "@type json :: nil | boolean() | number() | binary() | [json()] | %{optional(binary()) => json()}"
    ]
    for name, members in roles.items():
        types.append(
            f"@type {snake(name)} :: "
            + (" | ".join(str(x) + ".t()" for x in members) or "none()")
        )
    for name, members in enums.items():
        types.append(f"@type {snake(name)} :: " + " | ".join(members))
    files["lib/slackblocks/types.ex"] = (
        '# Generated by generator/generate.py.\ndefmodule Slackblocks.Types do\n  @moduledoc "Role unions, closed enum atoms and JSON data types."\n  '
        + "\n  ".join(types)
        + "\nend\n"
    )
    files[
        "lib/slackblocks/internal/encoders.ex"
    ] = f"""# Generated by generator/generate.py.
defimpl Jason.Encoder, for: {literal(list(schema))} do
  def encode(value, opts), do: Jason.Encode.map(Slackblocks.to_map!(value), opts)
end
"""
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    files = generate()
    # Honor the host's external scratch volume; CI uses its normal temporary directory.
    scratch = "/Volumes/Scratch" if Path("/Volumes/Scratch").is_dir() else None
    with tempfile.TemporaryDirectory(
        prefix="slackblocks-elixir-generator-", dir=scratch
    ) as tmp:
        paths = []
        for name, content in files.items():
            p = Path(tmp) / name
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text(content)
            paths.append(str(p))
        version = subprocess.check_output(
            ["elixir", "--short-version"], text=True
        ).strip()
        if version != "1.16.3":
            raise SystemExit(f"Generation requires Elixir 1.16.3; found {version}")
        subprocess.run(["mix", "format", *paths], check=True)
        expected = {name: (Path(tmp) / name).read_text() for name in files}
    owned = {
        str(p.relative_to(PACKAGE))
        for p in (PACKAGE / "lib").rglob("*.ex")
        if p.read_text().startswith("# Generated by generator/generate.py.")
    }
    different = sorted(
        name
        for name, content in expected.items()
        if not (PACKAGE / name).exists() or (PACKAGE / name).read_text() != content
    )
    obsolete = owned - set(expected)
    if args.check and (different or obsolete):
        raise SystemExit(
            "Generated output differs: " + ", ".join(different + sorted(obsolete))
        )
    if not args.check:
        for name in obsolete:
            (PACKAGE / name).unlink()
        for name, content in expected.items():
            p = PACKAGE / name
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text(content)
    print(f"Elixir generated file set verified: {len(expected)} files")


if __name__ == "__main__":
    main()
