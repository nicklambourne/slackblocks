// Frozen releases contain links to the current Recipe Book. Update their
// rendered destination without rewriting the archived Markdown sources.
module.exports = function recipeBookLinks() {
  return function walk(node) {
    if (node.type === "link") {
      node.url = node.url.replace(
        /^(?:\/slackblocks\/|\/)?usage\/cookbook(?=[?#]|$)/,
        "/usage/recipes",
      );
    }
    for (const child of node.children ?? []) walk(child);
  };
};
