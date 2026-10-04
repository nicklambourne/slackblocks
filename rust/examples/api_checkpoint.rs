//! Compile-tested API design checkpoint, using only the external crate facade.
use serde_json::{Value, json};
use slackblocks::{
    Accordion, AccordionSection, Block, ButtonElement, DataTableBlock, ErrorCategory, JsonNumber,
    MarkdownText, MessagePayload, PlainText, PlanBlock, RawNumber, RawText, RichTextBlock,
    RichTextList, RichTextListStyle, RichTextSection, RichTextStyle, RichTextText, SectionBlock,
    SelectOption, TableBlock, TableCell, TaskCardBlock, TaskStatus,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let accordion = Accordion::new([AccordionSection::builder("Details")
        .block(
            SectionBlock::builder()
                .text("Iterator composition")
                .build()?,
        )
        .build()?])?;
    let expanded = MessagePayload::builder("C0123456")
        .blocks(accordion)
        .build()?;
    assert_eq!(expanded.blocks().map(<[Block]>::len), Some(1));
    let option = SelectOption::builder()
        .text("First option")
        .value("first")
        .build()?;
    assert_eq!(option.value(), "first");
    let section = SectionBlock::builder()
        .text("Hello, Rust!")
        .accessory(
            ButtonElement::builder()
                .text("Read more")
                .action_id("read")
                .build()?,
        )
        .block_id("hello")
        .build()?;
    let edited = section
        .clone()
        .into_builder()
        .text(PlainText::new("Updated")?)
        .build()?;
    assert_ne!(edited, section);
    let message = MessagePayload::builder("C0123456").block(section).build()?;
    assert_eq!(message.blocks().map(<[Block]>::len), Some(1));
    let encoded = serde_json::to_string(&message)?;
    assert_eq!(serde_json::from_str::<MessagePayload>(&encoded)?, message);
    let omitted = MessagePayload::try_from(json!({"channel":"C0123456"}))?;
    assert_eq!(
        serde_json::to_value(omitted.into_builder().build()?)?,
        json!({"channel":"C0123456"})
    );

    let rich = RichTextBlock::builder()
        .element(
            RichTextList::builder()
                .style(RichTextListStyle::Bullet)
                .element(
                    RichTextSection::builder()
                        .element(
                            RichTextText::new("Native ownership")?
                                .into_builder()
                                .style(RichTextStyle::new().bold(true))
                                .build()?,
                        )
                        .build()?,
                )
                .build()?,
        )
        .build()?;
    let table = TableBlock::builder()
        .row([
            TableCell::from(RawText::new("Heading")?),
            rich.clone().into(),
        ])
        .row([TableCell::from(RawText::new("Ragged rows are valid")?)])
        .build()?;
    assert_eq!(table.rows()[1].len(), 1);
    let pending = TaskCardBlock::builder()
        .task_id("review")
        .title("Review")
        .status(TaskStatus::Pending)
        .details(rich)
        .build()?;
    let error = MessagePayload::builder("C0123456")
        .block(pending.clone())
        .build()
        .unwrap_err();
    assert_eq!(error.category(), ErrorCategory::TypeMismatch);
    assert_eq!(error.path(), "MessagePayload.blocks[0].status");
    let plan = PlanBlock::builder()
        .title("Release")
        .task(pending)
        .build()?;
    let wire = serde_json::to_value(&plan)?;
    assert!(wire["tasks"][0].get("type").is_none());
    assert_eq!(PlanBlock::try_from(wire)?, plan);
    let count = JsonNumber::from(9_007_199_254_740_993_u64);
    assert_eq!(serde_json::to_string(&count)?, "9007199254740993");
    let data = DataTableBlock::builder()
        .caption("Exact counts")
        .row([RawNumber::builder().value(0_i64).text("Count").build()?])
        .row([RawNumber::builder()
            .value(count)
            .text("9007199254740993")
            .build()?])
        .build()?;
    assert_eq!(data.rows().len(), 2);
    assert!(PlainText::new("").is_err());
    assert_eq!(
        serde_json::to_value(MarkdownText::new("*native*")?)?["text"],
        Value::String("*native*".into())
    );
    assert!(std::mem::size_of::<Block>() <= 32);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<MessagePayload>();
    println!("{encoded}");
    Ok(())
}
