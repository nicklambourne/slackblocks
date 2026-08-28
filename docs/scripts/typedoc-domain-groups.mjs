import { MarkdownPageEvent } from "typedoc-plugin-markdown";
import { fileURLToPath } from "node:url";
import path from "node:path";
import ts from "typescript";

const DOMAIN_TITLES = {
  blocks: "Blocks",
  components: "Components",
  elements: "Elements",
  errors: "Errors",
  objects: "Composition Objects",
  payloads: "Payloads",
  utilities: "Utilities",
};

const fluentSetters = new Map();
const fluentNames = new Set();
const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const typescriptDirectory = path.resolve(scriptsDirectory, "../../typescript");
const fluentSourceFiles = [
  "blocks.ts",
  "components.ts",
  "elements.ts",
  "objects.ts",
  "payloads.ts",
].map((file) => path.join(typescriptDirectory, "src/fluent", file));

function fluentTerminology(value) {
  let result = value;
  for (const name of fluentNames) {
    const legacyName = name[0].toLowerCase() + name.slice(1);
    result = result.replaceAll(`\`${legacyName}\``, `\`${name}()\``);
  }
  return result;
}

function markdownType(value) {
  return value
    .replace(/import\("[^"]+"\)\./g, "")
    .replaceAll("|", "\\|");
}

function collectFluentSetters() {
  const configPath = path.join(typescriptDirectory, "tsconfig.typedoc.json");
  const config = ts.readConfigFile(configPath, ts.sys.readFile);
  if (config.error) {
    throw new Error(
      ts.flattenDiagnosticMessageText(config.error.messageText, "\n"),
    );
  }

  const parsed = ts.parseJsonConfigFileContent(
    config.config,
    ts.sys,
    typescriptDirectory,
    undefined,
    configPath,
  );
  const program = ts.createProgram(parsed.fileNames, parsed.options);
  const checker = program.getTypeChecker();
  const functions = fluentSourceFiles.flatMap((file) => {
    const source = program.getSourceFile(file);
    if (!source) throw new Error(`TypeScript API source is missing: ${file}`);
    return source.statements.filter(
      (statement) =>
        ts.isFunctionDeclaration(statement) &&
        statement.name &&
        statement.modifiers?.some(
          (modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword,
        ),
    );
  });

  for (const declaration of functions) fluentNames.add(declaration.name.text);

  for (const declaration of functions) {
    if (
      !declaration.type ||
      !ts.isTypeReferenceNode(declaration.type) ||
      !["FluentBuilder", "FluentGroupBuilder"].includes(
        declaration.type.typeName.getText(),
      )
    ) {
      continue;
    }

    const inputNode = declaration.type.typeArguments?.[0];
    if (!inputNode) continue;
    const input = checker.getTypeFromTypeNode(inputNode);
    const fields = checker.getPropertiesOfType(input);
    if (fields.length === 0) continue;

    fluentSetters.set(
      declaration.name.text,
      fields.map((field) => {
        const location =
          field.valueDeclaration ?? field.declarations?.[0] ?? inputNode;
        const type = checker.getTypeOfSymbolAtLocation(field, location);
        const nonNullableType = checker.getNonNullableType(type);
        return {
          collection:
            checker.isArrayType(nonNullableType) ||
            checker.isTupleType(nonNullableType) ||
            nonNullableType.symbol?.name === "ReadonlyArray",
          description: fluentTerminology(
            ts.displayPartsToString(field.getDocumentationComment(checker)),
          ),
          name: field.name,
          optional: (field.flags & ts.SymbolFlags.Optional) !== 0,
          type: markdownType(
            checker.typeToString(type, location, ts.TypeFormatFlags.NoTruncation),
          ),
        };
      }),
    );
  }
}

function renderGenericSyntax(contents) {
  const escapedContents = contents
    .replaceAll("\\<", "&lt;")
    .replaceAll("\\>", "&gt;");
  const lines = escapedContents.split("\n");

  for (let index = 0; index < lines.length - 2; index += 1) {
    const heading = lines[index].match(/^## ([A-Za-z_$][\w$]*)$/);
    if (!heading || lines[index + 1] !== "") continue;

    const name = heading[1];
    const signature = lines[index + 2].match(
      new RegExp(`^> \\*\\*${name}\\*\\*&lt;(.+?)&gt;`),
    );
    if (!signature) continue;

    const typeParameters = signature[1].replaceAll("`", "");
    lines[index] =
      `## ${name}&lt;${typeParameters}&gt; {#${name.toLowerCase()}}`;
  }

  return lines.join("\n");
}

function tableAt(lines, start) {
  if (!lines[start]?.startsWith("| ")) return undefined;
  let end = start;
  while (lines[end]?.startsWith("| ")) end += 1;
  return { start, end, lines: lines.slice(start, end) };
}

function inputTypeFields(contents, typeName) {
  const lines = contents.split("\n");
  const sectionStart = lines.findIndex((line) => line === `## ${typeName}`);
  if (sectionStart === -1) return [];

  const sectionEnd = lines.findIndex(
    (line, index) => index > sectionStart && line === "***",
  );
  const propertiesHeading = lines.findIndex(
    (line, index) =>
      index > sectionStart &&
      (sectionEnd === -1 || index < sectionEnd) &&
      line === "### Properties",
  );
  if (propertiesHeading === -1) return [];

  const table = tableAt(lines, propertiesHeading + 2);
  if (!table || table.lines[0] !== "| Property | Type | Description |") {
    return [];
  }

  return table.lines.slice(2).map((line) =>
    line.replace(/^\| <a id="[^"]+"><\/a> /, "| "),
  );
}

function renderFactoryInputs(contents) {
  const lines = contents.split("\n");

  for (let index = 0; index < lines.length; index += 1) {
    if (!/^## [A-Za-z_$][\w$]*\(\)$/.test(lines[index])) continue;

    const sectionEnd = lines.findIndex(
      (line, nestedIndex) => nestedIndex > index && line === "***",
    );
    const parametersHeading = lines.findIndex(
      (line, nestedIndex) =>
        nestedIndex > index &&
        (sectionEnd === -1 || nestedIndex < sectionEnd) &&
        line === "### Parameters",
    );
    if (parametersHeading === -1) continue;

    const parameters = tableAt(lines, parametersHeading + 2);
    if (
      !parameters ||
      parameters.lines[0] !== "| Parameter | Type | Description |"
    ) {
      continue;
    }

    const rows = parameters.lines.slice(2);
    let fieldRoot;
    let inputRow;
    let fields = [];

    for (const row of rows) {
      const parameter = row.match(/^\| `([A-Za-z_$][\w$]*)` \|/)?.[1];
      if (!parameter || parameter === "settings") continue;

      fields = rows
        .filter((line) => line.startsWith(`| \`${parameter}.`))
        .map((line) => line.replace(`| \`${parameter}.`, "| `"));
      if (fields.length === 0) {
        const typeName = row.match(/\[`([^`]+)`\]\(#[^)]+\)/)?.[1];
        if (typeName) fields = inputTypeFields(contents, typeName);
      }
      if (fields.length === 0) continue;

      fieldRoot = parameter;
      inputRow = row;
      break;
    }

    if (!fieldRoot || !inputRow) continue;

    const inputDescription = inputRow.match(/ \| ([^|]+) \|$/)?.[1];
    const remainingRows = rows.filter(
      (line) => line !== inputRow && !line.startsWith(`| \`${fieldRoot}.`),
    );
    const replacement = [
      "### Input fields",
      "",
      ...(inputDescription ? [inputDescription, ""] : []),
      "| Input field | Type | Description |",
      "| ------ | ------ | ------ |",
      ...fields,
    ];

    if (remainingRows.length > 0) {
      replacement.push(
        "",
        "### Settings",
        "",
        "| Setting | Type | Description |",
        "| ------ | ------ | ------ |",
        ...remainingRows,
      );
    }

    lines.splice(
      parametersHeading,
      parameters.end - parametersHeading,
      ...replacement,
    );
    index += replacement.length - 1;
  }

  return lines.join("\n");
}

function renderFluentSetters(contents) {
  const lines = contents.split("\n");

  for (let index = 0; index < lines.length; index += 1) {
    const functionName = lines[index].match(/^## ([A-Z][A-Za-z0-9_$]*)\(\)$/)?.[1];
    const fields = fluentSetters.get(functionName);
    if (!fields) continue;

    const sectionEnd = lines.findIndex(
      (line, nestedIndex) => nestedIndex > index && line === "***",
    );
    const returnsHeading = lines.findIndex(
      (line, nestedIndex) =>
        nestedIndex > index &&
        (sectionEnd === -1 || nestedIndex < sectionEnd) &&
        line === "### Returns",
    );
    if (returnsHeading === -1) continue;

    const hasCollections = fields.some(({ collection }) => collection);
    const replacement = [
      "### Chainable setters",
      "",
      "Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.",
      ...(hasCollections
        ? [
            "Collection setters accept individual values, nested builders, or arrays and append each call.",
          ]
        : []),
      "",
      "| Setter | Value type | Required | Description |",
      "| ------ | ------ | ------ | ------ |",
      ...fields.map(
        ({ collection, description, name, optional, type }) =>
          `| \`.${name}(${collection ? "...values" : "value"})\` | \`${type}\` | ${optional ? "No" : "Yes"} | ${description || "—"} |`,
      ),
      "",
      "### Validation and errors",
      "",
      "`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.",
      "",
    ];

    lines.splice(returnsHeading, 0, ...replacement);
    index += replacement.length;
  }

  return lines.join("\n");
}

/** Flatten TypeDoc's kind-based index groups inside each domain. */
export function load(app) {
  collectFluentSetters();

  app.converter.on(
    "resolveEnd",
    (context) => {
      const entryPoints = Object.values(context.project.reflections).filter(
        (reflection) => reflection.parent?.isProject(),
      );

      for (const group of context.project.groups ?? []) {
        group.title = "none";
      }

      for (const reflection of entryPoints) {
        const [group, ...remainingGroups] = reflection.groups ?? [];
        if (!group) continue;

        group.title = "none";
        group.children = [group, ...remainingGroups]
          .flatMap((item) => item.children)
          .sort((left, right) =>
            left.name.localeCompare(right.name, "en", { sensitivity: "base" }),
          );
        group.categories = undefined;
        reflection.groups = [group];
      }
    },
    -1000,
  );

  app.renderer.on(MarkdownPageEvent.END, (page) => {
    page.contents = renderGenericSyntax(page.contents);
    page.contents = renderFactoryInputs(page.contents);
    page.contents = renderFluentSetters(page.contents);

    if (page.url === "objects.md") {
      page.contents = page.contents.replace(
        "## Markdown()",
        '<a id="mrkdwn"></a>\n\n## Markdown()',
      );
    }

    if (page.url !== "index.md") {
      page.contents = [
        "---",
        "toc_min_heading_level: 2",
        "toc_max_heading_level: 2",
        "---",
        "",
        page.contents,
      ].join("\n");
      return;
    }

    for (const [slug, title] of Object.entries(DOMAIN_TITLES)) {
      page.contents = page.contents.replace(
        `- [${slug}](${slug}.md)`,
        `- [${title}](${slug}.md)`,
      );
    }
  });
}
