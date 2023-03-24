from slackblocks import Modal
from slackblocks.blocks import DividerBlock, SectionBlock


def test_modal_without_blocks() -> None:
    modal = Modal("Hello, world!", close="Close button", submit="Submit button")
    with open("test/samples/modal_without_blocks.json", "r") as expected:
        assert repr(modal) == expected.read()


def test_modal_with_blocks() -> None:
    modal = Modal(
        "Hello, world!",
        close="Close button",
        submit="Submit button",
        blocks=[
            SectionBlock(text="first section block", block_id="1"),
            DividerBlock(block_id="2"),
            SectionBlock(text="second section block", block_id="3"),
        ],
    )
    with open("test/samples/modal_with_blocks.json", "r") as expected:
        assert repr(modal) == expected.read()


def test_to_dict() -> None:
    modal = Modal(
        "Hello, world!",
        close="Close button",
        submit="Submit button",
        blocks=[SectionBlock(text="first section block", block_id="1")],
    )
    assert modal.to_dict() == {
        "type": "modal",
        "title": {"type": "plain_text", "text": "Hello, world!"},
        "close": {"type": "plain_text", "text": "Close button"},
        "submit": {"type": "plain_text", "text": "Submit button"},
        "blocks": [
            {
                "type": "section",
                "block_id": "1",
                "text": {
                    "type": "mrkdwn",
                    "text": "first section block",
                    "verbatim": False,
                },
            }
        ],
    }
