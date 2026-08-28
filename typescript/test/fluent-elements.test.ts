import { describe, expect, it } from "vitest";

import {
  Button,
  Checkboxes,
  Confirmation,
  ImageElement,
  Option,
  PlainText,
  StaticMultiSelect,
  Workflow,
  WorkflowButton,
  button,
  checkboxes,
  confirmation,
  imageElement,
  option,
  staticMultiSelect,
  workflow,
  workflowButton,
} from "../src/index.js";

describe("fluent elements", () => {
  it("matches a nested interactive button", () => {
    expect(
      Button()
        .text(PlainText().text("Approve"))
        .actionId("approve")
        .style("primary")
        .confirm(
          Confirmation()
            .title("Confirm")
            .text("Approve this request?")
            .confirm("Approve")
            .deny("Cancel"),
        )
        .build(),
    ).toEqual(
      button({
        text: "Approve",
        actionId: "approve",
        style: "primary",
        confirm: confirmation({
          title: "Confirm",
          text: "Approve this request?",
          confirm: "Approve",
          deny: "Cancel",
        }),
      }),
    );
  });

  it("appends options across repeated collection calls", () => {
    const first = Option().text("First").value("first");
    const second = Option().text("Second").value("second");

    expect(
      Checkboxes()
        .actionId("choices")
        .options(first)
        .options([second])
        .initialOptions(first)
        .build(),
    ).toEqual(
      checkboxes({
        actionId: "choices",
        options: [
          option({ text: "First", value: "first" }),
          option({ text: "Second", value: "second" }),
        ],
        initialOptions: [option({ text: "First", value: "first" })],
      }),
    );
  });

  it("supports mutually exclusive image sources", () => {
    expect(
      ImageElement()
        .altText("A chart")
        .imageUrl("https://example.com/chart.png")
        .build(),
    ).toEqual(
      imageElement({
        altText: "A chart",
        imageUrl: "https://example.com/chart.png",
      }),
    );
  });

  it("builds static menus and workflow buttons from nested builders", () => {
    expect(
      StaticMultiSelect()
        .actionId("pick")
        .options(Option().text("One").value("one"))
        .build(),
    ).toEqual(
      staticMultiSelect({
        actionId: "pick",
        options: [option({ text: "One", value: "one" })],
      }),
    );

    expect(
      WorkflowButton()
        .text("Run")
        .workflow(Workflow().trigger({ url: "https://example.com" }))
        .build(),
    ).toEqual(
      workflowButton({
        text: "Run",
        workflow: workflow({ trigger: { url: "https://example.com" } }),
      }),
    );
  });

  it("validates required element fields at build time", () => {
    expect(() => Button().text("Missing action ID").build()).toThrow(/action_id/);
  });
});
