from slackblocks import SectionBlock


def basic_section_block_test() -> None:
    block = SectionBlock("Hello, world!")
    with open("test/samples/section_block", "r") as sample:
        assert sample.read() == repr(block)
