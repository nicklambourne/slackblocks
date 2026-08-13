import type { ReactNode } from "react";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import { ThemeClassNames } from "@docusaurus/theme-common";
import { useDocsVersion } from "@docusaurus/plugin-content-docs/client";
import type { Props } from "@theme/DocVersionBadge";
import { useLanguage, type Language } from "@site/src/components/LanguageContext";

type PackageVersions = Record<Language | "spec", string>;

const languageLabels: Record<Language, string> = {
  python: "Python",
  typescript: "TypeScript",
};

export default function DocVersionBadge({ className }: Props): ReactNode {
  const versionMetadata = useDocsVersion();
  const { siteConfig } = useDocusaurusContext();
  const { language } = useLanguage();
  const packageVersions = siteConfig.customFields
    .packageVersions as PackageVersions;

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
      {languageLabels[language]} v{packageVersions[language]} · Spec v
      {packageVersions.spec}
    </span>
  );
}
