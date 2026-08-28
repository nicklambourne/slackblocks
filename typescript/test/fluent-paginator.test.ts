import { describe, expect, it } from "vitest";

import {
  Message,
  Paginator,
  SectionBlock,
  actionsBlock,
  button,
  contextBlock,
  message,
  mrkdwn,
  sectionBlock,
} from "../src/index.js";

describe("Paginator", () => {
  const pages = ["One", "Two", "Three"].map((text) =>
    SectionBlock().text(text),
  );

  it("renders a selected page and navigation as ordinary blocks", () => {
    expect(
      Paginator()
        .blocks(pages)
        .actionIdPrefix("results")
        .page(2)
        .pageSize(1)
        .build(),
    ).toEqual([
      sectionBlock({ text: "Two" }),
      contextBlock({ elements: [mrkdwn("Page 2 of 3")] }),
      actionsBlock({
        elements: [
          button({ text: "Previous", actionId: "results.previous", value: "1" }),
          button({ text: "Next", actionId: "results.next", value: "3" }),
        ],
      }),
    ]);
  });

  it("expands directly inside a fluent block collection", () => {
    expect(
      Message()
        .channel("C123")
        .blocks(
          Paginator()
            .blocks(pages)
            .actionIdPrefix("results")
            .pageSize(2),
        )
        .build(),
    ).toEqual(
      message({
        channel: "C123",
        blocks: [
          sectionBlock({ text: "One" }),
          sectionBlock({ text: "Two" }),
          contextBlock({ elements: [mrkdwn("Page 1 of 2")] }),
          actionsBlock({
            elements: [
              button({ text: "Next", actionId: "results.next", value: "2" }),
            ],
          }),
        ],
      }),
    );
  });

  it("omits navigation when all content fits", () => {
    expect(
      Paginator().blocks(pages).actionIdPrefix("results").pageSize(3).build(),
    ).toEqual([
      sectionBlock({ text: "One" }),
      sectionBlock({ text: "Two" }),
      sectionBlock({ text: "Three" }),
    ]);
  });

  it("rejects invalid page configuration at build time", () => {
    expect(() =>
      Paginator()
        .blocks(pages)
        .actionIdPrefix("results")
        .page(4)
        .pageSize(1)
        .build(),
    ).toThrow(/between 1 and 3/);
  });
});
