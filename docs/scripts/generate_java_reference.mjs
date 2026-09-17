// Generates the Java API reference with the javadoc tool.
//
// javadoc parses java/src/main/java, resolves types and {@link} references, and runs
// java-reference/ReferenceDoclet.java, which writes the public API as JSON. This script renders
// that JSON as one MDX page per API area. It needs a JDK (17+) on PATH or in JAVA_HOME; Maven
// resolves the library's compile classpath through the checked-in wrapper.

import { execFileSync } from "node:child_process";
import { mkdir, readFile, rm } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { writeReferencePages } from "./reference_pages.mjs";

const scriptsDirectory = path.dirname(fileURLToPath(import.meta.url));
const repository = path.resolve(scriptsDirectory, "../..");
const javaRoot = path.join(repository, "java");
const outputRoot = path.join(repository, "docs/docs/reference/java");
const workDirectory = path.join(javaRoot, "target/reference");
const basePackage = "io.github.nicklambourne.slackblocks";

const domains = [
  { slug: "blocks", title: "Blocks", position: 1, packages: ["block"] },
  { slug: "elements", title: "Elements", position: 2, packages: ["element"] },
  { slug: "objects", title: "Composition Objects", position: 3, packages: ["object"] },
  { slug: "payloads", title: "Payloads And Views", position: 4, packages: ["payload"] },
  { slug: "components", title: "Components", position: 5, packages: ["component"] },
  { slug: "core", title: "Core Types", position: 6, packages: [""], names: ["Buildable", "SlackObject", "Slackblocks", "SlackblocksJson"] },
  { slug: "errors", title: "Validation And Errors", position: 7, packages: [""], names: ["ErrorCategory", "ValidationException"] },
];

function tool(name) {
  const home = process.env.JAVA_HOME;
  return home ? path.join(home, "bin", name) : name;
}

function run(command, args, options = {}) {
  try {
    return execFileSync(command, args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], ...options });
  } catch (error) {
    const detail = [error.stdout, error.stderr].filter(Boolean).join("\n");
    throw new Error(`${path.basename(command)} ${args.join(" ")} failed. A JDK 17 or newer must be on PATH or in JAVA_HOME.\n${detail}`);
  }
}

async function javadocJson() {
  await rm(workDirectory, { recursive: true, force: true });
  await mkdir(workDirectory, { recursive: true });
  const classpathFile = path.join(workDirectory, "classpath.txt");
  run(path.join(javaRoot, process.platform === "win32" ? "mvnw.cmd" : "mvnw"), [
    "-B", "-ntp", "-q", "dependency:build-classpath",
    `-Dmdep.outputFile=${classpathFile}`, "-Dmdep.includeScope=compile",
  ], { cwd: javaRoot });
  const classpath = (await readFile(classpathFile, "utf8")).trim();
  const docletClasses = path.join(workDirectory, "doclet");
  run(tool("javac"), ["-d", docletClasses, path.join(scriptsDirectory, "java-reference/ReferenceDoclet.java")]);
  const output = path.join(workDirectory, "reference.json");
  run(tool("javadoc"), [
    "-quiet",
    "-doclet", "ReferenceDoclet",
    "-docletpath", docletClasses,
    "-classpath", classpath,
    "-sourcepath", path.join(javaRoot, "src/main/java"),
    "-subpackages", basePackage,
    "-exclude", `${basePackage}.internal`,
    "-o", output,
  ]);
  return JSON.parse(await readFile(output, "utf8"));
}

function domainFor(type) {
  const relative = type.package === basePackage ? "" : type.package.slice(basePackage.length + 1);
  return domains.find((domain) => domain.packages.includes(relative) && (!domain.names || domain.names.includes(type.name)));
}

function introduction(domain) {
  let text = `This page documents the public Java API for ${domain.title.toLowerCase()}, generated from the Javadoc by the javadoc tool. Values are immutable, builders are concrete and fluent, validation runs when you call \`.build()\`, and typed getters read built values back.\n\n`;
  if (domain.slug === "blocks") {
    text += "Built block values implement Slack's official `LayoutBlock` interface, so they can be passed directly to `ChatPostMessageRequest.blocks(...)` and other slack-java-sdk request builders.\n\n";
  }
  if (domain.slug === "components") {
    text += "Components expand to ordinary `List<Block>` values. The application remains responsible for channel selection, delivery options, and interaction handling through slack-java-sdk.\n\n";
  }
  return text;
}

const index = `---
sidebar_position: 0
---

# Java API reference

This is the complete guide to slackblocks' public Java API, generated from its Javadoc by the javadoc tool. Use it to find concrete fluent builder methods, typed getters, immutable return types, validation failures, payloads, and higher-level components.

Build a block with a concrete builder, call \`.build()\`, then pass the resulting value directly to Slack's official Java SDK. \`ChatPostMessageRequest\` keeps the channel, fallback text, threading, and delivery options where Java Slack developers already expect them.

- [Blocks](/reference/java/blocks)
- [Elements](/reference/java/elements)
- [Composition Objects](/reference/java/objects)
- [Payloads And Views](/reference/java/payloads)
- [Components](/reference/java/components)
- [Core Types](/reference/java/core)
- [Validation And Errors](/reference/java/errors)
`;

const count = await writeReferencePages({
  reference: await javadocJson(),
  domains,
  domainFor,
  language: "java",
  fence: "java",
  outputRoot,
  introduction,
  index,
});
console.log(`Generated ${count} Java API types with javadoc.`);
