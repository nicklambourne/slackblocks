---
toc_min_heading_level: 2
toc_max_heading_level: 2
---

# Errors

Typed validation errors returned by eager construction and explicit validation.

## ErrorCategory

> **ErrorCategory** = `"length-exceeded"` \| `"out-of-range"` \| `"mutually-exclusive"` \| `"type-mismatch"` \| `"missing-required"` \| `"invalid-usage"`

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

> **new InvalidUsageError**(`path`, `message`): [`InvalidUsageError`](#invalidusageerror)

Creates a validation error.

##### Parameters

| Parameter | Type | Description |
| ------ | ------ | ------ |
| `path` | `string` | Dot-and-index path to the invalid field. |
| `message` | `string` | Human-readable explanation of the constraint. |

##### Returns

[`InvalidUsageError`](#invalidusageerror)

##### Overrides

`Error.constructor`

### Properties

| Property | Modifier | Type | Default value | Description |
| ------ | ------ | ------ | ------ | ------ |
| <a id="property-category"></a> `category` | `readonly` | [`ErrorCategory`](#errorcategory) | `"invalid-usage"` | Machine-readable failure category. |
| <a id="property-path"></a> `path` | `readonly` | `string` | `undefined` | Dot-and-index path to the invalid payload field. |

***

## LengthError

A string, array, or collection exceeded an allowed minimum or maximum length.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

> **new LengthError**(`path`, `message`): [`LengthError`](#lengtherror)

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

A required field or one-of requirement was not satisfied.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

> **new MissingRequiredError**(`path`, `message`): [`MissingRequiredError`](#missingrequirederror)

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

Fields that cannot be used together were both supplied.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

> **new MutualExclusivityError**(`path`, `message`): [`MutualExclusivityError`](#mutualexclusivityerror)

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

A numeric value fell outside its allowed range.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

> **new OutOfRangeError**(`path`, `message`): [`OutOfRangeError`](#outofrangeerror)

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

A payload field or nested object has the wrong runtime type.

### Extends

- [`InvalidUsageError`](#invalidusageerror)

### Constructors

#### Constructor

> **new TypeMismatchError**(`path`, `message`): [`TypeMismatchError`](#typemismatcherror)

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
