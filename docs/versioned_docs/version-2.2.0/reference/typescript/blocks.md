---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Blocks

Fluent builders for the Block Kit containers that make up messages, modals,
and App Home tabs. Each PascalCase function returns a chainable builder; set
its content and options, then call `.build()` for validated Slack wire data.

See: <https://docs.slack.dev/reference/block-kit/blocks>.

## ActionsBlock()

```ts
function ActionsBlock(): FluentBuilder<{
  blockId?: string;
  elements: JsonObject[];
}, SlackWire<ActionsBlock>>;
```

Creates a fluent block that holds interactive controls such as buttons, select
menus, and date pickers. Add up to 25 supported elements with `elements()`;
Slack sends their action identifiers back in interaction payloads.

See: <https://docs.slack.dev/reference/block-kit/blocks/actions-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Interactive elements displayed in the row. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `elements`: [`JsonObject`](utilities.md#jsonobject)[];
\}, [`SlackWire`](utilities.md#slackwire)&lt;`ActionsBlock`&gt;&gt;

***

## AlertBlock()

```ts
function AlertBlock(): FluentBuilder<{
  blockId?: string;
  level?: AlertLevel;
  text: TextLike;
}, SlackObject<"alert">>;
```

Creates a fluent severity-labelled alert for a modal. Supply the alert copy as
a string or text object and choose one of Slack's supported severity levels.

See: <https://docs.slack.dev/reference/block-kit/blocks/alert-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Alert copy. Strings are converted to mrkdwn text. |
| `.level(value)` | <code><a href="/reference/typescript/blocks#alertlevel">AlertLevel</a></code> | No | Visual severity. Defaults to `default`. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `level?`: [`AlertLevel`](#alertlevel);
  `text`: [`TextLike`](objects.md#textlike);
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"alert"`&gt;&gt;

***

## AlertLevel

```ts
type AlertLevel = "default" | "info" | "warning" | "error" | "success";
```

Severity shown by an alert block.

***

## CardBlock()

```ts
function CardBlock(): FluentBuilder<CardBlockInput, SlackObject<"card">>;
```

Creates a fluent compact card containing text, images, and up to three button
actions. A card may stand alone or appear in a [CarouselBlock](#carouselblock); at least
one visible content field must be set before calling `.build()`.

See: <https://docs.slack.dev/reference/block-kit/blocks/card-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.heroImage(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Large image displayed above the card content. |
| `.icon(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Small image displayed beside the card heading. |
| `.title(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Primary heading, up to 150 characters. |
| `.subtitle(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Secondary heading, up to 150 characters. |
| `.body(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Main card copy, up to 200 characters. |
| `.actions(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Up to three button actions. |
| `.slackIcon(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Slack-hosted icon created with `SlackIcon()`. |
| `.subtext(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Supporting copy displayed below the body. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`CardBlockInput`](#cardblockinput), [`SlackObject`](utilities.md#slackobject)&lt;`"card"`&gt;&gt;

***

## CardBlockInput

Content and presentation fields configured by `CardBlock()`. A card must set at
least one of `heroImage`, `title`, `actions`, or `body` before it is built.

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

## CarouselBlock()

```ts
function CarouselBlock(): FluentBuilder<{
  blockId?: string;
  elements: JsonObject[];
}, SlackObject<"carousel">>;
```

Creates a fluent horizontally scrolling group of between one and ten cards.
Add each card with `elements()` using a built card, a [CardBlock](#cardblock)
builder, or an array containing either form.

See: <https://docs.slack.dev/reference/block-kit/blocks/carousel-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Between one and ten objects returned by `CardBlock()`. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `elements`: [`JsonObject`](utilities.md#jsonobject)[];
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"carousel"`&gt;&gt;

***

## ContainerBlock()

```ts
function ContainerBlock(): FluentBuilder<{
  blockId?: string;
  childBlocks: JsonObject[];
  defaultCollapsed?: boolean;
  hasHeaderDivider?: boolean;
  icon?: JsonObject;
  isCollapsible?: boolean;
  richTextTitle?: JsonObject;
  subtitle?: TextLike;
  title?: TextLike;
  width?: ContainerWidth;
}, SlackObject<"container">>;
```

Creates a fluent titled container that groups up to ten supported child
blocks. Set either `title()` or `richTextTitle()` and optionally make the
container collapsible, choose its width, or add supporting header content.

See: <https://docs.slack.dev/reference/block-kit/blocks/container-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.childBlocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Up to ten blocks supported by Slack containers. |
| `.title(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text title. Mutually exclusive with `richTextTitle`. |
| `.richTextTitle(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Rich-text title block. Mutually exclusive with `title`. |
| `.subtitle(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional supporting copy below the title. |
| `.width(value)` | <code><a href="/reference/typescript/blocks#containerwidth">ContainerWidth</a></code> | No | Container width. Defaults to `standard`. |
| `.icon(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional image displayed in the header. |
| `.isCollapsible(value)` | <code>boolean</code> | No | Whether readers can expand and collapse the container. |
| `.defaultCollapsed(value)` | <code>boolean</code> | No | Whether a collapsible container starts collapsed. |
| `.hasHeaderDivider(value)` | <code>boolean</code> | No | Whether Slack draws a divider below the header. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `childBlocks`: [`JsonObject`](utilities.md#jsonobject)[];
  `defaultCollapsed?`: `boolean`;
  `hasHeaderDivider?`: `boolean`;
  `icon?`: [`JsonObject`](utilities.md#jsonobject);
  `isCollapsible?`: `boolean`;
  `richTextTitle?`: [`JsonObject`](utilities.md#jsonobject);
  `subtitle?`: [`TextLike`](objects.md#textlike);
  `title?`: [`TextLike`](objects.md#textlike);
  `width?`: [`ContainerWidth`](#containerwidth);
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"container"`&gt;&gt;

***

## ContainerWidth

```ts
type ContainerWidth = "narrow" | "standard" | "wide" | "full";
```

Horizontal width used by a container block.

***

## ContextActionsBlock()

```ts
function ContextActionsBlock(): FluentBuilder<{
  blockId?: string;
  elements: JsonObject[];
}, SlackObject<"context_actions">>;
```

Creates a fluent row of up to five contextual controls for feedback or compact
icon actions. Its elements must be built with FeedbackButtons or
IconButton and the block is intended for contextual actions.

See: <https://docs.slack.dev/reference/block-kit/blocks/context-actions-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Feedback-buttons or icon-button elements. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `elements`: [`JsonObject`](utilities.md#jsonobject)[];
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"context_actions"`&gt;&gt;

***

## ContextBlock()

```ts
function ContextBlock(): FluentBuilder<{
  blockId?: string;
  elements: JsonObject[];
}, SlackWire<ContextBlock>>;
```

Creates a fluent block for compact contextual information beneath or beside
primary content. Add up to ten text objects or image elements with
`elements()`.

See: <https://docs.slack.dev/reference/block-kit/blocks/context-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Text objects and image elements. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `elements`: [`JsonObject`](utilities.md#jsonobject)[];
\}, [`SlackWire`](utilities.md#slackwire)&lt;`ContextBlock`&gt;&gt;

***

## DataTableBlock()

```ts
function DataTableBlock(): FluentBuilder<{
  blockId?: string;
  caption: string;
  pageSize?: number;
  rowHeaderColumnIndex?: number;
  rows: JsonObject[][];
}, SlackObject<"data_table">>;
```

Creates a fluent sortable data table containing raw text, raw numbers, or rich
text. The first row supplies the headers and cannot contain rich text; add each
complete row with a separate `rows()` call.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-table-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.rows(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[][]</code> | Yes | Two to 201 equally sized rows containing raw text, raw numbers, or rich text. |
| `.caption(value)` | <code>string</code> | Yes | Accessible table caption. |
| `.pageSize(value)` | <code>number</code> | No | Rows per page, between 1 and 100. Defaults to 5. |
| `.rowHeaderColumnIndex(value)` | <code>number</code> | No | Zero-based column used as the row header. Defaults to 0. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `caption`: `string`;
  `pageSize?`: `number`;
  `rowHeaderColumnIndex?`: `number`;
  `rows`: [`JsonObject`](utilities.md#jsonobject)[][];
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"data_table"`&gt;&gt;

***

## DataVisualizationBlock()

```ts
function DataVisualizationBlock(): FluentBuilder<{
  blockId?: string;
  chart: JsonObject;
  title: string;
}, SlackObject<"data_visualization">>;
```

Creates a fluent data visualization rendered natively by Slack. Set a title
and a pie, bar, area, or line chart built with the corresponding composition
object builder.

See: <https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.title(value)` | <code>string</code> | Yes | Chart heading, up to 50 characters. |
| `.chart(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Object returned by `PieChart()`, `BarChart()`, `AreaChart()`, or `LineChart()`. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `chart`: [`JsonObject`](utilities.md#jsonobject);
  `title`: `string`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"data_visualization"`&gt;&gt;

***

## DividerBlock()

```ts
function DividerBlock(): FluentBuilder<{
  blockId?: string;
}, SlackWire<DividerBlock>>;
```

Creates a visual divider between adjacent blocks, similar to an HTML `<hr>`.
The block has no visible content; its only optional field is a deterministic
block identifier.

See: <https://docs.slack.dev/reference/block-kit/blocks/divider-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
\}, [`SlackWire`](utilities.md#slackwire)&lt;`DividerBlock`&gt;&gt;

***

## FileBlock()

```ts
function FileBlock(): FluentBuilder<{
  blockId?: string;
  externalId: string;
  source?: "remote";
}, SlackWire<FileBlock>>;
```

Creates a block that displays a remote file already registered with Slack.
Supply the external identifier returned by Slack's remote-files API; local or
directly uploaded files cannot be embedded with this block.

See: <https://docs.slack.dev/reference/block-kit/blocks/file-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.externalId(value)` | <code>string</code> | Yes | Identifier assigned when the remote file was added to Slack. |
| `.source(value)` | <code>"remote"</code> | No | Remote-file source. Slack currently accepts only `remote`. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `externalId`: `string`;
  `source?`: `"remote"`;
\}, [`SlackWire`](utilities.md#slackwire)&lt;`FileBlock`&gt;&gt;

***

## HeaderBlock()

```ts
function HeaderBlock(): FluentBuilder<{
  blockId?: string;
  text: TextLike;
}, SlackWire<HeaderBlock>>;
```

Creates a prominent plain-text heading rendered in a larger, bold font. Header
text is limited to 150 characters and Slack does not apply mrkdwn formatting.

See: <https://docs.slack.dev/reference/block-kit/blocks/header-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Heading copy. Strings are converted to plain text. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `text`: [`TextLike`](objects.md#textlike);
\}, [`SlackWire`](utilities.md#slackwire)&lt;`HeaderBlock`&gt;&gt;

***

## ImageBlock()

```ts
function ImageBlock(): FluentBuilder<{
  altText: string;
  blockId?: string;
  imageUrl: string;
  title?: TextLike;
}, SlackWire<ImageBlock>>;
```

Creates a block containing one image with accessible alternative text and an
optional title. Use ImageElement instead when the image must sit inside
a section or context block.

See: <https://docs.slack.dev/reference/block-kit/blocks/image-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.imageUrl(value)` | <code>string</code> | Yes | Public URL of the image. |
| `.altText(value)` | <code>string</code> | Yes | Accessible description of the image. |
| `.title(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional plain-text title. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `altText`: `string`;
  `blockId?`: `string`;
  `imageUrl`: `string`;
  `title?`: [`TextLike`](objects.md#textlike);
\}, [`SlackWire`](utilities.md#slackwire)&lt;`ImageBlock`&gt;&gt;

***

## InputBlock()

```ts
function InputBlock(): FluentBuilder<{
  blockId?: string;
  dispatchAction?: boolean;
  element: JsonObject;
  hint?: TextLike;
  label: TextLike;
  optional?: boolean;
}, SlackWire<InputBlock>>;
```

Creates a labelled form control for collecting information in a modal or App
Home view. Set the required label and one supported input element, then
optionally add a hint, allow omission, or dispatch changes immediately.

See: <https://docs.slack.dev/reference/block-kit/blocks/input-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.label(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text label displayed above the control. |
| `.element(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Input-compatible element such as a text input, picker, or select menu. |
| `.dispatchAction(value)` | <code>boolean</code> | No | Whether changes dispatch an interaction immediately. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |
| `.hint(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional plain-text help shown below the control. |
| `.optional(value)` | <code>boolean</code> | No | Whether the user may submit without completing this input. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `dispatchAction?`: `boolean`;
  `element`: [`JsonObject`](utilities.md#jsonobject);
  `hint?`: [`TextLike`](objects.md#textlike);
  `label`: [`TextLike`](objects.md#textlike);
  `optional?`: `boolean`;
\}, [`SlackWire`](utilities.md#slackwire)&lt;`InputBlock`&gt;&gt;

***

## MarkdownBlock()

```ts
function MarkdownBlock(): FluentBuilder<{
  blockId?: string;
  text: string;
}, SlackWire<MarkdownBlock>>;
```

Creates a block rendered with GitHub-flavored Markdown, including tables and
fenced code blocks. This differs from Slack `mrkdwn` used by section text and
is intended for richer AI or agent-generated output.

See: <https://docs.slack.dev/reference/block-kit/blocks/markdown-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code>string</code> | Yes | GitHub-flavored Markdown source. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `text`: `string`;
\}, [`SlackWire`](utilities.md#slackwire)&lt;`MarkdownBlock`&gt;&gt;

***

## PlanBlock()

```ts
function PlanBlock(): FluentBuilder<{
  blockId?: string;
  tasks?: JsonObject[];
  title: string;
}, SlackObject<"plan">>;
```

Creates a fluent titled sequence of task cards. Add tasks with `tasks()` using
built task cards, [TaskCardBlock](#taskcardblock) builders, or arrays containing either
form.

See: <https://docs.slack.dev/reference/block-kit/blocks/plan-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.title(value)` | <code>string</code> | Yes | Human-readable plan title. |
| `.tasks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Task-card blocks. Their outer `type` and `block_id` fields are omitted in the plan. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `tasks?`: [`JsonObject`](utilities.md#jsonobject)[];
  `title`: `string`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"plan"`&gt;&gt;

***

## RichTextBlock()

```ts
function RichTextBlock(): FluentBuilder<{
  blockId?: string;
  elements: JsonObject[];
}, SlackWire<RichTextBlock>>;
```

Creates a rich-text block from Slack's structured rich-text sections, lists,
code blocks, and quotes. Use it when text needs formatting or nesting that is
unavailable through ordinary section `mrkdwn`.

See: <https://docs.slack.dev/reference/block-kit/blocks/rich-text-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.elements(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Rich-text layout objects. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `elements`: [`JsonObject`](utilities.md#jsonobject)[];
\}, [`SlackWire`](utilities.md#slackwire)&lt;`RichTextBlock`&gt;&gt;

***

## SectionBlock()

```ts
function SectionBlock(): FluentBuilder<SectionBlockInput, SlackWire<SectionBlock>>;
```

Creates one of Block Kit's most flexible blocks: a section can show main text,
arrange short fields into columns, and display an interactive or visual
accessory beside the content. Set at least `text()` or `fields()`.

See: <https://docs.slack.dev/reference/block-kit/blocks/section-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Main copy. Strings are converted to mrkdwn text. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |
| `.fields(...values)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a>[]</code> | No | Up to ten text fields displayed in columns. |
| `.accessory(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional interactive or visual element displayed beside the text. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`SectionBlockInput`](#sectionblockinput), [`SlackWire`](utilities.md#slackwire)&lt;`SectionBlock`&gt;&gt;

***

## SectionBlockInput

Text, field, accessory, and identity fields configured by `SectionBlock()`.
Every section must contain main text, one or more fields, or both.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-accessory"></a> `accessory?` | [`JsonObject`](utilities.md#jsonobject) | Optional interactive or visual element displayed beside the text. |
| <a id="property-blockid-1"></a> `blockId?` | `string` | Deterministic identifier, up to 255 characters. |
| <a id="property-fields"></a> `fields?` | [`TextLike`](objects.md#textlike)[] | Up to ten text fields displayed in columns. |
| <a id="property-text"></a> `text?` | [`TextLike`](objects.md#textlike) | Main copy. Strings are converted to mrkdwn text. |

***

## TableBlock()

```ts
function TableBlock(): FluentBuilder<{
  blockId?: string;
  columnSettings?: JsonObject[];
  rows: JsonObject[][];
}, SlackWire<TableBlock>>;
```

Creates a table block for structured rows and optional column display settings.
Add each complete row with a separate `rows()` call so nested cell arrays retain
their row boundaries.

See: <https://docs.slack.dev/reference/block-kit/blocks/table-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.rows(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[][]</code> | Yes | Up to 100 equally sized rows of raw-text or rich-text cells. |
| `.columnSettings(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Optional display settings for each column. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `columnSettings?`: [`JsonObject`](utilities.md#jsonobject)[];
  `rows`: [`JsonObject`](utilities.md#jsonobject)[][];
\}, [`SlackWire`](utilities.md#slackwire)&lt;`TableBlock`&gt;&gt;

***

## TaskCardBlock()

```ts
function TaskCardBlock(): FluentBuilder<{
  blockId?: string;
  details?: JsonObject;
  output?: JsonObject;
  sources?: JsonObject[];
  status?: TaskStatus;
  taskId: string;
  title: string;
}, SlackObject<"task_card">>;
```

Creates a fluent task card containing a stable identifier, title, lifecycle
state, rich-text details or output, and source links. Task cards may stand
alone or be collected in a [PlanBlock](#planblock).

See: <https://docs.slack.dev/reference/block-kit/blocks/task-card-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.taskId(value)` | <code>string</code> | Yes | Stable task identifier. |
| `.title(value)` | <code>string</code> | Yes | Human-readable task title. |
| `.details(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional rich-text task details. |
| `.output(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional rich-text task output. |
| `.sources(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Optional source links created with `UrlSource()`. |
| `.status(value)` | <code><a href="/reference/typescript/blocks#taskstatus">TaskStatus</a></code> | No | Current task lifecycle state. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `blockId?`: `string`;
  `details?`: [`JsonObject`](utilities.md#jsonobject);
  `output?`: [`JsonObject`](utilities.md#jsonobject);
  `sources?`: [`JsonObject`](utilities.md#jsonobject)[];
  `status?`: [`TaskStatus`](#taskstatus);
  `taskId`: `string`;
  `title`: `string`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"task_card"`&gt;&gt;

***

## TaskStatus

```ts
type TaskStatus = "pending" | "in_progress" | "complete" | "error";
```

Lifecycle state shown by a task card.

***

## VideoBlock()

```ts
function VideoBlock(): FluentBuilder<{
  altText: string;
  authorName?: string;
  blockId?: string;
  description?: TextLike;
  providerIconUrl?: string;
  providerName?: string;
  thumbnailUrl: string;
  title: TextLike;
  titleUrl?: string;
  videoUrl: string;
}, SlackWire<VideoBlock>>;
```

Creates a block that embeds video content in a message, modal, or App Home tab.
Slack enforces its own provider allow-list when accepting the payload, so an
unsupported video URL can still produce a Slack API error after local validation.

See: <https://docs.slack.dev/reference/block-kit/blocks/video-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.altText(value)` | <code>string</code> | Yes | Accessible summary, up to 200 characters. |
| `.thumbnailUrl(value)` | <code>string</code> | Yes | Public preview-image URL. |
| `.title(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text video title, up to 200 characters. |
| `.videoUrl(value)` | <code>string</code> | Yes | URL of a video hosted by a Slack-supported provider. |
| `.blockId(value)` | <code>string</code> | No | Deterministic identifier, up to 255 characters. |
| `.authorName(value)` | <code>string</code> | No | Optional author name, up to 50 characters. |
| `.description(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional plain-text description, up to 200 characters. |
| `.providerIconUrl(value)` | <code>string</code> | No | Optional provider icon URL. |
| `.providerName(value)` | <code>string</code> | No | Optional provider name, up to 50 characters. |
| `.titleUrl(value)` | <code>string</code> | No | Optional destination when the title is selected. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `altText`: `string`;
  `authorName?`: `string`;
  `blockId?`: `string`;
  `description?`: [`TextLike`](objects.md#textlike);
  `providerIconUrl?`: `string`;
  `providerName?`: `string`;
  `thumbnailUrl`: `string`;
  `title`: [`TextLike`](objects.md#textlike);
  `titleUrl?`: `string`;
  `videoUrl`: `string`;
\}, [`SlackWire`](utilities.md#slackwire)&lt;`VideoBlock`&gt;&gt;
