use slackblocks::*;
use std::error::Error;

fn emit<E: Clone + Into<Element> + Into<InputElement>>(
    family: &str,
    element: E,
) -> Result<(), Box<dyn Error>> {
    for parent in ["actions", "section", "input"] {
        let payload = if parent == "input" {
            serde_json::to_value(
                ModalView::builder()
                    .title("Multi-select probe")
                    .submit("Save")
                    .blocks([InputBlock::builder()
                        .label("Choose values")
                        .element(element.clone())
                        .block_id("choices")
                        .build()?])
                    .build()?,
            )?
        } else {
            let block: Block = if parent == "actions" {
                ActionsBlock::builder()
                    .elements([element.clone()])
                    .block_id("choices")
                    .build()?
                    .into()
            } else {
                SectionBlock::builder()
                    .text("Choose values")
                    .accessory(element.clone())
                    .block_id("choices")
                    .build()?
                    .into()
            };
            serde_json::to_value(
                MessagePayload::builder("C0123456")
                    .text("Choose values")
                    .blocks([block])
                    .build()?,
            )?
        };
        println!(
            "{}",
            serde_json::json!({"family": family, "parent": parent, "surface": if parent == "input" {"modal"} else {"message"}, "payload": payload})
        );
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    emit(
        "static",
        StaticMultiSelectElement::builder()
            .action_id("choose")
            .options([SelectOption::builder().text("One").value("one").build()?])
            .build()?,
    )?;
    emit(
        "external",
        ExternalMultiSelectElement::builder()
            .action_id("choose")
            .min_query_length(0)
            .build()?,
    )?;
    emit(
        "users",
        UserMultiSelectElement::builder()
            .action_id("choose")
            .build()?,
    )?;
    emit(
        "conversations",
        ConversationMultiSelectElement::builder()
            .action_id("choose")
            .build()?,
    )?;
    emit(
        "channels",
        ChannelMultiSelectElement::builder()
            .action_id("choose")
            .build()?,
    )?;
    Ok(())
}
