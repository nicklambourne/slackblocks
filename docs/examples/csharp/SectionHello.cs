using System;
using Slackblocks.Blocks;
using Slackblocks.Payloads;

public static class SectionHello
{
    public static void Main()
    {
        var payload = new MessagePayload(
            "C0123456",
            blocks: [new SectionBlock(text: "Hello from slackblocks!", blockId: "hello")]);
        Console.WriteLine(payload.ToJson());
    }
}
