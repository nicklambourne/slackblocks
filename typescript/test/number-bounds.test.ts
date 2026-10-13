import { expect, it } from "vitest";
import { NumberInput, TypeMismatchError } from "../src/index.js";
import { numberInput } from "../src/legacy/elements.js";

it("sends string bounds from both native APIs, preserving numeric convenience", () => {
  for (const value of [0, -2, 1.25, "-1.5", "10.25"]) {
    const input = NumberInput().isDecimalAllowed(true).minValue(value).maxValue(value).build();
    expect(input.min_value).toBe(String(value));
    expect(input.max_value).toBe(String(value));
    expect(numberInput({isDecimalAllowed: true, minValue:value}).min_value).toBe(String(value));
  }
  expect(NumberInput().isDecimalAllowed(true).build()).not.toHaveProperty("min_value");
});
it("rejects nonfinite, invalid, and numerically inverted string bounds", () => {
  for (const value of [Infinity, NaN, "invalid", "", "1\n", "1\r\n"]) {
    expect(() => NumberInput().isDecimalAllowed(true).minValue(value).build()).toThrow();
  }
  expect(() => NumberInput().isDecimalAllowed(true).minValue("10").maxValue("2").build()).toThrow(/min_value/);
});

it("compares decimal bounds exactly beyond binary float precision", () => {
  for (const [min,max] of [["9007199254740993","9007199254740992"],["0.10000000000000001","0.1"],["-9007199254740992","-9007199254740993"],["-0.1","-0.10000000000000001"],["1e-1000","9e-1001"]] as const) {
    expect(() => NumberInput().isDecimalAllowed(true).minValue(min).maxValue(max).build()).toThrow(/min_value/);
    expect(() => NumberInput().isDecimalAllowed(true).minValue(max).maxValue(min).build()).not.toThrow();
  }
  expect(() => NumberInput().isDecimalAllowed(true).minValue("0.1000").maxValue("1e-1").build()).not.toThrow();
});

it("rejects non-string/non-number bounds before invoking user coercion", () => {
  let coercions = 0;
  const object = { toString: () => { coercions++; return "10"; } };
  for (const value of [object, ["10"], 10n]) {
    for (const field of ["minValue", "maxValue"] as const) {
      // @ts-expect-error Exercise the untyped JavaScript consumer boundary.
      expect(() => NumberInput().isDecimalAllowed(true)[field](value).build()).toThrow(TypeMismatchError);
      // @ts-expect-error Exercise the legacy factory with the same invalid input.
      expect(() => numberInput({ isDecimalAllowed: true, minValue: value, maxValue: value })).toThrow(TypeMismatchError);
    }
  }
  expect(coercions).toBe(0);
});
