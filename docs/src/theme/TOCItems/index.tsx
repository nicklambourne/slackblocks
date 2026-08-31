import { useEffect, useState } from "react";
import { useLocation } from "@docusaurus/router";
import { useLanguage } from "@site/src/components/LanguageContext";
import TOCItems from "@theme-original/TOCItems";
import type { Props } from "@theme/TOCItems";

function isVisibleTocValue(language: string, value: string): boolean {
  return language !== "python" || value !== "from_dict";
}

function displayTocValues(
  stripFactoryParentheses: boolean,
  toc: Props["toc"],
): Props["toc"] {
  if (!stripFactoryParentheses) return toc;

  return toc.map((item) => ({
    ...item,
    value: item.value.replace(/\(\)$/, ""),
  }));
}

function visibleHeadingIds(language: string, toc: Props["toc"]): Set<string> {
  return new Set(
    toc
      .filter((item) => {
        if (!isVisibleTocValue(language, item.value)) return false;
        const heading = document.getElementById(item.id);
        const content = heading?.closest<HTMLElement>(
          "[data-language-content]",
        );
        return !content || content.dataset.languageContent === language;
      })
      .map((item) => item.id),
  );
}

export default function LanguageTOCItems(props: Props) {
  const { pathname } = useLocation();
  const { language } = useLanguage();
  const [visibleIds, setVisibleIds] = useState<Set<string> | null>(null);

  useEffect(() => {
    setVisibleIds(visibleHeadingIds(language, props.toc));
  }, [language, props.toc]);

  const visibleToc = (
    visibleIds
      ? props.toc.filter((item) => visibleIds.has(item.id))
      : props.toc
  ).filter((item) => isVisibleTocValue(language, item.value));

  return (
    <TOCItems
      {...props}
      toc={displayTocValues(
        pathname.includes("/reference/typescript/"),
        visibleToc,
      )}
    />
  );
}
