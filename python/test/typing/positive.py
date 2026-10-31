from slackblocks import (
    Attachment,
    Button,
    ConfirmationDialogue,
    LayoutBlock,
    LayoutItem,
    LayoutRow,
    Message,
    MessageResponse,
    PlainText,
    RichText,
    RichTextEmoji,
    SectionBlock,
    WebhookMessage,
    Workflow,
    WorkflowButton,
)

section = SectionBlock(text=PlainText("A typed external consumer"))
message = Message(channel="C123", blocks=[section], metadata={"event_type": "typed"})
response = MessageResponse(blocks=[section])
webhook = WebhookMessage(blocks=[section])
attachment = Attachment(blocks=[section])
button = Button(text="Continue", action_id="continue")
confirmation = ConfirmationDialogue("Continue?", "Confirm action", "Yes", "No", style="danger")
rich_text = RichText("Typed text", bold=True)
emoji = RichTextEmoji("wave", skin_tone=6)
workflow_button = WorkflowButton(
    "Run", workflow=Workflow.from_url("https://slack.com/shortcuts/example"), confirm=confirmation
)

layout = LayoutBlock(
    "Grid",
    [LayoutRow([LayoutItem(section, width="wide", horizontal_alignment="end")])],
    fallback=[section],
    is_label_visible=False,
)
Message("C123", blocks=[layout])
