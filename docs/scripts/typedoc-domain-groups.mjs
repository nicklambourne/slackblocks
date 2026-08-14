import { MarkdownPageEvent } from "typedoc-plugin-markdown";

const DOMAIN_TITLES = {
  blocks: "Blocks",
  elements: "Elements",
  errors: "Errors",
  messages: "Messages",
  objects: "Composition Objects",
  "rich-text": "Rich Text",
  utilities: "Utilities",
  views: "Views",
};

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

/** Flatten TypeDoc's kind-based index groups inside each domain. */
export function load(app) {
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
