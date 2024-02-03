"""
Custom error classes for clearer error reporting.
"""


class InvalidUsageError(Exception):
    """
    You have violated the `slackblocks` API or Slack Web API in a way
        that has been caught by validation checks.

    Args:
        message: a custom error message providing details of the API
            violation.
    """

    def __init__(self, message: str) -> "InvalidUsageError":
        super(InvalidUsageError, self).__init__(message)
