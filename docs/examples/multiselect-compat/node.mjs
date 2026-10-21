// Set SLACKBLOCKS_MODULE to a built package URL when checking a source checkout.
const s = await import(process.env.SLACKBLOCKS_MODULE ?? "@nicklambourne/slackblocks");
for (const [family, element] of Object.entries({
  static: s.StaticMultiSelect().actionId("choose").options(s.Option().text("One").value("one").build()).build(),
  external: s.ExternalMultiSelect().actionId("choose").minQueryLength(0).build(),
  users: s.UserMultiSelect().actionId("choose").build(),
  conversations: s.ConversationMultiSelect().actionId("choose").build(),
  channels: s.ChannelMultiSelect().actionId("choose").build(),
})) {
  for (const [parent, block] of Object.entries({
    actions: s.ActionsBlock().elements(element).blockId("choices").build(),
    section: s.SectionBlock().text("Choose values").accessory(element).blockId("choices").build(),
    input: s.InputBlock().label("Choose values").element(element).blockId("choices").build(),
  })) {
    const payload = parent === "input"
      ? s.Modal().title("Multi-select probe").blocks(block).submit("Save").build()
      : s.Message().channel("C0123456").text("Choose values").blocks(block).build();
    console.log(JSON.stringify({family, parent, surface: parent === "input" ? "modal" : "message", payload}));
  }
}
