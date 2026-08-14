import HtmlNavbarItem from "@theme-original/NavbarItem/HtmlNavbarItem";
import type { Props } from "@theme/NavbarItem/HtmlNavbarItem";
import LanguageSelector from "@site/src/components/LanguageSelector";
import VersionSelector from "@site/src/components/VersionSelector";

export default function HtmlNavbarItemOverride(props: Props) {
  if (props.value === "language-selector") {
    return <LanguageSelector mobile={props.mobile} />;
  }
  if (props.value === "version-selector") {
    return props.mobile ? null : (
      <div className="navbar__item docs-version-selector-desktop">
        <VersionSelector />
      </div>
    );
  }

  return <HtmlNavbarItem {...props} />;
}
