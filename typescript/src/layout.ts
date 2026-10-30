// Layout-specific structural rules and modeled composite traversal.
import limits from "../../spec/limits.json" with { type: "json" };
import { InvalidUsageError, LengthError, MissingRequiredError, TypeMismatchError } from "./errors.js";
import type { JsonObject, JsonValue } from "./types.js";

const CHILDREN = new Set(["actions", "alert", "card", "carousel", "container", "context", "data_table", "data_visualization", "divider", "file", "header", "image", "input", "rich_text", "section", "table", "video"]);
const FALLBACK = new Set(["context", "divider", "header", "image", "rich_text", "section", "table", "video"]);
const WIDE = new Set(["carousel", "data_table", "data_visualization"]);

function object(value: JsonValue | undefined, path: string): JsonObject {
  if (value === undefined) throw new MissingRequiredError(path, "required");
  if (value === null || Array.isArray(value) || typeof value !== "object") throw new TypeMismatchError(path, "expected an object");
  return value;
}
function array(value: JsonValue | undefined, path: string, min: number, max?: number): JsonValue[] {
  if (value === undefined) throw new MissingRequiredError(path, "required");
  if (!Array.isArray(value)) throw new TypeMismatchError(path, "expected an array");
  if (value.length < min || (max !== undefined && value.length > max)) throw new LengthError(path, "invalid number of items");
  return value;
}
function keys(value: JsonObject, allowed: string[], path: string): void {
  for (const key of Object.keys(value)) if (!allowed.includes(key)) throw new TypeMismatchError(`${path}.${key}`, "unsupported property");
}
function choice(value: JsonValue | undefined, allowed: string[], path: string): void {
  if (value !== undefined && (typeof value !== "string" || !allowed.includes(value))) throw new TypeMismatchError(path, "unsupported value");
}
export function validateLayoutItem(value: JsonValue | undefined, path: string): void {
  const item = object(value, path);
  keys(item, ["block", "width", "horizontal_alignment"], path);
  const block = object(item.block, `${path}.block`);
  if (typeof block.type !== "string" || !CHILDREN.has(block.type)) throw new TypeMismatchError(`${path}.block`, "unsupported layout child");
  choice(item.width, ["narrow", "standard", "wide"], `${path}.width`);
  choice(item.horizontal_alignment, ["start", "center", "end"], `${path}.horizontal_alignment`);
}
export function validateLayoutRow(value: JsonValue | undefined, path: string): void {
  const row = object(value, path);
  keys(row, ["items"], path);
  const items = array(row.items, `${path}.items`, limits.layout.items.min_items, limits.layout.items.max_items);
  let total = 0, chart = 0, wide = 0;
  items.forEach((value, i) => {
    const itemPath = `${path}.items[${i}]`;
    validateLayoutItem(value, itemPath);
    const item = object(value, itemPath), block = object(item.block, `${itemPath}.block`);
    const weight = item.width === "narrow" ? 1 : item.width === "wide" ? 3 : 2;
    total += weight;
    if (WIDE.has(String(block.type))) wide += 1;
    if (block.type === "data_visualization") chart = weight;
  });
  if (wide > 1) throw new InvalidUsageError(path, "at most one carousel, data table, or data visualization per row");
  if (chart > 0 && chart * 2 <= total) throw new InvalidUsageError(path, "a data visualization requires more than half the row weight");
}
export function validateLayoutBlock(block: JsonObject, path: string): void {
  if (block.label === undefined) throw new MissingRequiredError(`${path}.label`, "required");
  if (typeof block.label !== "string") throw new TypeMismatchError(`${path}.label`, "expected a string");
  if ([...block.label].length < limits.layout.label.min_length || [...block.label].length > limits.layout.label.max_length) throw new LengthError(`${path}.label`, "expected 1–150 characters");
  array(block.rows, `${path}.rows`, limits.layout.rows.min_items, limits.layout.rows.max_items).forEach((row, i) => validateLayoutRow(row, `${path}.rows[${i}]`));
  if (block.is_label_visible !== undefined && typeof block.is_label_visible !== "boolean") throw new TypeMismatchError(`${path}.is_label_visible`, "expected a boolean");
  choice(block.grid_lines, ["none", "rows", "outer"], `${path}.grid_lines`);
  if (block.fallback !== undefined) array(block.fallback, `${path}.fallback`, 0).forEach((value, i) => {
    const fallback = object(value, `${path}.fallback[${i}]`);
    if (typeof fallback.type !== "string" || !FALLBACK.has(fallback.type)) throw new TypeMismatchError(`${path}.fallback[${i}]`, "unsupported fallback block");
  });
  validateCompositeDepth(block, path);
}
// Visit only actual composite child positions, never application metadata.
export function compositeChildren(block: JsonObject): JsonObject[] {
  if (block.type === "container") return (Array.isArray(block.child_blocks) ? block.child_blocks : []).map((b, i) => object(b, `child_blocks[${i}]`));
  if (block.type === "layout") return (Array.isArray(block.rows) ? block.rows : []).flatMap((r) => array(object(r, "row").items, "items", 1).map((i) => object(object(i, "item").block, "block")));
  return [];
}
export function validateCompositeDepth(block: JsonObject, path: string, depth = 0, hasLayout = false): void {
  hasLayout ||= block.type === "layout";
  if (block.type === "layout" || block.type === "container") depth++;
  if (hasLayout && depth > 2) throw new InvalidUsageError(path, "layout/container nesting exceeds two composite levels");
  compositeChildren(block).forEach((b) => validateCompositeDepth(b, path, depth, hasLayout));
}

export function compositeDescendants(blocks: readonly JsonValue[]): JsonObject[] {
  return blocks.flatMap((value) => {
    const block = object(value, "block");
    const fallback = block.type === "layout" && Array.isArray(block.fallback) ? block.fallback : [];
    return [block, ...compositeDescendants([...compositeChildren(block), ...fallback])];
  });
}
