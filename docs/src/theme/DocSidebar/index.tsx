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
  objects: "Composition Objects",
  "rich-text": "Rich Text",
  classes: "Classes",
  functions: "Functions",
  interfaces: "Interfaces",
  "type-aliases": "Type Aliases",
  variables: "Variables",
};

const TYPESCRIPT_REFERENCE_ORDER = [
  "blocks",
  "elements",
  "objects",
  "rich-text",
  "messages",
  "views",
  "utilities",
  "errors",
];

function typescriptReferencePosition(item: PropSidebarItem): number {
  if (item.type === "html") return Number.MAX_SAFE_INTEGER;
  const slug = item.href?.match(/\/reference\/typescript\/([^/?#]+)\/?$/)?.[1];
  const position = TYPESCRIPT_REFERENCE_ORDER.indexOf(
    slug ?? item.label.toLowerCase(),
  );
  return position < 0 ? Number.MAX_SAFE_INTEGER : position;
}

const TYPESCRIPT_API_TYPE_PREFIX =
  /^(?:Class|Function|Interface|Type Alias|Variable):\s*/;

function isCategory(item: PropSidebarItem): item is PropSidebarItemCategory {
  return item.type === "category";
}

function capitaliseSidebarItem(item: PropSidebarItem): PropSidebarItem {
  if (item.type === "html") return item;

  const withoutTypePrefix = item.label.replace(
    TYPESCRIPT_API_TYPE_PREFIX,
    "",
  );
  const label =
    SIDEBAR_LABELS[item.label] ??
    (withoutTypePrefix !== item.label
      ? withoutTypePrefix
      : item.label.replace(/^[a-z]/, (letter) => letter.toUpperCase()));
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

function mergeTypeScriptDomainIndexes(
  items: PropSidebarItem[],
): PropSidebarItem[] {
  const categoryLabels = new Set(
    items
      .filter(isCategory)
      .map((item) => item.label.toLowerCase()),
  );
  const indexLinks = new Map(
    items
      .filter((item) => item.type === "link")
      .map((item) => [item.label.toLowerCase(), item.href]),
  );

  return items.flatMap((item) => {
    if (item.type === "html") return item;
    const label = item.label.toLowerCase();
    if (item.type === "link" && categoryLabels.has(label)) return [];
    if (!isCategory(item)) return item;
    return {
      ...item,
      href: indexLinks.get(label) ?? item.href,
      items: [...item.items].sort((left, right) => {
        const leftLabel = left.type === "html" ? "" : left.label;
        const rightLabel = right.type === "html" ? "" : right.label;
        return leftLabel.localeCompare(rightLabel, "en", {
          sensitivity: "base",
        });
      }),
    };
  });
}

function filterReference(
  item: PropSidebarItemCategory,
  language: Language,
): PropSidebarItemCategory {
  const languageReference = item.items.find((child) =>
    isLanguageReference(child, language),
  );
  if (!languageReference) return item;

  const languageItems =
    language === "typescript"
      ? mergeTypeScriptDomainIndexes([...languageReference.items]).sort(
          (left, right) =>
            typescriptReferencePosition(left) -
            typescriptReferencePosition(right),
        )
      : languageReference.items;

  const sharedItems = item.items.filter(
    (child) =>
      !isLanguageReference(child, "python") &&
      !isLanguageReference(child, "typescript"),
  );

  return {
    ...item,
    href: languageReference.href,
    items: [...sharedItems, ...languageItems],
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
