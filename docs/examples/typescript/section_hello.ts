import { message, section } from "slackblocks";

export const payload = message({
  channel: "C0123456",
  blocks: [section({ text: "Hello from slackblocks!", blockId: "hello" })],
});
