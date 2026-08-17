---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Elements

Interactive and visual element factories used inside Block Kit blocks.

Factories accept camelCase input and return validated Slack wire objects.

## button()

> **button**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"button"`&gt;

Creates an interactive button.

### Input fields

Button label, action identifier, and optional behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| `actionId` | `string` | Identifier returned when the button is selected. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog created with `confirmation`. |
| `style?` | `"primary"` \| `"danger"` | Optional visual emphasis. |
| `text` | [`TextLike`](objects.md#textlike) | Plain-text label displayed on the button. |
| `url?` | `string` | Optional URL opened by the button. |
| `value?` | `string` | Optional application-defined value returned with the interaction. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"button"`&gt;

A validated Slack `button` element.

### Throws

InvalidUsageError when a field violates Slack's constraints.

***

## ButtonInput

Fields accepted by [button](#button).

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

## channelMultiSelect()

> **channelMultiSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"multi_channels_select"`&gt;

Creates a multi-select menu populated with public channels visible to the current user.

### Input fields

Channel selection, confirmation, focus, and placeholder behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialChannels?` | `string`[] | Public channel IDs selected when the menu first loads. |
| `maxSelectedItems?` | `number` | Maximum number of channels that may be selected; the minimum is one. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"multi_channels_select"`&gt;

A validated Slack `multi_channels_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, or selection constraint is invalid.

***

## ChannelMultiSelectInput

Fields accepted by [channelMultiSelect](#channelmultiselect).

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

## channelSelect()

> **channelSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"channels_select"`&gt;

Creates a single-select menu populated with public channels visible to the current user.

### Input fields

Initial channel, response URL, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when a channel is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialChannel?` | `string` | Public channel ID selected when the menu first loads. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |
| `responseUrlEnabled?` | `boolean` | Include a `response_url` in a parent modal's submission payload. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"channels_select"`&gt;

A validated Slack `channels_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, or initial selection is invalid.

***

## ChannelSelectInput

Fields accepted by [channelSelect](#channelselect).

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

## checkboxes()

> **checkboxes**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"checkboxes"`&gt;

Creates a checkbox group.

### Input fields

Action identifier, options, and optional Slack checkbox fields.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when the checkbox selection changes, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the changed selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialOptions?` | [`JsonObject`](utilities.md#jsonobject)[] | Options from `options` that are selected when the element first loads. |
| `options` | [`JsonObject`](utilities.md#jsonobject)[] | Up to ten option objects displayed as checkboxes. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"checkboxes"`&gt;

A validated Slack `checkboxes` element.

### Throws

InvalidUsageError when options or identifiers violate Slack's constraints.

***

## CheckboxesInput

Fields accepted by [checkboxes](#checkboxes).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-3"></a> `actionId` | `string` | Identifier returned when the checkbox selection changes, up to 255 characters. |
| <a id="property-confirm-3"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the changed selection is submitted. |
| <a id="property-focusonload-2"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialoptions"></a> `initialOptions?` | [`JsonObject`](utilities.md#jsonobject)[] | Options from `options` that are selected when the element first loads. |
| <a id="property-options"></a> `options` | [`JsonObject`](utilities.md#jsonobject)[] | Up to ten option objects displayed as checkboxes. |

***

## conversationMultiSelect()

> **conversationMultiSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"multi_conversations_select"`&gt;

Creates a multi-select menu of public channels, private channels, DMs, and group DMs.

### Input fields

Conversation filters, initial selection, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `defaultToCurrentConversation?` | `boolean` | Select the conversation from which the view was opened by default. |
| `filter?` | [`JsonObject`](utilities.md#jsonobject) | Filter controlling which public channels, private channels, DMs, and group DMs appear. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialConversations?` | `string`[] | Conversation IDs selected when the menu first loads. |
| `maxSelectedItems?` | `number` | Maximum number of conversations that may be selected; the minimum is one. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"multi_conversations_select"`&gt;

A validated Slack `multi_conversations_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, filter, or selection is invalid.

***

## ConversationMultiSelectInput

Fields accepted by [conversationMultiSelect](#conversationmultiselect).

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

## conversationSelect()

> **conversationSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"conversations_select"`&gt;

Creates a single-select menu of public channels, private channels, DMs, and group DMs.

### Input fields

Conversation filter, initial selection, response URL, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when a conversation is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `defaultToCurrentConversation?` | `boolean` | Select the conversation from which the view was opened by default. |
| `filter?` | [`JsonObject`](utilities.md#jsonobject) | Filter controlling which public channels, private channels, DMs, and group DMs appear. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialConversation?` | `string` | Conversation ID selected when the menu first loads. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |
| `responseUrlEnabled?` | `boolean` | Include a `response_url` in a parent modal's submission payload. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"conversations_select"`&gt;

A validated Slack `conversations_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, filter, or selection is invalid.

***

## ConversationSelectInput

Fields accepted by [conversationSelect](#conversationselect).

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

## datePicker()

> **datePicker**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"datepicker"`&gt;

Creates a date picker.

### Input fields

Action identifier and optional Slack date-picker fields.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when a date is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a date is selected. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialDate?` | `string` | Initially selected date in `YYYY-MM-DD` format. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a date is selected, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"datepicker"`&gt;

A validated Slack `datepicker` element.

### Throws

InvalidUsageError when a field violates Slack's constraints.

***

## DatePickerInput

Fields accepted by [datePicker](#datepicker).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-6"></a> `actionId` | `string` | Identifier returned when a date is selected, up to 255 characters. |
| <a id="property-confirm-6"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a date is selected. |
| <a id="property-focusonload-5"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialdate"></a> `initialDate?` | `string` | Initially selected date in `YYYY-MM-DD` format. |
| <a id="property-placeholder-4"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a date is selected, up to 150 characters. |

***

## dateTimePicker()

> **dateTimePicker**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"datetimepicker"`&gt;

Creates a date-and-time picker.

### Input fields

Action identifier and optional Slack date-time-picker fields.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when a date and time are selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a date and time are selected. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialDateTime?` | `number` | Initially selected date and time as a Unix timestamp in seconds. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"datetimepicker"`&gt;

A validated Slack `datetimepicker` element.

### Throws

InvalidUsageError when a field violates Slack's constraints.

***

## DateTimePickerInput

Fields accepted by [dateTimePicker](#datetimepicker).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-7"></a> `actionId` | `string` | Identifier returned when a date and time are selected, up to 255 characters. |
| <a id="property-confirm-7"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a date and time are selected. |
| <a id="property-focusonload-6"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialdatetime"></a> `initialDateTime?` | `number` | Initially selected date and time as a Unix timestamp in seconds. |

***

## EmailElementInput

Fields accepted by [emailInput](#emailinput).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-8"></a> `actionId` | `string` | Identifier used to find the submitted email value, up to 255 characters. |
| <a id="property-dispatchactionconfig"></a> `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| <a id="property-focusonload-7"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialvalue"></a> `initialValue?` | `string` | Email address present when the input first loads. |
| <a id="property-placeholder-5"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

***

## emailInput()

> **emailInput**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"email_text_input"`&gt;

Creates an email-address input.

### Input fields

Action identifier and optional Slack email-input fields.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier used to find the submitted email value, up to 255 characters. |
| `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialValue?` | `string` | Email address present when the input first loads. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"email_text_input"`&gt;

A validated Slack `email_text_input` element.

### Throws

InvalidUsageError when a field violates Slack's constraints.

***

## externalMultiSelect()

> **externalMultiSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"multi_external_select"`&gt;

Creates a multi-select menu whose options are loaded from the app's configured Options Load URL.

### Input fields

Query threshold, initial options, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialOptions?` | [`JsonObject`](utilities.md#jsonobject)[] | Options selected when the menu first loads. |
| `maxSelectedItems?` | `number` | Maximum number of options that may be selected; the minimum is one. |
| `minQueryLength?` | `number` | Minimum typed characters before Slack requests options; defaults to three. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"multi_external_select"`&gt;

A validated Slack `multi_external_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, query, or selection is invalid.

***

## ExternalMultiSelectInput

Fields accepted by [externalMultiSelect](#externalmultiselect).

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

## externalSelect()

> **externalSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"external_select"`&gt;

Creates a single-select menu whose options are loaded from the app's Options Load URL.

### Input fields

Query threshold, initial option, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when an option is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialOption?` | [`JsonObject`](utilities.md#jsonobject) | Option selected when the menu first loads. |
| `minQueryLength?` | `number` | Minimum typed characters before Slack requests options; defaults to three. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"external_select"`&gt;

A validated Slack `external_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, query, or option is invalid.

***

## ExternalSelectInput

Fields accepted by [externalSelect](#externalselect).

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

## feedbackButton()

> **feedbackButton**(`input`, `settings?`): [`JsonObject`](utilities.md#jsonobject)

Creates one choice used by a feedback-buttons element.

### Input fields

Choice text, returned value, and optional accessible label.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| `text` | [`TextLike`](objects.md#textlike) | Plain-text feedback choice. |
| `value` | `string` | Application-defined value returned with the feedback. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`JsonObject`](utilities.md#jsonobject)

A validated feedback-button object.

### Throws

InvalidUsageError when a field exceeds Slack's limits.

***

## FeedbackButtonInput

Fields accepted by [feedbackButton](#feedbackbutton).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-accessibilitylabel-1"></a> `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| <a id="property-text-1"></a> `text` | [`TextLike`](objects.md#textlike) | Plain-text feedback choice. |
| <a id="property-value-1"></a> `value` | `string` | Application-defined value returned with the feedback. |

***

## feedbackButtons()

> **feedbackButtons**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"feedback_buttons"`&gt;

Creates a paired positive/negative feedback control.

### Input fields

Positive and negative feedback buttons plus an optional action identifier.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId?` | `string` | Optional identifier returned with the interaction. |
| `negativeButton` | [`JsonObject`](utilities.md#jsonobject) | Negative choice created with `feedbackButton`. |
| `positiveButton` | [`JsonObject`](utilities.md#jsonobject) | Positive choice created with `feedbackButton`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"feedback_buttons"`&gt;

A validated Slack `feedback_buttons` element.

### Throws

InvalidUsageError when either feedback choice is invalid.

***

## fileInput()

> **fileInput**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"file_input"`&gt;

Creates a file-upload input.

### Input fields

Action identifier, allowed extensions, and maximum file count.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned with submitted files. |
| `filetypes?` | `string`[] | Optional allowed file extensions. |
| `maxFiles?` | `number` | Maximum files accepted, between 1 and 10. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"file_input"`&gt;

A validated Slack `file_input` element.

### Throws

InvalidUsageError when `maxFiles` is outside 1–10.

***

## iconButton()

> **iconButton**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"icon_button"`&gt;

Creates a compact icon action for a context-actions block.

### Input fields

Accessible text, icon behavior, and optional visibility restrictions.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `accessibilityLabel?` | `string` | Accessible label when the text is insufficient. |
| `actionId?` | `string` | Optional identifier returned with the interaction. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog. |
| `icon?` | `"trash"` | Icon name. Slack currently accepts only `trash`. |
| `text` | [`TextLike`](objects.md#textlike) | Plain-text description of the icon action. |
| `value?` | `string` | Optional application-defined interaction value. |
| `visibleToUserIds?` | `string`[] | Up to ten user IDs allowed to see the action. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"icon_button"`&gt;

A validated Slack `icon_button` element.

### Throws

InvalidUsageError when the icon or visible-user list is invalid.

***

## imageElement()

> **imageElement**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"image"`&gt;

Creates an image element from a URL or Slack-hosted file.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `input` | [`ImageElementInput`](#imageelementinput) | Accessible text and exactly one image source. |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"image"`&gt;

A validated Slack `image` element.

### Throws

InvalidUsageError when both or neither image source is provided.

***

## ImageElementInput

> **ImageElementInput** = \{ `altText`: `string`; `imageUrl`: `string`; `slackFile?`: `never`; \} \| \{ `altText`: `string`; `imageUrl?`: `never`; `slackFile`: [`JsonObject`](utilities.md#jsonobject); \}

URL-backed or Slack-file-backed image fields accepted by [imageElement](#imageelement).

### Union Members

#### Type Literal

\{ `altText`: `string`; `imageUrl`: `string`; `slackFile?`: `never`; \}

| Name | Type | Description |
| ------ | ------ | ------ |
| `altText` | `string` | Accessible plain-text summary of the image. |
| `imageUrl` | `string` | Public image URL, up to 3,000 characters. |
| `slackFile?` | `never` | A Slack file cannot be combined with `imageUrl`. |

***

#### Type Literal

\{ `altText`: `string`; `imageUrl?`: `never`; `slackFile`: [`JsonObject`](utilities.md#jsonobject); \}

| Name | Type | Description |
| ------ | ------ | ------ |
| `altText` | `string` | Accessible plain-text summary of the image. |
| `imageUrl?` | `never` | An image URL cannot be combined with `slackFile`. |
| `slackFile` | [`JsonObject`](utilities.md#jsonobject) | Slack-hosted file reference created with `slackFile`. |

***

## NumberElementInput

Fields accepted by [numberInput](#numberinput).

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

## numberInput()

> **numberInput**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"number_input"`&gt;

Creates a single-line input for whole or decimal numeric values.

### Input fields

Decimal behavior, initial value, range, dispatch, and display settings.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier used to find the submitted numeric value, up to 255 characters. |
| `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialValue?` | `string` | Numeric text present when the input first loads. |
| `isDecimalAllowed?` | `boolean` | Whether the input accepts decimal values as well as whole numbers. |
| `maxValue?` | `number` | Maximum accepted value; it cannot be less than `minValue`. |
| `minValue?` | `number` | Minimum accepted value; it cannot exceed `maxValue`. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"number_input"`&gt;

A validated Slack `number_input` element.

### Throws

InvalidUsageError when an identifier, placeholder, or numeric range is invalid.

***

## overflow()

> **overflow**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"overflow"`&gt;

Creates an overflow menu.

### Input fields

Action identifier, two to five options, and optional Slack fields.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when an option is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after an option is selected. |
| `options` | [`JsonObject`](utilities.md#jsonobject)[] | Between two and five option objects displayed in the compact menu. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"overflow"`&gt;

A validated Slack `overflow` element.

### Throws

InvalidUsageError when the option list violates Slack's constraints.

***

## OverflowInput

Fields accepted by [overflow](#overflow).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-12"></a> `actionId` | `string` | Identifier returned when an option is selected, up to 255 characters. |
| <a id="property-confirm-10"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after an option is selected. |
| <a id="property-options-1"></a> `options` | [`JsonObject`](utilities.md#jsonobject)[] | Between two and five option objects displayed in the compact menu. |

***

## PlainTextElementInput

Fields accepted by [plainTextInput](#plaintextinput).

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

## plainTextInput()

> **plainTextInput**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"plain_text_input"`&gt;

Creates a single- or multi-line freeform text input for a modal or App Home tab.

### Input fields

Initial text, length limits, multiline, dispatch, and display settings.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier used to find the submitted text value, up to 255 characters. |
| `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialValue?` | `string` | Text present when the input first loads. |
| `maxLength?` | `number` | Maximum number of characters the user may enter, between 1 and 3000. |
| `minLength?` | `number` | Minimum number of characters the user must enter, between 0 and 3000. |
| `multiline?` | `boolean` | Whether the input is a multi-line textarea instead of a single line. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"plain_text_input"`&gt;

A validated Slack `plain_text_input` element.

### Throws

InvalidUsageError when an identifier, placeholder, or length constraint is invalid.

***

## radioButtons()

> **radioButtons**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"radio_buttons"`&gt;

Creates a radio-button group.

### Input fields

Action identifier, options, and optional Slack fields.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialOption?` | [`JsonObject`](utilities.md#jsonobject) | Option from `options` selected when the element first loads. |
| `options` | [`JsonObject`](utilities.md#jsonobject)[] | Up to ten options displayed as radio buttons. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"radio_buttons"`&gt;

A validated Slack `radio_buttons` element.

### Throws

InvalidUsageError when options or identifiers violate Slack's constraints.

***

## RadioButtonsInput

Fields accepted by [radioButtons](#radiobuttons).

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

Fields accepted by [richTextInput](#richtextinput).

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

## richTextInput()

> **richTextInput**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_input"`&gt;

Creates a rich-text input for a modal or App Home tab.

### Input fields

Initial content, dispatch configuration, line limits, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier used to find the submitted rich-text value, up to 255 characters. |
| `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when editing dispatches a `block_actions` payload. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialValue?` | [`JsonObject`](utilities.md#jsonobject) | Rich-text content present when the editor first loads. |
| `maxLines?` | `number` | Maximum visible editor lines before scrolling, between 1 and 100. |
| `minLines?` | `number` | Minimum visible editor lines before scrolling, between 1 and 100. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty editor, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"rich_text_input"`&gt;

A validated Slack `rich_text_input` element.

### Throws

InvalidUsageError when an identifier, placeholder, content, or line limit is invalid.

***

## staticMultiSelect()

> **staticMultiSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"multi_static_select"`&gt;

Creates a multi-select menu from options embedded directly in the Block Kit payload.

### Input fields

Options or option groups, initial selection, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialOptions?` | [`JsonObject`](utilities.md#jsonobject)[] | Options selected when the menu first loads. |
| `maxSelectedItems?` | `number` | Maximum number of options that may be selected; the minimum is one. |
| `optionGroups?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 groups of options; mutually exclusive with `options`. |
| `options?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 directly supplied options; mutually exclusive with `optionGroups`. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"multi_static_select"`&gt;

A validated Slack `multi_static_select` element.

### Throws

InvalidUsageError when options, identifiers, or selection constraints are invalid.

***

## StaticMultiSelectInput

Fields accepted by [staticMultiSelect](#staticmultiselect).

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

## staticSelect()

> **staticSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"static_select"`&gt;

Creates a single-select menu from options embedded directly in the Block Kit payload.

### Input fields

Options or option groups, initial option, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when an option is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialOption?` | [`JsonObject`](utilities.md#jsonobject) | Option selected when the menu first loads. |
| `optionGroups?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 groups of options; mutually exclusive with `options`. |
| `options?` | [`JsonObject`](utilities.md#jsonobject)[] | Up to 100 directly supplied options; mutually exclusive with `optionGroups`. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"static_select"`&gt;

A validated Slack `static_select` element.

### Throws

InvalidUsageError when options, identifiers, or selection constraints are invalid.

***

## StaticSelectInput

Fields accepted by [staticSelect](#staticselect).

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

## timePicker()

> **timePicker**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"timepicker"`&gt;

Creates a time picker.

### Input fields

Initial time, timezone, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when a time is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown after a time is selected. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialTime?` | `string` | Initially selected time in 24-hour `HH:mm` format. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a time is selected, up to 150 characters. |
| `timezone?` | `string` | IANA timezone displayed as supporting text and returned with interactions. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"timepicker"`&gt;

A validated Slack `timepicker` element.

### Throws

InvalidUsageError when an identifier, placeholder, time, or timezone is invalid.

***

## TimePickerInput

Fields accepted by [timePicker](#timepicker).

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

Fields accepted by [urlInput](#urlinput).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-19"></a> `actionId` | `string` | Identifier used to find the submitted URL, up to 255 characters. |
| <a id="property-dispatchactionconfig-4"></a> `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| <a id="property-focusonload-17"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialvalue-4"></a> `initialValue?` | `string` | URL present when the input first loads. |
| <a id="property-placeholder-14"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

***

## urlInput()

> **urlInput**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"url_text_input"`&gt;

Creates a URL input.

### Input fields

Initial URL, dispatch configuration, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier used to find the submitted URL, up to 255 characters. |
| `dispatchActionConfig?` | [`JsonObject`](utilities.md#jsonobject) | Configuration controlling when typing dispatches a `block_actions` payload. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialValue?` | `string` | URL present when the input first loads. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown in the empty input, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"url_text_input"`&gt;

A validated Slack `url_text_input` element.

### Throws

InvalidUsageError when an identifier, placeholder, or initial URL is invalid.

***

## urlSource()

> **urlSource**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"url"`&gt;

Creates a source link for a task card.

### Input fields

Public URL and human-readable link text.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `text` | `string` | Human-readable source label. |
| `url` | `string` | Public source URL. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"url"`&gt;

A validated Slack `url` source element.

### Throws

InvalidUsageError when a field violates Slack's constraints.

***

## userMultiSelect()

> **userMultiSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"multi_users_select"`&gt;

Creates a multi-select menu populated with users visible in the active workspace.

### Input fields

Initial users, confirmation, selection limit, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when the selection changes, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialUsers?` | `string`[] | User IDs selected when the menu first loads. |
| `maxSelectedItems?` | `number` | Maximum number of users that may be selected; the minimum is one. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"multi_users_select"`&gt;

A validated Slack `multi_users_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, or selection constraint is invalid.

***

## UserMultiSelectInput

Fields accepted by [userMultiSelect](#usermultiselect).

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

## userSelect()

> **userSelect**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"users_select"`&gt;

Creates a single-select menu populated with users visible in the active workspace.

### Input fields

Initial user, confirmation, and display behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `actionId` | `string` | Identifier returned when a user is selected, up to 255 characters. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| `initialUser?` | `string` | User ID selected when the menu first loads. |
| `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"users_select"`&gt;

A validated Slack `users_select` element.

### Throws

InvalidUsageError when an identifier, placeholder, or initial user is invalid.

***

## UserSelectInput

Fields accepted by [userSelect](#userselect).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-actionid-21"></a> `actionId` | `string` | Identifier returned when a user is selected, up to 255 characters. |
| <a id="property-confirm-16"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog shown before the selection is submitted. |
| <a id="property-focusonload-19"></a> `focusOnLoad?` | `boolean` | Whether this element receives focus when its containing view opens. |
| <a id="property-initialuser"></a> `initialUser?` | `string` | User ID selected when the menu first loads. |
| <a id="property-placeholder-16"></a> `placeholder?` | [`TextLike`](objects.md#textlike) | Plain-text prompt shown before a selection, up to 150 characters. |

***

## workflowButton()

> **workflowButton**(`input`, `settings?`): [`SlackObject`](utilities.md#slackobject)&lt;`"workflow_button"`&gt;

Creates a button that launches a Slack workflow.

### Input fields

Button label, workflow trigger, and optional interaction behavior.

| Input field | Type | Description |
| ------ | ------ | ------ |
| `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| `actionId?` | `string` | Optional interaction identifier. |
| `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog. |
| `style?` | `"primary"` \| `"danger"` | Optional visual emphasis. |
| `text` | [`TextLike`](objects.md#textlike) | Plain-text label displayed on the button. |
| `workflow` | [`JsonObject`](utilities.md#jsonobject) | Workflow object created with `workflow`. |

### Settings

| Setting | Type | Description |
| ------ | ------ | ------ |
| `settings` | [`FactorySettings`](utilities.md#factorysettings) | Per-call validation settings. |

### Returns

[`SlackObject`](utilities.md#slackobject)&lt;`"workflow_button"`&gt;

A validated Slack `workflow_button` element.

### Throws

InvalidUsageError when text or identifiers violate Slack's constraints.

***

## WorkflowButtonInput

Fields accepted by [workflowButton](#workflowbutton).

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-accessibilitylabel-2"></a> `accessibilityLabel?` | `string` | Accessible label when the visible text is insufficient. |
| <a id="property-actionid-22"></a> `actionId?` | `string` | Optional interaction identifier. |
| <a id="property-confirm-17"></a> `confirm?` | [`JsonObject`](utilities.md#jsonobject) | Optional confirmation dialog. |
| <a id="property-style-1"></a> `style?` | `"primary"` \| `"danger"` | Optional visual emphasis. |
| <a id="property-text-2"></a> `text` | [`TextLike`](objects.md#textlike) | Plain-text label displayed on the button. |
| <a id="property-workflow"></a> `workflow` | [`JsonObject`](utilities.md#jsonobject) | Workflow object created with `workflow`. |
