import {expect, it} from "vitest";
import {ColumnSettings, RawText, TableBlock, TypeMismatchError, tableBlock} from "../src/index.js";
it("retains skipped positions through the fluent API and array overload", () => {
  const settings = ColumnSettings().align("right");
  for (const table of [TableBlock().rows([RawText().text("A")]).columnSettings(null, settings, null), TableBlock().rows([RawText().text("A")]).columnSettings([null, settings, null])]) {
    expect(table.build().column_settings).toEqual([null, {align: "right"}, null]);
  }
});
it.each([false, 1, "right", {align: ["right"]}, {align: "invalid"}, {is_wrapped: 1}])("rejects invalid non-null settings: %j", setting => {
  expect(() => tableBlock({rows: [[{type: "raw_text", text: "A"}]], columnSettings: [null, setting as never]})).toThrow(TypeMismatchError);
});
