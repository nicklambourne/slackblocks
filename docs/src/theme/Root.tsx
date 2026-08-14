import type { PropsWithChildren } from "react";
import { LanguageProvider } from "@site/src/components/LanguageContext";

export default function Root({ children }: PropsWithChildren) {
  return <LanguageProvider>{children}</LanguageProvider>;
}
