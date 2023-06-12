from slackblocks import HomeTabView, Modal
from slackblocks.blocks import DividerBlock, SectionBlock

from .utils import fetch_sample


def test_modal_with_blocks() -> None:
    modal = Modal(
        title="Hello, world!",
        close="Close button",
        submit="Submit button",
        blocks=[
            SectionBlock(text="first section block", block_id="1"),
            DividerBlock(block_id="2"),
            SectionBlock(text="second section block", block_id="3"),
        ],
    )
    assert fetch_sample("test/samples/views/modal_with_blocks.json") == repr(modal)


def test_hometab_view() -> None:
    view = HomeTabView(blocks=[SectionBlock(text="Example Block", block_id="fake_id")])
    assert fetch_sample("test/samples/views/hometab_view.json") == repr(view)


def test_to_dict() -> None:
    modal = Modal(
        title="Hello, world!",
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
                },
            }
        ],
        "clear_on_close": False,
        "notify_on_close": False,
        "submit_disabled": False,
    }
