---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Utilities

Types and helpers for building and validating Block Kit payloads.

## assertValid()

```ts
function assertValid(payload): asserts payload is JsonObject;
```

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

```ts
function blockKitBuilderUrl(payload, teamId?): string;
```

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

```ts
type BlockKitPayload = JsonObject;
```

Generic validated Block Kit object.

***

## Buildable&lt;Value&gt; {#buildable}

```ts
type Buildable<Value> = 
  | Value
  | FluentBuilder<object, Value>
  | Value extends readonly infer Item[] ? readonly Buildable<Item>[] : never;
```

A value accepted by a fluent parent: wire data, nested data, or another builder.

### Type Parameters

| Type Parameter |
| ------ |
| `Value` |

***

## FactorySettings

Per-call behavior supported by every public factory.

### Properties

| Property | Type | Description |
| ------ | ------ | ------ |
| <a id="property-validate"></a> `validate?` | `boolean` | Whether to validate the constructed object immediately. Defaults to `true`. Disable only when intentionally creating an intermediate partial object. |

***

## FluentBuilder&lt;Input, Output&gt; {#fluentbuilder}

```ts
type FluentBuilder<Input, Output> = FluentMethods<Input, Output> & object;
```

Typed, chainable construction of a plain Slack wire object.

### Type Declaration

| Name | Type | Description |
| ------ | ------ | ------ |
| `build()` | (`settings?`) => `Output` | Materialises and validates the completed Slack object. |

### Type Parameters

| Type Parameter |
| ------ |
| `Input` *extends* `object` |
| `Output` |

***

## FluentGroupBuilder&lt;Input, Output&gt; {#fluentgroupbuilder}

```ts
type FluentGroupBuilder<Input, Output> = FluentBuilder<Input, Output[]>;
```

A fluent component that expands to multiple values in a parent collection.

### Type Parameters

| Type Parameter |
| ------ |
| `Input` *extends* `object` |
| `Output` |

***

## JsonObject

Object with JSON-compatible values and Slack-shaped string keys.

### Indexable

```ts
[key: string]: JsonValue
```

JSON field value by wire-format key.

***

## JsonPrimitive

```ts
type JsonPrimitive = boolean | number | string | null;
```

JSON scalar accepted by Slack payloads.

***

## JsonValue

```ts
type JsonValue = 
  | JsonPrimitive
  | JsonObject
  | JsonValue[];
```

Recursive JSON value accepted by Slack payloads.

***

## SlackCompatibleBlock

```ts
type SlackCompatibleBlock = SlackWire<KnownBlock>;
```

Compatibility helper for call sites that accept Slack's official block types.

***

## SlackObject&lt;Type&gt; {#slackobject}

```ts
type SlackObject<Type> = JsonObject & object;
```

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

```ts
type SlackWire<Type> = Type & JsonObject;
```

Official Slack SDK type intersected with its JSON wire representation.

### Type Parameters

| Type Parameter |
| ------ |
| `Type` |

***

## validate()

```ts
function validate(payload): payload is JsonObject;
```

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
