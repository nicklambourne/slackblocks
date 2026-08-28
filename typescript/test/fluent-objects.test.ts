import { describe, expect, it } from "vitest";

import {
  AxisConfig,
  BarChart,
  DataPoint,
  DataSeries,
  Markdown,
  Option,
  OptionGroup,
  PlainText,
  RichText,
  RichTextSection,
  barChart,
  dataPoint,
  dataSeries,
  mrkdwn,
  option,
  optionGroup,
  plainText,
  axisConfig,
} from "../src/index.js";

describe("fluent composition objects", () => {
  it("matches the functional text factories", () => {
    expect(PlainText().text("Hello").emoji(true).build()).toEqual(
      plainText("Hello", { emoji: true }),
    );
    expect(Markdown().text("*Hello*").verbatim(true).build()).toEqual(
      mrkdwn("*Hello*", { verbatim: true }),
    );
  });

  it("materialises nested builders and appends collection setters", () => {
    const first = Option().text("First").value("first");
    const second = Option().text("Second").value("second");

    expect(OptionGroup().label("Choices").options(first).options(second).build()).toEqual(
      optionGroup({
        label: "Choices",
        options: [
          option({ text: "First", value: "first" }),
          option({ text: "Second", value: "second" }),
        ],
      }),
    );
  });

  it("supports nested chart construction", () => {
    const point = DataPoint().label("Mon").value(12);
    const series = DataSeries().name("Requests").data(point);
    const axis = AxisConfig().categories("Mon").xLabel("Day");

    expect(BarChart().series(series).axisConfig(axis).build()).toEqual(
      barChart(
        [dataSeries({ name: "Requests", data: [dataPoint({ label: "Mon", value: 12 })] })],
        axisConfig({ categories: ["Mon"], xLabel: "Day" }),
      ),
    );
  });

  it("builds rich-text trees", () => {
    expect(
      RichTextSection()
        .elements(RichText().text("Hello").style({ bold: true }))
        .build(),
    ).toEqual({
      type: "rich_text_section",
      elements: [{ type: "text", text: "Hello", style: { bold: true } }],
    });
  });

  it("validates only when the complete object is built", () => {
    const incomplete = Option().text("Missing a value");
    expect(() => incomplete.build()).toThrow(/value/);
  });

  it("lets the last singular setter win", () => {
    expect(PlainText().text("first").text("second").build()).toEqual(
      plainText("second"),
    );
  });
});
