# Installation

You can install `slackblocks` using any Python package manager with access to PyPI. Installation commands for some of the more popular ones are included below.

=== "pip"
    ```bash
    pip install slackblocks
    ```

=== "poetry"
    ```bash
    poetry add slackblocks
    ```

=== "Pipenv"
    ```bash
    pipenv install slackblocks
    ```

=== "uv"
    ```bash
    uv add slackblocks
    ```

`slackblocks` is a pure Python package and is published to [PyPI](https://pypi.org/project/slackblocks/) as a wheel on every release. As of `v0.1.0` it has no dependencies outside of the Python standard library.

## Requirements

- Python 3.8.1 or newer.
- No runtime dependencies.

## Verifying your installation

```python
from slackblocks import Message, SectionBlock

print(Message(channel="#general", blocks=SectionBlock("Hello, world!")).json())
```

If this prints a formatted JSON object describing a message, you're ready to go.

## Version pinning

`slackblocks` follows semantic versioning. Pinning to a major version is recommended:

=== "pip (`requirements.txt`)"
    ```
    slackblocks~=1.2
    ```

=== "poetry (`pyproject.toml`)"
    ```toml
    slackblocks = "^1.2"
    ```

## Uninstalling

=== "pip"
    ```bash
    pip uninstall slackblocks
    ```

=== "poetry"
    ```bash
    poetry remove slackblocks
    ```

=== "Pipenv"
    ```bash
    pipenv uninstall slackblocks
    ```

=== "uv"
    ```bash
    uv remove slackblocks
    ```
