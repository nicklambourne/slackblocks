---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Elements

Fluent builders for interactive and visual elements placed inside Block Kit
blocks. These controls collect input, trigger actions, display images, and let
users choose from static, workspace, or application-provided data.

See: <https://docs.slack.dev/reference/block-kit/block-elements>.

## Button()

```ts
function Button(): FluentBuilder<ButtonInput, SlackObject<"button">>;
```

Creates a fluent interactive button that can submit an action, open a URL, or
carry an application-defined value. Slack returns `actionId` and `value` in the
interaction payload when the user selects it.

See: <https://docs.slack.dev/reference/block-kit/block-elements/button-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text label displayed on the button. |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the button is selected. |
| `.url(value)` | <code>string</code> | No | Optional URL opened by the button. |
| `.value(value)` | <code>string</code> | No | Optional application-defined value returned with the interaction. |
| `.style(value)` | <code>"primary" &#124; "danger"</code> | No | Optional visual emphasis. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog created with `Confirmation()`. |
| `.accessibilityLabel(value)` | <code>string</code> | No | Accessible label when the visible text is insufficient. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ButtonInput`](#buttoninput), [`SlackObject`](utilities.md#slackobject)&lt;`"button"`&gt;&gt;

***

## ButtonInput

Configuration for an interactive button, including its label, action identifier,
behavior, confirmation step, and accessibility text.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-accessibilitylabel"></a> `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| <a id="property-actionid"></a> `actionId` | `string` | Identifier returned when the button is selected. |
| <a id="property-confirm"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog created with `confirmation`. |
| <a id="property-style"></a> `style?` | `"primary"` \| `"danger"` | Optional visual emphasis. |
| <a id="property-text"></a> `text` | [`TextLike`](objects.md#textlike) | Plain-text label displayed on the button. |
| <a id="property-url"></a> `url?` | `string` | Optional URL opened by the button. |
| <a id="property-value"></a> `value?` | `string` | Optional application-defined value returned with the interaction. |

***

## ChannelMultiSelect()

```ts
function ChannelMultiSelect(): FluentBuilder<ChannelMultiSelectInput, SlackObject<"multi_channels_select">>;
```

Creates a multi-select populated with public channels visible to the current
user. It can preselect channel IDs, limit the number selected, and show a
confirmation dialog before submitting the change.

See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the selection changes, up to 255 characters. |
| `.initialChannels(...values)` | <code>string[]</code> | No | Public channel IDs selected when the menu first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.maxSelectedItems(value)` | <code>number</code> | No | Maximum number of channels that may be selected; the minimum is one. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ChannelMultiSelectInput`](#channelmultiselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"multi_channels_select"`&gt;&gt;

***

## ChannelMultiSelectInput

Configuration for a public-channel multi-select, including initial channels, selection
limit, confirmation step, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-1"></a> `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| <a id="property-confirm-1"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialchannels"></a> `initialChannels?` | `string`[] | Public channel IDs selected when the menu first loads. |
| <a id="property-maxselecteditems"></a> `maxSelectedItems?` | `number` | Maximum number of channels that may be selected; the minimum is one. |
| <a id="property-placeholder"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## ChannelSelect()

```ts
function ChannelSelect(): FluentBuilder<ChannelSelectInput, SlackObject<"channels_select">>;
```

Creates a single-select populated with public channels visible to the current
user. It may start with one channel selected and can expose a response URL when
used inside a modal.

See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#channels_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when a channel is selected, up to 255 characters. |
| `.initialChannel(value)` | <code>string</code> | No | Public channel ID selected when the menu first loads. |
| `.responseUrlEnabled(value)` | <code>boolean</code> | No | Include a `response_url` in a parent modal's submission payload. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ChannelSelectInput`](#channelselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"channels_select"`&gt;&gt;

***

## ChannelSelectInput

Configuration for a public-channel single-select, including its initial channel, modal
response URL, confirmation step, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-2"></a> `actionId` | `string` | Identifier returned when a channel is selected, up to 255 characters. |
| <a id="property-confirm-2"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-1"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialchannel"></a> `initialChannel?` | `string` | Public channel ID selected when the menu first loads. |
| <a id="property-placeholder-1"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |
| <a id="property-responseurlenabled"></a> `responseUrlEnabled?` | `boolean` | Include a `response_url` in a parent modal's submission payload. |

***

## Checkboxes()

```ts
function Checkboxes(): FluentBuilder<CheckboxesInput, SlackObject<"checkboxes">>;
```

Creates a checkbox group that lets a user choose multiple items from a list of
up to ten options. Initial selections must correspond to options included in
the same element.

See: <https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the checkbox selection changes, up to 255 characters. |
| `.options(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Up to ten option objects displayed as checkboxes. |
| `.initialOptions(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Options from `options` that are selected when the element first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the changed selection is submitted. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`CheckboxesInput`](#checkboxesinput), [`SlackObject`](utilities.md#slackobject)&lt;`"checkboxes"`&gt;&gt;

***

## CheckboxesInput

Configuration for a checkbox group, including its choices, initial selection,
confirmation step, and focus behavior.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-3"></a> `actionId` | `string` | Identifier returned when the checkbox selection changes, up to 255 characters. |
| <a id="property-confirm-3"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the changed selection is submitted. |
| <a id="property-focusonload-2"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialoptions"></a> `initialOptions?` | [`JsonObject`](utilities.md#jsonobject)[] | Options from `options` that are selected when the element first loads. |
| <a id="property-options"></a> `options` | [`JsonObject`](utilities.md#jsonobject)[] | Up to ten option objects displayed as checkboxes. |

***

## ConversationMultiSelect()

```ts
function ConversationMultiSelect(): FluentBuilder<ConversationMultiSelectInput, SlackObject<"multi_conversations_select">>;
```

Creates a multi-select populated with conversations visible to the current
user, including the conversation from which a view was opened when requested.
Apply a conversation filter to control which channel and DM types appear.

See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the selection changes, up to 255 characters. |
| `.initialConversations(...values)` | <code>string[]</code> | No | Conversation IDs selected when the menu first loads. |
| `.defaultToCurrentConversation(value)` | <code>boolean</code> | No | Select the conversation from which the view was opened by default. |
| `.filter(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Filter controlling which public channels, private channels, DMs, and group DMs appear. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.maxSelectedItems(value)` | <code>number</code> | No | Maximum number of conversations that may be selected; the minimum is one. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ConversationMultiSelectInput`](#conversationmultiselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"multi_conversations_select"`&gt;&gt;

***

## ConversationMultiSelectInput

Configuration for a conversation multi-select, including initial conversations,
filtering, selection limits, and modal-aware defaults.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-4"></a> `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| <a id="property-confirm-4"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-defaulttocurrentconversation"></a> `defaultToCurrentConversation?` | `boolean` | Select the conversation from which the view was opened by default. |
| <a id="property-filter"></a> `filter?` | [`JsonObject`](utilities.md#jsonobject) | Filter controlling which public channels, private channels, DMs, and group DMs appear. |
| <a id="property-focusonload-3"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialconversations"></a> `initialConversations?` | `string`[] | Conversation IDs selected when the menu first loads. |
| <a id="property-maxselecteditems-1"></a> `maxSelectedItems?` | `number` | Maximum number of conversations that may be selected; the minimum is one. |
| <a id="property-placeholder-2"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## ConversationSelect()

```ts
function ConversationSelect(): FluentBuilder<ConversationSelectInput, SlackObject<"conversations_select">>;
```

Creates a single-select populated with visible public channels, private
channels, direct messages, and group DMs. Apply a conversation filter to limit
the available conversation types.

See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when a conversation is selected, up to 255 characters. |
| `.initialConversation(value)` | <code>string</code> | No | Conversation ID selected when the menu first loads. |
| `.defaultToCurrentConversation(value)` | <code>boolean</code> | No | Select the conversation from which the view was opened by default. |
| `.filter(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Filter controlling which public channels, private channels, DMs, and group DMs appear. |
| `.responseUrlEnabled(value)` | <code>boolean</code> | No | Include a `response_url` in a parent modal's submission payload. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ConversationSelectInput`](#conversationselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"conversations_select"`&gt;&gt;

***

## ConversationSelectInput

Configuration for a conversation single-select, including filtering, modal-aware
defaults, response URL behavior, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-5"></a> `actionId` | `string` | Identifier returned when a conversation is selected, up to 255 characters. |
| <a id="property-confirm-5"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-defaulttocurrentconversation-1"></a> `defaultToCurrentConversation?` | `boolean` | Select the conversation from which the view was opened by default. |
| <a id="property-filter-1"></a> `filter?` | [`JsonObject`](utilities.md#jsonobject) | Filter controlling which public channels, private channels, DMs, and group DMs appear. |
| <a id="property-focusonload-4"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialconversation"></a> `initialConversation?` | `string` | Conversation ID selected when the menu first loads. |
| <a id="property-placeholder-3"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |
| <a id="property-responseurlenabled-1"></a> `responseUrlEnabled?` | `boolean` | Include a `response_url` in a parent modal's submission payload. |

***

## DatePicker()

```ts
function DatePicker(): FluentBuilder<DatePickerInput, SlackObject<"datepicker">>;
```

Creates an interactive calendar control for selecting one date. The optional
initial value uses `YYYY-MM-DD`, and Slack returns the selected date with the
configured action identifier.

See: <https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when a date is selected, up to 255 characters. |
| `.initialDate(value)` | <code>string</code> | No | Initially selected date in `YYYY-MM-DD` format. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown after a date is selected. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a date is selected, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`DatePickerInput`](#datepickerinput), [`SlackObject`](utilities.md#slackobject)&lt;`"datepicker"`&gt;&gt;

***

## DatePickerInput

Configuration for a calendar date picker, including its action identifier, initial ISO
date, prompt, and interaction behavior.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-6"></a> `actionId` | `string` | Identifier returned when a date is selected, up to 255 characters. |
| <a id="property-confirm-6"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a date is selected. |
| <a id="property-focusonload-5"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialdate"></a> `initialDate?` | `string` | Initially selected date in `YYYY-MM-DD` format. |
| <a id="property-placeholder-4"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a date is selected, up to 150 characters. |

***

## DateTimePicker()

```ts
function DateTimePicker(): FluentBuilder<DateTimePickerInput, SlackObject<"datetimepicker">>;
```

Creates an interactive control for selecting both a date and a time of day.
Initial and submitted values are represented as Unix timestamps in seconds.

See: <https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when a date and time are selected, up to 255 characters. |
| `.initialDateTime(value)` | <code>number</code> | No | Initially selected date and time as a Unix timestamp in seconds. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown after a date and time are selected. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`DateTimePickerInput`](#datetimepickerinput), [`SlackObject`](utilities.md#slackobject)&lt;`"datetimepicker"`&gt;&gt;

***

## DateTimePickerInput

Configuration for a combined date-and-time picker whose initial and submitted values use
Unix timestamps in seconds.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-7"></a> `actionId` | `string` | Identifier returned when a date and time are selected, up to 255 characters. |
| <a id="property-confirm-7"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a date and time are selected. |
| <a id="property-focusonload-6"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialdatetime"></a> `initialDateTime?` | `number` | Initially selected date and time as a Unix timestamp in seconds. |

***

## EmailElementInput

Configuration for an email-address input, including its initial value, empty-state
prompt, focus, and dispatch behavior.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-8"></a> `actionId` | `string` | Identifier used to find the submitted email value, up to 255 characters. |
| <a id="property-dispatchactionconfig"></a> `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| <a id="property-focusonload-7"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialvalue"></a> `initialValue?` | `string` | Email address present when the input first loads. |
| <a id="property-placeholder-5"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

***

## EmailInput()

```ts
function EmailInput(): FluentBuilder<EmailElementInput, SlackObject<"email_text_input">>;
```

Creates a single-line input specialized for email addresses. It can start with
an existing value and optionally dispatch interaction payloads while the user
edits the field.

See: <https://docs.slack.dev/reference/block-kit/block-elements/email-input-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier used to find the submitted email value, up to 255 characters. |
| `.initialValue(value)` | <code>string</code> | No | Email address present when the input first loads. |
| `.dispatchActionConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Configuration controlling when typing dispatches a `block_actions` payload. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown in the empty input, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`EmailElementInput`](#emailelementinput), [`SlackObject`](utilities.md#slackobject)&lt;`"email_text_input"`&gt;&gt;

***

## ExternalMultiSelect()

```ts
function ExternalMultiSelect(): FluentBuilder<ExternalMultiSelectInput, SlackObject<"multi_external_select">>;
```

Creates a dynamic multi-select whose options are supplied by your application.
Slack requests matching options after the user types the configured minimum
number of characters.

See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the selection changes, up to 255 characters. |
| `.minQueryLength(value)` | <code>number</code> | No | Minimum typed characters before Slack requests options; defaults to three. |
| `.initialOptions(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Options selected when the menu first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.maxSelectedItems(value)` | <code>number</code> | No | Maximum number of options that may be selected; the minimum is one. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ExternalMultiSelectInput`](#externalmultiselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"multi_external_select"`&gt;&gt;

***

## ExternalMultiSelectInput

Configuration for an externally populated multi-select, including query threshold,
initial options, selection limit, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-9"></a> `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| <a id="property-confirm-8"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-8"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialoptions-1"></a> `initialOptions?` | [`JsonObject`](utilities.md#jsonobject)[] | Options selected when the menu first loads. |
| <a id="property-maxselecteditems-2"></a> `maxSelectedItems?` | `number` | Maximum number of options that may be selected; the minimum is one. |
| <a id="property-minquerylength"></a> `minQueryLength?` | `number` | Minimum typed characters before Slack requests options; defaults to three. |
| <a id="property-placeholder-6"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## ExternalSelect()

```ts
function ExternalSelect(): FluentBuilder<ExternalSelectInput, SlackObject<"external_select">>;
```

Creates a dynamic single-select whose options are supplied by your application.
Slack requests matching options after the user types the configured minimum
number of characters.

See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when an option is selected, up to 255 characters. |
| `.minQueryLength(value)` | <code>number</code> | No | Minimum typed characters before Slack requests options; defaults to three. |
| `.initialOption(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Option selected when the menu first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`ExternalSelectInput`](#externalselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"external_select"`&gt;&gt;

***

## ExternalSelectInput

Configuration for an externally populated single-select, including query threshold,
initial option, confirmation step, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-10"></a> `actionId` | `string` | Identifier returned when an option is selected, up to 255 characters. |
| <a id="property-confirm-9"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-9"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialoption"></a> `initialOption?` | [`JsonObject`](utilities.md#jsonobject) | Option selected when the menu first loads. |
| <a id="property-minquerylength-1"></a> `minQueryLength?` | `number` | Minimum typed characters before Slack requests options; defaults to three. |
| <a id="property-placeholder-7"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## FeedbackButton()

```ts
function FeedbackButton(): FluentBuilder<FeedbackButtonInput, JsonObject>;
```

Creates one labelled positive or negative choice for a [FeedbackButtons](#feedbackbuttons)
control. The choice's value is returned to the application when the user gives
feedback.

See: <https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text feedback choice. |
| `.value(value)` | <code>string</code> | Yes | Application-defined value returned with the feedback. |
| `.accessibilityLabel(value)` | <code>string</code> | No | Accessible label when the visible text is insufficient. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`FeedbackButtonInput`](#feedbackbuttoninput), [`JsonObject`](utilities.md#jsonobject)&gt;

***

## FeedbackButtonInput

Configuration for one positive or negative feedback choice, including its visible text,
returned value, and accessibility label.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-accessibilitylabel-1"></a> `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| <a id="property-text-1"></a> `text` | [`TextLike`](objects.md#textlike) | Plain-text feedback choice. |
| <a id="property-value-1"></a> `value` | `string` | Application-defined value returned with the feedback. |

***

## FeedbackButtons()

```ts
function FeedbackButtons(): FluentBuilder<{
  actionId?: string;
  negativeButton: JsonObject;
  positiveButton: JsonObject;
}, SlackObject<"feedback_buttons">>;
```

Creates a paired positive and negative feedback control for a context-actions
block. Build each choice with [FeedbackButton](#feedbackbutton) so its visible text,
returned value, and accessibility label are validated.

See: <https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.positiveButton(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Positive choice created with `FeedbackButton()`. |
| `.negativeButton(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Negative choice created with `FeedbackButton()`. |
| `.actionId(value)` | <code>string</code> | No | Optional identifier returned with the interaction. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `actionId?`: `string`;
  `negativeButton`: [`JsonObject`](utilities.md#jsonobject);
  `positiveButton`: [`JsonObject`](utilities.md#jsonobject);
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"feedback_buttons"`&gt;&gt;

***

## FileInput()

```ts
function FileInput(): FluentBuilder<{
  actionId: string;
  filetypes?: string[];
  maxFiles?: number;
}, SlackObject<"file_input">>;
```

Creates an interactive input that lets users upload files to Slack. Restrict
accepted formats with `filetypes()` and control the permitted number of files
with `maxFiles()`.

See: <https://docs.slack.dev/reference/block-kit/block-elements/file-input-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned with submitted files. |
| `.filetypes(...values)` | <code>string[]</code> | No | Optional allowed file extensions. |
| `.maxFiles(value)` | <code>number</code> | No | Maximum files accepted, between 1 and 10. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `actionId`: `string`;
  `filetypes?`: `string`[];
  `maxFiles?`: `number`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"file_input"`&gt;&gt;

***

## IconButton()

```ts
function IconButton(): FluentBuilder<{
  accessibilityLabel?: string;
  actionId?: string;
  confirm?: JsonObject;
  icon?: "trash";
  text: TextLike;
  value?: string;
  visibleToUserIds?: string[];
}, SlackObject<"icon_button">>;
```

Creates a compact icon-only action for a context-actions block. Slack currently
supports the `trash` icon, and the control can optionally be restricted to a
list of up to ten visible users.

See: <https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text description of the icon action. |
| `.icon(value)` | <code>"trash"</code> | No | Icon name. Slack currently accepts only `trash`. |
| `.actionId(value)` | <code>string</code> | No | Optional identifier returned with the interaction. |
| `.value(value)` | <code>string</code> | No | Optional application-defined interaction value. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog. |
| `.accessibilityLabel(value)` | <code>string</code> | No | Accessible label when the text is insufficient. |
| `.visibleToUserIds(...values)` | <code>string[]</code> | No | Up to ten user IDs allowed to see the action. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `accessibilityLabel?`: `string`;
  `actionId?`: `string`;
  `confirm?`: [`JsonObject`](utilities.md#jsonobject);
  `icon?`: `"trash"`;
  `text`: [`TextLike`](objects.md#textlike);
  `value?`: `string`;
  `visibleToUserIds?`: `string`[];
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"icon_button"`&gt;&gt;

***

## ImageElement()

```ts
function ImageElement(): FluentBuilder<ImageElementBuilderInput, SlackObject<"image">>;
```

Creates an image element for use inside section and context blocks. Supply
accessible alternative text and exactly one public image URL or Slack-hosted
file reference; use `ImageBlock()` for a standalone image.

See: <https://docs.slack.dev/reference/block-kit/block-elements/image-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.altText(value)` | <code>string</code> | Yes | Alternative text for screen readers and unavailable images. |
| `.imageUrl(value)` | <code>string</code> | No | Public URL of the image. Mutually exclusive with `SlackFile()`. |
| `.slackFile(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Slack-hosted file object. Mutually exclusive with `imageUrl`. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;`ImageElementBuilderInput`, [`SlackObject`](utilities.md#slackobject)&lt;`"image"`&gt;&gt;

***

## ImageElementInput

```ts
type ImageElementInput =
  | {
  altText: string;
  imageUrl: string;
  slackFile?: never;
}
  | {
  altText: string;
  imageUrl?: never;
  slackFile: JsonObject;
};
```

Configuration for an image element. Accessible alternative text is always
required, together with exactly one public image URL or Slack-hosted file.

### Union Members

#### Type Literal

```ts
{
  altText: string;
  imageUrl: string;
  slackFile?: never;
}
```

| Name | Type | Description |
| ------ | ------ | ------ |
| `altText` | `string` | Accessible plain-text summary of the image. |
| `imageUrl` | `string` | Public image URL, up to 3,000 characters. |
| `slackFile?` | `never` | A Slack file cannot be combined with `imageUrl`. |

***

#### Type Literal

```ts
{
  altText: string;
  imageUrl?: never;
  slackFile: JsonObject;
}
```

| Name | Type | Description |
| ------ | ------ | ------ |
| `altText` | `string` | Accessible plain-text summary of the image. |
| `imageUrl?` | `never` | An image URL cannot be combined with `slackFile`. |
| `slackFile` | [`JsonObject`](utilities.md#jsonobject) | Slack-hosted file reference created with `slackFile`. |

***

## NumberElementInput

Configuration for a numeric input, including decimal support, initial value, permitted
range, prompt, and dispatch behavior.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-11"></a> `actionId` | `string` | Identifier used to find the submitted numeric value, up to 255 characters. |
| <a id="property-dispatchactionconfig-1"></a> `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| <a id="property-focusonload-10"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialvalue-1"></a> `initialValue?` | `string` | Numeric text present when the input first loads. |
| <a id="property-isdecimalallowed"></a> `isDecimalAllowed?` | `boolean` | Whether the input accepts decimal values as well as whole numbers. |
| <a id="property-maxvalue"></a> `maxValue?` | `number` | Maximum accepted value; it cannot be less than `minValue`. |
| <a id="property-minvalue"></a> `minValue?` | `number` | Minimum accepted value; it cannot exceed `maxValue`. |
| <a id="property-placeholder-8"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

***

## NumberInput()

```ts
function NumberInput(): FluentBuilder<NumberElementInput, SlackObject<"number_input">>;
```

Creates an input that accepts whole numbers and, when enabled, decimal values
such as `0.25`, `5.5`, or `-10`. Optional minimum and maximum values constrain
what the user may submit.

See: <https://docs.slack.dev/reference/block-kit/block-elements/number-input-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier used to find the submitted numeric value, up to 255 characters. |
| `.isDecimalAllowed(value)` | <code>boolean</code> | No | Whether the input accepts decimal values as well as whole numbers. |
| `.initialValue(value)` | <code>string</code> | No | Numeric text present when the input first loads. |
| `.minValue(value)` | <code>number</code> | No | Minimum accepted value; it cannot exceed `maxValue`. |
| `.maxValue(value)` | <code>number</code> | No | Maximum accepted value; it cannot be less than `minValue`. |
| `.dispatchActionConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Configuration controlling when typing dispatches a `block_actions` payload. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown in the empty input, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`NumberElementInput`](#numberelementinput), [`SlackObject`](utilities.md#slackobject)&lt;`"number_input"`&gt;&gt;

***

## Overflow()

```ts
function Overflow(): FluentBuilder<OverflowInput, SlackObject<"overflow">>;
```

Creates a compact overflow menu, conventionally displayed as an ellipsis, for
secondary actions. Slack requires between two and five options and returns the
selected option with the action identifier.

See: <https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when an option is selected, up to 255 characters. |
| `.options(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Between two and five option objects displayed in the compact menu. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown after an option is selected. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`OverflowInput`](#overflowinput), [`SlackObject`](utilities.md#slackobject)&lt;`"overflow"`&gt;&gt;

***

## OverflowInput

Configuration for a compact overflow menu containing between two and five secondary
action options.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-12"></a> `actionId` | `string` | Identifier returned when an option is selected, up to 255 characters. |
| <a id="property-confirm-10"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after an option is selected. |
| <a id="property-options-1"></a> `options` | [`JsonObject`](utilities.md#jsonobject)[] | Between two and five option objects displayed in the compact menu. |

***

## PlainTextElementInput

Configuration for a free-form text field, including multiline display, initial text,
character bounds, prompt, and dispatch behavior.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-13"></a> `actionId` | `string` | Identifier used to find the submitted text value, up to 255 characters. |
| <a id="property-dispatchactionconfig-2"></a> `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| <a id="property-focusonload-11"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialvalue-2"></a> `initialValue?` | `string` | Text present when the input first loads. |
| <a id="property-maxlength"></a> `maxLength?` | `number` | Maximum number of characters the user may enter, between 1 and 3000. |
| <a id="property-minlength"></a> `minLength?` | `number` | Minimum number of characters the user must enter, between 0 and 3000. |
| <a id="property-multiline"></a> `multiline?` | `boolean` | Whether the input is a multi-line textarea instead of a single line. |
| <a id="property-placeholder-9"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

***

## PlainTextInput()

```ts
function PlainTextInput(): FluentBuilder<PlainTextElementInput, SlackObject<"plain_text_input">>;
```

Creates a free-form plain-text field similar to an HTML `<input>` or textarea.
Configure single-line or multiline display, initial text, character limits,
and when editing should dispatch an interaction payload.

See: <https://docs.slack.dev/reference/block-kit/block-elements/plain-text-input-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier used to find the submitted text value, up to 255 characters. |
| `.initialValue(value)` | <code>string</code> | No | Text present when the input first loads. |
| `.multiline(value)` | <code>boolean</code> | No | Whether the input is a multi-line textarea instead of a single line. |
| `.minLength(value)` | <code>number</code> | No | Minimum number of characters the user must enter, between 0 and 3000. |
| `.maxLength(value)` | <code>number</code> | No | Maximum number of characters the user may enter, between 1 and 3000. |
| `.dispatchActionConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Configuration controlling when typing dispatches a `block_actions` payload. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown in the empty input, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`PlainTextElementInput`](#plaintextelementinput), [`SlackObject`](utilities.md#slackobject)&lt;`"plain_text_input"`&gt;&gt;

***

## RadioButtons()

```ts
function RadioButtons(): FluentBuilder<RadioButtonsInput, SlackObject<"radio_buttons">>;
```

Creates a radio-button group that lets a user choose exactly one item from up
to ten options. An initial option may be selected before the element is shown.

See: <https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the selection changes, up to 255 characters. |
| `.options(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | Yes | Up to ten options displayed as radio buttons. |
| `.initialOption(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Option from `options` selected when the element first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`RadioButtonsInput`](#radiobuttonsinput), [`SlackObject`](utilities.md#slackobject)&lt;`"radio_buttons"`&gt;&gt;

***

## RadioButtonsInput

Configuration for a single-choice radio-button group, including its options, initial
choice, confirmation step, and focus behavior.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-14"></a> `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| <a id="property-confirm-11"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-12"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialoption-1"></a> `initialOption?` | [`JsonObject`](utilities.md#jsonobject) | Option from `options` selected when the element first loads. |
| <a id="property-options-2"></a> `options` | [`JsonObject`](utilities.md#jsonobject)[] | Up to ten options displayed as radio buttons. |

***

## RichTextElementInput

Configuration for a WYSIWYG rich-text editor, including initial content, prompt,
dispatch behavior, and visible line bounds.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-15"></a> `actionId` | `string` | Identifier used to find the submitted rich-text value, up to 255 characters. |
| <a id="property-dispatchactionconfig-3"></a> `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when editing dispatches a `block_actions` payload. |
| <a id="property-focusonload-13"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialvalue-3"></a> `initialValue?` | [`JsonObject`](utilities.md#jsonobject) | Rich-text content present when the editor first loads. |
| <a id="property-maxlines"></a> `maxLines?` | `number` | Maximum visible editor lines before scrolling, between 1 and 100. |
| <a id="property-minlines"></a> `minLines?` | `number` | Minimum visible editor lines before scrolling, between 1 and 100. |
| <a id="property-placeholder-10"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty editor, up to 150 characters. |

***

## RichTextInput()

```ts
function RichTextInput(): FluentBuilder<RichTextElementInput, SlackObject<"rich_text_input">>;
```

Creates a WYSIWYG rich-text editor similar to Slack's message composer. It can
start with structured rich text, dispatch changes as the user edits, and limit
the editor's visible height with minimum and maximum line counts.

See: <https://docs.slack.dev/reference/block-kit/block-elements/rich-text-input-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier used to find the submitted rich-text value, up to 255 characters. |
| `.initialValue(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Rich-text content present when the editor first loads. |
| `.dispatchActionConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Configuration controlling when editing dispatches a `block_actions` payload. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown in the empty editor, up to 150 characters. |
| `.minLines(value)` | <code>number</code> | No | Minimum visible editor lines before scrolling, between 1 and 100. |
| `.maxLines(value)` | <code>number</code> | No | Maximum visible editor lines before scrolling, between 1 and 100. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`RichTextElementInput`](#richtextelementinput), [`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_input"`&gt;&gt;

***

## StaticMultiSelect()

```ts
function StaticMultiSelect(): FluentBuilder<StaticMultiSelectInput, SlackObject<"multi_static_select">>;
```

Creates a multi-select from options defined directly in the Block Kit payload.
Supply either individual options or option groups, not both, and optionally
mark matching options as initially selected.

See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the selection changes, up to 255 characters. |
| `.options(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Up to 100 directly supplied options; mutually exclusive with `optionGroups`. |
| `.optionGroups(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Up to 100 groups of options; mutually exclusive with `options`. |
| `.initialOptions(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Options selected when the menu first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.maxSelectedItems(value)` | <code>number</code> | No | Maximum number of options that may be selected; the minimum is one. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`StaticMultiSelectInput`](#staticmultiselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"multi_static_select"`&gt;&gt;

***

## StaticMultiSelectInput

Configuration for a static multi-select. Supply either direct options or option groups,
never both.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-16"></a> `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| <a id="property-confirm-12"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-14"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialoptions-2"></a> `initialOptions?` | [`JsonObject`](utilities.md#jsonobject)[] | Options selected when the menu first loads. |
| <a id="property-maxselecteditems-3"></a> `maxSelectedItems?` | `number` | Maximum number of options that may be selected; the minimum is one. |
| <a id="property-optiongroups"></a> `optionGroups?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 groups of options; mutually exclusive with `options`. |
| <a id="property-options-3"></a> `options?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 directly supplied options; mutually exclusive with `optionGroups`. |
| <a id="property-placeholder-11"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## StaticSelect()

```ts
function StaticSelect(): FluentBuilder<StaticSelectInput, SlackObject<"static_select">>;
```

Creates a single-select from options defined directly in the Block Kit payload.
Supply either individual options or option groups, not both, and optionally set
one matching initial option.

See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when an option is selected, up to 255 characters. |
| `.options(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Up to 100 directly supplied options; mutually exclusive with `optionGroups`. |
| `.optionGroups(...values)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a>[]</code> | No | Up to 100 groups of options; mutually exclusive with `options`. |
| `.initialOption(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Option selected when the menu first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`StaticSelectInput`](#staticselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"static_select"`&gt;&gt;

***

## StaticSelectInput

Configuration for a static single-select. Supply either direct options or option groups,
never both.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-17"></a> `actionId` | `string` | Identifier returned when an option is selected, up to 255 characters. |
| <a id="property-confirm-13"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-15"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialoption-2"></a> `initialOption?` | [`JsonObject`](utilities.md#jsonobject) | Option selected when the menu first loads. |
| <a id="property-optiongroups-1"></a> `optionGroups?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 groups of options; mutually exclusive with `options`. |
| <a id="property-options-4"></a> `options?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 directly supplied options; mutually exclusive with `optionGroups`. |
| <a id="property-placeholder-12"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## TimePicker()

```ts
function TimePicker(): FluentBuilder<TimePickerInput, SlackObject<"timepicker">>;
```

Creates an interactive control for selecting a time of day. Initial values use
24-hour `HH:mm` format, and an optional IANA timezone is displayed as supporting
text and returned with interactions.

See: <https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when a time is selected, up to 255 characters. |
| `.initialTime(value)` | <code>string</code> | No | Initially selected time in 24-hour `HH:mm` format. |
| `.timezone(value)` | <code>string</code> | No | IANA timezone displayed as supporting text and returned with interactions. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown after a time is selected. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a time is selected, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`TimePickerInput`](#timepickerinput), [`SlackObject`](utilities.md#slackobject)&lt;`"timepicker"`&gt;&gt;

***

## TimePickerInput

Configuration for a time-of-day picker, including its initial 24-hour time, timezone,
confirmation step, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-18"></a> `actionId` | `string` | Identifier returned when a time is selected, up to 255 characters. |
| <a id="property-confirm-14"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a time is selected. |
| <a id="property-focusonload-16"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialtime"></a> `initialTime?` | `string` | Initially selected time in 24-hour `HH:mm` format. |
| <a id="property-placeholder-13"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a time is selected, up to 150 characters. |
| <a id="property-timezone"></a> `timezone?` | `string` | IANA timezone displayed as supporting text and returned with interactions. |

***

## UrlElementInput

Configuration for a URL input, including its initial value, empty-state prompt, focus,
and dispatch behavior.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-19"></a> `actionId` | `string` | Identifier used to find the submitted URL, up to 255 characters. |
| <a id="property-dispatchactionconfig-4"></a> `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| <a id="property-focusonload-17"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialvalue-4"></a> `initialValue?` | `string` | URL present when the input first loads. |
| <a id="property-placeholder-14"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

***

## UrlInput()

```ts
function UrlInput(): FluentBuilder<UrlElementInput, SlackObject<"url_text_input">>;
```

Creates a single-line field specialized for collecting a URL. It can start with
an existing value and optionally dispatch interaction payloads as the user
edits the field.

See: <https://docs.slack.dev/reference/block-kit/block-elements/url-input-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier used to find the submitted URL, up to 255 characters. |
| `.initialValue(value)` | <code>string</code> | No | URL present when the input first loads. |
| `.dispatchActionConfig(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Configuration controlling when typing dispatches a `block_actions` payload. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown in the empty input, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`UrlElementInput`](#urlelementinput), [`SlackObject`](utilities.md#slackobject)&lt;`"url_text_input"`&gt;&gt;

***

## UrlSource()

```ts
function UrlSource(): FluentBuilder<{
  text: string;
  url: string;
}, SlackObject<"url">>;
```

Creates a labelled URL source for a task card. Use source links to identify the
external documents, tickets, or other resources from which a task originated.

See: <https://docs.slack.dev/reference/block-kit/blocks/task-card-block>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.url(value)` | <code>string</code> | Yes | Public source URL. |
| `.text(value)` | <code>string</code> | Yes | Human-readable source label. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;\{
  `text`: `string`;
  `url`: `string`;
\}, [`SlackObject`](utilities.md#slackobject)&lt;`"url"`&gt;&gt;

***

## UserMultiSelect()

```ts
function UserMultiSelect(): FluentBuilder<UserMultiSelectInput, SlackObject<"multi_users_select">>;
```

Creates a multi-select populated automatically with workspace users visible to
the current user. It can preselect user IDs and enforce a maximum number of
selections.

See: <https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#user_multi_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.
Collection setters accept individual values, nested builders, or arrays and append each call.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when the selection changes, up to 255 characters. |
| `.initialUsers(...values)` | <code>string[]</code> | No | User IDs selected when the menu first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.maxSelectedItems(value)` | <code>number</code> | No | Maximum number of users that may be selected; the minimum is one. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`UserMultiSelectInput`](#usermultiselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"multi_users_select"`&gt;&gt;

***

## UserMultiSelectInput

Configuration for a workspace-user multi-select, including initial users, selection
limit, confirmation step, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-20"></a> `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| <a id="property-confirm-15"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-18"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialusers"></a> `initialUsers?` | `string`[] | User IDs selected when the menu first loads. |
| <a id="property-maxselecteditems-4"></a> `maxSelectedItems?` | `number` | Maximum number of users that may be selected; the minimum is one. |
| <a id="property-placeholder-15"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## UserSelect()

```ts
function UserSelect(): FluentBuilder<UserSelectInput, SlackObject<"users_select">>;
```

Creates a single-select populated automatically with workspace users visible
to the current user. It can begin with one user ID already selected.

See: <https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.actionId(value)` | <code>string</code> | Yes | Identifier returned when a user is selected, up to 255 characters. |
| `.initialUser(value)` | <code>string</code> | No | User ID selected when the menu first loads. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog shown before the selection is submitted. |
| `.focusOnLoad(value)` | <code>boolean</code> | No | Whether this element receives focus when its containing view opens. |
| `.placeholder(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | No | Plain-text prompt shown before a selection, up to 150 characters. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`UserSelectInput`](#userselectinput), [`SlackObject`](utilities.md#slackobject)&lt;`"users_select"`&gt;&gt;

***

## UserSelectInput

Configuration for a workspace-user single-select, including its initial user,
confirmation step, focus behavior, and prompt.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-21"></a> `actionId` | `string` | Identifier returned when a user is selected, up to 255 characters. |
| <a id="property-confirm-16"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-19"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialuser"></a> `initialUser?` | `string` | User ID selected when the menu first loads. |
| <a id="property-placeholder-16"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## WorkflowButton()

```ts
function WorkflowButton(): FluentBuilder<WorkflowButtonInput, SlackObject<"workflow_button">>;
```

Creates a button that launches a Slack link trigger with optional customizable
inputs. Build the nested workflow, trigger, and input parameters with the
corresponding composition-object builders.

See: <https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element>.

### Chainable setters

Call these setters in any order before `.build()`. Repeating a singular setter replaces its previous value.

| Setter | Value type | Required | Description |
| ------ | ------ | ------ | ------ |
| `.text(value)` | <code><a href="/reference/typescript/objects#textlike">TextLike</a></code> | Yes | Plain-text label displayed on the button. |
| `.workflow(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | Yes | Workflow object created with `Workflow()`. |
| `.actionId(value)` | <code>string</code> | No | Optional interaction identifier. |
| `.confirm(value)` | <code><a href="/reference/typescript/utilities#jsonobject">JsonObject</a></code> | No | Optional confirmation dialog. |
| `.style(value)` | <code>"primary" &#124; "danger"</code> | No | Optional visual emphasis. |
| `.accessibilityLabel(value)` | <code>string</code> | No | Accessible label when the visible text is insufficient. |

### Validation and errors

`.build()` materializes nested builders and validates the finished Slack object. It throws a typed slackblocks validation error when a required value is missing, a value has the wrong type or range, mutually exclusive setters are combined, or Slack rejects the resulting shape. Pass `{ validate: false }` to `.build()` only when intentionally creating an intermediate partial object.

### Returns

[`FluentBuilder`](utilities.md#fluentbuilder)&lt;[`WorkflowButtonInput`](#workflowbuttoninput), [`SlackObject`](utilities.md#slackobject)&lt;`"workflow_button"`&gt;&gt;

***

## WorkflowButtonInput

Configuration for a workflow button that launches a Slack link trigger with optional
interaction and accessibility settings.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-accessibilitylabel-2"></a> `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| <a id="property-actionid-22"></a> `actionId?` | `string` | Optional interaction identifier. |
| <a id="property-confirm-17"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog. |
| <a id="property-style-1"></a> `style?` | `"primary"` \| `"danger"` | Optional visual emphasis. |
| <a id="property-text-2"></a> `text` | [`TextLike`](objects.md#textlike) | Plain-text label displayed on the button. |
| <a id="property-workflow"></a> `workflow` | [`JsonObject`](utilities.md#jsonobject) | Workflow object created with `workflow`. |
