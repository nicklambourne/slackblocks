import { useEffect, useRef, useState } from "react";
import { useLocation } from "@docusaurus/router";
import {
  legacyDocumentationVersion,
  useLanguage,
  type Language,
} from "@site/src/components/LanguageContext";

type Props = {
  mobile?: boolean;
};

const languageLabels: Record<Language, string> = {
  python: "Python",
  typescript: "TypeScript",
  go: "Go",
};

function LanguageLogo({ language }: { language: Language }) {
  if (language === "python") {
    return (
      <svg
        aria-hidden="true"
        className="language-selector__logo"
        viewBox="0 0 24 24"
      >
        <path
          fill="#3776ab"
          d="M11.95 0c-1 0-1.94.09-2.78.24-2.48.43-2.93 1.35-2.93 3.03v2.22h5.85v.74H4.05C2.35 6.23.86 7.25.39 9.19c-.54 2.23-.56 3.62 0 5.94.42 1.73 1.42 2.96 3.12 2.96h2.02v-2.68c0-1.93 1.67-3.64 3.66-3.64h5.84c1.63 0 2.93-1.34 2.93-2.97V3.27c0-1.58-1.34-2.77-2.93-3.04A18.5 18.5 0 0 0 11.95 0ZM8.78 1.79c.61 0 1.1.5 1.1 1.11 0 .62-.49 1.11-1.1 1.11-.6 0-1.09-.49-1.09-1.11 0-.61.49-1.11 1.09-1.11Z"
        />
        <path
          fill="#ffd343"
          d="M18.68 6.24v2.6c0 2.02-1.7 3.72-3.65 3.72H9.18c-1.6 0-2.92 1.37-2.92 2.97v5.57c0 1.59 1.38 2.52 2.92 2.98 1.85.54 3.62.64 5.85 0 1.47-.43 2.93-1.28 2.93-2.98v-2.22h-5.85v-.75h8.77c1.7 0 2.34-1.18 2.93-2.97.61-1.83.58-3.59 0-5.93-.42-1.69-1.22-2.99-2.93-2.99h-2.2Zm-3.28 14.12c.6 0 1.1.49 1.1 1.1 0 .62-.5 1.12-1.1 1.12-.61 0-1.1-.5-1.1-1.12 0-.61.49-1.1 1.1-1.1Z"
        />
      </svg>
    );
  }

  if (language === "typescript") return (
    <svg
      aria-hidden="true"
      className="language-selector__logo"
      viewBox="0 0 24 24"
    >
      <rect fill="#3178c6" height="22" rx="2" width="22" x="1" y="1" />
      <text
        fill="#fff"
        fontFamily="Arial, sans-serif"
        fontSize="10"
        fontWeight="700"
        textAnchor="middle"
        x="12"
        y="17"
      >
        TS
      </text>
    </svg>
  );

  // Official Go wordmark: https://go.dev/images/go-logo-blue.svg
  return (
    <svg
      aria-hidden="true"
      className="language-selector__logo language-selector__logo--go"
      viewBox="0 0 207 78"
    >
      <g fill="#00acd7" fillRule="evenodd">
        <path d="m16.2 24.1c-.4 0-.5-.2-.3-.5l2.1-2.7c.2-.3.7-.5 1.1-.5h35.7c.4 0 .5.3.3.6l-1.7 2.6c-.2.3-.7.6-1 .6z" />
        <path d="m1.1 33.3c-.4 0-.5-.2-.3-.5l2.1-2.7c.2-.3.7-.5 1.1-.5h45.6c.4 0 .6.3.5.6l-.8 2.4c-.1.4-.5.6-.9.6z" />
        <path d="m25.3 42.5c-.4 0-.5-.3-.3-.6l1.4-2.5c.2-.3.6-.6 1-.6h20c.4 0 .6.3.6.7l-.2 2.4c0 .4-.4.7-.7.7z" />
        <g transform="translate(55)">
          <path d="m74.1 22.3c-6.3 1.6-10.6 2.8-16.8 4.4-1.5.4-1.6.5-2.9-1-1.5-1.7-2.6-2.8-4.7-3.8-6.3-3.1-12.4-2.2-18.1 1.5-6.8 4.4-10.3 10.9-10.2 19 .1 8 5.6 14.6 13.5 15.7 6.8.9 12.5-1.5 17-6.6.9-1.1 1.7-2.3 2.7-3.7-3.6 0-8.1 0-19.3 0-2.1 0-2.6-1.3-1.9-3 1.3-3.1 3.7-8.3 5.1-10.9.3-.6 1-1.6 2.5-1.6h36.4c-.2 2.7-.2 5.4-.6 8.1-1.1 7.2-3.8 13.8-8.2 19.6-7.2 9.5-16.6 15.4-28.5 17-9.8 1.3-18.9-.6-26.9-6.6-7.4-5.6-11.6-13-12.7-22.2-1.3-10.9 1.9-20.7 8.5-29.3 7.1-9.3 16.5-15.2 28-17.3 9.4-1.7 18.4-.6 26.5 4.9 5.3 3.5 9.1 8.3 11.6 14.1.6.9.2 1.4-1 1.7z" />
          <path
            d="m107.2 77.6c-9.1-.2-17.4-2.8-24.4-8.8-5.9-5.1-9.6-11.6-10.8-19.3-1.8-11.3 1.3-21.3 8.1-30.2 7.3-9.6 16.1-14.6 28-16.7 10.2-1.8 19.8-.8 28.5 5.1 7.9 5.4 12.8 12.7 14.1 22.3 1.7 13.5-2.2 24.5-11.5 33.9-6.6 6.7-14.7 10.9-24 12.8-2.7.5-5.4.6-8 .9zm23.8-40.4c-.1-1.3-.1-2.3-.3-3.3-1.8-9.9-10.9-15.5-20.4-13.3-9.3 2.1-15.3 8-17.5 17.4-1.8 7.8 2 15.7 9.2 18.9 5.5 2.4 11 2.1 16.3-.6 7.9-4.1 12.2-10.5 12.7-19.1z"
            fillRule="nonzero"
          />
        </g>
      </g>
    </svg>
  );
}

export default function LanguageSelector({ mobile = false }: Props) {
  const { language, selectLanguage } = useLanguage();
  const location = useLocation();
  const legacyVersion = legacyDocumentationVersion(location.pathname);
  const [open, setOpen] = useState(false);
  const selectorRef = useRef<HTMLDivElement>(null);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const selectedLanguage: Language = legacyVersion ? "python" : language;
  const otherLanguages = (Object.keys(languageLabels) as Language[]).filter(
    (candidate) => candidate !== selectedLanguage,
  );

  useEffect(() => {
    if (!open) return undefined;

    const closeOutside = (event: MouseEvent | TouchEvent | FocusEvent) => {
      if (!selectorRef.current?.contains(event.target as Node)) {
        setOpen(false);
      }
    };
    const closeOnEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setOpen(false);
        triggerRef.current?.focus();
      }
    };

    document.addEventListener("mousedown", closeOutside);
    document.addEventListener("touchstart", closeOutside);
    document.addEventListener("focusin", closeOutside);
    document.addEventListener("keydown", closeOnEscape);

    return () => {
      document.removeEventListener("mousedown", closeOutside);
      document.removeEventListener("touchstart", closeOutside);
      document.removeEventListener("focusin", closeOutside);
      document.removeEventListener("keydown", closeOnEscape);
    };
  }, [open]);

  const selector = (
    <div className="language-selector" ref={selectorRef}>
      <button
        aria-expanded={open}
        aria-haspopup="menu"
        aria-label={`Documentation language: ${languageLabels[selectedLanguage]}`}
        className="language-selector__trigger"
        onClick={() => setOpen((visible) => !visible)}
        ref={triggerRef}
        type="button"
      >
        <LanguageLogo language={selectedLanguage} />
        <span>{languageLabels[selectedLanguage]}</span>
        <span aria-hidden="true" className="language-selector__caret" />
      </button>
      {open && (
        <ul className="language-selector__menu" role="menu">
          {otherLanguages.map((otherLanguage) => <li key={otherLanguage}>
            <button
              className="language-selector__option"
              onClick={() => {
                selectLanguage(otherLanguage);
                setOpen(false);
              }}
              role="menuitem"
              type="button"
            >
              <LanguageLogo language={otherLanguage} />
              <span>{languageLabels[otherLanguage]}</span>
            </button>
          </li>)}
        </ul>
      )}
    </div>
  );

  if (mobile) {
    return <li className="menu__list-item language-selector-mobile">{selector}</li>;
  }

  return <div className="navbar__item language-selector-desktop">{selector}</div>;
}
