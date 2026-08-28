/**
 * Shared machinery for the public fluent Block Kit API.
 *
 * @module fluent
 */
import type { FactorySettings } from "../types.js";

const FLUENT_BUILDER = Symbol("slackblocks.fluent-builder");

type NonNullish<Value> = Exclude<Value, null | undefined>;

/** A value accepted by a fluent parent: either wire data or another builder. */
export type Buildable<Value> = Value | FluentBuilder<object, Value>;

type CollectionArgument<Value> =
  | Buildable<NonNullish<Value>>
  | readonly Buildable<NonNullish<Value>>[];

type FluentMethods<Input extends object, Output> = {
  [Key in keyof Input]-?: NonNullish<Input[Key]> extends readonly (infer Item)[]
    ? (...values: CollectionArgument<Item>[]) => FluentBuilder<Input, Output>
    : (value: Buildable<NonNullish<Input[Key]>>) => FluentBuilder<Input, Output>;
};

/** Typed, chainable construction of a plain Slack wire object. */
export type FluentBuilder<Input extends object, Output> = FluentMethods<Input, Output> & {
  /** Materialises and validates the completed Slack object. */
  build(settings?: FactorySettings): Output;
};

type FluentFactory<Input extends object, Output> = (
  input: Input,
  settings?: FactorySettings,
) => Output;

type CollectionMode = "flat" | "nested";

interface FluentBuilderOptions<Input extends object> {
  collections?: Partial<Record<keyof Input, CollectionMode>>;
}

interface FluentBuilderRuntime {
  [FLUENT_BUILDER]: true;
  build(settings?: FactorySettings): unknown;
}

function isFluentBuilder(value: unknown): value is FluentBuilderRuntime {
  return (
    value !== null &&
    typeof value === "object" &&
    FLUENT_BUILDER in value &&
    value[FLUENT_BUILDER] === true
  );
}

function materialise(value: unknown, settings: FactorySettings): unknown {
  if (isFluentBuilder(value)) return value.build(settings);
  if (Array.isArray(value)) return value.map((item) => materialise(item, settings));
  return value;
}

/** @internal */
export function createFluentBuilder<Input extends object, Output>(
  factory: FluentFactory<Input, Output>,
  options: FluentBuilderOptions<Input> = {},
): FluentBuilder<Input, Output> {
  const state = new Map<keyof Input, unknown>();
  let proxy: FluentBuilder<Input, Output>;

  const runtime: FluentBuilderRuntime = {
    [FLUENT_BUILDER]: true,
    build(settings: FactorySettings = {}) {
      const input = Object.fromEntries(
        [...state.entries()].map(([key, value]) => [
          key,
          materialise(value, settings),
        ]),
      ) as Input;
      return factory(input, settings);
    },
  };

  proxy = new Proxy(runtime, {
    get(target, property, receiver) {
      if (Reflect.has(target, property)) {
        return Reflect.get(target, property, receiver);
      }
      if (typeof property !== "string") return undefined;

      return (...values: unknown[]) => {
        if (values.length === 0) {
          throw new TypeError(`${property}() expects a value`);
        }

        const key = property as keyof Input;
        const collectionMode = options.collections?.[key];
        if (collectionMode === undefined) {
          if (values.length !== 1) {
            throw new TypeError(`${property}() expects one value`);
          }
          state.set(key, values[0]);
          return proxy;
        }

        const appended =
          collectionMode === "flat" && values.length === 1 && Array.isArray(values[0])
            ? values[0]
            : values;
        const existing = state.get(key);
        state.set(key, [...(Array.isArray(existing) ? existing : []), ...appended]);
        return proxy;
      };
    },
  }) as unknown as FluentBuilder<Input, Output>;

  return proxy;
}
