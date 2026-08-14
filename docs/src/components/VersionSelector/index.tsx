import type { ChangeEvent } from "react";
import { useHistory, useLocation } from "@docusaurus/router";
import {
  useActiveDocContext,
  useDocsPreferredVersion,
  useDocsVersionCandidates,
  useVersions,
} from "@docusaurus/plugin-content-docs/client";

export default function VersionSelector() {
  const history = useHistory();
  const location = useLocation();
  const versions = useVersions(undefined);
  const activeDocContext = useActiveDocContext(undefined);
  const versionCandidates = useDocsVersionCandidates(undefined);
  const { savePreferredVersionName } = useDocsPreferredVersion();
  const displayedVersion = versionCandidates[0] ?? versions[0];

  if (!displayedVersion || versions.length <= 1) return null;

  const selectVersion = (event: ChangeEvent<HTMLSelectElement>) => {
    const version = versions.find(({ name }) => name === event.target.value);
    if (!version) return;

    const targetDoc =
      activeDocContext.alternateDocVersions[version.name] ??
      version.docs.find(({ id }) => id === version.mainDocId);
    if (!targetDoc) return;

    savePreferredVersionName(version.name);
    history.push(`${targetDoc.path}${location.search}${location.hash}`);
  };

  return (
    <select
      aria-label="Documentation version"
      className="docs-version-select"
      onChange={selectVersion}
      value={displayedVersion.name}
    >
      {versions.map((version) => (
        <option key={version.name} value={version.name}>
          {version.label}
        </option>
      ))}
    </select>
  );
}
