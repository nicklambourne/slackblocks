import pytest

from slackblocks import (
    Block,
    ContainerBlock,
    DividerBlock,
    HomeTabView,
    LayoutBlock,
    LayoutItem,
    LayoutRow,
    Message,
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
