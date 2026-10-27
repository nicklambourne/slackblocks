import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const languages = ["python", "typescript", "go", "java", "csharp", "ruby", "rust", "php", "elixir", "haskell"];
const source = await readFile(path.join(root, "docs/reference/index.mdx"), "utf8");
const sharedIndex = source.split("<LanguageContent>")[0];
const html = await readFile(path.join(root, "build/reference.html"), "utf8");
for (const language of languages) {
  assert.ok(sharedIndex.includes(`](/reference/${language})`), `${language}: current link must be visible outside the language switcher`);
  assert.ok(html.includes(`href="/slackblocks/reference/${language}"`), `${language}: rendered current link missing`);
  const target = await readFile(path.join(root, `build/reference/${language}.html`), "utf8");
  assert.match(target, /<h1[^>]*>/, `${language}: target must be a rendered reference page`);
}
const recipeIndex = await readFile(path.join(root, "build/recipes.html"), "utf8");
for (const recipe of ["multiselect-compatibility", "workflow-state", "reliable-delivery", "live-prerequisites"]) {
  assert.ok(recipeIndex.includes(`href="/slackblocks/recipes/${recipe}"`), `${recipe}: missing discovery link`);
  await readFile(path.join(root, `build/recipes/${recipe}.html`), "utf8");
}
const phpObjects = await readFile(path.join(root, "build/reference/php/objects.html"), "utf8");
assert.match(phpObjects, /id="richtextusergroup"/);
assert.match(phpObjects, /userGroupId/);
console.log("Current ten-language references and executable recipe routes are discoverable.");
