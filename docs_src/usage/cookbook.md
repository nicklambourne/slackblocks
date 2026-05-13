# Cookbook

End-to-end recipes for common Slack messaging patterns. Each example produces a complete, valid `Message` (or `Modal`) you can pass directly to the Slack SDK.

The reference docs cover *what* each class does in isolation; this page is for *how* to combine them into something useful.

## Build status notification

A typical CI/CD success or failure notification: header, summary fields, divider, contextual footer.

```python
from slackblocks import (
    ContextBlock,
    DividerBlock,
    HeaderBlock,
    Message,
    SectionBlock,
    Text,
)


def build_status_message(
    *,
    channel: str,
    project: str,
    branch: str,
    commit: str,
    author: str,
    duration: str,
    passed: bool,
) -> Message:
    icon = ":white_check_mark:" if passed else ":x:"
    status = "passed" if passed else "failed"

    return Message(
        channel=channel,
        text=f"Build {status} for {project}@{branch}",  # plain-text fallback
        blocks=[
            HeaderBlock(f"{icon} {project} build {status}"),
            SectionBlock(
                fields=[
                    f"*Branch*\n`{branch}`",
                    f"*Commit*\n`{commit[:7]}`",
                    f"*Author*\n{author}",
                    f"*Duration*\n{duration}",
                ],
            ),
            DividerBlock(),
            ContextBlock(
                elements=[
                    Text(f":calendar: Triggered by push to `{branch}`"),
                ],
            ),
        ],
    )
```

Tips:

- Always pass a `text=` fallback to `Message` — Slack uses it for desktop and mobile push notifications.
- `SectionBlock` accepts up to 10 `fields`, each up to 2,000 characters.

## Approval request with action buttons

A two-button approval flow. The buttons carry `value` and `action_id` so your interaction handler can identify them.

```python
from slackblocks import (
    ActionsBlock,
    Button,
    ContextBlock,
    HeaderBlock,
    Message,
    SectionBlock,
    Text,
)


def approval_request(*, channel: str, request_id: str, requester: str, summary: str) -> Message:
    return Message(
        channel=channel,
        text=f"Approval requested by {requester}",
        blocks=[
            HeaderBlock(":hand: Approval needed"),
            SectionBlock(text=f"*Requested by:* {requester}\n\n{summary}"),
            ActionsBlock(
                block_id=f"approval:{request_id}",
                elements=[
                    Button(
                        text="Approve",
                        action_id="approve",
                        value=request_id,
                        style="primary",
                    ),
                    Button(
                        text="Reject",
                        action_id="reject",
                        value=request_id,
                        style="danger",
                    ),
                    Button(
                        text="View details",
                        action_id="details",
                        url=f"https://approvals.example.com/{request_id}",
                    ),
                ],
            ),
            ContextBlock(
                elements=[Text(f"Request ID: `{request_id}`")],
            ),
        ],
    )
```

When the user clicks a button, Slack will POST an interaction payload to your app. Match on `action_id` (`approve` / `reject` / `details`) and pull the request ID from `block_id` or the button's `value`.

## Rich-formatted alert

Use `RichTextBlock` when you want inline bold/italic/code styling without writing markdown by hand.

```python
from slackblocks import (
    Message,
    RichText,
    RichTextBlock,
    RichTextLink,
    RichTextSection,
)


def deploy_alert(*, channel: str, service: str, version: str, dashboard_url: str) -> Message:
    return Message(
        channel=channel,
        text=f"Deployed {service} {version}",
        blocks=[
            RichTextBlock(
                RichTextSection(
                    elements=[
                        RichText(text="Deployed "),
                        RichText(text=service, bold=True),
                        RichText(text=" version "),
                        RichText(text=version, code=True),
                        RichText(text=". "),
                        RichTextLink(url=dashboard_url, text="View dashboard"),
                        RichText(text="."),
                    ],
                ),
            ),
        ],
    )
```

## Confirmation modal

A modal is opened in response to an interaction `trigger_id` from Slack.

```python
from slack_sdk import WebClient
from slackblocks import (
    InputBlock,
    Modal,
    PlainTextInput,
    SectionBlock,
)


def open_confirmation_modal(client: WebClient, trigger_id: str, target: str) -> None:
    modal = Modal(
        title="Confirm deletion",
        submit="Delete",
        close="Cancel",
        callback_id="confirm_delete",
        private_metadata=target,  # round-tripped back to your handler on submit
        blocks=[
            SectionBlock(f":warning: This will permanently delete *{target}*."),
            InputBlock(
                label="Type the name to confirm",
                element=PlainTextInput(
                    action_id="confirmation_text",
                    placeholder=target,
                ),
                block_id="confirmation",
            ),
        ],
    )

    client.views_open(trigger_id=trigger_id, view=modal.to_dict())
```

When the user submits, your `view_submission` handler will receive the input under `state.values["confirmation"]["confirmation_text"]`.

## Threaded reply

Pass `thread_ts` (the timestamp of the parent message) to reply in-thread.

```python
from slackblocks import Message, SectionBlock

reply = Message(
    channel="#general",
    thread_ts="1700000000.000100",
    blocks=SectionBlock("Reticulating splines... 30% complete."),
)
```

## Ephemeral slash-command response

Reply to a slash command with a message only the invoking user sees:

```python
from slackblocks import MessageResponse, SectionBlock

body = MessageResponse(
    blocks=SectionBlock(":mag: Searching... I'll DM you when I'm done."),
    ephemeral=True,
).json()
# return `body` as the JSON response to Slack's slash-command webhook
```

## Multi-column status report (table)

`TableBlock` is useful for compact tabular output.

```python
from slackblocks import (
    ColumnSettings,
    Message,
    RawText,
    RichText,
    RichTextSection,
    TableBlock,
)


def header_cell(text: str) -> RichTextSection:
    return RichTextSection(elements=[RichText(text=text, bold=True)])


report = Message(
    channel="#ops",
    text="Daily service report",
    blocks=[
        TableBlock(
            column_settings=[
                ColumnSettings(align="left"),
                ColumnSettings(align="right"),
                ColumnSettings(align="right"),
            ],
            rows=[
                [header_cell("Service"), header_cell("p99 (ms)"), header_cell("Errors")],
                [RawText("api"),     RawText("142"), RawText("3")],
                [RawText("worker"),  RawText("89"),  RawText("0")],
                [RawText("billing"), RawText("310"), RawText("12")],
            ],
        ),
    ],
)
```

## Deprecated: colored-bar attachment

Slack has long deprecated message attachments, but they're still the only way to get the colored vertical bar on the left of a message. `slackblocks` supports them when you really need that styling:

```python
from slackblocks import (
    Attachment,
    Color,
    Message,
    SectionBlock,
)

msg = Message(
    channel="#alerts",
    text="Disk usage warning",
    attachments=[
        Attachment(
            color=Color.YELLOW,
            blocks=[SectionBlock(":warning: `/var` is at 87% capacity on `prod-db-1`.")],
        ),
    ],
)
```

Prefer plain blocks where possible — Slack may drop attachment support in the future.


## Preview a message in the browser

Use `block_kit_builder_url` to build a [Block Kit Builder](https://app.slack.com/block-kit-builder) URL and open it in your browser to verify a message before posting it. Useful while iterating on layout — no Slack credentials needed.

```python
from slackblocks import (
    block_kit_builder_url,
    DividerBlock,
    HeaderBlock,
    Markdown,
    SectionBlock,
)

blocks = [
    HeaderBlock("Build #482 passed :white_check_mark:"),
    SectionBlock(text=Markdown("All 1,247 tests green in 3m 12s.")),
    DividerBlock(),
    SectionBlock(fields=["*Author*\n@nick", "*Branch*\n`main`"]),
]

print(block_kit_builder_url(blocks))
# https://app.slack.com/block-kit-builder/#%7B%22blocks%22%3A%5B...%5D%7D
```

`block_kit_builder_url` accepts:

- A single `Block`, `Element`, or anything else with a `_resolve()` method.
- A list of `Block` objects (wrapped in `{"blocks": [...]}` automatically).
- A `Message`, `WebhookMessage`, `MessageResponse`, or `View`.
- A raw `dict` (escape hatch).

Pass `team_id="T0123ABCD"` for a workspace-specific URL.


## Parse incoming Slack JSON

If your app handles Slack interactivity payloads or events, you can parse the JSON back into `slackblocks` objects with `Block.from_dict`:

```python
import json
from slackblocks import Block, SectionBlock

incoming = json.loads(slack_request_body)

for raw_block in incoming.get("blocks", []):
    block = Block.from_dict(raw_block)
    if isinstance(block, SectionBlock):
        print("Section text:", block.text.text if block.text else None)
```

`Block.from_dict` reads `data["type"]` and dispatches to the right subclass. The composition objects (`Text`, `Option`, `Confirm`, etc.) round-trip fully via their own `from_dict` classmethods.

In the current release, blocks containing interactive elements (`ActionsBlock`, `InputBlock`, `SectionBlock` with an `accessory`, `ContextBlock` with image elements) raise `NotImplementedError` — this is on the roadmap for a follow-up release.


## One-line workflow trigger

`Workflow.from_url` collapses the most common workflow construction:

```python
from slackblocks import (
    ActionsBlock,
    Message,
    SectionBlock,
    Workflow,
    WorkflowButton,
)

msg = Message(
    channel="#release",
    blocks=[
        SectionBlock("Run the release workflow with the parameters below."),
        ActionsBlock(
            elements=[
                WorkflowButton(
                    text="Release",
                    workflow=Workflow.from_url(
                        "https://slack.com/shortcuts/Ft012KXZK1MZ/...",
                        env="prod",
                        retries="3",
                    ),
                ),
            ],
        ),
    ],
)
```

Each keyword pair becomes one `InputParameter` on the underlying `Trigger`. Calling `Workflow.from_url(url)` with no parameters omits the `customizable_input_parameters` field.


## Typed exception handling

`InvalidUsageError` is the base type for every validation failure raised by `slackblocks`. As of `2.x`, five subclasses let you `except` for specific failure categories instead of string-matching the message:

```python
from slackblocks import (
    Button,
    LengthError,
    MutualExclusivityError,
    SlackFile,
    Image,
)

try:
    Button(text="x" * 100, action_id="b")
except LengthError as e:
    # specifically a length violation, not a missing-required or type-mismatch
    log.warning("button text too long: %s", e)

try:
    Image(
        image_url="https://x.png",
        slack_file=SlackFile(url="https://y.png", id=None),
    )
except MutualExclusivityError as e:
    log.error("image source ambiguous: %s", e)
```

Existing `except InvalidUsageError` blocks continue to catch every subclass — the new types are purely additive.
