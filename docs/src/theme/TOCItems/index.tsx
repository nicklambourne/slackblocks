import { useEffect, useState } from "react";
import { useLanguage } from "@site/src/components/LanguageContext";
import TOCItems from "@theme-original/TOCItems";
import type { Props } from "@theme/TOCItems";

function isVisibleTocValue(language: string, value: string): boolean {
  return language !== "python" || value !== "from_dict";
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
  const { language } = useLanguage();
  const [visibleIds, setVisibleIds] = useState<Set<string> | null>(null);

  useEffect(() => {
    setVisibleIds(visibleHeadingIds(language, props.toc));
  }, [language, props.toc]);

  const toc = (
    visibleIds
      ? props.toc.filter((item) => visibleIds.has(item.id))
      : props.toc
  ).filter((item) => isVisibleTocValue(language, item.value));

  return <TOCItems {...props} toc={toc} />;
}
