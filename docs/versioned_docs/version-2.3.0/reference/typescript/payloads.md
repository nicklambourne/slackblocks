---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Payloads

Fluent builders for complete messages, interaction responses, webhooks,
secondary attachments, modals, and App Home tabs. Their built objects can be
passed directly to the corresponding Slack SDK or HTTP API method.

## Attachment()

```ts
function Attachment(): FluentBuilder<{
  blocks: JsonObject[];
  color?: string;
  fallback?: string;
}, JsonObject>;
```

Creates lower-priority supporting content using Slack's legacy secondary
attachment format. Attachments add context beneath a message, while `fallback()`
supplies text for notifications and clients that cannot display Block Kit.

See: <https://docs.slack.dev/legacy/legacy-messaging/legacy-secondary-message-attachments>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Blocks displayed inside the attachment. |
| `.color(value)` | <code>string</code> | No | Optional side-border color: a `Color` value or a six-digit hex code. |
| `.fallback(value)` | <code>string</code> | No | Plain-text fallback for notifications and clients without Block Kit support. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blocks`: [`JsonObject`](utilities.md#jsonobject)[];
  `color?`: `string`;
  `fallback?`: `string`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## HomeTab()

```ts
function HomeTab(): FluentBuilder<{
  blocks: JsonObject[];
  callbackId?: string;
  externalId?: string;
  privateMetadata?: string;
}, SlackObject<"home">>;
```

Creates an App Home tab view for Slack's `views.publish` method. Add up to 100
compatible blocks and optional identifiers or private metadata for the
application.

See: <https://docs.slack.dev/surfaces/app-home>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Between one and 100 App Home-compatible blocks. |
| `.privateMetadata(value)` | <code>string</code> | No | Opaque application metadata returned with view interactions. |
| `.callbackId(value)` | <code>string</code> | No | Application-defined callback identifier. |
| `.externalId(value)` | <code>string</code> | No | Application-defined external identifier. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blocks`: [`JsonObject`](utilities.md#jsonobject)[];
  `callbackId?`: `string`;
  `externalId?`: `string`;
  `privateMetadata?`: `string`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"home"`&gt;&gt;

***

## Message()

```ts
function Message(): FluentBuilder<MessageInput, JsonObject>;
```

Creates a message payload for Slack Web API methods such as
`chat.postMessage`. Set the destination channel and add Block Kit blocks,
secondary attachments, fallback text, metadata, or unfurl behavior as needed.

See: <https://docs.slack.dev/reference/methods/chat.postMessage>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.channel(value)` | <code>string</code> | Yes | Channel, group, or direct-message conversation identifier. |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Block Kit blocks displayed in the message. |
| `.attachments(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Optional secondary attachments. |
| `.text(value)` | <code>string</code> | No | Notification and accessibility fallback text. |
| `.mrkdwn(value)` | <code>boolean</code> | No | Whether Slack parses `text` as mrkdwn. Defaults to `true`. |
| `.unfurlLinks(value)` | <code>boolean</code> | No | Whether Slack unfurls links. |
| `.unfurlMedia(value)` | <code>boolean</code> | No | Whether Slack unfurls media. |
| `.metadata(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional message metadata. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`MessageInput`](#messageinput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## MessageInput

Destination, Block Kit content, fallback text, attachments, metadata, and unfurl
behavior for a Slack Web API message payload.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-attachments"></a> `attachments?` | [`JsonObject`](utilities.md#jsonobject)[] | Optional secondary attachments. |
| <a id="property-blocks"></a> `blocks?` | [`JsonObject`](utilities.md#jsonobject)[] | Block Kit blocks displayed in the message. |
| <a id="property-channel"></a> `channel` | `string` | Channel, group, or direct-message conversation identifier. |
| <a id="property-metadata"></a> `metadata?` | [`JsonObject`](utilities.md#jsonobject) | Optional message metadata. |
| <a id="property-mrkdwn"></a> `mrkdwn?` | `boolean` | Whether Slack parses `text` as mrkdwn. Defaults to `true`. |
| <a id="property-text"></a> `text?` | `string` | Notification and accessibility fallback text. |
| <a id="property-unfurllinks"></a> `unfurlLinks?` | `boolean` | Whether Slack unfurls links. |
| <a id="property-unfurlmedia"></a> `unfurlMedia?` | `boolean` | Whether Slack unfurls media. |

***

## MessageResponse()

```ts
function MessageResponse(): FluentBuilder<{
  attachments?: JsonObject[];
  blocks?: JsonObject[];
  mrkdwn?: boolean;
  replaceOriginal?: boolean;
  responseType?: "ephemeral" | "in_channel";
  text?: string;
}, JsonObject>;
```

Creates the immediate response payload returned for a slash command or
interactive request. Configure its blocks, fallback text, visibility, and
whether it replaces the original interaction message.

See: <https://docs.slack.dev/interactivity/implementing-slash-commands#responding_to_commands>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Block Kit blocks displayed in the response. |
| `.attachments(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Optional secondary attachments. |
| `.text(value)` | <code>string</code> | No | Notification and accessibility fallback text. |
| `.mrkdwn(value)` | <code>boolean</code> | No | Whether Slack parses `text` as mrkdwn. Defaults to `true`. |
| `.replaceOriginal(value)` | <code>boolean</code> | No | Replace the original interaction message. Defaults to `false`. |
| `.responseType(value)` | <code>"ephemeral" &#124; "in_channel"</code> | No | Response visibility. Defaults to `in_channel`. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `attachments?`: [`JsonObject`](utilities.md#jsonobject)[];
  `blocks?`: [`JsonObject`](utilities.md#jsonobject)[];
  `mrkdwn?`: `boolean`;
  `replaceOriginal?`: `boolean`;
  `responseType?`: `"ephemeral"` \| `"in_channel"`;
  `text?`: `string`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## Modal()

```ts
function Modal(): FluentBuilder<{
  blocks: JsonObject[];
  callbackId?: string;
  clearOnClose?: boolean;
  close?: TextLike;
  externalId?: string;
  notifyOnClose?: boolean;
  privateMetadata?: string;
  submit?: TextLike;
  submitDisabled?: boolean;
  title: TextLike;
}, SlackObject<"modal">>;
```

Creates a modal view for Slack's `views.open`, `views.update`, and `views.push`
methods. Configure the title, compatible blocks, controls, metadata, and close
behavior before building the payload.

See: <https://docs.slack.dev/surfaces/modals>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.title(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text modal title, up to 24 characters. |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Between one and 100 modal-compatible blocks. |
| `.close(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional plain-text close-button label. |
| `.submit(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional plain-text submit-button label. |
| `.privateMetadata(value)` | <code>string</code> | No | Opaque application metadata returned with view interactions. |
| `.callbackId(value)` | <code>string</code> | No | Application-defined callback identifier. |
| `.clearOnClose(value)` | <code>boolean</code> | No | Close every view above this modal when it closes. |
| `.notifyOnClose(value)` | <code>boolean</code> | No | Send a `view_closed` event when the modal closes. |
| `.externalId(value)` | <code>string</code> | No | Application-defined external identifier. |
| `.submitDisabled(value)` | <code>boolean</code> | No | Keep the submit button disabled until an input changes. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blocks`: [`JsonObject`](utilities.md#jsonobject)[];
  `callbackId?`: `string`;
  `clearOnClose?`: `boolean`;
  `close?`: [`TextLike`](objects.md#textlike);
  `externalId?`: `string`;
  `notifyOnClose?`: `boolean`;
  `privateMetadata?`: `string`;
  `submit?`: [`TextLike`](objects.md#textlike);
  `submitDisabled?`: `boolean`;
  `title`: [`TextLike`](objects.md#textlike);
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"modal"`&gt;&gt;

***

## WebhookMessage()

```ts
function WebhookMessage(): FluentBuilder<{
  attachments?: JsonObject[];
  blocks?: JsonObject[];
  deleteOriginal?: boolean;
  metadata?: JsonObject;
  replaceOriginal?: boolean;
  responseType?: "ephemeral" | "in_channel";
  text?: string;
  unfurlLinks?: boolean;
  unfurlMedia?: boolean;
}, JsonObject>;
```

Creates a payload for an incoming webhook or an interaction response URL.
Unlike a Web API message, this form can replace or delete the original message
and does not require a destination channel field.

See: <https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Block Kit blocks displayed in the message. |
| `.attachments(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Optional secondary attachments. |
| `.text(value)` | <code>string</code> | No | Notification and accessibility fallback text. |
| `.responseType(value)` | <code>"ephemeral" &#124; "in_channel"</code> | No | Response visibility for response URLs. |
| `.replaceOriginal(value)` | <code>boolean</code> | No | Replace the original interaction message. |
| `.deleteOriginal(value)` | <code>boolean</code> | No | Delete the original interaction message. |
| `.unfurlLinks(value)` | <code>boolean</code> | No | Whether Slack unfurls links. |
| `.unfurlMedia(value)` | <code>boolean</code> | No | Whether Slack unfurls media. |
| `.metadata(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional message metadata. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `attachments?`: [`JsonObject`](utilities.md#jsonobject)[];
  `blocks?`: [`JsonObject`](utilities.md#jsonobject)[];
  `deleteOriginal?`: `boolean`;
  `metadata?`: [`JsonObject`](utilities.md#jsonobject);
  `replaceOriginal?`: `boolean`;
  `responseType?`: `"ephemeral"` \| `"in_channel"`;
  `text?`: `string`;
  `unfurlLinks?`: `boolean`;
  `unfurlMedia?`: `boolean`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;
