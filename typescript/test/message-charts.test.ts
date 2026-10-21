import { expect, it } from "vitest";
import {
  HomeTab, LengthError, Message, MessageResponse, WebhookMessage,
  assertValid, attachment, chartSegment, dataVisualizationBlock,
  message, messageResponse, pieChart, webhookMessage,
  type JsonObject,
} from "../src/index.js";

const chart = (index: number) => dataVisualizationBlock({
  title: "Chart", blockId: `chart_${index}`,
  chart: pieChart([chartSegment({ label: "A", value: 1 })]),
});
const charts: [JsonObject, JsonObject, JsonObject] = [chart(0), chart(1), chart(2)];

it("counts charts across message blocks and attachments in every factory", () => {
  const factories = [
    (input: { blocks: JsonObject[]; attachments: JsonObject[] }) => message({ channel: "C123", ...input }),
    messageResponse,
    webhookMessage,
  ];
  const supporting = attachment({ blocks: [charts[1]] });
  const within = { blocks: [charts[0]], attachments: [supporting] };
  const beyond = {
    blocks: [charts[0], charts[2]],
    attachments: [supporting],
  };
  for (const factory of factories) {
    expect(() => factory(within)).not.toThrow();
    expect(() => factory(beyond)).toThrow(LengthError);
  }
  expect(() => assertValid({ blocks: charts })).toThrow(LengthError);
  expect(() => Message().channel("C123").blocks(...charts).build()).toThrow(LengthError);
  expect(() => MessageResponse().blocks(...charts).build()).toThrow(LengthError);
  expect(() => WebhookMessage().blocks(...charts).build()).toThrow(LengthError);
});

it("keeps Home views and opaque message metadata outside the message-wide count", () => {
  expect(() => HomeTab().blocks(...charts).build()).not.toThrow();
  const metadata = { event_type: "charts", event_payload: { blocks: charts } };
  const payload = message({ channel: "C123", blocks: charts.slice(0, 2), metadata });
  expect(payload.metadata).toEqual(metadata);
  expect(() => assertValid(payload)).not.toThrow();
});
