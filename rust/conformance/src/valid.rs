// Independent public-API examples. Hand-maintained; never loaded from expected JSON.
use serde_json::Value;
use slackblocks::*;

pub struct ValidCase {
    pub id: &'static str,
    pub build: fn() -> Result<Value, ValidationError>,
    pub parse: fn(Value) -> Result<Value, ValidationError>,
}

pub const VALID: &[ValidCase] = &[
    ValidCase {
        id: "attachments/attachment_multi_block",
        build: || {
            let value = Attachment::builder()
                .blocks(vec![
                    Block::from(
                        SectionBlock::builder()
                            .block_id("fake_block_id_0")
                            .text(
                                MarkdownText::builder()
                                    .text("I like pretty colours")
                                    .build()?,
                            )
                            .build()?,
                    ),
                    Block::from(
                        SectionBlock::builder()
                            .block_id("fake_block_id_1")
                            .text(
                                MarkdownText::builder()
                                    .text("I don't like pretty colours")
                                    .build()?,
                            )
                            .build()?,
                    ),
                ])
                .color("#8800ff")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| Attachment::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "attachments/attachment_simple",
        build: || {
            let value = Attachment::builder()
                .blocks(vec![Block::from(
                    SectionBlock::builder()
                        .block_id("fake_block_id")
                        .text(
                            MarkdownText::builder()
                                .text("I like pretty colours")
                                .build()?,
                        )
                        .build()?,
                )])
                .color("#000000")
                .fallback("Colours preference")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| Attachment::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/actions_block_checkboxes",
        build: || {
            let value = ActionsBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ActionsBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/alert_block",
        build: || {
            let value = AlertBlock::builder()
                .block_id("fake_block_id")
                .text(
                    MarkdownText::builder()
                        .text("The work is mysterious and important.")
                        .build()?,
                )
                .level(AlertLevel::Info)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| AlertBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/card_block",
        build: || {
            let value = CardBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| CardBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/carousel_block",
        build: || {
            let value = CarouselBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| CarouselBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/container_block",
        build: || {
            let value = ContainerBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ContainerBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/context_actions_feedback_buttons",
        build: || {
            let value = ContextActionsBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ContextActionsBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/context_actions_icon_button",
        build: || {
            let value = ContextActionsBlock::builder()
                .block_id("fake_block_id")
                .elements(vec![ContextActionsElement::from(
                    IconButtonElement::builder()
                        .icon(IconButtonIcon::Trash)
                        .text(PlainText::builder().text("Delete").build()?)
                        .action_id("delete_button")
                        .value("delete_item")
                        .build()?,
                )])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ContextActionsBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/context_block_text_only",
        build: || {
            let value = ContextBlock::builder()
                .block_id("fake_block_id")
                .elements(vec![ContextElement::from(
                    MarkdownText::builder().text("Hello, world!").build()?,
                )])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ContextBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/data_table_block",
        build: || {
            let value = DataTableBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DataTableBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/data_visualization_area",
        build: || {
            let value = DataVisualizationBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DataVisualizationBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/data_visualization_bar",
        build: || {
            let value = DataVisualizationBlock::builder()
                .block_id("fake_block_id")
                .title("Pies by Tastiness")
                .chart(
                    BarChart::builder()
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
                                .categories(vec![
                                    String::from("Pumpkin"),
                                    String::from("Blueberry"),
                                ])
                                .x_label("Pies")
                                .y_label("Tastiness")
                                .build()?,
                        )
                        .build()?,
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DataVisualizationBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/data_visualization_line",
        build: || {
            let value = DataVisualizationBlock::builder()
                .block_id("fake_block_id")
                .title("Weekly Paper Sales")
                .chart(
                    LineChart::builder()
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
                        .build()?,
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DataVisualizationBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/data_visualization_pie",
        build: || {
            let value = DataVisualizationBlock::builder()
                .block_id("fake_block_id")
                .title("My Favorite Candy Bars")
                .chart(
                    PieChart::builder()
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
                        .build()?,
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DataVisualizationBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/divider_block_only",
        build: || {
            let value = DividerBlock::builder().block_id("fake_block_id").build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DividerBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/file_block_only",
        build: || {
            let value = FileBlock::builder()
                .external_id("external_id")
                .source("remote")
                .block_id("fake_block_id")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| FileBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/header_block_emoji_at_limit",
        build: || {
            let value = HeaderBlock::builder()
                .block_id("fake_block_id")
                .text(PlainText::builder().text("😀".repeat(150)).build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| HeaderBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/header_block_only",
        build: || {
            let value = HeaderBlock::builder()
                .block_id("fake_block_id")
                .text(PlainText::builder().text("AloHa!").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| HeaderBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/header_block_text_at_limit",
        build: || {
            let value = HeaderBlock::builder()
                .block_id("fake_block_id")
                .text(PlainText::builder().text("x".repeat(150)).build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| HeaderBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/image_block_only",
        build: || {
            let value = ImageBlock::builder()
                .block_id("fake_block_id")
                .image_url("https://api.slack.com/img/blocks/bkb_template_images/beagle.png")
                .alt_text("image1")
                .title(PlainText::builder().text("image1").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ImageBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/image_block_slack_file",
        build: || {
            let value = ImageBlock::builder()
                .block_id("fake_block_id")
                .slack_file(
                    SlackFile::builder()
                        .url("https://files.slack.com/files-pri/T0123456-F0123ABC456/kitten.png")
                        .build()?,
                )
                .alt_text("An incredibly cute kitten.")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ImageBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/input_block_only",
        build: || {
            let value = InputBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| InputBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/markdown_block_basic",
        build: || {
            let value = MarkdownBlock::builder()
                .block_id("fake_block_id")
                .text("**Hello**, _world_!")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MarkdownBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/markdown_empty",
        build: || {
            let value = MarkdownBlock::builder()
                .block_id("fake_block_id")
                .text("")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MarkdownBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/plan_block",
        build: || {
            let value = PlanBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| PlanBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/rich_text_block_basic",
        build: || {
            let value = RichTextBlock::builder()
                .block_id("fake_block_id")
                .elements(vec![RichTextBlockElement::from(
                    RichTextSection::builder()
                        .elements(vec![
                            RichTextSectionElement::from(
                                RichTextText::builder()
                                    .text("You 'bout to witness hip-hop in its most purest")
                                    .style(RichTextStyle::new().bold(true))
                                    .build()?,
                            ),
                            RichTextSectionElement::from(
                                RichTextText::builder()
                                    .text("Most rawest form, flow almost flawless")
                                    .style(RichTextStyle::new().strike(true))
                                    .build()?,
                            ),
                            RichTextSectionElement::from(
                                RichTextText::builder()
                                    .text("Most hardest, most honest known artist")
                                    .style(RichTextStyle::new().italic(true))
                                    .build()?,
                            ),
                        ])
                        .build()?,
                )])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/section_block_both_text_and_fields",
        build: || {
            let value = SectionBlock::builder()
                .block_id("fake_block_id")
                .text(MarkdownText::builder().text("Hello").build()?)
                .fields(vec![
                    TextInput::from(MarkdownText::builder().text("Are you").build()?),
                    TextInput::from(PlainText::builder().text("There?").emoji(true).build()?),
                ])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SectionBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/section_block_empty_text_field_value",
        build: || {
            let value = SectionBlock::builder()
                .block_id("fake_block_id")
                .fields(vec![
                    TextInput::from(MarkdownText::builder().text("Highly").build()?),
                    TextInput::from(PlainText::builder().text("Strung").emoji(true).build()?),
                ])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SectionBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/section_block_fields",
        build: || {
            let value = SectionBlock::builder()
                .block_id("fake_block_id")
                .text(MarkdownText::builder().text("Test:").build()?)
                .fields(vec![
                    TextInput::from(PlainText::builder().text("foo").build()?),
                    TextInput::from(MarkdownText::builder().text("bar").build()?),
                ])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SectionBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/section_block_single_field_value_coercion",
        build: || {
            let value = SectionBlock::builder()
                .block_id("fake_block_id")
                .fields(vec![TextInput::from(
                    MarkdownText::builder().text("Lowly").build()?,
                )])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SectionBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/section_block_text_only",
        build: || {
            let value = SectionBlock::builder()
                .block_id("fake_block_id")
                .text(MarkdownText::builder().text("Hello, world!").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SectionBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/table_block",
        build: || {
            let value = TableBlock::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| TableBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/table_ragged_rows",
        build: || {
            let value = TableBlock::builder()
                .block_id("fake_block_id")
                .rows(vec![
                    vec![
                        TableCell::from(RawText::builder().text("Header A").build()?),
                        TableCell::from(RawText::builder().text("Header B").build()?),
                    ],
                    vec![TableCell::from(
                        RawText::builder().text("Only one cell").build()?,
                    )],
                ])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| TableBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/task_card_block",
        build: || {
            let value = TaskCardBlock::builder()
                .block_id("fake_block_id")
                .task_id("task_1")
                .title("Fetching weather data")
                .output(
                    RichTextBlock::builder()
                        .block_id("task_output")
                        .elements(vec![RichTextBlockElement::from(
                            RichTextSection::builder()
                                .elements(vec![RichTextSectionElement::from(
                                    RichTextText::builder()
                                        .text("Found weather data for Chicago from 2 sources")
                                        .build()?,
                                )])
                                .build()?,
                        )])
                        .build()?,
                )
                .sources(vec![
                    UrlSource::builder()
                        .url("https://weather.com/")
                        .text("weather.com")
                        .build()?,
                    UrlSource::builder()
                        .url("https://www.accuweather.com/")
                        .text("accuweather.com")
                        .build()?,
                ])
                .status(TaskStatus::InProgress)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| TaskCardBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/video_block_basic",
        build: || {
            let value = VideoBlock::builder()
                .block_id("b1")
                .alt_text("alt")
                .thumbnail_url("https://example.com/t.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/v.mp4")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| VideoBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "blocks/video_block_full",
        build: || {
            let value = VideoBlock::builder()
                .block_id("video_1")
                .alt_text("How to use Slack")
                .thumbnail_url("https://example.com/thumb.png")
                .title(PlainText::builder().text("Getting Started").build()?)
                .video_url("https://example.com/video.mp4")
                .author_name("Slack")
                .description(PlainText::builder().text("A short intro").build()?)
                .provider_icon_url("https://example.com/icon.png")
                .provider_name("YouTube")
                .title_url("https://example.com")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| VideoBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/button_basic",
        build: || {
            let value = ButtonElement::builder()
                .text(PlainText::builder().text("Click Me").build()?)
                .action_id("button")
                .value("click_me")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ButtonElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/button_without_action_id",
        build: || {
            let value = ButtonElement::builder()
                .text(PlainText::builder().text("Click Me").build()?)
                .value("click_me")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ButtonElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/button_link",
        build: || {
            let value = ButtonElement::builder()
                .text(PlainText::builder().text("Link!").build()?)
                .action_id("button")
                .url("https://ndl.im/")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ButtonElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/button_style",
        build: || {
            let value = ButtonElement::builder()
                .text(PlainText::builder().text("Load").build()?)
                .action_id("button")
                .style(ButtonStyle::Primary)
                .value("im_a_style_button")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ButtonElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/button_text_at_limit",
        build: || {
            let value = ButtonElement::builder().text(PlainText::builder().text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).action_id("button").build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ButtonElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/checkbox_basic",
        build: || {
            let value = CheckboxesElement::builder()
                .action_id("and...action")
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
                .initial_options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("A")
                        .build()?,
                ])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| CheckboxesElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/date_picker_basic",
        build: || {
            let value = DatePickerElement::builder()
                .action_id("datepicker")
                .initial_date("1970-01-01")
                .placeholder(PlainText::builder().text("Pick a date").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DatePickerElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/datetime_picker_basic",
        build: || {
            let value = DateTimePickerElement::builder()
                .action_id("datetime_picker")
                .initial_date_time(1628633830_i64)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| DateTimePickerElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/email_input_basic",
        build: || {
            let value = EmailInputElement::builder()
                .action_id("email_input")
                .placeholder(PlainText::builder().text("Enter your email").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| EmailInputElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/file_input_basic",
        build: || {
            let value = FileInputElement::builder()
                .action_id("file_input_action_id_1")
                .filetypes(vec![String::from("jpg"), String::from("png")])
                .max_files(5_i64)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| FileInputElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/image_basic",
        build: || {
            let value = ImageElement::builder()
                .image_url("https://ndl.im/img/logo.png")
                .alt_text("Logo for ndl.im")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ImageElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/image_slack_file_id",
        build: || {
            let value = ImageElement::builder()
                .slack_file(SlackFile::builder().id("F0123ABC456").build()?)
                .alt_text("An incredibly cute kitten.")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ImageElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/image_slack_file_url",
        build: || {
            let value = ImageElement::builder()
                .slack_file(
                    SlackFile::builder()
                        .url("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png")
                        .build()?,
                )
                .alt_text("An incredibly cute kitten.")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ImageElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/multi_select_channel",
        build: || {
            let value = ChannelMultiSelectElement::builder()
                .action_id("multi_channels_select")
                .placeholder(PlainText::builder().text("Select channels").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ChannelMultiSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/multi_select_conversation",
        build: || {
            let value = ConversationMultiSelectElement::builder()
                .action_id("multi_conversations_select")
                .placeholder(PlainText::builder().text("Select conversations").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| {
            ConversationMultiSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap())
        },
    },
    ValidCase {
        id: "elements/multi_select_external",
        build: || {
            let value = ExternalMultiSelectElement::builder()
                .action_id("multi_external_select")
                .min_query_length(3_i64)
                .placeholder(PlainText::builder().text("Select items").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| {
            ExternalMultiSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap())
        },
    },
    ValidCase {
        id: "elements/multi_select_static",
        build: || {
            let value = StaticMultiSelectElement::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| StaticMultiSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/multi_select_user",
        build: || {
            let value = UserMultiSelectElement::builder()
                .action_id("multi_users_select")
                .placeholder(
                    PlainText::builder()
                        .text("Select one or more users")
                        .build()?,
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| UserMultiSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/multi_select_user_with_initial_users",
        build: || {
            let value = UserMultiSelectElement::builder()
                .action_id("multi_users_select")
                .initial_users(vec![
                    String::from("U064B5H1309"),
                    String::from("U063JR973UP"),
                ])
                .placeholder(
                    PlainText::builder()
                        .text("Select one or more users")
                        .build()?,
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| UserMultiSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/number_input_basic",
        build: || {
            let value = NumberInputElement::builder()
                .is_decimal_allowed(false)
                .action_id("number_input")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| NumberInputElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/overflow_menu_basic",
        build: || {
            let value = OverflowElement::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| OverflowElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/plaintext_input_basic",
        build: || {
            let value = PlainTextInputElement::builder()
                .action_id("plaintext_input")
                .placeholder(PlainText::builder().text("Enter your plain text").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| PlainTextInputElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/radio_button_group_basic",
        build: || {
            let value = RadioButtonsElement::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RadioButtonsElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/rich_text_input_basic",
        build: || {
            let value = RichTextInputElement::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextInputElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/select_menu_channel",
        build: || {
            let value = ChannelSelectElement::builder()
                .action_id("channels_select")
                .placeholder(PlainText::builder().text("Select a channel").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ChannelSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/select_menu_conversation",
        build: || {
            let value = ConversationSelectElement::builder()
                .action_id("conversations_select")
                .placeholder(
                    PlainText::builder()
                        .text("Select one conversation")
                        .build()?,
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ConversationSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/select_menu_external",
        build: || {
            let value = ExternalSelectElement::builder()
                .action_id("external_select")
                .min_query_length(4_i64)
                .placeholder(PlainText::builder().text("Select one item").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ExternalSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/select_menu_static",
        build: || {
            let value = StaticSelectElement::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| StaticSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/select_menu_user",
        build: || {
            let value = UserSelectElement::builder()
                .action_id("users_select")
                .placeholder(PlainText::builder().text("Select one user").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| UserSelectElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/timepicker_basic",
        build: || {
            let value = TimePickerElement::builder()
                .action_id("timepicker")
                .initial_time("12:00")
                .placeholder(PlainText::builder().text("Select your time").build()?)
                .timezone("Australia/Sydney")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| TimePickerElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/url_input_basic",
        build: || {
            let value = UrlInputElement::builder()
                .action_id("url_text_input")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| UrlInputElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/url_source_basic",
        build: || {
            let value = UrlSource::builder()
                .url("https://docs.slack.dev/")
                .text("Slack API docs")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| UrlSource::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "elements/workflow_button_basic",
        build: || {
            let value = WorkflowButtonElement::builder().action_id("run_workflow").text(PlainText::builder().text("Run Your Workflow").build()?).workflow(Workflow::builder().trigger(Trigger::builder().url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c").customizable_input_parameters(vec![InputParameter::builder().name("name_a").value("value_a").build()?,InputParameter::builder().name("name_b").value("value_b").build()?]).build()?).build()?).build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| WorkflowButtonElement::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/message_basic",
        build: || {
            let value = MessagePayload::builder("#slackblocks")
                .mrkdwn(true)
                .blocks(vec![Block::from(
                    SectionBlock::builder()
                        .block_id("fake_block_id")
                        .text(MarkdownText::builder().text("Hello, world!").build()?)
                        .build()?,
                )])
                .text("")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MessagePayload::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/message_basic_attachment",
        build: || {
            let value = MessagePayload::builder("#slackblocks")
                .mrkdwn(true)
                .attachments(vec![
                    Attachment::builder()
                        .blocks(vec![Block::from(
                            SectionBlock::builder()
                                .block_id("block1")
                                .text(MarkdownText::builder().text("Hello, world!").build()?)
                                .build()?,
                        )])
                        .color("#000000")
                        .build()?,
                ])
                .text("")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MessagePayload::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/message_compound",
        build: || {
            let value = MessagePayload::builder("#slackblocks")
                .mrkdwn(true)
                .blocks(vec![
                    Block::from(
                        SectionBlock::builder()
                            .block_id("fake_block1")
                            .text(MarkdownText::builder().text("Block, One").build()?)
                            .build()?,
                    ),
                    Block::from(
                        ImageBlock::builder()
                            .block_id("fake_block3")
                            .image_url("http://bit.ly/slack-block-test-image")
                            .alt_text("crash")
                            .title(PlainText::builder().text(" ").build()?)
                            .build()?,
                    ),
                ])
                .attachments(vec![
                    Attachment::builder()
                        .blocks(vec![Block::from(
                            SectionBlock::builder()
                                .block_id("fake_block1")
                                .text(MarkdownText::builder().text("Block, One").build()?)
                                .build()?,
                        )])
                        .color("#8800ff")
                        .build()?,
                    Attachment::builder()
                        .blocks(vec![
                            Block::from(
                                SectionBlock::builder()
                                    .block_id("fake_block2")
                                    .text(MarkdownText::builder().text("Block, Two").build()?)
                                    .build()?,
                            ),
                            Block::from(
                                ImageBlock::builder()
                                    .block_id("fake_block3")
                                    .image_url("http://bit.ly/slack-block-test-image")
                                    .alt_text("crash")
                                    .title(PlainText::builder().text(" ").build()?)
                                    .build()?,
                            ),
                        ])
                        .color("#ffff00")
                        .build()?,
                ])
                .text("")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MessagePayload::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/message_response",
        build: || {
            let value = MessageResponse::builder()
                .mrkdwn(true)
                .blocks(vec![Block::from(
                    SectionBlock::builder()
                        .block_id("fake_block_id")
                        .text(MarkdownText::builder().text("Hello, world!").build()?)
                        .build()?,
                )])
                .text("")
                .replace_original(false)
                .response_type(ResponseType::Ephemeral)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MessageResponse::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/message_with_attachments",
        build: || {
            let value = MessagePayload::builder("#slackblocks")
                .mrkdwn(true)
                .attachments(vec![
                    Attachment::builder()
                        .blocks(vec![Block::from(
                            SectionBlock::builder()
                                .block_id("fake_block_id")
                                .text(MarkdownText::builder().text("Hello, world!").build()?)
                                .build()?,
                        )])
                        .color("#ffff00")
                        .build()?,
                ])
                .text("")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MessagePayload::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/message_with_optional_arguments",
        build: || {
            let value = MessagePayload::builder("#slackblocks")
                .mrkdwn(true)
                .blocks(vec![Block::from(
                    SectionBlock::builder()
                        .block_id("fake_block_id")
                        .text(MarkdownText::builder().text("Hello, world!").build()?)
                        .build()?,
                )])
                .text("")
                .unfurl_links(false)
                .unfurl_media(false)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MessagePayload::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/webhook_message_basic",
        build: || {
            let value = WebhookMessage::builder()
                .blocks(vec![
                    Block::from(
                        SectionBlock::builder()
                            .block_id("fake_block_id")
                            .text(
                                MarkdownText::builder()
                                    .text("You wouldn't do ol' Hook in now, would you, lad?")
                                    .build()?,
                            )
                            .build()?,
                    ),
                    Block::from(
                        SectionBlock::builder()
                            .block_id("fake_block_id")
                            .text(
                                MarkdownText::builder()
                                    .text("Well, all right... if you... say you're a codfish.")
                                    .build()?,
                            )
                            .build()?,
                    ),
                ])
                .response_type(ResponseType::Ephemeral)
                .replace_original(true)
                .unfurl_links(false)
                .unfurl_media(false)
                .metadata(
                    match Value::Object(
                        [("sender".into(), Value::from("Walt"))]
                            .into_iter()
                            .collect(),
                    ) {
                        Value::Object(map) => map,
                        _ => unreachable!(),
                    },
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| WebhookMessage::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "messages/webhook_message_delete",
        build: || {
            let value = WebhookMessage::builder()
                .attachments(vec![
                    Attachment::builder()
                        .blocks(vec![Block::from(
                            SectionBlock::builder()
                                .block_id("fake_block_id")
                                .text(MarkdownText::builder().text("I'M A CODFISH!").build()?)
                                .build()?,
                        )])
                        .build()?,
                ])
                .blocks(vec![
                    Block::from(
                        SectionBlock::builder()
                            .block_id("fake_block_id")
                            .text(MarkdownText::builder().text("I'm a codfish.").build()?)
                            .build()?,
                    ),
                    Block::from(
                        SectionBlock::builder()
                            .block_id("fake_block_id")
                            .text(MarkdownText::builder().text("Louder!").build()?)
                            .build()?,
                    ),
                ])
                .response_type(ResponseType::InChannel)
                .delete_original(true)
                .unfurl_links(true)
                .unfurl_media(true)
                .metadata(
                    match Value::Object(
                        [("sender".into(), Value::from("Walt"))]
                            .into_iter()
                            .collect(),
                    ) {
                        Value::Object(map) => map,
                        _ => unreachable!(),
                    },
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| WebhookMessage::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/confirmation_dialogue_basic",
        build: || {
            let value = ConfirmationDialogue::builder()
                .title(PlainText::builder().text("Maybe?").build()?)
                .text(
                    PlainText::builder()
                        .text("Would you like to play checkers?")
                        .build()?,
                )
                .confirm(PlainText::builder().text("Yes").build()?)
                .deny(PlainText::builder().text("Nope!").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ConfirmationDialogue::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/conversation_filter_basic",
        build: || {
            let value = ConversationFilter::builder()
                .include(vec![String::from("public"), String::from("mpim")])
                .exclude_bot_users(true)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ConversationFilter::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/dispatch_action_configuration_basic",
        build: || {
            let value = DispatchActionConfiguration::builder()
                .trigger_actions_on(vec![String::from("on_character_entered")])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| {
            DispatchActionConfiguration::try_from(v).map(|v| serde_json::to_value(v).unwrap())
        },
    },
    ValidCase {
        id: "objects/input_parameter_basic",
        build: || {
            let value = InputParameter::builder()
                .name("name")
                .value("value")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| InputParameter::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/option_basic",
        build: || {
            let value = SelectOption::builder()
                .text(PlainText::builder().text("Canberra").build()?)
                .value("canberra")
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SelectOption::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/option_group_basic",
        build: || {
            let value = SelectOptionGroup::builder()
                .label(PlainText::builder().text("Group A").build()?)
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SelectOptionGroup::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/option_value_at_limit",
        build: || {
            let value = SelectOption::builder()
                .text(PlainText::builder().text("At limit").build()?)
                .value("x".repeat(150))
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| SelectOption::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/text_markdown_basic",
        build: || {
            let value = MarkdownText::builder().text("hi").build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MarkdownText::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/text_markdown_verbatim",
        build: || {
            let value = MarkdownText::builder().text("hi").verbatim(true).build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| MarkdownText::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/text_plaintext_basic",
        build: || {
            let value = PlainText::builder().text("hi").build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| PlainText::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/text_plaintext_emoji",
        build: || {
            let value = PlainText::builder().text("hi").emoji(true).build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| PlainText::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/trigger_basic",
        build: || {
            let value = Trigger::builder()
                .url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c")
                .customizable_input_parameters(vec![
                    InputParameter::builder().name("A").value("A").build()?,
                    InputParameter::builder().name("B").value("B").build()?,
                ])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| Trigger::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "objects/workflow_basic",
        build: || {
            let value = Workflow::builder().trigger(Trigger::builder().url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c").customizable_input_parameters(vec![InputParameter::builder().name("A").value("A").build()?,InputParameter::builder().name("B").value("B").build()?]).build()?).build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| Workflow::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_basic",
        build: || {
            let value = RichTextText::builder()
                .text("I am a bold rich text block!")
                .style(RichTextStyle::new().bold(true).italic(true).strike(false))
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextText::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_channel_basic",
        build: || {
            let value = RichTextChannel::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextChannel::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_code_block_basic",
        build: || {
            let value = RichTextCodeBlock::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder()
                        .text("\ndef hello_world():\n    print('hello, world')")
                        .build()?,
                )])
                .border(0_i64)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextCodeBlock::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_emoji_basic",
        build: || {
            let value = RichTextEmoji::builder().name("wave").build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextEmoji::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_link_basic",
        build: || {
            let value = RichTextLink::builder()
                .url("https://google.com/")
                .text("Google")
                .r#unsafe(false)
                .style(
                    RichTextStyle::new()
                        .bold(true)
                        .italic(false)
                        .strike(true)
                        .code(true),
                )
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextLink::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_list_basic",
        build: || {
            let value = RichTextList::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextList::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_list_ordered",
        build: || {
            let value = RichTextList::builder()
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
                ])
                .style(RichTextListStyle::Ordered)
                .indent(1_i64)
                .offset(2_i64)
                .border(1_i64)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextList::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_quote_basic",
        build: || {
            let value = RichTextQuote::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder()
                        .text("Great and good are seldom the same man")
                        .build()?,
                )])
                .border(1_i64)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextQuote::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_section_basic",
        build: || {
            let value = RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder()
                        .text("The only true wisdom is in knowing you know nothing")
                        .build()?,
                )])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextSection::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_user_basic",
        build: || {
            let value = RichTextUser::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextUser::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "rich_text/rich_text_user_group_basic",
        build: || {
            let value = RichTextUserGroup::builder()
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
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| RichTextUserGroup::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "views/hometab_view",
        build: || {
            let value = HomeTabView::builder()
                .blocks(vec![Block::from(
                    SectionBlock::builder()
                        .block_id("fake_id")
                        .text(MarkdownText::builder().text("Example Block").build()?)
                        .build()?,
                )])
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| HomeTabView::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "views/modal_with_blocks",
        build: || {
            let value = ModalView::builder()
                .blocks(vec![
                    Block::from(
                        SectionBlock::builder()
                            .block_id("1")
                            .text(
                                MarkdownText::builder()
                                    .text("first section block")
                                    .build()?,
                            )
                            .build()?,
                    ),
                    Block::from(DividerBlock::builder().block_id("2").build()?),
                    Block::from(
                        SectionBlock::builder()
                            .block_id("3")
                            .text(
                                MarkdownText::builder()
                                    .text("second section block")
                                    .build()?,
                            )
                            .build()?,
                    ),
                ])
                .title(PlainText::builder().text("Hello, world!").build()?)
                .close(PlainText::builder().text("Close button").build()?)
                .submit(PlainText::builder().text("Submit button").build()?)
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ModalView::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
    ValidCase {
        id: "views/modal_without_blocks",
        build: || {
            let value = ModalView::builder()
                .title(PlainText::builder().text("Empty").build()?)
                .blocks(Vec::<Block>::new())
                .build()?;
            Ok(serde_json::to_value(value).unwrap())
        },
        parse: |v| ModalView::try_from(v).map(|v| serde_json::to_value(v).unwrap()),
    },
];
