"""
Custom error classes for clearer error reporting.
"""
class InvalidUsageError(Exception):
    def __init__(self, message: str):
        super(InvalidUsageError, self).__init__(message)
