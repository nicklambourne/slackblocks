import type { ChangeEvent } from "react";
import { useHistory, useLocation } from "@docusaurus/router";
import {
  useActiveDocContext,
  useDocsPreferredVersion,
  useDocsVersionCandidates,
  useVersions,
  type GlobalDoc,
  type GlobalVersion,
} from "@docusaurus/plugin-content-docs/client";
import legacyManifest from "@site/legacy/manifest.json";
import { useLanguage } from "@site/src/components/LanguageContext";

type Props = {
  mobile?: boolean;
};

const CURRENT_VERSION = "current";
const LEGACY_VERSION_LANGUAGES = new Map(
  legacyManifest.versions.map(({ language_availability, version }) => [
    version,
    new Set(language_availability),
  ]),
);
const HISTORICAL_REFERENCE_PAGES = new Set([
  "attachments",
  "blocks",
  "elements",
  "messages",
  "modals",
  "objects",
  "rich_text",
  "utils",
  "views",
]);

function versionMainDoc(version: GlobalVersion): GlobalDoc | undefined {
  return version.docs.find(({ id }) => id === version.mainDocId);
}

function historicalReferenceId(currentId: string): string | null {
  const match = currentId.match(/^reference\/(python|typescript)(?:\/(.+))?$/);
  if (!match) return null;

  const [, language, currentPage = ""] = match;
  if (!currentPage) return "reference/blocks";

  const page =
    language === "python"
      ? currentPage === "builder"
        ? "utils"
        : currentPage
      : currentPage === "rich-text"
        ? "rich_text"
        : currentPage === "utilities"
          ? "utils"
          : currentPage;
  return HISTORICAL_REFERENCE_PAGES.has(page)
    ? `reference/${page}`
    : "reference/blocks";
}

function currentReferenceId(historicalId: string): string | null {
  const match = historicalId.match(/^reference\/(.+)$/);
  if (!match) return null;

  const page = match[1] === "utils" ? "builder" : match[1];
  return HISTORICAL_REFERENCE_PAGES.has(match[1])
    ? `reference/python/${page}`
    : null;
}

function targetDoc(
  version: GlobalVersion,
  activeDoc: GlobalDoc | undefined,
  alternateDoc: GlobalDoc | undefined,
): GlobalDoc | undefined {
  if (alternateDoc) return alternateDoc;
  if (!activeDoc) return versionMainDoc(version);

  const candidateIds =
    version.name === CURRENT_VERSION
      ? [currentReferenceId(activeDoc.id)]
      : [
          historicalReferenceId(activeDoc.id),
          activeDoc.id === "quick-start" ? "usage/installation" : null,
        ];
  for (const candidateId of candidateIds) {
    const candidate = version.docs.find(({ id }) => id === candidateId);
    if (candidate) return candidate;
  }
  return versionMainDoc(version);
}

function targetSearch(search: string, versionName: string): string {
  const parameters = new URLSearchParams(search);
  if (versionName !== CURRENT_VERSION) parameters.delete("language");
  const serialized = parameters.toString();
  return serialized ? `?${serialized}` : "";
}

function targetHash(
  hash: string,
  activeDoc: GlobalDoc | undefined,
  nextDoc: GlobalDoc,
): string {
  if (!hash || !activeDoc) return "";
  if (activeDoc.id === nextDoc.id) return hash;

  const currentReference = currentReferenceId(activeDoc.id);
  const historicalReference = activeDoc.id.startsWith("reference/python/")
    ? historicalReferenceId(activeDoc.id)
    : null;
  return currentReference === nextDoc.id || historicalReference === nextDoc.id
    ? hash
    : "";
}

export default function VersionSelector({ mobile = false }: Props) {
  const history = useHistory();
  const location = useLocation();
  const { language } = useLanguage();
  const versions = useVersions(undefined);
  const activeDocContext = useActiveDocContext(undefined);
  const versionCandidates = useDocsVersionCandidates(undefined);
  const { savePreferredVersionName } = useDocsPreferredVersion();
  const availableVersions = versions.filter(
    (version) =>
      version.name === CURRENT_VERSION ||
      LEGACY_VERSION_LANGUAGES.get(version.name)?.has(language),
  );
  const activeVersion = versionCandidates[0] ?? versions[0];
  const displayedVersion =
    availableVersions.find(({ name }) => name === activeVersion?.name) ??
    availableVersions[0];

  if (!displayedVersion) return null;

  const selectVersion = (event: ChangeEvent<HTMLSelectElement>) => {
    const version = availableVersions.find(
      ({ name }) => name === event.target.value,
    );
    if (!version) return;

    const nextDoc = targetDoc(
      version,
      activeDocContext.activeDoc,
      activeDocContext.alternateDocVersions[version.name],
    );
    if (!nextDoc) return;

    savePreferredVersionName(version.name);
    history.push(
      `${nextDoc.path}${targetSearch(location.search, version.name)}${targetHash(
        location.hash,
        activeDocContext.activeDoc,
        nextDoc,
      )}`,
    );
  };

  return (
    <select
      aria-label="Documentation version"
      className={`docs-version-select${mobile ? " docs-version-select--mobile" : ""}`}
      onChange={selectVersion}
      value={displayedVersion.name}
    >
      {availableVersions.map((version) => (
        <option key={version.name} value={version.name}>
          {version.label}
        </option>
      ))}
    </select>
  );
}
