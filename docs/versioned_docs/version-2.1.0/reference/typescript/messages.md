---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Messages

Payload factories for Web API messages, webhooks, and interaction responses.

## attachment()

> **attachment**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates lower-priority supporting content using Slack's legacy secondary-attachment format.

Attachments can add context beneath a message, while `fallback` supplies a
plain-text summary for notifications and clients that cannot display Block Kit.

### Input fields

Attachment blocks plus optional color and fallback text.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blocks` | [`JsonObject`](utilities.md#jsonobject)[] | Blocks displayed inside the attachment. |
| `color?` | `string` | Optional side-border color: a `Color` value or a six-digit hex code. |
| `fallback?` | `string` | Plain-text fallback for notifications and clients without Block Kit support. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack attachment object.

### Throws

InvalidUsageError when the color is not a valid hex code or a nested
  block violates a supported Block Kit constraint.

### See

https://docs.slack.dev/legacy/legacy-messaging/legacy-secondary-message-attachments

***

## Color

> `const` **Color**: `object`

Preset side-border colors for legacy attachments.

The values mirror the Python `Color` enum: three Slack-recognized aliases
(`good`, `warning`, `danger`) plus common hex colors.

### Type Declaration

| Name | Type | Default value |
| ------ | ------ | ------ |
| <a id="property-black"></a> `BLACK` | `"#000000"` | `"#000000"` |
| <a id="property-blue"></a> `BLUE` | `"#0000ff"` | `"#0000ff"` |
| <a id="property-danger"></a> `DANGER` | `"danger"` | `"danger"` |
| <a id="property-good"></a> `GOOD` | `"good"` | `"good"` |
| <a id="property-green"></a> `GREEN` | `"#00ff00"` | `"#00ff00"` |
| <a id="property-orange"></a> `ORANGE` | `"#ff8800"` | `"#ff8800"` |
| <a id="property-purple"></a> `PURPLE` | `"#8800ff"` | `"#8800ff"` |
| <a id="property-red"></a> `RED` | `"#ff0000"` | `"#ff0000"` |
| <a id="property-warning"></a> `WARNING` | `"warning"` | `"warning"` |
| <a id="property-yellow"></a> `YELLOW` | `"#ffff00"` | `"#ffff00"` |

***

## message()

> **message**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a payload for Slack Web API message methods such as `chat.postMessage`.

### Input fields

Destination channel and message content.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `attachments?` | [`JsonObject`](utilities.md#jsonobject)[] | Optional secondary attachments. |
| `blocks?` | [`JsonObject`](utilities.md#jsonobject)[] | Block Kit blocks displayed in the message. |
| `channel` | `string` | Channel, group, or direct-message conversation identifier. |
| `metadata?` | [`JsonObject`](utilities.md#jsonobject) | Optional message metadata. |
| `mrkdwn?` | `boolean` | Whether Slack parses `text` as mrkdwn. Defaults to `true`. |
| `text?` | `string` | Notification and accessibility fallback text. |
| `unfurlLinks?` | `boolean` | Whether Slack unfurls links. |
| `unfurlMedia?` | `boolean` | Whether Slack unfurls media. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack-shaped message payload ready to spread into an SDK call.

### Throws

InvalidUsageError when nested Block Kit content is invalid.

***

## MessageInput

Fields accepted by [message](#message).

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

## messageResponse()

> **messageResponse**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a response payload for slash commands and interactive requests.

### Input fields

Response content, visibility, and replacement behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `attachments?` | [`JsonObject`](utilities.md#jsonobject)[] | Optional secondary attachments. |
| `blocks?` | [`JsonObject`](utilities.md#jsonobject)[] | Block Kit blocks displayed in the response. |
| `mrkdwn?` | `boolean` | Whether Slack parses `text` as mrkdwn. Defaults to `true`. |
| `replaceOriginal?` | `boolean` | Replace the original interaction message. Defaults to `false`. |
| `responseType?` | `"ephemeral"` \| `"in_channel"` | Response visibility. Defaults to `in_channel`. |
| `text?` | `string` | Notification and accessibility fallback text. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack interaction-response payload.

### Throws

InvalidUsageError when nested Block Kit content is invalid.

***

## webhookMessage()

> **webhookMessage**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a payload for an incoming webhook or response URL.

### Input fields

Message content, visibility, replacement behavior, and unfurl settings.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `attachments?` | [`JsonObject`](utilities.md#jsonobject)[] | Optional secondary attachments. |
| `blocks?` | [`JsonObject`](utilities.md#jsonobject)[] | Block Kit blocks displayed in the message. |
| `deleteOriginal?` | `boolean` | Delete the original interaction message. |
| `metadata?` | [`JsonObject`](utilities.md#jsonobject) | Optional message metadata. |
| `replaceOriginal?` | `boolean` | Replace the original interaction message. |
| `responseType?` | `"ephemeral"` \| `"in_channel"` | Response visibility for response URLs. |
| `text?` | `string` | Notification and accessibility fallback text. |
| `unfurlLinks?` | `boolean` | Whether Slack unfurls links. |
| `unfurlMedia?` | `boolean` | Whether Slack unfurls media. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack webhook-message payload.

### Throws

InvalidUsageError when nested Block Kit content is invalid.
