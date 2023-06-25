import pytest

from slackblocks import (
    ActionsBlock,
    CheckboxGroup,
    ContextBlock,
    DividerBlock,
    HeaderBlock,
    ImageBlock,
    InvalidUsageError,
    Option,
    SectionBlock,
    Text,
    TextType,
)

from .utils import OPTION_A, THREE_OPTIONS, TWO_OPTIONS, fetch_sample


def test_basic_section_block() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    assert fetch_sample(
        path="test/samples/blocks/section_block_text_only.json"
    ) == repr(block)


def test_basic_section_fields() -> None:
    block = SectionBlock(
        "Test:",
        fields=[Text(text="foo", type_=TextType.PLAINTEXT), Text(text="bar")],
        block_id="fake_block_id",
    )
    assert fetch_sample(path="test/samples/blocks/section_block_fields.json") == repr(
        block
    )


def test_section_empty_text_field_value() -> None:
    block = SectionBlock(
        block_id="fake_block_id",
        fields=[
            Text("Highly", type_=TextType.MARKDOWN),
            Text("Strung", type_=TextType.PLAINTEXT, emoji=True),
        ],
    )
    assert fetch_sample(
        path="test/samples/blocks/section_block_empty_text_field_value.json"
    ) == repr(block)


def test_section_neither_fields_nor_text() -> None:
    with pytest.raises(InvalidUsageError):
        SectionBlock(
            block_id="fake_block_id",
            text=None,
            fields=None,
        )


def test_section_invalid_field_content() -> None:
    with pytest.raises(InvalidUsageError):
        SectionBlock(
            block_id="fake_block_id",
            fields=[
                None,
            ],
        )


def test_section_single_field_value_coercion() -> None:
    block = SectionBlock(
        block_id="fake_block_id",
        fields="Lowly",
    )
    assert fetch_sample(
        path="test/samples/blocks/section_block_single_field_value_coercion.json"
    ) == repr(block)


def test_section_both_text_and_fields() -> None:
    block = SectionBlock(
        text="Hello",
        block_id="fake_block_id",
        fields=[
            Text("Are you", type_=TextType.MARKDOWN),
            Text("There?", type_=TextType.PLAINTEXT, emoji=True),
        ],
    )
    assert fetch_sample(
        path="test/samples/blocks/section_block_both_text_and_fields.json"
    ) == repr(block)


def test_basic_context_block() -> None:
    block = ContextBlock(elements=[Text("Hello, world!")], block_id="fake_block_id")
    assert fetch_sample(
        path="test/samples/blocks/context_block_text_only.json"
    ) == repr(block)


def test_basic_divider_block() -> None:
    block = DividerBlock(block_id="fake_block_id")
    assert fetch_sample(path="test/samples/blocks/divider_block_only.json") == repr(
        block
    )


def test_basic_image_block() -> None:
    block = ImageBlock(
        image_url="https://api.slack.com/img/blocks/bkb_template_images/beagle.png",
        alt_text="image1",
        title="image1",
        block_id="fake_block_id",
    )
    assert fetch_sample(path="test/samples/blocks/image_block_only.json") == repr(block)


def test_basic_header_block() -> None:
    block = HeaderBlock(text="AloHa!", block_id="fake_block_id")
    assert fetch_sample(path="test/samples/blocks/header_block_only.json") == repr(
        block
    )


def test_basic_action_block() -> None:
    block = ActionsBlock(
        block_id="5d1d342f-d65c-4ac5-a2f5-690e48ef207e",
        elements=[Option(text="Hi", value="Hi")],
    )
    assert fetch_sample(path="test/samples/blocks/actions_block_only.json") == repr(
        block
    )


def test_checkbox_action_block() -> None:
    options = [
        Option(text="*a*", value="a", description="*a*"),
        Option(text="*b*", value="b", description="*b*"),
        Option(text="*c*", value="c", description="*c*"),
    ]
    block = ActionsBlock(
        block_id="fake_block_id",
        elements=CheckboxGroup(action_id="actionId-0", options=options),
    )
    assert fetch_sample(
        path="test/samples/blocks/actions_block_checkboxes.json"
    ) == repr(block)
