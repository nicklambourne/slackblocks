"""Modeled composite-block traversal. Opaque payload metadata is never visited."""

from typing import Any

from .errors import InvalidUsageError


def child_blocks(block: dict[str, Any]) -> list[dict[str, Any]]:
    if block.get("type") == "container":
        return block.get("child_blocks", [])
    if block.get("type") == "layout":
        return [item["block"] for row in block.get("rows", []) for item in row["items"]]
    return []


def validate_composite_depth(
    block: dict[str, Any], depth: int = 0, has_layout: bool = False
) -> None:
    kind = block.get("type")
    has_layout = has_layout or kind == "layout"
    if kind in ("layout", "container"):
        depth += 1
    if has_layout and depth > 2:
        raise InvalidUsageError("Layout and container nesting cannot exceed two composite levels.")
    for nested in child_blocks(block):
        validate_composite_depth(nested, depth, has_layout)


def descendants_with_paths(
    blocks: list[dict[str, Any]], path: str = "blocks"
) -> list[tuple[str, dict[str, Any]]]:
    """Walk modeled children and fallback while retaining their wire locations."""
    result = []

    def visit(block: dict[str, Any], block_path: str) -> None:
        result.append((block_path, block))
        if block.get("type") == "container":
            for index, nested in enumerate(block.get("child_blocks", [])):
                visit(nested, f"{block_path}.child_blocks[{index}]")
        elif block.get("type") == "layout":
            for row_index, row in enumerate(block.get("rows", [])):
                for item_index, item in enumerate(row["items"]):
                    visit(
                        item["block"], f"{block_path}.rows[{row_index}].items[{item_index}].block"
                    )
            for index, nested in enumerate(block.get("fallback", [])):
                visit(nested, f"{block_path}.fallback[{index}]")

    for index, block in enumerate(blocks):
        visit(block, f"{path}[{index}]")
    return result


def descendants(blocks: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Walk actual children and fallback content, without interpreting application data."""
    return [block for _, block in descendants_with_paths(blocks)]
