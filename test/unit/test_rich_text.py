from slackblocks.rich_text import (
    ListType,
    RichText,
    RichTextLink,
    RichTextList,
    RichTextPreformattedCodeBlock,
    RichTextQuote,
    RichTextSection,
)

from .utils import fetch_sample


def test_rich_text_basic() -> None:
    assert fetch_sample(path="test/samples/rich_text/rich_text_basic.json") == repr(
        RichText(
            text="I am a bold rich text block!",
            bold=True,
            italic=True,
            strike=False,
        )
    )


def test_rich_text_link_basic() -> None:
    assert fetch_sample(
        path="test/samples/rich_text/rich_text_link_basic.json"
    ) == repr(
        RichTextLink(
            url="https://google.com/",
            text="Google",
            unsafe=False,
            bold=True,
            italic=False,
        )
    )


def test_rich_text_list_basic() -> None:
    assert fetch_sample(
        path="test/samples/rich_text/rich_text_list_basic.json"
    ) == repr(
        RichTextList(
            elements=[
                RichTextSection(
                    elements=[
                        RichText(
                            text="Oh",
                        )
                    ]
                ),
                RichTextSection(
                    elements=[
                        RichText(
                            text="Hi",
                        )
                    ]
                ),
                RichTextSection(elements=[RichText(text="Mark")]),
            ],
            style=ListType.BULLET,
            indent=0,
            offset=0,
            border=1,
        )
    )


def test_rich_text_list_ordered() -> None:
    assert fetch_sample(
        path="test/samples/rich_text/rich_text_list_ordered.json"
    ) == repr(
        RichTextList(
            elements=[
                RichTextSection(
                    elements=[
                        RichText(
                            text="Oh",
                        )
                    ]
                ),
                RichTextSection(
                    elements=[
                        RichText(
                            text="Hi",
                        )
                    ]
                ),
            ],
            style=ListType.ORDERED,
            indent=1,
            offset=2,
            border=3,
        )
    )


def test_rich_text_preformatted_code_block_basic() -> None:
    assert fetch_sample(
        path="test/samples/rich_text/rich_text_preformatted_code_block_basic.json"
    ) == repr(
        RichTextPreformattedCodeBlock(
            elements=[RichText(text="\ndef hello_world():\n    print('hello, world')")],
            border=0,
        )
    )


def test_rich_text_quote_basic() -> None:
    assert fetch_sample(
        path="test/samples/rich_text/rich_text_quote_basic.json"
    ) == repr(
        RichTextQuote(
            elements=[RichText(text="Great and good are seldom the same man")], border=1
        ),
    )


def test_rich_text_section() -> None:
    assert fetch_sample(
        path="test/samples/rich_text/rich_text_section_basic.json"
    ) == repr(
        RichTextSection(
            elements=[
                RichText(text="The only true wisdom is in knowing you know nothing")
            ]
        )
    )
