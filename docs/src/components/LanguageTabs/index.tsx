import { Children, isValidElement, type PropsWithChildren } from "react";
import TabItem from "@theme/TabItem";
import Tabs from "@theme/Tabs";

export default function LanguageTabs({ children }: PropsWithChildren) {
  const items = Children.toArray(children).map((child, index) => {
    if (!isValidElement<PropsWithChildren>(child)) return null;
    if (child.type === PythonTab) {
      return (
        <TabItem key={index} value="python" label="Python">
          {child.props.children}
        </TabItem>
      );
    }
    if (child.type === TypeScriptTab) {
      return (
        <TabItem key={index} value="typescript" label="TypeScript">
          {child.props.children}
        </TabItem>
      );
    }
    if (child.type === JsonTab) {
      return (
        <TabItem key={index} value="json" label="JSON">
          {child.props.children}
        </TabItem>
      );
    }
    return null;
  });

  return (
    <Tabs groupId="language" queryString>
      {items}
    </Tabs>
  );
}

export function PythonTab({ children }: PropsWithChildren) {
  return <>{children}</>;
}

export function TypeScriptTab({ children }: PropsWithChildren) {
  return <>{children}</>;
}

export function JsonTab({ children }: PropsWithChildren) {
  return <>{children}</>;
}
