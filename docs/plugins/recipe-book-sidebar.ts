import type { Options } from "@docusaurus/plugin-content-docs";

// Keep document IDs stable for historical version switching while grouping
// current operational guides beneath the existing Recipe Book.
const sidebarItemsGenerator: NonNullable<Options["sidebarItemsGenerator"]> = async (args) => {
  const items = await args.defaultSidebarItemsGenerator(args);
  if (args.version.versionName !== "current") return items;
  const recipes = items.find(
    (item) => item.type === "category" && item.label === "Recipes and operational guides",
  );
  if (!recipes || recipes.type !== "category") {
    throw new Error("Missing operational recipe guides in the current sidebar");
  }
  return items.filter((item) => item !== recipes).map((item) => {
    if (item.type !== "category" || item.label !== "Usage") return item;
    return {
      ...item,
      items: item.items.map((child) => child.type === "doc" && child.id === "usage/cookbook"
        ? {
            type: "category" as const,
            label: "Recipe Book",
            link: { type: "doc" as const, id: child.id },
            items: recipes.items,
          }
        : child),
    };
  });
};

export default sidebarItemsGenerator;
