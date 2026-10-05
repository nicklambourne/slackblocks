// Public accessor/editing and ingress regression tests; hand-maintained.
use serde_json::{Value, json};
use slackblocks::*;
fn wire(v: impl serde::Serialize) -> Value {
    serde_json::to_value(v).unwrap()
}

#[test]
fn actions_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ActionsBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(ActionsBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ActionsBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["elements"],
            json!([
                {
                    "type": "checkboxes",
                    "action_id": "actionId-0",
                    "options": [
                        {
                            "text": {
                                "type": "mrkdwn",
                                "text": "*a*"
                            },
                            "value": "a",
                            "description": {
                                "type": "plain_text",
                                "text": "*a*"
                            }
                        },
                        {
                            "text": {
                                "type": "mrkdwn",
                                "text": "*b*"
                            },
                            "value": "b",
                            "description": {
                                "type": "plain_text",
                                "text": "*b*"
                            }
                        },
                        {
                            "text": {
                                "type": "mrkdwn",
                                "text": "*c*"
                            },
                            "value": "c",
                            "description": {
                                "type": "plain_text",
                                "text": "*c*"
                            }
                        }
                    ]
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
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
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "checkboxes",
                "action_id": "actionId-0",
                "options": [
                    {
                        "text": {
                            "type": "mrkdwn",
                            "text": "*a*"
                        },
                        "value": "a",
                        "description": {
                            "type": "plain_text",
                            "text": "*a*"
                        }
                    },
                    {
                        "text": {
                            "type": "mrkdwn",
                            "text": "*b*"
                        },
                        "value": "b",
                        "description": {
                            "type": "plain_text",
                            "text": "*b*"
                        }
                    },
                    {
                        "text": {
                            "type": "mrkdwn",
                            "text": "*c*"
                        },
                        "value": "c",
                        "description": {
                            "type": "plain_text",
                            "text": "*c*"
                        }
                    }
                ]
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(ActionsBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(ActionsBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(ActionsBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ActionsBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ActionsBlock::try_from(Value::Null).is_err());
    assert!(ActionsBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ActionsBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn alert_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = AlertBlock::builder()
        .block_id("fake_block_id")
        .text(
            MarkdownText::builder()
                .text("The work is mysterious and important.")
                .build()?,
        )
        .level(AlertLevel::Info)
        .build()?;
    let expected = wire(&original);
    assert_eq!(AlertBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<AlertBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(
                MarkdownText::builder()
                    .text("The work is mysterious and important.")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "mrkdwn", "text": "The work is mysterious and important."})
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(AlertBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(AlertBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.level()),
        expected.get("level").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .level(AlertLevel::Info)
            .build()?;
        assert_eq!(wire(edited)["level"], Value::from("info"));
    }
    {
        let edited = original.clone().into_builder().clear_level().build()?;
        assert!(wire(edited).get("level").is_none());
    }
    let mut invalid = expected.clone();
    invalid["level"] = Value::from("not-a-variant");
    assert!(AlertBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(AlertBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(AlertBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(AlertBlock::try_from(Value::Null).is_err());
    assert!(AlertBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(AlertBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn card_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = CardBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(CardBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<CardBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.hero_image()),
        expected.get("hero_image").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .hero_image(
                ImageElement::builder()
                    .image_url("https://picsum.photos/400/300")
                    .alt_text("Sample hero image")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["hero_image"],
            json!({"type": "image", "image_url": "https://picsum.photos/400/300", "alt_text": "Sample hero image"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_hero_image().build()?;
        assert!(wire(edited).get("hero_image").is_none());
    }
    let mut invalid = expected.clone();
    invalid["hero_image"] = json!([]);
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.icon()),
        expected.get("icon").cloned().unwrap_or(Value::Null)
    );
    {
        let error = (original
            .clone()
            .into_builder()
            .icon(
                ImageElement::builder()
                    .image_url("https://picsum.photos/400/300")
                    .alt_text("Sample hero image")
                    .build()?,
            )
            .build())
        .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "CardBlock");
    }
    {
        let edited = original.clone().into_builder().clear_icon().build()?;
        assert!(wire(edited).get("icon").is_none());
    }
    let mut invalid = expected.clone();
    invalid["icon"] = json!([]);
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.title()),
        expected.get("title").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .title(MarkdownText::builder().text("Lumon Industries").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["title"],
            json!({"type": "mrkdwn", "text": "Lumon Industries"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_title().build()?;
        assert!(wire(edited).get("title").is_none());
    }
    let mut invalid = expected.clone();
    invalid["title"] = json!([]);
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.subtitle()),
        expected.get("subtitle").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .subtitle(
                MarkdownText::builder()
                    .text("Committed to work-life balance")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["subtitle"],
            json!({"type": "mrkdwn", "text": "Committed to work-life balance"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_subtitle().build()?;
        assert!(wire(edited).get("subtitle").is_none());
    }
    let mut invalid = expected.clone();
    invalid["subtitle"] = json!([]);
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.body()),
        expected.get("body").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .body(
                MarkdownText::builder()
                    .text("Please enjoy each card equally.")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["body"],
            json!({"type": "mrkdwn", "text": "Please enjoy each card equally."})
        );
    }
    {
        let edited = original.clone().into_builder().clear_body().build()?;
        assert!(wire(edited).get("body").is_none());
    }
    let mut invalid = expected.clone();
    invalid["body"] = json!([]);
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.actions()),
        expected.get("actions").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .actions(vec![
                ButtonElement::builder()
                    .text(PlainText::builder().text("Action Button").build()?)
                    .action_id("button_action")
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["actions"],
            json!([
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "Action Button"
                    },
                    "action_id": "button_action"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .action(
            ButtonElement::builder()
                .text(PlainText::builder().text("Action Button").build()?)
                .action_id("button_action")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["actions"].as_array().unwrap().len(),
            expected["actions"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["actions"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": "Action Button"
                },
                "action_id": "button_action"
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_actions().build()?;
        assert!(wire(edited).get("actions").is_none());
    }
    let mut invalid = expected.clone();
    invalid["actions"] = json!({});
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.slack_icon()),
        expected.get("slack_icon").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .slack_icon(SlackIcon::builder().name("bot").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["slack_icon"],
            json!({"type": "icon", "name": "bot"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_slack_icon().build()?;
        assert!(wire(edited).get("slack_icon").is_none());
    }
    let mut invalid = expected.clone();
    invalid["slack_icon"] = json!([]);
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.subtext()),
        expected.get("subtext").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .subtext(
                MarkdownText::builder()
                    .text("A card assembled by slackblocks.")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["subtext"],
            json!({"type": "mrkdwn", "text": "A card assembled by slackblocks."})
        );
    }
    {
        let edited = original.clone().into_builder().clear_subtext().build()?;
        assert!(wire(edited).get("subtext").is_none());
    }
    let mut invalid = expected.clone();
    invalid["subtext"] = json!([]);
    assert!(CardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(CardBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(CardBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(CardBlock::try_from(Value::Null).is_err());
    assert!(CardBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(CardBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn carousel_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = CarouselBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(CarouselBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<CarouselBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["elements"],
            json!([
                {
                    "type": "card",
                    "block_id": "card_1",
                    "title": {
                        "type": "mrkdwn",
                        "text": "First result"
                    }
                },
                {
                    "type": "card",
                    "block_id": "card_2",
                    "title": {
                        "type": "mrkdwn",
                        "text": "Second result"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
            CardBlock::builder()
                .block_id("card_1")
                .title(MarkdownText::builder().text("First result").build()?)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({"type": "card", "block_id": "card_1", "title": {"type": "mrkdwn", "text": "First result"}})
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(CarouselBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(CarouselBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(CarouselBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(CarouselBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(CarouselBlock::try_from(Value::Null).is_err());
    assert!(CarouselBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(CarouselBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn container_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ContainerBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(ContainerBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ContainerBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.child_blocks()), expected["child_blocks"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["child_blocks"],
            json!([
                {
                    "type": "section",
                    "block_id": "child_1",
                    "text": {
                        "type": "mrkdwn",
                        "text": "All systems operational."
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .child_block(
            SectionBlock::builder()
                .block_id("child_1")
                .text(
                    MarkdownText::builder()
                        .text("All systems operational.")
                        .build()?,
                )
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["child_blocks"].as_array().unwrap().len(),
            expected["child_blocks"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["child_blocks"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "section",
                "block_id": "child_1",
                "text": {
                    "type": "mrkdwn",
                    "text": "All systems operational."
                }
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["child_blocks"] = json!({});
    assert!(ContainerBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("child_blocks");
    assert!(ContainerBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.title()),
        expected.get("title").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .title(PlainText::builder().text("Deployment summary").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["title"],
            json!({"type": "plain_text", "text": "Deployment summary"})
        );
    }
    {
        let error = (original.clone().into_builder().clear_title().build())
            .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MissingRequired);
        assert_eq!(error.path(), "ContainerBlock");
    }
    let mut invalid = expected.clone();
    invalid["title"] = json!([]);
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.rich_text_title()),
        expected
            .get("rich_text_title")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .rich_text_title(
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
        assert_eq!(
            wire(edited)["rich_text_title"],
            json!({
                "type": "rich_text",
                "block_id": "plan_output",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Profile data loaded"
                            }
                        ]
                    }
                ]
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_rich_text_title()
            .build()?;
        assert!(wire(edited).get("rich_text_title").is_none());
    }
    let mut invalid = expected.clone();
    invalid["rich_text_title"] = json!([]);
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.subtitle()),
        expected.get("subtitle").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .subtitle(
                MarkdownText::builder()
                    .text("Production is healthy")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["subtitle"],
            json!({"type": "mrkdwn", "text": "Production is healthy"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_subtitle().build()?;
        assert!(wire(edited).get("subtitle").is_none());
    }
    let mut invalid = expected.clone();
    invalid["subtitle"] = json!([]);
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.width()),
        expected.get("width").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .width(ContainerWidth::Standard)
            .build()?;
        assert_eq!(wire(edited)["width"], Value::from("standard"));
    }
    {
        let edited = original.clone().into_builder().clear_width().build()?;
        assert!(wire(edited).get("width").is_none());
    }
    let mut invalid = expected.clone();
    invalid["width"] = Value::from("not-a-variant");
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.icon()),
        expected.get("icon").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .icon(
                ImageElement::builder()
                    .image_url("https://picsum.photos/400/300")
                    .alt_text("Sample hero image")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["icon"],
            json!({"type": "image", "image_url": "https://picsum.photos/400/300", "alt_text": "Sample hero image"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_icon().build()?;
        assert!(wire(edited).get("icon").is_none());
    }
    let mut invalid = expected.clone();
    invalid["icon"] = json!([]);
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.is_collapsible()),
        expected
            .get("is_collapsible")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .is_collapsible(false)
            .build()?;
        assert_eq!(wire(edited)["is_collapsible"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_is_collapsible()
            .build()?;
        assert!(wire(edited).get("is_collapsible").is_none());
    }
    let mut invalid = expected.clone();
    invalid["is_collapsible"] = serde_json::json!(0);
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.default_collapsed()),
        expected
            .get("default_collapsed")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .default_collapsed(false)
            .build()?;
        assert_eq!(wire(edited)["default_collapsed"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_default_collapsed()
            .build()?;
        assert!(wire(edited).get("default_collapsed").is_none());
    }
    let mut invalid = expected.clone();
    invalid["default_collapsed"] = serde_json::json!(0);
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.has_header_divider()),
        expected
            .get("has_header_divider")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .has_header_divider(true)
            .build()?;
        assert_eq!(wire(edited)["has_header_divider"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_has_header_divider()
            .build()?;
        assert!(wire(edited).get("has_header_divider").is_none());
    }
    let mut invalid = expected.clone();
    invalid["has_header_divider"] = serde_json::json!(0);
    assert!(ContainerBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(ContainerBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ContainerBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ContainerBlock::try_from(Value::Null).is_err());
    assert!(ContainerBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ContainerBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn context_actions_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ContextActionsBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(ContextActionsBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ContextActionsBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["elements"],
            json!([
                {
                    "type": "feedback_buttons",
                    "positive_button": {
                        "text": {
                            "type": "plain_text",
                            "text": "Good"
                        },
                        "value": "positive_feedback",
                        "accessibility_label": "Mark this response as good"
                    },
                    "negative_button": {
                        "text": {
                            "type": "plain_text",
                            "text": "Bad"
                        },
                        "value": "negative_feedback",
                        "accessibility_label": "Mark this response as bad"
                    },
                    "action_id": "feedback_buttons_1"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
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
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "feedback_buttons",
                "positive_button": {
                    "text": {
                        "type": "plain_text",
                        "text": "Good"
                    },
                    "value": "positive_feedback",
                    "accessibility_label": "Mark this response as good"
                },
                "negative_button": {
                    "text": {
                        "type": "plain_text",
                        "text": "Bad"
                    },
                    "value": "negative_feedback",
                    "accessibility_label": "Mark this response as bad"
                },
                "action_id": "feedback_buttons_1"
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(ContextActionsBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(ContextActionsBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(ContextActionsBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ContextActionsBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ContextActionsBlock::try_from(Value::Null).is_err());
    assert!(ContextActionsBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ContextActionsBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn context_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ContextBlock::builder()
        .block_id("fake_block_id")
        .elements(vec![ContextElement::from(
            MarkdownText::builder().text("Hello, world!").build()?,
        )])
        .build()?;
    let expected = wire(&original);
    assert_eq!(ContextBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ContextBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .elements(vec![ContextElement::from(
                MarkdownText::builder().text("Hello, world!").build()?,
            )])
            .build()?;
        assert_eq!(
            wire(edited)["elements"],
            json!([{"type": "mrkdwn", "text": "Hello, world!"}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(MarkdownText::builder().text("Hello, world!").build()?)
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({"type": "mrkdwn", "text": "Hello, world!"})
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(ContextBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(ContextBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(ContextBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ContextBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ContextBlock::try_from(Value::Null).is_err());
    assert!(ContextBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ContextBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn data_table_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DataTableBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(DataTableBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<DataTableBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.rows()), expected["rows"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["rows"],
            Value::Array(vec![
                json!([{"type": "raw_text", "text": "Name"}, {"type": "raw_text", "text": "Score"}]),
                Value::Array(vec![
                    json!({"type": "raw_text", "text": "Alice"}),
                    Value::Object(
                        [
                            ("type".into(), Value::from("raw_number")),
                            ("value".into(), serde_json::json!(42)),
                            ("text".into(), Value::from("42"))
                        ]
                        .into_iter()
                        .collect()
                    )
                ])
            ])
        );
    }
    {
        let value = original
            .clone()
            .into_builder()
            .row(vec![
                DataTableCell::from(RawText::builder().text("Name").build()?),
                DataTableCell::from(RawText::builder().text("Score").build()?),
            ])
            .build()?;
        assert_eq!(value.rows().len(), original.rows().len() + 1);
        assert_eq!(
            wire(value)["rows"].as_array().unwrap().last().unwrap(),
            &json!([{"type": "raw_text", "text": "Name"}, {"type": "raw_text", "text": "Score"}])
        );
    }
    let mut invalid = expected.clone();
    invalid["rows"] = json!({});
    assert!(DataTableBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("rows");
    assert!(DataTableBlock::try_from(missing).is_err());
    assert_eq!(wire(original.caption()), expected["caption"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .caption("Team scores")
            .build()?;
        assert_eq!(wire(edited)["caption"], Value::from("Team scores"));
    }
    let mut invalid = expected.clone();
    invalid["caption"] = Value::from(false);
    assert!(DataTableBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("caption");
    assert!(DataTableBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.page_size()),
        expected.get("page_size").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().page_size(5_i64).build()?;
        assert_eq!(wire(edited)["page_size"], serde_json::json!(5));
    }
    {
        let edited = original.clone().into_builder().clear_page_size().build()?;
        assert!(wire(edited).get("page_size").is_none());
    }
    let mut invalid = expected.clone();
    invalid["page_size"] = Value::from("wrong");
    assert!(DataTableBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.row_header_column_index()),
        expected
            .get("row_header_column_index")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .row_header_column_index(0_i64)
            .build()?;
        assert_eq!(
            wire(edited)["row_header_column_index"],
            serde_json::json!(0)
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_row_header_column_index()
            .build()?;
        assert!(wire(edited).get("row_header_column_index").is_none());
    }
    let mut invalid = expected.clone();
    invalid["row_header_column_index"] = Value::from("wrong");
    assert!(DataTableBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(DataTableBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(DataTableBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DataTableBlock::try_from(Value::Null).is_err());
    assert!(DataTableBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(DataTableBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn data_visualization_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DataVisualizationBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(
        DataVisualizationBlock::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<DataVisualizationBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.title()), expected["title"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .title("Daily Active Users")
            .build()?;
        assert_eq!(wire(edited)["title"], Value::from("Daily Active Users"));
    }
    let mut invalid = expected.clone();
    invalid["title"] = Value::from(false);
    assert!(DataVisualizationBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("title");
    assert!(DataVisualizationBlock::try_from(missing).is_err());
    assert_eq!(wire(original.chart()), expected["chart"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["chart"],
            Value::Object(
                [
                    ("type".into(), Value::from("area")),
                    (
                        "series".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("name".into(), Value::from("Free Tier")),
                                (
                                    "data".into(),
                                    Value::Array(vec![
                                        Value::Object(
                                            [
                                                ("label".into(), Value::from("Mon")),
                                                ("value".into(), serde_json::json!(12000))
                                            ]
                                            .into_iter()
                                            .collect()
                                        ),
                                        Value::Object(
                                            [
                                                ("label".into(), Value::from("Tue")),
                                                ("value".into(), serde_json::json!(13500))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ])
                                )
                            ]
                            .into_iter()
                            .collect()
                        )])
                    ),
                    (
                        "axis_config".into(),
                        json!({"categories": ["Mon", "Tue"], "x_label": "Day", "y_label": "Users"})
                    )
                ]
                .into_iter()
                .collect()
            )
        );
    }
    let mut invalid = expected.clone();
    invalid["chart"] = json!([]);
    assert!(DataVisualizationBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("chart");
    assert!(DataVisualizationBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(DataVisualizationBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(DataVisualizationBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DataVisualizationBlock::try_from(Value::Null).is_err());
    assert!(DataVisualizationBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(DataVisualizationBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn divider_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DividerBlock::builder().block_id("fake_block_id").build()?;
    let expected = wire(&original);
    assert_eq!(DividerBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<DividerBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(DividerBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(DividerBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DividerBlock::try_from(Value::Null).is_err());
    assert!(DividerBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(DividerBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn file_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = FileBlock::builder()
        .external_id("external_id")
        .source("remote")
        .block_id("fake_block_id")
        .build()?;
    let expected = wire(&original);
    assert_eq!(FileBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<FileBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.external_id()), expected["external_id"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .external_id("external_id")
            .build()?;
        assert_eq!(wire(edited)["external_id"], Value::from("external_id"));
    }
    let mut invalid = expected.clone();
    invalid["external_id"] = Value::from(false);
    assert!(FileBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("external_id");
    assert!(FileBlock::try_from(missing).is_err());
    assert_eq!(wire(original.source()), expected["source"]);
    {
        let edited = original.clone().into_builder().source("remote").build()?;
        assert_eq!(wire(edited)["source"], Value::from("remote"));
    }
    let mut invalid = expected.clone();
    invalid["source"] = Value::from(false);
    assert!(FileBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("source");
    assert!(FileBlock::try_from(missing).is_ok());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(FileBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(FileBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(FileBlock::try_from(Value::Null).is_err());
    assert!(FileBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(FileBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn header_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = HeaderBlock::builder()
        .block_id("fake_block_id")
        .text(PlainText::builder().text("😀".repeat(150)).build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(HeaderBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<HeaderBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(PlainText::builder().text("😀".repeat(150)).build()?)
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            Value::Object(
                [
                    ("type".into(), Value::from("plain_text")),
                    ("text".into(), Value::from("😀".repeat(150)))
                ]
                .into_iter()
                .collect()
            )
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(HeaderBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(HeaderBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(HeaderBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(HeaderBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(HeaderBlock::try_from(Value::Null).is_err());
    assert!(HeaderBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(HeaderBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn image_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ImageBlock::builder()
        .block_id("fake_block_id")
        .image_url("https://api.slack.com/img/blocks/bkb_template_images/beagle.png")
        .alt_text("image1")
        .title(PlainText::builder().text("image1").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(ImageBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ImageBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.image_url()),
        expected.get("image_url").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .image_url("https://api.slack.com/img/blocks/bkb_template_images/beagle.png")
            .build()?;
        assert_eq!(
            wire(edited)["image_url"],
            Value::from("https://api.slack.com/img/blocks/bkb_template_images/beagle.png")
        );
    }
    {
        let error = (original.clone().into_builder().clear_image_url().build())
            .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MissingRequired);
        assert_eq!(error.path(), "ImageBlock");
    }
    let mut invalid = expected.clone();
    invalid["image_url"] = Value::from(false);
    assert!(ImageBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.slack_file()),
        expected.get("slack_file").cloned().unwrap_or(Value::Null)
    );
    {
        let error = (original
            .clone()
            .into_builder()
            .slack_file(
                SlackFile::builder()
                    .url("https://files.slack.com/files-pri/T0123456-F0123ABC456/kitten.png")
                    .build()?,
            )
            .build())
        .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "ImageBlock");
    }
    {
        let edited = original.clone().into_builder().clear_slack_file().build()?;
        assert!(wire(edited).get("slack_file").is_none());
    }
    let mut invalid = expected.clone();
    invalid["slack_file"] = json!([]);
    assert!(ImageBlock::try_from(invalid).is_err());
    assert_eq!(wire(original.alt_text()), expected["alt_text"]);
    {
        let edited = original.clone().into_builder().alt_text("image1").build()?;
        assert_eq!(wire(edited)["alt_text"], Value::from("image1"));
    }
    let mut invalid = expected.clone();
    invalid["alt_text"] = Value::from(false);
    assert!(ImageBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("alt_text");
    assert!(ImageBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.title()),
        expected.get("title").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .title(PlainText::builder().text("image1").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["title"],
            json!({"type": "plain_text", "text": "image1"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_title().build()?;
        assert!(wire(edited).get("title").is_none());
    }
    let mut invalid = expected.clone();
    invalid["title"] = json!([]);
    assert!(ImageBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(ImageBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ImageBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ImageBlock::try_from(Value::Null).is_err());
    assert!(ImageBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ImageBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn input_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = InputBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(InputBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<InputBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.label()), expected["label"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .label(PlainText::builder().text("Label").emoji(true).build()?)
            .build()?;
        assert_eq!(
            wire(edited)["label"],
            json!({"type": "plain_text", "text": "Label", "emoji": true})
        );
    }
    let mut invalid = expected.clone();
    invalid["label"] = json!([]);
    assert!(InputBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("label");
    assert!(InputBlock::try_from(missing).is_err());
    assert_eq!(wire(original.element()), expected["element"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .element(
                PlainTextInputElement::builder()
                    .action_id("action")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["element"],
            json!({"type": "plain_text_input", "action_id": "action"})
        );
    }
    let mut invalid = expected.clone();
    invalid["element"] = json!([]);
    assert!(InputBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("element");
    assert!(InputBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.dispatch_action()),
        expected
            .get("dispatch_action")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .dispatch_action(true)
            .build()?;
        assert_eq!(wire(edited)["dispatch_action"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_dispatch_action()
            .build()?;
        assert!(wire(edited).get("dispatch_action").is_none());
    }
    let mut invalid = expected.clone();
    invalid["dispatch_action"] = serde_json::json!(0);
    assert!(InputBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(InputBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.hint()),
        expected.get("hint").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .hint(PlainText::builder().text("Hint").emoji(true).build()?)
            .build()?;
        assert_eq!(
            wire(edited)["hint"],
            json!({"type": "plain_text", "text": "Hint", "emoji": true})
        );
    }
    {
        let edited = original.clone().into_builder().clear_hint().build()?;
        assert!(wire(edited).get("hint").is_none());
    }
    let mut invalid = expected.clone();
    invalid["hint"] = json!([]);
    assert!(InputBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.optional()),
        expected.get("optional").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().optional(true).build()?;
        assert_eq!(wire(edited)["optional"], Value::from(true));
    }
    {
        let edited = original.clone().into_builder().clear_optional().build()?;
        assert!(wire(edited).get("optional").is_none());
    }
    let mut invalid = expected.clone();
    invalid["optional"] = serde_json::json!(0);
    assert!(InputBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(InputBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(InputBlock::try_from(Value::Null).is_err());
    assert!(InputBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(InputBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn markdown_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = MarkdownBlock::builder()
        .block_id("fake_block_id")
        .text("**Hello**, _world_!")
        .build()?;
    let expected = wire(&original);
    assert_eq!(MarkdownBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<MarkdownBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text("**Hello**, _world_!")
            .build()?;
        assert_eq!(wire(edited)["text"], Value::from("**Hello**, _world_!"));
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(MarkdownBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(MarkdownBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(MarkdownBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(MarkdownBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(MarkdownBlock::try_from(Value::Null).is_err());
    assert!(MarkdownBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(MarkdownBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn plan_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = PlanBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(PlanBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<PlanBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.title()), expected["title"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .title("Thinking completed")
            .build()?;
        assert_eq!(wire(edited)["title"], Value::from("Thinking completed"));
    }
    let mut invalid = expected.clone();
    invalid["title"] = Value::from(false);
    assert!(PlanBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("title");
    assert!(PlanBlock::try_from(missing).is_err());
    let mut tasks = wire(original.tasks());
    for task in tasks.as_array_mut().unwrap() {
        assert_eq!(task["type"], "task_card");
        task.as_object_mut().unwrap().remove("type");
    }
    assert_eq!(tasks, expected["tasks"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["tasks"],
            json!([
                {
                    "task_id": "call_001",
                    "title": "Fetched user profile information",
                    "status": "complete",
                    "output": {
                        "type": "rich_text",
                        "block_id": "plan_output",
                        "elements": [
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "text",
                                        "text": "Profile data loaded"
                                    }
                                ]
                            }
                        ]
                    }
                },
                {
                    "task_id": "call_002",
                    "title": "Checked user permissions",
                    "status": "pending"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .task(
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
        )
        .build();
    {
        let error =
            (appended).expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::InvalidUsage);
        assert_eq!(error.path(), "PlanBlock.tasks");
    }
    let mut invalid = expected.clone();
    invalid["tasks"] = json!({});
    assert!(PlanBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("tasks");
    assert!(PlanBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(PlanBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(PlanBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(PlanBlock::try_from(Value::Null).is_err());
    assert!(PlanBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(PlanBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(RichTextBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["elements"],
            json!([{"type": "rich_text_section", "elements": [{"type": "text", "text": "Profile data loaded"}]}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder()
                        .text("Profile data loaded")
                        .build()?,
                )])
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({"type": "rich_text_section", "elements": [{"type": "text", "text": "Profile data loaded"}]})
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(RichTextBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(RichTextBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("plan_output")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("plan_output"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(RichTextBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextBlock::try_from(Value::Null).is_err());
    assert!(RichTextBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn section_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = SectionBlock::builder()
        .block_id("fake_block_id")
        .text(MarkdownText::builder().text("Hello").build()?)
        .fields(vec![
            TextInput::from(MarkdownText::builder().text("Are you").build()?),
            TextInput::from(PlainText::builder().text("There?").emoji(true).build()?),
        ])
        .build()?;
    let expected = wire(&original);
    assert_eq!(SectionBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<SectionBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.text()),
        expected.get("text").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .text(
                MarkdownText::builder()
                    .text("I like pretty colours")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "mrkdwn", "text": "I like pretty colours"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_text().build()?;
        assert!(wire(edited).get("text").is_none());
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(SectionBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.fields()),
        expected.get("fields").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .fields(vec![
                TextInput::from(MarkdownText::builder().text("Are you").build()?),
                TextInput::from(PlainText::builder().text("There?").emoji(true).build()?),
            ])
            .build()?;
        assert_eq!(
            wire(edited)["fields"],
            json!([{"type": "mrkdwn", "text": "Are you"}, {"type": "plain_text", "text": "There?", "emoji": true}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .field(MarkdownText::builder().text("Are you").build()?)
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["fields"].as_array().unwrap().len(),
            expected["fields"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["fields"].as_array().unwrap().last().unwrap(),
            &json!({"type": "mrkdwn", "text": "Are you"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_fields().build()?;
        assert!(wire(edited).get("fields").is_none());
    }
    let mut invalid = expected.clone();
    invalid["fields"] = json!({});
    assert!(SectionBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.accessory()),
        expected.get("accessory").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .accessory(
                ButtonElement::builder()
                    .text(PlainText::builder().text("Action Button").build()?)
                    .action_id("button_action")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["accessory"],
            json!({
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": "Action Button"
                },
                "action_id": "button_action"
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_accessory().build()?;
        assert!(wire(edited).get("accessory").is_none());
    }
    let mut invalid = expected.clone();
    invalid["accessory"] = json!([]);
    assert!(SectionBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id_0")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id_0"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(SectionBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(SectionBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(SectionBlock::try_from(Value::Null).is_err());
    assert!(SectionBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(SectionBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn table_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = TableBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(TableBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<TableBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.rows()), expected["rows"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["rows"],
            json!([
                [
                    {
                        "type": "raw_text",
                        "text": "Header A"
                    },
                    {
                        "type": "raw_text",
                        "text": "Header B"
                    }
                ],
                [
                    {
                        "type": "raw_text",
                        "text": "Data 1A"
                    },
                    {
                        "type": "rich_text",
                        "elements": [
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "link",
                                        "url": "https://slack.com",
                                        "text": "Data 1B"
                                    }
                                ]
                            }
                        ]
                    }
                ],
                [
                    {
                        "type": "raw_text",
                        "text": "Data 2A"
                    },
                    {
                        "type": "rich_text",
                        "elements": [
                            {
                                "type": "rich_text_section",
                                "elements": [
                                    {
                                        "type": "link",
                                        "url": "https://slack.com",
                                        "text": "Data 2B"
                                    }
                                ]
                            }
                        ]
                    }
                ]
            ])
        );
    }
    {
        let value = original
            .clone()
            .into_builder()
            .row(vec![
                TableCell::from(RawText::builder().text("Header A").build()?),
                TableCell::from(RawText::builder().text("Header B").build()?),
            ])
            .build()?;
        assert_eq!(value.rows().len(), original.rows().len() + 1);
        assert_eq!(
            wire(value)["rows"].as_array().unwrap().last().unwrap(),
            &json!([{"type": "raw_text", "text": "Header A"}, {"type": "raw_text", "text": "Header B"}])
        );
    }
    let mut invalid = expected.clone();
    invalid["rows"] = json!({});
    assert!(TableBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("rows");
    assert!(TableBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.column_settings()),
        expected
            .get("column_settings")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .column_settings(vec![
                ColumnSettings::builder().is_wrapped(true).build()?,
                ColumnSettings::builder()
                    .align(ColumnAlign::Right)
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["column_settings"],
            json!([{"is_wrapped": true}, {"align": "right"}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .column_setting(ColumnSettings::builder().is_wrapped(true).build()?)
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["column_settings"].as_array().unwrap().len(),
            expected["column_settings"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["column_settings"].as_array().unwrap().last().unwrap(),
            &json!({"is_wrapped": true})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_column_settings()
            .build()?;
        assert!(wire(edited).get("column_settings").is_none());
    }
    let mut invalid = expected.clone();
    invalid["column_settings"] = json!({});
    assert!(TableBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(TableBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(TableBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(TableBlock::try_from(Value::Null).is_err());
    assert!(TableBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(TableBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn task_card_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = TaskCardBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(TaskCardBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<TaskCardBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.task_id()), expected["task_id"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .task_id("call_001")
            .build()?;
        assert_eq!(wire(edited)["task_id"], Value::from("call_001"));
    }
    let mut invalid = expected.clone();
    invalid["task_id"] = Value::from(false);
    assert!(TaskCardBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("task_id");
    assert!(TaskCardBlock::try_from(missing).is_err());
    assert_eq!(wire(original.title()), expected["title"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .title("Fetched user profile information")
            .build()?;
        assert_eq!(
            wire(edited)["title"],
            Value::from("Fetched user profile information")
        );
    }
    let mut invalid = expected.clone();
    invalid["title"] = Value::from(false);
    assert!(TaskCardBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("title");
    assert!(TaskCardBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.details()),
        expected.get("details").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .details(
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
        assert_eq!(
            wire(edited)["details"],
            json!({
                "type": "rich_text",
                "block_id": "plan_output",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Profile data loaded"
                            }
                        ]
                    }
                ]
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_details().build()?;
        assert!(wire(edited).get("details").is_none());
    }
    let mut invalid = expected.clone();
    invalid["details"] = json!([]);
    assert!(TaskCardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.output()),
        expected.get("output").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["output"],
            json!({
                "type": "rich_text",
                "block_id": "plan_output",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "Profile data loaded"
                            }
                        ]
                    }
                ]
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_output().build()?;
        assert!(wire(edited).get("output").is_none());
    }
    let mut invalid = expected.clone();
    invalid["output"] = json!([]);
    assert!(TaskCardBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.sources()),
        expected.get("sources").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["sources"],
            json!([
                {
                    "type": "url",
                    "url": "https://weather.com/",
                    "text": "weather.com"
                },
                {
                    "type": "url",
                    "url": "https://www.accuweather.com/",
                    "text": "accuweather.com"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .source(
            UrlSource::builder()
                .url("https://weather.com/")
                .text("weather.com")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["sources"].as_array().unwrap().len(),
            expected["sources"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["sources"].as_array().unwrap().last().unwrap(),
            &json!({"type": "url", "url": "https://weather.com/", "text": "weather.com"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_sources().build()?;
        assert!(wire(edited).get("sources").is_none());
    }
    let mut invalid = expected.clone();
    invalid["sources"] = json!({});
    assert!(TaskCardBlock::try_from(invalid).is_err());
    assert_eq!(wire(original.status()), expected["status"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .status(TaskStatus::Complete)
            .build()?;
        assert_eq!(wire(edited)["status"], Value::from("complete"));
    }
    let mut invalid = expected.clone();
    invalid["status"] = Value::from("not-a-variant");
    assert!(TaskCardBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("status");
    assert!(TaskCardBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .block_id("fake_block_id")
            .build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("fake_block_id"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(TaskCardBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(TaskCardBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(TaskCardBlock::try_from(Value::Null).is_err());
    assert!(TaskCardBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(TaskCardBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn video_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = VideoBlock::builder()
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
    let expected = wire(&original);
    assert_eq!(VideoBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<VideoBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.alt_text()), expected["alt_text"]);
    {
        let edited = original.clone().into_builder().alt_text("alt").build()?;
        assert_eq!(wire(edited)["alt_text"], Value::from("alt"));
    }
    let mut invalid = expected.clone();
    invalid["alt_text"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("alt_text");
    assert!(VideoBlock::try_from(missing).is_err());
    assert_eq!(wire(original.thumbnail_url()), expected["thumbnail_url"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .thumbnail_url("https://example.com/t.png")
            .build()?;
        assert_eq!(
            wire(edited)["thumbnail_url"],
            Value::from("https://example.com/t.png")
        );
    }
    let mut invalid = expected.clone();
    invalid["thumbnail_url"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("thumbnail_url");
    assert!(VideoBlock::try_from(missing).is_err());
    assert_eq!(wire(original.title()), expected["title"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .title(PlainText::builder().text("Title").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["title"],
            json!({"type": "plain_text", "text": "Title"})
        );
    }
    let mut invalid = expected.clone();
    invalid["title"] = json!([]);
    assert!(VideoBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("title");
    assert!(VideoBlock::try_from(missing).is_err());
    assert_eq!(wire(original.video_url()), expected["video_url"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .video_url("https://example.com/v.mp4")
            .build()?;
        assert_eq!(
            wire(edited)["video_url"],
            Value::from("https://example.com/v.mp4")
        );
    }
    let mut invalid = expected.clone();
    invalid["video_url"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("video_url");
    assert!(VideoBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.block_id()),
        expected.get("block_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().block_id("b1").build()?;
        assert_eq!(wire(edited)["block_id"], Value::from("b1"));
    }
    {
        let edited = original.clone().into_builder().clear_block_id().build()?;
        assert!(wire(edited).get("block_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["block_id"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.author_name()),
        expected.get("author_name").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .author_name("Slack")
            .build()?;
        assert_eq!(wire(edited)["author_name"], Value::from("Slack"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_author_name()
            .build()?;
        assert!(wire(edited).get("author_name").is_none());
    }
    let mut invalid = expected.clone();
    invalid["author_name"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.description()),
        expected.get("description").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .description(PlainText::builder().text("A short intro").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["description"],
            json!({"type": "plain_text", "text": "A short intro"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_description()
            .build()?;
        assert!(wire(edited).get("description").is_none());
    }
    let mut invalid = expected.clone();
    invalid["description"] = json!([]);
    assert!(VideoBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.provider_icon_url()),
        expected
            .get("provider_icon_url")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .provider_icon_url("https://example.com/icon.png")
            .build()?;
        assert_eq!(
            wire(edited)["provider_icon_url"],
            Value::from("https://example.com/icon.png")
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_provider_icon_url()
            .build()?;
        assert!(wire(edited).get("provider_icon_url").is_none());
    }
    let mut invalid = expected.clone();
    invalid["provider_icon_url"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.provider_name()),
        expected
            .get("provider_name")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .provider_name("YouTube")
            .build()?;
        assert_eq!(wire(edited)["provider_name"], Value::from("YouTube"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_provider_name()
            .build()?;
        assert!(wire(edited).get("provider_name").is_none());
    }
    let mut invalid = expected.clone();
    invalid["provider_name"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    assert_eq!(
        wire(original.title_url()),
        expected.get("title_url").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .title_url("https://example.com")
            .build()?;
        assert_eq!(
            wire(edited)["title_url"],
            Value::from("https://example.com")
        );
    }
    {
        let edited = original.clone().into_builder().clear_title_url().build()?;
        assert!(wire(edited).get("title_url").is_none());
    }
    let mut invalid = expected.clone();
    invalid["title_url"] = Value::from(false);
    assert!(VideoBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(VideoBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(VideoBlock::try_from(Value::Null).is_err());
    assert!(VideoBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(VideoBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn button_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ButtonElement::builder()
        .text(PlainText::builder().text("Load").build()?)
        .action_id("button")
        .style(ButtonStyle::Primary)
        .value("im_a_style_button")
        .build()?;
    let expected = wire(&original);
    assert_eq!(ButtonElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ButtonElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(PlainText::builder().text("Action Button").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "plain_text", "text": "Action Button"})
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(ButtonElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(ButtonElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("button_action")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("button_action"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(ButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.url()),
        expected.get("url").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .url("https://ndl.im/")
            .build()?;
        assert_eq!(wire(edited)["url"], Value::from("https://ndl.im/"));
    }
    {
        let edited = original.clone().into_builder().clear_url().build()?;
        assert!(wire(edited).get("url").is_none());
    }
    let mut invalid = expected.clone();
    invalid["url"] = Value::from(false);
    assert!(ButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.value()),
        expected.get("value").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().value("click_me").build()?;
        assert_eq!(wire(edited)["value"], Value::from("click_me"));
    }
    {
        let edited = original.clone().into_builder().clear_value().build()?;
        assert!(wire(edited).get("value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from(false);
    assert!(ButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .style(ButtonStyle::Primary)
            .build()?;
        assert_eq!(wire(edited)["style"], Value::from("primary"));
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = Value::from("not-a-variant");
    assert!(ButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.accessibility_label()),
        expected
            .get("accessibility_label")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .accessibility_label("sample")
            .build()?;
        assert_eq!(wire(edited)["accessibility_label"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_accessibility_label()
            .build()?;
        assert!(wire(edited).get("accessibility_label").is_none());
    }
    let mut invalid = expected.clone();
    invalid["accessibility_label"] = Value::from(false);
    assert!(ButtonElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ButtonElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ButtonElement::try_from(Value::Null).is_err());
    assert!(ButtonElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ButtonElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn channel_multi_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ChannelMultiSelectElement::builder()
        .action_id("multi_channels_select")
        .placeholder(PlainText::builder().text("Select channels").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(
        ChannelMultiSelectElement::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<ChannelMultiSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("multi_channels_select")
            .build()?;
        assert_eq!(
            wire(edited)["action_id"],
            Value::from("multi_channels_select")
        );
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(ChannelMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_channels()),
        expected
            .get("initial_channels")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_channels(vec![String::from("sample")])
            .build()?;
        assert_eq!(wire(edited)["initial_channels"], json!(["sample"]));
    }
    let appended = original
        .clone()
        .into_builder()
        .initial_channel("sample")
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["initial_channels"].as_array().unwrap().len(),
            expected["initial_channels"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["initial_channels"].as_array().unwrap().last().unwrap(),
            &Value::from("sample")
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_channels()
            .build()?;
        assert!(wire(edited).get("initial_channels").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_channels"] = json!({});
    assert!(ChannelMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ChannelMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_selected_items()),
        expected
            .get("max_selected_items")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .max_selected_items(1_i64)
            .build()?;
        assert_eq!(wire(edited)["max_selected_items"], serde_json::json!(1));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_max_selected_items()
            .build()?;
        assert!(wire(edited).get("max_selected_items").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_selected_items"] = Value::from("wrong");
    assert!(ChannelMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(ChannelMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select channels").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select channels"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(ChannelMultiSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(
        ChannelMultiSelectElement::try_from(wire(&extended))?,
        extended
    );
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ChannelMultiSelectElement::try_from(Value::Null).is_err());
    assert!(ChannelMultiSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ChannelMultiSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn channel_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ChannelSelectElement::builder()
        .action_id("channels_select")
        .placeholder(PlainText::builder().text("Select a channel").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(ChannelSelectElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ChannelSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("channels_select")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("channels_select"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(ChannelSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_channel()),
        expected
            .get("initial_channel")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_channel("sample")
            .build()?;
        assert_eq!(wire(edited)["initial_channel"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_channel()
            .build()?;
        assert!(wire(edited).get("initial_channel").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_channel"] = Value::from(false);
    assert!(ChannelSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.response_url_enabled()),
        expected
            .get("response_url_enabled")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .response_url_enabled(true)
            .build()?;
        assert_eq!(wire(edited)["response_url_enabled"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_response_url_enabled()
            .build()?;
        assert!(wire(edited).get("response_url_enabled").is_none());
    }
    let mut invalid = expected.clone();
    invalid["response_url_enabled"] = serde_json::json!(0);
    assert!(ChannelSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ChannelSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(ChannelSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select a channel").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select a channel"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(ChannelSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ChannelSelectElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ChannelSelectElement::try_from(Value::Null).is_err());
    assert!(ChannelSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ChannelSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn checkboxes_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = CheckboxesElement::builder()
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
    let expected = wire(&original);
    assert_eq!(CheckboxesElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<CheckboxesElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("actionId-0")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("actionId-0"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(CheckboxesElement::try_from(invalid).is_err());
    assert_eq!(wire(original.options()), expected["options"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["options"],
            json!([
                {
                    "text": {
                        "type": "mrkdwn",
                        "text": "*a*"
                    },
                    "value": "a",
                    "description": {
                        "type": "plain_text",
                        "text": "*a*"
                    }
                },
                {
                    "text": {
                        "type": "mrkdwn",
                        "text": "*b*"
                    },
                    "value": "b",
                    "description": {
                        "type": "plain_text",
                        "text": "*b*"
                    }
                },
                {
                    "text": {
                        "type": "mrkdwn",
                        "text": "*c*"
                    },
                    "value": "c",
                    "description": {
                        "type": "plain_text",
                        "text": "*c*"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .option(
            SelectOption::builder()
                .text(MarkdownText::builder().text("*a*").build()?)
                .value("a")
                .description(PlainText::builder().text("*a*").build()?)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["options"].as_array().unwrap().len(),
            expected["options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["options"].as_array().unwrap().last().unwrap(),
            &json!({
                "text": {
                    "type": "mrkdwn",
                    "text": "*a*"
                },
                "value": "a",
                "description": {
                    "type": "plain_text",
                    "text": "*a*"
                }
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["options"] = json!({});
    assert!(CheckboxesElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("options");
    assert!(CheckboxesElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.initial_options()),
        expected
            .get("initial_options")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_options(vec![
                SelectOption::builder()
                    .text(PlainText::builder().text("A").build()?)
                    .value("A")
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["initial_options"],
            json!([{"text": {"type": "plain_text", "text": "A"}, "value": "A"}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .initial_option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["initial_options"].as_array().unwrap().len(),
            expected["initial_options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["initial_options"].as_array().unwrap().last().unwrap(),
            &json!({"text": {"type": "plain_text", "text": "A"}, "value": "A"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_options()
            .build()?;
        assert!(wire(edited).get("initial_options").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_options"] = json!({});
    assert!(CheckboxesElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(CheckboxesElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(CheckboxesElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(CheckboxesElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(CheckboxesElement::try_from(Value::Null).is_err());
    assert!(CheckboxesElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(CheckboxesElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn conversation_multi_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ConversationMultiSelectElement::builder()
        .action_id("multi_conversations_select")
        .placeholder(PlainText::builder().text("Select conversations").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(
        ConversationMultiSelectElement::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<ConversationMultiSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("multi_conversations_select")
            .build()?;
        assert_eq!(
            wire(edited)["action_id"],
            Value::from("multi_conversations_select")
        );
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_conversations()),
        expected
            .get("initial_conversations")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_conversations(vec![String::from("sample")])
            .build()?;
        assert_eq!(wire(edited)["initial_conversations"], json!(["sample"]));
    }
    let appended = original
        .clone()
        .into_builder()
        .initial_conversation("sample")
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["initial_conversations"]
                .as_array()
                .unwrap()
                .last()
                .unwrap(),
            &Value::from("sample")
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_conversations()
            .build()?;
        assert!(wire(edited).get("initial_conversations").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_conversations"] = json!({});
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.default_to_current_conversation()),
        expected
            .get("default_to_current_conversation")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .default_to_current_conversation(true)
            .build()?;
        assert_eq!(
            wire(edited)["default_to_current_conversation"],
            Value::from(true)
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_default_to_current_conversation()
            .build()?;
        assert!(
            wire(edited)
                .get("default_to_current_conversation")
                .is_none()
        );
    }
    let mut invalid = expected.clone();
    invalid["default_to_current_conversation"] = serde_json::json!(0);
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.filter()),
        expected.get("filter").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .filter(
                ConversationFilter::builder()
                    .include(vec![String::from("public"), String::from("mpim")])
                    .exclude_bot_users(true)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["filter"],
            json!({"include": ["public", "mpim"], "exclude_bot_users": true})
        );
    }
    {
        let edited = original.clone().into_builder().clear_filter().build()?;
        assert!(wire(edited).get("filter").is_none());
    }
    let mut invalid = expected.clone();
    invalid["filter"] = json!([]);
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_selected_items()),
        expected
            .get("max_selected_items")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .max_selected_items(1_i64)
            .build()?;
        assert_eq!(wire(edited)["max_selected_items"], serde_json::json!(1));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_max_selected_items()
            .build()?;
        assert!(wire(edited).get("max_selected_items").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_selected_items"] = Value::from("wrong");
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select conversations").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select conversations"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(ConversationMultiSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(
        ConversationMultiSelectElement::try_from(wire(&extended))?,
        extended
    );
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ConversationMultiSelectElement::try_from(Value::Null).is_err());
    assert!(ConversationMultiSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ConversationMultiSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn conversation_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ConversationSelectElement::builder()
        .action_id("conversations_select")
        .placeholder(
            PlainText::builder()
                .text("Select one conversation")
                .build()?,
        )
        .build()?;
    let expected = wire(&original);
    assert_eq!(
        ConversationSelectElement::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<ConversationSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("conversations_select")
            .build()?;
        assert_eq!(
            wire(edited)["action_id"],
            Value::from("conversations_select")
        );
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_conversation()),
        expected
            .get("initial_conversation")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_conversation("sample")
            .build()?;
        assert_eq!(wire(edited)["initial_conversation"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_conversation()
            .build()?;
        assert!(wire(edited).get("initial_conversation").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_conversation"] = Value::from(false);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.default_to_current_conversation()),
        expected
            .get("default_to_current_conversation")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .default_to_current_conversation(true)
            .build()?;
        assert_eq!(
            wire(edited)["default_to_current_conversation"],
            Value::from(true)
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_default_to_current_conversation()
            .build()?;
        assert!(
            wire(edited)
                .get("default_to_current_conversation")
                .is_none()
        );
    }
    let mut invalid = expected.clone();
    invalid["default_to_current_conversation"] = serde_json::json!(0);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.filter()),
        expected.get("filter").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .filter(
                ConversationFilter::builder()
                    .include(vec![String::from("public"), String::from("mpim")])
                    .exclude_bot_users(true)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["filter"],
            json!({"include": ["public", "mpim"], "exclude_bot_users": true})
        );
    }
    {
        let edited = original.clone().into_builder().clear_filter().build()?;
        assert!(wire(edited).get("filter").is_none());
    }
    let mut invalid = expected.clone();
    invalid["filter"] = json!([]);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.response_url_enabled()),
        expected
            .get("response_url_enabled")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .response_url_enabled(true)
            .build()?;
        assert_eq!(wire(edited)["response_url_enabled"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_response_url_enabled()
            .build()?;
        assert!(wire(edited).get("response_url_enabled").is_none());
    }
    let mut invalid = expected.clone();
    invalid["response_url_enabled"] = serde_json::json!(0);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(
                PlainText::builder()
                    .text("Select one conversation")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select one conversation"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(ConversationSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(
        ConversationSelectElement::try_from(wire(&extended))?,
        extended
    );
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ConversationSelectElement::try_from(Value::Null).is_err());
    assert!(ConversationSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ConversationSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn date_picker_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DatePickerElement::builder()
        .action_id("datepicker")
        .initial_date("1970-01-01")
        .placeholder(PlainText::builder().text("Pick a date").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(DatePickerElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<DatePickerElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("datepicker")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("datepicker"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(DatePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_date()),
        expected.get("initial_date").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_date("1970-01-01")
            .build()?;
        assert_eq!(wire(edited)["initial_date"], Value::from("1970-01-01"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_date()
            .build()?;
        assert!(wire(edited).get("initial_date").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_date"] = Value::from(false);
    assert!(DatePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(DatePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(DatePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Pick a date").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Pick a date"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(DatePickerElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(DatePickerElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DatePickerElement::try_from(Value::Null).is_err());
    assert!(DatePickerElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(DatePickerElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn date_time_picker_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DateTimePickerElement::builder()
        .action_id("datetime_picker")
        .initial_date_time(1628633830_i64)
        .build()?;
    let expected = wire(&original);
    assert_eq!(DateTimePickerElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<DateTimePickerElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("datetime_picker")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("datetime_picker"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(DateTimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_date_time()),
        expected
            .get("initial_date_time")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_date_time(1628633830_i64)
            .build()?;
        assert_eq!(
            wire(edited)["initial_date_time"],
            serde_json::json!(1628633830)
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_date_time()
            .build()?;
        assert!(wire(edited).get("initial_date_time").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_date_time"] = Value::from("wrong");
    assert!(DateTimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(DateTimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(DateTimePickerElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(DateTimePickerElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DateTimePickerElement::try_from(Value::Null).is_err());
    assert!(DateTimePickerElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(DateTimePickerElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn email_input_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = EmailInputElement::builder()
        .action_id("email_input")
        .placeholder(PlainText::builder().text("Enter your email").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(EmailInputElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<EmailInputElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("email_input")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("email_input"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(EmailInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_value()),
        expected
            .get("initial_value")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_value("sample")
            .build()?;
        assert_eq!(wire(edited)["initial_value"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_value()
            .build()?;
        assert!(wire(edited).get("initial_value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_value"] = Value::from(false);
    assert!(EmailInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.dispatch_action_config()),
        expected
            .get("dispatch_action_config")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .dispatch_action_config(
                DispatchActionConfiguration::builder()
                    .trigger_actions_on(vec![String::from("on_character_entered")])
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["dispatch_action_config"],
            json!({"trigger_actions_on": ["on_character_entered"]})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_dispatch_action_config()
            .build()?;
        assert!(wire(edited).get("dispatch_action_config").is_none());
    }
    let mut invalid = expected.clone();
    invalid["dispatch_action_config"] = json!([]);
    assert!(EmailInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(EmailInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Enter your email").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Enter your email"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(EmailInputElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(EmailInputElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(EmailInputElement::try_from(Value::Null).is_err());
    assert!(EmailInputElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(EmailInputElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn external_multi_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ExternalMultiSelectElement::builder()
        .action_id("multi_external_select")
        .min_query_length(3_i64)
        .placeholder(PlainText::builder().text("Select items").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(
        ExternalMultiSelectElement::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<ExternalMultiSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("multi_external_select")
            .build()?;
        assert_eq!(
            wire(edited)["action_id"],
            Value::from("multi_external_select")
        );
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(ExternalMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.min_query_length()),
        expected
            .get("min_query_length")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .min_query_length(3_i64)
            .build()?;
        assert_eq!(wire(edited)["min_query_length"], serde_json::json!(3));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_min_query_length()
            .build()?;
        assert!(wire(edited).get("min_query_length").is_none());
    }
    let mut invalid = expected.clone();
    invalid["min_query_length"] = Value::from("wrong");
    assert!(ExternalMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_options()),
        expected
            .get("initial_options")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_options(vec![
                SelectOption::builder()
                    .text(MarkdownText::builder().text("*a*").build()?)
                    .value("a")
                    .description(PlainText::builder().text("*a*").build()?)
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["initial_options"],
            json!([
                {
                    "text": {
                        "type": "mrkdwn",
                        "text": "*a*"
                    },
                    "value": "a",
                    "description": {
                        "type": "plain_text",
                        "text": "*a*"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .initial_option(
            SelectOption::builder()
                .text(MarkdownText::builder().text("*a*").build()?)
                .value("a")
                .description(PlainText::builder().text("*a*").build()?)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["initial_options"].as_array().unwrap().len(),
            expected["initial_options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["initial_options"].as_array().unwrap().last().unwrap(),
            &json!({
                "text": {
                    "type": "mrkdwn",
                    "text": "*a*"
                },
                "value": "a",
                "description": {
                    "type": "plain_text",
                    "text": "*a*"
                }
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_options()
            .build()?;
        assert!(wire(edited).get("initial_options").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_options"] = json!({});
    assert!(ExternalMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ExternalMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_selected_items()),
        expected
            .get("max_selected_items")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .max_selected_items(1_i64)
            .build()?;
        assert_eq!(wire(edited)["max_selected_items"], serde_json::json!(1));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_max_selected_items()
            .build()?;
        assert!(wire(edited).get("max_selected_items").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_selected_items"] = Value::from("wrong");
    assert!(ExternalMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(ExternalMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select items").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select items"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(ExternalMultiSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(
        ExternalMultiSelectElement::try_from(wire(&extended))?,
        extended
    );
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ExternalMultiSelectElement::try_from(Value::Null).is_err());
    assert!(ExternalMultiSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ExternalMultiSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn external_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ExternalSelectElement::builder()
        .action_id("external_select")
        .min_query_length(4_i64)
        .placeholder(PlainText::builder().text("Select one item").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(ExternalSelectElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ExternalSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("external_select")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("external_select"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(ExternalSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.min_query_length()),
        expected
            .get("min_query_length")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .min_query_length(4_i64)
            .build()?;
        assert_eq!(wire(edited)["min_query_length"], serde_json::json!(4));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_min_query_length()
            .build()?;
        assert!(wire(edited).get("min_query_length").is_none());
    }
    let mut invalid = expected.clone();
    invalid["min_query_length"] = Value::from("wrong");
    assert!(ExternalSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_option()),
        expected
            .get("initial_option")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_option(
                SelectOption::builder()
                    .text(MarkdownText::builder().text("*a*").build()?)
                    .value("a")
                    .description(PlainText::builder().text("*a*").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["initial_option"],
            json!({
                "text": {
                    "type": "mrkdwn",
                    "text": "*a*"
                },
                "value": "a",
                "description": {
                    "type": "plain_text",
                    "text": "*a*"
                }
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_option()
            .build()?;
        assert!(wire(edited).get("initial_option").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_option"] = json!([]);
    assert!(ExternalSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ExternalSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(ExternalSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select one item").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select one item"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(ExternalSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ExternalSelectElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ExternalSelectElement::try_from(Value::Null).is_err());
    assert!(ExternalSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ExternalSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn feedback_buttons_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = FeedbackButtonsElement::builder()
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
    let expected = wire(&original);
    assert_eq!(
        FeedbackButtonsElement::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<FeedbackButtonsElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.positive_button()),
        expected["positive_button"]
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .positive_button(
                FeedbackButton::builder()
                    .text(PlainText::builder().text("Good").build()?)
                    .value("positive_feedback")
                    .accessibility_label("Mark this response as good")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["positive_button"],
            json!({
                "text": {
                    "type": "plain_text",
                    "text": "Good"
                },
                "value": "positive_feedback",
                "accessibility_label": "Mark this response as good"
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["positive_button"] = json!([]);
    assert!(FeedbackButtonsElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("positive_button");
    assert!(FeedbackButtonsElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.negative_button()),
        expected["negative_button"]
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .negative_button(
                FeedbackButton::builder()
                    .text(PlainText::builder().text("Bad").build()?)
                    .value("negative_feedback")
                    .accessibility_label("Mark this response as bad")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["negative_button"],
            json!({
                "text": {
                    "type": "plain_text",
                    "text": "Bad"
                },
                "value": "negative_feedback",
                "accessibility_label": "Mark this response as bad"
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["negative_button"] = json!([]);
    assert!(FeedbackButtonsElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("negative_button");
    assert!(FeedbackButtonsElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("feedback_buttons_1")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("feedback_buttons_1"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(FeedbackButtonsElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(FeedbackButtonsElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(FeedbackButtonsElement::try_from(Value::Null).is_err());
    assert!(FeedbackButtonsElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(FeedbackButtonsElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn file_input_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = FileInputElement::builder()
        .action_id("file_input_action_id_1")
        .filetypes(vec![String::from("jpg"), String::from("png")])
        .max_files(5_i64)
        .build()?;
    let expected = wire(&original);
    assert_eq!(FileInputElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<FileInputElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("file_input_action_id_1")
            .build()?;
        assert_eq!(
            wire(edited)["action_id"],
            Value::from("file_input_action_id_1")
        );
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(FileInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.filetypes()),
        expected.get("filetypes").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .filetypes(vec![String::from("jpg"), String::from("png")])
            .build()?;
        assert_eq!(wire(edited)["filetypes"], json!(["jpg", "png"]));
    }
    let appended = original.clone().into_builder().filetype("jpg").build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["filetypes"].as_array().unwrap().len(),
            expected["filetypes"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["filetypes"].as_array().unwrap().last().unwrap(),
            &Value::from("jpg")
        );
    }
    {
        let edited = original.clone().into_builder().clear_filetypes().build()?;
        assert!(wire(edited).get("filetypes").is_none());
    }
    let mut invalid = expected.clone();
    invalid["filetypes"] = json!({});
    assert!(FileInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_files()),
        expected.get("max_files").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().max_files(5_i64).build()?;
        assert_eq!(wire(edited)["max_files"], serde_json::json!(5));
    }
    {
        let edited = original.clone().into_builder().clear_max_files().build()?;
        assert!(wire(edited).get("max_files").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_files"] = Value::from("wrong");
    assert!(FileInputElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(FileInputElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(FileInputElement::try_from(Value::Null).is_err());
    assert!(FileInputElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(FileInputElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn icon_button_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = IconButtonElement::builder()
        .icon(IconButtonIcon::Trash)
        .text(PlainText::builder().text("Delete").build()?)
        .action_id("delete_button")
        .value("delete_item")
        .build()?;
    let expected = wire(&original);
    assert_eq!(IconButtonElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<IconButtonElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(PlainText::builder().text("Delete").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "plain_text", "text": "Delete"})
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(IconButtonElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(IconButtonElement::try_from(missing).is_err());
    assert_eq!(wire(original.icon()), expected["icon"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .icon(IconButtonIcon::Trash)
            .build()?;
        assert_eq!(wire(edited)["icon"], Value::from("trash"));
    }
    let mut invalid = expected.clone();
    invalid["icon"] = Value::from("not-a-variant");
    assert!(IconButtonElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("icon");
    assert!(IconButtonElement::try_from(missing).is_ok());
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("delete_button")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("delete_button"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(IconButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.value()),
        expected.get("value").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .value("delete_item")
            .build()?;
        assert_eq!(wire(edited)["value"], Value::from("delete_item"));
    }
    {
        let edited = original.clone().into_builder().clear_value().build()?;
        assert!(wire(edited).get("value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from(false);
    assert!(IconButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(IconButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.accessibility_label()),
        expected
            .get("accessibility_label")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .accessibility_label("sample")
            .build()?;
        assert_eq!(wire(edited)["accessibility_label"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_accessibility_label()
            .build()?;
        assert!(wire(edited).get("accessibility_label").is_none());
    }
    let mut invalid = expected.clone();
    invalid["accessibility_label"] = Value::from(false);
    assert!(IconButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.visible_to_user_ids()),
        expected
            .get("visible_to_user_ids")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .visible_to_user_ids(vec![String::from("sample")])
            .build()?;
        assert_eq!(wire(edited)["visible_to_user_ids"], json!(["sample"]));
    }
    let appended = original
        .clone()
        .into_builder()
        .visible_to_user_id("sample")
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["visible_to_user_ids"]
                .as_array()
                .unwrap()
                .last()
                .unwrap(),
            &Value::from("sample")
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_visible_to_user_ids()
            .build()?;
        assert!(wire(edited).get("visible_to_user_ids").is_none());
    }
    let mut invalid = expected.clone();
    invalid["visible_to_user_ids"] = json!({});
    assert!(IconButtonElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(IconButtonElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(IconButtonElement::try_from(Value::Null).is_err());
    assert!(IconButtonElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(IconButtonElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn image_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ImageElement::builder()
        .image_url("https://picsum.photos/400/300")
        .alt_text("Sample hero image")
        .build()?;
    let expected = wire(&original);
    assert_eq!(ImageElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ImageElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.alt_text()), expected["alt_text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .alt_text("Sample hero image")
            .build()?;
        assert_eq!(wire(edited)["alt_text"], Value::from("Sample hero image"));
    }
    let mut invalid = expected.clone();
    invalid["alt_text"] = Value::from(false);
    assert!(ImageElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("alt_text");
    assert!(ImageElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.image_url()),
        expected.get("image_url").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .image_url("https://picsum.photos/400/300")
            .build()?;
        assert_eq!(
            wire(edited)["image_url"],
            Value::from("https://picsum.photos/400/300")
        );
    }
    {
        let error = (original.clone().into_builder().clear_image_url().build())
            .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MissingRequired);
        assert_eq!(error.path(), "ImageElement");
    }
    let mut invalid = expected.clone();
    invalid["image_url"] = Value::from(false);
    assert!(ImageElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.slack_file()),
        expected.get("slack_file").cloned().unwrap_or(Value::Null)
    );
    {
        let error = (original
            .clone()
            .into_builder()
            .slack_file(SlackFile::builder().id("F0123ABC456").build()?)
            .build())
        .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "ImageElement");
    }
    {
        let edited = original.clone().into_builder().clear_slack_file().build()?;
        assert!(wire(edited).get("slack_file").is_none());
    }
    let mut invalid = expected.clone();
    invalid["slack_file"] = json!([]);
    assert!(ImageElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ImageElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ImageElement::try_from(Value::Null).is_err());
    assert!(ImageElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ImageElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn number_input_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = NumberInputElement::builder()
        .is_decimal_allowed(false)
        .action_id("number_input")
        .build()?;
    let expected = wire(&original);
    assert_eq!(NumberInputElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<NumberInputElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("number_input")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("number_input"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(NumberInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.is_decimal_allowed()),
        expected["is_decimal_allowed"]
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .is_decimal_allowed(false)
            .build()?;
        assert_eq!(wire(edited)["is_decimal_allowed"], Value::from(false));
    }
    let mut invalid = expected.clone();
    invalid["is_decimal_allowed"] = serde_json::json!(0);
    assert!(NumberInputElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing
        .as_object_mut()
        .unwrap()
        .remove("is_decimal_allowed");
    assert!(NumberInputElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.initial_value()),
        expected
            .get("initial_value")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_value("sample")
            .build()?;
        assert_eq!(wire(edited)["initial_value"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_value()
            .build()?;
        assert!(wire(edited).get("initial_value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_value"] = Value::from(false);
    assert!(NumberInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.min_value()),
        expected.get("min_value").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().min_value(1.0_f64).build()?;
        assert_eq!(wire(edited)["min_value"], serde_json::json!(1.0));
    }
    {
        let edited = original.clone().into_builder().clear_min_value().build()?;
        assert!(wire(edited).get("min_value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["min_value"] = Value::from("wrong");
    assert!(NumberInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_value()),
        expected.get("max_value").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().max_value(1.0_f64).build()?;
        assert_eq!(wire(edited)["max_value"], serde_json::json!(1.0));
    }
    {
        let edited = original.clone().into_builder().clear_max_value().build()?;
        assert!(wire(edited).get("max_value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_value"] = Value::from("wrong");
    assert!(NumberInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.dispatch_action_config()),
        expected
            .get("dispatch_action_config")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .dispatch_action_config(
                DispatchActionConfiguration::builder()
                    .trigger_actions_on(vec![String::from("on_character_entered")])
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["dispatch_action_config"],
            json!({"trigger_actions_on": ["on_character_entered"]})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_dispatch_action_config()
            .build()?;
        assert!(wire(edited).get("dispatch_action_config").is_none());
    }
    let mut invalid = expected.clone();
    invalid["dispatch_action_config"] = json!([]);
    assert!(NumberInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(NumberInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("*a*").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "*a*"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(NumberInputElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(NumberInputElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(NumberInputElement::try_from(Value::Null).is_err());
    assert!(NumberInputElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(NumberInputElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn overflow_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = OverflowElement::builder()
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
    let expected = wire(&original);
    assert_eq!(OverflowElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<OverflowElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("overflow")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("overflow"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(OverflowElement::try_from(invalid).is_err());
    assert_eq!(wire(original.options()), expected["options"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["options"],
            json!([
                {
                    "text": {
                        "type": "plain_text",
                        "text": "A"
                    },
                    "value": "A"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "B"
                    },
                    "value": "B"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "C"
                    },
                    "value": "C"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["options"].as_array().unwrap().len(),
            expected["options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["options"].as_array().unwrap().last().unwrap(),
            &json!({"text": {"type": "plain_text", "text": "A"}, "value": "A"})
        );
    }
    let mut invalid = expected.clone();
    invalid["options"] = json!({});
    assert!(OverflowElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("options");
    assert!(OverflowElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(OverflowElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(OverflowElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(OverflowElement::try_from(Value::Null).is_err());
    assert!(OverflowElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(OverflowElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn plain_text_input_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = PlainTextInputElement::builder()
        .action_id("plaintext_input")
        .placeholder(PlainText::builder().text("Enter your plain text").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(PlainTextInputElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<PlainTextInputElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("action")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("action"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_value()),
        expected
            .get("initial_value")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_value("sample")
            .build()?;
        assert_eq!(wire(edited)["initial_value"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_value()
            .build()?;
        assert!(wire(edited).get("initial_value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_value"] = Value::from(false);
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.multiline()),
        expected.get("multiline").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().multiline(true).build()?;
        assert_eq!(wire(edited)["multiline"], Value::from(true));
    }
    {
        let edited = original.clone().into_builder().clear_multiline().build()?;
        assert!(wire(edited).get("multiline").is_none());
    }
    let mut invalid = expected.clone();
    invalid["multiline"] = serde_json::json!(0);
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.min_length()),
        expected.get("min_length").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().min_length(1_i64).build()?;
        assert_eq!(wire(edited)["min_length"], serde_json::json!(1));
    }
    {
        let edited = original.clone().into_builder().clear_min_length().build()?;
        assert!(wire(edited).get("min_length").is_none());
    }
    let mut invalid = expected.clone();
    invalid["min_length"] = Value::from("wrong");
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_length()),
        expected.get("max_length").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().max_length(1_i64).build()?;
        assert_eq!(wire(edited)["max_length"], serde_json::json!(1));
    }
    {
        let edited = original.clone().into_builder().clear_max_length().build()?;
        assert!(wire(edited).get("max_length").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_length"] = Value::from("wrong");
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.dispatch_action_config()),
        expected
            .get("dispatch_action_config")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .dispatch_action_config(
                DispatchActionConfiguration::builder()
                    .trigger_actions_on(vec![String::from("on_character_entered")])
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["dispatch_action_config"],
            json!({"trigger_actions_on": ["on_character_entered"]})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_dispatch_action_config()
            .build()?;
        assert!(wire(edited).get("dispatch_action_config").is_none());
    }
    let mut invalid = expected.clone();
    invalid["dispatch_action_config"] = json!([]);
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Enter your plain text").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Enter your plain text"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(PlainTextInputElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(PlainTextInputElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(PlainTextInputElement::try_from(Value::Null).is_err());
    assert!(PlainTextInputElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(PlainTextInputElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn radio_buttons_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RadioButtonsElement::builder()
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
    let expected = wire(&original);
    assert_eq!(RadioButtonsElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RadioButtonsElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("radio_buttons")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("radio_buttons"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(RadioButtonsElement::try_from(invalid).is_err());
    assert_eq!(wire(original.options()), expected["options"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["options"],
            json!([
                {
                    "text": {
                        "type": "plain_text",
                        "text": "A"
                    },
                    "value": "A"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "B"
                    },
                    "value": "B"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "C"
                    },
                    "value": "C"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["options"].as_array().unwrap().len(),
            expected["options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["options"].as_array().unwrap().last().unwrap(),
            &json!({"text": {"type": "plain_text", "text": "A"}, "value": "A"})
        );
    }
    let mut invalid = expected.clone();
    invalid["options"] = json!({});
    assert!(RadioButtonsElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("options");
    assert!(RadioButtonsElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.initial_option()),
        expected
            .get("initial_option")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_option(
                SelectOption::builder()
                    .text(PlainText::builder().text("A").build()?)
                    .value("A")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["initial_option"],
            json!({"text": {"type": "plain_text", "text": "A"}, "value": "A"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_option()
            .build()?;
        assert!(wire(edited).get("initial_option").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_option"] = json!([]);
    assert!(RadioButtonsElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(RadioButtonsElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(RadioButtonsElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RadioButtonsElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RadioButtonsElement::try_from(Value::Null).is_err());
    assert!(RadioButtonsElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RadioButtonsElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_input_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextInputElement::builder()
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
    let expected = wire(&original);
    assert_eq!(RichTextInputElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextInputElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.action_id()), expected["action_id"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("action_id")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("action_id"));
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(RichTextInputElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("action_id");
    assert!(RichTextInputElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.initial_value()),
        expected
            .get("initial_value")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["initial_value"],
            json!({
                "type": "rich_text",
                "elements": [
                    {
                        "type": "rich_text_section",
                        "elements": [
                            {
                                "type": "text",
                                "text": "I'm rich"
                            }
                        ]
                    }
                ]
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_value()
            .build()?;
        assert!(wire(edited).get("initial_value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_value"] = json!([]);
    assert!(RichTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.dispatch_action_config()),
        expected
            .get("dispatch_action_config")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .dispatch_action_config(
                DispatchActionConfiguration::builder()
                    .trigger_actions_on(vec![String::from("on_character_entered")])
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["dispatch_action_config"],
            json!({"trigger_actions_on": ["on_character_entered"]})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_dispatch_action_config()
            .build()?;
        assert!(wire(edited).get("dispatch_action_config").is_none());
    }
    let mut invalid = expected.clone();
    invalid["dispatch_action_config"] = json!([]);
    assert!(RichTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(false)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(RichTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Hello").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Hello"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(RichTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.min_lines()),
        expected.get("min_lines").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().min_lines(1_i64).build()?;
        assert_eq!(wire(edited)["min_lines"], serde_json::json!(1));
    }
    {
        let edited = original.clone().into_builder().clear_min_lines().build()?;
        assert!(wire(edited).get("min_lines").is_none());
    }
    let mut invalid = expected.clone();
    invalid["min_lines"] = Value::from("wrong");
    assert!(RichTextInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_lines()),
        expected.get("max_lines").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().max_lines(1_i64).build()?;
        assert_eq!(wire(edited)["max_lines"], serde_json::json!(1));
    }
    {
        let edited = original.clone().into_builder().clear_max_lines().build()?;
        assert!(wire(edited).get("max_lines").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_lines"] = Value::from("wrong");
    assert!(RichTextInputElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextInputElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextInputElement::try_from(Value::Null).is_err());
    assert!(RichTextInputElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextInputElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn static_multi_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = StaticMultiSelectElement::builder()
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
    let expected = wire(&original);
    assert_eq!(
        StaticMultiSelectElement::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<StaticMultiSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("multi_static_select")
            .build()?;
        assert_eq!(
            wire(edited)["action_id"],
            Value::from("multi_static_select")
        );
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.options()),
        expected.get("options").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["options"],
            json!([
                {
                    "text": {
                        "type": "plain_text",
                        "text": "A"
                    },
                    "value": "A"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "B"
                    },
                    "value": "B"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["options"].as_array().unwrap().len(),
            expected["options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["options"].as_array().unwrap().last().unwrap(),
            &json!({"text": {"type": "plain_text", "text": "A"}, "value": "A"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_options().build()?;
        assert!(wire(edited).get("options").is_none());
    }
    let mut invalid = expected.clone();
    invalid["options"] = json!({});
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.option_groups()),
        expected
            .get("option_groups")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let error = (original
            .clone()
            .into_builder()
            .option_groups(vec![
                SelectOptionGroup::builder()
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
                    .build()?,
            ])
            .build())
        .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "StaticMultiSelectElement");
    }
    let appended = original
        .clone()
        .into_builder()
        .option_group(
            SelectOptionGroup::builder()
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
                .build()?,
        )
        .build();
    {
        let error =
            (appended).expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "StaticMultiSelectElement");
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_option_groups()
            .build()?;
        assert!(wire(edited).get("option_groups").is_none());
    }
    let mut invalid = expected.clone();
    invalid["option_groups"] = json!({});
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_options()),
        expected
            .get("initial_options")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_options(vec![
                SelectOption::builder()
                    .text(MarkdownText::builder().text("*a*").build()?)
                    .value("a")
                    .description(PlainText::builder().text("*a*").build()?)
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["initial_options"],
            json!([
                {
                    "text": {
                        "type": "mrkdwn",
                        "text": "*a*"
                    },
                    "value": "a",
                    "description": {
                        "type": "plain_text",
                        "text": "*a*"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .initial_option(
            SelectOption::builder()
                .text(MarkdownText::builder().text("*a*").build()?)
                .value("a")
                .description(PlainText::builder().text("*a*").build()?)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["initial_options"].as_array().unwrap().len(),
            expected["initial_options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["initial_options"].as_array().unwrap().last().unwrap(),
            &json!({
                "text": {
                    "type": "mrkdwn",
                    "text": "*a*"
                },
                "value": "a",
                "description": {
                    "type": "plain_text",
                    "text": "*a*"
                }
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_options()
            .build()?;
        assert!(wire(edited).get("initial_options").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_options"] = json!({});
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_selected_items()),
        expected
            .get("max_selected_items")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .max_selected_items(1_i64)
            .build()?;
        assert_eq!(wire(edited)["max_selected_items"], serde_json::json!(1));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_max_selected_items()
            .build()?;
        assert!(wire(edited).get("max_selected_items").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_selected_items"] = Value::from("wrong");
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select one or more").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select one or more"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(StaticMultiSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(
        StaticMultiSelectElement::try_from(wire(&extended))?,
        extended
    );
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(StaticMultiSelectElement::try_from(Value::Null).is_err());
    assert!(StaticMultiSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(StaticMultiSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn static_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = StaticSelectElement::builder()
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
    let expected = wire(&original);
    assert_eq!(StaticSelectElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<StaticSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("static_select")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("static_select"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(StaticSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.options()),
        expected.get("options").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["options"],
            json!([
                {
                    "text": {
                        "type": "plain_text",
                        "text": "A"
                    },
                    "value": "A"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "B"
                    },
                    "value": "B"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "C"
                    },
                    "value": "C"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["options"].as_array().unwrap().len(),
            expected["options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["options"].as_array().unwrap().last().unwrap(),
            &json!({"text": {"type": "plain_text", "text": "A"}, "value": "A"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_options().build()?;
        assert!(wire(edited).get("options").is_none());
    }
    let mut invalid = expected.clone();
    invalid["options"] = json!({});
    assert!(StaticSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.option_groups()),
        expected
            .get("option_groups")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let error = (original
            .clone()
            .into_builder()
            .option_groups(vec![
                SelectOptionGroup::builder()
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
                    .build()?,
            ])
            .build())
        .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "StaticSelectElement");
    }
    let appended = original
        .clone()
        .into_builder()
        .option_group(
            SelectOptionGroup::builder()
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
                .build()?,
        )
        .build();
    {
        let error =
            (appended).expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "StaticSelectElement");
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_option_groups()
            .build()?;
        assert!(wire(edited).get("option_groups").is_none());
    }
    let mut invalid = expected.clone();
    invalid["option_groups"] = json!({});
    assert!(StaticSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_option()),
        expected
            .get("initial_option")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_option(
                SelectOption::builder()
                    .text(MarkdownText::builder().text("*a*").build()?)
                    .value("a")
                    .description(PlainText::builder().text("*a*").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["initial_option"],
            json!({
                "text": {
                    "type": "mrkdwn",
                    "text": "*a*"
                },
                "value": "a",
                "description": {
                    "type": "plain_text",
                    "text": "*a*"
                }
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_option()
            .build()?;
        assert!(wire(edited).get("initial_option").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_option"] = json!([]);
    assert!(StaticSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(StaticSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(StaticSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select one item").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select one item"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(StaticSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(StaticSelectElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(StaticSelectElement::try_from(Value::Null).is_err());
    assert!(StaticSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(StaticSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn time_picker_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = TimePickerElement::builder()
        .action_id("timepicker")
        .initial_time("12:00")
        .placeholder(PlainText::builder().text("Select your time").build()?)
        .timezone("Australia/Sydney")
        .build()?;
    let expected = wire(&original);
    assert_eq!(TimePickerElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<TimePickerElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("timepicker")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("timepicker"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(TimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_time()),
        expected.get("initial_time").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_time("12:00")
            .build()?;
        assert_eq!(wire(edited)["initial_time"], Value::from("12:00"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_time()
            .build()?;
        assert!(wire(edited).get("initial_time").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_time"] = Value::from(false);
    assert!(TimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.timezone()),
        expected.get("timezone").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .timezone("Australia/Sydney")
            .build()?;
        assert_eq!(wire(edited)["timezone"], Value::from("Australia/Sydney"));
    }
    {
        let edited = original.clone().into_builder().clear_timezone().build()?;
        assert!(wire(edited).get("timezone").is_none());
    }
    let mut invalid = expected.clone();
    invalid["timezone"] = Value::from(false);
    assert!(TimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(TimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(TimePickerElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select your time").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select your time"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(TimePickerElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(TimePickerElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(TimePickerElement::try_from(Value::Null).is_err());
    assert!(TimePickerElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(TimePickerElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn url_input_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = UrlInputElement::builder()
        .action_id("url_text_input")
        .build()?;
    let expected = wire(&original);
    assert_eq!(UrlInputElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<UrlInputElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("url_text_input")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("url_text_input"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(UrlInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_value()),
        expected
            .get("initial_value")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_value("sample")
            .build()?;
        assert_eq!(wire(edited)["initial_value"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_value()
            .build()?;
        assert!(wire(edited).get("initial_value").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_value"] = Value::from(false);
    assert!(UrlInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.dispatch_action_config()),
        expected
            .get("dispatch_action_config")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .dispatch_action_config(
                DispatchActionConfiguration::builder()
                    .trigger_actions_on(vec![String::from("on_character_entered")])
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["dispatch_action_config"],
            json!({"trigger_actions_on": ["on_character_entered"]})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_dispatch_action_config()
            .build()?;
        assert!(wire(edited).get("dispatch_action_config").is_none());
    }
    let mut invalid = expected.clone();
    invalid["dispatch_action_config"] = json!([]);
    assert!(UrlInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(UrlInputElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("*a*").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "*a*"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(UrlInputElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(UrlInputElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(UrlInputElement::try_from(Value::Null).is_err());
    assert!(UrlInputElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(UrlInputElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn user_multi_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = UserMultiSelectElement::builder()
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
    let expected = wire(&original);
    assert_eq!(
        UserMultiSelectElement::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<UserMultiSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("multi_users_select")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("multi_users_select"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(UserMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_users()),
        expected
            .get("initial_users")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_users(vec![
                String::from("U064B5H1309"),
                String::from("U063JR973UP"),
            ])
            .build()?;
        assert_eq!(
            wire(edited)["initial_users"],
            json!(["U064B5H1309", "U063JR973UP"])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .initial_user("U064B5H1309")
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["initial_users"].as_array().unwrap().len(),
            expected["initial_users"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["initial_users"].as_array().unwrap().last().unwrap(),
            &Value::from("U064B5H1309")
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_users()
            .build()?;
        assert!(wire(edited).get("initial_users").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_users"] = json!({});
    assert!(UserMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(UserMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.max_selected_items()),
        expected
            .get("max_selected_items")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .max_selected_items(1_i64)
            .build()?;
        assert_eq!(wire(edited)["max_selected_items"], serde_json::json!(1));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_max_selected_items()
            .build()?;
        assert!(wire(edited).get("max_selected_items").is_none());
    }
    let mut invalid = expected.clone();
    invalid["max_selected_items"] = Value::from("wrong");
    assert!(UserMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(UserMultiSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(
                PlainText::builder()
                    .text("Select one or more users")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select one or more users"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(UserMultiSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(UserMultiSelectElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(UserMultiSelectElement::try_from(Value::Null).is_err());
    assert!(UserMultiSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(UserMultiSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn user_select_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = UserSelectElement::builder()
        .action_id("users_select")
        .placeholder(PlainText::builder().text("Select one user").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(UserSelectElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<UserSelectElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.action_id()),
        expected.get("action_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("users_select")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("users_select"));
    }
    {
        let edited = original.clone().into_builder().clear_action_id().build()?;
        assert!(wire(edited).get("action_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(UserSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.initial_user()),
        expected.get("initial_user").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .initial_user("sample")
            .build()?;
        assert_eq!(wire(edited)["initial_user"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_initial_user()
            .build()?;
        assert!(wire(edited).get("initial_user").is_none());
    }
    let mut invalid = expected.clone();
    invalid["initial_user"] = Value::from(false);
    assert!(UserSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(UserSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.focus_on_load()),
        expected
            .get("focus_on_load")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .focus_on_load(true)
            .build()?;
        assert_eq!(wire(edited)["focus_on_load"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_focus_on_load()
            .build()?;
        assert!(wire(edited).get("focus_on_load").is_none());
    }
    let mut invalid = expected.clone();
    invalid["focus_on_load"] = serde_json::json!(0);
    assert!(UserSelectElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.placeholder()),
        expected.get("placeholder").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .placeholder(PlainText::builder().text("Select one user").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["placeholder"],
            json!({"type": "plain_text", "text": "Select one user"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_placeholder()
            .build()?;
        assert!(wire(edited).get("placeholder").is_none());
    }
    let mut invalid = expected.clone();
    invalid["placeholder"] = json!([]);
    assert!(UserSelectElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(UserSelectElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(UserSelectElement::try_from(Value::Null).is_err());
    assert!(UserSelectElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(UserSelectElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn workflow_button_element_editing_and_ingress() -> Result<(), ValidationError> {
    let original = WorkflowButtonElement::builder().action_id("run_workflow").text(PlainText::builder().text("Run Your Workflow").build()?).workflow(Workflow::builder().trigger(Trigger::builder().url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c").customizable_input_parameters(vec![InputParameter::builder().name("name_a").value("value_a").build()?,InputParameter::builder().name("name_b").value("value_b").build()?]).build()?).build()?).build()?;
    let expected = wire(&original);
    assert_eq!(WorkflowButtonElement::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<WorkflowButtonElement>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(PlainText::builder().text("Run Your Workflow").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "plain_text", "text": "Run Your Workflow"})
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(WorkflowButtonElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(WorkflowButtonElement::try_from(missing).is_err());
    assert_eq!(wire(original.workflow()), expected["workflow"]);
    {
        let edited = original.clone().into_builder().workflow(Workflow::builder().trigger(Trigger::builder().url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c").customizable_input_parameters(vec![InputParameter::builder().name("name_a").value("value_a").build()?,InputParameter::builder().name("name_b").value("value_b").build()?]).build()?).build()?).build()?;
        assert_eq!(
            wire(edited)["workflow"],
            json!({
                "trigger": {
                    "url": "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
                    "customizable_input_parameters": [
                        {
                            "name": "name_a",
                            "value": "value_a"
                        },
                        {
                            "name": "name_b",
                            "value": "value_b"
                        }
                    ]
                }
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["workflow"] = json!([]);
    assert!(WorkflowButtonElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("workflow");
    assert!(WorkflowButtonElement::try_from(missing).is_err());
    assert_eq!(wire(original.action_id()), expected["action_id"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .action_id("run_workflow")
            .build()?;
        assert_eq!(wire(edited)["action_id"], Value::from("run_workflow"));
    }
    let mut invalid = expected.clone();
    invalid["action_id"] = Value::from(false);
    assert!(WorkflowButtonElement::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("action_id");
    assert!(WorkflowButtonElement::try_from(missing).is_err());
    assert_eq!(
        wire(original.confirm()),
        expected.get("confirm").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(
                ConfirmationDialogue::builder()
                    .title(PlainText::builder().text("Maybe?").build()?)
                    .text(
                        PlainText::builder()
                            .text("Would you like to play checkers?")
                            .build()?,
                    )
                    .confirm(PlainText::builder().text("Yes").build()?)
                    .deny(PlainText::builder().text("Nope!").build()?)
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({
                "title": {
                    "type": "plain_text",
                    "text": "Maybe?"
                },
                "text": {
                    "type": "plain_text",
                    "text": "Would you like to play checkers?"
                },
                "confirm": {
                    "type": "plain_text",
                    "text": "Yes"
                },
                "deny": {
                    "type": "plain_text",
                    "text": "Nope!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_confirm().build()?;
        assert!(wire(edited).get("confirm").is_none());
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(WorkflowButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .style(ButtonStyle::Primary)
            .build()?;
        assert_eq!(wire(edited)["style"], Value::from("primary"));
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = Value::from("not-a-variant");
    assert!(WorkflowButtonElement::try_from(invalid).is_err());
    assert_eq!(
        wire(original.accessibility_label()),
        expected
            .get("accessibility_label")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .accessibility_label("sample")
            .build()?;
        assert_eq!(wire(edited)["accessibility_label"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_accessibility_label()
            .build()?;
        assert!(wire(edited).get("accessibility_label").is_none());
    }
    let mut invalid = expected.clone();
    invalid["accessibility_label"] = Value::from(false);
    assert!(WorkflowButtonElement::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(WorkflowButtonElement::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(WorkflowButtonElement::try_from(Value::Null).is_err());
    assert!(WorkflowButtonElement::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(WorkflowButtonElement::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn area_chart_editing_and_ingress() -> Result<(), ValidationError> {
    let original = AreaChart::builder()
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
    let expected = wire(&original);
    assert_eq!(AreaChart::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<AreaChart>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.series()), expected["series"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .series(vec![
                DataSeries::builder()
                    .name("Free Tier")
                    .data(vec![
                        DataPoint::builder().label("Mon").value(12000_i64).build()?,
                        DataPoint::builder().label("Tue").value(13500_i64).build()?,
                    ])
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["series"],
            Value::Array(vec![Value::Object(
                [
                    ("name".into(), Value::from("Free Tier")),
                    (
                        "data".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("label".into(), Value::from("Mon")),
                                    ("value".into(), serde_json::json!(12000))
                                ]
                                .into_iter()
                                .collect()
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("Tue")),
                                    ("value".into(), serde_json::json!(13500))
                                ]
                                .into_iter()
                                .collect()
                            )
                        ])
                    )
                ]
                .into_iter()
                .collect()
            )])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .series_entry(
            DataSeries::builder()
                .name("Free Tier")
                .data(vec![
                    DataPoint::builder().label("Mon").value(12000_i64).build()?,
                    DataPoint::builder().label("Tue").value(13500_i64).build()?,
                ])
                .build()?,
        )
        .build();
    {
        let error =
            (appended).expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::InvalidUsage);
        assert_eq!(error.path(), "AreaChart.series");
    }
    let mut invalid = expected.clone();
    invalid["series"] = json!({});
    assert!(AreaChart::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("series");
    assert!(AreaChart::try_from(missing).is_err());
    assert_eq!(wire(original.axis_config()), expected["axis_config"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .axis_config(
                AxisConfig::builder()
                    .categories(vec![String::from("Mon"), String::from("Tue")])
                    .x_label("Day")
                    .y_label("Users")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["axis_config"],
            json!({"categories": ["Mon", "Tue"], "x_label": "Day", "y_label": "Users"})
        );
    }
    let mut invalid = expected.clone();
    invalid["axis_config"] = json!([]);
    assert!(AreaChart::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("axis_config");
    assert!(AreaChart::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(AreaChart::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(AreaChart::try_from(Value::Null).is_err());
    assert!(AreaChart::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(AreaChart::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn axis_config_editing_and_ingress() -> Result<(), ValidationError> {
    let original = AxisConfig::builder()
        .categories(vec![String::from("Mon"), String::from("Tue")])
        .x_label("Day")
        .y_label("Users")
        .build()?;
    let expected = wire(&original);
    assert_eq!(AxisConfig::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<AxisConfig>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.categories()), expected["categories"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .categories(vec![String::from("Mon"), String::from("Tue")])
            .build()?;
        assert_eq!(wire(edited)["categories"], json!(["Mon", "Tue"]));
    }
    let appended = original.clone().into_builder().category("Mon").build();
    {
        let error =
            (appended).expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::InvalidUsage);
        assert_eq!(error.path(), "AxisConfig.categories");
    }
    let mut invalid = expected.clone();
    invalid["categories"] = json!({});
    assert!(AxisConfig::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("categories");
    assert!(AxisConfig::try_from(missing).is_err());
    assert_eq!(
        wire(original.x_label()),
        expected.get("x_label").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().x_label("Day").build()?;
        assert_eq!(wire(edited)["x_label"], Value::from("Day"));
    }
    {
        let edited = original.clone().into_builder().clear_x_label().build()?;
        assert!(wire(edited).get("x_label").is_none());
    }
    let mut invalid = expected.clone();
    invalid["x_label"] = Value::from(false);
    assert!(AxisConfig::try_from(invalid).is_err());
    assert_eq!(
        wire(original.y_label()),
        expected.get("y_label").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().y_label("Users").build()?;
        assert_eq!(wire(edited)["y_label"], Value::from("Users"));
    }
    {
        let edited = original.clone().into_builder().clear_y_label().build()?;
        assert!(wire(edited).get("y_label").is_none());
    }
    let mut invalid = expected.clone();
    invalid["y_label"] = Value::from(false);
    assert!(AxisConfig::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(AxisConfig::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(AxisConfig::try_from(Value::Null).is_err());
    assert!(AxisConfig::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn bar_chart_editing_and_ingress() -> Result<(), ValidationError> {
    let original = BarChart::builder()
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
    let expected = wire(&original);
    assert_eq!(BarChart::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<BarChart>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.series()), expected["series"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["series"],
            Value::Array(vec![Value::Object(
                [
                    ("name".into(), Value::from("Pies")),
                    (
                        "data".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("label".into(), Value::from("Pumpkin")),
                                    ("value".into(), serde_json::json!(70))
                                ]
                                .into_iter()
                                .collect()
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("Blueberry")),
                                    ("value".into(), serde_json::json!(90))
                                ]
                                .into_iter()
                                .collect()
                            )
                        ])
                    )
                ]
                .into_iter()
                .collect()
            )])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .series_entry(
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
        )
        .build();
    {
        let error =
            (appended).expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::InvalidUsage);
        assert_eq!(error.path(), "BarChart.series");
    }
    let mut invalid = expected.clone();
    invalid["series"] = json!({});
    assert!(BarChart::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("series");
    assert!(BarChart::try_from(missing).is_err());
    assert_eq!(wire(original.axis_config()), expected["axis_config"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .axis_config(
                AxisConfig::builder()
                    .categories(vec![String::from("Pumpkin"), String::from("Blueberry")])
                    .x_label("Pies")
                    .y_label("Tastiness")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["axis_config"],
            json!({"categories": ["Pumpkin", "Blueberry"], "x_label": "Pies", "y_label": "Tastiness"})
        );
    }
    let mut invalid = expected.clone();
    invalid["axis_config"] = json!([]);
    assert!(BarChart::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("axis_config");
    assert!(BarChart::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(BarChart::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(BarChart::try_from(Value::Null).is_err());
    assert!(BarChart::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(BarChart::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn chart_segment_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ChartSegment::builder()
        .label("Kit Kat")
        .value(45_i64)
        .build()?;
    let expected = wire(&original);
    assert_eq!(ChartSegment::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ChartSegment>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.label()), expected["label"]);
    {
        let edited = original.clone().into_builder().label("Kit Kat").build()?;
        assert_eq!(wire(edited)["label"], Value::from("Kit Kat"));
    }
    let mut invalid = expected.clone();
    invalid["label"] = Value::from(false);
    assert!(ChartSegment::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("label");
    assert!(ChartSegment::try_from(missing).is_err());
    assert_eq!(wire(original.value()), expected["value"]);
    {
        let edited = original.clone().into_builder().value(45_i64).build()?;
        assert_eq!(wire(edited)["value"], serde_json::json!(45));
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from("wrong");
    assert!(ChartSegment::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("value");
    assert!(ChartSegment::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ChartSegment::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ChartSegment::try_from(Value::Null).is_err());
    assert!(ChartSegment::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn column_settings_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ColumnSettings::builder().is_wrapped(true).build()?;
    let expected = wire(&original);
    assert_eq!(ColumnSettings::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ColumnSettings>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.align()),
        expected.get("align").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .align(ColumnAlign::Right)
            .build()?;
        assert_eq!(wire(edited)["align"], Value::from("right"));
    }
    {
        let edited = original.clone().into_builder().clear_align().build()?;
        assert!(wire(edited).get("align").is_none());
    }
    let mut invalid = expected.clone();
    invalid["align"] = Value::from("not-a-variant");
    assert!(ColumnSettings::try_from(invalid).is_err());
    assert_eq!(
        wire(original.is_wrapped()),
        expected.get("is_wrapped").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().is_wrapped(true).build()?;
        assert_eq!(wire(edited)["is_wrapped"], Value::from(true));
    }
    {
        let edited = original.clone().into_builder().clear_is_wrapped().build()?;
        assert!(wire(edited).get("is_wrapped").is_none());
    }
    let mut invalid = expected.clone();
    invalid["is_wrapped"] = serde_json::json!(0);
    assert!(ColumnSettings::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ColumnSettings::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ColumnSettings::try_from(Value::Null).is_err());
    assert!(ColumnSettings::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn confirmation_dialogue_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ConfirmationDialogue::builder()
        .title(PlainText::builder().text("Maybe?").build()?)
        .text(
            PlainText::builder()
                .text("Would you like to play checkers?")
                .build()?,
        )
        .confirm(PlainText::builder().text("Yes").build()?)
        .deny(PlainText::builder().text("Nope!").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(ConfirmationDialogue::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ConfirmationDialogue>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.title()), expected["title"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .title(PlainText::builder().text("Maybe?").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["title"],
            json!({"type": "plain_text", "text": "Maybe?"})
        );
    }
    let mut invalid = expected.clone();
    invalid["title"] = json!([]);
    assert!(ConfirmationDialogue::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("title");
    assert!(ConfirmationDialogue::try_from(missing).is_err());
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(
                PlainText::builder()
                    .text("Would you like to play checkers?")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "plain_text", "text": "Would you like to play checkers?"})
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(ConfirmationDialogue::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(ConfirmationDialogue::try_from(missing).is_err());
    assert_eq!(wire(original.confirm()), expected["confirm"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .confirm(PlainText::builder().text("Yes").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["confirm"],
            json!({"type": "plain_text", "text": "Yes"})
        );
    }
    let mut invalid = expected.clone();
    invalid["confirm"] = json!([]);
    assert!(ConfirmationDialogue::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("confirm");
    assert!(ConfirmationDialogue::try_from(missing).is_err());
    assert_eq!(wire(original.deny()), expected["deny"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .deny(PlainText::builder().text("Nope!").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["deny"],
            json!({"type": "plain_text", "text": "Nope!"})
        );
    }
    let mut invalid = expected.clone();
    invalid["deny"] = json!([]);
    assert!(ConfirmationDialogue::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("deny");
    assert!(ConfirmationDialogue::try_from(missing).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .style(ButtonStyle::Primary)
            .build()?;
        assert_eq!(wire(edited)["style"], Value::from("primary"));
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = Value::from("not-a-variant");
    assert!(ConfirmationDialogue::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ConfirmationDialogue::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ConfirmationDialogue::try_from(Value::Null).is_err());
    assert!(ConfirmationDialogue::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn conversation_filter_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ConversationFilter::builder()
        .include(vec![String::from("public"), String::from("mpim")])
        .exclude_bot_users(true)
        .build()?;
    let expected = wire(&original);
    assert_eq!(ConversationFilter::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ConversationFilter>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.include()),
        expected.get("include").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .include(vec![String::from("public"), String::from("mpim")])
            .build()?;
        assert_eq!(wire(edited)["include"], json!(["public", "mpim"]));
    }
    let appended = original
        .clone()
        .into_builder()
        .add_include("public")
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["include"].as_array().unwrap().len(),
            expected["include"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["include"].as_array().unwrap().last().unwrap(),
            &Value::from("public")
        );
    }
    {
        let edited = original.clone().into_builder().clear_include().build()?;
        assert!(wire(edited).get("include").is_none());
    }
    let mut invalid = expected.clone();
    invalid["include"] = json!({});
    assert!(ConversationFilter::try_from(invalid).is_err());
    assert_eq!(
        wire(original.exclude_external_shared_channels()),
        expected
            .get("exclude_external_shared_channels")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .exclude_external_shared_channels(true)
            .build()?;
        assert_eq!(
            wire(edited)["exclude_external_shared_channels"],
            Value::from(true)
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_exclude_external_shared_channels()
            .build()?;
        assert!(
            wire(edited)
                .get("exclude_external_shared_channels")
                .is_none()
        );
    }
    let mut invalid = expected.clone();
    invalid["exclude_external_shared_channels"] = serde_json::json!(0);
    assert!(ConversationFilter::try_from(invalid).is_err());
    assert_eq!(
        wire(original.exclude_bot_users()),
        expected
            .get("exclude_bot_users")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .exclude_bot_users(true)
            .build()?;
        assert_eq!(wire(edited)["exclude_bot_users"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_exclude_bot_users()
            .build()?;
        assert!(wire(edited).get("exclude_bot_users").is_none());
    }
    let mut invalid = expected.clone();
    invalid["exclude_bot_users"] = serde_json::json!(0);
    assert!(ConversationFilter::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ConversationFilter::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ConversationFilter::try_from(Value::Null).is_err());
    assert!(ConversationFilter::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn data_point_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DataPoint::builder().label("Mon").value(12000_i64).build()?;
    let expected = wire(&original);
    assert_eq!(DataPoint::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<DataPoint>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.label()), expected["label"]);
    {
        let edited = original.clone().into_builder().label("Mon").build()?;
        assert_eq!(wire(edited)["label"], Value::from("Mon"));
    }
    let mut invalid = expected.clone();
    invalid["label"] = Value::from(false);
    assert!(DataPoint::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("label");
    assert!(DataPoint::try_from(missing).is_err());
    assert_eq!(wire(original.value()), expected["value"]);
    {
        let edited = original.clone().into_builder().value(12000_i64).build()?;
        assert_eq!(wire(edited)["value"], serde_json::json!(12000));
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from("wrong");
    assert!(DataPoint::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("value");
    assert!(DataPoint::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(DataPoint::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DataPoint::try_from(Value::Null).is_err());
    assert!(DataPoint::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn data_series_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DataSeries::builder()
        .name("Free Tier")
        .data(vec![
            DataPoint::builder().label("Mon").value(12000_i64).build()?,
            DataPoint::builder().label("Tue").value(13500_i64).build()?,
        ])
        .build()?;
    let expected = wire(&original);
    assert_eq!(DataSeries::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<DataSeries>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.name()), expected["name"]);
    {
        let edited = original.clone().into_builder().name("Free Tier").build()?;
        assert_eq!(wire(edited)["name"], Value::from("Free Tier"));
    }
    let mut invalid = expected.clone();
    invalid["name"] = Value::from(false);
    assert!(DataSeries::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("name");
    assert!(DataSeries::try_from(missing).is_err());
    assert_eq!(wire(original.data()), expected["data"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .data(vec![
                DataPoint::builder().label("Mon").value(12000_i64).build()?,
                DataPoint::builder().label("Tue").value(13500_i64).build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["data"],
            Value::Array(vec![
                Value::Object(
                    [
                        ("label".into(), Value::from("Mon")),
                        ("value".into(), serde_json::json!(12000))
                    ]
                    .into_iter()
                    .collect()
                ),
                Value::Object(
                    [
                        ("label".into(), Value::from("Tue")),
                        ("value".into(), serde_json::json!(13500))
                    ]
                    .into_iter()
                    .collect()
                )
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .point(DataPoint::builder().label("Mon").value(12000_i64).build()?)
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["data"].as_array().unwrap().len(),
            expected["data"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["data"].as_array().unwrap().last().unwrap(),
            &Value::Object(
                [
                    ("label".into(), Value::from("Mon")),
                    ("value".into(), serde_json::json!(12000))
                ]
                .into_iter()
                .collect()
            )
        );
    }
    let mut invalid = expected.clone();
    invalid["data"] = json!({});
    assert!(DataSeries::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("data");
    assert!(DataSeries::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(DataSeries::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DataSeries::try_from(Value::Null).is_err());
    assert!(DataSeries::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn dispatch_action_configuration_editing_and_ingress() -> Result<(), ValidationError> {
    let original = DispatchActionConfiguration::builder()
        .trigger_actions_on(vec![String::from("on_character_entered")])
        .build()?;
    let expected = wire(&original);
    assert_eq!(
        DispatchActionConfiguration::try_from(expected.clone())?,
        original
    );
    assert_eq!(
        serde_json::from_value::<DispatchActionConfiguration>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.trigger_actions_on()),
        expected
            .get("trigger_actions_on")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .trigger_actions_on(vec![String::from("on_character_entered")])
            .build()?;
        assert_eq!(
            wire(edited)["trigger_actions_on"],
            json!(["on_character_entered"])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .trigger_action("on_character_entered")
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["trigger_actions_on"]
                .as_array()
                .unwrap()
                .last()
                .unwrap(),
            &Value::from("on_character_entered")
        );
    }
    {
        let error = (original
            .clone()
            .into_builder()
            .clear_trigger_actions_on()
            .build())
        .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MissingRequired);
        assert_eq!(
            error.path(),
            "DispatchActionConfiguration.trigger_actions_on"
        );
    }
    let mut invalid = expected.clone();
    invalid["trigger_actions_on"] = json!({});
    assert!(DispatchActionConfiguration::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(
        DispatchActionConfiguration::try_from(wire(&extended))?,
        extended
    );
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(DispatchActionConfiguration::try_from(Value::Null).is_err());
    assert!(DispatchActionConfiguration::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn feedback_button_editing_and_ingress() -> Result<(), ValidationError> {
    let original = FeedbackButton::builder()
        .text(PlainText::builder().text("Good").build()?)
        .value("positive_feedback")
        .accessibility_label("Mark this response as good")
        .build()?;
    let expected = wire(&original);
    assert_eq!(FeedbackButton::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<FeedbackButton>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(PlainText::builder().text("Good").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "plain_text", "text": "Good"})
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(FeedbackButton::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(FeedbackButton::try_from(missing).is_err());
    assert_eq!(wire(original.value()), expected["value"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .value("positive_feedback")
            .build()?;
        assert_eq!(wire(edited)["value"], Value::from("positive_feedback"));
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from(false);
    assert!(FeedbackButton::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("value");
    assert!(FeedbackButton::try_from(missing).is_err());
    assert_eq!(
        wire(original.accessibility_label()),
        expected
            .get("accessibility_label")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .accessibility_label("Mark this response as good")
            .build()?;
        assert_eq!(
            wire(edited)["accessibility_label"],
            Value::from("Mark this response as good")
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_accessibility_label()
            .build()?;
        assert!(wire(edited).get("accessibility_label").is_none());
    }
    let mut invalid = expected.clone();
    invalid["accessibility_label"] = Value::from(false);
    assert!(FeedbackButton::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(FeedbackButton::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(FeedbackButton::try_from(Value::Null).is_err());
    assert!(FeedbackButton::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn input_parameter_editing_and_ingress() -> Result<(), ValidationError> {
    let original = InputParameter::builder()
        .name("name_a")
        .value("value_a")
        .build()?;
    let expected = wire(&original);
    assert_eq!(InputParameter::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<InputParameter>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.name()), expected["name"]);
    {
        let edited = original.clone().into_builder().name("name_a").build()?;
        assert_eq!(wire(edited)["name"], Value::from("name_a"));
    }
    let mut invalid = expected.clone();
    invalid["name"] = Value::from(false);
    assert!(InputParameter::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("name");
    assert!(InputParameter::try_from(missing).is_err());
    assert_eq!(wire(original.value()), expected["value"]);
    {
        let edited = original.clone().into_builder().value("value_a").build()?;
        assert_eq!(wire(edited)["value"], Value::from("value_a"));
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from(false);
    assert!(InputParameter::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("value");
    assert!(InputParameter::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(InputParameter::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(InputParameter::try_from(Value::Null).is_err());
    assert!(InputParameter::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn line_chart_editing_and_ingress() -> Result<(), ValidationError> {
    let original = LineChart::builder()
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
    let expected = wire(&original);
    assert_eq!(LineChart::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<LineChart>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.series()), expected["series"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["series"],
            Value::Array(vec![
                Value::Object(
                    [
                        ("name".into(), Value::from("Website")),
                        (
                            "data".into(),
                            Value::Array(vec![
                                Value::Object(
                                    [
                                        ("label".into(), Value::from("Week 1")),
                                        ("value".into(), serde_json::json!(32000))
                                    ]
                                    .into_iter()
                                    .collect()
                                ),
                                Value::Object(
                                    [
                                        ("label".into(), Value::from("Week 2")),
                                        ("value".into(), serde_json::json!(35000))
                                    ]
                                    .into_iter()
                                    .collect()
                                )
                            ])
                        )
                    ]
                    .into_iter()
                    .collect()
                ),
                Value::Object(
                    [
                        ("name".into(), Value::from("In-store")),
                        (
                            "data".into(),
                            Value::Array(vec![
                                Value::Object(
                                    [
                                        ("label".into(), Value::from("Week 1")),
                                        ("value".into(), serde_json::json!(28000))
                                    ]
                                    .into_iter()
                                    .collect()
                                ),
                                Value::Object(
                                    [
                                        ("label".into(), Value::from("Week 2")),
                                        ("value".into(), serde_json::json!(31000))
                                    ]
                                    .into_iter()
                                    .collect()
                                )
                            ])
                        )
                    ]
                    .into_iter()
                    .collect()
                )
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .series_entry(
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
        )
        .build();
    {
        let error =
            (appended).expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::InvalidUsage);
        assert_eq!(error.path(), "LineChart.series");
    }
    let mut invalid = expected.clone();
    invalid["series"] = json!({});
    assert!(LineChart::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("series");
    assert!(LineChart::try_from(missing).is_err());
    assert_eq!(wire(original.axis_config()), expected["axis_config"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .axis_config(
                AxisConfig::builder()
                    .categories(vec![String::from("Week 1"), String::from("Week 2")])
                    .x_label("Week")
                    .y_label("Paper Sales (USD)")
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["axis_config"],
            json!({"categories": ["Week 1", "Week 2"], "x_label": "Week", "y_label": "Paper Sales (USD)"})
        );
    }
    let mut invalid = expected.clone();
    invalid["axis_config"] = json!([]);
    assert!(LineChart::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("axis_config");
    assert!(LineChart::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(LineChart::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(LineChart::try_from(Value::Null).is_err());
    assert!(LineChart::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(LineChart::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn markdown_text_editing_and_ingress() -> Result<(), ValidationError> {
    let original = MarkdownText::builder().text("hi").verbatim(true).build()?;
    let expected = wire(&original);
    assert_eq!(MarkdownText::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<MarkdownText>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text("I like pretty colours")
            .build()?;
        assert_eq!(wire(edited)["text"], Value::from("I like pretty colours"));
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(MarkdownText::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(MarkdownText::try_from(missing).is_err());
    assert_eq!(
        wire(original.verbatim()),
        expected.get("verbatim").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().verbatim(true).build()?;
        assert_eq!(wire(edited)["verbatim"], Value::from(true));
    }
    {
        let edited = original.clone().into_builder().clear_verbatim().build()?;
        assert!(wire(edited).get("verbatim").is_none());
    }
    let mut invalid = expected.clone();
    invalid["verbatim"] = serde_json::json!(0);
    assert!(MarkdownText::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(MarkdownText::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(MarkdownText::try_from(Value::Null).is_err());
    assert!(MarkdownText::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(MarkdownText::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn select_option_editing_and_ingress() -> Result<(), ValidationError> {
    let original = SelectOption::builder()
        .text(MarkdownText::builder().text("*a*").build()?)
        .value("a")
        .description(PlainText::builder().text("*a*").build()?)
        .build()?;
    let expected = wire(&original);
    assert_eq!(SelectOption::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<SelectOption>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text(MarkdownText::builder().text("*a*").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["text"],
            json!({"type": "mrkdwn", "text": "*a*"})
        );
    }
    let mut invalid = expected.clone();
    invalid["text"] = json!([]);
    assert!(SelectOption::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(SelectOption::try_from(missing).is_err());
    assert_eq!(wire(original.value()), expected["value"]);
    {
        let edited = original.clone().into_builder().value("a").build()?;
        assert_eq!(wire(edited)["value"], Value::from("a"));
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from(false);
    assert!(SelectOption::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("value");
    assert!(SelectOption::try_from(missing).is_err());
    assert_eq!(
        wire(original.description()),
        expected.get("description").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .description(PlainText::builder().text("*a*").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["description"],
            json!({"type": "plain_text", "text": "*a*"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_description()
            .build()?;
        assert!(wire(edited).get("description").is_none());
    }
    let mut invalid = expected.clone();
    invalid["description"] = json!([]);
    assert!(SelectOption::try_from(invalid).is_err());
    assert_eq!(
        wire(original.url()),
        expected.get("url").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().url("sample").build()?;
        assert_eq!(wire(edited)["url"], Value::from("sample"));
    }
    {
        let edited = original.clone().into_builder().clear_url().build()?;
        assert!(wire(edited).get("url").is_none());
    }
    let mut invalid = expected.clone();
    invalid["url"] = Value::from(false);
    assert!(SelectOption::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(SelectOption::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(SelectOption::try_from(Value::Null).is_err());
    assert!(SelectOption::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn select_option_group_editing_and_ingress() -> Result<(), ValidationError> {
    let original = SelectOptionGroup::builder()
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
    let expected = wire(&original);
    assert_eq!(SelectOptionGroup::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<SelectOptionGroup>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.label()), expected["label"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .label(PlainText::builder().text("Group A").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["label"],
            json!({"type": "plain_text", "text": "Group A"})
        );
    }
    let mut invalid = expected.clone();
    invalid["label"] = json!([]);
    assert!(SelectOptionGroup::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("label");
    assert!(SelectOptionGroup::try_from(missing).is_err());
    assert_eq!(wire(original.options()), expected["options"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["options"],
            json!([
                {
                    "text": {
                        "type": "plain_text",
                        "text": "A"
                    },
                    "value": "A"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "B"
                    },
                    "value": "B"
                },
                {
                    "text": {
                        "type": "plain_text",
                        "text": "C"
                    },
                    "value": "C"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .option(
            SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("A")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["options"].as_array().unwrap().len(),
            expected["options"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["options"].as_array().unwrap().last().unwrap(),
            &json!({"text": {"type": "plain_text", "text": "A"}, "value": "A"})
        );
    }
    let mut invalid = expected.clone();
    invalid["options"] = json!({});
    assert!(SelectOptionGroup::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("options");
    assert!(SelectOptionGroup::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(SelectOptionGroup::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(SelectOptionGroup::try_from(Value::Null).is_err());
    assert!(SelectOptionGroup::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn pie_chart_editing_and_ingress() -> Result<(), ValidationError> {
    let original = PieChart::builder()
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
    let expected = wire(&original);
    assert_eq!(PieChart::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<PieChart>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.segments()), expected["segments"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["segments"],
            Value::Array(vec![
                Value::Object(
                    [
                        ("label".into(), Value::from("Kit Kat")),
                        ("value".into(), serde_json::json!(45))
                    ]
                    .into_iter()
                    .collect()
                ),
                Value::Object(
                    [
                        ("label".into(), Value::from("Twix")),
                        ("value".into(), serde_json::json!(28))
                    ]
                    .into_iter()
                    .collect()
                ),
                Value::Object(
                    [
                        ("label".into(), Value::from("Crunch")),
                        ("value".into(), serde_json::json!(18))
                    ]
                    .into_iter()
                    .collect()
                ),
                Value::Object(
                    [
                        ("label".into(), Value::from("Milky Way")),
                        ("value".into(), serde_json::json!(9))
                    ]
                    .into_iter()
                    .collect()
                )
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .segment(
            ChartSegment::builder()
                .label("Kit Kat")
                .value(45_i64)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["segments"].as_array().unwrap().len(),
            expected["segments"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["segments"].as_array().unwrap().last().unwrap(),
            &Value::Object(
                [
                    ("label".into(), Value::from("Kit Kat")),
                    ("value".into(), serde_json::json!(45))
                ]
                .into_iter()
                .collect()
            )
        );
    }
    let mut invalid = expected.clone();
    invalid["segments"] = json!({});
    assert!(PieChart::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("segments");
    assert!(PieChart::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(PieChart::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(PieChart::try_from(Value::Null).is_err());
    assert!(PieChart::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(PieChart::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn plain_text_editing_and_ingress() -> Result<(), ValidationError> {
    let original = PlainText::builder().text("Label").emoji(true).build()?;
    let expected = wire(&original);
    assert_eq!(PlainText::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<PlainText>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original.clone().into_builder().text("*a*").build()?;
        assert_eq!(wire(edited)["text"], Value::from("*a*"));
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(PlainText::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(PlainText::try_from(missing).is_err());
    assert_eq!(
        wire(original.emoji()),
        expected.get("emoji").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().emoji(true).build()?;
        assert_eq!(wire(edited)["emoji"], Value::from(true));
    }
    {
        let edited = original.clone().into_builder().clear_emoji().build()?;
        assert!(wire(edited).get("emoji").is_none());
    }
    let mut invalid = expected.clone();
    invalid["emoji"] = serde_json::json!(0);
    assert!(PlainText::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(PlainText::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(PlainText::try_from(Value::Null).is_err());
    assert!(PlainText::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(PlainText::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn raw_number_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RawNumber::builder().value(42_i64).text("42").build()?;
    let expected = wire(&original);
    assert_eq!(RawNumber::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RawNumber>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.value()), expected["value"]);
    {
        let edited = original.clone().into_builder().value(42_i64).build()?;
        assert_eq!(wire(edited)["value"], serde_json::json!(42));
    }
    let mut invalid = expected.clone();
    invalid["value"] = Value::from("wrong");
    assert!(RawNumber::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("value");
    assert!(RawNumber::try_from(missing).is_err());
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original.clone().into_builder().text("42").build()?;
        assert_eq!(wire(edited)["text"], Value::from("42"));
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(RawNumber::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(RawNumber::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RawNumber::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RawNumber::try_from(Value::Null).is_err());
    assert!(RawNumber::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RawNumber::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn raw_text_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RawText::builder().text("Name").build()?;
    let expected = wire(&original);
    assert_eq!(RawText::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RawText>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original.clone().into_builder().text("Name").build()?;
        assert_eq!(wire(edited)["text"], Value::from("Name"));
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(RawText::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(RawText::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RawText::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RawText::try_from(Value::Null).is_err());
    assert!(RawText::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RawText::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_channel_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextChannel::builder()
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
    let expected = wire(&original);
    assert_eq!(RichTextChannel::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextChannel>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.channel_id()), expected["channel_id"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .channel_id("C0261C65XNY")
            .build()?;
        assert_eq!(wire(edited)["channel_id"], Value::from("C0261C65XNY"));
    }
    let mut invalid = expected.clone();
    invalid["channel_id"] = Value::from(false);
    assert!(RichTextChannel::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("channel_id");
    assert!(RichTextChannel::try_from(missing).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["style"],
            json!({
                "bold": true,
                "italic": false,
                "strike": true,
                "highlight": true,
                "client_highlight": true,
                "unlink": false
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = json!([]);
    assert!(RichTextChannel::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextChannel::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextChannel::try_from(Value::Null).is_err());
    assert!(RichTextChannel::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextChannel::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_code_block_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextCodeBlock::builder()
        .elements(vec![RichTextSectionElement::from(
            RichTextText::builder()
                .text("\ndef hello_world():\n    print('hello, world')")
                .build()?,
        )])
        .border(0_i64)
        .build()?;
    let expected = wire(&original);
    assert_eq!(RichTextCodeBlock::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextCodeBlock>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .elements(vec![RichTextSectionElement::from(
                RichTextText::builder()
                    .text("\ndef hello_world():\n    print('hello, world')")
                    .build()?,
            )])
            .build()?;
        assert_eq!(
            wire(edited)["elements"],
            json!([{"type": "text", "text": "\ndef hello_world():\n    print('hello, world')"}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
            RichTextText::builder()
                .text("\ndef hello_world():\n    print('hello, world')")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({"type": "text", "text": "\ndef hello_world():\n    print('hello, world')"})
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(RichTextCodeBlock::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(RichTextCodeBlock::try_from(missing).is_err());
    assert_eq!(
        wire(original.border()),
        expected.get("border").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().border(0_i64).build()?;
        assert_eq!(wire(edited)["border"], serde_json::json!(0));
    }
    {
        let edited = original.clone().into_builder().clear_border().build()?;
        assert!(wire(edited).get("border").is_none());
    }
    let mut invalid = expected.clone();
    invalid["border"] = Value::from("wrong");
    assert!(RichTextCodeBlock::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextCodeBlock::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextCodeBlock::try_from(Value::Null).is_err());
    assert!(RichTextCodeBlock::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextCodeBlock::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_emoji_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextEmoji::builder().name("wave").build()?;
    let expected = wire(&original);
    assert_eq!(RichTextEmoji::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextEmoji>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.name()), expected["name"]);
    {
        let edited = original.clone().into_builder().name("wave").build()?;
        assert_eq!(wire(edited)["name"], Value::from("wave"));
    }
    let mut invalid = expected.clone();
    invalid["name"] = Value::from(false);
    assert!(RichTextEmoji::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("name");
    assert!(RichTextEmoji::try_from(missing).is_err());
    assert_eq!(
        wire(original.skin_tone()),
        expected.get("skin_tone").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().skin_tone(1_i64).build()?;
        assert_eq!(wire(edited)["skin_tone"], serde_json::json!(1));
    }
    {
        let edited = original.clone().into_builder().clear_skin_tone().build()?;
        assert!(wire(edited).get("skin_tone").is_none());
    }
    let mut invalid = expected.clone();
    invalid["skin_tone"] = Value::from("wrong");
    assert!(RichTextEmoji::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextEmoji::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextEmoji::try_from(Value::Null).is_err());
    assert!(RichTextEmoji::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextEmoji::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_link_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextLink::builder()
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
    let expected = wire(&original);
    assert_eq!(RichTextLink::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextLink>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.url()), expected["url"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .url("https://slack.com")
            .build()?;
        assert_eq!(wire(edited)["url"], Value::from("https://slack.com"));
    }
    let mut invalid = expected.clone();
    invalid["url"] = Value::from(false);
    assert!(RichTextLink::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("url");
    assert!(RichTextLink::try_from(missing).is_err());
    assert_eq!(
        wire(original.text()),
        expected.get("text").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().text("Data 1B").build()?;
        assert_eq!(wire(edited)["text"], Value::from("Data 1B"));
    }
    {
        let edited = original.clone().into_builder().clear_text().build()?;
        assert!(wire(edited).get("text").is_none());
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(RichTextLink::try_from(invalid).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .style(
                RichTextStyle::new()
                    .bold(true)
                    .italic(false)
                    .strike(true)
                    .code(true),
            )
            .build()?;
        assert_eq!(
            wire(edited)["style"],
            json!({"bold": true, "italic": false, "strike": true, "code": true})
        );
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = json!([]);
    assert!(RichTextLink::try_from(invalid).is_err());
    assert_eq!(
        wire(original.r#unsafe()),
        expected.get("unsafe").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().r#unsafe(false).build()?;
        assert_eq!(wire(edited)["unsafe"], Value::from(false));
    }
    {
        let edited = original.clone().into_builder().clear_unsafe().build()?;
        assert!(wire(edited).get("unsafe").is_none());
    }
    let mut invalid = expected.clone();
    invalid["unsafe"] = serde_json::json!(0);
    assert!(RichTextLink::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextLink::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextLink::try_from(Value::Null).is_err());
    assert!(RichTextLink::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextLink::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_list_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextList::builder()
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
    let expected = wire(&original);
    assert_eq!(RichTextList::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextList>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.style()), expected["style"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .style(RichTextListStyle::Bullet)
            .build()?;
        assert_eq!(wire(edited)["style"], Value::from("bullet"));
    }
    let mut invalid = expected.clone();
    invalid["style"] = Value::from("not-a-variant");
    assert!(RichTextList::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("style");
    assert!(RichTextList::try_from(missing).is_err());
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["elements"],
            json!([
                {
                    "type": "rich_text_section",
                    "elements": [
                        {
                            "type": "text",
                            "text": "Oh"
                        }
                    ]
                },
                {
                    "type": "rich_text_section",
                    "elements": [
                        {
                            "type": "text",
                            "text": "Hi"
                        }
                    ]
                },
                {
                    "type": "rich_text_section",
                    "elements": [
                        {
                            "type": "text",
                            "text": "Mark"
                        }
                    ]
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
            RichTextSection::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Oh").build()?,
                )])
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({"type": "rich_text_section", "elements": [{"type": "text", "text": "Oh"}]})
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(RichTextList::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(RichTextList::try_from(missing).is_err());
    assert_eq!(
        wire(original.indent()),
        expected.get("indent").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().indent(0_i64).build()?;
        assert_eq!(wire(edited)["indent"], serde_json::json!(0));
    }
    {
        let edited = original.clone().into_builder().clear_indent().build()?;
        assert!(wire(edited).get("indent").is_none());
    }
    let mut invalid = expected.clone();
    invalid["indent"] = Value::from("wrong");
    assert!(RichTextList::try_from(invalid).is_err());
    assert_eq!(
        wire(original.offset()),
        expected.get("offset").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().offset(0_i64).build()?;
        assert_eq!(wire(edited)["offset"], serde_json::json!(0));
    }
    {
        let edited = original.clone().into_builder().clear_offset().build()?;
        assert!(wire(edited).get("offset").is_none());
    }
    let mut invalid = expected.clone();
    invalid["offset"] = Value::from("wrong");
    assert!(RichTextList::try_from(invalid).is_err());
    assert_eq!(
        wire(original.border()),
        expected.get("border").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().border(1_i64).build()?;
        assert_eq!(wire(edited)["border"], serde_json::json!(1));
    }
    {
        let edited = original.clone().into_builder().clear_border().build()?;
        assert!(wire(edited).get("border").is_none());
    }
    let mut invalid = expected.clone();
    invalid["border"] = Value::from("wrong");
    assert!(RichTextList::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextList::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextList::try_from(Value::Null).is_err());
    assert!(RichTextList::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextList::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_quote_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextQuote::builder()
        .elements(vec![RichTextSectionElement::from(
            RichTextText::builder()
                .text("Great and good are seldom the same man")
                .build()?,
        )])
        .border(1_i64)
        .build()?;
    let expected = wire(&original);
    assert_eq!(RichTextQuote::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextQuote>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .elements(vec![RichTextSectionElement::from(
                RichTextText::builder()
                    .text("Great and good are seldom the same man")
                    .build()?,
            )])
            .build()?;
        assert_eq!(
            wire(edited)["elements"],
            json!([{"type": "text", "text": "Great and good are seldom the same man"}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
            RichTextText::builder()
                .text("Great and good are seldom the same man")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({"type": "text", "text": "Great and good are seldom the same man"})
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(RichTextQuote::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(RichTextQuote::try_from(missing).is_err());
    assert_eq!(
        wire(original.border()),
        expected.get("border").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().border(1_i64).build()?;
        assert_eq!(wire(edited)["border"], serde_json::json!(1));
    }
    {
        let edited = original.clone().into_builder().clear_border().build()?;
        assert!(wire(edited).get("border").is_none());
    }
    let mut invalid = expected.clone();
    invalid["border"] = Value::from("wrong");
    assert!(RichTextQuote::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextQuote::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextQuote::try_from(Value::Null).is_err());
    assert!(RichTextQuote::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextQuote::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_section_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextSection::builder()
        .elements(vec![RichTextSectionElement::from(
            RichTextText::builder()
                .text("Profile data loaded")
                .build()?,
        )])
        .build()?;
    let expected = wire(&original);
    assert_eq!(RichTextSection::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextSection>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.elements()), expected["elements"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .elements(vec![RichTextSectionElement::from(
                RichTextText::builder()
                    .text("Profile data loaded")
                    .build()?,
            )])
            .build()?;
        assert_eq!(
            wire(edited)["elements"],
            json!([{"type": "text", "text": "Profile data loaded"}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .element(
            RichTextText::builder()
                .text("Profile data loaded")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["elements"].as_array().unwrap().len(),
            expected["elements"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["elements"].as_array().unwrap().last().unwrap(),
            &json!({"type": "text", "text": "Profile data loaded"})
        );
    }
    let mut invalid = expected.clone();
    invalid["elements"] = json!({});
    assert!(RichTextSection::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("elements");
    assert!(RichTextSection::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextSection::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextSection::try_from(Value::Null).is_err());
    assert!(RichTextSection::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextSection::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_text_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextText::builder()
        .text("You 'bout to witness hip-hop in its most purest")
        .style(RichTextStyle::new().bold(true))
        .build()?;
    let expected = wire(&original);
    assert_eq!(RichTextText::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextText>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text("Profile data loaded")
            .build()?;
        assert_eq!(wire(edited)["text"], Value::from("Profile data loaded"));
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(RichTextText::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(RichTextText::try_from(missing).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .style(RichTextStyle::new().bold(true))
            .build()?;
        assert_eq!(wire(edited)["style"], json!({"bold": true}));
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = json!([]);
    assert!(RichTextText::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextText::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextText::try_from(Value::Null).is_err());
    assert!(RichTextText::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextText::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_user_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextUser::builder()
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
    let expected = wire(&original);
    assert_eq!(RichTextUser::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextUser>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.user_id()), expected["user_id"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .user_id("DR36TNNLA")
            .build()?;
        assert_eq!(wire(edited)["user_id"], Value::from("DR36TNNLA"));
    }
    let mut invalid = expected.clone();
    invalid["user_id"] = Value::from(false);
    assert!(RichTextUser::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("user_id");
    assert!(RichTextUser::try_from(missing).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["style"],
            json!({
                "bold": true,
                "italic": false,
                "strike": true,
                "highlight": true,
                "client_highlight": true,
                "unlink": false
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = json!([]);
    assert!(RichTextUser::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextUser::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextUser::try_from(Value::Null).is_err());
    assert!(RichTextUser::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextUser::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn rich_text_user_group_editing_and_ingress() -> Result<(), ValidationError> {
    let original = RichTextUserGroup::builder()
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
    let expected = wire(&original);
    assert_eq!(RichTextUserGroup::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<RichTextUserGroup>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.usergroup_id()), expected["usergroup_id"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .usergroup_id("C01RGRU0RUK")
            .build()?;
        assert_eq!(wire(edited)["usergroup_id"], Value::from("C01RGRU0RUK"));
    }
    let mut invalid = expected.clone();
    invalid["usergroup_id"] = Value::from(false);
    assert!(RichTextUserGroup::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("usergroup_id");
    assert!(RichTextUserGroup::try_from(missing).is_err());
    assert_eq!(
        wire(original.style()),
        expected.get("style").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
        assert_eq!(
            wire(edited)["style"],
            json!({
                "bold": true,
                "italic": false,
                "strike": true,
                "highlight": true,
                "client_highlight": true,
                "unlink": false
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_style().build()?;
        assert!(wire(edited).get("style").is_none());
    }
    let mut invalid = expected.clone();
    invalid["style"] = json!([]);
    assert!(RichTextUserGroup::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(RichTextUserGroup::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(RichTextUserGroup::try_from(Value::Null).is_err());
    assert!(RichTextUserGroup::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(RichTextUserGroup::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn slack_file_editing_and_ingress() -> Result<(), ValidationError> {
    let original = SlackFile::builder()
        .url("https://files.slack.com/files-pri/T0123456-F0123ABC456/kitten.png")
        .build()?;
    let expected = wire(&original);
    assert_eq!(SlackFile::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<SlackFile>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.id()),
        expected.get("id").cloned().unwrap_or(Value::Null)
    );
    {
        let error = (original.clone().into_builder().id("F0123ABC456").build())
            .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "SlackFile");
    }
    {
        let edited = original.clone().into_builder().clear_id().build()?;
        assert!(wire(edited).get("id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["id"] = Value::from(false);
    assert!(SlackFile::try_from(invalid).is_err());
    assert_eq!(
        wire(original.url()),
        expected.get("url").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .url("https://files.slack.com/files-pri/T0123456-F0123ABC456/kitten.png")
            .build()?;
        assert_eq!(
            wire(edited)["url"],
            Value::from("https://files.slack.com/files-pri/T0123456-F0123ABC456/kitten.png")
        );
    }
    {
        let error = (original.clone().into_builder().clear_url().build())
            .expect_err("this edit violates a field dependency or uniqueness rule");
        assert_eq!(error.category(), ErrorCategory::MutuallyExclusive);
        assert_eq!(error.path(), "SlackFile");
    }
    let mut invalid = expected.clone();
    invalid["url"] = Value::from(false);
    assert!(SlackFile::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(SlackFile::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(SlackFile::try_from(Value::Null).is_err());
    assert!(SlackFile::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn slack_icon_editing_and_ingress() -> Result<(), ValidationError> {
    let original = SlackIcon::builder().name("bot").build()?;
    let expected = wire(&original);
    assert_eq!(SlackIcon::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<SlackIcon>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.name()), expected["name"]);
    {
        let edited = original.clone().into_builder().name("bot").build()?;
        assert_eq!(wire(edited)["name"], Value::from("bot"));
    }
    let mut invalid = expected.clone();
    invalid["name"] = Value::from(false);
    assert!(SlackIcon::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("name");
    assert!(SlackIcon::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(SlackIcon::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(SlackIcon::try_from(Value::Null).is_err());
    assert!(SlackIcon::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(SlackIcon::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn trigger_editing_and_ingress() -> Result<(), ValidationError> {
    let original = Trigger::builder()
        .url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c")
        .customizable_input_parameters(vec![
            InputParameter::builder()
                .name("name_a")
                .value("value_a")
                .build()?,
            InputParameter::builder()
                .name("name_b")
                .value("value_b")
                .build()?,
        ])
        .build()?;
    let expected = wire(&original);
    assert_eq!(Trigger::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<Trigger>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.url()), expected["url"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c")
            .build()?;
        assert_eq!(
            wire(edited)["url"],
            Value::from(
                "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c"
            )
        );
    }
    let mut invalid = expected.clone();
    invalid["url"] = Value::from(false);
    assert!(Trigger::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("url");
    assert!(Trigger::try_from(missing).is_err());
    assert_eq!(
        wire(original.customizable_input_parameters()),
        expected
            .get("customizable_input_parameters")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .customizable_input_parameters(vec![
                InputParameter::builder()
                    .name("name_a")
                    .value("value_a")
                    .build()?,
                InputParameter::builder()
                    .name("name_b")
                    .value("value_b")
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["customizable_input_parameters"],
            json!([{"name": "name_a", "value": "value_a"}, {"name": "name_b", "value": "value_b"}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .customizable_input_parameter(
            InputParameter::builder()
                .name("name_a")
                .value("value_a")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["customizable_input_parameters"]
                .as_array()
                .unwrap()
                .last()
                .unwrap(),
            &json!({"name": "name_a", "value": "value_a"})
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_customizable_input_parameters()
            .build()?;
        assert!(wire(edited).get("customizable_input_parameters").is_none());
    }
    let mut invalid = expected.clone();
    invalid["customizable_input_parameters"] = json!({});
    assert!(Trigger::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(Trigger::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(Trigger::try_from(Value::Null).is_err());
    assert!(Trigger::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn url_source_editing_and_ingress() -> Result<(), ValidationError> {
    let original = UrlSource::builder()
        .url("https://weather.com/")
        .text("weather.com")
        .build()?;
    let expected = wire(&original);
    assert_eq!(UrlSource::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<UrlSource>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.url()), expected["url"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .url("https://weather.com/")
            .build()?;
        assert_eq!(wire(edited)["url"], Value::from("https://weather.com/"));
    }
    let mut invalid = expected.clone();
    invalid["url"] = Value::from(false);
    assert!(UrlSource::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("url");
    assert!(UrlSource::try_from(missing).is_err());
    assert_eq!(wire(original.text()), expected["text"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .text("weather.com")
            .build()?;
        assert_eq!(wire(edited)["text"], Value::from("weather.com"));
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(UrlSource::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("text");
    assert!(UrlSource::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(UrlSource::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(UrlSource::try_from(Value::Null).is_err());
    assert!(UrlSource::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(UrlSource::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn workflow_editing_and_ingress() -> Result<(), ValidationError> {
    let original = Workflow::builder()
        .trigger(
            Trigger::builder()
                .url("https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c")
                .customizable_input_parameters(vec![
                    InputParameter::builder()
                        .name("name_a")
                        .value("value_a")
                        .build()?,
                    InputParameter::builder()
                        .name("name_b")
                        .value("value_b")
                        .build()?,
                ])
                .build()?,
        )
        .build()?;
    let expected = wire(&original);
    assert_eq!(Workflow::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<Workflow>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.trigger()), expected["trigger"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .trigger(
                Trigger::builder()
                    .url(
                        "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
                    )
                    .customizable_input_parameters(vec![
                        InputParameter::builder()
                            .name("name_a")
                            .value("value_a")
                            .build()?,
                        InputParameter::builder()
                            .name("name_b")
                            .value("value_b")
                            .build()?,
                    ])
                    .build()?,
            )
            .build()?;
        assert_eq!(
            wire(edited)["trigger"],
            json!({
                "url": "https://slack.com/shortcuts/Ft012KXZK1MZ/8831723c452aac3e87c6d3219bebd44c",
                "customizable_input_parameters": [
                    {
                        "name": "name_a",
                        "value": "value_a"
                    },
                    {
                        "name": "name_b",
                        "value": "value_b"
                    }
                ]
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["trigger"] = json!([]);
    assert!(Workflow::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("trigger");
    assert!(Workflow::try_from(missing).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(Workflow::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(Workflow::try_from(Value::Null).is_err());
    assert!(Workflow::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn attachment_editing_and_ingress() -> Result<(), ValidationError> {
    let original = Attachment::builder()
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
    let expected = wire(&original);
    assert_eq!(Attachment::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<Attachment>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.blocks()), expected["blocks"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["blocks"],
            json!([
                {
                    "type": "section",
                    "block_id": "fake_block_id_0",
                    "text": {
                        "type": "mrkdwn",
                        "text": "I like pretty colours"
                    }
                },
                {
                    "type": "section",
                    "block_id": "fake_block_id_1",
                    "text": {
                        "type": "mrkdwn",
                        "text": "I don't like pretty colours"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .block(
            SectionBlock::builder()
                .block_id("fake_block_id_0")
                .text(
                    MarkdownText::builder()
                        .text("I like pretty colours")
                        .build()?,
                )
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["blocks"].as_array().unwrap().len(),
            expected["blocks"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["blocks"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "section",
                "block_id": "fake_block_id_0",
                "text": {
                    "type": "mrkdwn",
                    "text": "I like pretty colours"
                }
            })
        );
    }
    let mut invalid = expected.clone();
    invalid["blocks"] = json!({});
    assert!(Attachment::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("blocks");
    assert!(Attachment::try_from(missing).is_err());
    assert_eq!(
        wire(original.color()),
        expected.get("color").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().color("#8800ff").build()?;
        assert_eq!(wire(edited)["color"], Value::from("#8800ff"));
    }
    {
        let edited = original.clone().into_builder().clear_color().build()?;
        assert!(wire(edited).get("color").is_none());
    }
    let mut invalid = expected.clone();
    invalid["color"] = Value::from(false);
    assert!(Attachment::try_from(invalid).is_err());
    assert_eq!(
        wire(original.fallback()),
        expected.get("fallback").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .fallback("Colours preference")
            .build()?;
        assert_eq!(wire(edited)["fallback"], Value::from("Colours preference"));
    }
    {
        let edited = original.clone().into_builder().clear_fallback().build()?;
        assert!(wire(edited).get("fallback").is_none());
    }
    let mut invalid = expected.clone();
    invalid["fallback"] = Value::from(false);
    assert!(Attachment::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(Attachment::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(Attachment::try_from(Value::Null).is_err());
    assert!(Attachment::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn home_tab_view_editing_and_ingress() -> Result<(), ValidationError> {
    let original = HomeTabView::builder()
        .blocks(vec![Block::from(
            SectionBlock::builder()
                .block_id("fake_id")
                .text(MarkdownText::builder().text("Example Block").build()?)
                .build()?,
        )])
        .build()?;
    let expected = wire(&original);
    assert_eq!(HomeTabView::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<HomeTabView>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.blocks()), expected["blocks"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .blocks(vec![Block::from(
                SectionBlock::builder()
                    .block_id("fake_id")
                    .text(MarkdownText::builder().text("Example Block").build()?)
                    .build()?,
            )])
            .build()?;
        assert_eq!(
            wire(edited)["blocks"],
            json!([{"type": "section", "block_id": "fake_id", "text": {"type": "mrkdwn", "text": "Example Block"}}])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .block(
            SectionBlock::builder()
                .block_id("fake_id")
                .text(MarkdownText::builder().text("Example Block").build()?)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["blocks"].as_array().unwrap().len(),
            expected["blocks"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["blocks"].as_array().unwrap().last().unwrap(),
            &json!({"type": "section", "block_id": "fake_id", "text": {"type": "mrkdwn", "text": "Example Block"}})
        );
    }
    let mut invalid = expected.clone();
    invalid["blocks"] = json!({});
    assert!(HomeTabView::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("blocks");
    assert!(HomeTabView::try_from(missing).is_err());
    assert_eq!(
        wire(original.private_metadata()),
        expected
            .get("private_metadata")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .private_metadata("sample")
            .build()?;
        assert_eq!(wire(edited)["private_metadata"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_private_metadata()
            .build()?;
        assert!(wire(edited).get("private_metadata").is_none());
    }
    let mut invalid = expected.clone();
    invalid["private_metadata"] = Value::from(false);
    assert!(HomeTabView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.callback_id()),
        expected.get("callback_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .callback_id("sample")
            .build()?;
        assert_eq!(wire(edited)["callback_id"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_callback_id()
            .build()?;
        assert!(wire(edited).get("callback_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["callback_id"] = Value::from(false);
    assert!(HomeTabView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.external_id()),
        expected.get("external_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .external_id("sample")
            .build()?;
        assert_eq!(wire(edited)["external_id"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_external_id()
            .build()?;
        assert!(wire(edited).get("external_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["external_id"] = Value::from(false);
    assert!(HomeTabView::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(HomeTabView::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(HomeTabView::try_from(Value::Null).is_err());
    assert!(HomeTabView::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(HomeTabView::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn message_payload_editing_and_ingress() -> Result<(), ValidationError> {
    let original = MessagePayload::builder("#slackblocks")
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
    let expected = wire(&original);
    assert_eq!(MessagePayload::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<MessagePayload>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.channel()), expected["channel"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .channel("#slackblocks")
            .build()?;
        assert_eq!(wire(edited)["channel"], Value::from("#slackblocks"));
    }
    let mut invalid = expected.clone();
    invalid["channel"] = Value::from(false);
    assert!(MessagePayload::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("channel");
    assert!(MessagePayload::try_from(missing).is_err());
    assert_eq!(
        wire(original.blocks()),
        expected.get("blocks").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .blocks(vec![Block::from(
                SectionBlock::builder()
                    .block_id("fake_block_id")
                    .text(MarkdownText::builder().text("Hello, world!").build()?)
                    .build()?,
            )])
            .build()?;
        assert_eq!(
            wire(edited)["blocks"],
            json!([
                {
                    "type": "section",
                    "block_id": "fake_block_id",
                    "text": {
                        "type": "mrkdwn",
                        "text": "Hello, world!"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .block(
            SectionBlock::builder()
                .block_id("fake_block_id")
                .text(MarkdownText::builder().text("Hello, world!").build()?)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["blocks"].as_array().unwrap().len(),
            expected["blocks"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["blocks"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "section",
                "block_id": "fake_block_id",
                "text": {
                    "type": "mrkdwn",
                    "text": "Hello, world!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_blocks().build()?;
        assert!(wire(edited).get("blocks").is_none());
    }
    let mut invalid = expected.clone();
    invalid["blocks"] = json!({});
    assert!(MessagePayload::try_from(invalid).is_err());
    assert_eq!(
        wire(original.attachments()),
        expected.get("attachments").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["attachments"],
            json!([
                {
                    "blocks": [
                        {
                            "type": "section",
                            "block_id": "block1",
                            "text": {
                                "type": "mrkdwn",
                                "text": "Hello, world!"
                            }
                        }
                    ],
                    "color": "#000000"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .attachment(
            Attachment::builder()
                .blocks(vec![Block::from(
                    SectionBlock::builder()
                        .block_id("block1")
                        .text(MarkdownText::builder().text("Hello, world!").build()?)
                        .build()?,
                )])
                .color("#000000")
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["attachments"].as_array().unwrap().len(),
            expected["attachments"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["attachments"].as_array().unwrap().last().unwrap(),
            &json!({
                "blocks": [
                    {
                        "type": "section",
                        "block_id": "block1",
                        "text": {
                            "type": "mrkdwn",
                            "text": "Hello, world!"
                        }
                    }
                ],
                "color": "#000000"
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_attachments()
            .build()?;
        assert!(wire(edited).get("attachments").is_none());
    }
    let mut invalid = expected.clone();
    invalid["attachments"] = json!({});
    assert!(MessagePayload::try_from(invalid).is_err());
    assert_eq!(
        wire(original.text()),
        expected.get("text").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().text("").build()?;
        assert_eq!(wire(edited)["text"], Value::from(""));
    }
    {
        let edited = original.clone().into_builder().clear_text().build()?;
        assert!(wire(edited).get("text").is_none());
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(MessagePayload::try_from(invalid).is_err());
    assert_eq!(
        wire(original.mrkdwn()),
        expected.get("mrkdwn").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().mrkdwn(true).build()?;
        assert_eq!(wire(edited)["mrkdwn"], Value::from(true));
    }
    {
        let edited = original.clone().into_builder().clear_mrkdwn().build()?;
        assert!(wire(edited).get("mrkdwn").is_none());
    }
    let mut invalid = expected.clone();
    invalid["mrkdwn"] = serde_json::json!(0);
    assert!(MessagePayload::try_from(invalid).is_err());
    assert_eq!(
        wire(original.unfurl_links()),
        expected.get("unfurl_links").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .unfurl_links(false)
            .build()?;
        assert_eq!(wire(edited)["unfurl_links"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_unfurl_links()
            .build()?;
        assert!(wire(edited).get("unfurl_links").is_none());
    }
    let mut invalid = expected.clone();
    invalid["unfurl_links"] = serde_json::json!(0);
    assert!(MessagePayload::try_from(invalid).is_err());
    assert_eq!(
        wire(original.unfurl_media()),
        expected.get("unfurl_media").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .unfurl_media(false)
            .build()?;
        assert_eq!(wire(edited)["unfurl_media"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_unfurl_media()
            .build()?;
        assert!(wire(edited).get("unfurl_media").is_none());
    }
    let mut invalid = expected.clone();
    invalid["unfurl_media"] = serde_json::json!(0);
    assert!(MessagePayload::try_from(invalid).is_err());
    assert_eq!(
        wire(original.metadata()),
        expected.get("metadata").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .metadata(match json!({"future": false}) {
                Value::Object(map) => map,
                _ => unreachable!(),
            })
            .build()?;
        assert_eq!(wire(edited)["metadata"], json!({"future": false}));
    }
    {
        let edited = original.clone().into_builder().clear_metadata().build()?;
        assert!(wire(edited).get("metadata").is_none());
    }
    let mut invalid = expected.clone();
    invalid["metadata"] = json!([]);
    assert!(MessagePayload::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(MessagePayload::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(MessagePayload::try_from(Value::Null).is_err());
    assert!(MessagePayload::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn message_response_editing_and_ingress() -> Result<(), ValidationError> {
    let original = MessageResponse::builder()
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
    let expected = wire(&original);
    assert_eq!(MessageResponse::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<MessageResponse>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.blocks()),
        expected.get("blocks").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .blocks(vec![Block::from(
                SectionBlock::builder()
                    .block_id("fake_block_id")
                    .text(MarkdownText::builder().text("Hello, world!").build()?)
                    .build()?,
            )])
            .build()?;
        assert_eq!(
            wire(edited)["blocks"],
            json!([
                {
                    "type": "section",
                    "block_id": "fake_block_id",
                    "text": {
                        "type": "mrkdwn",
                        "text": "Hello, world!"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .block(
            SectionBlock::builder()
                .block_id("fake_block_id")
                .text(MarkdownText::builder().text("Hello, world!").build()?)
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["blocks"].as_array().unwrap().len(),
            expected["blocks"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["blocks"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "section",
                "block_id": "fake_block_id",
                "text": {
                    "type": "mrkdwn",
                    "text": "Hello, world!"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_blocks().build()?;
        assert!(wire(edited).get("blocks").is_none());
    }
    let mut invalid = expected.clone();
    invalid["blocks"] = json!({});
    assert!(MessageResponse::try_from(invalid).is_err());
    assert_eq!(
        wire(original.attachments()),
        expected.get("attachments").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .attachments(vec![
                Attachment::builder()
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
                    .build()?,
            ])
            .build()?;
        assert_eq!(
            wire(edited)["attachments"],
            json!([
                {
                    "blocks": [
                        {
                            "type": "section",
                            "block_id": "fake_block_id_0",
                            "text": {
                                "type": "mrkdwn",
                                "text": "I like pretty colours"
                            }
                        },
                        {
                            "type": "section",
                            "block_id": "fake_block_id_1",
                            "text": {
                                "type": "mrkdwn",
                                "text": "I don't like pretty colours"
                            }
                        }
                    ],
                    "color": "#8800ff"
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .attachment(
            Attachment::builder()
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
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["attachments"].as_array().unwrap().len(),
            expected["attachments"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["attachments"].as_array().unwrap().last().unwrap(),
            &json!({
                "blocks": [
                    {
                        "type": "section",
                        "block_id": "fake_block_id_0",
                        "text": {
                            "type": "mrkdwn",
                            "text": "I like pretty colours"
                        }
                    },
                    {
                        "type": "section",
                        "block_id": "fake_block_id_1",
                        "text": {
                            "type": "mrkdwn",
                            "text": "I don't like pretty colours"
                        }
                    }
                ],
                "color": "#8800ff"
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_attachments()
            .build()?;
        assert!(wire(edited).get("attachments").is_none());
    }
    let mut invalid = expected.clone();
    invalid["attachments"] = json!({});
    assert!(MessageResponse::try_from(invalid).is_err());
    assert_eq!(
        wire(original.text()),
        expected.get("text").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().text("").build()?;
        assert_eq!(wire(edited)["text"], Value::from(""));
    }
    {
        let edited = original.clone().into_builder().clear_text().build()?;
        assert!(wire(edited).get("text").is_none());
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(MessageResponse::try_from(invalid).is_err());
    assert_eq!(
        wire(original.mrkdwn()),
        expected.get("mrkdwn").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().mrkdwn(true).build()?;
        assert_eq!(wire(edited)["mrkdwn"], Value::from(true));
    }
    {
        let edited = original.clone().into_builder().clear_mrkdwn().build()?;
        assert!(wire(edited).get("mrkdwn").is_none());
    }
    let mut invalid = expected.clone();
    invalid["mrkdwn"] = serde_json::json!(0);
    assert!(MessageResponse::try_from(invalid).is_err());
    assert_eq!(
        wire(original.replace_original()),
        expected
            .get("replace_original")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .replace_original(false)
            .build()?;
        assert_eq!(wire(edited)["replace_original"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_replace_original()
            .build()?;
        assert!(wire(edited).get("replace_original").is_none());
    }
    let mut invalid = expected.clone();
    invalid["replace_original"] = serde_json::json!(0);
    assert!(MessageResponse::try_from(invalid).is_err());
    assert_eq!(
        wire(original.response_type()),
        expected
            .get("response_type")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .response_type(ResponseType::Ephemeral)
            .build()?;
        assert_eq!(wire(edited)["response_type"], Value::from("ephemeral"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_response_type()
            .build()?;
        assert!(wire(edited).get("response_type").is_none());
    }
    let mut invalid = expected.clone();
    invalid["response_type"] = Value::from("not-a-variant");
    assert!(MessageResponse::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(MessageResponse::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(MessageResponse::try_from(Value::Null).is_err());
    assert!(MessageResponse::try_from(json!([])).is_err());
    Ok(())
}

#[test]
fn modal_view_editing_and_ingress() -> Result<(), ValidationError> {
    let original = ModalView::builder()
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
    let expected = wire(&original);
    assert_eq!(ModalView::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<ModalView>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(wire(original.title()), expected["title"]);
    {
        let edited = original
            .clone()
            .into_builder()
            .title(PlainText::builder().text("Hello, world!").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["title"],
            json!({"type": "plain_text", "text": "Hello, world!"})
        );
    }
    let mut invalid = expected.clone();
    invalid["title"] = json!([]);
    assert!(ModalView::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("title");
    assert!(ModalView::try_from(missing).is_err());
    assert_eq!(wire(original.blocks()), expected["blocks"]);
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["blocks"],
            json!([
                {
                    "type": "section",
                    "block_id": "1",
                    "text": {
                        "type": "mrkdwn",
                        "text": "first section block"
                    }
                },
                {
                    "type": "divider",
                    "block_id": "2"
                },
                {
                    "type": "section",
                    "block_id": "3",
                    "text": {
                        "type": "mrkdwn",
                        "text": "second section block"
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .block(
            SectionBlock::builder()
                .block_id("1")
                .text(
                    MarkdownText::builder()
                        .text("first section block")
                        .build()?,
                )
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["blocks"].as_array().unwrap().len(),
            expected["blocks"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["blocks"].as_array().unwrap().last().unwrap(),
            &json!({"type": "section", "block_id": "1", "text": {"type": "mrkdwn", "text": "first section block"}})
        );
    }
    let mut invalid = expected.clone();
    invalid["blocks"] = json!({});
    assert!(ModalView::try_from(invalid).is_err());
    let mut missing = expected.clone();
    missing.as_object_mut().unwrap().remove("blocks");
    assert!(ModalView::try_from(missing).is_err());
    assert_eq!(
        wire(original.close()),
        expected.get("close").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .close(PlainText::builder().text("Close button").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["close"],
            json!({"type": "plain_text", "text": "Close button"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_close().build()?;
        assert!(wire(edited).get("close").is_none());
    }
    let mut invalid = expected.clone();
    invalid["close"] = json!([]);
    assert!(ModalView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.submit()),
        expected.get("submit").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .submit(PlainText::builder().text("Submit button").build()?)
            .build()?;
        assert_eq!(
            wire(edited)["submit"],
            json!({"type": "plain_text", "text": "Submit button"})
        );
    }
    {
        let edited = original.clone().into_builder().clear_submit().build()?;
        assert!(wire(edited).get("submit").is_none());
    }
    let mut invalid = expected.clone();
    invalid["submit"] = json!([]);
    assert!(ModalView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.private_metadata()),
        expected
            .get("private_metadata")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .private_metadata("sample")
            .build()?;
        assert_eq!(wire(edited)["private_metadata"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_private_metadata()
            .build()?;
        assert!(wire(edited).get("private_metadata").is_none());
    }
    let mut invalid = expected.clone();
    invalid["private_metadata"] = Value::from(false);
    assert!(ModalView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.callback_id()),
        expected.get("callback_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .callback_id("sample")
            .build()?;
        assert_eq!(wire(edited)["callback_id"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_callback_id()
            .build()?;
        assert!(wire(edited).get("callback_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["callback_id"] = Value::from(false);
    assert!(ModalView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.clear_on_close()),
        expected
            .get("clear_on_close")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_on_close(true)
            .build()?;
        assert_eq!(wire(edited)["clear_on_close"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_clear_on_close()
            .build()?;
        assert!(wire(edited).get("clear_on_close").is_none());
    }
    let mut invalid = expected.clone();
    invalid["clear_on_close"] = serde_json::json!(0);
    assert!(ModalView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.notify_on_close()),
        expected
            .get("notify_on_close")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .notify_on_close(true)
            .build()?;
        assert_eq!(wire(edited)["notify_on_close"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_notify_on_close()
            .build()?;
        assert!(wire(edited).get("notify_on_close").is_none());
    }
    let mut invalid = expected.clone();
    invalid["notify_on_close"] = serde_json::json!(0);
    assert!(ModalView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.external_id()),
        expected.get("external_id").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .external_id("sample")
            .build()?;
        assert_eq!(wire(edited)["external_id"], Value::from("sample"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_external_id()
            .build()?;
        assert!(wire(edited).get("external_id").is_none());
    }
    let mut invalid = expected.clone();
    invalid["external_id"] = Value::from(false);
    assert!(ModalView::try_from(invalid).is_err());
    assert_eq!(
        wire(original.submit_disabled()),
        expected
            .get("submit_disabled")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .submit_disabled(true)
            .build()?;
        assert_eq!(wire(edited)["submit_disabled"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_submit_disabled()
            .build()?;
        assert!(wire(edited).get("submit_disabled").is_none());
    }
    let mut invalid = expected.clone();
    invalid["submit_disabled"] = serde_json::json!(0);
    assert!(ModalView::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(ModalView::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(ModalView::try_from(Value::Null).is_err());
    assert!(ModalView::try_from(json!([])).is_err());
    let mut wrong = expected;
    wrong["type"] = json!("not-a-tag");
    assert!(ModalView::try_from(wrong).is_err());
    Ok(())
}

#[test]
fn webhook_message_editing_and_ingress() -> Result<(), ValidationError> {
    let original = WebhookMessage::builder()
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
        .metadata(match json!({"sender": "Walt"}) {
            Value::Object(map) => map,
            _ => unreachable!(),
        })
        .build()?;
    let expected = wire(&original);
    assert_eq!(WebhookMessage::try_from(expected.clone())?, original);
    assert_eq!(
        serde_json::from_value::<WebhookMessage>(expected.clone()).unwrap(),
        original
    );
    assert_eq!(original.clone().into_builder().build()?, original);
    assert_eq!(wire(original.extensions()), json!({}));
    assert_eq!(
        wire(original.blocks()),
        expected.get("blocks").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["blocks"],
            json!([
                {
                    "type": "section",
                    "block_id": "fake_block_id",
                    "text": {
                        "type": "mrkdwn",
                        "text": "You wouldn't do ol' Hook in now, would you, lad?"
                    }
                },
                {
                    "type": "section",
                    "block_id": "fake_block_id",
                    "text": {
                        "type": "mrkdwn",
                        "text": "Well, all right... if you... say you're a codfish."
                    }
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .block(
            SectionBlock::builder()
                .block_id("fake_block_id")
                .text(
                    MarkdownText::builder()
                        .text("You wouldn't do ol' Hook in now, would you, lad?")
                        .build()?,
                )
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["blocks"].as_array().unwrap().len(),
            expected["blocks"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["blocks"].as_array().unwrap().last().unwrap(),
            &json!({
                "type": "section",
                "block_id": "fake_block_id",
                "text": {
                    "type": "mrkdwn",
                    "text": "You wouldn't do ol' Hook in now, would you, lad?"
                }
            })
        );
    }
    {
        let edited = original.clone().into_builder().clear_blocks().build()?;
        assert!(wire(edited).get("blocks").is_none());
    }
    let mut invalid = expected.clone();
    invalid["blocks"] = json!({});
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.attachments()),
        expected.get("attachments").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
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
            .build()?;
        assert_eq!(
            wire(edited)["attachments"],
            json!([
                {
                    "blocks": [
                        {
                            "type": "section",
                            "block_id": "fake_block_id",
                            "text": {
                                "type": "mrkdwn",
                                "text": "I'M A CODFISH!"
                            }
                        }
                    ]
                }
            ])
        );
    }
    let appended = original
        .clone()
        .into_builder()
        .attachment(
            Attachment::builder()
                .blocks(vec![Block::from(
                    SectionBlock::builder()
                        .block_id("fake_block_id")
                        .text(MarkdownText::builder().text("I'M A CODFISH!").build()?)
                        .build()?,
                )])
                .build()?,
        )
        .build();
    {
        let value = appended?;
        let json = wire(value);
        assert_eq!(
            json["attachments"].as_array().unwrap().len(),
            expected["attachments"].as_array().map_or(0, Vec::len) + 1
        );
        assert_eq!(
            json["attachments"].as_array().unwrap().last().unwrap(),
            &json!({
                "blocks": [
                    {
                        "type": "section",
                        "block_id": "fake_block_id",
                        "text": {
                            "type": "mrkdwn",
                            "text": "I'M A CODFISH!"
                        }
                    }
                ]
            })
        );
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_attachments()
            .build()?;
        assert!(wire(edited).get("attachments").is_none());
    }
    let mut invalid = expected.clone();
    invalid["attachments"] = json!({});
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.text()),
        expected.get("text").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original.clone().into_builder().text("sample").build()?;
        assert_eq!(wire(edited)["text"], Value::from("sample"));
    }
    {
        let edited = original.clone().into_builder().clear_text().build()?;
        assert!(wire(edited).get("text").is_none());
    }
    let mut invalid = expected.clone();
    invalid["text"] = Value::from(false);
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.response_type()),
        expected
            .get("response_type")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .response_type(ResponseType::Ephemeral)
            .build()?;
        assert_eq!(wire(edited)["response_type"], Value::from("ephemeral"));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_response_type()
            .build()?;
        assert!(wire(edited).get("response_type").is_none());
    }
    let mut invalid = expected.clone();
    invalid["response_type"] = Value::from("not-a-variant");
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.replace_original()),
        expected
            .get("replace_original")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .replace_original(true)
            .build()?;
        assert_eq!(wire(edited)["replace_original"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_replace_original()
            .build()?;
        assert!(wire(edited).get("replace_original").is_none());
    }
    let mut invalid = expected.clone();
    invalid["replace_original"] = serde_json::json!(0);
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.delete_original()),
        expected
            .get("delete_original")
            .cloned()
            .unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .delete_original(true)
            .build()?;
        assert_eq!(wire(edited)["delete_original"], Value::from(true));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_delete_original()
            .build()?;
        assert!(wire(edited).get("delete_original").is_none());
    }
    let mut invalid = expected.clone();
    invalid["delete_original"] = serde_json::json!(0);
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.unfurl_links()),
        expected.get("unfurl_links").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .unfurl_links(false)
            .build()?;
        assert_eq!(wire(edited)["unfurl_links"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_unfurl_links()
            .build()?;
        assert!(wire(edited).get("unfurl_links").is_none());
    }
    let mut invalid = expected.clone();
    invalid["unfurl_links"] = serde_json::json!(0);
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.unfurl_media()),
        expected.get("unfurl_media").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .unfurl_media(false)
            .build()?;
        assert_eq!(wire(edited)["unfurl_media"], Value::from(false));
    }
    {
        let edited = original
            .clone()
            .into_builder()
            .clear_unfurl_media()
            .build()?;
        assert!(wire(edited).get("unfurl_media").is_none());
    }
    let mut invalid = expected.clone();
    invalid["unfurl_media"] = serde_json::json!(0);
    assert!(WebhookMessage::try_from(invalid).is_err());
    assert_eq!(
        wire(original.metadata()),
        expected.get("metadata").cloned().unwrap_or(Value::Null)
    );
    {
        let edited = original
            .clone()
            .into_builder()
            .metadata(match json!({"sender": "Walt"}) {
                Value::Object(map) => map,
                _ => unreachable!(),
            })
            .build()?;
        assert_eq!(wire(edited)["metadata"], json!({"sender": "Walt"}));
    }
    {
        let edited = original.clone().into_builder().clear_metadata().build()?;
        assert!(wire(edited).get("metadata").is_none());
    }
    let mut invalid = expected.clone();
    invalid["metadata"] = json!([]);
    assert!(WebhookMessage::try_from(invalid).is_err());
    let extended = original
        .clone()
        .into_builder()
        .extension("future", Value::Null)
        .build()?;
    assert_eq!(extended.extensions()["future"], Value::Null);
    assert_eq!(WebhookMessage::try_from(wire(&extended))?, extended);
    assert!(
        original
            .clone()
            .into_builder()
            .extension("type", "wrong")
            .build()
            .is_err()
    );
    assert!(WebhookMessage::try_from(Value::Null).is_err());
    assert!(WebhookMessage::try_from(json!([])).is_err());
    Ok(())
}
