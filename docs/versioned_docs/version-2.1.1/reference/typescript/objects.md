---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Composition Objects

Composition-object factories used as fields inside blocks and elements.

These helpers cover text, options, confirmations, workflow metadata, files,
table cells, icons, and chart data.

## areaChart()

> **areaChart**(`series`, `axis`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"area"`&gt;

Creates an area chart.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `series` | [`JsonObject`](utilities.md#jsonobject)[] | One to 12 uniquely named data series. |
| `axis` | [`JsonObject`](utilities.md#jsonobject) | Axis configuration whose categories match every series. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"area"`&gt;

A validated Slack `area` chart object.

### Throws

InvalidUsageError when series and axis categories do not match.

***

## asText()

> **asText**(`value`, `kind?`, `settings?`): [`TextObject`](#textobject)

Converts a string to a text object while preserving existing text objects.

### Parameters

| Parameter | Type | Default value | Description |
| ------ | ------ | ------ | ------ |
| `value` | [`TextLike`](#textlike) | `undefined` | String or existing text object. |
| `kind` | `"plain_text"` \| `"mrkdwn"` | `"mrkdwn"` | Text kind used for strings. Defaults to `mrkdwn`. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | `{}` | Per-call validation settings. |

### Returns

[`TextObject`](#textobject)

A Slack text composition object.

### Throws

InvalidUsageError when the text violates Slack's length constraints.

***

## axisConfig()

> **axisConfig**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates category and label configuration for an axis-based chart.

### Input fields

Ordered categories and optional axis labels.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `categories` | `string`[] | Unique category labels, in display order. |
| `xLabel?` | `string` | Optional horizontal-axis label, up to 50 characters. |
| `yLabel?` | `string` | Optional vertical-axis label, up to 50 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated chart-axis configuration object.

### Throws

InvalidUsageError when labels are duplicated or exceed Slack's limits.

***

## AxisConfigInput

Fields accepted by [axisConfig](#axisconfig).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-categories"></a> `categories` | `string`[] | Unique category labels, in display order. |
| <a id="property-xlabel"></a> `xLabel?` | `string` | Optional horizontal-axis label, up to 50 characters. |
| <a id="property-ylabel"></a> `yLabel?` | `string` | Optional vertical-axis label, up to 50 characters. |

***

## barChart()

> **barChart**(`series`, `axis`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"bar"`&gt;

Creates a grouped bar chart.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `series` | [`JsonObject`](utilities.md#jsonobject)[] | One to 12 uniquely named data series. |
| `axis` | [`JsonObject`](utilities.md#jsonobject) | Axis configuration whose categories match every series. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"bar"`&gt;

A validated Slack `bar` chart object.

### Throws

InvalidUsageError when series and axis categories do not match.

***

## chartSegment()

> **chartSegment**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates one labelled segment for a pie chart.

### Input fields

Segment label and positive value.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `label` | `string` | Segment label, up to 20 characters. |
| `value` | `number` | Positive finite segment value. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated chart-segment object.

### Throws

InvalidUsageError when the label or value violates chart constraints.

***

## ChartSegmentInput

Fields accepted by [chartSegment](#chartsegment).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-label"></a> `label` | `string` | Segment label, up to 20 characters. |
| <a id="property-value"></a> `value` | `number` | Positive finite segment value. |

***

## columnSettings()

> **columnSettings**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates display settings for one table column.

### Input fields

Horizontal alignment and optional wrapping behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `align?` | `"left"` \| `"center"` \| `"right"` | Horizontal cell alignment. |
| `isWrapped?` | `boolean` | Whether long cell content wraps. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack table column-settings object.

***

## confirmation()

> **confirmation**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a confirmation dialog for an interactive element.

### Input fields

Dialog title, question, and button labels.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `confirm` | [`TextLike`](#textlike) | Plain-text confirm-button label, up to 30 characters. |
| `deny` | [`TextLike`](#textlike) | Plain-text cancel-button label, up to 30 characters. |
| `text` | [`TextLike`](#textlike) | Confirmation question, up to 300 characters. |
| `title` | [`TextLike`](#textlike) | Plain-text dialog title, up to 100 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated Slack confirmation object.

### Throws

InvalidUsageError when a text field exceeds Slack's limit.

***

## ConfirmationInput

Fields accepted by [confirmation](#confirmation).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-confirm"></a> `confirm` | [`TextLike`](#textlike) | Plain-text confirm-button label, up to 30 characters. |
| <a id="property-deny"></a> `deny` | [`TextLike`](#textlike) | Plain-text cancel-button label, up to 30 characters. |
| <a id="property-text"></a> `text` | [`TextLike`](#textlike) | Confirmation question, up to 300 characters. |
| <a id="property-title"></a> `title` | [`TextLike`](#textlike) | Plain-text dialog title, up to 100 characters. |

***

## conversationFilter()

> **conversationFilter**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates filters for a conversation select menu.

### Input fields

Conversation types and optional exclusions. At least one field is required.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `excludeBotUsers?` | `boolean` | Exclude direct messages with bots. |
| `excludeExternalSharedChannels?` | `boolean` | Exclude externally shared conversations. |
| `include?` | `string`[] | Conversation kinds to include, such as `im`, `mpim`, `private`, or `public`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated Slack conversation-filter object.

### Throws

MissingRequiredError when no filter field is supplied.

***

## dataPoint()

> **dataPoint**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates one labelled data point for an axis-based chart.

### Input fields

Category label and finite value.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `label` | `string` | Category label, up to 20 characters. |
| `value` | `number` | Finite numeric value. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated data-point object.

### Throws

InvalidUsageError when the label or value violates chart constraints.

***

## DataPointInput

Fields accepted by [dataPoint](#datapoint).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-label-1"></a> `label` | `string` | Category label, up to 20 characters. |
| <a id="property-value-1"></a> `value` | `number` | Finite numeric value. |

***

## dataSeries()

> **dataSeries**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates one named series for an axis-based chart.

### Input fields

Series name and ordered data points.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `data` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 20 points created with `dataPoint`. |
| `name` | `string` | Unique series name, up to 20 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated chart-series object.

### Throws

InvalidUsageError when the name or point count violates chart constraints.

***

## DataSeriesInput

Fields accepted by [dataSeries](#dataseries).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-data"></a> `data` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 20 points created with `dataPoint`. |
| <a id="property-name"></a> `name` | `string` | Unique series name, up to 20 characters. |

***

## dispatchActionConfiguration()

> **dispatchActionConfiguration**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a dispatch-action configuration for an input element.

### Input fields

Interaction events that should dispatch immediately.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `triggerActionsOn` | `string`[] | Events such as `on_enter_pressed` or `on_character_entered`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack dispatch-action configuration object.

***

## inputParameter()

> **inputParameter**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a customizable workflow input parameter.

### Input fields

Workflow parameter name and value.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `name` | `string` | Workflow parameter name. |
| `value` | `string` | Value passed to the workflow. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack workflow input-parameter object.

***

## lineChart()

> **lineChart**(`series`, `axis`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"line"`&gt;

Creates a line chart.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `series` | [`JsonObject`](utilities.md#jsonobject)[] | One to 12 uniquely named data series. |
| `axis` | [`JsonObject`](utilities.md#jsonobject) | Axis configuration whose categories match every series. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"line"`&gt;

A validated Slack `line` chart object.

### Throws

InvalidUsageError when series and axis categories do not match.

***

## MarkdownOptions

Optional behavior for [mrkdwn](#mrkdwn).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-verbatim"></a> `verbatim?` | `boolean` | Whether Slack should treat the text literally instead of auto-parsing links and mentions. |

***

## mrkdwn()

> **mrkdwn**(`text`, `options?`, `settings?`): [`TextObject`](#textobject)

Creates a Slack mrkdwn composition object.

### Input fields

Optional parsing behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `verbatim?` | `boolean` | Whether Slack should treat the text literally instead of auto-parsing links and mentions. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `text` | `string` | Slack mrkdwn content. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`TextObject`](#textobject)

A validated Slack `mrkdwn` object.

### Throws

InvalidUsageError when the text violates Slack's length constraints.

***

## option()

> **option**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates one option for a select, checkbox, radio, or overflow element.

### Input fields

Option label, value, and optional description or URL.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `description?` | [`TextLike`](#textlike) | Optional plain-text supporting copy, up to 75 characters. |
| `text` | [`TextLike`](#textlike) | Plain-text option label, up to 75 characters. |
| `url?` | `string` | Optional destination URL for overflow menus. |
| `value` | `string` | Application-defined value, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated Slack option object.

### Throws

InvalidUsageError when text or values exceed Slack's limits.

***

## optionGroup()

> **optionGroup**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a labelled group of options for a static select menu.

### Input fields

Group label and option list.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `label` | [`TextLike`](#textlike) | Plain-text group label, up to 75 characters. |
| `options` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 100 option objects. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated Slack option-group object.

### Throws

InvalidUsageError when the label or option count violates Slack's limits.

***

## OptionGroupInput

Fields accepted by [optionGroup](#optiongroup).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-label-2"></a> `label` | [`TextLike`](#textlike) | Plain-text group label, up to 75 characters. |
| <a id="property-options"></a> `options` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 100 option objects. |

***

## OptionInput

Fields accepted by [option](#option).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-description"></a> `description?` | [`TextLike`](#textlike) | Optional plain-text supporting copy, up to 75 characters. |
| <a id="property-text-1"></a> `text` | [`TextLike`](#textlike) | Plain-text option label, up to 75 characters. |
| <a id="property-url"></a> `url?` | `string` | Optional destination URL for overflow menus. |
| <a id="property-value-2"></a> `value` | `string` | Application-defined value, up to 150 characters. |

***

## pieChart()

> **pieChart**(`segments`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"pie"`&gt;

Creates a pie chart.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `segments` | [`JsonObject`](utilities.md#jsonobject)[] | One to 12 segments created with `chartSegment`. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"pie"`&gt;

A validated Slack `pie` chart object.

### Throws

InvalidUsageError when segment data violates Slack's chart constraints.

***

## plainText()

> **plainText**(`text`, `options?`, `settings?`): [`TextObject`](#textobject)

Creates a plain-text composition object.

### Input fields

Optional emoji behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `emoji?` | `boolean` | Whether Slack should render emoji shortcodes. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `text` | `string` | Text content. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`TextObject`](#textobject)

A validated Slack `plain_text` object.

### Throws

InvalidUsageError when the text violates Slack's length constraints.

***

## PlainTextOptions

Optional behavior for [plainText](#plaintext).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-emoji"></a> `emoji?` | `boolean` | Whether Slack should render emoji shortcodes. |

***

## rawNumber()

> **rawNumber**(`value`, `text`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"raw_number"`&gt;

Creates a sortable numeric cell for a data table.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `value` | `number` | Finite number used for sorting. |
| `text` | `string` | Human-readable cell text. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"raw_number"`&gt;

A validated Slack `raw_number` object.

### Throws

TypeMismatchError when `value` is not finite.

***

## rawText()

> **rawText**(`text`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"raw_text"`&gt;

Creates an unformatted text cell for a table or data table.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `text` | `string` | Cell content. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"raw_text"`&gt;

A validated Slack `raw_text` object.

### Throws

InvalidUsageError when the cell text violates Slack's constraints.

***

## slackFile()

> **slackFile**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a Slack-hosted image reference.

### Input fields

Exactly one Slack file ID or Slack file URL.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `id?` | `string` | Slack file identifier. |
| `url?` | `string` | Slack-hosted file URL. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated Slack file-reference object.

### Throws

InvalidUsageError when both or neither source is supplied.

***

## slackIcon()

> **slackIcon**(`name`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"icon"`&gt;

Creates a Slack-provided icon for a card.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `name` | [`SlackIconName`](#slackiconname) | One of Slack's supported icon names. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"icon"`&gt;

A validated Slack `icon` object.

### Throws

TypeMismatchError when the icon name is unsupported at runtime.

***

## SlackIconName

> **SlackIconName** = `"archive"` \| `"book"` \| `"bookmark"` \| `"bot"` \| `"bug"` \| `"calendar"` \| `"call"` \| `"caret-left"` \| `"caret-right"` \| `"check"` \| `"clipboard"` \| `"code"` \| `"comment"` \| `"compass"` \| `"copy"` \| `"cube"` \| `"download"` \| `"edit"` \| `"email"` \| `"eye-closed"` \| `"eye-open"` \| `"file"` \| `"flag"` \| `"folder"` \| `"gear"` \| `"globe"` \| `"heart"` \| `"help"` \| `"image"` \| `"info"` \| `"key"` \| `"lightbulb"` \| `"link"` \| `"map"` \| `"mobile"` \| `"new-window"` \| `"pin"` \| `"plus"` \| `"refine"` \| `"refresh"` \| `"rocket"` \| `"save"` \| `"screen"` \| `"share"` \| `"sparkle"` \| `"star"` \| `"star-filled"` \| `"tag"` \| `"thumbs-down"` \| `"thumbs-up"` \| `"trash"` \| `"upload"` \| `"user"` \| `"warning"`

Slack-provided icon name accepted by [slackIcon](#slackicon).

***

## TextLike

> **TextLike** = `string` \| [`TextObject`](#textobject)

Text accepted by factories: a string or an existing Slack text object.

***

## TextObject

> **TextObject** = [`SlackObject`](utilities.md#slackobject)&lt;`"plain_text"` \| `"mrkdwn"`&gt;

A Slack plain-text or mrkdwn composition object.

***

## trigger()

> **trigger**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates a workflow trigger definition.

### Input fields

Trigger URL and optional customizable parameters.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `customizableInputParameters?` | [`JsonObject`](utilities.md#jsonobject)[] | Optional parameters created with `inputParameter`. |
| `url` | `string` | Slack workflow trigger URL. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack workflow-trigger object.

***

## workflow()

> **workflow**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Wraps a trigger for use by a workflow button.

### Input fields

Workflow trigger object.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `trigger` | [`JsonObject`](utilities.md#jsonobject) | Trigger created with `trigger`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A Slack workflow object.
