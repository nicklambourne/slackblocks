import pytest

from slackblocks import (
    AlertBlock,
    Block,
    ContainerBlock,
    DividerBlock,
    HomeTabView,
    LayoutBlock,
    LayoutItem,
    LayoutRow,
    Message,
    ModalView,
    RawText,
    TableBlock,
    TypeMismatchError,
)


def layout(**kwargs):
    return LayoutBlock("Grid", [LayoutRow([LayoutItem(DividerBlock())])], **kwargs)


def test_surface_boundaries_and_fallback_alternative():
    Message(channel="C123", blocks=[layout() for _ in range(50)])
    HomeTabView(blocks=[layout() for _ in range(100)])
    # A fallback replaces the grid; the two alternatives are not added together.
    Message(channel="C123", blocks=[layout(fallback=[DividerBlock() for _ in range(51)])])
    # Existing container-only content keeps its pre-layout budget behavior.
    Message(
        channel="C123",
        blocks=[
            ContainerBlock(title="Existing", child_blocks=[DividerBlock() for _ in range(10)])
            for _ in range(50)
        ],
    )


def test_layout_omission_and_explicit_defaults():
    value = layout()._resolve()
    assert "fallback" not in value and "is_label_visible" not in value
    assert "width" not in value["rows"][0]["items"][0]
    value = layout(fallback=[], is_label_visible=False, grid_lines="none")._resolve()
    assert value["fallback"] == [] and value["is_label_visible"] is False
    assert value["grid_lines"] == "none"


@pytest.mark.parametrize("kwargs", [{"width": "full"}, {"horizontal_alignment": "left"}])
def test_layout_uses_its_own_width_and_alignment_vocabulary(kwargs):
    with pytest.raises(TypeMismatchError):
        LayoutItem(DividerBlock(), **kwargs)


def test_unknown_grid_and_nonboolean_visibility():
    for kwargs in ({"grid_lines": "columns"}, {"is_label_visible": 1}):
        with pytest.raises(TypeMismatchError):
            layout(**kwargs)


def test_generic_parser_recognizes_layout_and_reports_the_explicit_limitation():
    # Composite parsing remains unsupported, but layout is a known block type.
    for parser in (Block.from_dict, LayoutBlock.from_dict):
        with pytest.raises(NotImplementedError, match="nested block parsing"):
            parser(layout()._resolve())


@pytest.mark.parametrize("surface", ["message", "home"])
@pytest.mark.parametrize("position", ["layout", "container-layout", "top-level-sibling"])
def test_surface_error_retains_the_actual_nested_path(surface, position):
    bad = AlertBlock("Modal only")
    nested = LayoutBlock(
        "Grid",
        [
            LayoutRow([LayoutItem(DividerBlock())]),
            LayoutRow([LayoutItem(DividerBlock()), LayoutItem(bad)]),
        ],
    )
    if position == "layout":
        blocks = [DividerBlock(), nested]
        path = "blocks[1].rows[1].items[1].block"
    elif position == "container-layout":
        blocks = [
            DividerBlock(),
            ContainerBlock(title="Group", child_blocks=[DividerBlock(), nested]),
        ]
        path = "blocks[1].child_blocks[1].rows[1].items[1].block"
    else:
        blocks = [layout(fallback=[DividerBlock()]), bad]
        path = "blocks[1]"
    with pytest.raises(TypeMismatchError) as caught:
        if surface == "message":
            Message(channel="C123", blocks=blocks)
        else:
            HomeTabView(blocks=blocks)
    assert str(caught.value) == f"{path} type 'alert' is not supported on {surface} surfaces"


def test_surface_error_retains_the_fallback_path():
    blocks = [DividerBlock(), layout(fallback=[DividerBlock(), TableBlock([[RawText("Cell")]])])]
    with pytest.raises(TypeMismatchError) as caught:
        ModalView(title="Test", blocks=blocks)
    assert str(caught.value) == (
        "blocks[1].fallback[1] type 'table' is not supported on modal surfaces"
    )
