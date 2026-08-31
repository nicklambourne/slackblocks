/**
 * Fluent builders for messages, webhooks, interaction responses, and views.
 *
 * @module payloads
 */
import {
  attachment as createAttachment,
  message as createMessage,
  messageResponse as createMessageResponse,
  webhookMessage as createWebhookMessage,
  type MessageInput,
} from "../legacy/messages.js";
import type { JsonObject } from "../types.js";
import { homeTab as createHomeTab, modal as createModal } from "../legacy/views.js";
import { createFluentBuilder, type FluentBuilder } from "./core.js";

type FirstInput<Factory extends (...args: never[]) => unknown> = Parameters<Factory>[0];
type Output<Factory extends (...args: never[]) => unknown> = ReturnType<Factory>;

/**
 * Creates lower-priority supporting content using Slack's legacy secondary
 * attachment format. Attachments add context beneath a message, while `fallback()`
 * supplies text for notifications and clients that cannot display Block Kit.
 *
 * See: <https://docs.slack.dev/legacy/legacy-messaging/legacy-secondary-message-attachments>.
 */
export function Attachment(): FluentBuilder<
  FirstInput<typeof createAttachment>,
  JsonObject
> {
  return createFluentBuilder(createAttachment, { collections: { blocks: "flat" } });
}

/**
 * Creates a message payload for Slack Web API methods such as
 * `chat.postMessage`. Set the destination channel and add Block Kit blocks,
 * secondary attachments, fallback text, metadata, or unfurl behavior as needed.
 *
 * See: <https://docs.slack.dev/reference/methods/chat.postMessage>.
 */
export function Message(): FluentBuilder<MessageInput, JsonObject> {
  return createFluentBuilder(createMessage, {
    collections: { blocks: "flat", attachments: "flat" },
  });
}

/**
 * Creates the immediate response payload returned for a slash command or
 * interactive request. Configure its blocks, fallback text, visibility, and
 * whether it replaces the original interaction message.
 *
 * See: <https://docs.slack.dev/interactivity/implementing-slash-commands#responding_to_commands>.
 */
export function MessageResponse(): FluentBuilder<
  FirstInput<typeof createMessageResponse>,
  JsonObject
> {
  return createFluentBuilder(createMessageResponse, {
    collections: { blocks: "flat", attachments: "flat" },
  });
}

/**
 * Creates a payload for an incoming webhook or an interaction response URL.
 * Unlike a Web API message, this form can replace or delete the original message
 * and does not require a destination channel field.
 *
 * See: <https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks>.
 */
export function WebhookMessage(): FluentBuilder<
  FirstInput<typeof createWebhookMessage>,
  JsonObject
> {
  return createFluentBuilder(createWebhookMessage, {
    collections: { blocks: "flat", attachments: "flat" },
  });
}

/**
 * Creates a modal view for Slack's `views.open`, `views.update`, and `views.push`
 * methods. Configure the title, compatible blocks, controls, metadata, and close
 * behavior before building the payload.
 *
 * See: <https://docs.slack.dev/surfaces/modals>.
 */
export function Modal(): FluentBuilder<
  FirstInput<typeof createModal>,
  Output<typeof createModal>
> {
  return createFluentBuilder(createModal, { collections: { blocks: "flat" } });
}

/**
 * Creates an App Home tab view for Slack's `views.publish` method. Add up to 100
 * compatible blocks and optional identifiers or private metadata for the
 * application.
 *
 * See: <https://docs.slack.dev/surfaces/app-home>.
 */
export function HomeTab(): FluentBuilder<
  FirstInput<typeof createHomeTab>,
  Output<typeof createHomeTab>
> {
  return createFluentBuilder(createHomeTab, { collections: { blocks: "flat" } });
}
