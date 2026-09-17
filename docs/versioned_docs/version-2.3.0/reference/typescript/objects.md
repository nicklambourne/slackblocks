---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Composition Objects

Fluent builders for reusable composition objects nested inside blocks and
elements. This includes text, selectable options, confirmation dialogs, files,
workflow metadata, rich text, table cells, and chart data.

See: <https://docs.slack.dev/reference/block-kit/composition-objects>.

## AreaChart()

```ts
function AreaChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"area">>;
```

Creates a layered area chart for a data visualization block. Supply one or more
named series and an axis configuration whose categories match every point in
those series.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.series(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | One or more named data series to plot. |
| `.axisConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Axis labels and category configuration. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`AxisChartBuilderInput`, [`SlackObject`](utilities.md#slackobject)&lt;`"area"`&gt;&gt;

***

## AxisConfig()

```ts
function AxisConfig(): FluentBuilder<AxisConfigInput, JsonObject>;
```

Creates category labels and optional axis titles for a bar, area, or line chart.
Categories must be unique and match the labels represented in every data
series.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.categories(...values)` | <code>string[]</code> | Yes | Unique category labels, in display order. |
| `.xLabel(value)` | <code>string</code> | No | Optional horizontal-axis label, up to 50 characters. |
| `.yLabel(value)` | <code>string</code> | No | Optional vertical-axis label, up to 50 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`AxisConfigInput`](#axisconfiginput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## AxisConfigInput

Ordered unique categories and optional axis titles shared by every series in an axis-
based chart.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-categories"></a> `categories` | `string`[] | Unique category labels, in display order. |
| <a id="property-xlabel"></a> `xLabel?` | `string` | Optional horizontal-axis label, up to 50 characters. |
| <a id="property-ylabel"></a> `yLabel?` | `string` | Optional vertical-axis label, up to 50 characters. |

***

## BarChart()

```ts
function BarChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"bar">>;
```

Creates a grouped bar chart for a data visualization block. Supply one or more
named series and an axis configuration whose categories match every point in
those series.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.series(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | One or more named data series to plot. |
| `.axisConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Axis labels and category configuration. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`AxisChartBuilderInput`, [`SlackObject`](utilities.md#slackobject)&lt;`"bar"`&gt;&gt;

***

## ChartSegment()

```ts
function ChartSegment(): FluentBuilder<ChartSegmentInput, JsonObject>;
```

Creates one labelled, positive-valued segment in a pie chart. Segment labels
are limited to 20 characters and each value must be a finite number greater
than zero.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.label(value)` | <code>string</code> | Yes | Segment label, up to 20 characters. |
| `.value(value)` | <code>number</code> | Yes | Positive finite segment value. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ChartSegmentInput`](#chartsegmentinput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## ChartSegmentInput

Label and positive finite value for one segment in a Slack-rendered pie chart.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-label"></a> `label` | `string` | Segment label, up to 20 characters. |
| <a id="property-value"></a> `value` | `number` | Positive finite segment value. |

***

## ColumnSettings()

```ts
function ColumnSettings(): FluentBuilder<{
  align?: "left" | "center" | "right";
  isWrapped?: boolean;
}, JsonObject>;
```

Creates display settings for one column in a table block. Configure horizontal
alignment and whether long cell content should wrap within the column.

See: <https://docs.slack.dev/reference/block-kit/blocks/table-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.align(value)` | <code>"left" &#124; "center" &#124; "right"</code> | No | Horizontal cell alignment. |
| `.isWrapped(value)` | <code>boolean</code> | No | Whether long cell content wraps. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `align?`: `"left"` \| `"center"` \| `"right"`;
  `isWrapped?`: `boolean`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## Confirmation()

```ts
function Confirmation(): FluentBuilder<ConfirmationInput, JsonObject>;
```

Creates a confirmation dialog that asks a user to approve or cancel an
interactive action. Configure its title, explanatory text, and the labels on
both the confirm and deny buttons.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/confirmation-dialog-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.title(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text dialog title, up to 100 characters. |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Confirmation question, up to 300 characters. |
| `.confirm(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text confirm-button label, up to 30 characters. |
| `.deny(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text cancel-button label, up to 30 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ConfirmationInput`](#confirmationinput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## ConfirmationInput

Text and button labels required to present a confirmation step before an interactive
action is submitted.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-confirm"></a> `confirm` | [`TextLike`](#textlike) | Plain-text confirm-button label, up to 30 characters. |
| <a id="property-deny"></a> `deny` | [`TextLike`](#textlike) | Plain-text cancel-button label, up to 30 characters. |
| <a id="property-text"></a> `text` | [`TextLike`](#textlike) | Confirmation question, up to 300 characters. |
| <a id="property-title"></a> `title` | [`TextLike`](#textlike) | Plain-text dialog title, up to 100 characters. |

***

## ConversationFilter()

```ts
function ConversationFilter(): FluentBuilder<{
  excludeBotUsers?: boolean;
  excludeExternalSharedChannels?: boolean;
  include?: string[];
}, JsonObject>;
```

Creates a filter for conversation single-select and multi-select elements.
Include selected conversation kinds and optionally exclude externally shared
channels or direct messages with bots; at least one filter field is required.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.include(...values)` | <code>string[]</code> | No | Conversation kinds to include, such as `im`, `mpim`, `private`, or `public`. |
| `.excludeExternalSharedChannels(value)` | <code>boolean</code> | No | Exclude externally shared conversations. |
| `.excludeBotUsers(value)` | <code>boolean</code> | No | Exclude direct messages with bots. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `excludeBotUsers?`: `boolean`;
  `excludeExternalSharedChannels?`: `boolean`;
  `include?`: `string`[];
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## DataPoint()

```ts
function DataPoint(): FluentBuilder<DataPointInput, JsonObject>;
```

Creates one labelled numeric point in a bar, area, or line chart series. Its
label must correspond to one of the chart's axis categories and its value must
be finite.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.label(value)` | <code>string</code> | Yes | Category label, up to 20 characters. |
| `.value(value)` | <code>number</code> | Yes | Finite numeric value. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`DataPointInput`](#datapointinput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## DataPointInput

Category label and finite numeric value for one point in an axis-based chart series.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-label-1"></a> `label` | `string` | Category label, up to 20 characters. |
| <a id="property-value-1"></a> `value` | `number` | Finite numeric value. |

***

## DataSeries()

```ts
function DataSeries(): FluentBuilder<DataSeriesInput, JsonObject>;
```

Creates a named series containing between one and 20 data points for a bar,
area, or line chart. Add points in display order with repeated `data()` calls.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.name(value)` | <code>string</code> | Yes | Unique series name, up to 20 characters. |
| `.data(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Between one and 20 points created with `DataPoint()`. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`DataSeriesInput`](#dataseriesinput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## DataSeriesInput

Unique name and ordered data points for one series in a bar, area, or line chart.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-data"></a> `data` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 20 points created with `dataPoint`. |
| <a id="property-name"></a> `name` | `string` | Unique series name, up to 20 characters. |

***

## DispatchActionConfiguration()

```ts
function DispatchActionConfiguration(): FluentBuilder<{
  triggerActionsOn: string[];
}, JsonObject>;
```

Creates a dispatch-action configuration for an input element. Select the input
events, such as Enter being pressed or a character being entered, that should
immediately send a `block_actions` payload to the application.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.triggerActionsOn(...values)` | <code>string[]</code> | Yes | Events such as `on_enter_pressed` or `on_character_entered`. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `triggerActionsOn`: `string`[];
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## InputParameter()

```ts
function InputParameter(): FluentBuilder<{
  name: string;
  value: string;
}, JsonObject>;
```

Creates one customizable input parameter passed to a Slack workflow trigger.
The parameter name must match an input defined by the workflow, and its value
is supplied when the workflow button is used.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.name(value)` | <code>string</code> | Yes | Workflow parameter name. |
| `.value(value)` | <code>string</code> | Yes | Value passed to the workflow. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `name`: `string`;
  `value`: `string`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## LineChart()

```ts
function LineChart(): FluentBuilder<AxisChartBuilderInput, SlackObject<"line">>;
```

Creates a line chart for a data visualization block. Supply one or more named
series and an axis configuration whose categories match every point in those
series.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.series(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | One or more named data series to plot. |
| `.axisConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Axis labels and category configuration. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`AxisChartBuilderInput`, [`SlackObject`](utilities.md#slackobject)&lt;`"line"`&gt;&gt;

***

<a id="mrkdwn"></a>

## Markdown()

```ts
function Markdown(): FluentBuilder<MarkdownBuilderInput, TextObject>;
```

Creates a text composition object rendered with Slack's `mrkdwn` syntax. Use
`verbatim()` when links, mentions, and other tokens should remain literal
rather than being parsed automatically.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/text-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code>string</code> | Yes | Slack mrkdwn text to display. |
| `.verbatim(value)` | <code>boolean</code> | No | Whether Slack should treat the text literally instead of auto-parsing links and mentions. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`MarkdownBuilderInput`, [`TextObject`](#textobject)&gt;

***

## MarkdownOptions

Optional parsing behavior for a `mrkdwn` text object, controlling automatic links and
mentions.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-verbatim"></a> `verbatim?` | `boolean` | Whether Slack should treat the text literally instead of auto-parsing links and mentions. |

***

## Option()

```ts
function Option(): FluentBuilder<OptionInput, JsonObject>;
```

Creates one selectable item for a select menu, multi-select menu, checkbox
group, radio-button group, or overflow menu. The application-defined `value`
is returned when the user chooses the option.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/option-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text option label, up to 75 characters. |
| `.value(value)` | <code>string</code> | Yes | Application-defined value, up to 150 characters. |
| `.description(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional plain-text supporting copy, up to 75 characters. |
| `.url(value)` | <code>string</code> | No | Optional destination URL for overflow menus. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`OptionInput`](#optioninput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## OptionGroup()

```ts
function OptionGroup(): FluentBuilder<OptionGroupInput, JsonObject>;
```

Creates a labelled group containing between one and 100 selectable options.
Option groups can organize choices in static single-select and multi-select
menus.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.label(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text group label, up to 75 characters. |
| `.options(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Between one and 100 option objects. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`OptionGroupInput`](#optiongroupinput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## OptionGroupInput

Label and choices for a group of between one and 100 options in a static select menu.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-label-2"></a> `label` | [`TextLike`](#textlike) | Plain-text group label, up to 75 characters. |
| <a id="property-options"></a> `options` | [`JsonObject`](utilities.md#jsonobject)[] | Between one and 100 option objects. |

***

## OptionInput

Label, returned value, and optional supporting content for one selectable option.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-description"></a> `description?` | [`TextLike`](#textlike) | Optional plain-text supporting copy, up to 75 characters. |
| <a id="property-text-1"></a> `text` | [`TextLike`](#textlike) | Plain-text option label, up to 75 characters. |
| <a id="property-url"></a> `url?` | `string` | Optional destination URL for overflow menus. |
| <a id="property-value-2"></a> `value` | `string` | Application-defined value, up to 150 characters. |

***

## PieChart()

```ts
function PieChart(): FluentBuilder<{
  segments: JsonObject[];
}, SlackObject<"pie">>;
```

Creates a pie chart containing between one and 12 labelled segments for a data
visualization block. Add built segments or [ChartSegment](#chartsegment) builders with
repeated `segments()` calls.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.segments(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | — |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `segments`: [`JsonObject`](utilities.md#jsonobject)[];
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"pie"`&gt;&gt;

***

## PlainText()

```ts
function PlainText(): FluentBuilder<PlainTextBuilderInput, TextObject>;
```

Creates a plain-text composition object with no Slack formatting. Use it for
labels and other fields that require `plain_text`; emoji shortcodes can be
converted by enabling the optional `emoji` setting.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/text-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code>string</code> | Yes | Text to display. |
| `.emoji(value)` | <code>boolean</code> | No | Whether Slack should render emoji shortcodes. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`PlainTextBuilderInput`, [`TextObject`](#textobject)&gt;

***

## PlainTextOptions

Optional rendering behavior for a plain-text object, controlling whether Slack expands
emoji shortcodes.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-emoji"></a> `emoji?` | `boolean` | Whether Slack should render emoji shortcodes. |

***

## RawNumber()

```ts
function RawNumber(): FluentBuilder<{
  text: string;
  value: number;
}, SlackObject<"raw_number">>;
```

Creates a numeric data-table cell with separate machine-sortable and
human-readable values. The numeric value must be finite, while `text()` controls
what Slack displays to the reader.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.value(value)` | <code>number</code> | Yes | — |
| `.text(value)` | <code>string</code> | Yes | — |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `text`: `string`;
  `value`: `number`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"raw_number"`&gt;&gt;

***

## RawText()

```ts
function RawText(): FluentBuilder<{
  text: string;
}, SlackObject<"raw_text">>;
```

Creates an unformatted `raw_text` cell for a table or data table. Slack displays
the supplied text literally, without applying `mrkdwn` or rich-text formatting.

See: <https://docs.slack.dev/reference/block-kit/blocks/table-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code>string</code> | Yes | — |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `text`: `string`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"raw_text"`&gt;&gt;

***

## RichText()

```ts
function RichText(): FluentBuilder<RichTextBuilderInput, JsonObject>;
```

Creates the core text run used by Slack's structured rich-text API. Apply bold,
italic, strikethrough, or code styling and place the result inside a rich-text
section, list, code block, or quote.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#text-element-type>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code>string</code> | Yes | Text content for this run. |
| `.style(value)` | <code><a href="/reference/typescript/objects#richtextstyle">RichTextStyle</a></code> | No | Optional Slack rich-text styling. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`RichTextBuilderInput`, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextChannel()

```ts
function RichTextChannel(): FluentBuilder<RichTextMentionBuilderInput, JsonObject>;
```

Creates a structured rich-text mention for a Slack channel, such as `#general`.
Slack resolves the supplied channel ID when rendering the containing rich-text
block.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#channel-element-type>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.id(value)` | <code>string</code> | Yes | Slack channel, user, or user-group identifier. |
| `.style(value)` | <code><a href="/reference/typescript/objects#richtextstyle">RichTextStyle</a></code> | No | Optional Slack rich-text styling. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`RichTextMentionBuilderInput`, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextCodeBlock()

```ts
function RichTextCodeBlock(): FluentBuilder<RichTextLayoutBuilderInput, JsonObject>;
```

Creates a preformatted rich-text code block, roughly equivalent to a fenced code
block in Markdown. Add structured rich-text elements as its content and
optionally configure the surrounding border.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_preformatted>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Rich-text elements displayed inside the layout. |
| `.border(value)` | <code>number</code> | No | Optional border width. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`RichTextLayoutBuilderInput`, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextEmoji()

```ts
function RichTextEmoji(): FluentBuilder<{
  name: string;
}, JsonObject>;
```

Creates a structured rich-text emoji using a built-in Slack name or a custom
workspace emoji name. Supply the name without surrounding colon characters.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#emoji-element-type>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.name(value)` | <code>string</code> | Yes | — |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `name`: `string`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextLink()

```ts
function RichTextLink(): FluentBuilder<{
  style?: RichTextStyle;
  text?: string;
  unsafe?: boolean;
  url: string;
}, JsonObject>;
```

Creates a structured rich-text link with a destination URL and optional display
text or style. When text is omitted, Slack displays the URL itself.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#link-element-type>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.url(value)` | <code>string</code> | Yes | Destination URL. |
| `.text(value)` | <code>string</code> | No | Optional visible label. Slack displays the URL when omitted. |
| `.unsafe(value)` | <code>boolean</code> | No | Mark a URL as unsafe when mirroring a Slack-provided payload. |
| `.style(value)` | <code><a href="/reference/typescript/objects#richtextstyle">RichTextStyle</a></code> | No | Optional inline formatting. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `style?`: [`RichTextStyle`](#richtextstyle);
  `text?`: `string`;
  `unsafe?`: `boolean`;
  `url`: `string`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextList()

```ts
function RichTextList(): FluentBuilder<{
  border?: number;
  elements: JsonObject[];
  indent?: number;
  offset?: number;
  style: "bullet" | "ordered";
}, JsonObject>;
```

Creates an ordered or bulleted list of rich-text sections. Configure indentation
and list style, then add each section with `elements()` in display order.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_list>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Rich-text section objects used as list items. |
| `.style(value)` | <code>"bullet" &#124; "ordered"</code> | Yes | List marker style. |
| `.indent(value)` | <code>number</code> | No | Nesting depth. |
| `.offset(value)` | <code>number</code> | No | Starting number for an ordered list. |
| `.border(value)` | <code>number</code> | No | Optional border thickness. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `border?`: `number`;
  `elements`: [`JsonObject`](utilities.md#jsonobject)[];
  `indent?`: `number`;
  `offset?`: `number`;
  `style`: `"bullet"` \| `"ordered"`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextQuote()

```ts
function RichTextQuote(): FluentBuilder<RichTextLayoutBuilderInput, JsonObject>;
```

Creates a rich-text quotation rendered with a vertical bar beside its content.
Add structured rich-text elements in display order and optionally configure the
surrounding border.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_quote>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Rich-text elements displayed inside the layout. |
| `.border(value)` | <code>number</code> | No | Optional border width. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`RichTextLayoutBuilderInput`, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextSection()

```ts
function RichTextSection(): FluentBuilder<{
  elements: JsonObject[];
}, JsonObject>;
```

Creates the basic paragraph-like container for structured rich-text elements.
Add text runs, links, emoji, and mentions with `elements()` before placing the
section in a rich-text block or higher-level rich-text layout.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_section>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | — |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `elements`: [`JsonObject`](utilities.md#jsonobject)[];
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

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

## RichTextUser()

```ts
function RichTextUser(): FluentBuilder<RichTextMentionBuilderInput, JsonObject>;
```

Creates a structured rich-text mention for one Slack user. Slack resolves the
supplied user ID to the appropriate display name when rendering the containing
rich-text block.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-element-type>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.id(value)` | <code>string</code> | Yes | Slack channel, user, or user-group identifier. |
| `.style(value)` | <code><a href="/reference/typescript/objects#richtextstyle">RichTextStyle</a></code> | No | Optional Slack rich-text styling. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`RichTextMentionBuilderInput`, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## RichTextUserGroup()

```ts
function RichTextUserGroup(): FluentBuilder<RichTextMentionBuilderInput, JsonObject>;
```

Creates a structured rich-text mention for a Slack user group. Slack resolves
the supplied user-group ID when rendering the containing rich-text block.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#usergroup-element-type>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.id(value)` | <code>string</code> | Yes | Slack channel, user, or user-group identifier. |
| `.style(value)` | <code><a href="/reference/typescript/objects#richtextstyle">RichTextStyle</a></code> | No | Optional Slack rich-text styling. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`RichTextMentionBuilderInput`, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## SlackFile()

```ts
function SlackFile(): FluentBuilder<{
  id?: string;
  url?: string;
}, JsonObject>;
```

Creates a Slack-hosted image reference for an image block or image element.
Supply exactly one Slack file ID or Slack-hosted file URL; the two source forms
are mutually exclusive.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/slack-file-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.id(value)` | <code>string</code> | No | Slack file identifier. |
| `.url(value)` | <code>string</code> | No | Slack-hosted file URL. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `id?`: `string`;
  `url?`: `string`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## SlackIcon()

```ts
function SlackIcon(): FluentBuilder<{
  name: SlackIconName;
}, SlackObject<"icon">>;
```

Creates a named icon supplied and rendered by Slack for use in a card block.
Choose one of the supported [SlackIconName](#slackiconname) values instead of supplying
an image URL.

See: <https://docs.slack.dev/reference/block-kit/blocks/card-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.name(value)` | <code><a href="/reference/typescript/objects#slackiconname">SlackIconName</a></code> | Yes | — |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `name`: [`SlackIconName`](#slackiconname);
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"icon"`&gt;&gt;

***

## SlackIconName

```ts
type SlackIconName = 
  | "archive"
  | "book"
  | "bookmark"
  | "bot"
  | "bug"
  | "calendar"
  | "call"
  | "caret-left"
  | "caret-right"
  | "check"
  | "clipboard"
  | "code"
  | "comment"
  | "compass"
  | "copy"
  | "cube"
  | "download"
  | "edit"
  | "email"
  | "eye-closed"
  | "eye-open"
  | "file"
  | "flag"
  | "folder"
  | "gear"
  | "globe"
  | "heart"
  | "help"
  | "image"
  | "info"
  | "key"
  | "lightbulb"
  | "link"
  | "map"
  | "mobile"
  | "new-window"
  | "pin"
  | "plus"
  | "refine"
  | "refresh"
  | "rocket"
  | "save"
  | "screen"
  | "share"
  | "sparkle"
  | "star"
  | "star-filled"
  | "tag"
  | "thumbs-down"
  | "thumbs-up"
  | "trash"
  | "upload"
  | "user"
  | "warning";
```

Slack-provided icon name accepted by slackIcon.

***

## TextLike

```ts
type TextLike = string | TextObject;
```

Text accepted by factories: a string or an existing Slack text object.

***

## TextObject

```ts
type TextObject = SlackObject<"plain_text" | "mrkdwn">;
```

A Slack plain-text or mrkdwn composition object.

***

## Trigger()

```ts
function Trigger(): FluentBuilder<{
  customizableInputParameters?: JsonObject[];
  url: string;
}, JsonObject>;
```

Creates the link-trigger definition nested inside a workflow object. Supply the
trigger URL generated by Slack and optionally add customizable input parameters.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.url(value)` | <code>string</code> | Yes | Slack workflow trigger URL. |
| `.customizableInputParameters(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Optional parameters created with `InputParameter()`. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `customizableInputParameters?`: [`JsonObject`](utilities.md#jsonobject)[];
  `url`: `string`;
\}, [`JsonObject`](utilities.md#jsonobject)&gt;

***

## Workflow()

```ts
function Workflow(): FluentBuilder<{
  trigger: JsonObject;
}, JsonObject>;
```

Creates a workflow composition object for a workflow button. It wraps a trigger
built with [Trigger](#trigger), including any customizable values the application
wants to pass when the user launches the workflow.

See: <https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.trigger(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Trigger created with `Trigger()`. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `trigger`: [`JsonObject`](utilities.md#jsonobject);
\}, [`JsonObject`](utilities.md#jsonobject)&gt;
