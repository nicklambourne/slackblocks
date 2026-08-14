import { readFileSync, readdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

import limits from "../../spec/limits.json" with { type: "json" };

import * as api from "../src/index.js";
import {
  specVersion,
  actionsBlock,
  alertBlock,
  areaChart,
  axisConfig,
  barChart,
  cardBlock,
  carouselBlock,
  chartSegment,
  attachment,
  button,
  channelMultiSelect,
  channelSelect,
  checkboxes,
  confirmation,
  containerBlock,
  contextBlock,
  contextActionsBlock,
  conversationFilter,
  conversationMultiSelect,
  conversationSelect,
  datePicker,
  dateTimePicker,
  dataPoint,
  dataSeries,
  dataTableBlock,
  dataVisualizationBlock,
  dispatchActionConfiguration,
  dividerBlock,
  emailInput,
  externalMultiSelect,
  externalSelect,
  feedbackButton,
  feedbackButtons,
  fileBlock,
  fileInput,
  headerBlock,
  homeTab,
  imageElement,
  imageBlock,
  iconButton,
  inputBlock,
  inputParameter,
  markdownBlock,
  message,
  messageResponse,
  modal,
  mrkdwn,
  numberInput,
  option,
  optionGroup,
  overflow,
  pieChart,
  planBlock,
  plainTextInput,
  radioButtons,
  rawNumber,
  rawText,
  richText,
  richTextBlock,
  richTextChannel,
  richTextCodeBlock,
  richTextEmoji,
  richTextInput,
  richTextLink,
  richTextList,
  richTextQuote,
  richTextSection,
  richTextUser,
  richTextUserGroup,
  sectionBlock,
  slackIcon,
  staticMultiSelect,
  staticSelect,
  tableBlock,
  taskCardBlock,
  timePicker,
  trigger,
  type ErrorCategory,
  InvalidUsageError,
  plainText,
  urlInput,
  urlSource,
  userMultiSelect,
  userSelect,
  videoBlock,
  webhookMessage,
  workflow,
  workflowButton,
  lineChart,
} from "../src/index.js";

const PACKAGE_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const SPEC_ROOT = resolve(PACKAGE_ROOT, "../spec");

interface ManifestFixture {
  id: string;
  slack_docs: string;
}

interface Manifest {
  spec_version: string;
  fixtures: ManifestFixture[];
}

interface Coverage {
  spec_version: string;
  capabilities: Record<string, string[]>;
}

interface InvalidManifest {
  spec_version: string;
  cases: Array<{ id: string; category: ErrorCategory; constraint: string }>;
}

function readJson<T>(path: string): T {
  return JSON.parse(readFileSync(path, "utf8")) as T;
}

function camelCase(key: string): string {
  return key.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());
}

function toCamel(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(toCamel);
  if (value !== null && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, nested]) => [camelCase(key), toCamel(nested)]),
    );
  }
  return value;
}

type FixtureInput = any;

function withoutType(input: FixtureInput): FixtureInput {
  const { type: _type, ...rest } = input;
  return rest;
}

function constructCard(payload: FixtureInput): unknown {
  const value = withoutType(payload);
  return cardBlock({
    ...value,
    heroImage: value.heroImage === undefined ? undefined : imageElement(withoutType(value.heroImage)),
    icon: value.icon === undefined ? undefined : imageElement(withoutType(value.icon)),
    actions: value.actions?.map((action: FixtureInput) => button(withoutType(action))),
    slackIcon:
      value.slackIcon === undefined ? undefined : slackIcon(value.slackIcon.name),
  });
}

function constructContextAction(payload: FixtureInput): unknown {
  const value = withoutType(payload);
  if (payload.type === "icon_button") return iconButton(value);
  if (payload.type === "feedback_buttons") {
    return feedbackButtons({
      ...value,
      positiveButton: feedbackButton(value.positiveButton),
      negativeButton: feedbackButton(value.negativeButton),
    });
  }
  throw new Error(`No context-action factory for ${String(payload.type)}`);
}

function constructChart(payload: FixtureInput): unknown {
  if (payload.type === "pie") {
    return pieChart(payload.segments.map((segment: FixtureInput) => chartSegment(segment)));
  }
  if (["area", "bar", "line"].includes(payload.type)) {
    const series =
      payload.series.map((series: FixtureInput) =>
        dataSeries({
          ...series,
          data: series.data.map((point: FixtureInput) => dataPoint(point)),
        }),
      );
    const axis = axisConfig(payload.axisConfig);
    if (payload.type === "area") return areaChart(series, axis);
    if (payload.type === "bar") return barChart(series, axis);
    return lineChart(series, axis);
  }
  throw new Error(`No chart factory for ${String(payload.type)}`);
}

function constructTask(payload: FixtureInput): unknown {
  const value = withoutType(payload);
  return taskCardBlock({
    ...value,
    details: value.details === undefined ? undefined : constructBlock(value.details),
    output: value.output === undefined ? undefined : constructBlock(value.output),
    sources: value.sources?.map((source: FixtureInput) => urlSource(withoutType(source))),
  });
}

function constructBlock(payload: FixtureInput): unknown {
  const value = withoutType(payload);
  switch (payload.type) {
    case "actions": return actionsBlock(value);
    case "alert": return alertBlock(value);
    case "card": return constructCard(payload);
    case "carousel":
      return carouselBlock({
        ...value,
        elements: value.elements.map((card: FixtureInput) => constructCard(card)),
      });
    case "container":
      return containerBlock({
        ...value,
        childBlocks: value.childBlocks.map((block: FixtureInput) => constructBlock(block)),
      });
    case "context": return contextBlock(value);
    case "context_actions":
      return contextActionsBlock({
        ...value,
        elements: value.elements.map((element: FixtureInput) => constructContextAction(element)),
      });
    case "data_table":
      return dataTableBlock({
        ...value,
        rows: value.rows.map((row: FixtureInput[]) =>
          row.map((cell) =>
            cell.type === "raw_number"
              ? rawNumber(cell.value, cell.text)
              : rawText(cell.text),
          ),
        ),
      });
    case "data_visualization":
      return dataVisualizationBlock({ ...value, chart: constructChart(value.chart) });
    case "divider": return dividerBlock(value);
    case "file": return fileBlock(value);
    case "header": return headerBlock(value);
    case "image": return imageBlock(value);
    case "input": return inputBlock(value);
    case "markdown": return markdownBlock(value);
    case "plan":
      return planBlock({
        ...value,
        tasks: value.tasks?.map((task: FixtureInput) => constructTask(task)),
      });
    case "rich_text": return richTextBlock(value);
    case "section": return sectionBlock(value);
    case "table": return tableBlock(value);
    case "task_card": return constructTask(payload);
    case "video": return videoBlock(value);
    default: throw new Error(`No block factory for ${String(payload.type)}`);
  }
}

function constructElement(input: FixtureInput): unknown {
  const value = withoutType(input);
  switch (input.type) {
    case "button": return button(value);
    case "channels_select": return channelSelect(value);
    case "checkboxes": return checkboxes(value);
    case "conversations_select": return conversationSelect(value);
    case "datepicker": return datePicker(value);
    case "datetimepicker": return dateTimePicker(value);
    case "email_text_input": return emailInput(value);
    case "feedback_buttons": return feedbackButtons(value);
    case "external_select": return externalSelect(value);
    case "image": return imageElement(value);
    case "icon_button": return iconButton(value);
    case "multi_channels_select": return channelMultiSelect(value);
    case "multi_conversations_select": return conversationMultiSelect(value);
    case "multi_external_select": return externalMultiSelect(value);
    case "multi_static_select": return staticMultiSelect(value);
    case "multi_users_select": return userMultiSelect(value);
    case "number_input": return numberInput(value);
    case "overflow": return overflow(value);
    case "plain_text_input": return plainTextInput(value);
    case "radio_buttons": return radioButtons(value);
    case "rich_text_input": return richTextInput(value);
    case "static_select": return staticSelect(value);
    case "timepicker": return timePicker(value);
    case "url_text_input": return urlInput(value);
    case "url": return urlSource(value);
    case "users_select": return userSelect(value);
    case "workflow_button": return workflowButton(value);
    default: throw new Error(`No element factory for ${String(input.type)}`);
  }
}

function constructObject(id: string, input: FixtureInput): unknown {
  switch (id) {
    case "confirmation_dialogue_basic": return confirmation(input);
    case "conversation_filter_basic": return conversationFilter(input);
    case "dispatch_action_configuration_basic": return dispatchActionConfiguration(input);
    case "input_parameter_basic": return inputParameter(input);
    case "option_basic":
    case "option_value_at_limit": return option(input);
    case "option_group_basic": return optionGroup(input);
    case "text_markdown_basic":
    case "text_markdown_verbatim": return mrkdwn(input.text, withoutType(input));
    case "text_plaintext_basic":
    case "text_plaintext_emoji": return plainText(input.text, withoutType(input));
    case "trigger_basic": return trigger(input);
    case "workflow_basic": return workflow(input);
    default: throw new Error(`No composition-object factory for ${id}`);
  }
}

function constructRichText(input: FixtureInput): unknown {
  const value = withoutType(input);
  switch (input.type) {
    case "channel": return richTextChannel(value.channelId, value.style);
    case "emoji": return richTextEmoji(value.name);
    case "link": return richTextLink(value);
    case "rich_text_list": return richTextList(value);
    case "rich_text_preformatted": return richTextCodeBlock(value.elements, { border: value.border });
    case "rich_text_quote": return richTextQuote(value.elements, { border: value.border });
    case "rich_text_section": return richTextSection(value.elements);
    case "text": return richText(value.text, value.style);
    case "user": return richTextUser(value.userId, value.style);
    case "usergroup": return richTextUserGroup(value.usergroupId, value.style);
    default: throw new Error(`No rich-text factory for ${String(input.type)}`);
  }
}

function constructFixture(id: string, expected: FixtureInput): unknown {
  const [category, name] = id.split("/") as [string, string];
  const input = toCamel(expected) as FixtureInput;
  switch (category) {
    case "attachments": return attachment(input);
    case "blocks": return constructBlock(input);
    case "elements": return name === "file_input_basic" ? fileInput(input) : constructElement(input);
    case "messages":
      if (name === "message_response") return messageResponse(input);
      return name.startsWith("webhook_") ? webhookMessage(input) : message(input);
    case "objects": return constructObject(name, input);
    case "rich_text": return constructRichText(input);
    case "views": return input.type === "modal" ? modal(withoutType(input)) : homeTab(withoutType(input));
    default: throw new Error(`Unknown fixture category ${category}`);
  }
}

const manifest = readJson<Manifest>(resolve(SPEC_ROOT, "manifest.json"));
const packageJson = readJson<{ slackblocksSpecVersion: string }>(
  resolve(dirname(fileURLToPath(import.meta.url)), "../package.json"),
);
const coverage = readJson<Coverage>(resolve(SPEC_ROOT, "coverage.json"));

function scalarPaths(value: unknown, prefix: string[] = []): string[] {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return [prefix.join(".")];
  }
  return Object.entries(value).flatMap(([key, nested]) =>
    scalarPaths(nested, [...prefix, key]),
  );
}

describe("valid conformance corpus", () => {
  it("declares the current spec and a non-empty fixture corpus", () => {
    expect(manifest.spec_version).toBe(specVersion);
    expect(packageJson.slackblocksSpecVersion).toBe(specVersion);
    expect(manifest.fixtures.length).toBeGreaterThan(0);
    expect(new Set(manifest.fixtures.map(({ id }) => id)).size).toBe(
      manifest.fixtures.length,
    );
  });

  it("covers every declared shared JSON capability with an official fixture", () => {
    expect(coverage.spec_version).toBe(manifest.spec_version);
    const fixtures = new Map(manifest.fixtures.map((fixture) => [fixture.id, fixture]));
    for (const [capability, fixtureIds] of Object.entries(coverage.capabilities)) {
      expect(fixtureIds.length, capability).toBeGreaterThan(0);
      for (const fixtureId of fixtureIds) {
        const fixture = fixtures.get(fixtureId);
        expect(fixture, `${capability} -> ${fixtureId}`).toBeDefined();
        expect(fixture?.slack_docs).toMatch(/^https:\/\/docs\.slack\.dev\//);
      }
    }
  });

  for (const fixture of manifest.fixtures) {
    it(fixture.id, () => {
      const expected = readJson<Record<string, unknown>>(
        resolve(SPEC_ROOT, "fixtures/valid", `${fixture.id}.json`),
      );
      expect(constructFixture(fixture.id, expected)).toEqual(expected);
    });
  }

  it("has an empty TypeScript skip list", () => {
    const entries = readFileSync(resolve(PACKAGE_ROOT, "conformance/skiplist.txt"), "utf8")
      .split("\n")
      .filter((line) => line !== "" && !line.startsWith("#"));
    expect(entries).toEqual([]);
  });

  it("contains no undeclared fixture files", () => {
    const ids = new Set(manifest.fixtures.map(({ id }) => id));
    const categories = readdirSync(resolve(SPEC_ROOT, "fixtures/valid"));
    const files = categories.flatMap((category) =>
      readdirSync(resolve(SPEC_ROOT, "fixtures/valid", category)).map(
        (file) => `${category}/${file.replace(/\.json$/, "")}`,
      ),
    );
    expect(new Set(files)).toEqual(ids);
  });
});

const choice = () => option({ text: "A", value: "a" });
const video = (overrides: Record<string, unknown> = {}) =>
  videoBlock({
    altText: "Video",
    thumbnailUrl: "https://example.com/thumbnail.png",
    title: "Title",
    videoUrl: "https://example.com/video.mp4",
    ...overrides,
  });
const validCard = () => cardBlock({ title: "Card" });
const validFeedbackButton = () => feedbackButton({ text: "Good", value: "good" });
const validTableRows = () => [
  [rawText("Name")],
  [rawText("Alice")],
];
const validAxis = () => axisConfig({ categories: ["A"] });
const validSeries = (name = "Series") =>
  dataSeries({ name, data: [dataPoint({ label: "A", value: 1 })] });

const invalidCases: Record<string, () => unknown> = {
  "text-empty": () => plainText(""),
  "text-too-long": () => plainText("x".repeat(3001)),
  "button-action-id-too-long": () =>
    button({ text: "A", actionId: "x".repeat(limits.action_id.max_length + 1) }),
  "button-text-too-long": () =>
    button({ text: "x".repeat(limits.button.text.max_length + 1), actionId: "a" }),
  "button-url-too-long": () =>
    button({ text: "A", actionId: "a", url: "x".repeat(limits.button.url.max_length + 1) }),
  "button-value-too-long": () =>
    button({
      text: "A",
      actionId: "a",
      value: "x".repeat(limits.button.value.max_length + 1),
    }),
  "confirmation-title-too-long": () =>
    confirmation({
      title: "x".repeat(limits.confirmation.title.max_length + 1),
      text: "Text",
      confirm: "Yes",
      deny: "No",
    }),
  "confirmation-text-too-long": () =>
    confirmation({
      title: "Title",
      text: "x".repeat(limits.confirmation.text.max_length + 1),
      confirm: "Yes",
      deny: "No",
    }),
  "confirmation-confirm-too-long": () =>
    confirmation({
      title: "Title",
      text: "Text",
      confirm: "x".repeat(limits.confirmation.confirm.max_length + 1),
      deny: "No",
    }),
  "confirmation-deny-too-long": () =>
    confirmation({
      title: "Title",
      text: "Text",
      confirm: "Yes",
      deny: "x".repeat(limits.confirmation.deny.max_length + 1),
    }),
  "option-text-too-long": () =>
    option({ text: "x".repeat(limits.option.text.max_length + 1), value: "a" }),
  "option-value-too-long": () =>
    option({ text: "A", value: "x".repeat(limits.option.value.max_length + 1) }),
  "option-description-too-long": () =>
    option({
      text: "A",
      value: "a",
      description: "x".repeat(limits.option.description.max_length + 1),
    }),
  "option-group-label-too-long": () =>
    optionGroup({
      label: "x".repeat(limits.option_group.label.max_length + 1),
      options: [choice()],
    }),
  "option-group-empty": () => optionGroup({ label: "Group", options: [] }),
  "option-group-too-many-options": () =>
    optionGroup({
      label: "Group",
      options: Array.from({ length: limits.option_group.options.max_items + 1 }, choice),
    }),
  "select-placeholder-too-long": () =>
    staticSelect({
      actionId: "a",
      options: [choice()],
      placeholder: "x".repeat(limits.select.placeholder.max_length + 1),
    }),
  "select-too-many-options": () =>
    staticSelect({
      actionId: "a",
      options: Array.from({ length: limits.select.options.max_items + 1 }, choice),
    }),
  "select-too-many-option-groups": () =>
    staticSelect({
      actionId: "a",
      optionGroups: Array.from(
        { length: limits.select.option_groups.max_items + 1 },
        () => optionGroup({ label: "Group", options: [choice()] }),
      ),
    }),
  "overflow-empty": () => overflow({ actionId: "a", options: [] }),
  "overflow-too-many-options": () =>
    overflow({
      actionId: "a",
      options: Array.from({ length: limits.overflow.options.max_items + 1 }, choice),
    }),
  "checkboxes-empty": () => checkboxes({ actionId: "a", options: [] }),
  "checkboxes-too-many-options": () =>
    checkboxes({
      actionId: "a",
      options: Array.from({ length: limits.checkboxes.options.max_items + 1 }, choice),
    }),
  "radio-buttons-empty": () => radioButtons({ actionId: "a", options: [] }),
  "radio-buttons-too-many-options": () =>
    radioButtons({
      actionId: "a",
      options: Array.from({ length: limits.radio_buttons.options.max_items + 1 }, choice),
    }),
  // Astral-plane emoji pin that the limit counts Unicode code points.
  "option-url-too-long": () =>
    option({
      text: "A",
      value: "a",
      url: "\u{1F642}".repeat(limits.option.url.max_length + 1),
    }),
  "url-source-url-empty": () => urlSource({ url: "", text: "text" }),
  "url-source-url-too-long": () =>
    urlSource({ url: "x".repeat(limits.url_source.url.max_length + 1), text: "text" }),
  // Both implementations pin the table block to at most 100 rows.
  "table-too-many-rows": () =>
    tableBlock({ rows: Array.from({ length: 101 }, () => [rawText("A")]) }),
  "table-ragged-rows": () =>
    tableBlock({ rows: [[rawText("A"), rawText("B")], [rawText("C")]] }),
  "table-column-settings-mismatch": () =>
    tableBlock({
      rows: [[rawText("A"), rawText("B")]],
      columnSettings: [{ isWrapped: true }],
    }),
  "file-input-max-files-too-small": () =>
    fileInput({ actionId: "a", maxFiles: limits.file_input.max_files.min - 1 }),
  "file-input-max-files-too-large": () =>
    fileInput({ actionId: "a", maxFiles: limits.file_input.max_files.max + 1 }),
  "plain-text-input-max-length-too-large": () =>
    plainTextInput({
      actionId: "a",
      maxLength: limits.plain_text_input.max_length.max + 1,
    }),
  "actions-too-many-elements": () =>
    actionsBlock({
      elements: Array.from({ length: limits.actions.elements.max_items + 1 }, () =>
        button({ text: "A", actionId: "a" }),
      ),
    }),
  "context-too-many-elements": () =>
    contextBlock({
      elements: Array.from(
        { length: limits.context.elements.max_items + 1 },
        () => mrkdwn("A"),
      ),
    }),
  "header-text-too-long": () =>
    headerBlock({ text: "x".repeat(limits.header.text.max_length + 1) }),
  "image-url-too-long": () =>
    imageBlock({ imageUrl: "x".repeat(limits.image.image_url.max_length + 1), altText: "Alt" }),
  "image-alt-text-too-long": () =>
    imageBlock({
      imageUrl: "https://example.com/image.png",
      altText: "x".repeat(limits.image.alt_text.max_length + 1),
    }),
  "input-label-too-long": () =>
    inputBlock({
      label: "x".repeat(limits.input.label.max_length + 1),
      element: plainTextInput({ actionId: "a" }),
    }),
  "input-hint-too-long": () =>
    inputBlock({
      label: "Label",
      hint: "x".repeat(limits.input.hint.max_length + 1),
      element: plainTextInput({ actionId: "a" }),
    }),
  "markdown-empty": () => markdownBlock({ text: "" }),
  "markdown-too-long": () =>
    markdownBlock({ text: "x".repeat(limits.markdown.text.max_length + 1) }),
  "section-text-too-long": () =>
    sectionBlock({ text: "x".repeat(limits.section.text.max_length + 1) }),
  "section-too-many-fields": () =>
    sectionBlock({ fields: Array(limits.section.fields.max_items + 1).fill("x") }),
  "section-field-too-long": () =>
    sectionBlock({ fields: ["x".repeat(limits.section.fields.item_max_length + 1)] }),
  "video-alt-text-empty": () => video({ altText: "" }),
  "video-alt-text-too-long": () =>
    video({ altText: "x".repeat(limits.video.alt_text.max_length + 1) }),
  "video-title-too-long": () =>
    video({ title: "x".repeat(limits.video.title.max_length + 1) }),
  "video-author-name-too-long": () =>
    video({ authorName: "x".repeat(limits.video.author_name.max_length + 1) }),
  "video-description-too-long": () =>
    video({ description: "x".repeat(limits.video.description.max_length + 1) }),
  "video-provider-name-too-long": () =>
    video({ providerName: "x".repeat(limits.video.provider_name.max_length + 1) }),
  "view-missing-blocks": () => homeTab({ blocks: [] }),
  "view-too-many-blocks": () =>
    homeTab({
      blocks: Array.from({ length: limits.view.blocks.max_items + 1 }, () => dividerBlock()),
    }),
  "view-private-metadata-too-long": () =>
    homeTab({
      blocks: [dividerBlock()],
      privateMetadata: "x".repeat(limits.view.private_metadata.max_length + 1),
    }),
  "view-callback-id-too-long": () =>
    homeTab({
      blocks: [dividerBlock()],
      callbackId: "x".repeat(limits.view.callback_id.max_length + 1),
    }),
  "view-title-too-long": () =>
    modal({
      title: "x".repeat(limits.view.title.max_length + 1),
      blocks: [dividerBlock()],
    }),
  "view-close-too-long": () =>
    modal({
      title: "Title",
      close: "x".repeat(limits.view.close.max_length + 1),
      blocks: [dividerBlock()],
    }),
  "view-submit-too-long": () =>
    modal({
      title: "Title",
      submit: "x".repeat(limits.view.submit.max_length + 1),
      blocks: [dividerBlock()],
    }),
  "section-missing-content": () => sectionBlock({}),
  "static-select-options-and-groups": () =>
    staticSelect({ actionId: "a", options: [choice()], optionGroups: [{ label: plainText("A") }] }),
  "image-url-and-slack-file": () =>
    imageElement({
      altText: "image",
      imageUrl: "https://example.com/image.png",
      slackFile: { id: "F123" },
    } as any),
  "number-input-inverted-range": () =>
    numberInput({ actionId: "a", isDecimalAllowed: true, minValue: 2, maxValue: 1 }),
  "context-invalid-element": () => contextBlock({ elements: [dividerBlock()] }),
  "input-invalid-element": () =>
    inputBlock({ label: "Label", element: button({ text: "A", actionId: "a" }) }),
  "block-id-too-long": () =>
    dividerBlock({ blockId: "x".repeat(limits.block_id.max_length + 1) }),
  "button-accessibility-label-too-long": () =>
    button({
      text: "A",
      actionId: "a",
      accessibilityLabel: "x".repeat(limits.button.accessibility_label.max_length + 1),
    }),
  "alert-text-too-long": () =>
    alertBlock({ text: "x".repeat(limits.alert.text.max_length + 1) }),
  "card-title-too-long": () =>
    cardBlock({ title: "x".repeat(limits.card.title.max_length + 1) }),
  "card-subtitle-too-long": () =>
    cardBlock({ title: "Card", subtitle: "x".repeat(limits.card.subtitle.max_length + 1) }),
  "card-body-too-long": () =>
    cardBlock({ body: "x".repeat(limits.card.body.max_length + 1) }),
  "card-too-many-actions": () =>
    cardBlock({
      actions: Array.from({ length: limits.card.actions.max_items + 1 }, (_, index) =>
        button({ text: "A", actionId: `a-${index}` }),
      ),
    }),
  "card-subtext-too-long": () =>
    cardBlock({ title: "Card", subtext: "x".repeat(limits.card.subtext.max_length + 1) }),
  "carousel-empty": () => carouselBlock({ elements: [] }),
  "carousel-too-many-cards": () =>
    carouselBlock({
      elements: Array.from({ length: limits.carousel.elements.max_items + 1 }, validCard),
    }),
  "container-title-too-long": () =>
    containerBlock({
      title: "x".repeat(limits.container.title.max_length + 1),
      childBlocks: [dividerBlock()],
    }),
  "container-subtitle-too-long": () =>
    containerBlock({
      title: "Container",
      subtitle: "x".repeat(limits.container.subtitle.max_length + 1),
      childBlocks: [dividerBlock()],
    }),
  "container-too-many-child-blocks": () =>
    containerBlock({
      title: "Container",
      childBlocks: Array.from(
        { length: limits.container.child_blocks.max_items + 1 },
        () => dividerBlock(),
      ),
    }),
  "context-actions-too-many-elements": () =>
    contextActionsBlock({
      elements: Array.from(
        { length: limits.context_actions.elements.max_items + 1 },
        (_, index) => iconButton({ text: "Delete", actionId: `delete-${index}` }),
      ),
    }),
  "feedback-button-text-too-long": () =>
    feedbackButtons({
      positiveButton: feedbackButton({
        text: "x".repeat(limits.feedback_button.text.max_length + 1),
        value: "good",
      }),
      negativeButton: validFeedbackButton(),
    }),
  "feedback-button-value-too-long": () =>
    feedbackButtons({
      positiveButton: feedbackButton({
        text: "Good",
        value: "x".repeat(limits.feedback_button.value.max_length + 1),
      }),
      negativeButton: validFeedbackButton(),
    }),
  "feedback-button-accessibility-label-too-long": () =>
    feedbackButtons({
      positiveButton: feedbackButton({
        text: "Good",
        value: "good",
        accessibilityLabel: "x".repeat(
          limits.feedback_button.accessibility_label.max_length + 1,
        ),
      }),
      negativeButton: validFeedbackButton(),
    }),
  "icon-button-too-many-visible-users": () =>
    iconButton({
      text: "Delete",
      visibleToUserIds: Array.from(
        { length: limits.icon_button.visible_to_user_ids.max_items + 1 },
        (_, index) => `U${index}`,
      ),
    }),
  "data-table-too-few-rows": () =>
    dataTableBlock({ rows: [[rawText("Name")]], caption: "Names" }),
  "data-table-too-many-rows": () =>
    dataTableBlock({
      rows: Array.from({ length: limits.data_table.rows.max_items + 1 }, () => [rawText("A")]),
      caption: "Names",
    }),
  "data-table-too-few-columns": () =>
    dataTableBlock({ rows: [[], []], caption: "Empty" }),
  "data-table-too-many-columns": () =>
    dataTableBlock({
      rows: [0, 1].map(() =>
        Array.from({ length: limits.data_table.columns.max_items + 1 }, () => rawText("A")),
      ),
      caption: "Wide",
    }),
  "data-table-page-size-too-small": () =>
    dataTableBlock({
      rows: validTableRows(),
      caption: "Names",
      pageSize: limits.data_table.page_size.min - 1,
    }),
  "data-table-page-size-too-large": () =>
    dataTableBlock({
      rows: validTableRows(),
      caption: "Names",
      pageSize: limits.data_table.page_size.max + 1,
    }),
  "data-table-cell-text-empty": () =>
    dataTableBlock({ rows: [[rawText("Name")], [rawText("")]], caption: "Names" }),
  "data-table-content-too-long": () =>
    dataTableBlock({
      rows: [[rawText("Name")], [rawText("x".repeat(limits.data_table.content.max_length))]],
      caption: "Names",
    }),
  "data-visualization-title-too-long": () =>
    dataVisualizationBlock({
      title: "x".repeat(limits.data_visualization.title.max_length + 1),
      chart: pieChart([chartSegment({ label: "A", value: 1 })]),
    }),
  "pie-chart-empty": () => pieChart([]),
  "pie-chart-too-many-segments": () =>
    pieChart(
      Array.from({ length: limits.data_visualization.segments.max_items + 1 }, (_, index) =>
        chartSegment({ label: `S${index}`, value: 1 }),
      ),
    ),
  "chart-segment-label-too-long": () =>
    chartSegment({
      label: "x".repeat(limits.data_visualization.segment.label.max_length + 1),
      value: 1,
    }),
  "chart-segment-value-not-positive": () => chartSegment({ label: "A", value: 0 }),
  "chart-series-empty": () => lineChart([], validAxis()),
  "chart-too-many-series": () =>
    lineChart(
      Array.from({ length: limits.data_visualization.series.max_items + 1 }, (_, index) =>
        validSeries(`S${index}`),
      ),
      validAxis(),
    ),
  "data-series-name-too-long": () =>
    dataSeries({
      name: "x".repeat(limits.data_visualization.series_name.max_length + 1),
      data: [dataPoint({ label: "A", value: 1 })],
    }),
  "data-series-empty": () => dataSeries({ name: "Series", data: [] }),
  "data-series-too-many-points": () =>
    dataSeries({
      name: "Series",
      data: Array.from({ length: limits.data_visualization.data.max_items + 1 }, (_, index) =>
        dataPoint({ label: `P${index}`, value: index }),
      ),
    }),
  "data-point-label-too-long": () =>
    dataPoint({
      label: "x".repeat(limits.data_visualization.point_label.max_length + 1),
      value: 1,
    }),
  "axis-categories-empty": () => axisConfig({ categories: [] }),
  "axis-too-many-categories": () =>
    axisConfig({
      categories: Array.from(
        { length: limits.data_visualization.categories.max_items + 1 },
        (_, index) => `C${index}`,
      ),
    }),
  "axis-category-label-too-long": () =>
    axisConfig({
      categories: ["x".repeat(limits.data_visualization.category_label.max_length + 1)],
    }),
  "axis-label-too-long": () =>
    axisConfig({
      categories: ["A"],
      xLabel: "x".repeat(limits.data_visualization.axis_label.max_length + 1),
    }),
};

const invalidManifest = readJson<InvalidManifest>(
  resolve(SPEC_ROOT, "fixtures/invalid/manifest.json"),
);

// Exported functions that intentionally have no entry in coverage.json
// because they do not, by themselves, produce Slack JSON.
const FACTORY_EXCLUSIONS = new Set([
  // Error classes (validation outcomes, not JSON producers).
  "InvalidUsageError",
  "LengthError",
  "OutOfRangeError",
  "MutualExclusivityError",
  "TypeMismatchError",
  "MissingRequiredError",
  // Utilities: text coercion, builder URL, and explicit validation.
  "asText",
  "blockKitBuilderUrl",
  "assertValid",
  "validate",
]);

// Mapping from exported factory function to its capability in coverage.json.
const CAPABILITY_BY_FACTORY: Record<string, string> = {
  actionsBlock: "blocks.actions",
  alertBlock: "blocks.alert",
  cardBlock: "blocks.card",
  carouselBlock: "blocks.carousel",
  containerBlock: "blocks.container",
  contextActionsBlock: "blocks.context_actions",
  contextBlock: "blocks.context",
  dataTableBlock: "blocks.data_table",
  dataVisualizationBlock: "blocks.data_visualization",
  dividerBlock: "blocks.divider",
  fileBlock: "blocks.file",
  headerBlock: "blocks.header",
  imageBlock: "blocks.image",
  inputBlock: "blocks.input",
  markdownBlock: "blocks.markdown",
  planBlock: "blocks.plan",
  richTextBlock: "blocks.rich_text",
  sectionBlock: "blocks.section",
  tableBlock: "blocks.table",
  taskCardBlock: "blocks.task_card",
  videoBlock: "blocks.video",
  // Chart constructors serialize inside the data-visualization block.
  areaChart: "blocks.data_visualization",
  barChart: "blocks.data_visualization",
  lineChart: "blocks.data_visualization",
  pieChart: "blocks.data_visualization",
  button: "elements.button",
  channelMultiSelect: "elements.multi_select_channels",
  channelSelect: "elements.select_channels",
  checkboxes: "elements.checkboxes",
  conversationMultiSelect: "elements.multi_select_conversations",
  conversationSelect: "elements.select_conversations",
  datePicker: "elements.date_picker",
  dateTimePicker: "elements.datetime_picker",
  emailInput: "elements.email_input",
  externalMultiSelect: "elements.multi_select_external",
  externalSelect: "elements.select_external",
  feedbackButton: "objects.feedback_button",
  feedbackButtons: "elements.feedback_buttons",
  fileInput: "elements.file_input",
  iconButton: "elements.icon_button",
  imageElement: "elements.image",
  numberInput: "elements.number_input",
  overflow: "elements.overflow",
  plainTextInput: "elements.plain_text_input",
  radioButtons: "elements.radio_buttons",
  richTextInput: "elements.rich_text_input",
  staticMultiSelect: "elements.multi_select_static",
  staticSelect: "elements.select_static",
  timePicker: "elements.time_picker",
  urlInput: "elements.url_input",
  urlSource: "elements.url_source",
  userMultiSelect: "elements.multi_select_users",
  userSelect: "elements.select_users",
  workflowButton: "elements.workflow_button",
  attachment: "messages.attachment",
  message: "messages.message",
  messageResponse: "messages.message_response",
  webhookMessage: "messages.webhook_message",
  axisConfig: "objects.axis_config",
  chartSegment: "objects.chart_segment",
  columnSettings: "objects.column_settings",
  confirmation: "objects.confirmation",
  conversationFilter: "objects.conversation_filter",
  dataPoint: "objects.data_point",
  dataSeries: "objects.data_series",
  dispatchActionConfiguration: "objects.dispatch_action_configuration",
  inputParameter: "objects.input_parameter",
  mrkdwn: "objects.markdown_text",
  option: "objects.option",
  optionGroup: "objects.option_group",
  plainText: "objects.plain_text",
  rawNumber: "objects.raw_number",
  rawText: "objects.raw_text",
  slackFile: "objects.slack_file",
  slackIcon: "objects.slack_icon",
  trigger: "objects.trigger",
  workflow: "objects.workflow",
  richText: "rich_text.text",
  richTextChannel: "rich_text.channel",
  richTextCodeBlock: "rich_text.code_block",
  richTextEmoji: "rich_text.emoji",
  richTextLink: "rich_text.link",
  richTextList: "rich_text.list",
  richTextQuote: "rich_text.quote",
  richTextSection: "rich_text.section",
  richTextUser: "rich_text.user",
  richTextUserGroup: "rich_text.user_group",
  homeTab: "views.home_tab",
  modal: "views.modal",
};

describe("capability registry", () => {
  it("maps every exported factory to a registered capability", () => {
    const exported = Object.entries(api)
      .filter(([, value]) => typeof value === "function")
      .map(([exportName]) => exportName)
      .filter((exportName) => !FACTORY_EXCLUSIONS.has(exportName));
    expect(new Set(exported)).toEqual(new Set(Object.keys(CAPABILITY_BY_FACTORY)));
  });

  it("registers every mapped capability in coverage.json, and only those", () => {
    for (const [factory, capability] of Object.entries(CAPABILITY_BY_FACTORY)) {
      expect(coverage.capabilities[capability], `${factory} -> ${capability}`).toBeDefined();
    }
    expect(new Set(Object.values(CAPABILITY_BY_FACTORY))).toEqual(
      new Set(Object.keys(coverage.capabilities)),
    );
  });

  it("keeps the exclusions list disjoint from the mapping", () => {
    for (const excluded of FACTORY_EXCLUSIONS) {
      expect(CAPABILITY_BY_FACTORY[excluded]).toBeUndefined();
    }
  });
});

describe("invalid conformance corpus", () => {
  it("exercises every scalar limit in the shared registry", () => {
    const covered = new Set(invalidManifest.cases.map(({ constraint }) => constraint));
    expect(scalarPaths(limits).filter((path) => !covered.has(path))).toEqual([]);
  });

  it("contains unique case IDs and constraints", () => {
    expect(new Set(invalidManifest.cases.map(({ id }) => id)).size).toBe(
      invalidManifest.cases.length,
    );
    expect(new Set(invalidManifest.cases.map(({ constraint }) => constraint)).size).toBe(
      invalidManifest.cases.length,
    );
  });
  it("has a construction for every case", () => {
    expect(Object.keys(invalidCases).sort()).toEqual(
      invalidManifest.cases.map(({ id }) => id).sort(),
    );
  });

  for (const invalidCase of invalidManifest.cases) {
    it(`${invalidCase.id} -> ${invalidCase.category}`, () => {
      try {
        invalidCases[invalidCase.id]?.();
        expect.fail("construction did not throw");
      } catch (error) {
        expect(error).toBeInstanceOf(InvalidUsageError);
        expect((error as InvalidUsageError).category).toBe(invalidCase.category);
      }
    });
  }
});
