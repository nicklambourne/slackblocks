import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const buildArgument = process.argv.indexOf("--build-dir");
const buildRoot = buildArgument < 0 ? path.join(root, "build") : path.resolve(process.argv[buildArgument + 1]);
const languages = ["python", "typescript", "go", "java", "csharp", "ruby", "rust", "php", "elixir", "haskell"];
const source = await readFile(path.join(root, "docs/reference/index.mdx"), "utf8");
const sharedIndex = source.split("<LanguageContent>")[0];
const html = await readFile(path.join(buildRoot, "reference.html"), "utf8");
for (const language of languages) {
  assert.ok(sharedIndex.includes(`](/reference/${language})`), `${language}: current link must be visible outside the language switcher`);
  assert.ok(html.includes(`href="/slackblocks/reference/${language}"`), `${language}: rendered current link missing`);
  const target = await readFile(path.join(buildRoot, `reference/${language}.html`), "utf8");
  assert.match(target, /<h1[^>]*>/, `${language}: target must be a rendered reference page`);
}
const recipeBook = await readFile(path.join(buildRoot, "usage/recipes.html"), "utf8");
assert.match(recipeBook, /<h1[^>]*>Recipe Book<\/h1>/);
assert.doesNotMatch(recipeBook, /Recipes and operational guides|Find a recipe/);
const recipes = JSON.parse(await readFile(path.join(root, "examples/recipes/catalog.json"), "utf8"));
for (const recipe of [...recipes.map(({ id }) => id), "multiselect-compatibility", "workflow-state", "reliable-delivery", "live-prerequisites"]) {
  assert.ok(recipeBook.includes(`href="/slackblocks/usage/recipes/${recipe}"`), `${recipe}: missing Recipe Book link`);
  await readFile(path.join(buildRoot, `usage/recipes/${recipe}.html`), "utf8");
}
for (const language of languages) {
  const indexes = JSON.parse(await readFile(path.join(buildRoot, `search-index-${language}.json`), "utf8"));
  const urls = indexes.flatMap(({ documents }) => documents.map(({ u }) => u));
  assert.ok(urls.includes("/slackblocks/usage/recipes"), `${language}: Recipe Book missing from search`);
  for (const { id } of recipes) assert.ok(urls.includes(`/slackblocks/usage/recipes/${id}`), `${language}: ${id} missing from search`);
  assert.ok(!urls.some((url) => /^\/slackblocks\/(?:recipes|usage\/cookbook)(?:[\/#]|$)/.test(url)), `${language}: obsolete recipes route in current search`);
}
const phpObjects = await readFile(path.join(buildRoot, "reference/php/objects.html"), "utf8");
assert.match(phpObjects, /id="richtextusergroup"/);
assert.match(phpObjects, /userGroupId/);
console.log("Current ten-language references and executable recipe routes are discoverable.");
