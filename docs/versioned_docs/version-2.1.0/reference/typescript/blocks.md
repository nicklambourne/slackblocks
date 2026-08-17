---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Blocks

Block factories for messages, modals, and App Home tabs.

Every factory accepts camelCase input, returns Slack's snake_case wire shape,
and validates the result unless `settings.validate` is `false`.

## actionsBlock()

> **actionsBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`ActionsBlock`&gt;

Creates a row of interactive elements.

### Input fields

Up to 25 buttons, select menus, or other action elements.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Interactive elements displayed in the row. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`ActionsBlock`&gt;

A validated Slack `actions` block.

### Throws

InvalidUsageError when an element is unsupported or the limit is exceeded.

***

## alertBlock()

> **alertBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"alert"`&gt;

Creates a severity-labelled notice for a modal.

### Input fields

Alert content, severity, and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `level?` | [`AlertLevel`](#alertlevel) | Visual severity. Defaults to `default`. |
| `text` | [`TextLike`](objects.md#textlike) | Alert copy. Strings are converted to mrkdwn text. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"alert"`&gt;

A validated Slack `alert` block.

### Throws

InvalidUsageError when a field violates Slack's constraints.

***

## AlertLevel

> **AlertLevel** = `"default"` \| `"info"` \| `"warning"` \| `"error"` \| `"success"`

Severity shown by an alert block.

***

## cardBlock()

> **cardBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"card"`&gt;

Creates a compact content card with text, imagery, and optional actions.

### Input fields

Card fields. At least one visible content field must be supplied.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actions?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to three button actions. |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `body?` | [`TextLike`](objects.md#textlike) | Main card copy, up to 200 characters. |
| `heroImage?` | [`JsonObject`](utilities.md#jsonobject) | Large image displayed above the card content. |
| `icon?` | [`JsonObject`](utilities.md#jsonobject) | Small image displayed beside the card heading. |
| `slackIcon?` | [`JsonObject`](utilities.md#jsonobject) | Slack-hosted icon created with `slackIcon`. |
| `subtext?` | [`TextLike`](objects.md#textlike) | Supporting copy displayed below the body. |
| `subtitle?` | [`TextLike`](objects.md#textlike) | Secondary heading, up to 150 characters. |
| `title?` | [`TextLike`](objects.md#textlike) | Primary heading, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"card"`&gt;

A validated Slack `card` block.

### Throws

InvalidUsageError when content is missing or exceeds Slack's limits.

***

## CardBlockInput

Fields accepted by [cardBlock](#cardblock).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actions"></a> `actions?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to three button actions. |
| <a id="property-blockid"></a> `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| <a id="property-body"></a> `body?` | [`TextLike`](objects.md#textlike) | Main card copy, up to 200 characters. |
| <a id="property-heroimage"></a> `heroImage?` | [`JsonObject`](utilities.md#jsonobject) | Large image displayed above the card content. |
| <a id="property-icon"></a> `icon?` | [`JsonObject`](utilities.md#jsonobject) | Small image displayed beside the card heading. |
| <a id="property-slackicon"></a> `slackIcon?` | [`JsonObject`](utilities.md#jsonobject) | Slack-hosted icon created with `slackIcon`. |
| <a id="property-subtext"></a> `subtext?` | [`TextLike`](objects.md#textlike) | Supporting copy displayed below the body. |
| <a id="property-subtitle"></a> `subtitle?` | [`TextLike`](objects.md#textlike) | Secondary heading, up to 150 characters. |
| <a id="property-title"></a> `title?` | [`TextLike`](objects.md#textlike) | Primary heading, up to 150 characters. |

***

## carouselBlock()

> **carouselBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"carousel"`&gt;

Creates a horizontally scrolling collection of cards.

### Input fields

One to ten card blocks and an optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and ten objects returned by `cardBlock`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"carousel"`&gt;

A validated Slack `carousel` block.

### Throws

InvalidUsageError when the card count is outside Slack's limits.

***

## containerBlock()

> **containerBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"container"`&gt;

Creates a titled container that groups related child blocks.

### Input fields

Child blocks plus optional heading, width, icon, and collapse behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `childBlocks` | [`JsonObject`](utilities.md#jsonobject)[] | Up to ten blocks supported by Slack containers. |
| `defaultCollapsed?` | `boolean` | Whether a collapsible container starts collapsed. |
| `hasHeaderDivider?` | `boolean` | Whether Slack draws a divider below the header. |
| `icon?` | [`JsonObject`](utilities.md#jsonobject) | Optional image displayed in the header. |
| `isCollapsible?` | `boolean` | Whether readers can expand and collapse the container. |
| `richTextTitle?` | [`JsonObject`](utilities.md#jsonobject) | Rich-text title block. Mutually exclusive with `title`. |
| `subtitle?` | [`TextLike`](objects.md#textlike) | Optional supporting copy below the title. |
| `title?` | [`TextLike`](objects.md#textlike) | Plain-text title. Mutually exclusive with `richTextTitle`. |
| `width?` | [`ContainerWidth`](#containerwidth) | Container width. Defaults to `standard`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"container"`&gt;

A validated Slack `container` block.

### Throws

InvalidUsageError when child content or collapse options are invalid.

***

## ContainerWidth

> **ContainerWidth** = `"narrow"` \| `"standard"` \| `"wide"` \| `"full"`

Horizontal width used by a container block.

***

## contextActionsBlock()

> **contextActionsBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"context_actions"`&gt;

Creates contextual feedback or icon controls.

### Input fields

Up to five feedback-buttons or icon-button elements.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Feedback-buttons or icon-button elements. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"context_actions"`&gt;

A validated Slack `context_actions` block.

### Throws

InvalidUsageError when an element is unsupported or the limit is exceeded.

***

## contextBlock()

> **contextBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`ContextBlock`&gt;

Creates compact contextual text and images.

### Input fields

Up to ten text objects or image elements.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Text objects and image elements. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`ContextBlock`&gt;

A validated Slack `context` block.

### Throws

InvalidUsageError when an element is unsupported or the limit is exceeded.

***

## dataTableBlock()

> **dataTableBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"data_table"`&gt;

Creates a sortable data table.

### Input fields

Caption, rows, pagination size, and row-header configuration.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `caption` | `string` | Accessible table caption. |
| `pageSize?` | `number` | Rows per page, between 1 and 100. Defaults to 5. |
| `rowHeaderColumnIndex?` | `number` | Zero-based column used as the row header. Defaults to 0. |
| `rows` | [`JsonObject`](utilities.md#jsonobject)[][] | Two to 201 equally sized rows containing raw text, raw numbers, or rich text. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"data_table"`&gt;

A validated Slack `data_table` block.

### Throws

InvalidUsageError when row dimensions, cells, or pagination are invalid.

***

## dataVisualizationBlock()

> **dataVisualizationBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"data_visualization"`&gt;

Creates a chart rendered by Slack.

### Input fields

Chart title, chart object, and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `chart` | [`JsonObject`](utilities.md#jsonobject) | Object returned by `pieChart`, `barChart`, `areaChart`, or `lineChart`. |
| `title` | `string` | Chart heading, up to 50 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"data_visualization"`&gt;

A validated Slack `data_visualization` block.

### Throws

InvalidUsageError when chart data or labels violate Slack's limits.

***

## dividerBlock()

> **dividerBlock**(`input?`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`DividerBlock`&gt;

Creates a visual divider between blocks.

### Input fields

Optional deterministic block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`DividerBlock`&gt;

A validated Slack `divider` block.

### Throws

InvalidUsageError when the block identifier is too long.

***

## fileBlock()

> **fileBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`FileBlock`&gt;

Creates a block that displays a Slack remote file.

### Input fields

Remote-file identifier, source, and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `externalId` | `string` | Identifier assigned when the remote file was added to Slack. |
| `source?` | `"remote"` | Remote-file source. Slack currently accepts only `remote`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`FileBlock`&gt;

A validated Slack `file` block.

### Throws

InvalidUsageError when required file data is missing.

***

## headerBlock()

> **headerBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`HeaderBlock`&gt;

Creates a prominent plain-text heading.

### Input fields

Heading text and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `text` | [`TextLike`](objects.md#textlike) | Heading copy. Strings are converted to plain text. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`HeaderBlock`&gt;

A validated Slack `header` block.

### Throws

InvalidUsageError when the heading exceeds Slack's limit.

***

## imageBlock()

> **imageBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`ImageBlock`&gt;

Creates an image block with optional title text.

### Input fields

Image URL, accessible alternative, title, and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `altText` | `string` | Accessible description of the image. |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `imageUrl` | `string` | Public URL of the image. |
| `title?` | [`TextLike`](objects.md#textlike) | Optional plain-text title. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`ImageBlock`&gt;

A validated Slack `image` block.

### Throws

InvalidUsageError when text or URL fields violate Slack's constraints.

***

## inputBlock()

> **inputBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`InputBlock`&gt;

Creates a labelled form control for a modal or App Home tab.

### Input fields

Label, input-compatible element, and optional form behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `dispatchAction?` | `boolean` | Whether changes dispatch an interaction immediately. |
| `element` | [`JsonObject`](utilities.md#jsonobject) | Input-compatible element such as a text input, picker, or select menu. |
| `hint?` | [`TextLike`](objects.md#textlike) | Optional plain-text help shown below the control. |
| `label` | [`TextLike`](objects.md#textlike) | Plain-text label displayed above the control. |
| `optional?` | `boolean` | Whether the user may submit without completing this input. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`InputBlock`&gt;

A validated Slack `input` block.

### Throws

InvalidUsageError when the element is unsupported or text exceeds Slack's limits.

***

## markdownBlock()

> **markdownBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`MarkdownBlock`&gt;

Creates a block rendered from GitHub-flavored Markdown.

### Input fields

Markdown source and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `text` | `string` | GitHub-flavored Markdown source. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`MarkdownBlock`&gt;

A validated Slack `markdown` block.

### Throws

InvalidUsageError when the Markdown source violates Slack's limits.

***

## planBlock()

> **planBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"plan"`&gt;

Creates a titled sequence of task cards.

### Input fields

Plan title, tasks, and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `tasks?` | [`JsonObject`](utilities.md#jsonobject)[] | Task-card blocks. Their outer `type` and `block_id` fields are omitted in the plan. |
| `title` | `string` | Human-readable plan title. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"plan"`&gt;

A validated Slack `plan` block.

### Throws

InvalidUsageError when task content violates Slack's constraints.

***

## richTextBlock()

> **richTextBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`RichTextBlock`&gt;

Creates a rich-text block from rich-text layout objects.

### Input fields

Rich-text sections, lists, quotes, or code blocks.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Rich-text layout objects. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`RichTextBlock`&gt;

A validated Slack `rich_text` block.

### Throws

InvalidUsageError when an element is not a supported rich-text object.

***

## sectionBlock()

> **sectionBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`SectionBlock`&gt;

Creates a flexible text block with optional fields or an accessory.

### Input fields

Section text, fields, accessory, and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `accessory?` | [`JsonObject`](utilities.md#jsonobject) | Optional interactive or visual element displayed beside the text. |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `fields?` | [`TextLike`](objects.md#textlike)[] | Up to ten text fields displayed in columns. |
| `text?` | [`TextLike`](objects.md#textlike) | Main copy. Strings are converted to mrkdwn text. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`SectionBlock`&gt;

A validated Slack `section` block.

### Throws

InvalidUsageError when text is missing or a field exceeds Slack's limits.

***

## SectionBlockInput

Fields accepted by [sectionBlock](#sectionblock).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-accessory"></a> `accessory?` | [`JsonObject`](utilities.md#jsonobject) | Optional interactive or visual element displayed beside the text. |
| <a id="property-blockid-1"></a> `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| <a id="property-fields"></a> `fields?` | [`TextLike`](objects.md#textlike)[] | Up to ten text fields displayed in columns. |
| <a id="property-text"></a> `text?` | [`TextLike`](objects.md#textlike) | Main copy. Strings are converted to mrkdwn text. |

***

## tableBlock()

> **tableBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`TableBlock`&gt;

Creates a table block from raw-text or rich-text cells.

### Input fields

Rows, optional column display settings, and optional block identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `columnSettings?` | [`JsonObject`](utilities.md#jsonobject)[] | Optional display settings for each column. |
| `rows` | [`JsonObject`](utilities.md#jsonobject)[][] | Up to 100 equally sized rows of raw-text or rich-text cells. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`TableBlock`&gt;

A validated Slack `table` block.

### Throws

InvalidUsageError when dimensions, cells, or column settings are invalid.

***

## taskCardBlock()

> **taskCardBlock**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"task_card"`&gt;

Creates one task card for a plan.

### Input fields

Task identity, title, rich content, sources, and optional status.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `details?` | [`JsonObject`](utilities.md#jsonobject) | Optional rich-text task details. |
| `output?` | [`JsonObject`](utilities.md#jsonobject) | Optional rich-text task output. |
| `sources?` | [`JsonObject`](utilities.md#jsonobject)[] | Optional source links created with `urlSource`. |
| `status?` | [`TaskStatus`](#taskstatus) | Current task lifecycle state. |
| `taskId` | `string` | Stable task identifier. |
| `title` | `string` | Human-readable task title. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"task_card"`&gt;

A validated Slack `task_card` block.

### Throws

InvalidUsageError when identifiers, content, or source links are invalid.

***

## TaskStatus

> **TaskStatus** = `"pending"` \| `"in_progress"` \| `"complete"` \| `"error"`

Lifecycle state shown by a task card.

***

## videoBlock()

> **videoBlock**(`input`, `settings?`): [`SlackWire`](utilities.md#slackwire)&lt;`VideoBlock`&gt;

Creates an embedded video block.

Slack, rather than this library, enforces its provider allowlist when the
payload is submitted.

### Input fields

Video URL, thumbnail, accessible text, title, and optional metadata.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `altText` | `string` | Accessible summary, up to 200 characters. |
| `authorName?` | `string` | Optional author name, up to 50 characters. |
| `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| `description?` | [`TextLike`](objects.md#textlike) | Optional plain-text description, up to 200 characters. |
| `providerIconUrl?` | `string` | Optional provider icon URL. |
| `providerName?` | `string` | Optional provider name, up to 50 characters. |
| `thumbnailUrl` | `string` | Public preview-image URL. |
| `title` | [`TextLike`](objects.md#textlike) | Plain-text video title, up to 200 characters. |
| `titleUrl?` | `string` | Optional destination when the title is selected. |
| `videoUrl` | `string` | URL of a video hosted by a Slack-supported provider. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackWire`](utilities.md#slackwire)&lt;`VideoBlock`&gt;

A validated Slack `video` block.

### Throws

InvalidUsageError when a text field violates Slack's constraints.
