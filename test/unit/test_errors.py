import pytest
from slackblocks import Attachment
from slackblocks.errors import InvalidUsageError


def test_invalid_usage_exception() -> None:
    with pytest.raises(InvalidUsageError):
        attachment = Attachment(blocks=[], color="0000000000000")
        print(attachment)
