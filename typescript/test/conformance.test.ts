import { readFileSync, readdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

import limits from "../../spec/limits.json" with { type: "json" };

import {
  actionsBlock,
  attachment,
  button,
  channelMultiSelect,
  channelSelect,
  checkboxes,
  confirmation,
  contextBlock,
  conversationFilter,
  conversationMultiSelect,
  conversationSelect,
  datePicker,
  dateTimePicker,
  dispatchActionConfiguration,
  dividerBlock,
  emailInput,
  externalMultiSelect,
  externalSelect,
  fileBlock,
  fileInput,
  headerBlock,
  homeTab,
  imageElement,
  imageBlock,
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
  plainTextInput,
  radioButtons,
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
  staticMultiSelect,
  staticSelect,
  tableBlock,
  timePicker,
  trigger,
  type ErrorCategory,
  InvalidUsageError,
  plainText,
  urlInput,
  userMultiSelect,
  userSelect,
  videoBlock,
  webhookMessage,
  workflow,
  workflowButton,
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

function constructBlock(payload: FixtureInput): unknown {
  const value = withoutType(payload);
  switch (payload.type) {
    case "actions": return actionsBlock(value);
    case "context": return contextBlock(value);
    case "divider": return dividerBlock(value);
    case "file": return fileBlock(value);
    case "header": return headerBlock(value);
    case "image": return imageBlock(value);
    case "input": return inputBlock(value);
    case "markdown": return markdownBlock(value);
    case "rich_text": return richTextBlock(value);
    case "section": return sectionBlock(value);
    case "table": return tableBlock(value);
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
    case "external_select": return externalSelect(value);
    case "image": return imageElement(value);
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
    case "option_basic": return option(input);
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
const coverage = readJson<Coverage>(resolve(SPEC_ROOT, "coverage.json"));
const skippedCases = new Set(
  readFileSync(resolve(PACKAGE_ROOT, "conformance/skiplist.txt"), "utf8")
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line !== "" && !line.startsWith("#"))
    .map((line) => line.split(/\s+/, 1)[0]!),
);

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
    expect(manifest.spec_version).toBe("1.0.0");
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
    const testFixture = skippedCases.has(fixture.id) ? it.skip : it;
    testFixture(fixture.id, () => {
      const expected = readJson<Record<string, unknown>>(
        resolve(SPEC_ROOT, "fixtures/valid", `${fixture.id}.json`),
      );
      expect(constructFixture(fixture.id, expected)).toEqual(expected);
    });
  }

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
};

const invalidManifest = readJson<InvalidManifest>(
  resolve(SPEC_ROOT, "fixtures/invalid/manifest.json"),
);

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
      invalidManifest.cases
        .filter(({ id }) => !skippedCases.has(id))
        .map(({ id }) => id)
        .sort(),
    );
  });

  it("only skips declared valid fixtures and invalid cases", () => {
    const declared = new Set([
      ...manifest.fixtures.map(({ id }) => id),
      ...invalidManifest.cases.map(({ id }) => id),
    ]);
    expect([...skippedCases].filter((id) => !declared.has(id))).toEqual([]);
  });

  for (const invalidCase of invalidManifest.cases) {
    const testInvalidCase = skippedCases.has(invalidCase.id) ? it.skip : it;
    testInvalidCase(`${invalidCase.id} -> ${invalidCase.category}`, () => {
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
