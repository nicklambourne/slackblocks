import type { ReactNode } from "react";
import { ThemeClassNames } from "@docusaurus/theme-common";
import { useNavbarSecondaryMenu } from "@docusaurus/theme-common/internal";
import type { Props } from "@theme/Navbar/MobileSidebar/Layout";

export default function NavbarMobileSidebarLayout({
  header,
}: Props): ReactNode {
  const secondaryMenu = useNavbarSecondaryMenu();

  return (
    <div
      className={`${ThemeClassNames.layout.navbar.mobileSidebar.container} navbar-sidebar`}
    >
      {header}
      <div className="navbar-sidebar__items">
        <div
          className={`${ThemeClassNames.layout.navbar.mobileSidebar.panel} navbar-sidebar__item menu`}
        >
          {secondaryMenu.content}
        </div>
      </div>
    </div>
  );
}
