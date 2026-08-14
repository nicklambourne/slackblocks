import { useEffect, useState } from "react";
import { useLanguage } from "@site/src/components/LanguageContext";
import TOCItems from "@theme-original/TOCItems";
import type { Props } from "@theme/TOCItems";

function visibleHeadingIds(language: string, toc: Props["toc"]): Set<string> {
  return new Set(
    toc
      .filter((item) => {
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

  const toc = visibleIds
    ? props.toc.filter((item) => visibleIds.has(item.id))
    : props.toc;

  return <TOCItems {...props} toc={toc} />;
}
