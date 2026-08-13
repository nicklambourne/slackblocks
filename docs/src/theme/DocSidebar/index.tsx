import { useMemo } from "react";
import type {
  PropSidebarItem,
  PropSidebarItemCategory,
} from "@docusaurus/plugin-content-docs";
import { useLanguage, type Language } from "@site/src/components/LanguageContext";
import DocSidebar from "@theme-original/DocSidebar";
import type { Props } from "@theme/DocSidebar";

const TYPESCRIPT_HIDDEN_DOCS = new Set([
  "usage/compatibility",
  "usage/migration",
  "usage/troubleshooting",
  "usage/using_blocks",
]);

const USAGE_ORDER = [
  "usage/installation",
  "usage/using_blocks",
  "usage/sending_messages",
  "usage/cookbook",
  "usage/troubleshooting",
  "usage/migration",
];

function isCategory(item: PropSidebarItem): item is PropSidebarItemCategory {
  return item.type === "category";
}

function isLanguageReference(
  item: PropSidebarItem,
  language: Language,
): item is PropSidebarItemCategory {
  return (
    isCategory(item) &&
    item.href?.includes(`/reference/${language}/`) === true
  );
}

function filterUsage(
  item: PropSidebarItemCategory,
  language: Language,
): PropSidebarItemCategory {
  const positions = new Map(USAGE_ORDER.map((docId, index) => [docId, index]));
  const items = item.items
    .filter(
      (child) =>
        child.type !== "link" ||
        (child.docId !== "usage/compatibility" &&
          (language === "python" ||
            !child.docId ||
            !TYPESCRIPT_HIDDEN_DOCS.has(child.docId))),
    )
    .sort((left, right) => {
      const leftPosition =
        left.type === "link" && left.docId
          ? (positions.get(left.docId) ?? Number.MAX_SAFE_INTEGER)
          : Number.MAX_SAFE_INTEGER;
      const rightPosition =
        right.type === "link" && right.docId
          ? (positions.get(right.docId) ?? Number.MAX_SAFE_INTEGER)
          : Number.MAX_SAFE_INTEGER;
      return leftPosition - rightPosition;
    });

  return {
    ...item,
    collapsed: false,
    items,
  };
}

function filterReference(
  item: PropSidebarItemCategory,
  language: Language,
): PropSidebarItemCategory {
  const languageReference = item.items.find((child) =>
    isLanguageReference(child, language),
  );
  if (!languageReference) return item;

  const sharedItems = item.items.filter(
    (child) =>
      !isLanguageReference(child, "python") &&
      !isLanguageReference(child, "typescript"),
  );

  return {
    ...item,
    href: languageReference.href,
    items: [...sharedItems, ...languageReference.items],
  };
}

export function filterSidebar(
  sidebar: readonly PropSidebarItem[],
  language: Language,
): PropSidebarItem[] {
  return sidebar.map((item) => {
    if (!isCategory(item)) return item;
    if (item.label === "Usage") return filterUsage(item, language);
    if (item.label === "API Reference") return filterReference(item, language);
    return item;
  });
}

export default function LanguageDocSidebar(props: Props) {
  const { language } = useLanguage();
  const sidebar = useMemo(
    () => filterSidebar(props.sidebar, language),
    [language, props.sidebar],
  );

  return <DocSidebar {...props} sidebar={sidebar} />;
}
