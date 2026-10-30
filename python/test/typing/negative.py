from slackblocks import (
    ActionsBlock,
    ConfirmationDialogue,
    DividerBlock,
    LayoutBlock,
    LayoutItem,
    LayoutRow,
    MarkdownBlock,
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

layout = LayoutBlock("Grid", [LayoutRow([LayoutItem(DividerBlock())])])
LayoutItem(layout)
LayoutItem(MarkdownBlock("No"))
LayoutItem(DividerBlock(), width="full")
actions = [ActionsBlock([])]
LayoutBlock("Grid", layout.rows, fallback=actions)
