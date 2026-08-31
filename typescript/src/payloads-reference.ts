/**
 * Fluent builders for complete messages, interaction responses, webhooks,
 * secondary attachments, modals, and App Home tabs. Their built objects can be
 * passed directly to the corresponding Slack SDK or HTTP API method.
 *
 * @module payloads
 */
export * from "./fluent/payloads.js";
export type { MessageInput } from "./legacy/messages.js";
