import type { ReactNode } from "react";
import { ThemeClassNames } from "@docusaurus/theme-common";
import { useDocsVersion } from "@docusaurus/plugin-content-docs/client";
import type { Props } from "@theme/DocVersionBadge";
import { useLanguage, type Language } from "@site/src/components/LanguageContext";

const languageLabels: Record<Language, string> = {
  python: "Python",
  typescript: "TypeScript",
  go: "Go",
};

export default function DocVersionBadge({ className }: Props): ReactNode {
  const versionMetadata = useDocsVersion();
  const { language } = useLanguage();

  if (!versionMetadata.badge) {
    return null;
  }

  return (
    <span
      className={[
        className,
        ThemeClassNames.docs.docVersionBadge,
        "badge badge--secondary",
      ]
        .filter(Boolean)
        .join(" ")}
    >
      {languageLabels[language]} v{versionMetadata.label}
    </span>
  );
}
