import { describe, expect, it } from "vitest";

import {
  ActionsBlock,
  BarChart,
  Button,
  CardBlock,
  CarouselBlock,
  ContainerBlock,
  DataPoint,
  DataSeries,
  DataTableBlock,
  DataVisualizationBlock,
  HeaderBlock,
  PlanBlock,
  RawNumber,
  RawText,
  SectionBlock,
  TaskCardBlock,
  UrlSource,
  actionsBlock,
  axisConfig,
  barChart,
  button,
  cardBlock,
  carouselBlock,
  containerBlock,
  dataPoint,
  dataSeries,
  dataTableBlock,
  dataVisualizationBlock,
  headerBlock,
  planBlock,
  rawNumber,
  rawText,
  sectionBlock,
  taskCardBlock,
  urlSource,
} from "../src/index.js";

describe("fluent blocks", () => {
  it("composes section and action blocks from nested elements", () => {
    expect(
      SectionBlock()
        .text("Choose an action")
        .fields("Owner: Jane")
        .fields("Status: Ready")
        .accessory(Button().text("Open").actionId("open"))
        .build(),
    ).toEqual(
      sectionBlock({
        text: "Choose an action",
        fields: ["Owner: Jane", "Status: Ready"],
        accessory: button({ text: "Open", actionId: "open" }),
      }),
    );

    expect(
      ActionsBlock()
        .elements(Button().text("Approve").actionId("approve"))
        .build(),
    ).toEqual(
      actionsBlock({
        elements: [button({ text: "Approve", actionId: "approve" })],
      }),
    );
  });

  it("composes cards, carousels, and containers", () => {
    const card = CardBlock()
      .title("Release")
      .body("Version 2.2 is ready")
      .actions(Button().text("Review").actionId("review"));

    expect(CarouselBlock().elements(card).build()).toEqual(
      carouselBlock({
        elements: [
          cardBlock({
            title: "Release",
            body: "Version 2.2 is ready",
            actions: [button({ text: "Review", actionId: "review" })],
          }),
        ],
      }),
    );

    expect(
      ContainerBlock()
        .title("Summary")
        .childBlocks(HeaderBlock().text("Release"))
        .build(),
    ).toEqual(
      containerBlock({
        title: "Summary",
        childBlocks: [headerBlock({ text: "Release" })],
      }),
    );
  });

  it("builds nested tables one row at a time", () => {
    expect(
      DataTableBlock()
        .caption("Results")
        .rows([RawText().text("Name"), RawText().text("Score")])
        .rows([RawText().text("Ada"), RawNumber().value(10).text("10")])
        .build(),
    ).toEqual(
      dataTableBlock({
        caption: "Results",
        rows: [
          [rawText("Name"), rawText("Score")],
          [rawText("Ada"), rawNumber(10, "10")],
        ],
      }),
    );
  });

  it("composes charts and plan task cards", () => {
    const chart = BarChart()
      .series(
        DataSeries()
          .name("Requests")
          .data(DataPoint().label("Monday").value(12)),
      )
      .axisConfig(axisConfig({ categories: ["Monday"] }));

    expect(DataVisualizationBlock().title("Traffic").chart(chart).build()).toEqual(
      dataVisualizationBlock({
        title: "Traffic",
        chart: barChart(
          [
            dataSeries({
              name: "Requests",
              data: [dataPoint({ label: "Monday", value: 12 })],
            }),
          ],
          axisConfig({ categories: ["Monday"] }),
        ),
      }),
    );

    expect(
      PlanBlock()
        .title("Launch")
        .tasks(
          TaskCardBlock()
            .taskId("docs")
            .title("Publish docs")
            .sources(UrlSource().url("https://example.com").text("Preview")),
        )
        .build(),
    ).toEqual(
      planBlock({
        title: "Launch",
        tasks: [
          taskCardBlock({
            taskId: "docs",
            title: "Publish docs",
            sources: [urlSource({ url: "https://example.com", text: "Preview" })],
          }),
        ],
      }),
    );
  });

  it("validates incomplete blocks only at build time", () => {
    expect(() => SectionBlock().blockId("empty").build()).toThrow();
  });
});
