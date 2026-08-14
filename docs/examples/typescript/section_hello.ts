import { message, sectionBlock } from "slackblocks";

export const payload = message({
  channel: "C0123456",
  blocks: [sectionBlock({ text: "Hello from slackblocks!", blockId: "hello" })],
});
