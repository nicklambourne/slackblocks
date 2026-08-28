import { describe, expect, it } from "vitest";

import {
  Attachment,
  Button,
  HomeTab,
  InputBlock,
  Message,
  MessageResponse,
  Modal,
  PlainTextInput,
  SectionBlock,
  WebhookMessage,
  attachment,
  button,
  homeTab,
  inputBlock,
  message,
  messageResponse,
  modal,
  plainTextInput,
  sectionBlock,
  webhookMessage,
} from "../src/index.js";

describe("fluent payloads", () => {
  it("builds a complete Web API message from nested builders", () => {
    expect(
      Message()
        .channel("C123")
        .text("Approval requested")
        .blocks(
          SectionBlock()
            .text("Please review")
            .accessory(Button().text("Open").actionId("open")),
        )
        .attachments(
          Attachment()
            .color("good")
            .fallback("Approved")
            .blocks(SectionBlock().text("Ready")),
        )
        .build(),
    ).toEqual(
      message({
        channel: "C123",
        text: "Approval requested",
        blocks: [
          sectionBlock({
            text: "Please review",
            accessory: button({ text: "Open", actionId: "open" }),
          }),
        ],
        attachments: [
          attachment({
            color: "good",
            fallback: "Approved",
            blocks: [sectionBlock({ text: "Ready" })],
          }),
        ],
      }),
    );
  });

  it("builds interaction and webhook responses", () => {
    expect(
      MessageResponse()
        .responseType("ephemeral")
        .blocks(SectionBlock().text("Only you can see this"))
        .build(),
    ).toEqual(
      messageResponse({
        responseType: "ephemeral",
        blocks: [sectionBlock({ text: "Only you can see this" })],
      }),
    );

    expect(
      WebhookMessage()
        .text("Deployment complete")
        .blocks(SectionBlock().text("Version 2.2"))
        .build(),
    ).toEqual(
      webhookMessage({
        text: "Deployment complete",
        blocks: [sectionBlock({ text: "Version 2.2" })],
      }),
    );
  });

  it("builds modal and App Home views", () => {
    const input = InputBlock()
      .label("Name")
      .element(PlainTextInput().actionId("name"));

    expect(Modal().title("Profile").submit("Save").blocks(input).build()).toEqual(
      modal({
        title: "Profile",
        submit: "Save",
        blocks: [
          inputBlock({
            label: "Name",
            element: plainTextInput({ actionId: "name" }),
          }),
        ],
      }),
    );

    expect(HomeTab().blocks(SectionBlock().text("Welcome home")).build()).toEqual(
      homeTab({ blocks: [sectionBlock({ text: "Welcome home" })] }),
    );
  });

  it("validates payload fields only at build time", () => {
    const attachment = Attachment()
      .blocks(SectionBlock().text("Status"))
      .color("not-a-color");
    expect(() => attachment.build()).toThrow(/color/);
  });
});
