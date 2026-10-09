// Public role and vocabulary round trips; hand-maintained.
use serde_json::{Value, json};
use slackblocks::*;
#[test]
fn alert_level_vocabulary() {
    for (value, wire) in [
        (AlertLevel::Default, "default"),
        (AlertLevel::Info, "info"),
        (AlertLevel::Warning, "warning"),
        (AlertLevel::Error, "error"),
        (AlertLevel::Success, "success"),
    ] {
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<AlertLevel>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<AlertLevel>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(AlertLevel::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<AlertLevel>().is_err());
}
#[test]
fn container_width_vocabulary() {
    for (value, wire) in [
        (ContainerWidth::Narrow, "narrow"),
        (ContainerWidth::Standard, "standard"),
        (ContainerWidth::Wide, "wide"),
        (ContainerWidth::Full, "full"),
    ] {
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<ContainerWidth>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<ContainerWidth>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(ContainerWidth::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<ContainerWidth>().is_err());
}
#[test]
fn task_status_vocabulary() {
    for (value, wire) in [
        (TaskStatus::Pending, "pending"),
        (TaskStatus::InProgress, "in_progress"),
        (TaskStatus::Complete, "complete"),
        (TaskStatus::Error, "error"),
    ] {
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<TaskStatus>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<TaskStatus>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(TaskStatus::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<TaskStatus>().is_err());
}
#[test]
fn button_style_vocabulary() {
    for (value, wire) in [
        (ButtonStyle::Primary, "primary"),
        (ButtonStyle::Danger, "danger"),
    ] {
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<ButtonStyle>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<ButtonStyle>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(ButtonStyle::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<ButtonStyle>().is_err());
}
#[test]
fn icon_button_icon_vocabulary() {
    {
        let (value, wire) = (IconButtonIcon::Trash, "trash");
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<IconButtonIcon>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<IconButtonIcon>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(IconButtonIcon::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<IconButtonIcon>().is_err());
}
#[test]
fn column_align_vocabulary() {
    for (value, wire) in [
        (ColumnAlign::Left, "left"),
        (ColumnAlign::Center, "center"),
        (ColumnAlign::Right, "right"),
    ] {
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<ColumnAlign>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<ColumnAlign>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(ColumnAlign::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<ColumnAlign>().is_err());
}
#[test]
fn rich_text_list_style_vocabulary() {
    for (value, wire) in [
        (RichTextListStyle::Bullet, "bullet"),
        (RichTextListStyle::Ordered, "ordered"),
    ] {
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<RichTextListStyle>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<RichTextListStyle>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(RichTextListStyle::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<RichTextListStyle>().is_err());
}
#[test]
fn response_type_vocabulary() {
    for (value, wire) in [
        (ResponseType::InChannel, "in_channel"),
        (ResponseType::Ephemeral, "ephemeral"),
    ] {
        assert_eq!(value.as_str(), wire);
        assert_eq!(value.to_string(), wire);
        assert_eq!(wire.parse::<ResponseType>().unwrap(), value);
        assert_eq!(
            serde_json::from_value::<ResponseType>(json!(wire)).unwrap(),
            value
        );
        assert_eq!(serde_json::to_value(value).unwrap(), json!(wire));
    }
    assert!(ResponseType::try_from(Value::Null).is_err());
    assert!("not-a-variant".parse::<ResponseType>().is_err());
}
#[test]
fn block_role() -> Result<(), ValidationError> {
    let concrete = ActionsBlock::builder()
        .block_id("fake_block_id")
        .elements(vec![Element::from(
            CheckboxesElement::builder()
                .action_id("actionId-0")
                .options(vec![
                    SelectOption::builder()
                        .text(MarkdownText::builder().text("*a*").build()?)
                        .value("a")
                        .description(PlainText::builder().text("*a*").build()?)
                        .build()?,
                    SelectOption::builder()
                        .text(MarkdownText::builder().text("*b*").build()?)
                        .value("b")
                        .description(PlainText::builder().text("*b*").build()?)
                        .build()?,
                    SelectOption::builder()
                        .text(MarkdownText::builder().text("*c*").build()?)
                        .value("c")
                        .description(PlainText::builder().text("*c*").build()?)
                        .build()?,
                ])
                .build()?,
        )])
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = AlertBlock::builder()
        .block_id("fake_block_id")
        .text(
            MarkdownText::builder()
                .text("The work is mysterious and important.")
                .build()?,
        )
        .level(AlertLevel::Info)
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = CardBlock::builder()
        .block_id("fake_block_id")
        .hero_image(
            ImageElement::builder()
                .image_url("https://picsum.photos/400/300")
                .alt_text("Sample hero image")
                .build()?,
        )
        .title(MarkdownText::builder().text("Lumon Industries").build()?)
        .subtitle(
            MarkdownText::builder()
                .text("Committed to work-life balance")
                .build()?,
        )
        .body(
            MarkdownText::builder()
                .text("Please enjoy each card equally.")
                .build()?,
        )
        .actions(vec![
            ButtonElement::builder()
                .text(PlainText::builder().text("Action Button").build()?)
                .action_id("button_action")
                .build()?,
        ])
        .slack_icon(SlackIcon::builder().name("bot").build()?)
        .subtext(
            MarkdownText::builder()
                .text("A card assembled by slackblocks.")
                .build()?,
        )
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = CarouselBlock::builder()
        .block_id("fake_block_id")
        .elements(vec![
            CardBlock::builder()
                .block_id("card_1")
                .title(MarkdownText::builder().text("First result").build()?)
                .build()?,
            CardBlock::builder()
                .block_id("card_2")
                .title(MarkdownText::builder().text("Second result").build()?)
                .build()?,
        ])
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = ContainerBlock::builder()
        .block_id("fake_block_id")
        .title(PlainText::builder().text("Deployment summary").build()?)
        .subtitle(
            MarkdownText::builder()
                .text("Production is healthy")
                .build()?,
        )
        .child_blocks(vec![Block::from(
            SectionBlock::builder()
                .block_id("child_1")
                .text(
                    MarkdownText::builder()
                        .text("All systems operational.")
                        .build()?,
                )
                .build()?,
        )])
        .width(ContainerWidth::Standard)
        .is_collapsible(false)
        .default_collapsed(false)
        .has_header_divider(true)
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = ContextActionsBlock::builder()
        .block_id("fake_block_id")
        .elements(vec![ContextActionsElement::from(
            FeedbackButtonsElement::builder()
                .positive_button(
                    FeedbackButton::builder()
                        .text(PlainText::builder().text("Good").build()?)
                        .value("positive_feedback")
                        .accessibility_label("Mark this response as good")
                        .build()?,
                )
                .negative_button(
                    FeedbackButton::builder()
                        .text(PlainText::builder().text("Bad").build()?)
                        .value("negative_feedback")
                        .accessibility_label("Mark this response as bad")
                        .build()?,
                )
                .action_id("feedback_buttons_1")
                .build()?,
        )])
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = ContextBlock::builder()
        .block_id("fake_block_id")
        .elements(vec![ContextElement::from(
            MarkdownText::builder().text("Hello, world!").build()?,
        )])
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = DataTableBlock::builder()
        .block_id("fake_block_id")
        .rows(vec![
            vec![
                DataTableCell::from(RawText::builder().text("Name").build()?),
                DataTableCell::from(RawText::builder().text("Score").build()?),
            ],
            vec![
                DataTableCell::from(RawText::builder().text("Alice").build()?),
                DataTableCell::from(RawNumber::builder().value(42_i64).text("42").build()?),
            ],
        ])
        .page_size(5_i64)
        .caption("Team scores")
        .row_header_column_index(0_i64)
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = DataVisualizationBlock::builder()
        .block_id("fake_block_id")
        .title("Daily Active Users")
        .chart(
            AreaChart::builder()
                .series(vec![
                    DataSeries::builder()
                        .name("Free Tier")
                        .data(vec![
                            DataPoint::builder().label("Mon").value(12000_i64).build()?,
                            DataPoint::builder().label("Tue").value(13500_i64).build()?,
                        ])
                        .build()?,
                ])
                .axis_config(
                    AxisConfig::builder()
                        .categories(vec![String::from("Mon"), String::from("Tue")])
                        .x_label("Day")
                        .y_label("Users")
                        .build()?,
                )
                .build()?,
        )
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = DividerBlock::builder().block_id("fake_block_id").build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = FileBlock::builder()
        .external_id("external_id")
        .source("remote")
        .block_id("fake_block_id")
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = HeaderBlock::builder()
        .block_id("fake_block_id")
        .text(PlainText::builder().text("😀".repeat(150)).build()?)
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = ImageBlock::builder()
        .block_id("fake_block_id")
        .image_url("https://api.slack.com/img/blocks/bkb_template_images/beagle.png")
        .alt_text("image1")
        .title(PlainText::builder().text("image1").build()?)
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = InputBlock::builder()
        .block_id("fake_block_id")
        .label(PlainText::builder().text("Label").emoji(true).build()?)
        .element(
            PlainTextInputElement::builder()
                .action_id("action")
                .build()?,
        )
        .hint(PlainText::builder().text("Hint").emoji(true).build()?)
        .optional(true)
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = MarkdownBlock::builder()
        .block_id("fake_block_id")
        .text("**Hello**, _world_!")
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = PlanBlock::builder()
        .block_id("fake_block_id")
        .title("Thinking completed")
        .tasks(vec![
            TaskCardBlock::builder()
                .task_id("call_001")
                .title("Fetched user profile information")
                .status(TaskStatus::Complete)
                .output(
                    RichTextBlock::builder()
                        .block_id("plan_output")
                        .elements(vec![RichTextBlockElement::from(
                            RichTextSection::builder()
                                .elements(vec![RichTextSectionElement::from(
                                    RichTextText::builder()
                                        .text("Profile data loaded")
                                        .build()?,
                                )])
                                .build()?,
                        )])
                        .build()?,
                )
                .build()?,
            TaskCardBlock::builder()
                .task_id("call_002")
                .title("Checked user permissions")
                .status(TaskStatus::Pending)
                .build()?,
        ])
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = RichTextBlock::builder()
        .block_id("plan_output")
        .elements(vec![RichTextBlockElement::from(
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder()
                        .text("Profile data loaded")
                        .build()?,
                )])
                .build()?,
        )])
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = SectionBlock::builder()
        .block_id("fake_block_id_0")
        .text(
            MarkdownText::builder()
                .text("I like pretty colours")
                .build()?,
        )
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = TableBlock::builder()
        .block_id("fake_block_id")
        .rows(vec![
            vec![
                TableCell::from(RawText::builder().text("Header A").build()?),
                TableCell::from(RawText::builder().text("Header B").build()?),
            ],
            vec![
                TableCell::from(RawText::builder().text("Data 1A").build()?),
                TableCell::from(
                    RichTextBlock::builder()
                        .elements(vec![RichTextBlockElement::from(
                            RichTextSection::builder()
                                .elements(vec![RichTextSectionElement::from(
                                    RichTextLink::builder()
                                        .url("https://slack.com")
                                        .text("Data 1B")
                                        .build()?,
                                )])
                                .build()?,
                        )])
                        .build()?,
                ),
            ],
            vec![
                TableCell::from(RawText::builder().text("Data 2A").build()?),
                TableCell::from(
                    RichTextBlock::builder()
                        .elements(vec![RichTextBlockElement::from(
                            RichTextSection::builder()
                                .elements(vec![RichTextSectionElement::from(
                                    RichTextLink::builder()
                                        .url("https://slack.com")
                                        .text("Data 2B")
                                        .build()?,
                                )])
                                .build()?,
                        )])
                        .build()?,
                ),
            ],
        ])
        .column_settings(vec![
            ColumnSettings::builder().is_wrapped(true).build()?,
            ColumnSettings::builder()
                .align(ColumnAlign::Right)
                .build()?,
        ])
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = TaskCardBlock::builder()
        .task_id("call_001")
        .title("Fetched user profile information")
        .status(TaskStatus::Complete)
        .output(
            RichTextBlock::builder()
                .block_id("plan_output")
                .elements(vec![RichTextBlockElement::from(
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder()
                                .text("Profile data loaded")
                                .build()?,
                        )])
                        .build()?,
                )])
                .build()?,
        )
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    let concrete = VideoBlock::builder()
        .block_id("b1")
        .alt_text("alt")
        .thumbnail_url("https://example.com/t.png")
        .title(PlainText::builder().text("Title").build()?)
        .video_url("https://example.com/v.mp4")
        .build()?;
    let value = Block::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Block::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Block>(json).unwrap(), value);
    assert!(Block::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn context_element_role() -> Result<(), ValidationError> {
    let concrete = ImageElement::builder()
        .image_url("https://picsum.photos/400/300")
        .alt_text("Sample hero image")
        .build()?;
    let value = ContextElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(ContextElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<ContextElement>(json).unwrap(),
        value
    );
    let concrete = MarkdownText::builder()
        .text("I like pretty colours")
        .build()?;
    let value = ContextElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(ContextElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<ContextElement>(json).unwrap(),
        value
    );
    let concrete = PlainText::builder().text("*a*").build()?;
    let value = ContextElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(ContextElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<ContextElement>(json).unwrap(),
        value
    );
    assert!(ContextElement::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn context_actions_element_role() -> Result<(), ValidationError> {
    let concrete = FeedbackButtonsElement::builder()
        .positive_button(
            FeedbackButton::builder()
                .text(PlainText::builder().text("Good").build()?)
                .value("positive_feedback")
                .accessibility_label("Mark this response as good")
                .build()?,
        )
        .negative_button(
            FeedbackButton::builder()
                .text(PlainText::builder().text("Bad").build()?)
                .value("negative_feedback")
                .accessibility_label("Mark this response as bad")
                .build()?,
        )
        .action_id("feedback_buttons_1")
        .build()?;
    let value = ContextActionsElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(ContextActionsElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<ContextActionsElement>(json).unwrap(),
        value
    );
    let concrete = IconButtonElement::builder()
        .icon(IconButtonIcon::Trash)
        .text(PlainText::builder().text("Delete").build()?)
        .action_id("delete_button")
        .value("delete_item")
        .build()?;
    let value = ContextActionsElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(ContextActionsElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<ContextActionsElement>(json).unwrap(),
        value
    );
    assert!(ContextActionsElement::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn table_cell_role() -> Result<(), ValidationError> {
    let concrete = RichTextBlock::builder()
        .block_id("plan_output")
        .elements(vec![RichTextBlockElement::from(
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder()
                        .text("Profile data loaded")
                        .build()?,
                )])
                .build()?,
        )])
        .build()?;
    let value = TableCell::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(TableCell::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<TableCell>(json).unwrap(), value);
    let concrete = RawText::builder().text("Name").build()?;
    let value = TableCell::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(TableCell::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<TableCell>(json).unwrap(), value);
    assert!(TableCell::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn data_table_cell_role() -> Result<(), ValidationError> {
    let concrete = RichTextBlock::builder()
        .block_id("plan_output")
        .elements(vec![RichTextBlockElement::from(
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder()
                        .text("Profile data loaded")
                        .build()?,
                )])
                .build()?,
        )])
        .build()?;
    let value = DataTableCell::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(DataTableCell::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<DataTableCell>(json).unwrap(),
        value
    );
    let concrete = RawNumber::builder().value(42_i64).text("42").build()?;
    let value = DataTableCell::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(DataTableCell::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<DataTableCell>(json).unwrap(),
        value
    );
    let concrete = RawText::builder().text("Name").build()?;
    let value = DataTableCell::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(DataTableCell::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<DataTableCell>(json).unwrap(),
        value
    );
    assert!(DataTableCell::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn element_role() -> Result<(), ValidationError> {
    let concrete = ButtonElement::builder()
        .text(PlainText::builder().text("Action Button").build()?)
        .action_id("button_action")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = ChannelMultiSelectElement::builder()
        .action_id("multi_channels_select")
        .placeholder(PlainText::builder().text("Select channels").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = ChannelSelectElement::builder()
        .action_id("channels_select")
        .placeholder(PlainText::builder().text("Select a channel").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = CheckboxesElement::builder()
        .action_id("actionId-0")
        .options(vec![
            SelectOption::builder()
                .text(MarkdownText::builder().text("*a*").build()?)
                .value("a")
                .description(PlainText::builder().text("*a*").build()?)
                .build()?,
            SelectOption::builder()
                .text(MarkdownText::builder().text("*b*").build()?)
                .value("b")
                .description(PlainText::builder().text("*b*").build()?)
                .build()?,
            SelectOption::builder()
                .text(MarkdownText::builder().text("*c*").build()?)
                .value("c")
                .description(PlainText::builder().text("*c*").build()?)
                .build()?,
        ])
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = ConversationMultiSelectElement::builder()
        .action_id("multi_conversations_select")
        .placeholder(PlainText::builder().text("Select conversations").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = ConversationSelectElement::builder()
        .action_id("conversations_select")
        .placeholder(
            PlainText::builder()
                .text("Select one conversation")
                .build()?,
        )
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = DatePickerElement::builder()
        .action_id("datepicker")
        .initial_date("1970-01-01")
        .placeholder(PlainText::builder().text("Pick a date").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = DateTimePickerElement::builder()
        .action_id("datetime_picker")
        .initial_date_time(1628633830_i64)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = EmailInputElement::builder()
        .action_id("email_input")
        .placeholder(PlainText::builder().text("Enter your email").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = ExternalMultiSelectElement::builder()
        .action_id("multi_external_select")
        .min_query_length(3_i64)
        .placeholder(PlainText::builder().text("Select items").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = ExternalSelectElement::builder()
        .action_id("external_select")
        .min_query_length(4_i64)
        .placeholder(PlainText::builder().text("Select one item").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = FeedbackButtonsElement::builder()
        .positive_button(
            FeedbackButton::builder()
                .text(PlainText::builder().text("Good").build()?)
                .value("positive_feedback")
                .accessibility_label("Mark this response as good")
                .build()?,
        )
        .negative_button(
            FeedbackButton::builder()
                .text(PlainText::builder().text("Bad").build()?)
                .value("negative_feedback")
                .accessibility_label("Mark this response as bad")
                .build()?,
        )
        .action_id("feedback_buttons_1")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = FileInputElement::builder()
        .action_id("file_input_action_id_1")
        .filetypes(vec![String::from("jpg"), String::from("png")])
        .max_files(5_i64)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = IconButtonElement::builder()
        .icon(IconButtonIcon::Trash)
        .text(PlainText::builder().text("Delete").build()?)
        .action_id("delete_button")
        .value("delete_item")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = ImageElement::builder()
        .image_url("https://picsum.photos/400/300")
        .alt_text("Sample hero image")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = NumberInputElement::builder()
        .is_decimal_allowed(false)
        .action_id("number_input")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = OverflowElement::builder()
        .action_id("overflow")
        .options(vec![
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("B").build()?)
                .value("B")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("C").build()?)
                .value("C")
                .build()?,
        ])
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = PlainTextInputElement::builder()
        .action_id("action")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = RadioButtonsElement::builder()
        .action_id("radio_buttons")
        .options(vec![
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("B").build()?)
                .value("B")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("C").build()?)
                .value("C")
                .build()?,
        ])
        .initial_option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = RichTextInputElement::builder()
        .action_id("action_id")
        .initial_value(
            RichTextBlock::builder()
                .elements(vec![RichTextBlockElement::from(
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder().text("I'm rich").build()?,
                        )])
                        .build()?,
                )])
                .build()?,
        )
        .focus_on_load(false)
        .placeholder(PlainText::builder().text("Hello").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = StaticMultiSelectElement::builder()
        .action_id("multi_static_select")
        .options(vec![
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("B").build()?)
                .value("B")
                .build()?,
        ])
        .placeholder(PlainText::builder().text("Select one or more").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = StaticSelectElement::builder()
        .action_id("static_select")
        .options(vec![
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("B").build()?)
                .value("B")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("C").build()?)
                .value("C")
                .build()?,
        ])
        .placeholder(PlainText::builder().text("Select one item").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = TimePickerElement::builder()
        .action_id("timepicker")
        .initial_time("12:00")
        .placeholder(PlainText::builder().text("Select your time").build()?)
        .timezone("Australia/Sydney")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = UrlInputElement::builder()
        .action_id("url_text_input")
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = UserMultiSelectElement::builder()
        .action_id("multi_users_select")
        .placeholder(
            PlainText::builder()
                .text("Select one or more users")
                .build()?,
        )
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete = UserSelectElement::builder()
        .action_id("users_select")
        .placeholder(PlainText::builder().text("Select one user").build()?)
        .build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    let concrete=WorkflowButtonElement::builder().action_id("run_workflow").text(PlainText::builder().text("Run Your Workflow").build()?).workflow(Workflow::builder().trigger(Trigger::builder().url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c").customizable_input_parameters(vec![InputParameter::builder().name("name_a").value("value_a").build()?,InputParameter::builder().name("name_b").value("value_b").build()?]).build()?).build()?).build()?;
    let value = Element::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Element::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Element>(json).unwrap(), value);
    assert!(Element::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn input_element_role() -> Result<(), ValidationError> {
    let concrete = ChannelMultiSelectElement::builder()
        .action_id("multi_channels_select")
        .placeholder(PlainText::builder().text("Select channels").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = ChannelSelectElement::builder()
        .action_id("channels_select")
        .placeholder(PlainText::builder().text("Select a channel").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = CheckboxesElement::builder()
        .action_id("actionId-0")
        .options(vec![
            SelectOption::builder()
                .text(MarkdownText::builder().text("*a*").build()?)
                .value("a")
                .description(PlainText::builder().text("*a*").build()?)
                .build()?,
            SelectOption::builder()
                .text(MarkdownText::builder().text("*b*").build()?)
                .value("b")
                .description(PlainText::builder().text("*b*").build()?)
                .build()?,
            SelectOption::builder()
                .text(MarkdownText::builder().text("*c*").build()?)
                .value("c")
                .description(PlainText::builder().text("*c*").build()?)
                .build()?,
        ])
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = ConversationMultiSelectElement::builder()
        .action_id("multi_conversations_select")
        .placeholder(PlainText::builder().text("Select conversations").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = ConversationSelectElement::builder()
        .action_id("conversations_select")
        .placeholder(
            PlainText::builder()
                .text("Select one conversation")
                .build()?,
        )
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = DatePickerElement::builder()
        .action_id("datepicker")
        .initial_date("1970-01-01")
        .placeholder(PlainText::builder().text("Pick a date").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = DateTimePickerElement::builder()
        .action_id("datetime_picker")
        .initial_date_time(1628633830_i64)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = EmailInputElement::builder()
        .action_id("email_input")
        .placeholder(PlainText::builder().text("Enter your email").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = ExternalMultiSelectElement::builder()
        .action_id("multi_external_select")
        .min_query_length(3_i64)
        .placeholder(PlainText::builder().text("Select items").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = ExternalSelectElement::builder()
        .action_id("external_select")
        .min_query_length(4_i64)
        .placeholder(PlainText::builder().text("Select one item").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = FileInputElement::builder()
        .action_id("file_input_action_id_1")
        .filetypes(vec![String::from("jpg"), String::from("png")])
        .max_files(5_i64)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = NumberInputElement::builder()
        .is_decimal_allowed(false)
        .action_id("number_input")
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = PlainTextInputElement::builder()
        .action_id("action")
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = RadioButtonsElement::builder()
        .action_id("radio_buttons")
        .options(vec![
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("B").build()?)
                .value("B")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("C").build()?)
                .value("C")
                .build()?,
        ])
        .initial_option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = RichTextInputElement::builder()
        .action_id("action_id")
        .initial_value(
            RichTextBlock::builder()
                .elements(vec![RichTextBlockElement::from(
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder().text("I'm rich").build()?,
                        )])
                        .build()?,
                )])
                .build()?,
        )
        .focus_on_load(false)
        .placeholder(PlainText::builder().text("Hello").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = StaticMultiSelectElement::builder()
        .action_id("multi_static_select")
        .options(vec![
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("B").build()?)
                .value("B")
                .build()?,
        ])
        .placeholder(PlainText::builder().text("Select one or more").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = StaticSelectElement::builder()
        .action_id("static_select")
        .options(vec![
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("B").build()?)
                .value("B")
                .build()?,
            SelectOption::builder()
                .text(PlainText::builder().text("C").build()?)
                .value("C")
                .build()?,
        ])
        .placeholder(PlainText::builder().text("Select one item").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = TimePickerElement::builder()
        .action_id("timepicker")
        .initial_time("12:00")
        .placeholder(PlainText::builder().text("Select your time").build()?)
        .timezone("Australia/Sydney")
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = UrlInputElement::builder()
        .action_id("url_text_input")
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = UserMultiSelectElement::builder()
        .action_id("multi_users_select")
        .placeholder(
            PlainText::builder()
                .text("Select one or more users")
                .build()?,
        )
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    let concrete = UserSelectElement::builder()
        .action_id("users_select")
        .placeholder(PlainText::builder().text("Select one user").build()?)
        .build()?;
    let value = InputElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(InputElement::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<InputElement>(json).unwrap(), value);
    assert!(InputElement::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn text_role() -> Result<(), ValidationError> {
    let concrete = MarkdownText::builder()
        .text("I like pretty colours")
        .build()?;
    let value = Text::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Text::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Text>(json).unwrap(), value);
    let concrete = PlainText::builder().text("*a*").build()?;
    let value = Text::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Text::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Text>(json).unwrap(), value);
    assert!(Text::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn chart_role() -> Result<(), ValidationError> {
    let concrete = AreaChart::builder()
        .series(vec![
            DataSeries::builder()
                .name("Free Tier")
                .data(vec![
                    DataPoint::builder().label("Mon").value(12000_i64).build()?,
                    DataPoint::builder().label("Tue").value(13500_i64).build()?,
                ])
                .build()?,
        ])
        .axis_config(
            AxisConfig::builder()
                .categories(vec![String::from("Mon"), String::from("Tue")])
                .x_label("Day")
                .y_label("Users")
                .build()?,
        )
        .build()?;
    let value = Chart::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Chart::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Chart>(json).unwrap(), value);
    let concrete = BarChart::builder()
        .series(vec![
            DataSeries::builder()
                .name("Pies")
                .data(vec![
                    DataPoint::builder()
                        .label("Pumpkin")
                        .value(70_i64)
                        .build()?,
                    DataPoint::builder()
                        .label("Blueberry")
                        .value(90_i64)
                        .build()?,
                ])
                .build()?,
        ])
        .axis_config(
            AxisConfig::builder()
                .categories(vec![String::from("Pumpkin"), String::from("Blueberry")])
                .x_label("Pies")
                .y_label("Tastiness")
                .build()?,
        )
        .build()?;
    let value = Chart::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Chart::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Chart>(json).unwrap(), value);
    let concrete = LineChart::builder()
        .series(vec![
            DataSeries::builder()
                .name("Website")
                .data(vec![
                    DataPoint::builder()
                        .label("Week 1")
                        .value(32000_i64)
                        .build()?,
                    DataPoint::builder()
                        .label("Week 2")
                        .value(35000_i64)
                        .build()?,
                ])
                .build()?,
            DataSeries::builder()
                .name("In-store")
                .data(vec![
                    DataPoint::builder()
                        .label("Week 1")
                        .value(28000_i64)
                        .build()?,
                    DataPoint::builder()
                        .label("Week 2")
                        .value(31000_i64)
                        .build()?,
                ])
                .build()?,
        ])
        .axis_config(
            AxisConfig::builder()
                .categories(vec![String::from("Week 1"), String::from("Week 2")])
                .x_label("Week")
                .y_label("Paper Sales (USD)")
                .build()?,
        )
        .build()?;
    let value = Chart::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Chart::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Chart>(json).unwrap(), value);
    let concrete = PieChart::builder()
        .segments(vec![
            ChartSegment::builder()
                .label("Kit Kat")
                .value(45_i64)
                .build()?,
            ChartSegment::builder()
                .label("Twix")
                .value(28_i64)
                .build()?,
            ChartSegment::builder()
                .label("Crunch")
                .value(18_i64)
                .build()?,
            ChartSegment::builder()
                .label("Milky Way")
                .value(9_i64)
                .build()?,
        ])
        .build()?;
    let value = Chart::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(Chart::try_from(json.clone())?, value);
    assert_eq!(serde_json::from_value::<Chart>(json).unwrap(), value);
    assert!(Chart::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn rich_text_block_element_role() -> Result<(), ValidationError> {
    let concrete = RichTextCodeBlock::builder()
        .elements(vec![RichTextSectionElement::from(
            RichTextText::builder()
                .text("\ndef hello_world():\n    print('hello, world')")
                .build()?,
        )])
        .border(0_i64)
        .build()?;
    let value = RichTextBlockElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextBlockElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextBlockElement>(json).unwrap(),
        value
    );
    let concrete = RichTextList::builder()
        .elements(vec![
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Oh").build()?,
                )])
                .build()?,
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Hi").build()?,
                )])
                .build()?,
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Mark").build()?,
                )])
                .build()?,
        ])
        .style(RichTextListStyle::Bullet)
        .indent(0_i64)
        .offset(0_i64)
        .border(1_i64)
        .build()?;
    let value = RichTextBlockElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextBlockElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextBlockElement>(json).unwrap(),
        value
    );
    let concrete = RichTextQuote::builder()
        .elements(vec![RichTextSectionElement::from(
            RichTextText::builder()
                .text("Great and good are seldom the same man")
                .build()?,
        )])
        .border(1_i64)
        .build()?;
    let value = RichTextBlockElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextBlockElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextBlockElement>(json).unwrap(),
        value
    );
    let concrete = RichTextSection::builder()
        .elements(vec![RichTextSectionElement::from(
            RichTextText::builder()
                .text("Profile data loaded")
                .build()?,
        )])
        .build()?;
    let value = RichTextBlockElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextBlockElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextBlockElement>(json).unwrap(),
        value
    );
    assert!(RichTextBlockElement::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
#[test]
fn rich_text_section_element_role() -> Result<(), ValidationError> {
    let concrete = RichTextChannel::builder()
        .channel_id("C0261C65XNY")
        .style(
            RichTextStyle::new()
                .bold(true)
                .italic(false)
                .strike(true)
                .highlight(true)
                .client_highlight(true)
                .unlink(false),
        )
        .build()?;
    let value = RichTextSectionElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextSectionElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextSectionElement>(json).unwrap(),
        value
    );
    let concrete = RichTextEmoji::builder().name("wave").build()?;
    let value = RichTextSectionElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextSectionElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextSectionElement>(json).unwrap(),
        value
    );
    let concrete = RichTextLink::builder()
        .url("https://slack.com")
        .text("Data 1B")
        .build()?;
    let value = RichTextSectionElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextSectionElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextSectionElement>(json).unwrap(),
        value
    );
    let concrete = RichTextText::builder()
        .text("Profile data loaded")
        .build()?;
    let value = RichTextSectionElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextSectionElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextSectionElement>(json).unwrap(),
        value
    );
    let concrete = RichTextUser::builder()
        .user_id("DR36TNNLA")
        .style(
            RichTextStyle::new()
                .bold(true)
                .italic(false)
                .strike(true)
                .highlight(true)
                .client_highlight(true)
                .unlink(false),
        )
        .build()?;
    let value = RichTextSectionElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextSectionElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextSectionElement>(json).unwrap(),
        value
    );
    let concrete = RichTextUserGroup::builder()
        .usergroup_id("C01RGRU0RUK")
        .style(
            RichTextStyle::new()
                .bold(true)
                .italic(false)
                .strike(true)
                .highlight(true)
                .client_highlight(true)
                .unlink(false),
        )
        .build()?;
    let value = RichTextSectionElement::from(concrete);
    let json = serde_json::to_value(&value).unwrap();
    assert_eq!(RichTextSectionElement::try_from(json.clone())?, value);
    assert_eq!(
        serde_json::from_value::<RichTextSectionElement>(json).unwrap(),
        value
    );
    assert!(RichTextSectionElement::try_from(json!({"type":"not-a-role"})).is_err());
    Ok(())
}
