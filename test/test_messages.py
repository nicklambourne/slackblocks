from slackblocks import Message, SectionBlock


def basic_message_test() -> None:
    x = SectionBlock("Hello, world!")
    m = Message(blocks=x)
