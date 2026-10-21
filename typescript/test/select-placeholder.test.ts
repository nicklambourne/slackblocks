import { expect, it } from "vitest";

import {
  ExternalSelect,
  ExternalMultiSelect,
  UserSelect,
  UserMultiSelect,
  ChannelSelect,
  ChannelMultiSelect,
  ConversationSelect,
  ConversationMultiSelect,
  StaticSelect,
  StaticMultiSelect,
  Option,
  LengthError,
} from "../src/index.js";

const factories = {
  ExternalSelect: (s: string) => ExternalSelect().placeholder(s).build(),
  ExternalMultiSelect: (s: string) => ExternalMultiSelect().placeholder(s).build(),
  UserSelect: (s: string) => UserSelect().placeholder(s).build(),
  UserMultiSelect: (s: string) => UserMultiSelect().placeholder(s).build(),
  ChannelSelect: (s: string) => ChannelSelect().placeholder(s).build(),
  ChannelMultiSelect: (s: string) => ChannelMultiSelect().placeholder(s).build(),
  ConversationSelect: (s: string) => ConversationSelect().placeholder(s).build(),
  ConversationMultiSelect: (s: string) => ConversationMultiSelect().placeholder(s).build(),
  StaticSelect: (s: string) => StaticSelect().placeholder(s).options(Option().text("A").value("a")).build(),
  StaticMultiSelect: (s: string) => StaticMultiSelect().placeholder(s).options(Option().text("A").value("a")).build(),
};

for (const [name, makeValue] of Object.entries(factories)) {
  it(`${name} checks Unicode placeholder boundaries`, () => {
    for (const unit of ["x", "🚀"]) {
      for (const n of [149, 150, 151]) {
        const value = unit.repeat(n);
        if (n === 151) {
          try {
            makeValue(value);
            throw new Error("accepted too-long placeholder");
          } catch (error) {
            expect(error).toBeInstanceOf(LengthError);
            expect((error as LengthError).path).toBe("placeholder.text");
          }
        } else {
          expect(makeValue(value).placeholder).toMatchObject({ text: value });
        }
      }
    }
  });
}
