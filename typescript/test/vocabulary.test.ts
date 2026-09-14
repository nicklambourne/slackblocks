import { describe, expect, it } from "vitest";

import vocabulary from "../../spec/vocabulary.json" with { type: "json" };

import { slackIcon, type SlackIconName } from "../src/legacy/objects.js";
import { validateSurfaceBlocks, type BlockSurface } from "../src/surfaces.js";

describe("shared vocabulary", () => {
  it("accepts exactly the Slack icon names in spec/vocabulary.json", () => {
    for (const name of vocabulary.slack_icon_names) {
      expect(slackIcon(name as SlackIconName).name).toBe(name);
    }
    expect(() => slackIcon("not-a-slack-icon" as SlackIconName)).toThrow();
  });

  it("allows exactly the block types in spec/vocabulary.json on each surface", () => {
    const surfaces = vocabulary.surface_block_types as Record<BlockSurface, string[]>;
    const everyType = new Set([...Object.values(surfaces).flat(), "not_a_block"]);
    for (const [surface, allowed] of Object.entries(surfaces) as [BlockSurface, string[]][]) {
      for (const type of everyType) {
        const check = () => validateSurfaceBlocks([{ type }], surface, "blocks");
        if (allowed.includes(type)) {
          expect(check, `${type} on ${surface}`).not.toThrow();
        } else {
          expect(check, `${type} on ${surface}`).toThrow();
        }
      }
    }
  });
});
