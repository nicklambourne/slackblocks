use slackblocks::*;
fn main() -> Result<(), ValidationError> {
    let button = ButtonElement::builder().text("Run").build()?;
    let context = ContextBlock::builder().element(MarkdownText::new("*Hello*")?).build()?;
    let input = InputBlock::builder().label("Name").element(PlainTextInputElement::builder().build()?).build()?;
    let original = MessagePayload::builder("C1").block(context).build()?;
    let changed = original.clone().into_builder().block(input).build()?;
    let _ = (button.text(), original.blocks(), changed.blocks());
    Ok(())
}
