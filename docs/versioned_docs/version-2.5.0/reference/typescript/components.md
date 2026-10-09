---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Components

Higher-level fluent components assembled from ordinary Block Kit blocks.
Components keep their output transparent: `.build()` returns standard Slack
wire objects that can be inspected, rearranged, or mixed with other builders.

## Accordion()

```ts
function Accordion(): FluentGroupBuilder<AccordionInput, JsonObject>;
```

Starts an accordion made from Slack-native collapsible containers.

Each section expands independently in Slack and requires no application-side
interaction handler.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.sections(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Sections created with {@link AccordionSection}, in display order. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentGroupBuilder`](utilities.md#fluentgroupbuilder)&lt;[`AccordionInput`](#accordioninput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## AccordionInput

Configuration accepted by [Accordion](#accordion).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-sections"></a> `sections` | [`JsonObject`](utilities.md#jsonobject)[] | Sections created with [AccordionSection](#accordionsection), in display order. |

***

## AccordionSection()

```ts
function AccordionSection(): FluentBuilder<AccordionSectionInput, JsonObject>;
```

Creates one Slack-native collapsible container for an [Accordion](#accordion). Set a
heading and child blocks, then optionally add supporting copy, an icon, width,
divider behavior, or an initially expanded state.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.title(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text heading shown above the collapsible content. |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Blocks revealed when the section is expanded. |
| `.subtitle(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Optional supporting copy below the heading. |
| `.icon(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional image displayed in the header. |
| `.expanded(value)` | <code>boolean</code> | No | Whether this section starts expanded. Defaults to `false`. |
| `.width(value)` | <code><a href="/reference/typescript/blocks#containerwidth">ContainerWidth</a></code> | No | Horizontal width of this section. Defaults to `standard`. |
| `.hasHeaderDivider(value)` | <code>boolean</code> | No | Whether Slack draws a divider below the header. |
| `.blockId(value)` | <code>string</code> | No | Optional deterministic identifier for this section. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`AccordionSectionInput`](#accordionsectioninput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## AccordionSectionInput

Configuration accepted by [AccordionSection](#accordionsection).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-blockid"></a> `blockId?` | `string` | Optional deterministic identifier for this section. |
| <a id="property-blocks"></a> `blocks` | [`JsonObject`](utilities.md#jsonobject)[] | Blocks revealed when the section is expanded. |
| <a id="property-expanded"></a> `expanded?` | `boolean` | Whether this section starts expanded. Defaults to `false`. |
| <a id="property-hasheaderdivider"></a> `hasHeaderDivider?` | `boolean` | Whether Slack draws a divider below the header. |
| <a id="property-icon"></a> `icon?` | [`JsonObject`](utilities.md#jsonobject) | Optional image displayed in the header. |
| <a id="property-subtitle"></a> `subtitle?` | [`TextLike`](objects.md#textlike) | Optional supporting copy below the heading. |
| <a id="property-title"></a> `title` | [`TextLike`](objects.md#textlike) | Plain-text heading shown above the collapsible content. |
| <a id="property-width"></a> `width?` | [`ContainerWidth`](blocks.md#containerwidth) | Horizontal width of this section. Defaults to `standard`. |

***

## Paginator()

```ts
function Paginator(): FluentGroupBuilder<PaginatorInput, JsonObject>;
```

Starts a pure paginator component.

The component renders the selected page plus ordinary context and action
blocks. Generated button values contain the one-based page to render next;
interaction handling remains in the application.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.blocks(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Blocks to paginate, in display order. |
| `.actionIdPrefix(value)` | <code>string</code> | Yes | Prefix used for the generated previous and next action identifiers. |
| `.page(value)` | <code>number</code> | No | One-based page to render. Defaults to 1. |
| `.pageSize(value)` | <code>number</code> | No | Blocks displayed per page. Defaults to 5. |
| `.previousText(value)` | <code>string</code> | No | Label for the previous-page button. Defaults to `Previous`. |
| `.nextText(value)` | <code>string</code> | No | Label for the next-page button. Defaults to `Next`. |
| `.showPageIndicator(value)` | <code>boolean</code> | No | Whether to render `Page n of m` above the controls. Defaults to `true`. |
| `.blockId(value)` | <code>string</code> | No | Optional identifier for the generated actions block. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentGroupBuilder`](utilities.md#fluentgroupbuilder)&lt;[`PaginatorInput`](#paginatorinput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## PaginatorInput

Configuration accepted by [Paginator](#paginator).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionidprefix"></a> `actionIdPrefix` | `string` | Prefix used for the generated previous and next action identifiers. |
| <a id="property-blockid-1"></a> `blockId?` | `string` | Optional identifier for the generated actions block. |
| <a id="property-blocks-1"></a> `blocks` | [`JsonObject`](utilities.md#jsonobject)[] | Blocks to paginate, in display order. |
| <a id="property-nexttext"></a> `nextText?` | `string` | Label for the next-page button. Defaults to `Next`. |
| <a id="property-page"></a> `page?` | `number` | One-based page to render. Defaults to 1. |
| <a id="property-pagesize"></a> `pageSize?` | `number` | Blocks displayed per page. Defaults to 5. |
| <a id="property-previoustext"></a> `previousText?` | `string` | Label for the previous-page button. Defaults to `Previous`. |
| <a id="property-showpageindicator"></a> `showPageIndicator?` | `boolean` | Whether to render `Page n of m` above the controls. Defaults to `true`. |
