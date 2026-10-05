//! Accepted counterparts to the field-dependency errors in `api_roundtrips`.
use serde_json::{Value, json};
use slackblocks::*;

fn wire(value: impl serde::Serialize) -> Value {
    serde_json::to_value(value).unwrap()
}

#[test]
fn image_sources_and_card_icons_can_be_replaced() -> Result<(), ValidationError> {
    let file = SlackFile::builder()
        .url("https://files.slack.com/example.png")
        .build()?;
    let replaced = file
        .clone()
        .into_builder()
        .clear_url()
        .id("F0123ABC456")
        .build()?;
    assert_eq!(wire(&replaced), json!({"id": "F0123ABC456"}));
    assert_eq!(file.url(), Some("https://files.slack.com/example.png"));

    let image = ImageElement::builder()
        .image_url("https://example.com/old.png")
        .alt_text("Old")
        .build()?;
    let edited = image
        .clone()
        .into_builder()
        .clear_image_url()
        .slack_file(replaced.clone())
        .alt_text("New")
        .build()?;
    assert_eq!(
        wire(&edited),
        json!({"type": "image", "slack_file": {"id": "F0123ABC456"}, "alt_text": "New"})
    );
    assert_eq!(image.image_url(), Some("https://example.com/old.png"));

    let block = ImageBlock::builder()
        .image_url("https://example.com/old.png")
        .alt_text("Old")
        .build()?;
    assert_eq!(
        wire(
            block
                .into_builder()
                .clear_image_url()
                .slack_file(replaced)
                .alt_text("New")
                .build()?
        ),
        wire(&edited)
    );

    let card = CardBlock::builder()
        .title("Title")
        .slack_icon(SlackIcon::builder().name("rocket").build()?)
        .build()?;
    let changed = card
        .into_builder()
        .clear_slack_icon()
        .icon(edited)
        .build()?;
    assert!(wire(&changed).get("slack_icon").is_none());
    assert_eq!(
        wire(changed)["icon"]["slack_file"],
        json!({"id": "F0123ABC456"})
    );
    Ok(())
}

#[test]
fn dependent_clear_operations_preserve_the_required_alternative() -> Result<(), ValidationError> {
    let rich = RichTextBlock::builder()
        .element(
            RichTextSection::builder()
                .element(RichTextText::new("Rich title")?)
                .build()?,
        )
        .build()?;
    let container = ContainerBlock::builder()
        .title("Old title")
        .child_blocks([DividerBlock::builder().build()?])
        .build()?;
    let edited = container
        .into_builder()
        .rich_text_title(rich.clone())
        .clear_title()
        .build()?;
    assert!(wire(&edited).get("title").is_none());
    assert_eq!(wire(edited)["rich_text_title"], wire(rich));

    let config = DispatchActionConfiguration::builder()
        .trigger_actions_on(["on_enter_pressed"])
        .build()?;
    let edited = config
        .into_builder()
        .clear_trigger_actions_on()
        .trigger_actions_on(["on_character_entered"])
        .build()?;
    assert_eq!(
        wire(edited),
        json!({"trigger_actions_on": ["on_character_entered"]})
    );
    Ok(())
}

#[test]
fn select_options_can_be_replaced_with_groups_and_appended() -> Result<(), ValidationError> {
    let a = SelectOption::builder().text("A").value("a").build()?;
    let b = SelectOption::builder().text("B").value("b").build()?;
    let group_a = SelectOptionGroup::builder()
        .label("First")
        .option(a.clone())
        .build()?;
    let group_b = SelectOptionGroup::builder()
        .label("Second")
        .option(b)
        .build()?;
    let expected = json!([
        {"label": {"type": "plain_text", "text": "First"}, "options": [{"text": {"type": "plain_text", "text": "A"}, "value": "a"}]},
        {"label": {"type": "plain_text", "text": "Second"}, "options": [{"text": {"type": "plain_text", "text": "B"}, "value": "b"}]}
    ]);
    let single = StaticSelectElement::builder()
        .options([a.clone()])
        .build()?;
    let edited = single
        .into_builder()
        .clear_options()
        .option_groups([group_a.clone()])
        .option_group(group_b.clone())
        .build()?;
    assert!(wire(&edited).get("options").is_none());
    assert_eq!(wire(edited)["option_groups"], expected);

    let multiple = StaticMultiSelectElement::builder().options([a]).build()?;
    let edited = multiple
        .into_builder()
        .clear_options()
        .option_groups([group_a])
        .option_group(group_b)
        .build()?;
    assert!(wire(&edited).get("options").is_none());
    assert_eq!(wire(edited)["option_groups"], expected);
    Ok(())
}

#[test]
fn appenders_accept_distinct_categories_series_and_tasks() -> Result<(), ValidationError> {
    let axis = AxisConfig::builder().categories(["A"]).build()?;
    let axis = axis.into_builder().category("B").build()?;
    assert_eq!(wire(&axis)["categories"], json!(["A", "B"]));
    let first = DataSeries::builder()
        .name("First")
        .data([
            DataPoint::builder().label("A").value(1_i64).build()?,
            DataPoint::builder().label("B").value(2_i64).build()?,
        ])
        .build()?;
    let second = first.clone().into_builder().name("Second").build()?;
    let expected = json!([
        {"name": "First", "data": [{"label": "A", "value": 1}, {"label": "B", "value": 2}]},
        {"name": "Second", "data": [{"label": "A", "value": 1}, {"label": "B", "value": 2}]}
    ]);
    let area = AreaChart::builder()
        .axis_config(axis.clone())
        .series([first.clone()])
        .build()?;
    assert_eq!(
        wire(area.into_builder().series_entry(second.clone()).build()?)["series"],
        expected
    );
    let bar = BarChart::builder()
        .axis_config(axis.clone())
        .series([first.clone()])
        .build()?;
    assert_eq!(
        wire(bar.into_builder().series_entry(second.clone()).build()?)["series"],
        expected
    );
    let line = LineChart::builder()
        .axis_config(axis)
        .series([first])
        .build()?;
    assert_eq!(
        wire(line.into_builder().series_entry(second).build()?)["series"],
        expected
    );

    let first = TaskCardBlock::builder()
        .task_id("first")
        .title("First")
        .status(TaskStatus::Pending)
        .build()?;
    let second = first
        .clone()
        .into_builder()
        .task_id("second")
        .title("Second")
        .build()?;
    let plan = PlanBlock::builder().title("Plan").tasks([first]).build()?;
    assert_eq!(
        wire(plan.into_builder().task(second).build()?)["tasks"],
        json!([
            {"task_id": "first", "title": "First", "status": "pending"},
            {"task_id": "second", "title": "Second", "status": "pending"}
        ])
    );
    Ok(())
}
