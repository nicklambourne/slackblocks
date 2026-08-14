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

  return (
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
}

export default function LanguageSelector({ mobile = false }: Props) {
  const { language, selectLanguage } = useLanguage();
  const location = useLocation();
  const legacyVersion = legacyDocumentationVersion(location.pathname);
  const [open, setOpen] = useState(false);
  const selectorRef = useRef<HTMLDivElement>(null);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const selectedLanguage: Language = legacyVersion ? "python" : language;
  const otherLanguage: Language =
    selectedLanguage === "python" ? "typescript" : "python";

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
          <li>
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
          </li>
        </ul>
      )}
    </div>
  );

  if (mobile) {
    return <li className="menu__list-item language-selector-mobile">{selector}</li>;
  }

  return <div className="navbar__item language-selector-desktop">{selector}</div>;
}
