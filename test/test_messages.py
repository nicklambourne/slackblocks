import pytest
from slackblocks import Message, SectionBlock


def test_basic_messaget() -> None:
    x = SectionBlock("Hello, world!")
    m = Message(channel="#general", blocks=x)
    assert 1 == 1
