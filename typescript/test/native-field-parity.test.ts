import { describe, expect, it } from "vitest";
import { Confirmation, InvalidUsageError, RichTextEmoji } from "../src/index.js";
import { confirmation } from "../src/legacy/objects.js";
import { richTextEmoji } from "../src/legacy/rich-text.js";

describe("native optional-field parity", () => {
  it("preserves confirmation style through fluent and object APIs", () => {
    const input = { title: "Proceed?", text: "Confirm", confirm: "Yes", deny: "No" };
    const builder = Confirmation().title(input.title).text(input.text).confirm(input.confirm).deny(input.deny);
    expect(builder.build()).not.toHaveProperty("style");
    expect(builder.style("danger").build()).toEqual(confirmation({ ...input, style: "danger" }));
    expect(builder.build().style).toBe("danger");
    // @ts-expect-error Only shared named styles are accepted by typed consumers.
    builder.style("invalid");
    expect(() => builder.build()).toThrow("style");
  });

  it("preserves emoji tone and the legacy settings argument", () => {
    expect(RichTextEmoji().name("wave").build()).toEqual({ type: "emoji", name: "wave" });
    expect(RichTextEmoji().name("wave").skinTone(6).build()).toEqual({ type: "emoji", name: "wave", skin_tone: 6 });
    expect(richTextEmoji("wave", { validate: true }, 6)).toEqual(RichTextEmoji().name("wave").skinTone(6).build());
  });

  it("does not silently discard misspelled fields on these builders", () => {
    expect(() => {
      // @ts-expect-error Deliberate untyped JavaScript misspelling at runtime.
      Confirmation().stlye("danger");
    }).toThrow(InvalidUsageError);
    expect(() => {
      // @ts-expect-error Deliberate untyped JavaScript misspelling at runtime.
      RichTextEmoji().skin_tone(6);
    }).toThrow(InvalidUsageError);
  });
});
