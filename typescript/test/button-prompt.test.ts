import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { expect, it } from "vitest";
import { Button, MissingRequiredError, TypeMismatchError } from "../src/index.js";
import { button } from "../src/legacy/elements.js";

it("preserves native prompt fields in both APIs without inventing a display default", () => {
  const input = { text: "Ask", agentPrompt: "Ask Slackbot", agentPromptDisplay: "x".repeat(80) };
  const builder = Button().text(input.text).agentPrompt(input.agentPrompt).agentPromptDisplay(input.agentPromptDisplay);
  expect(builder.build()).toEqual(button(input));
  expect(Button().text("Ask").agentPrompt("").agentPromptDisplay("").build()).toMatchObject({ agent_prompt: "", agent_prompt_display: "" });
  expect(Button().text("Ask").agentPrompt("Ask Slackbot").build()).not.toHaveProperty("agent_prompt_display");
  const snapshot = builder.build();
  builder.agentPrompt("Changed");
  expect(snapshot.agent_prompt).toBe("Ask Slackbot");
});

it("validates untyped consumers without coercing prompt fields", () => {
  for (const field of ["agentPrompt", "agentPromptDisplay"] as const) {
    for (const value of [false, 1, [], {}]) {
      // @ts-expect-error Exercise the untyped JavaScript boundary.
      expect(() => Button().text("Ask")[field](value).build()).toThrow(TypeMismatchError);
    }
  }
  expect(() => button({ text: "Ask", agentPromptDisplay: "" })).toThrow(MissingRequiredError);
});

it("executes the documented prompt recipe", () => {
  const source = readFileSync(new URL("../../docs/docs/usage/agent-prompts.mdx", import.meta.url), "utf8");
  const snippet = source.match(/```typescript\n([\s\S]*?)```/)?.[1];
  assert.ok(snippet, "missing TypeScript prompt recipe");
  const code = snippet.replace(/^import .*;$/m, "");
  const output: string[] = [];
  new Function("Button", "console", code)(Button, { log: (value: string) => output.push(value) });
  expect(output).toHaveLength(1);
  const rendered = output[0];
  assert.ok(rendered, "prompt recipe produced no JSON");
  expect(JSON.parse(rendered)).toMatchObject({ agent_prompt: "Summarize this incident.", agent_prompt_display: "Incident summary", action_id: "summarize_incident" });
});
