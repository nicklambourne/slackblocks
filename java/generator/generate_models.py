#!/usr/bin/env python3
"""Generate concrete Java Slack object values and fluent builders."""

from __future__ import annotations

import json
import re
import shutil
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GO = ROOT / "go"
JAVA = ROOT / "java"
OUTPUT = JAVA / "src/main/java/io/github/nicklambourne/slackblocks"

ELEMENT_TYPES = {
    "Button", "FeedbackButtons", "IconButton", "Checkboxes", "DatePicker",
    "DateTimePicker", "EmailInput", "FileInput", "ImageElement",
    "ChannelMultiSelect", "ConversationMultiSelect", "ExternalMultiSelect",
    "StaticMultiSelect", "UserMultiSelect", "NumberInput", "Overflow",
    "PlainTextInput", "RadioButtons", "ChannelSelect", "ConversationSelect",
    "ExternalSelect", "StaticSelect", "UserSelect", "TimePicker", "URLInput",
    "WorkflowButton", "RichTextInput",
}

RENAMES = {
    "Markdown": "MarkdownText",
    "Message": "MessagePayload",
    "Modal": "ModalView",
    "HomeTab": "HomeTabView",
    "RichText": "RichTextText",
}

STRING_METHODS = {
    "BlockID", "URL", "Name", "ID", "SlackFileID", "SlackFileURL", "Align",
    "XLabel", "YLabel", "ChannelID", "UserID", "UserGroupID", "ActionID",
    "AccessibilityLabel", "AltText", "AuthorName", "Caption", "Channel",
    "CallbackID", "Color", "ExternalID", "Fallback", "ImageURL", "InitialChannel",
    "InitialConversation", "InitialDate", "InitialTime", "InitialUser", "Level",
    "PrivateMetadata", "ProviderIconURL", "ProviderName", "ResponseType", "Source",
    "Status", "TaskID", "ThumbnailURL", "Timezone", "TitleURL", "VideoURL", "Width",
}

BOOLEAN_METHODS = {
    "Emoji", "Verbatim", "ExcludeExternalSharedChannels", "ExcludeBotUsers",
    "Wrap", "IsWrapped", "ClearOnClose", "Code", "DefaultCollapsed", "Expanded",
    "DefaultToCurrentConversation", "DeleteOriginal", "DispatchAction",
    "FocusOnLoad", "HasHeaderDivider", "Highlight", "IsCollapsible",
    "IsDecimalAllowed", "Italic", "Mrkdwn", "Multiline", "NotifyOnClose",
    "Optional", "ReplaceOriginal", "ResponseURLEnabled", "Strike",
    "SubmitDisabled", "UnfurlLinks", "UnfurlMedia", "Unlink", "Unsafe", "Bold",
}

INTEGER_METHODS = {
    "Border", "Offset", "Indent", "SkinTone", "MaxFiles", "MaxLength",
    "MaxLines", "MaxSelectedItems", "MinLength", "MinLines", "MinQueryLength",
    "PageSize", "RowHeaderColumnIndex",
}
DOUBLE_METHODS = {"MaxValue", "MinValue"}
LONG_METHODS = {"InitialDateTime"}
STRING_LIST_METHODS = {
    "Include", "TriggerActionsOn", "Categories", "Filetypes", "InitialChannels",
    "InitialConversations", "InitialUsers", "VisibleToUserIDs",
}

SPECIAL_OBJECTS = {"FeedbackButton", "URLSource"}
TEXT_CLASSES = {"PlainText", "Markdown"}


@dataclass(frozen=True)
class Definition:
    constructor: str
    go_name: str
    java_name: str
    package: str
    wire_type: str
    methods: tuple[str, ...]
    description: str
    coercions: dict[str, str]
    defaults: dict[str, object]


def java_name(go_name: str, source: str) -> str:
    name = RENAMES.get(go_name, go_name).replace("URL", "Url")
    if source == "elements.go" and go_name in ELEMENT_TYPES and not name.endswith("Element"):
        name += "Element"
    return name


def lower_camel(name: str) -> str:
    return name.replace("URL", "Url").replace("ID", "Id")[:1].lower() + name.replace("URL", "Url").replace("ID", "Id")[1:]


def wire_field(method: str, fields_source: str) -> tuple[str, str]:
    match = re.search(
        rf"func \(b \*builder\) {re.escape(method)}\([^)]*\) \*builder \{{(.*?)(?=\nfunc |\Z)",
        fields_source,
        re.S,
    )
    if not match:
        raise RuntimeError(f"Cannot locate Go field method {method}")
    body = match.group(1)
    field_match = re.search(r'b\.(?:set|append|appendNested)\("([^"]+)"', body)
    if not field_match:
        raise RuntimeError(f"Cannot locate wire field for {method}")
    operation = "append" if "b.append(" in body else "appendNested" if "b.appendNested(" in body else "set"
    return field_match.group(1), operation


def parse_definitions() -> list[Definition]:
    method_manifest = json.loads((GO / "internal/builder_methods.json").read_text())
    definitions: list[Definition] = []
    for source_name, default_package in (
        ("objects.go", "object"),
        ("elements.go", "element"),
        ("blocks.go", "block"),
        ("payloads.go", "payload"),
    ):
        source = (GO / source_name).read_text()
        starts = list(re.finditer(r"func (New\w+)\(", source))
        for index, start in enumerate(starts):
            constructor = start.group(1)
            if constructor not in method_manifest:
                continue
            end = starts[index + 1].start() if index + 1 < len(starts) else len(source)
            body = source[start.start():end]
            builder = re.search(r'(?:newBuilder|elementBuilder)\("([^"]+)",\s*"([^"]*)"\)', body)
            if not builder:
                raise RuntimeError(f"Cannot locate builder metadata for {constructor}")
            go_name, wire_type = builder.groups()
            package = "object" if go_name in SPECIAL_OBJECTS else default_package
            comment = re.search(rf"// {re.escape(constructor)} (.*?)\nfunc {re.escape(constructor)}", source, re.S)
            description = (
                " ".join(line.removeprefix("//").strip() for line in comment.group(1).splitlines())
                if comment else f"Creates a {go_name} Slack object."
            )
            description = description[:1].upper() + description[1:]
            coercions = {
                field: kind for field, kind in re.findall(
                    r'(?:\.\s*)?coerce\("([^"]+)",\s*(plainTextLike|markdownLike)\)', body
                )
            }
            if "elementBuilder(" in body:
                coercions.setdefault("placeholder", "plainTextLike")
            defaults: dict[str, object] = {}
            for field, literal in re.findall(
                r'(?:builder\.)?values\["([^"]+)"\]\s*=\s*(true|false|\d+|"[^"]*")', body
            ):
                if literal == "true":
                    value: object = True
                elif literal == "false":
                    value = False
                elif literal.startswith('"'):
                    value = json.loads(literal)
                else:
                    value = int(literal)
                defaults[field] = value
            definitions.append(Definition(
                constructor, go_name, java_name(go_name, source_name), package,
                wire_type, tuple(method_manifest[constructor]), description,
                coercions, defaults,
            ))
    missing = set(method_manifest) - {item.constructor for item in definitions}
    missing.discard("NewAccordionSection")
    if missing:
        raise RuntimeError(f"Missing definitions: {sorted(missing)}")
    return sorted(definitions, key=lambda item: (item.package, item.java_name))


def method_source(definition: Definition, method: str, fields_source: str) -> str:
    field, operation = wire_field(method, fields_source)
    java_method = lower_camel(method)
    coercion = definition.coercions.get(field)
    lines: list[str] = []

    def scalar(parameter: str, statement: str, detail: str) -> None:
        lines.extend([
            "    /**", f"     * {detail}", "     *",
            f"     * @param value value for Slack's {{@code {field}}} field",
            "     * @return this builder", "     */",
            f"    public Builder {java_method}({parameter} value) {{",
            f"      {statement}", "      return this;", "    }", "",
        ])

    if method in BOOLEAN_METHODS:
        scalar("boolean", f'state.set("{field}", value);', f"Sets whether {field.replace('_', ' ')} is enabled.")
    elif method in INTEGER_METHODS:
        scalar("int", f'state.set("{field}", value);', f"Sets {field.replace('_', ' ')}.")
    elif method in LONG_METHODS:
        scalar("long", f'state.set("{field}", value);', f"Sets {field.replace('_', ' ')}.")
    elif method in DOUBLE_METHODS:
        scalar("double", f'state.set("{field}", value);', f"Sets {field.replace('_', ' ')}.")
    elif method in STRING_METHODS:
        scalar("String", f'state.set("{field}", Objects.requireNonNull(value, "{java_method}"));', f"Sets {field.replace('_', ' ')}.")
    elif method in STRING_LIST_METHODS:
        lines.extend([
            "    /**", f"     * Adds values to Slack's {{@code {field}}} field in order.", "     *",
            "     * @param values values to append", "     * @return this builder", "     */",
            f"    public Builder {java_method}(String... values) {{",
            f'      state.append("{field}", (Object[]) Objects.requireNonNull(values, "{java_method}"));',
            "      return this;", "    }", "",
        ])
    elif method == "Rows":
        lines.extend([
            "    /**", "     * Adds one or more complete table rows in display order.", "     *",
            "     * @param rows rows whose entries are Slack cell values",
            "     * @return this builder", "     */", "    @SafeVarargs", "    @SuppressWarnings({\"varargs\", \"unchecked\"})",
            f"    public final Builder {java_method}(List<? extends SlackObject>... rows) {{",
            f'      state.append("{field}", (Object[]) Objects.requireNonNull(rows, "{java_method}"));',
            "      return this;", "    }", "",
        ])
    elif operation == "append":
        lines.extend([
            "    /**", f"     * Adds values to Slack's {{@code {field}}} field in order.", "     *",
            "     * @param values Slack values to append", "     * @return this builder", "     */",
            f"    public Builder {java_method}(SlackObject... values) {{",
            f'      state.append("{field}", (Object[]) Objects.requireNonNull(values, "{java_method}"));',
            "      return this;", "    }", "",
        ])
        if method == "Fields":
            lines.extend([
                "    /**", "     * Adds Markdown section fields in display order.", "     *",
                "     * @param values Markdown strings to append", "     * @return this builder", "     */",
                "    public Builder markdownFields(String... values) {",
                '      Objects.requireNonNull(values, "values");',
                "      for (String value : values) {",
                '        state.append("fields", WireObjects.text("mrkdwn", Objects.requireNonNull(value, "field")));',
                "      }", "      return this;", "    }", "",
            ])
    else:
        if method == "Value" and definition.go_name in {"RawNumber", "ChartSegment", "DataPoint"}:
            scalar("double", f'state.set("{field}", value);', "Sets the numeric value.")
        elif method == "Value":
            scalar("String", f'state.set("{field}", Objects.requireNonNull(value, "{java_method}"));', "Sets the application-defined value.")
        else:
            expression = (
                f'WireObjects.text("plain_text", Objects.requireNonNull(value, "{java_method}"))'
                if coercion == "plainTextLike"
                else f'WireObjects.text("mrkdwn", Objects.requireNonNull(value, "{java_method}"))'
                if coercion == "markdownLike"
                else f'Objects.requireNonNull(value, "{java_method}")'
            )
            scalar("String", f'state.set("{field}", {expression});', f"Sets {field.replace('_', ' ')} from a string.")
            lines.extend([
                "    /**", f"     * Sets Slack's {{@code {field}}} field from a typed Slack value.", "     *",
                "     * @param value nested Slack value", "     * @return this builder", "     */",
                f"    public Builder {java_method}(SlackObject value) {{",
                f'      state.set("{field}", Objects.requireNonNull(value, "{java_method}"));',
                "      return this;", "    }", "",
            ])
    if method == "Text" and coercion in {"plainTextLike", "markdownLike"}:
        alias = "plainText" if coercion == "plainTextLike" else "markdownText"
        kind = "plain text" if coercion == "plainTextLike" else "Slack Markdown"
        lines.extend([
            "    /**", f"     * Sets the text using {kind}.", "     *",
            "     * @param value text content", "     * @return this builder", "     */",
            f"    public Builder {alias}(String value) {{",
            f"      return {java_method}(value);", "    }", "",
        ])
    return "\n".join(lines).rstrip()


def convenience_source(definition: Definition) -> str:
    name = definition.java_name
    lines: list[str] = []
    if definition.go_name in {"PlainText", "Markdown", "RawText"}:
        lines += [
            "  /**", "   * Creates and validates this text value in one call.", "   *",
            "   * @param text text content", f"   * @return an immutable {name}", "   */",
            f"  public static {name} of(String text) {{", "    return builder().text(text).build();", "  }", "",
        ]
    if definition.go_name == "HeaderBlock":
        lines += [
            "  /**", "   * Starts a header builder with its required text.", "   *",
            "   * @param text header text", "   * @return a new builder", "   */",
            "  public static Builder builder(String text) {", "    return builder().text(text);", "  }", "",
        ]
    if definition.go_name == "Button":
        lines += [
            "  /**", "   * Starts a button builder with its required label and action identifier.", "   *",
            "   * @param text button label", "   * @param actionId identifier returned in interaction payloads",
            "   * @return a new builder", "   */",
            "  public static Builder builder(String text, String actionId) {",
            "    return builder().text(text).actionId(actionId);", "  }", "",
        ]
    if definition.go_name == "Option":
        lines += [
            "  /**", "   * Starts an option builder with its required label and value.", "   *",
            "   * @param text visible option label", "   * @param value application-defined value",
            "   * @return a new builder", "   */",
            "  public static Builder builder(String text, String value) {",
            "    return builder().text(text).value(value);", "  }", "",
        ]
    if definition.go_name == "ImageBlock":
        lines += [
            "  /**", "   * Starts an image builder with its required URL and alternative text.", "   *",
            "   * @param imageUrl public image URL", "   * @param altText accessible image description",
            "   * @return a new builder", "   */",
            "  public static Builder builder(String imageUrl, String altText) {",
            "    return builder().imageUrl(imageUrl).altText(altText);", "  }", "",
        ]
    if definition.go_name == "DividerBlock":
        lines += [
            "  /**", "   * Creates an unconfigured divider block.", "   *",
            "   * @return an immutable divider", "   */", f"  public static {name} create() {{",
            "    return builder().build();", "  }", "",
        ]
    return "\n".join(lines).rstrip()


def class_source(definition: Definition, fields_source: str) -> str:
    package = f"io.github.nicklambourne.slackblocks.{definition.package}"
    imports = {
        "com.google.gson.annotations.JsonAdapter",
        "io.github.nicklambourne.slackblocks.Buildable",
        "io.github.nicklambourne.slackblocks.SlackObject",
        "io.github.nicklambourne.slackblocks.internal.BuilderState",
        "io.github.nicklambourne.slackblocks.internal.SlackObjectJsonAdapter",
        "io.github.nicklambourne.slackblocks.internal.WireObjects",
        "java.util.List", "java.util.Map", "java.util.Objects",
    }
    if definition.package == "block":
        declaration = f"public final class {definition.java_name} implements Block"
    elif definition.package == "element":
        imports.add("com.slack.api.model.block.element.BlockElement")
        declaration = f"public final class {definition.java_name} extends BlockElement implements Element"
    elif definition.go_name in TEXT_CLASSES:
        imports.add("com.slack.api.model.block.composition.TextObject")
        declaration = f"public final class {definition.java_name} extends TextObject implements SlackObject"
    else:
        declaration = f"public final class {definition.java_name} implements SlackObject"

    defaults = []
    for field, value in definition.defaults.items():
        literal = str(value).lower() if isinstance(value, bool) else json.dumps(value) if isinstance(value, str) else str(value)
        defaults.append(f'      state.set("{field}", {literal});')
    methods = "\n\n".join(method_source(definition, method, fields_source) for method in definition.methods)
    convenience = convenience_source(definition)
    extra_methods = ""
    if definition.go_name in TEXT_CLASSES:
        extra_methods = """
  /**
   * Returns the Slack text representation type.
   *
   * @return Slack wire type
   */
  @Override
  public String getType() {
    return (String) values.get("type");
  }

  /**
   * Returns the text content.
   *
   * @return text content
   */
  @Override
  public String getText() {
    return (String) values.get("text");
  }
"""
    transform = ""
    if definition.go_name == "Attachment":
        transform = """
      Object color = state.get("color");
      if (color instanceof String text
          && text.length() == 6
          && !text.equals("danger")
          && !text.equals("good")
          && !text.equals("warning")) {
        state.set("color", "#" + text);
      }
"""
    return f"""package {package};

{chr(10).join(f'import {item};' for item in sorted(imports))}

/**
 * {definition.description}
 *
 * @see <a href="https://docs.slack.dev/reference/block-kit">Slack Block Kit reference</a>
 */
@JsonAdapter(SlackObjectJsonAdapter.class)
{declaration} {{
  /** Immutable builder snapshot. */
  private final Map<String, Object> values;

  /** Stores one validated builder snapshot. */
  private {definition.java_name}(Map<String, Object> values) {{
    this.values = values;
  }}

  /**
   * Starts a new concrete fluent builder.
   *
   * @return a new builder
   */
  public static Builder builder() {{
    return new Builder();
  }}

{convenience}
  /** {{@inheritDoc}} */
  @Override
  public Map<String, Object> toMap() {{
    return WireObjects.materialize(values);
  }}
{extra_methods}
  /** {{@inheritDoc}} */
  @Override
  public boolean equals(Object other) {{
    return this == other || (other instanceof {definition.java_name} that && toMap().equals(that.toMap()));
  }}

  /** {{@inheritDoc}} */
  @Override
  public int hashCode() {{
    return toMap().hashCode();
  }}

  /** {{@inheritDoc}} */
  @Override
  public String toString() {{
    return toJson();
  }}

  /** Concrete fluent builder for {{@link {definition.java_name}}}. */
  public static final class Builder implements Buildable<{definition.java_name}> {{
    /** Mutable state isolated to this builder. */
    private final BuilderState state = new BuilderState("{definition.go_name}", "{definition.wire_type}");

    /** Creates an empty builder with Slack defaults applied. */
    private Builder() {{
{chr(10).join(defaults)}
    }}

{methods}
    /**
     * Sets a forward-compatible wire field that does not yet have a named fluent method.
     * Prefer the named methods for normal Block Kit use.
     *
     * @param field Slack JSON field name
     * @param value wire-compatible value
     * @return this builder
     */
    public Builder wireField(String field, Object value) {{
      if (field.isEmpty()) {{
        throw new IllegalArgumentException("field must not be empty");
      }}
      state.set(field, Objects.requireNonNull(value, "value"));
      return this;
    }}

    /** {{@inheritDoc}} */
    @Override
    public {definition.java_name} build() {{
{transform}      return state.build({definition.java_name}::new);
    }}
  }}
}}
"""


def package_info(package: str) -> str:
    return f"""/** Immutable Slack {package.replace('_', ' ')} values and concrete fluent builders. */
@NullMarked
package io.github.nicklambourne.slackblocks.{package};

import org.jspecify.annotations.NullMarked;
"""


def main() -> None:
    fields_source = (GO / "fields.go").read_text()
    definitions = parse_definitions()
    for package in ("block", "element", "object", "payload"):
        directory = OUTPUT / package
        if directory.exists():
            shutil.rmtree(directory)
        directory.mkdir(parents=True)
        (directory / "package-info.java").write_text(package_info(package))

    (OUTPUT / "block/Block.java").write_text(
        """package io.github.nicklambourne.slackblocks.block;

import com.slack.api.model.block.LayoutBlock;
import io.github.nicklambourne.slackblocks.SlackObject;
import org.jspecify.annotations.Nullable;

/** A validated Slack layout block accepted directly by Slack's official Java SDK. */
public interface Block extends LayoutBlock, SlackObject {
  /**
   * Returns the Slack block type.
   *
   * @return Slack wire type
   */
  @Override
  default String getType() {
    return (String) toMap().get("type");
  }

  /**
   * Returns the configured block identifier.
   *
   * @return block identifier, or {@code null} when none was set
   */
  @Override
  @Nullable
  default String getBlockId() {
    return (String) toMap().get("block_id");
  }
}
"""
    )
    (OUTPUT / "element/Element.java").write_text(
        """package io.github.nicklambourne.slackblocks.element;

import io.github.nicklambourne.slackblocks.SlackObject;

/** A validated interactive, visual, or input element nested inside a Slack block. */
public interface Element extends SlackObject {}
"""
    )
    for definition in definitions:
        (OUTPUT / definition.package / f"{definition.java_name}.java").write_text(class_source(definition, fields_source))
    (JAVA / "generator/generated-files.txt").write_text(
        "\n".join(f"{item.package}/{item.java_name}.java" for item in definitions) + "\n"
    )
    print(f"Generated {len(definitions)} concrete Java value types.")


if __name__ == "__main__":
    main()
