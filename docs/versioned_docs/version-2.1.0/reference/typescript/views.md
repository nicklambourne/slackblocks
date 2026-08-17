---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Views

View factories for Slack modals and App Home tabs.

## homeTab()

> **homeTab**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"home"`&gt;

Creates an App Home tab view payload.

### Input fields

Home-tab blocks and optional application metadata.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blocks` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 100 App Home-compatible blocks. |
| `callbackId?` | `string` | Application-defined callback identifier. |
| `externalId?` | `string` | Application-defined external identifier. |
| `privateMetadata?` | `string` | Opaque application metadata returned with view interactions. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"home"`&gt;

A validated Slack `home` view.

### Throws

InvalidUsageError when blocks or metadata violate Slack's constraints.

***

## modal()

> **modal**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"modal"`&gt;

Creates a modal view payload.

### Input fields

Modal title, blocks, controls, metadata, and callback behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blocks` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 100 modal-compatible blocks. |
| `callbackId?` | `string` | Application-defined callback identifier. |
| `clearOnClose?` | `boolean` | Close every view above this modal when it closes. |
| `close?` | [`TextLike`](objects.md#textlike) | Optional plain-text close-button label. |
| `externalId?` | `string` | Application-defined external identifier. |
| `notifyOnClose?` | `boolean` | Send a `view_closed` event when the modal closes. |
| `privateMetadata?` | `string` | Opaque application metadata returned with view interactions. |
| `submit?` | [`TextLike`](objects.md#textlike) | Optional plain-text submit-button label. |
| `submitDisabled?` | `boolean` | Keep the submit button disabled until an input changes. |
| `title` | [`TextLike`](objects.md#textlike) | Plain-text modal title, up to 24 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"modal"`&gt;

A validated Slack `modal` view.

### Throws

InvalidUsageError when blocks or text fields violate Slack's constraints.
