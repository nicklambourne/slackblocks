using Slackblocks;
using Slackblocks.Blocks;
using Slackblocks.Elements;
using Slackblocks.Objects;
using Slackblocks.Payloads;

static void Emit(string family, IInputElement element) {
    foreach (var parent in new[] { "actions", "section", "input" }) {
        string payload;
        if (parent == "input") {
            payload = new ModalView(title: "Multi-select probe", submit: "Save", blocks: [new InputBlock(label: "Choose values", element: element, blockId: "choices")]).ToJson();
        } else {
            IBlock block = parent == "actions" ? new ActionsBlock(elements: [element], blockId: "choices") : new SectionBlock(text: "Choose values", accessory: element, blockId: "choices");
            payload = new MessagePayload(channel: "C0123456", text: "Choose values", blocks: [block]).ToJson();
        }
        Console.WriteLine("{\"family\":\"" + family + "\",\"parent\":\"" + parent + "\",\"surface\":\"" + (parent == "input" ? "modal" : "message") + "\",\"payload\":" + payload + "}");
    }
}
Emit("static", new StaticMultiSelectElement(actionId: "choose", options: [new Option(text: "One", value: "one")]));
Emit("external", new ExternalMultiSelectElement(actionId: "choose", minQueryLength: 0));
Emit("users", new UserMultiSelectElement(actionId: "choose"));
Emit("conversations", new ConversationMultiSelectElement(actionId: "choose"));
Emit("channels", new ChannelMultiSelectElement(actionId: "choose"));
