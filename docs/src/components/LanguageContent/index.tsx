import { Children, isValidElement, type PropsWithChildren } from "react";
import { useLanguage } from "@site/src/components/LanguageContext";

export default function LanguageContent({ children }: PropsWithChildren) {
  const { language } = useLanguage();
  const content = Children.toArray(children).map((child, index) => {
    if (!isValidElement<PropsWithChildren>(child)) return null;
    const childLanguage =
      child.type === Python
        ? "python"
        : child.type === TypeScript
          ? "typescript"
          : child.type === Go
            ? "go"
          : null;
    if (!childLanguage) return null;

    return (
      <div
        key={index}
        data-language-content={childLanguage}
        hidden={language !== childLanguage}
      >
        {child.props.children}
      </div>
    );
  });

  return <div className="language-content">{content}</div>;
}

export function Python({ children }: PropsWithChildren) {
  return <>{children}</>;
}

export function TypeScript({ children }: PropsWithChildren) {
  return <>{children}</>;
}

export function Go({ children }: PropsWithChildren) {
  return <>{children}</>;
}
