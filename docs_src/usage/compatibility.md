# Compatibility

This page documents which Python versions are supported by each `slackblocks` release line, so you can choose the right line for your runtime.

## Support matrix

| `slackblocks` version | Minimum Python | Notes |
| --- | --- | --- |
| `1.x` (current stable) | 3.8.1 | The last release line to support Python 3.8 and 3.9. |
| `2.x` (in development) | 3.10 | Modern typing (`X | Y`, `list[X]`), dataclasses with `slots`, `match` statements, and other Python 3.10+ features used internally. |

If you cannot upgrade Python, pin to the appropriate major version:

=== "pip (`requirements.txt`)"
    ```
    slackblocks>=1,<2     # Python 3.8/3.9 friendly
    slackblocks>=2,<3     # Python 3.10+ only
    ```

=== "poetry (`pyproject.toml`)"
    ```toml
    slackblocks = "^1"    # Python 3.8/3.9 friendly
    slackblocks = "^2"    # Python 3.10+ only
    ```

=== "uv (`pyproject.toml`)"
    ```toml
    "slackblocks>=1,<2"   # Python 3.8/3.9 friendly
    "slackblocks>=2,<3"   # Python 3.10+ only
    ```

## Why we move forward

`slackblocks` follows the upstream Python release cycle. Once a Python version reaches its [official end-of-life](https://devguide.python.org/versions/), it stops receiving security patches, and we prefer not to ship new feature work targeting unmaintained runtimes.

| Python version | EOL date |
| --- | --- |
| 3.8 | 2024-10-07 |
| 3.9 | 2025-10-31 |
| 3.10 | 2026-10 |
| 3.11 | 2027-10 |
| 3.12 | 2028-10 |
| 3.13 | 2029-10 |
| 3.14 | 2030-10 |

Bugfix releases on existing lines may continue past these dates on a best-effort basis, but new feature work will target supported Pythons.

## Choosing a version

- **You're on Python 3.8 or 3.9**: install the latest `1.x`.
- **You're on Python 3.10 or newer**: install `2.x` once released; `1.x` will continue to work for the foreseeable future.
- **You're starting a new project on a modern Python**: prefer `2.x` to take advantage of the improved typing surface.

For an upgrade walkthrough from `1.x` to `2.x`, see the
[Migration Guide](migration.md).
