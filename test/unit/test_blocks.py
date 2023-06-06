from slackblocks import (
    ActionsBlock,
    CheckboxGroup,
    ContextBlock,
    DividerBlock,
    ImageBlock,
    Option,
    SectionBlock,
    Text,
    HeaderBlock,
    TextType,
)


def test_basic_section_block() -> None:
    block = SectionBlock("Hello, world!", block_id="fake_block_id")
    with open("test/samples/blocks/section_block_text_only.json", "r") as expected:
        assert repr(block) == expected.read()


def test_basic_section_fields() -> None:
    block = SectionBlock(
        "Test:",
        fields=[Text(text="foo", type_=TextType.PLAINTEXT), Text(text="bar")],
        block_id="fake_block_id",
    )
    with open("test/samples/blocks/section_block_fields.json", "r") as expected:
        assert repr(block) == expected.read()


def test_basic_context_block() -> None:
    block = ContextBlock(elements=[Text("Hello, world!")], block_id="fake_block_id")
    with open("test/samples/blocks/context_block_text_only.json", "r") as expected:
        assert repr(block) == expected.read()


def test_basic_divider_block() -> None:
    block = DividerBlock(block_id="fake_block_id")
    with open("test/samples/blocks/divider_block_only.json", "r") as expected:
        assert repr(block) == expected.read()


def test_basic_image_block() -> None:
    block = ImageBlock(
        image_url="https://api.slack.com/img/blocks/bkb_template_images/beagle.png",
        alt_text="image1",
        title="image1",
        block_id="fake_block_id",
    )
    with open("test/samples/blocks/image_block_only.json", "r") as expected:
        assert repr(block) == expected.read()


def test_basic_header_block() -> None:
    block = HeaderBlock(text="AloHa!", block_id="fake_block_id")
    with open("test/samples/blocks/header_block_only.json", "r") as expected:
        assert repr(block) == expected.read()


def test_basic_action_block() -> None:
    block = ActionsBlock(block_id="5d1d342f-d65c-4ac5-a2f5-690e48ef207e", elements=[Option(text="Hi", value="Hi")])
    with open("test/samples/blocks/actions_block_only.json", "r") as expected:
        assert repr(block) == expected.read()


def test_checkbox_action_block() -> None:
    options = [
        Option(text="*a*", value="a", description="*a*"),
        Option(text="*b*", value="b", description="*b*"),
        Option(text="*c*", value="c", description="*c*"),
    ]
    block = ActionsBlock(block_id="fake_block_id", elements=CheckboxGroup(action_id="actionId-0", options=options))
    with open("test/samples/blocks/actions_block_checkboxes.json", "r") as expected:
        assert repr(block) == expected.read()