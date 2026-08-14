import { describe, expect, it } from "vitest";

import * as slackblocks from "../src/index.js";
import {
  assertValid,
  attachment,
  barChart,
  axisConfig,
  button,
  cardBlock,
  checkboxes,
  Color,
  containerBlock,
  dataPoint,
  dataSeries,
  dataTableBlock,
  dividerBlock,
  fileInput,
  headerBlock,
  imageBlock,
  inputBlock,
  InvalidUsageError,
  LengthError,
  message,
  MissingRequiredError,
  mrkdwn,
  option,
  OutOfRangeError,
  radioButtons,
  rawText,
  richTextList,
  richTextSection,
  richText,
  sectionBlock,
  staticSelect,
  tableBlock,
  timePicker,
  TypeMismatchError,
  userMultiSelect,
  validate,
  webhookMessage,
} from "../src/index.js";

const choice = (value = "a") => option({ text: "A", value });
const listItem = () => richTextSection([richText("Item")]);

describe("message metadata payloads", () => {
  it("round-trips event_payload contents verbatim while snake-casing metadata keys", () => {
    const payload = message({
      channel: "C0123456",
      metadata: {
        eventType: "x",
        eventPayload: { myCamelKey: "v", type: "card", URLValue: 1 },
      },
    });
    expect(payload.metadata).toEqual({
      event_type: "x",
      event_payload: { myCamelKey: "v", type: "card", URLValue: 1 },
    });
  });

  it("keeps nested event_payload structures untouched", () => {
    const payload = webhookMessage({
      metadata: {
        eventType: "deploy",
        eventPayload: { steps: [{ stepName: "build", type: "divider" }] },
      },
    });
    expect(payload.metadata).toEqual({
      event_type: "deploy",
      event_payload: { steps: [{ stepName: "build", type: "divider" }] },
    });
  });

  it("skips event_payload contents when validating raw JSON", () => {
    expect(
      validate({
        channel: "C0123456",
        metadata: {
          event_type: "x",
          event_payload: { type: "card", value: Number.NaN },
        },
      }),
    ).toBe(true);
  });
});

describe("factory type ownership", () => {
  it("ignores a caller-supplied type field", () => {
    const element = button({ text: "A", actionId: "a", type: "divider" } as never);
    expect(element.type).toBe("button");
  });

  it("rejects unknown input fields at compile time", () => {
    // @ts-expect-error excess properties are not part of ButtonInput
    expect(button({ text: "A", actionId: "a", totallyBogus: 1 }).type).toBe("button");
  });
});

describe("input-compatible elements", () => {
  it("accepts file inputs and time pickers", () => {
    expect(
      inputBlock({ label: "Files", element: fileInput({ actionId: "f" }) }).type,
    ).toBe("input");
    expect(
      inputBlock({ label: "Time", element: timePicker({ actionId: "t" }) }).type,
    ).toBe("input");
  });

  it("still rejects non-input elements", () => {
    expect(() => inputBlock({ label: "Nope", element: dividerBlock() })).toThrowError(
      TypeMismatchError,
    );
  });
});

describe("image blocks", () => {
  it("requires alt text at runtime", () => {
    expect(() =>
      (imageBlock as (input: unknown) => unknown)({
        imageUrl: "https://example.com/image.png",
      }),
    ).toThrowError(MissingRequiredError);
    // @ts-expect-error altText is required
    void (() => imageBlock({ imageUrl: "https://example.com/image.png" }));
  });

  it("accepts an image with alt text", () => {
    expect(
      imageBlock({ imageUrl: "https://example.com/image.png", altText: "An image" }).type,
    ).toBe("image");
  });
});

describe("rich-text lists", () => {
  it("requires a style", () => {
    expect(() =>
      (richTextList as (input: unknown) => unknown)({ elements: [listItem()] }),
    ).toThrowError(MissingRequiredError);
    // @ts-expect-error style is required
    void (() => richTextList({ elements: [listItem()] }));
  });

  it("accepts bullet and ordered styles", () => {
    expect(richTextList({ elements: [listItem()], style: "bullet" }).style).toBe("bullet");
    expect(richTextList({ elements: [listItem()], style: "ordered" }).style).toBe("ordered");
  });
});

describe("table blocks", () => {
  const row = (...cells: string[]) => cells.map((cell) => rawText(cell));

  it("accepts a rectangular table with matching column settings", () => {
    expect(
      tableBlock({
        rows: [row("Name", "Role"), row("Alice", "Admin")],
        columnSettings: [{ is_wrapped: true }, { align: "right" }],
      }).type,
    ).toBe("table");
  });

  it("rejects ragged rows", () => {
    expect(() => tableBlock({ rows: [row("Name", "Role"), row("Alice")] })).toThrowError(
      InvalidUsageError,
    );
  });

  it("rejects more than 100 rows", () => {
    expect(() =>
      tableBlock({ rows: Array.from({ length: 101 }, () => row("A")) }),
    ).toThrowError(LengthError);
  });

  it("rejects rows wider than 20 cells", () => {
    expect(() =>
      tableBlock({ rows: [Array.from({ length: 21 }, () => rawText("A"))] }),
    ).toThrowError(LengthError);
  });

  it("rejects column settings that do not match the column count", () => {
    expect(() =>
      tableBlock({
        rows: [row("Name", "Role")],
        columnSettings: [{ align: "left" }],
      }),
    ).toThrowError(InvalidUsageError);
  });

  it("rejects unsupported cell types", () => {
    expect(() => tableBlock({ rows: [[dividerBlock()]] })).toThrowError(TypeMismatchError);
  });
});

describe("raw JSON validation", () => {
  it("rejects known objects with missing required fields", () => {
    expect(validate({ type: "header" })).toBe(false);
    expect(validate({ type: "button", text: { type: "plain_text", text: "A" } })).toBe(false);
    expect(validate({ type: "image", alt_text: "x" })).toBe(false);
    expect(validate({ type: "image", image_url: "https://example.com/a.png" })).toBe(false);
    expect(validate({ type: "input", label: { type: "plain_text", text: "L" } })).toBe(false);
    expect(validate({ type: "markdown" })).toBe(false);
    expect(validate({ type: "video", alt_text: "A" })).toBe(false);
    expect(validate({ type: "file" })).toBe(false);
    expect(validate({ type: "overflow", action_id: "a" })).toBe(false);
    expect(validate({ type: "modal", blocks: [{ type: "divider" }] })).toBe(false);
  });

  it("reports missing required fields with the missing-required category", () => {
    expect(() => assertValid({ type: "header" })).toThrowError(MissingRequiredError);
    expect(() =>
      assertValid({ type: "button", text: { type: "plain_text", text: "A" } }),
    ).toThrowError(MissingRequiredError);
  });

  it("validates option entries reached through typed parents", () => {
    expect(() =>
      assertValid({
        type: "static_select",
        action_id: "a",
        options: [{ text: { type: "plain_text", text: "A" } }],
      }),
    ).toThrowError(MissingRequiredError);
    expect(() =>
      assertValid({
        type: "static_select",
        action_id: "a",
        options: [{ text: { type: "plain_text", text: "x".repeat(76) }, value: "a" }],
      }),
    ).toThrowError(LengthError);
    expect(
      validate({
        type: "static_select",
        action_id: "a",
        options: [{ text: { type: "plain_text", text: "A" }, value: "a" }],
      }),
    ).toBe(true);
  });

  it("validates option-group entries reached through typed parents", () => {
    expect(() =>
      assertValid({
        type: "multi_static_select",
        action_id: "a",
        option_groups: [{ options: [{ text: { type: "plain_text", text: "A" }, value: "a" }] }],
      }),
    ).toThrowError(MissingRequiredError);
    expect(() =>
      assertValid({
        type: "multi_static_select",
        action_id: "a",
        option_groups: [{ label: { type: "plain_text", text: "Group" }, options: [] }],
      }),
    ).toThrowError(LengthError);
  });

  it("validates confirm objects reached through typed parents", () => {
    expect(() =>
      assertValid({
        type: "button",
        text: { type: "plain_text", text: "A" },
        action_id: "a",
        confirm: { title: { type: "plain_text", text: "Sure?" } },
      }),
    ).toThrowError(MissingRequiredError);
    expect(() =>
      assertValid({
        type: "button",
        text: { type: "plain_text", text: "A" },
        action_id: "a",
        confirm: {
          title: { type: "plain_text", text: "x".repeat(101) },
          text: { type: "mrkdwn", text: "Really?" },
          confirm: { type: "plain_text", text: "Yes" },
          deny: { type: "plain_text", text: "No" },
        },
      }),
    ).toThrowError(LengthError);
    expect(
      validate({
        type: "button",
        text: { type: "plain_text", text: "A" },
        action_id: "a",
        confirm: {
          title: { type: "plain_text", text: "Sure?" },
          text: { type: "mrkdwn", text: "Really?" },
          confirm: { type: "plain_text", text: "Yes" },
          deny: { type: "plain_text", text: "No" },
        },
      }),
    ).toBe(true);
  });
});

describe("Unicode length limits", () => {
  it("counts code points instead of UTF-16 code units", () => {
    expect(headerBlock({ text: "😀".repeat(100) }).type).toBe("header");
    expect(() => headerBlock({ text: "😀".repeat(151) })).toThrowError(LengthError);
    expect(option({ text: "😀".repeat(75), value: "a" }).value).toBe("a");
  });
});

describe("empty optional collection parity", () => {
  it("omits empty section fields and enforces the text-or-fields rule", () => {
    expect(sectionBlock({ text: "Hi", fields: [] })).not.toHaveProperty("fields");
    expect(() => sectionBlock({ fields: [] })).toThrowError(MissingRequiredError);
  });

  it("omits empty message blocks and attachments", () => {
    const payload = message({ channel: "C1", blocks: [], attachments: [] });
    expect(payload).not.toHaveProperty("blocks");
    expect(payload).not.toHaveProperty("attachments");
  });

  it("keeps empty webhook message collections, matching Python", () => {
    expect(webhookMessage({ blocks: [], attachments: [] })).toEqual({
      blocks: [],
      attachments: [],
    });
  });

  it("omits empty attachment blocks", () => {
    expect(attachment({ blocks: [] })).toEqual({});
  });

  it("omits empty select collections", () => {
    expect(staticSelect({ actionId: "a", options: [] })).not.toHaveProperty("options");
    expect(staticSelect({ actionId: "a", optionGroups: [] })).not.toHaveProperty(
      "option_groups",
    );
    expect(
      checkboxes({ actionId: "a", options: [choice()], initialOptions: [] }),
    ).not.toHaveProperty("initial_options");
    expect(userMultiSelect({ actionId: "a", initialUsers: [] })).not.toHaveProperty(
      "initial_users",
    );
  });
});

describe("null handling", () => {
  it("drops null fields from the wire format", () => {
    expect(
      sectionBlock({ text: "Hi", accessory: null as never }),
    ).not.toHaveProperty("accessory");
    expect(message({ channel: "C1", metadata: null as never })).not.toHaveProperty(
      "metadata",
    );
  });
});

describe("non-finite numbers", () => {
  it("rejects non-finite chart values", () => {
    expect(() =>
      assertValid({
        type: "pie",
        segments: [{ label: "A", value: Number.POSITIVE_INFINITY }],
      }),
    ).toThrowError(TypeMismatchError);
    expect(() => dataPoint({ label: "A", value: Number.NaN })).toThrowError(
      TypeMismatchError,
    );
  });

  it("rejects non-finite values in generic numeric fields", () => {
    expect(() =>
      richTextList({ elements: [listItem()], style: "bullet", border: Number.NaN }),
    ).toThrowError(TypeMismatchError);
    expect(validate({ type: "divider", block_id: "b", weight: Number.NaN })).toBe(false);
  });
});

describe("checkbox and radio option counts", () => {
  it("rejects zero options", () => {
    expect(() => checkboxes({ actionId: "a", options: [] })).toThrowError(LengthError);
    expect(() => radioButtons({ actionId: "a", options: [] })).toThrowError(LengthError);
  });

  it("rejects more than ten options", () => {
    const options = Array.from({ length: 11 }, (_, index) => choice(`v${index}`));
    expect(() => checkboxes({ actionId: "a", options })).toThrowError(LengthError);
    expect(() => radioButtons({ actionId: "a", options })).toThrowError(LengthError);
  });

  it("accepts between one and ten options", () => {
    const options = Array.from({ length: 10 }, (_, index) => choice(`v${index}`));
    expect(checkboxes({ actionId: "a", options }).type).toBe("checkboxes");
    expect(radioButtons({ actionId: "a", options }).type).toBe("radio_buttons");
  });
});

describe("option URLs", () => {
  it("enforces the 3000-character URL limit", () => {
    expect(() =>
      option({ text: "A", value: "a", url: `https://example.com/${"x".repeat(3000)}` }),
    ).toThrowError(LengthError);
    expect(option({ text: "A", value: "a", url: "https://example.com" }).url).toBe(
      "https://example.com",
    );
  });
});

describe("OutOfRangeError", () => {
  it("replaces the RangeError export and keeps its category", () => {
    expect("RangeError" in slackblocks).toBe(false);
    try {
      fileInput({ actionId: "a", maxFiles: 11 });
      expect.fail("construction did not throw");
    } catch (error) {
      expect(error).toBeInstanceOf(OutOfRangeError);
      expect((error as OutOfRangeError).category).toBe("out-of-range");
    }
  });
});

describe("attachment colors", () => {
  it("accepts hex colors and normalizes bare hex codes", () => {
    expect(attachment({ blocks: [dividerBlock()], color: "#ff0000" }).color).toBe("#ff0000");
    expect(attachment({ blocks: [dividerBlock()], color: "ff0000" }).color).toBe("#ff0000");
    expect(attachment({ blocks: [dividerBlock()], color: Color.GOOD }).color).toBe("good");
  });

  it("rejects invalid colors", () => {
    expect(() => attachment({ blocks: [dividerBlock()], color: "red" })).toThrowError(
      TypeMismatchError,
    );
    expect(() => attachment({ blocks: [dividerBlock()], color: "#ff00" })).toThrowError(
      TypeMismatchError,
    );
  });

  it("mirrors the Python Color values", () => {
    expect(Color.RED).toBe("#ff0000");
    expect(Color.DANGER).toBe("danger");
    expect(Color.BLACK).toBe("#000000");
  });
});

describe("data table captions", () => {
  const rows = [[rawText("Name")], [rawText("Alice")]];

  it("rejects an empty caption", () => {
    expect(() => dataTableBlock({ rows, caption: "" })).toThrowError(LengthError);
  });

  it("reports a missing caption as missing-required", () => {
    expect(() =>
      assertValid({
        type: "data_table",
        rows: [[{ type: "raw_text", text: "Name" }], [{ type: "raw_text", text: "Alice" }]],
        page_size: 5,
        row_header_column_index: 0,
      }),
    ).toThrowError(MissingRequiredError);
  });
});

describe("card and container text coercion", () => {
  it("matches the Python coercion defaults", () => {
    expect(cardBlock({ title: "T", subtitle: "S", body: "B" })).toMatchObject({
      title: { type: "mrkdwn", text: "T" },
      subtitle: { type: "mrkdwn", text: "S" },
      body: { type: "mrkdwn", text: "B" },
    });
    expect(
      containerBlock({ title: "T", subtitle: "S", childBlocks: [dividerBlock()] }),
    ).toMatchObject({
      title: { type: "plain_text", text: "T" },
      subtitle: { type: "mrkdwn", text: "S" },
    });
  });
});

describe("axis chart validation", () => {
  it("still rejects series that do not match the axis categories", () => {
    const series = dataSeries({ name: "S", data: [dataPoint({ label: "B", value: 1 })] });
    expect(() => barChart([series], axisConfig({ categories: ["A"] }))).toThrowError(
      InvalidUsageError,
    );
  });

  it("still rejects duplicate series names", () => {
    const series = () =>
      dataSeries({ name: "S", data: [dataPoint({ label: "A", value: 1 })] });
    expect(() =>
      barChart([series(), series()], axisConfig({ categories: ["A"] })),
    ).toThrowError(InvalidUsageError);
  });
});

describe("text object coercion helpers", () => {
  it("keeps explicit text objects unchanged", () => {
    const block = sectionBlock({ text: mrkdwn("*Hi*") });
    expect(block.text).toEqual({ type: "mrkdwn", text: "*Hi*" });
  });
});
