use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use slackblocks::*;

fn roundtrip<T: Serialize + DeserializeOwned + TryFrom<Value, Error = ValidationError>>(value: &T) {
    let wire = serde_json::to_value(value).unwrap();
    let parsed = T::try_from(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
    let parsed: T = serde_json::from_str(&serde_json::to_string(value).unwrap()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), wire);
}

fn opaque() -> Value {
    json!({
        "type": "markdown",
        "rows": [{"type":"data_table"}],
        "tasks": [{"type":"task_card", "status":"pending"}],
        "nested": [{"type":"markdown", "text":"x".repeat(12001)}]
    })
}

#[test]
fn opaque_metadata_and_extensions_do_not_participate_in_block_validation()
-> Result<(), ValidationError> {
    let metadata = json!({"event_type":"audit", "event_payload":opaque()})
        .as_object()
        .unwrap()
        .clone();
    let section = SectionBlock::builder()
        .text("hello")
        .extension("audit", opaque())
        .build()?;
    let attachment = Attachment::builder()
        .block(section.clone())
        .extension("audit", opaque())
        .build()?;
    let message = MessagePayload::builder("C01234567")
        .block(section.clone())
        .attachment(attachment.clone())
        .metadata(metadata.clone())
        .extension("audit", opaque())
        .build()?;
    roundtrip(&message);
    roundtrip(
        &WebhookMessage::builder()
            .block(section.clone())
            .attachment(attachment.clone())
            .metadata(metadata)
            .extension("audit", opaque())
            .build()?,
    );
    roundtrip(
        &MessageResponse::builder()
            .block(section.clone())
            .attachment(attachment)
            .extension("audit", opaque())
            .build()?,
    );
    roundtrip(
        &ModalView::builder()
            .title("Modal")
            .block(section.clone())
            .extension("audit", opaque())
            .build()?,
    );
    roundtrip(
        &HomeTabView::builder()
            .block(section)
            .extension("audit", opaque())
            .build()?,
    );
    Ok(())
}

#[test]
fn table_totals_count_modeled_cell_text_only() -> Result<(), ValidationError> {
    let cell = RawText::builder()
        .text("cell")
        .extension("audit", opaque())
        .build()?;
    let table = DataTableBlock::builder()
        .caption("Data")
        .rows([vec![cell.clone()], vec![cell]])
        .extension("audit", opaque())
        .build()?;
    roundtrip(&table);
    roundtrip(&MessagePayload::builder("C1").block(table).build()?);
    Ok(())
}

#[test]
fn numeric_literals_never_silently_round() {
    for literal in [
        "18446744073709551617",
        "-9223372036854775809",
        "1.2345678901234567890123456789",
        "1e400",
        "1e-400",
    ] {
        assert!(
            serde_json::from_str::<JsonNumber>(literal).is_err(),
            "{literal}"
        );
        assert!(
            serde_json::from_slice::<JsonNumber>(literal.as_bytes()).is_err(),
            "{literal}"
        );
        let raw = format!(r#"{{"type":"raw_number","text":"total","value":{literal}}}"#);
        assert!(
            serde_json::from_str::<RawNumber>(&raw).is_err(),
            "{literal}"
        );
        assert!(
            serde_json::from_slice::<RawNumber>(raw.as_bytes()).is_err(),
            "{literal}"
        );
        let point = format!(r#"{{"label":"A","value":{literal}}}"#);
        assert!(
            serde_json::from_str::<DataPoint>(&point).is_err(),
            "{literal}"
        );
    }
    for literal in [
        "0",
        "-0.0",
        "1.5",
        "1e3",
        "1e-3",
        "9007199254740993",
        "-9223372036854775808",
        "18446744073709551615",
    ] {
        let value = serde_json::from_str::<JsonNumber>(literal).unwrap();
        let parsed = serde_json::from_slice::<JsonNumber>(literal.as_bytes()).unwrap();
        assert_eq!(value, parsed);
        assert_eq!(
            serde_json::from_str::<JsonNumber>(&serde_json::to_string(&value).unwrap()).unwrap(),
            value
        );
    }
}

#[test]
fn borrowed_strings_keep_contextual_text_coercion() -> Result<(), ValidationError> {
    let label = String::from("hello");
    let plain = ButtonElement::builder().text(&label).build()?;
    let markdown = SectionBlock::builder().text(&label).build()?;
    assert_eq!(plain.text().text(), label);
    assert!(matches!(markdown.text(), Some(Text::Markdown(_))));
    Ok(())
}

#[test]
fn rich_table_text_boundaries_include_each_modeled_rich_text_kind() -> Result<(), ValidationError> {
    let elements: Vec<RichTextSectionElement> = vec![
        RichTextText::builder()
            .text("🦀".repeat(4998))
            .extension("audit", opaque())
            .build()?
            .into(),
        RichTextLink::builder()
            .url("https://example.com")
            .text("l")
            .build()?
            .into(),
    ];
    let section = RichTextSection::builder()
        .elements(elements.clone())
        .build()?;
    let rich = RichTextBlock::builder()
        .elements([
            RichTextBlockElement::from(section.clone()),
            RichTextQuote::builder()
                .elements(elements.clone())
                .build()?
                .into(),
            RichTextCodeBlock::builder()
                .elements(elements)
                .build()?
                .into(),
            RichTextList::builder()
                .style(RichTextListStyle::Bullet)
                .element(section)
                .build()?
                .into(),
        ])
        .build()?;
    let rows = vec![
        vec![DataTableCell::from(RawText::new("head")?)],
        vec![rich.clone().into()],
    ];
    let table = DataTableBlock::builder()
        .caption("Data")
        .rows(rows)
        .build()?;
    roundtrip(&table);
    let overflow = rich
        .into_builder()
        .element(
            RichTextSection::builder()
                .element(RichTextText::new("x")?)
                .build()?,
        )
        .build()?;
    let error = DataTableBlock::builder()
        .caption("Data")
        .rows(vec![
            vec![DataTableCell::from(RawText::new("head")?)],
            vec![overflow.into()],
        ])
        .build()
        .unwrap_err();
    assert_eq!(error.category(), ErrorCategory::LengthExceeded);
    assert_eq!(error.path(), "DataTableBlock.rows");
    Ok(())
}

#[test]
fn nested_blocks_and_attachments_still_contribute_to_message_totals() -> Result<(), ValidationError>
{
    let markdown = MarkdownBlock::builder().text("x".repeat(6001)).build()?;
    let container = ContainerBlock::builder()
        .title("Nested")
        .child_block(markdown.clone())
        .build()?;
    let attachment = Attachment::builder().block(markdown).build()?;
    let error = MessagePayload::builder("C1")
        .block(container)
        .attachment(attachment)
        .build()
        .unwrap_err();
    assert_eq!(error.category(), ErrorCategory::LengthExceeded);
    assert_eq!(error.path(), "MessagePayload");
    let pending = TaskCardBlock::builder()
        .task_id("one")
        .title("Work")
        .status(TaskStatus::Pending)
        .build()?;
    let error = MessagePayload::builder("C1")
        .block(
            ContainerBlock::builder()
                .title("Tasks")
                .child_block(pending)
                .build()?,
        )
        .build()
        .unwrap_err();
    assert_eq!(error.category(), ErrorCategory::TypeMismatch);
    assert_eq!(
        error.path(),
        "MessagePayload.blocks[0].child_blocks[0].status"
    );
    Ok(())
}
