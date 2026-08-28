---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Utilities

Types and helpers for building and validating Block Kit payloads.

## assertValid()

> **assertValid**(`payload`): `asserts payload is JsonObject`

Asserts that an object is a valid Block Kit payload.

Validation walks nested blocks, elements, views, and composition objects and
reports the first failing field through a typed validation error.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `payload` | [`JsonValue`](#jsonvalue) | JSON value to validate. |

### Returns

`asserts payload is JsonObject`

### Throws

InvalidUsageError when the payload violates a supported Block Kit constraint.

***

## blockKitBuilderUrl()

> **blockKitBuilderUrl**(`payload`, `teamId?`): `string`

Builds a Block Kit Builder URL containing a serialized payload.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `payload` | [`JsonObject`](#jsonobject) \| [`JsonObject`](#jsonobject)[] | A complete payload or a list of blocks. |
| `teamId?` | `string` | Optional workspace ID used in the Builder URL. |

### Returns

`string`

A URL that opens the payload in Slack's Block Kit Builder.

***

## BlockKitPayload

> **BlockKitPayload** = [`JsonObject`](#jsonobject)

Generic validated Block Kit object.

***

## FactorySettings

Per-call behavior supported by every public factory.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-validate"></a> `validate?` | `boolean` | Whether to validate the constructed object immediately. Defaults to `true`. Disable only when intentionally creating an intermediate partial object. |

***

## JsonObject

Object with JSON-compatible values and Slack-shaped string keys.

### Indexable

> \[`key`: `string`\]: [`JsonValue`](#jsonvalue)

JSON field value by wire-format key.

***

## JsonPrimitive

> **JsonPrimitive** = `boolean` \| `number` \| `string` \| `null`

JSON scalar accepted by Slack payloads.

***

## JsonValue

> **JsonValue** = [`JsonPrimitive`](#jsonprimitive) \| [`JsonObject`](#jsonobject) \| [`JsonValue`](#jsonvalue)[]

Recursive JSON value accepted by Slack payloads.

***

## SlackCompatibleBlock

> **SlackCompatibleBlock** = [`SlackWire`](#slackwire)&lt;`KnownBlock`&gt;

Compatibility helper for call sites that accept Slack's official block types.

***

## SlackObject&lt;Type&gt; {#slackobject}

> **SlackObject**&lt;`Type`&gt; = [`JsonObject`](#jsonobject) & `object`

Slack-shaped JSON object whose `type` field is known.

### Type Declaration

| Name | Type | Description |
| ------ | ------ | ------ |
| `type` | `Type` | Discriminator identifying the Block Kit object on Slack's wire format. |

### Type Parameters

| Type Parameter |
| ------ |
| `Type` *extends* `string` |

***

## SlackWire&lt;Type&gt; {#slackwire}

> **SlackWire**&lt;`Type`&gt; = `Type` & [`JsonObject`](#jsonobject)

Official Slack SDK type intersected with its JSON wire representation.

### Type Parameters

| Type Parameter |
| ------ |
| `Type` |

***

## validate()

> **validate**(`payload`): `payload is JsonObject`

Checks whether a value is a valid Block Kit payload without throwing for validation failures.

Validation identifies objects by their `type` field, so it enforces required
fields and limits for every typed block, element, view, and rich-text object,
and it validates type-less `options`, `option_groups`, and `confirm`
composition objects contextually through their typed parents. Known
asymmetries with factory validation remain for type-less objects that
appear without a typed parent: standalone confirmation dialogs, options,
option groups, attachments, message payloads, workflow objects, and chart
axis configurations pass unchecked, and one-of rules enforced only by
factory signatures (for example `slackFile` requiring exactly one source)
are not rediscovered from raw JSON. The contents of message metadata
`event_payload` objects are always treated as opaque user data and skipped.

### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `payload` | `unknown` | Unknown value to validate. |

### Returns

`payload is JsonObject`

`true` for a valid payload; otherwise `false`.
