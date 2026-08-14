import { useMemo } from "react";
import { useLocation } from "@docusaurus/router";
import { useDocsSidebar } from "@docusaurus/plugin-content-docs/client";
import type {
  PropNavigationLink,
  PropSidebarItem,
} from "@docusaurus/plugin-content-docs";
import { useLanguage } from "@site/src/components/LanguageContext";
import { filterSidebar } from "@site/src/theme/DocSidebar";
import DocPaginator from "@theme/DocPaginator";

function navigationItems(
  items: readonly PropSidebarItem[],
): PropNavigationLink[] {
  return items.flatMap((item) => {
    if (item.type === "link") {
      return item.unlisted
        ? []
        : [{ title: item.label, permalink: item.href }];
    }
    if (item.type !== "category") return [];

    const category =
      item.href && !item.linkUnlisted
        ? [{ title: item.label, permalink: item.href }]
        : [];
    return [...category, ...navigationItems(item.items)];
  });
}

function normalizedPath(path: string): string {
  return path.replace(/\/$/, "");
}

export default function LanguageDocItemPaginator() {
  const { pathname } = useLocation();
  const { language } = useLanguage();
  const sidebar = useDocsSidebar();
  const items = useMemo(
    () =>
      sidebar
        ? navigationItems(filterSidebar(sidebar.items, language))
        : [],
    [language, sidebar],
  );
  const index = items.findIndex(
    (item) => normalizedPath(item.permalink) === normalizedPath(pathname),
  );

  if (index < 0) return null;

  return (
    <DocPaginator
      className="docusaurus-mt-lg"
      previous={items[index - 1]}
      next={items[index + 1]}
    />
  );
}
