---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Rich Text

Inline and layout factories for Slack rich-text blocks.

Build inline elements first, combine them in a section, list, quote, or code
block, then pass those layout objects to `richTextBlock`.

## richText()

> **richText**(`text`, `style?`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"text"`&gt;

Creates a styled rich-text text run.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `text` | `string` | Text content. |
| `style?` | [`RichTextStyle`](#richtextstyle) | Optional inline formatting. |
| `settings?` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"text"`&gt;

A validated Slack rich-text `text` element.

***

## richTextChannel()

> **richTextChannel**(`channelId`, `style?`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"channel"`&gt;

Creates a rich-text channel mention.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `channelId` | `string` | Slack channel identifier. |
| `style?` | [`RichTextStyle`](#richtextstyle) | Optional inline formatting. |
| `settings?` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"channel"`&gt;

A validated Slack rich-text `channel` element.

***

## richTextCodeBlock()

> **richTextCodeBlock**(`elements`, `options?`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_preformatted"`&gt;

Creates a preformatted rich-text code block.

### Input fields

Optional border thickness.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `border?` | `number` | Optional border thickness. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Inline rich-text elements displayed as code. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_preformatted"`&gt;

A validated Slack `rich_text_preformatted` object.

***

## richTextEmoji()

> **richTextEmoji**(`name`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"emoji"`&gt;

Creates a rich-text emoji.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `name` | `string` | Emoji name without surrounding colons. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"emoji"`&gt;

A validated Slack rich-text `emoji` element.

***

## richTextLink()

> **richTextLink**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"link"`&gt;

Creates a rich-text link.

### Input fields

Destination URL, optional label, safety flag, and formatting.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `style?` | [`RichTextStyle`](#richtextstyle) | Optional inline formatting. |
| `text?` | `string` | Optional visible label. Slack displays the URL when omitted. |
| `unsafe?` | `boolean` | Mark a URL as unsafe when mirroring a Slack-provided payload. |
| `url` | `string` | Destination URL. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"link"`&gt;

A validated Slack rich-text `link` element.

***

## richTextList()

> **richTextList**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_list"`&gt;

Creates an ordered or bulleted rich-text list.

### Input fields

List items, marker style, and optional list layout.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `border?` | `number` | Optional border thickness. |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Rich-text section objects used as list items. |
| `indent?` | `number` | Nesting depth. |
| `offset?` | `number` | Starting number for an ordered list. |
| `style` | `"bullet"` \| `"ordered"` | List marker style. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_list"`&gt;

A validated Slack `rich_text_list` object.

### Throws

MissingRequiredError when `style` is not provided.

***

## richTextQuote()

> **richTextQuote**(`elements`, `options?`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_quote"`&gt;

Creates a rich-text block quote.

### Input fields

Optional border thickness.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `border?` | `number` | Optional border thickness. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Inline rich-text elements displayed in the quote. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_quote"`&gt;

A validated Slack `rich_text_quote` object.

***

## richTextSection()

> **richTextSection**(`elements`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_section"`&gt;

Creates a paragraph-like rich-text section.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `elements` | [`JsonObject`](utilities.md#jsonobject)[] | Inline rich-text elements in display order. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_section"`&gt;

A validated Slack `rich_text_section` object.

***

## RichTextStyle

Inline formatting supported by rich-text text, links, users, and channels.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-bold"></a> `bold?` | `boolean` | Render the inline content in bold. |
| <a id="property-code"></a> `code?` | `boolean` | Render the inline content as code. |
| <a id="property-italic"></a> `italic?` | `boolean` | Render the inline content in italics. |
| <a id="property-strike"></a> `strike?` | `boolean` | Render the inline content with a strikethrough. |

***

## richTextUser()

> **richTextUser**(`userId`, `style?`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"user"`&gt;

Creates a rich-text user mention.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `userId` | `string` | Slack user identifier. |
| `style?` | [`RichTextStyle`](#richtextstyle) | Optional inline formatting. |
| `settings?` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"user"`&gt;

A validated Slack rich-text `user` element.

***

## richTextUserGroup()

> **richTextUserGroup**(`usergroupId`, `style?`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"usergroup"`&gt;

Creates a rich-text user-group mention.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `usergroupId` | `string` | Slack user-group identifier. |
| `style?` | [`RichTextStyle`](#richtextstyle) | Optional inline formatting. |
| `settings?` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"usergroup"`&gt;

A validated Slack rich-text `usergroup` element.
