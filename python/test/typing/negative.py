from slackblocks import (
    ConfirmationDialogue,
    Message,
    RichText,
    RichTextEmoji,
    SectionBlock,
    WorkflowButton,
)

Message(channel=42)
RichText(text=42)
SectionBlock(text=42)
ConfirmationDialogue("Proceed?", "Confirm", "Yes", "No", style="invalid")
RichTextEmoji("wave", skin_tone="six")
WorkflowButton("Run", confirm="yes")
Message("C123", metadata="invalid")
