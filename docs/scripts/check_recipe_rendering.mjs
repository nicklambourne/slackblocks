import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import ts from "typescript";

// Exercise the actual TSX component without Docusaurus's browser-only context.
const source = await readFile(new URL("../src/components/RecipeOutput/index.tsx", import.meta.url), "utf8");
const compiled = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.CommonJS, jsx: ts.JsxEmit.React, esModuleInterop: true },
}).outputText;
const exports = {};
const dependencies = {
  react: React,
  "@theme/CodeBlock": ({ children }) => React.createElement("pre", null, children),
  "../LanguageContext": { useLanguage: () => ({ language: "python" }) },
  "./styles.module.css": new Proxy({}, { get: (_, key) => key === "__esModule" ? false : String(key) }),
};
new Function("require", "exports", compiled)((name) => {
  assert.ok(Object.hasOwn(dependencies, name), `Unexpected component dependency: ${name}`);
  return dependencies[name];
}, exports);

function freeze(value) {
  if (value && typeof value === "object") {
    Object.values(value).forEach(freeze);
    Object.freeze(value);
  }
  return value;
}

export function render(payload) {
  const before = JSON.stringify(payload);
  const html = renderToStaticMarkup(React.createElement(exports.default, {
    recipe: "card-actions", outputs: { python: freeze(payload) },
  }));
  assert.equal(JSON.stringify(payload), before, "Rendering mutated the wire payload");
  return html.split("<details>")[0]; // Exclude the serialized-output code listing.
}

const button = (id, style) => ({ type: "button", text: { type: "plain_text", text: id }, action_id: id, ...(style ? { style } : {}) });
const primary = button("Approve", "primary");
const danger = button("Reject", "danger");
const normal = button("Details");
const card = (actions) => ({ type: "card", title: { type: "plain_text", text: "Test card" }, ...(actions ? { actions } : {}) });
const labels = (html) => [...html.matchAll(/<button\b[^>]*>(.*?)<\/button>/g)].map((match) => match[1]);
function group(html, side) {
  return html.match(new RegExp(`data-card-actions="${side}"[^>]*>(.*?)</div>`))?.[1] ?? "";
}
function permutations(items) {
  return items.length ? items.flatMap((item, index) => permutations(items.filter((_, i) => i !== index)).map((rest) => [item, ...rest])) : [[]];
}

for (const pair of [[primary, danger], [normal, danger], [primary, normal]]) {
  for (const actions of permutations(pair)) {
    const html = render(card(actions));
    assert.equal(group(html, "left"), "");
    const expected = pair.includes(danger)
      ? ["Reject", pair.includes(primary) ? "Approve" : "Details"] : ["Details", "Approve"];
    assert.deepEqual(labels(group(html, "right")), expected);
  }
}
for (const actions of permutations([primary, danger, normal])) {
  const html = render(card(actions));
  assert.deepEqual(labels(group(html, "left")), ["Reject"]);
  assert.deepEqual(labels(group(html, "right")), ["Details", "Approve"]);
}
for (const actions of [undefined, [], [primary], [danger], [normal], [button("First"), button("Second")], [button("First"), button("Second"), button("Third")]]) {
  const html = render(card(actions));
  assert.equal(group(html, "left"), "");
  assert.deepEqual(labels(group(html, "right")), (actions ?? []).map((item) => item.text.text));
  if (!actions?.length) assert.ok(!html.includes("data-card-actions="));
}
// Card grouping must not leak into ordinary Actions blocks.
assert.deepEqual(labels(render({ type: "actions", elements: [primary, danger, normal] })), ["Approve", "Reject", "Details"]);
const plain = (text) => ({ type: "plain_text", text });
const richCard = render({ ...card(), subtitle: plain("Subtitle"), body: plain("Body"), subtext: plain("Small print"), hero_image: { type: "image", image_url: "https://example.com/hero.png", alt_text: "Hero" }, icon: { type: "image", image_url: "https://example.com/icon.png", alt_text: "Icon" } });
for (const expected of ["Subtitle", "Body", "Small print", 'alt="Hero"', 'alt="Icon"']) assert.ok(richCard.includes(expected));
for (const url of ["https://example.com/hero.png", "https://example.com/icon.png"]) assert.ok(richCard.includes(`src="${url}"`));
assert.ok(render({ ...card(), slack_icon: { name: "calendar" } }).includes("Slack icon: calendar"));

const literal = "Literal *stars* and `code` <safe>";
for (const field of ["title", "subtitle", "body", "subtext"]) {
  const plain = render({ ...card(), [field]: { type: "plain_text", text: literal } });
  assert.ok(plain.includes("Literal *stars* and `code` &lt;safe&gt;"), field);
  const markdown = render({ ...card(), [field]: { type: "mrkdwn", text: literal } });
  assert.ok(markdown.includes("<strong>stars</strong> and <code>code</code> &lt;safe&gt;"), field);
}
assert.ok(render({ ...card(), title: { ...plain(":calendar: *literal*"), emoji: false } }).includes(":calendar: *literal*"));
assert.deepEqual(labels(render(card([button(literal)]))), ["Literal *stars* and `code` &lt;safe&gt;"]);
assert.ok(render(card([{ ...button("Short"), accessibility_label: "Full action description" }])).includes('aria-label="Full action description"'));
for (const field of ["hero_image", "icon"]) {
  for (const slack_file of [{ id: "F01234567" }, { url: "https://files.slack.com/files-pri/T123-F123/image.png" }]) {
    const html = render({ ...card(), [field]: { type: "image", slack_file, alt_text: "Private <image>" } });
    assert.ok(!html.includes("<img"), `${field} must not emit a broken or private image request`);
    assert.ok(!html.includes("files.slack.com"));
    assert.ok(html.includes('role="img" aria-label="Private &lt;image&gt;"'));
    assert.ok(html.includes("Slack image"));
  }
}

const recipe = await readFile(new URL("../docs/recipes/card-actions.mdx", import.meta.url), "utf8");
export const outputs = JSON.parse(recipe.match(/export const recipeOutputs = (.*?);\n/s)[1]);
assert.equal(Object.keys(outputs).length, 10);
for (const [language, payload] of Object.entries(outputs)) {
  const html = render(payload);
  assert.deepEqual(labels(html), ["Reject", "Approve", "Reject", "Details", "Approve"], language);
  assert.equal((html.match(/data-card-actions="left"/g) ?? []).length, 1, language);
}
console.log("Card preview: all native outputs, action permutations, input immutability, text types, accessible labels, public/private images and ordinary Actions passed.");
