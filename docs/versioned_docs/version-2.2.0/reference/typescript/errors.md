---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Errors

Typed validation errors raised while builders materialize or explicit payload
validation runs. Every subclass includes a machine-readable category and a
path identifying the invalid field.

## ErrorCategory

```ts
type ErrorCategory =
  | "length-exceeded"
  | "out-of-range"
  | "mutually-exclusive"
  | "type-mismatch"
  | "missing-required"
  | "invalid-usage";
```

Stable machine-readable category attached to every validation error.

***

## InvalidUsageError

Base class for invalid Block Kit input.

Catch this class to handle every slackblocks validation failure, or catch a
subclass when the reason matters to application behavior.

### Extends

- `Error`

### Extended by

- [`LengthError`](#lengtherror)
- [`OutOfRangeError`](#outofrangeerror)
- [`MutualExclusivityError`](#mutualexclusivityerror)
- [`TypeMismatchError`](#typemismatcherror)
- [`MissingRequiredError`](#missingrequirederror)

### Constructors

#### Constructor

```ts
new InvalidUsageError(path, message): InvalidUsageError;
```

Creates a validation error.

##### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `path` | `string` | Dot-and-index path to the invalid field. |
| `message` | `string` | Human-readable explanation of the constraint. |

##### Returns

[`InvalidUsageError`](#invalidusageerror)

##### Overrides

```ts
Error.constructor
```

### Properties

| Property | Modifier | Type | Default value | Description |
| ------ | ------ | ------ | ------ | ------ |
| <a id="property-category"></a> `category` | `readonly` | [`ErrorCategory`](#errorcategory) | `"invalid-usage"` | Machine-readable failure category. |
| <a id="property-path"></a> `path` | `readonly` | `string` | `undefined` | Dot-and-index path to the invalid payload field. |

***

## LengthError

A string, array, or collection falls outside a field's allowed length. Typical
causes include text exceeding Slack's limit, too many options, or an action
identifier longer than 255 characters.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

```ts
new LengthError(path, message): LengthError;
```

Creates a validation error.

##### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `path` | `string` | Dot-and-index path to the invalid field. |
| `message` | `string` | Human-readable explanation of the constraint. |

##### Returns

[`LengthError`](#lengtherror)

##### Inherited from

[`InvalidUsageError`](#invalidusageerror).[`constructor`](#constructor)

### Properties

| Property | Modifier | Type | Description | Overrides |
| ------ | ------ | ------ | ------ | ------ |
| <a id="property-category-1"></a> `category` | `readonly` | `"length-exceeded"` | Machine-readable failure category. | [`InvalidUsageError`](#invalidusageerror).[`category`](#property-category) |
| <a id="property-path-1"></a> `path` | `readonly` | `string` | Dot-and-index path to the invalid payload field. | - |

***

## MissingRequiredError

A required field or one-of requirement is not satisfied. Typical causes include
building a section without text or fields, or creating a conversation filter
without any filter options.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

```ts
new MissingRequiredError(path, message): MissingRequiredError;
```

Creates a validation error.

##### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `path` | `string` | Dot-and-index path to the invalid field. |
| `message` | `string` | Human-readable explanation of the constraint. |

##### Returns

[`MissingRequiredError`](#missingrequirederror)

##### Inherited from

[`InvalidUsageError`](#invalidusageerror).[`constructor`](#constructor)

### Properties

| Property | Modifier | Type | Description | Overrides |
| ------ | ------ | ------ | ------ | ------ |
| <a id="property-category-2"></a> `category` | `readonly` | `"missing-required"` | Machine-readable failure category. | [`InvalidUsageError`](#invalidusageerror).[`category`](#property-category) |
| <a id="property-path-2"></a> `path` | `readonly` | `string` | Dot-and-index path to the invalid payload field. | - |

***

## MutualExclusivityError

Mutually exclusive fields are supplied together. Examples include combining an
image URL with a Slack file, or providing both options and option groups to a
static select menu.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

```ts
new MutualExclusivityError(path, message): MutualExclusivityError;
```

Creates a validation error.

##### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `path` | `string` | Dot-and-index path to the invalid field. |
| `message` | `string` | Human-readable explanation of the constraint. |

##### Returns

[`MutualExclusivityError`](#mutualexclusivityerror)

##### Inherited from

[`InvalidUsageError`](#invalidusageerror).[`constructor`](#constructor)

### Properties

| Property | Modifier | Type | Description | Overrides |
| ------ | ------ | ------ | ------ | ------ |
| <a id="property-category-3"></a> `category` | `readonly` | `"mutually-exclusive"` | Machine-readable failure category. | [`InvalidUsageError`](#invalidusageerror).[`category`](#property-category) |
| <a id="property-path-3"></a> `path` | `readonly` | `string` | Dot-and-index path to the invalid payload field. | - |

***

## OutOfRangeError

A numeric value falls outside a field's allowed range. Typical causes include
minimum values greater than maximum values or counts outside Slack's supported
bounds.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

```ts
new OutOfRangeError(path, message): OutOfRangeError;
```

Creates a validation error.

##### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `path` | `string` | Dot-and-index path to the invalid field. |
| `message` | `string` | Human-readable explanation of the constraint. |

##### Returns

[`OutOfRangeError`](#outofrangeerror)

##### Inherited from

[`InvalidUsageError`](#invalidusageerror).[`constructor`](#constructor)

### Properties

| Property | Modifier | Type | Description | Overrides |
| ------ | ------ | ------ | ------ | ------ |
| <a id="property-category-4"></a> `category` | `readonly` | `"out-of-range"` | Machine-readable failure category. | [`InvalidUsageError`](#invalidusageerror).[`category`](#property-category) |
| <a id="property-path-4"></a> `path` | `readonly` | `string` | Dot-and-index path to the invalid payload field. | - |

***

## TypeMismatchError

A payload field or nested object has the wrong runtime type or an unsupported
discrete value. The error path identifies the exact field that failed runtime
validation.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

```ts
new TypeMismatchError(path, message): TypeMismatchError;
```

Creates a validation error.

##### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `path` | `string` | Dot-and-index path to the invalid field. |
| `message` | `string` | Human-readable explanation of the constraint. |

##### Returns

[`TypeMismatchError`](#typemismatcherror)

##### Inherited from

[`InvalidUsageError`](#invalidusageerror).[`constructor`](#constructor)

### Properties

| Property | Modifier | Type | Description | Overrides |
| ------ | ------ | ------ | ------ | ------ |
| <a id="property-category-5"></a> `category` | `readonly` | `"type-mismatch"` | Machine-readable failure category. | [`InvalidUsageError`](#invalidusageerror).[`category`](#property-category) |
| <a id="property-path-5"></a> `path` | `readonly` | `string` | Dot-and-index path to the invalid payload field. | - |
