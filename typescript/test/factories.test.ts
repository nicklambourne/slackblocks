import { describe, expect, it } from "vitest";
import type { KnownBlock } from "@slack/types";

import {
  blockKitBuilderUrl,
  button,
  LengthError,
  message,
  mrkdwn,
  sectionBlock,
  validate,
} from "../src/index.js";

describe("data-first factories", () => {
  it("returns objects assignable to Slack's official block types", () => {
    const block: KnownBlock = sectionBlock({ text: "Typed" });
    expect(block.type).toBe("section");
  });

  it("converts camelCase input to a Slack-shaped plain object", () => {
    const payload = message({
      channel: "C0123456",
      blocks: [
        sectionBlock({
          blockId: "deploy",
          text: mrkdwn("*Deploy complete* :rocket:"),
          accessory: button({
            text: "View logs",
            actionId: "logs",
            url: "https://example.com/logs",
          }),
        }),
      ],
    });

    expect(payload).toEqual({
      channel: "C0123456",
      mrkdwn: true,
      text: "",
      blocks: [
        {
          type: "section",
          block_id: "deploy",
          text: { type: "mrkdwn", text: "*Deploy complete* :rocket:" },
          accessory: {
            type: "button",
            text: { type: "plain_text", text: "View logs" },
            action_id: "logs",
            url: "https://example.com/logs",
          },
        },
      ],
    });
  });

  it("reports a field path for validation errors", () => {
    expect(() =>
      sectionBlock({
        blockId: "bad",
        accessory: button({ text: "A", actionId: "a" }),
        text: "x".repeat(3001),
      }),
    ).toThrowError(LengthError);

    try {
      sectionBlock({ text: "x".repeat(3001) });
    } catch (error) {
      expect((error as LengthError).path).toBe("text");
    }
  });

  it("allows validation to be disabled explicitly", () => {
    expect(sectionBlock({ text: "x".repeat(3001) }, { validate: false }).type).toBe("section");
  });

  it("validates arbitrary existing JSON with a type guard", () => {
    expect(validate({ type: "section", text: { type: "mrkdwn", text: "Hi" } })).toBe(true);
    expect(validate({ type: "section" })).toBe(false);
  });
});

describe("Block Kit Builder URL", () => {
  it("wraps a block list and accepts a team ID", () => {
    const url = blockKitBuilderUrl([sectionBlock({ text: "Hi" })], "T123");
    expect(url).toMatch(/^https:\/\/app\.slack\.com\/block-kit-builder\/T123#/);
    expect(decodeURIComponent(url.split("#")[1] ?? "")).toBe(
      JSON.stringify({ blocks: [sectionBlock({ text: "Hi" })] }),
    );
  });
});
