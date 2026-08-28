import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import ts from "typescript";

const SOURCE_FILES = [
  "../../typescript/src/fluent/blocks.ts",
  "../../typescript/src/fluent/components.ts",
  "../../typescript/src/fluent/elements.ts",
  "../../typescript/src/fluent/objects.ts",
  "../../typescript/src/fluent/payloads.ts",
  "../../typescript/src/builder.ts",
  "../../typescript/src/types.ts",
  "../../typescript/src/validation.ts",
  "../../typescript/src/errors.ts",
];

const FLUENT_FILES = new Set(
  SOURCE_FILES.filter((sourcePath) => sourcePath.includes("/fluent/")),
);

const failures = [];

function hasText(comment) {
  return (ts.getTextOfJSDocComment(comment) ?? "").trim().length > 0;
}

function hasSummary(node) {
  return (node.jsDoc ?? []).some((documentation) => hasText(documentation.comment));
}

function requireDocumentation(node, location) {
  if (!hasSummary(node)) failures.push(`${location}: missing API description`);
}

function inspectType(type, location) {
  if (!type) return;
  if (ts.isParenthesizedTypeNode(type)) {
    inspectType(type.type, location);
    return;
  }
  if (ts.isUnionTypeNode(type) || ts.isIntersectionTypeNode(type)) {
    type.types.forEach((nested) => inspectType(nested, location));
    return;
  }
  if (!ts.isTypeLiteralNode(type)) return;

  for (const member of type.members) {
    if (!ts.isPropertySignature(member)) continue;
    const name = member.name.getText();
    requireDocumentation(member, `${location}.${name}`);
    inspectType(member.type, `${location}.${name}`);
  }
}

function inspectParameters(node, location) {
  for (const parameter of node.parameters) {
    const name = parameter.name.getText();
    const tags = ts.getJSDocParameterTags(parameter);
    if (tags.length === 0 || tags.every((tag) => !hasText(tag.comment))) {
      failures.push(`${location}(${name}): missing @param description`);
    }
    inspectType(parameter.type, `${location}.${name}`);
  }
}

function inspectFunction(node, location, requireThrows) {
  requireDocumentation(node, location);
  inspectParameters(node, location);

  const isAssertion =
    node.type &&
    ts.isTypePredicateNode(node.type) &&
    node.type.assertsModifier !== undefined;
  const returns = ts.getJSDocReturnTag(node);
  if (!isAssertion && (returns === undefined || !hasText(returns.comment))) {
    failures.push(`${location}: missing @returns description`);
  }

  if (requireThrows) {
    const throwsTags = ts.getJSDocTags(node).filter(ts.isJSDocThrowsTag);
    if (throwsTags.length === 0 || throwsTags.every((tag) => !hasText(tag.comment))) {
      failures.push(`${location}: missing @throws description`);
    }
  }
}

function inspectInterface(node, location) {
  requireDocumentation(node, location);
  for (const member of node.members) {
    if (!ts.isPropertySignature(member) && !ts.isIndexSignatureDeclaration(member)) continue;
    requireDocumentation(member, `${location}.${member.name?.getText() ?? "[key]"}`);
    if (ts.isPropertySignature(member)) inspectType(member.type, `${location}.${member.name.getText()}`);
  }
}

function inspectClass(node, location) {
  requireDocumentation(node, location);
  for (const member of node.members) {
    if (
      !ts.isConstructorDeclaration(member) &&
      !ts.isPropertyDeclaration(member) &&
      !ts.isMethodDeclaration(member)
    ) {
      continue;
    }
    const name = ts.isConstructorDeclaration(member)
      ? "constructor"
      : member.name.getText();
    requireDocumentation(member, `${location}.${name}`);
    if (ts.isConstructorDeclaration(member) || ts.isMethodDeclaration(member)) {
      inspectParameters(member, `${location}.${name}`);
    }
  }
}

function isExported(node) {
  return node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword) === true;
}

for (const sourcePath of SOURCE_FILES) {
  const filePath = fileURLToPath(new URL(sourcePath, import.meta.url));
  const source = ts.createSourceFile(
    filePath,
    readFileSync(filePath, "utf8"),
    ts.ScriptTarget.Latest,
    true,
  );

  for (const statement of source.statements) {
    if (!isExported(statement) || !statement.name) continue;
    const location = `${sourcePath}:${statement.name.getText()}`;
    if (ts.isFunctionDeclaration(statement)) {
      if (FLUENT_FILES.has(sourcePath)) {
        requireDocumentation(statement, location);
        if (!/^[A-Z]/.test(statement.name.getText())) {
          failures.push(`${location}: fluent builders must use PascalCase`);
        }
      } else {
        inspectFunction(statement, location, false);
      }
    } else if (ts.isInterfaceDeclaration(statement)) inspectInterface(statement, location);
    else if (ts.isClassDeclaration(statement)) inspectClass(statement, location);
    else if (ts.isTypeAliasDeclaration(statement)) {
      requireDocumentation(statement, location);
      inspectType(statement.type, location);
    }
  }
}

if (failures.length > 0) {
  console.error(failures.join("\n"));
  process.exitCode = 1;
} else {
  console.log("TypeScript API documentation covers every public declaration and input.");
}
