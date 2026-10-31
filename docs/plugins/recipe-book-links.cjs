const { existsSync } = require("node:fs");
const path = require("node:path");

// Normalize old recipe URLs without rewriting the archived Markdown sources.
// Where a frozen recipe exists, a relative source link lets Docusaurus resolve
// that version's route instead of silently taking the reader to current docs.
module.exports = function recipeBookLinks() {
  return function transform(tree, file) {
    const source = file?.path;
    const snapshot = source?.match(/^(.*[/\\]versioned_docs[/\\]version-[^/\\]+)[/\\]/)?.[1];
    function walk(node) {
      if (node.type === "link") {
        node.url = node.url
          .replace(/^(?:\/slackblocks\/|\/)?usage\/cookbook(?=[?#]|$)/, "/usage/recipes")
          .replace(/^(?:\/slackblocks\/|\/)?recipes(?=[/?#]|$)/, "/usage/recipes");
        const recipe = node.url.match(/^\/(?:slackblocks\/)?usage\/recipes(?:\/([a-z0-9-]+))?([?#].*)?$/);
        if (snapshot && recipe) {
          const target = path.join(snapshot, recipe[1] ? `recipes/${recipe[1]}.mdx` : "usage/cookbook.mdx");
          if (existsSync(target)) {
            let relative = path.relative(path.dirname(source), target).split(path.sep).join("/");
            if (!relative.startsWith(".")) relative = `./${relative}`;
            node.url = relative + (recipe[2] ?? "");
          }
        }
      }
      for (const child of node.children ?? []) walk(child);
    }
    walk(tree);
  };
};
