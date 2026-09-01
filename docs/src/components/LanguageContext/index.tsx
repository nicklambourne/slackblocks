import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type PropsWithChildren,
} from "react";
import { useHistory, useLocation } from "@docusaurus/router";
import { useAllDocsData } from "@docusaurus/plugin-content-docs/client";
import legacyManifest from "@site/legacy/manifest.json";

export type Language = "python" | "typescript" | "go";

type LanguageContextValue = {
  language: Language;
  selectLanguage: (language: Language) => void;
};

const STORAGE_KEY = "slackblocks.language";
const LanguageContext = createContext<LanguageContextValue | null>(null);
const LEGACY_PREFIXES = new Set(
  legacyManifest.versions
    .filter(({ generated_snapshot_tree_hash: hash }) => Boolean(hash))
    .map(({ canonical_prefix: prefix }) => prefix),
);

export function legacyDocumentationVersion(pathname: string): string | null {
  const prefix = pathname.match(/\/(v\d+\.\d+\.\d+)(?:\/|$)/)?.[1];
  return prefix && LEGACY_PREFIXES.has(prefix) ? prefix.slice(1) : null;
}

function isLanguage(value: string | null): value is Language {
  return value === "python" || value === "typescript" || value === "go";
}

function referenceLanguage(pathname: string): Language | null {
  const language =
    pathname.match(/\/reference\/(python|typescript|go)(?:\/|$)/)?.[1] ?? null;
  return isLanguage(language) ? language : null;
}

function languagePath(
  pathname: string,
  language: Language,
  docPaths: ReadonlySet<string>,
): string {
  const referenceMatch = pathname.match(
    /^(.*\/reference\/)(python|typescript|go)(\/.*)?$/,
  );
  if (referenceMatch) {
    if (referenceMatch[2] === language) return pathname;
    const sibling = `${referenceMatch[1]}${language}${referenceMatch[3] ?? ""}`;
    return docPaths.has(sibling) ? sibling : `${referenceMatch[1]}${language}`;
  }

  const referenceIndexMatch = pathname.match(/^(.*\/reference)\/?$/);
  if (referenceIndexMatch) {
    return `${referenceIndexMatch[1]}/${language}`;
  }

  return pathname;
}

function latestLanguagePath(
  pathname: string,
  language: Language,
  docPaths: ReadonlySet<string>,
): string {
  const legacyMatch = pathname.match(
    /^(.*)\/v\d+\.\d+\.\d+(?=\/|$)(.*)$/,
  );
  const latestPathname = legacyMatch
    ? `${legacyMatch[1]}${legacyMatch[2] || "/"}`
    : pathname;
  const historicalReference = latestPathname.match(
    /^(.*\/reference)\/(?!python(?:\/|$)|typescript(?:\/|$)|go(?:\/|$))[^/]+(?:\/.*)?$/,
  );

  return historicalReference
    ? `${historicalReference[1]}/${language}`
    : languagePath(latestPathname, language, docPaths);
}

export function LanguageProvider({ children }: PropsWithChildren) {
  const history = useHistory();
  const location = useLocation();
  const allDocsData = useAllDocsData();
  const [language, setLanguage] = useState<Language>("python");
  const docPaths = useMemo(
    () =>
      new Set(
        Object.values(allDocsData).flatMap((pluginData) =>
          pluginData.versions.flatMap((version) =>
            version.docs.map((doc) => doc.path),
          ),
        ),
      ),
    [allDocsData],
  );

  useEffect(() => {
    const legacyVersion = legacyDocumentationVersion(location.pathname);
    const search = new URLSearchParams(location.search);
    const queryLanguage = search.get("language");
    const routeLanguage = referenceLanguage(location.pathname);
    const storedLanguage = window.localStorage.getItem(STORAGE_KEY);
    const nextLanguage = legacyVersion
      ? "python"
      : routeLanguage ??
        (isLanguage(queryLanguage) ? queryLanguage : null) ??
        (isLanguage(storedLanguage) ? storedLanguage : null) ??
        "python";

    // Once ?language= has been applied, drop it so shared URLs don't keep it.
    search.delete("language");
    const nextSearch = search.toString();
    const pathname = languagePath(location.pathname, nextLanguage, docPaths);
    if (pathname !== location.pathname || queryLanguage !== null) {
      history.replace({
        pathname,
        search: nextSearch ? `?${nextSearch}` : "",
        hash: location.hash,
      });
    }

    setLanguage(nextLanguage);
    // Persist only explicit choices: the selector (below) or ?language=.
    // A route-derived language must never overwrite the stored preference.
    if (!legacyVersion && isLanguage(queryLanguage)) {
      window.localStorage.setItem(STORAGE_KEY, queryLanguage);
    }
  }, [docPaths, history, location.hash, location.pathname, location.search]);

  const selectLanguage = useCallback(
    (nextLanguage: Language) => {
      setLanguage(nextLanguage);
      window.localStorage.setItem(STORAGE_KEY, nextLanguage);

      const pathname = latestLanguagePath(
        location.pathname,
        nextLanguage,
        docPaths,
      );
      if (pathname !== location.pathname) {
        history.replace({
          pathname,
          search: location.search,
          hash: "",
        });
      }
    },
    [docPaths, history, location.pathname, location.search],
  );

  const value = useMemo(
    () => ({ language, selectLanguage }),
    [language, selectLanguage],
  );

  return (
    <LanguageContext.Provider value={value}>{children}</LanguageContext.Provider>
  );
}

export function useLanguage(): LanguageContextValue {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error("useLanguage must be used within LanguageProvider");
  }
  return context;
}
