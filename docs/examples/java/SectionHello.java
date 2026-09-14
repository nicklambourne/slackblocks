import io.github.nicklambourne.slackblocks.block.SectionBlock;
import io.github.nicklambourne.slackblocks.payload.MessagePayload;

public final class SectionHello {
  private SectionHello() {}

  public static void main(String[] args) {
    MessagePayload payload =
        MessagePayload.builder()
            .channel("C0123456")
            .blocks(SectionBlock.builder().text("Hello from slackblocks!").blockId("hello").build())
            .build();
    System.out.println(payload.toJson());
  }
}
