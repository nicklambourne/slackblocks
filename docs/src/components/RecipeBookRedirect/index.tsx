import { useEffect } from "react";
import { useHistory, useLocation } from "@docusaurus/router";
import recipes from "@site/examples/recipes/catalog.json";

// Bookmarks from the former single-page cookbook still reach their recipe.
export default function RecipeBookRedirect() {
  const location = useLocation();
  const history = useHistory();
  useEffect(() => {
    const anchor = location.hash.slice(1).replace(/-\d+$/, "");
    const recipe = recipes.find(({ anchors }) => anchors.includes(anchor));
    if (recipe) history.replace(`${location.pathname.replace(/\/$/, "")}/${recipe.id}${location.search}`);
  }, [history, location.pathname, location.search, location.hash]);
  return null;
}
