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
import legacyManifest from "@site/legacy/manifest.json";

export type Language = "python" | "typescript";

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
  return value === "python" || value === "typescript";
}

function referenceLanguage(pathname: string): Language | null {
  const language =
    pathname.match(/\/reference\/(python|typescript)(?:\/|$)/)?.[1] ?? null;
  return isLanguage(language) ? language : null;
}

function languagePath(pathname: string, language: Language): string {
  const referenceMatch = pathname.match(
    /^(.*\/reference\/)(python|typescript)(?:\/.*)?$/,
  );
  if (referenceMatch) {
    return referenceMatch[2] === language
      ? pathname
      : `${referenceMatch[1]}${language}`;
  }

  const referenceIndexMatch = pathname.match(/^(.*\/reference)\/?$/);
  if (referenceIndexMatch) {
    return `${referenceIndexMatch[1]}/${language}`;
  }

  if (language === "typescript") {
    const pythonGuideMatch = pathname.match(
      /^(.*)\/usage\/(compatibility|migration)\/?$/,
    );
    if (pythonGuideMatch) return `${pythonGuideMatch[1]}/usage/installation`;
  }

  return pathname;
}

export function LanguageProvider({ children }: PropsWithChildren) {
  const history = useHistory();
  const location = useLocation();
  const [language, setLanguage] = useState<Language>("python");

  useEffect(() => {
    const legacyVersion = legacyDocumentationVersion(location.pathname);
    const queryLanguage = new URLSearchParams(location.search).get("language");
    const routeLanguage = referenceLanguage(location.pathname);
    const storedLanguage = window.localStorage.getItem(STORAGE_KEY);
    const nextLanguage = legacyVersion
      ? "python"
      : routeLanguage ??
        (isLanguage(queryLanguage) ? queryLanguage : null) ??
        (isLanguage(storedLanguage) ? storedLanguage : null) ??
        "python";

    const pathname = languagePath(location.pathname, nextLanguage);
    if (pathname !== location.pathname) {
      history.replace({
        pathname,
        search: location.search,
        hash: location.hash,
      });
    }

    setLanguage(nextLanguage);
    if (!legacyVersion && isLanguage(queryLanguage)) {
      window.localStorage.setItem(STORAGE_KEY, queryLanguage);
    }
  }, [history, location.hash, location.pathname, location.search]);

  const selectLanguage = useCallback(
    (nextLanguage: Language) => {
      if (legacyDocumentationVersion(location.pathname)) return;
      setLanguage(nextLanguage);
      window.localStorage.setItem(STORAGE_KEY, nextLanguage);

      const search = new URLSearchParams(location.search);
      search.set("language", nextLanguage);
      const pathname = languagePath(location.pathname, nextLanguage);

      history.replace({
        pathname,
        search: `?${search.toString()}`,
        hash: location.hash,
      });
    },
    [history, location.hash, location.pathname, location.search],
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
