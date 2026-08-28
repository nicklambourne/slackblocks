import { describe, expect, it } from "vitest";

import {
  Accordion,
  AccordionSection,
  Message,
  SectionBlock,
  containerBlock,
  message,
  sectionBlock,
} from "../src/index.js";

describe("Accordion", () => {
  it("renders Slack-native collapsible containers", () => {
    expect(
      Accordion()
        .sections(
          AccordionSection()
            .title("Details")
            .subtitle("More information")
            .blocks(SectionBlock().text("Hidden initially")),
        )
        .sections(
          AccordionSection()
            .title("Summary")
            .expanded(true)
            .blocks(SectionBlock().text("Visible initially")),
        )
        .build(),
    ).toEqual([
      containerBlock({
        title: "Details",
        subtitle: "More information",
        childBlocks: [sectionBlock({ text: "Hidden initially" })],
        isCollapsible: true,
        defaultCollapsed: true,
      }),
      containerBlock({
        title: "Summary",
        childBlocks: [sectionBlock({ text: "Visible initially" })],
        isCollapsible: true,
        defaultCollapsed: false,
      }),
    ]);
  });

  it("expands directly inside a fluent message", () => {
    expect(
      Message()
        .channel("C123")
        .blocks(
          Accordion().sections(
            AccordionSection()
              .title("Details")
              .blocks(SectionBlock().text("Content")),
          ),
        )
        .build(),
    ).toEqual(
      message({
        channel: "C123",
        blocks: [
          containerBlock({
            title: "Details",
            childBlocks: [sectionBlock({ text: "Content" })],
            isCollapsible: true,
            defaultCollapsed: true,
          }),
        ],
      }),
    );
  });

  it("rejects empty and non-accordion content", () => {
    expect(() => Accordion().build()).toThrow(/at least one section/);
    expect(() =>
      Accordion().sections(SectionBlock().text("Not collapsible")).build(),
    ).toThrow(/AccordionSection/);
  });
});
