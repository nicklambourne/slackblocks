use serde_json::{Value, json};
use slackblocks::*;
fn rich() -> RichTextBlock {
    RichTextBlock::builder()
        .element(
            RichTextSection::builder()
                .element(RichTextText::new("x").unwrap())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}
fn raw(s: &str) -> RawText {
    RawText::new(s).unwrap()
}
#[test]
fn table_contexts_and_nested_pending_tasks() -> Result<(), ValidationError> {
    assert!(
        TableBlock::builder()
            .rows([vec![raw("A")], vec![raw("B"), raw("C")]])
            .build()
            .is_ok()
    );
    assert_eq!(
        DataTableBlock::builder()
            .caption("Data")
            .rows([vec![raw("A")], vec![raw("B"), raw("C")]])
            .build()
            .unwrap_err()
            .category(),
        ErrorCategory::InvalidUsage
    );
    let cells: Vec<Vec<DataTableCell>> = vec![vec![rich().into()], vec![raw("body").into()]];
    assert_eq!(
        DataTableBlock::builder()
            .caption("Data")
            .rows(cells)
            .build()
            .unwrap_err()
            .category(),
        ErrorCategory::TypeMismatch
    );
    let table = DataTableBlock::builder()
        .caption("Data")
        .rows([vec![raw("A")], vec![raw("B")]]);
    assert!(
        table
            .clone()
            .extension("column_settings", json!([]))
            .build()
            .is_err()
    );
    let numeric = RawNumber::builder().text("").value(1_i64).build()?;
    assert!(
        DataTableBlock::builder()
            .caption("Data")
            .rows([vec![DataTableCell::from(raw("A"))], vec![numeric.into()]])
            .build()
            .is_err()
    );
    let task = TaskCardBlock::builder()
        .task_id("one")
        .title("Work")
        .status(TaskStatus::Pending)
        .build()?;
    let container = ContainerBlock::builder()
        .title("Progress")
        .child_block(task.clone())
        .build()?;
    let error = MessagePayload::builder("C1")
        .block(container)
        .build()
        .unwrap_err();
    assert!(error.path().contains("child_blocks[0].status"));
    assert!(Attachment::builder().block(task.clone()).build().is_err());
    let plan = PlanBlock::builder().title("Plan").task(task).build()?;
    let message = MessagePayload::builder("C1").block(plan).build()?;
    let json = serde_json::to_value(message).unwrap();
    assert!(json["blocks"][0]["tasks"][0].get("type").is_none());
    assert!(MessagePayload::try_from(json).is_ok());
    Ok(())
}
#[test]
fn contextual_combinations_and_chart_matching() -> Result<(), ValidationError> {
    let image = ImageElement::builder()
        .image_url("https://example.com/i.png")
        .alt_text("i")
        .build()?;
    assert!(
        CardBlock::builder()
            .title("Card")
            .icon(image)
            .slack_icon(SlackIcon::builder().name("check").build()?)
            .build()
            .is_err()
    );
    assert!(
        SlackIcon::builder()
            .name("not-a-slack-icon")
            .build()
            .is_err()
    );
    for id in ["F123", "X01234567", "F0123abcd", "F000000é"] {
        assert!(SlackFile::builder().id(id).build().is_err());
    }
    assert!(
        SlackFile::builder()
            .id("F01234567")
            .url("https://example.com/file")
            .build()
            .is_err()
    );
    assert!(SlackFile::builder().build().is_err());
    let divider = DividerBlock::builder().build()?;
    assert!(
        ContainerBlock::builder()
            .title("C")
            .child_block(divider)
            .is_collapsible(false)
            .default_collapsed(true)
            .build()
            .is_err()
    );
    assert!(
        ConversationFilter::builder()
            .include(["unknown"])
            .build()
            .is_err()
    );
    assert!(
        AxisConfig::builder()
            .categories(["A", "A"])
            .build()
            .is_err()
    );
    let series = DataSeries::builder()
        .name("Repeated")
        .point(DataPoint::builder().label("A").value(1_i64).build()?)
        .build()?;
    assert!(
        LineChart::builder()
            .axis_config(AxisConfig::builder().category("A").build()?)
            .series([series.clone(), series])
            .build()
            .is_err()
    );
    assert!(
        NumberInputElement::builder()
            .is_decimal_allowed(true)
            .min_value(f64::INFINITY)
            .build()
            .is_err()
    );
    assert!(
        NumberInputElement::builder()
            .is_decimal_allowed(true)
            .max_value(f64::NAN)
            .build()
            .is_err()
    );
    Ok(())
}
#[test]
fn rich_style_flags_and_text_coercion_are_checked() -> Result<(), ValidationError> {
    let style = RichTextStyle::new()
        .bold(false)
        .italic(true)
        .strike(false)
        .code(true)
        .highlight(false)
        .client_highlight(true)
        .unlink(false);
    assert_eq!(style.is_bold(), Some(false));
    assert_eq!(style.is_italic(), Some(true));
    assert_eq!(style.is_strike(), Some(false));
    assert_eq!(style.is_code(), Some(true));
    assert_eq!(style.is_highlight(), Some(false));
    assert_eq!(style.is_client_highlight(), Some(true));
    assert_eq!(style.is_unlink(), Some(false));
    assert_eq!(
        serde_json::from_value::<RichTextStyle>(serde_json::to_value(&style).unwrap()).unwrap(),
        style
    );
    assert!(
        RichTextText::builder()
            .text("x")
            .style(style)
            .build()
            .is_err()
    );
    assert!(RichTextStyle::try_from(json!({"unknown":true})).is_err());
    assert!(RichTextStyle::try_from(json!({"bold":"yes"})).is_err());
    let section =
        SectionBlock::try_from(json!({"type":"section","text":"Hello","fields":["World"]}))?;
    assert_eq!(
        serde_json::to_value(section).unwrap()["fields"][0]["type"],
        "mrkdwn"
    );
    let option = SelectOption::builder()
        .text("Plain by field")
        .value("value")
        .build()?;
    assert_eq!(
        serde_json::to_value(option).unwrap()["text"]["type"],
        "plain_text"
    );
    let typed = SelectOption::builder()
        .text(MarkdownText::new("*Typed markdown*")?)
        .value("value")
        .build()?;
    assert_eq!(
        serde_json::to_value(typed).unwrap()["text"]["type"],
        "mrkdwn"
    );
    assert!(PlainText::try_from(json!({"type":"plain_text","text":null})).is_err());
    assert!(
        DateTimePickerElement::try_from(
            json!({"type":"datetimepicker","initial_date_time":u64::MAX})
        )
        .is_err()
    );
    Ok(())
}
#[test]
fn extensions_cannot_override_absent_fields_and_errors_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ValidationError>();
    send_sync::<MessagePayload>();
    let button = ButtonElement::builder().text("Button");
    assert!(
        button
            .clone()
            .extension("url", "https://example.com")
            .build()
            .is_err()
    );
    assert!(button.clone().extension("", true).build().is_err());
    let value = button
        .extension(
            "future",
            json!({"enabled":false,"count":0,"items":[],"text":"","null":null}),
        )
        .build()
        .unwrap();
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(ButtonElement::try_from(json).unwrap(), value);
    let error = PlainText::new("").unwrap_err();
    assert_eq!(error.category().to_string(), "length-exceeded");
    assert!(error.to_string().contains(error.path()));
    assert!(serde_json::from_value::<Block>(Value::Null).is_err());
}
