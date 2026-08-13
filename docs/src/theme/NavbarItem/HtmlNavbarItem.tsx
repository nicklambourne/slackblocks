import HtmlNavbarItem from "@theme-original/NavbarItem/HtmlNavbarItem";
import type { Props } from "@theme/NavbarItem/HtmlNavbarItem";
import LanguageSelector from "@site/src/components/LanguageSelector";

export default function HtmlNavbarItemOverride(props: Props) {
  if (props.value === "language-selector") {
    return <LanguageSelector mobile={props.mobile} />;
  }

  return <HtmlNavbarItem {...props} />;
}
