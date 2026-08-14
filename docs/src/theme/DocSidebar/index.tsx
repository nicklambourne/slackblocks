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
]);

const USAGE_ORDER = [
  "usage/installation",
  "usage/using_blocks",
  "usage/sending_messages",
  "usage/cookbook",
  "usage/troubleshooting",
  "usage/migration",
];

const SIDEBAR_LABELS: Record<string, string> = {
  "Composition objects": "Composition Objects",
  "Rich text": "Rich Text",
  classes: "Classes",
  functions: "Functions",
  interfaces: "Interfaces",
  "type-aliases": "Type Aliases",
  variables: "Variables",
};

function isCategory(item: PropSidebarItem): item is PropSidebarItemCategory {
  return item.type === "category";
}

function capitaliseSidebarItem(item: PropSidebarItem): PropSidebarItem {
  if (item.type === "html") return item;

  const label =
    SIDEBAR_LABELS[item.label] ??
    item.label.replace(/^[a-z]/, (letter) => letter.toUpperCase());
  if (!isCategory(item)) return { ...item, label };

  return {
    ...item,
    label,
    items: item.items.map(capitaliseSidebarItem),
  };
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
  const usage = sidebar.find(
    (item): item is PropSidebarItemCategory =>
      isCategory(item) && item.label === "Usage",
  );
  const compatibility = usage?.items.find(
      (item) =>
        item.type === "link" && item.docId === "usage/compatibility",
    );

  return sidebar
    .map((item) => {
      if (!isCategory(item)) return item;
      if (item.label === "Usage") return filterUsage(item, language);
      if (item.label === "API Reference") return filterReference(item, language);
      return item;
    })
    .flatMap((item) => {
      if (
        language === "python" &&
        item.type === "link" &&
        item.docId === "contributing" &&
        compatibility
      ) {
        return [compatibility, item];
      }

      return [item];
    })
    .map(capitaliseSidebarItem);
}

export default function LanguageDocSidebar(props: Props) {
  const { language } = useLanguage();
  const sidebar = useMemo(
    () => filterSidebar(props.sidebar, language),
    [language, props.sidebar],
  );

  return <DocSidebar {...props} sidebar={sidebar} />;
}
