import { readFileSync, readdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

import {
  actions,
  attachment,
  button,
  channelMultiSelect,
  channelSelect,
  checkboxes,
  confirmation,
  context,
  conversationFilter,
  conversationMultiSelect,
  conversationSelect,
  datePicker,
  dateTimePicker,
  dispatchActionConfiguration,
  divider,
  emailInput,
  externalMultiSelect,
  externalSelect,
  fileBlock,
  fileInput,
  header,
  homeTab,
  imageElement,
  imageBlock,
  input,
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
  section,
  staticMultiSelect,
  staticSelect,
  table,
  timePicker,
  trigger,
  type ErrorCategory,
  InvalidUsageError,
  plainText,
  urlInput,
  userMultiSelect,
  userSelect,
  video,
  webhookMessage,
  workflow,
  workflowButton,
} from "../src/index.js";

const PACKAGE_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const SPEC_ROOT = resolve(PACKAGE_ROOT, "../spec");

interface Manifest {
  spec_version: string;
  fixtures: Array<{ id: string }>;
}

interface InvalidManifest {
  spec_version: string;
  cases: Array<{ id: string; category: ErrorCategory }>;
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
    case "actions": return actions(value);
    case "context": return context(value);
    case "divider": return divider(value);
    case "file": return fileBlock(value);
    case "header": return header(value);
    case "image": return imageBlock(value);
    case "input": return input(value);
    case "markdown": return markdownBlock(value);
    case "rich_text": return richTextBlock(value);
    case "section": return section(value);
    case "table": return table(value);
    case "video": return video(value);
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

describe("valid conformance corpus", () => {
  it("declares spec 1.0.0 and all 79 fixtures", () => {
    expect(manifest.spec_version).toBe("1.0.0");
    expect(manifest.fixtures).toHaveLength(79);
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

const invalidCases: Record<string, () => unknown> = {
  "text-empty": () => plainText(""),
  "text-too-long": () => plainText("x".repeat(3001)),
  "section-missing-content": () => section({}),
  "section-text-too-long": () => section({ text: "x".repeat(3001) }),
  "section-too-many-fields": () => section({ fields: Array(11).fill("x") }),
  "section-field-too-long": () => section({ fields: ["x".repeat(2001)] }),
  "header-text-too-long": () => header({ text: "x".repeat(151) }),
  "button-text-too-long": () => button({ text: "x".repeat(76), actionId: "a" }),
  "button-action-id-too-long": () => button({ text: "A", actionId: "x".repeat(256) }),
  "option-value-too-long": () => option({ text: "A", value: "x".repeat(76) }),
  "overflow-too-many-options": () =>
    overflow({ actionId: "a", options: Array.from({ length: 6 }, choice) }),
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
  "file-input-max-files-out-of-range": () => fileInput({ actionId: "a", maxFiles: 11 }),
  "context-invalid-element": () => context({ elements: [divider()] }),
  "input-invalid-element": () =>
    input({ label: "Label", element: button({ text: "A", actionId: "a" }) }),
  "view-missing-blocks": () => homeTab({ blocks: [] }),
};

const invalidManifest = readJson<InvalidManifest>(
  resolve(SPEC_ROOT, "fixtures/invalid/manifest.json"),
);

describe("invalid conformance corpus", () => {
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
