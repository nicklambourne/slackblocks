"""
Custom error classes for clearer error reporting.
"""

from __future__ import annotations


class InvalidUsageError(Exception):
    """
    You have violated the `slackblocks` API or Slack Web API in a way
        that has been caught by validation checks.

    Args:
        message: a custom error message providing details of the API
            violation.
    """

    def __init__(self, message: str) -> None:
        return super().__init__(message)
