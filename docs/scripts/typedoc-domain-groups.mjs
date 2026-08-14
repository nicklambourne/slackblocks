/** Flatten TypeDoc's kind-based index groups inside each domain. */
export function load(app) {
  app.converter.on(
    "resolveEnd",
    (context) => {
      const entryPoints = Object.values(context.project.reflections).filter(
        (reflection) => reflection.parent?.isProject(),
      );

      for (const group of context.project.groups ?? []) {
        group.title = "none";
      }

      for (const reflection of entryPoints) {
        const [group, ...remainingGroups] = reflection.groups ?? [];
        if (!group) continue;

        group.title = "none";
        group.children = [group, ...remainingGroups]
          .flatMap((item) => item.children)
          .sort((left, right) =>
            left.name.localeCompare(right.name, "en", { sensitivity: "base" }),
          );
        group.categories = undefined;
        reflection.groups = [group];
      }
    },
    -1000,
  );
}
