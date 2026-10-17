from slackblocks import (
    Attachment,
    Button,
    ConfirmationDialogue,
    Message,
    MessageResponse,
    PlainText,
    RichText,
    SectionBlock,
    WebhookMessage,
)

section = SectionBlock(text=PlainText("A typed external consumer"))
message = Message(channel="C123", blocks=[section])
response = MessageResponse(blocks=[section])
webhook = WebhookMessage(blocks=[section])
attachment = Attachment(blocks=[section])
button = Button(text="Continue", action_id="continue")
confirmation = ConfirmationDialogue("Continue?", "Confirm action", "Yes", "No")
rich_text = RichText("Typed text", bold=True)
