/** Fluent builders for messages, webhooks, interaction responses, and views. */
import {
  attachment as createAttachment,
  message as createMessage,
  messageResponse as createMessageResponse,
  webhookMessage as createWebhookMessage,
  type MessageInput,
} from "../messages.js";
import type { JsonObject } from "../types.js";
import { homeTab as createHomeTab, modal as createModal } from "../views.js";
import { createFluentBuilder, type FluentBuilder } from "./core.js";

type FirstInput<Factory extends (...args: never[]) => unknown> = Parameters<Factory>[0];
type Output<Factory extends (...args: never[]) => unknown> = ReturnType<Factory>;

/** Starts fluent lower-priority content attached to a message. */
export function Attachment(): FluentBuilder<
  FirstInput<typeof createAttachment>,
  JsonObject
> {
  return createFluentBuilder(createAttachment, { collections: { blocks: "flat" } });
}

/** Starts a fluent payload for Slack Web API message methods. */
export function Message(): FluentBuilder<MessageInput, JsonObject> {
  return createFluentBuilder(createMessage, {
    collections: { blocks: "flat", attachments: "flat" },
  });
}

/** Starts a fluent response to a slash command or interactive request. */
export function MessageResponse(): FluentBuilder<
  FirstInput<typeof createMessageResponse>,
  JsonObject
> {
  return createFluentBuilder(createMessageResponse, {
    collections: { blocks: "flat", attachments: "flat" },
  });
}

/** Starts a fluent incoming-webhook or response-URL payload. */
export function WebhookMessage(): FluentBuilder<
  FirstInput<typeof createWebhookMessage>,
  JsonObject
> {
  return createFluentBuilder(createWebhookMessage, {
    collections: { blocks: "flat", attachments: "flat" },
  });
}

/** Starts a fluent modal view payload. */
export function Modal(): FluentBuilder<
  FirstInput<typeof createModal>,
  Output<typeof createModal>
> {
  return createFluentBuilder(createModal, { collections: { blocks: "flat" } });
}

/** Starts a fluent App Home tab payload. */
export function HomeTab(): FluentBuilder<
  FirstInput<typeof createHomeTab>,
  Output<typeof createHomeTab>
> {
  return createFluentBuilder(createHomeTab, { collections: { blocks: "flat" } });
}
