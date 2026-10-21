import com.slack.api.model.block.element.BlockElement;
import io.github.nicklambourne.slackblocks.block.*;
import io.github.nicklambourne.slackblocks.element.*;
import io.github.nicklambourne.slackblocks.object.Option;
import io.github.nicklambourne.slackblocks.payload.MessagePayload;
import io.github.nicklambourne.slackblocks.payload.ModalView;

public class Multiselect {
  private static <E extends BlockElement & InputElement> void emit(String family, E element) {
    for (String parent : new String[] {"actions", "section", "input"}) {
      String payload;
      if (parent.equals("input")) {
        payload = ModalView.builder().title("Multi-select probe").submit("Save")
            .blocks(InputBlock.builder().label("Choose values").element(element).blockId("choices").build()).build().toJson();
      } else {
        var block = parent.equals("actions")
            ? ActionsBlock.builder().elements(element).blockId("choices").build()
            : SectionBlock.builder().text("Choose values").accessory(element).blockId("choices").build();
        payload = MessagePayload.builder().channel("C0123456").text("Choose values").blocks(block).build().toJson();
      }
      // Envelope only: payload is the native library's exact serialized output.
      System.out.println("{\"family\":\"" + family + "\",\"parent\":\"" + parent
          + "\",\"surface\":\"" + (parent.equals("input") ? "modal" : "message") + "\",\"payload\":" + payload + "}");
    }
  }
  public static void main(String[] args) {
    emit("static", StaticMultiSelectElement.builder().actionId("choose").options(Option.builder().text("One").value("one").build()).build());
    emit("external", ExternalMultiSelectElement.builder().actionId("choose").minQueryLength(0).build());
    emit("users", UserMultiSelectElement.builder().actionId("choose").build());
    emit("conversations", ConversationMultiSelectElement.builder().actionId("choose").build());
    emit("channels", ChannelMultiSelectElement.builder().actionId("choose").build());
  }
}
