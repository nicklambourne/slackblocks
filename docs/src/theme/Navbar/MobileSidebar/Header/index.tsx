import type { ReactNode } from "react";
import { translate } from "@docusaurus/Translate";
import { useNavbarMobileSidebar } from "@docusaurus/theme-common/internal";
import IconClose from "@theme/Icon/Close";
import NavbarColorModeToggle from "@theme/Navbar/ColorModeToggle";
import NavbarLogo from "@theme/Navbar/Logo";
import VersionSelector from "@site/src/components/VersionSelector";

function CloseButton() {
  const mobileSidebar = useNavbarMobileSidebar();

  return (
    <button
      type="button"
      aria-label={translate({
        id: "theme.docs.sidebar.closeSidebarButtonAriaLabel",
        message: "Close navigation bar",
        description: "The ARIA label for close button of mobile sidebar",
      })}
      className="clean-btn navbar-sidebar__close"
      onClick={() => mobileSidebar.toggle()}
    >
      <IconClose color="var(--ifm-color-emphasis-600)" />
    </button>
  );
}

export default function NavbarMobileSidebarHeader(): ReactNode {
  return (
    <div className="navbar-sidebar__brand">
      <NavbarLogo />
      <VersionSelector mobile />
      <div className="navbar-sidebar__actions">
        <a
          aria-label="GitHub repository"
          className="header-github-link navbar-sidebar__github-link"
          href="https://github.com/nicklambourne/slackblocks"
          rel="noopener noreferrer"
          target="_blank"
        />
        <NavbarColorModeToggle />
        <CloseButton />
      </div>
    </div>
  );
}
