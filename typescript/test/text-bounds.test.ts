import { expect, it } from "vitest";
import { ExternalSelect, ExternalMultiSelect, DatePicker, TimePicker, ImageElement, PlainTextInput, OutOfRangeError, TypeMismatchError } from "../src/index.js";
it("accepts satisfiable, omitted and equal text bounds", () => {
  for (const [min, max] of [[undefined, undefined], [20, undefined], [undefined, 10], [0, 1], [10, 10], [3000, 3000]]) {
    const builder = PlainTextInput();
    if (min !== undefined) builder.minLength(min);
    if (max !== undefined) builder.maxLength(max);
    const value = builder.build();
    expect(value.min_length).toBe(min);
    expect(value.max_length).toBe(max);
  }
  try { PlainTextInput().minLength(20).maxLength(10).build(); throw new Error("accepted inverted bounds"); }
  catch (error) { expect(error).toBeInstanceOf(OutOfRangeError); expect((error as OutOfRangeError).path).toBe("min_length"); }
});

it("preserves zero query, date shape, valid times and empty image alt text", () => {
  expect(ExternalSelect().minQueryLength(0).build().min_query_length).toBe(0);
  expect(ExternalMultiSelect().minQueryLength(0).build().min_query_length).toBe(0);
  for (const date of ["2028-02-29", "2028-02-30"]) expect(DatePicker().initialDate(date).build().initial_date).toBe(date);
  for (const time of ["00:00", "23:59"]) expect(TimePicker().initialTime(time).build().initial_time).toBe(time);
  expect(ImageElement().imageUrl("https://example.com/image.png").altText("").build().alt_text).toBe("");
});

it("rejects trailing newline and non-ASCII date/time strings", () => {
  for (const date of ["2028-02-29\n", "２０２８-02-29"]) {
    expect(() => DatePicker().initialDate(date).build()).toThrowError(TypeMismatchError);
  }
  for (const time of ["23:59\n", "２３:59"]) {
    expect(() => TimePicker().initialTime(time).build()).toThrowError(TypeMismatchError);
  }
});
