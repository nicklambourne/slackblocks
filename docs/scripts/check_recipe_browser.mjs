import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { chromium } from "playwright";
import { outputs, render } from "./check_recipe_rendering.mjs";

// Use the actual component and stylesheet, including its responsive rules.
// No Slack credentials, external images, network server or generated fixture files.
const css = await readFile(new URL("../src/components/RecipeOutput/styles.module.css", import.meta.url), "utf8");
const plain = (text) => ({ type: "plain_text", text });
const button = (label, style, id = style ?? "details") => ({ type: "button", action_id: id, text: plain(label), ...(style ? { style } : {}) });
const card = (id, actions) => ({ type: "card", block_id: id, title: plain(id), actions });
const long = "Label " + "x".repeat(69);
const fixtures = {
  blocks: [
    card("long-two", [button(long, "primary"), button(long, "danger")]),
    card("long-three", [button(long, "primary"), button(long, "danger"), button(long)]),
    card("long-single", [button(long, "primary")]),
    card("long-default", [button(long, undefined, "first"), button(long, undefined, "second"), button(long, undefined, "third")]),
    { type: "card", block_id: "private-images", title: plain("Private images"),
      hero_image: { type: "image", slack_file: { id: "F01234567" }, alt_text: "Private hero" },
      icon: { type: "image", slack_file: { url: "https://files.slack.com/files-pri/T123-F123/icon.png" }, alt_text: "Private icon" } },
  ],
};
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  const requests = [];
  await page.route("**/*", (route) => { requests.push(route.request().url()); return route.abort(); });
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  for (const width of [1280, 320]) {
    await page.setViewportSize({ width, height: 900 });
    for (const [language, payload] of [...Object.entries(outputs), ["edge-cases", fixtures]]) {
      await page.setContent(`<!doctype html><html><head><meta charset="utf-8"><style>
        :root { font-family: Arial, sans-serif; font-size: 16px; line-height: 1.5; }
        * { box-sizing: border-box; } body { margin: 16px; }
        ${css}
      </style></head><body>${render(payload)}</body></html>`);
      const cards = await page.locator("[data-card-id]").evaluateAll((elements) => elements.map((card) => {
        const actions = card.querySelector(".cardActions");
        const rect = (element) => {
          const { x, y, right, bottom, width, height } = element.getBoundingClientRect();
          return { x, y, right, bottom, width, height };
        };
        return {
          id: card.dataset.cardId, card: rect(card), actions: actions && rect(actions),
          buttons: [...card.querySelectorAll("button")].map((button) => ({
            ...rect(button), style: button.dataset.style, label: button.textContent,
            accessibleLabel: button.getAttribute("aria-label"),
            clipped: button.scrollWidth > button.clientWidth,
            overflow: getComputedStyle(button).textOverflow,
          })),
        };
      }));
      assert.equal(cards.length, payload.blocks.length, `${language}/${width}: missing card previews`);
      for (const { id, card, actions, buttons } of cards) {
        const context = `${language}/${width}/${id}`;
        const expected = payload.blocks.find((block) => block.block_id === id);
        assert.ok(expected, `${context}: unexpected card preview`);
        assert.equal(buttons.length, expected.actions?.length ?? 0, `${context}: missing actions`);
        assert.ok(card.x >= 0 && card.right <= width, `${context}: card overflow`);
        if (!buttons.length) continue;
        assert.ok(Math.abs(buttons.at(-1).right - actions.right) < 1, `${context}: actions must end at the right edge`);
        for (let i = 0; i < buttons.length; i++) {
          const b = buttons[i];
          assert.ok(b.x >= actions.x - 1 && b.right <= actions.right + 1, `${context}: button overflow`);
          assert.ok(b.height < 45, `${context}: button label must stay on one line`);
          if (i) {
            assert.ok(buttons[i - 1].right + 7 <= b.x, `${context}: overlapping buttons`);
            assert.ok(Math.abs(buttons[i - 1].y - b.y) < 1, `${context}: buttons left their row`);
          }
          if (b.style === "primary") assert.equal(i, buttons.length - 1, `${context}: primary must be last`);
        }
        if (id === "card_two") assert.ok(buttons[0].x > actions.x + 8, `${context}: two actions must stay grouped right`);
        if (buttons.length === 3 && buttons[0].style === "danger") {
          assert.ok(Math.abs(buttons[0].x - actions.x) < 1, `${context}: danger must sit at the far left`);
        }
        if (id.startsWith("long-")) {
          for (const b of buttons) {
            assert.equal(b.label, long, `${context}: full text must survive truncation`);
            assert.equal(b.accessibleLabel, long, `${context}: full accessible name must survive truncation`);
            assert.equal(b.overflow, "ellipsis");
          }
          assert.ok(buttons.some((b) => b.clipped), `${context}: long labels should truncate`);
        }
      }
      if (language === "edge-cases") {
        assert.equal(await page.locator("img").count(), 0);
        for (const name of ["Private hero", "Private icon"]) assert.equal(await page.getByRole("img", { name, exact: true }).count(), 1);
      }
      assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), `${language}/${width}: page overflow`);
    }
  }
  assert.deepEqual(requests, [], "Preview fixtures must not fetch private images or other external resources");
  assert.deepEqual(errors, [], "Preview emitted browser errors");
  console.log("Chromium recipe previews passed: ten languages, desktop/320px geometry, long labels, accessible Slack-file placeholders and no external requests.");
} finally {
  await browser.close();
}
