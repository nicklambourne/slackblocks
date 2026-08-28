/**
 * View factories for Slack modals and App Home tabs.
 *
 * @module views
 */
import { MissingRequiredError } from "./errors.js";
import { create } from "./internal.js";
import { asText, type TextLike } from "./objects.js";
import { validateSurfaceBlocks } from "./surfaces.js";
import type { FactorySettings, JsonObject } from "./types.js";

/**
 * Creates a modal view payload.
 *
 * @param input - Modal title, blocks, controls, metadata, and callback behavior.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `modal` view.
 * @throws InvalidUsageError when blocks or text fields violate Slack's constraints.
 */
export function modal(
  input: {
    /** Plain-text modal title, up to 24 characters. */
    title: TextLike;
    /** Between one and 100 modal-compatible blocks. */
    blocks: JsonObject[];
    /** Optional plain-text close-button label. */
    close?: TextLike;
    /** Optional plain-text submit-button label. */
    submit?: TextLike;
    /** Opaque application metadata returned with view interactions. */
    privateMetadata?: string;
    /** Application-defined callback identifier. */
    callbackId?: string;
    /** Close every view above this modal when it closes. */
    clearOnClose?: boolean;
    /** Send a `view_closed` event when the modal closes. */
    notifyOnClose?: boolean;
    /** Application-defined external identifier. */
    externalId?: string;
    /** Keep the submit button disabled until an input changes. */
    submitDisabled?: boolean;
  },
  settings: FactorySettings = {},
) {
  validateSurfaceBlocks(input.blocks, "modal", "modal.blocks");
  if (
    input.submit === undefined &&
    input.blocks.some((block) => block.type === "input")
  ) {
    throw new MissingRequiredError(
      "modal.submit",
      "required when the modal contains an input block",
    );
  }
  return create(
    "modal",
    {
      ...input,
      title: asText(input.title, "plain_text", settings),
      close: input.close === undefined ? undefined : asText(input.close, "plain_text", settings),
      submit:
        input.submit === undefined ? undefined : asText(input.submit, "plain_text", settings),
    },
    settings,
  );
}

/**
 * Creates an App Home tab view payload.
 *
 * @param input - Home-tab blocks and optional application metadata.
 * @param settings - Per-call validation settings.
 * @returns A validated Slack `home` view.
 * @throws InvalidUsageError when blocks or metadata violate Slack's constraints.
 */
export function homeTab(
  input: {
    /** Between one and 100 App Home-compatible blocks. */
    blocks: JsonObject[];
    /** Opaque application metadata returned with view interactions. */
    privateMetadata?: string;
    /** Application-defined callback identifier. */
    callbackId?: string;
    /** Application-defined external identifier. */
    externalId?: string;
  },
  settings: FactorySettings = {},
) {
  validateSurfaceBlocks(input.blocks, "home", "homeTab.blocks");
  return create("home", input, settings);
}
