import { create } from "./internal.js";
import { asText, type TextLike } from "./objects.js";
import type { FactorySettings, JsonObject } from "./types.js";

export function modal(
  input: {
    title: TextLike;
    blocks: JsonObject[];
    close?: TextLike;
    submit?: TextLike;
    privateMetadata?: string;
    callbackId?: string;
    clearOnClose?: boolean;
    notifyOnClose?: boolean;
    externalId?: string;
    submitDisabled?: boolean;
  },
  settings: FactorySettings = {},
) {
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

export function homeTab(
  input: {
    blocks: JsonObject[];
    privateMetadata?: string;
    callbackId?: string;
    externalId?: string;
  },
  settings: FactorySettings = {},
) {
  return create("home", input, settings);
}
