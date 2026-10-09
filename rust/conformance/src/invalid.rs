// Independent invalid public-API attempts; static role errors use checked ingress.
use serde_json::Value;
use slackblocks::*;

pub struct InvalidCase {
    pub id: &'static str,
    pub build: fn() -> Result<(), ValidationError>,
    pub parse: fn() -> Result<(), ValidationError>,
}
pub const INVALID: &[InvalidCase] = &[
    InvalidCase {
        id: "text-empty",
        build: || {
            let _ = PlainText::builder().text("").build()?;
            Ok(())
        },
        parse: || {
            PlainText::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plain_text")),
                    ("text".into(), Value::from("")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "text-too-long",
        build: || {
            let _ = PlainText::builder().text("x".repeat(3001)).build()?;
            Ok(())
        },
        parse: || {
            PlainText::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plain_text")),
                    ("text".into(), Value::from("x".repeat(3001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "button-action-id-too-long",
        build: || {
            let _ = ButtonElement::builder()
                .text(PlainText::builder().text("A").build()?)
                .action_id("x".repeat(256))
                .build()?;
            Ok(())
        },
        parse: || {
            ButtonElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("button")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("A")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("action_id".into(), Value::from("x".repeat(256))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "button-text-too-long",
        build: || {
            let _ = ButtonElement::builder().text(PlainText::builder().text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).action_id("a").build()?;
            Ok(())
        },
        parse: || {
            ButtonElement::try_from(Value::Object([("type".into(),Value::from("button")),("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())),("action_id".into(),Value::from("a"))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "button-url-too-long",
        build: || {
            let _ = ButtonElement::builder()
                .text(PlainText::builder().text("A").build()?)
                .action_id("a")
                .url("x".repeat(3001))
                .build()?;
            Ok(())
        },
        parse: || {
            ButtonElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("button")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("A")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("action_id".into(), Value::from("a")),
                    ("url".into(), Value::from("x".repeat(3001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "button-value-too-long",
        build: || {
            let _ = ButtonElement::builder()
                .text(PlainText::builder().text("A").build()?)
                .action_id("a")
                .value("x".repeat(2001))
                .build()?;
            Ok(())
        },
        parse: || {
            ButtonElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("button")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("A")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("action_id".into(), Value::from("a")),
                    ("value".into(), Value::from("x".repeat(2001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "confirmation-title-too-long",
        build: || {
            let _ = ConfirmationDialogue::builder()
                .title(PlainText::builder().text("x".repeat(101)).build()?)
                .text(MarkdownText::builder().text("Text").build()?)
                .confirm(PlainText::builder().text("Yes").build()?)
                .deny(PlainText::builder().text("No").build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            ConfirmationDialogue::try_from(Value::Object(
                [
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(101))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("mrkdwn")),
                                ("text".into(), Value::from("Text")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "confirm".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Yes")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "deny".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("No")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "confirmation-text-too-long",
        build: || {
            let _ = ConfirmationDialogue::builder()
                .title(PlainText::builder().text("Title").build()?)
                .text(MarkdownText::builder().text("x".repeat(301)).build()?)
                .confirm(PlainText::builder().text("Yes").build()?)
                .deny(PlainText::builder().text("No").build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            ConfirmationDialogue::try_from(Value::Object(
                [
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("mrkdwn")),
                                ("text".into(), Value::from("x".repeat(301))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "confirm".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Yes")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "deny".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("No")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "confirmation-confirm-too-long",
        build: || {
            let _ = ConfirmationDialogue::builder()
                .title(PlainText::builder().text("Title").build()?)
                .text(MarkdownText::builder().text("Text").build()?)
                .confirm(
                    PlainText::builder()
                        .text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .build()?,
                )
                .deny(PlainText::builder().text("No").build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            ConfirmationDialogue::try_from(Value::Object(
                [
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("mrkdwn")),
                                ("text".into(), Value::from("Text")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "confirm".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                (
                                    "text".into(),
                                    Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "deny".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("No")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "confirmation-deny-too-long",
        build: || {
            let _ = ConfirmationDialogue::builder()
                .title(PlainText::builder().text("Title").build()?)
                .text(MarkdownText::builder().text("Text").build()?)
                .confirm(PlainText::builder().text("Yes").build()?)
                .deny(
                    PlainText::builder()
                        .text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .build()?,
                )
                .build()?;
            Ok(())
        },
        parse: || {
            ConfirmationDialogue::try_from(Value::Object(
                [
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("mrkdwn")),
                                ("text".into(), Value::from("Text")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "confirm".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Yes")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "deny".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                (
                                    "text".into(),
                                    Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "option-text-too-long",
        build: || {
            let _ = SelectOption::builder().text(PlainText::builder().text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).value("a").build()?;
            Ok(())
        },
        parse: || {
            SelectOption::try_from(Value::Object([("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())),("value".into(),Value::from("a"))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "option-value-too-long",
        build: || {
            let _ = SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("x".repeat(151))
                .build()?;
            Ok(())
        },
        parse: || {
            SelectOption::try_from(Value::Object(
                [
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("A")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("value".into(), Value::from("x".repeat(151))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "option-url-too-long",
        build: || {
            let _ = SelectOption::builder()
                .text(PlainText::builder().text("A").build()?)
                .value("a")
                .url("🙂".repeat(3001))
                .build()?;
            Ok(())
        },
        parse: || {
            SelectOption::try_from(Value::Object(
                [
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("A")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("value".into(), Value::from("a")),
                    ("url".into(), Value::from("🙂".repeat(3001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "option-description-too-long",
        build: || {
            let _ = SelectOption::builder().text(PlainText::builder().text("A").build()?).value("a").description(PlainText::builder().text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).build()?;
            Ok(())
        },
        parse: || {
            SelectOption::try_from(Value::Object([("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("A"))].into_iter().collect())),("value".into(),Value::from("a")),("description".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect()))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "option-group-label-too-long",
        build: || {
            let _ = SelectOptionGroup::builder().label(PlainText::builder().text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).options(vec![SelectOption::builder().text(PlainText::builder().text("A").build()?).value("a").build()?]).build()?;
            Ok(())
        },
        parse: || {
            SelectOptionGroup::try_from(Value::Object([("label".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())),("options".into(),Value::Array(vec![Value::Object([("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("A"))].into_iter().collect())),("value".into(),Value::from("a"))].into_iter().collect())]))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "option-group-empty",
        build: || {
            let _ = SelectOptionGroup::builder()
                .label(PlainText::builder().text("Group").build()?)
                .options(Vec::<SelectOption>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            SelectOptionGroup::try_from(Value::Object(
                [
                    (
                        "label".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Group")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("options".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "option-group-too-many-options",
        build: || {
            let _ = SelectOptionGroup::builder()
                .label(PlainText::builder().text("Group").build()?)
                .options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("a")
                        .build()?;
                    101
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            SelectOptionGroup::try_from(Value::Object(
                [
                    (
                        "label".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Group")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "options".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("A"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("value".into(), Value::from("a"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            101
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "select-placeholder-too-long",
        build: || {
            let _ = StaticSelectElement::builder()
                .action_id("a")
                .options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("a")
                        .build()?,
                ])
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            StaticSelectElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("static_select")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "options".into(),
                        Value::Array(vec![Value::Object(
                            [
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("A")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                ("value".into(), Value::from("a")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plain-text-input-placeholder-too-long",
        build: || {
            let _ = PlainTextInputElement::builder()
                .action_id("a")
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            PlainTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plain_text_input")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "email-input-placeholder-too-long",
        build: || {
            let _ = EmailInputElement::builder()
                .action_id("a")
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            EmailInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("email_text_input")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "url-input-placeholder-too-long",
        build: || {
            let _ = UrlInputElement::builder()
                .action_id("a")
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            UrlInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("url_text_input")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "number-input-placeholder-too-long",
        build: || {
            let _ = NumberInputElement::builder()
                .action_id("a")
                .is_decimal_allowed(false)
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            NumberInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("number_input")),
                    ("action_id".into(), Value::from("a")),
                    ("is_decimal_allowed".into(), Value::from(false)),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "date-picker-placeholder-too-long",
        build: || {
            let _ = DatePickerElement::builder()
                .action_id("a")
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            DatePickerElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("datepicker")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "time-picker-placeholder-too-long",
        build: || {
            let _ = TimePickerElement::builder()
                .action_id("a")
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            TimePickerElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("timepicker")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-input-placeholder-too-long",
        build: || {
            let _ = RichTextInputElement::builder()
                .action_id("a")
                .placeholder(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_input")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "placeholder".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "select-too-many-options",
        build: || {
            let _ = StaticSelectElement::builder()
                .action_id("a")
                .options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("a")
                        .build()?;
                    101
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            StaticSelectElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("static_select")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "options".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("A"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("value".into(), Value::from("a"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            101
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "select-too-many-option-groups",
        build: || {
            let _ = StaticSelectElement::builder()
                .action_id("a")
                .option_groups(vec![
                    SelectOptionGroup::builder()
                        .label(PlainText::builder().text("Group").build()?)
                        .options(vec![
                            SelectOption::builder()
                                .text(PlainText::builder().text("A").build()?)
                                .value("a")
                                .build()?
                        ])
                        .build()?;
                    101
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            StaticSelectElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("static_select")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "option_groups".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    (
                                        "label".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("Group"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    (
                                        "options".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                (
                                                    "text".into(),
                                                    Value::Object(
                                                        [
                                                            (
                                                                "type".into(),
                                                                Value::from("plain_text")
                                                            ),
                                                            ("text".into(), Value::from("A"))
                                                        ]
                                                        .into_iter()
                                                        .collect()
                                                    )
                                                ),
                                                ("value".into(), Value::from("a"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )])
                                    )
                                ]
                                .into_iter()
                                .collect()
                            );
                            101
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "overflow-empty",
        build: || {
            let _ = OverflowElement::builder()
                .action_id("a")
                .options(Vec::<SelectOption>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            OverflowElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("overflow")),
                    ("action_id".into(), Value::from("a")),
                    ("options".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "overflow-too-many-options",
        build: || {
            let _ = OverflowElement::builder()
                .action_id("a")
                .options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("a")
                        .build()?;
                    6
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            OverflowElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("overflow")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "options".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("A"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("value".into(), Value::from("a"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            6
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "checkboxes-empty",
        build: || {
            let _ = CheckboxesElement::builder()
                .action_id("a")
                .options(Vec::<SelectOption>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            CheckboxesElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("checkboxes")),
                    ("action_id".into(), Value::from("a")),
                    ("options".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "checkboxes-too-many-options",
        build: || {
            let _ = CheckboxesElement::builder()
                .action_id("a")
                .options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("a")
                        .build()?;
                    11
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            CheckboxesElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("checkboxes")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "options".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("A"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("value".into(), Value::from("a"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            11
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "radio-buttons-empty",
        build: || {
            let _ = RadioButtonsElement::builder()
                .action_id("a")
                .options(Vec::<SelectOption>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            RadioButtonsElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("radio_buttons")),
                    ("action_id".into(), Value::from("a")),
                    ("options".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "radio-buttons-too-many-options",
        build: || {
            let _ = RadioButtonsElement::builder()
                .action_id("a")
                .options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("a")
                        .build()?;
                    11
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            RadioButtonsElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("radio_buttons")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "options".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("A"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("value".into(), Value::from("a"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            11
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "table-too-many-rows",
        build: || {
            let _ = TableBlock::builder()
                .rows(vec![
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                    vec![TableCell::from(RawText::builder().text("A").build()?)],
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            TableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("A"))
                                ]
                                .into_iter()
                                .collect()
                            )]);
                            101
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "table-too-many-columns",
        build: || {
            let _ = TableBlock::builder()
                .rows(vec![vec![
                    TableCell::from(
                        RawText::builder().text("A").build()?
                    );
                    21
                ]])
                .build()?;
            Ok(())
        },
        parse: || {
            TableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("table")),
                    (
                        "rows".into(),
                        Value::Array(vec![Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("A"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            21
                        ])]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "file-input-max-files-too-small",
        build: || {
            let _ = FileInputElement::builder()
                .action_id("a")
                .max_files(0_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            FileInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("file_input")),
                    ("action_id".into(), Value::from("a")),
                    ("max_files".into(), serde_json::json!(0)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "file-input-max-files-too-large",
        build: || {
            let _ = FileInputElement::builder()
                .action_id("a")
                .max_files(11_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            FileInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("file_input")),
                    ("action_id".into(), Value::from("a")),
                    ("max_files".into(), serde_json::json!(11)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "dispatch-action-no-triggers",
        build: || {
            let _ = DispatchActionConfiguration::builder()
                .trigger_actions_on(Vec::<String>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            DispatchActionConfiguration::try_from(Value::Object(
                [("trigger_actions_on".into(), Value::Array(vec![]))]
                    .into_iter()
                    .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "dispatch-action-missing-triggers",
        build: || {
            let _ = DispatchActionConfiguration::builder().build()?;
            Ok(())
        },
        parse: || {
            DispatchActionConfiguration::try_from(Value::Object([].into_iter().collect()))
                .map(|_| ())
        },
    },
    InvalidCase {
        id: "dispatch-action-too-many-triggers",
        build: || {
            let _ = DispatchActionConfiguration::builder()
                .trigger_actions_on(vec![
                    String::from("on_enter_pressed"),
                    String::from("on_character_entered"),
                    String::from("on_enter_pressed"),
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            DispatchActionConfiguration::try_from(Value::Object(
                [(
                    "trigger_actions_on".into(),
                    Value::Array(vec![
                        Value::from("on_enter_pressed"),
                        Value::from("on_character_entered"),
                        Value::from("on_enter_pressed"),
                    ]),
                )]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plain-text-input-max-length-too-large",
        build: || {
            let _ = PlainTextInputElement::builder()
                .action_id("a")
                .max_length(3001_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            PlainTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plain_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("max_length".into(), serde_json::json!(3001)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "actions-too-many-elements",
        build: || {
            let _ = ActionsBlock::builder()
                .elements(vec![
                    Element::from(
                        ButtonElement::builder()
                            .text(PlainText::builder().text("A").build()?)
                            .action_id("a")
                            .build()?
                    );
                    26
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            ActionsBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("actions")),
                    (
                        "elements".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("button")),
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("A"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("action_id".into(), Value::from("a"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            26
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "context-too-many-elements",
        build: || {
            let _ = ContextBlock::builder()
                .elements(vec![
                    ContextElement::from(
                        MarkdownText::builder().text("A").build()?
                    );
                    11
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            ContextBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("context")),
                    (
                        "elements".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("mrkdwn")),
                                    ("text".into(), Value::from("A"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            11
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "header-text-too-long",
        build: || {
            let _ = HeaderBlock::builder()
                .text(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            HeaderBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("header")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-url-too-long",
        build: || {
            let _ = ImageBlock::builder()
                .image_url("x".repeat(3001))
                .alt_text("Alt")
                .build()?;
            Ok(())
        },
        parse: || {
            ImageBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    ("image_url".into(), Value::from("x".repeat(3001))),
                    ("alt_text".into(), Value::from("Alt")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-alt-text-too-long",
        build: || {
            let _ = ImageBlock::builder()
                .image_url("https://example.com/image.png")
                .alt_text("x".repeat(2001))
                .build()?;
            Ok(())
        },
        parse: || {
            ImageBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    (
                        "image_url".into(),
                        Value::from("https://example.com/image.png"),
                    ),
                    ("alt_text".into(), Value::from("x".repeat(2001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "input-label-too-long",
        build: || {
            let _ = InputBlock::builder()
                .label(PlainText::builder().text("x".repeat(2001)).build()?)
                .element(PlainTextInputElement::builder().action_id("a").build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            InputBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("input")),
                    (
                        "label".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(2001))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "element".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text_input")),
                                ("action_id".into(), Value::from("a")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "input-hint-too-long",
        build: || {
            let _ = InputBlock::builder()
                .label(PlainText::builder().text("Label").build()?)
                .hint(PlainText::builder().text("x".repeat(2001)).build()?)
                .element(PlainTextInputElement::builder().action_id("a").build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            InputBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("input")),
                    (
                        "label".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Label")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "hint".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(2001))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "element".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text_input")),
                                ("action_id".into(), Value::from("a")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "markdown-too-long",
        build: || {
            let _ = MarkdownBlock::builder().text("x".repeat(12001)).build()?;
            Ok(())
        },
        parse: || {
            MarkdownBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("markdown")),
                    ("text".into(), Value::from("x".repeat(12001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "section-text-too-long",
        build: || {
            let _ = SectionBlock::builder()
                .text(MarkdownText::builder().text("x".repeat(3001)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            SectionBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("section")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("mrkdwn")),
                                ("text".into(), Value::from("x".repeat(3001))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "section-too-many-fields",
        build: || {
            let _ = SectionBlock::builder()
                .fields(vec![
                    TextInput::from(
                        MarkdownText::builder().text("x").build()?
                    );
                    11
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            SectionBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("section")),
                    (
                        "fields".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("mrkdwn")),
                                    ("text".into(), Value::from("x"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            11
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "section-field-too-long",
        build: || {
            let _ = SectionBlock::builder()
                .fields(vec![TextInput::from(
                    MarkdownText::builder().text("x".repeat(2001)).build()?,
                )])
                .build()?;
            Ok(())
        },
        parse: || {
            SectionBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("section")),
                    (
                        "fields".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("mrkdwn")),
                                ("text".into(), Value::from("x".repeat(2001))),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-alt-text-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("x".repeat(2001))
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/video.mp4")
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("x".repeat(2001))),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-title-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("x".repeat(201)).build()?)
                .video_url("https://example.com/video.mp4")
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(201))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-author-name-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/video.mp4")
                .author_name("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                    (
                        "author_name".into(),
                        Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-description-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/video.mp4")
                .description(PlainText::builder().text("x".repeat(201)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                    (
                        "description".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(201))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-provider-name-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/video.mp4")
                .provider_name("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                    (
                        "provider_name".into(),
                        Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "message-channel-empty",
        build: || {
            let _ = MessagePayload::builder("")
                .clear_mrkdwn()
                .clear_text()
                .build()?;
            Ok(())
        },
        parse: || {
            MessagePayload::try_from(Value::Object(
                [("channel".into(), Value::from(""))].into_iter().collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "message-too-many-blocks",
        build: || {
            let _ = MessagePayload::builder("C123")
                .blocks(vec![Block::from(DividerBlock::builder().build()?); 51])
                .clear_mrkdwn()
                .clear_text()
                .build()?;
            Ok(())
        },
        parse: || {
            MessagePayload::try_from(Value::Object(
                [
                    ("channel".into(), Value::from("C123")),
                    (
                        "blocks".into(),
                        Value::Array(vec![
                            Value::Object(
                                [("type".into(), Value::from("divider"))]
                                    .into_iter()
                                    .collect()
                            );
                            51
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "message-too-many-attachments",
        build: || {
            let _ = MessagePayload::builder("C123")
                .attachments(vec![
                    Attachment::builder()
                        .blocks(vec![Block::from(DividerBlock::builder().build()?)])
                        .build()?;
                    101
                ])
                .clear_mrkdwn()
                .clear_text()
                .build()?;
            Ok(())
        },
        parse: || {
            MessagePayload::try_from(Value::Object(
                [
                    ("channel".into(), Value::from("C123")),
                    (
                        "attachments".into(),
                        Value::Array(vec![
                            Value::Object(
                                [(
                                    "blocks".into(),
                                    Value::Array(vec![Value::Object(
                                        [("type".into(), Value::from("divider"))]
                                            .into_iter()
                                            .collect()
                                    )])
                                )]
                                .into_iter()
                                .collect()
                            );
                            101
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "message-invalid-block-surface",
        build: || {
            let _ = MessagePayload::builder("C123")
                .blocks(vec![Block::from(
                    AlertBlock::builder()
                        .text(PlainText::builder().text("Modal only").build()?)
                        .build()?,
                )])
                .clear_mrkdwn()
                .clear_text()
                .build()?;
            Ok(())
        },
        parse: || {
            MessagePayload::try_from(Value::Object(
                [
                    ("channel".into(), Value::from("C123")),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("alert")),
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("Modal only")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "modal-invalid-block-surface",
        build: || {
            let _ = ModalView::builder()
                .title(PlainText::builder().text("Invalid").build()?)
                .blocks(vec![Block::from(
                    MarkdownBlock::builder().text("Message only").build()?,
                )])
                .build()?;
            Ok(())
        },
        parse: || {
            ModalView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("modal")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Invalid")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("markdown")),
                                ("text".into(), Value::from("Message only")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "home-invalid-block-surface",
        build: || {
            let _ = HomeTabView::builder()
                .blocks(vec![Block::from(
                    AlertBlock::builder()
                        .text(PlainText::builder().text("Modal only").build()?)
                        .build()?,
                )])
                .build()?;
            Ok(())
        },
        parse: || {
            HomeTabView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("home")),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("alert")),
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("Modal only")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "modal-input-requires-submit",
        build: || {
            let _ = ModalView::builder()
                .title(PlainText::builder().text("Missing submit").build()?)
                .blocks(vec![Block::from(
                    InputBlock::builder()
                        .label(PlainText::builder().text("Name").build()?)
                        .element(PlainTextInputElement::builder().action_id("a").build()?)
                        .build()?,
                )])
                .build()?;
            Ok(())
        },
        parse: || {
            ModalView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("modal")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Missing submit")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("input")),
                                (
                                    "label".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("Name")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                (
                                    "element".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text_input")),
                                            ("action_id".into(), Value::from("a")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "view-too-many-blocks",
        build: || {
            let _ = HomeTabView::builder()
                .blocks(vec![Block::from(DividerBlock::builder().build()?); 101])
                .build()?;
            Ok(())
        },
        parse: || {
            HomeTabView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("home")),
                    (
                        "blocks".into(),
                        Value::Array(vec![
                            Value::Object(
                                [("type".into(), Value::from("divider"))]
                                    .into_iter()
                                    .collect()
                            );
                            101
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "view-private-metadata-too-long",
        build: || {
            let _ = HomeTabView::builder()
                .blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .private_metadata("x".repeat(3001))
                .build()?;
            Ok(())
        },
        parse: || {
            HomeTabView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("home")),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                    ("private_metadata".into(), Value::from("x".repeat(3001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "view-callback-id-too-long",
        build: || {
            let _ = HomeTabView::builder()
                .blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .callback_id("x".repeat(256))
                .build()?;
            Ok(())
        },
        parse: || {
            HomeTabView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("home")),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                    ("callback_id".into(), Value::from("x".repeat(256))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "view-title-too-long",
        build: || {
            let _ = ModalView::builder()
                .title(
                    PlainText::builder()
                        .text("xxxxxxxxxxxxxxxxxxxxxxxxx")
                        .build()?,
                )
                .blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .build()?;
            Ok(())
        },
        parse: || {
            ModalView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("modal")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("xxxxxxxxxxxxxxxxxxxxxxxxx")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "view-close-too-long",
        build: || {
            let _ = ModalView::builder()
                .title(PlainText::builder().text("Title").build()?)
                .close(
                    PlainText::builder()
                        .text("xxxxxxxxxxxxxxxxxxxxxxxxx")
                        .build()?,
                )
                .blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .build()?;
            Ok(())
        },
        parse: || {
            ModalView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("modal")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "close".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("xxxxxxxxxxxxxxxxxxxxxxxxx")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "view-submit-too-long",
        build: || {
            let _ = ModalView::builder()
                .title(PlainText::builder().text("Title").build()?)
                .submit(
                    PlainText::builder()
                        .text("xxxxxxxxxxxxxxxxxxxxxxxxx")
                        .build()?,
                )
                .blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .build()?;
            Ok(())
        },
        parse: || {
            ModalView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("modal")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "submit".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("xxxxxxxxxxxxxxxxxxxxxxxxx")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "section-missing-content",
        build: || {
            let _ = SectionBlock::builder().build()?;
            Ok(())
        },
        parse: || {
            SectionBlock::try_from(Value::Object(
                [("type".into(), Value::from("section"))]
                    .into_iter()
                    .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "section-empty-fields",
        build: || {
            let _ = SectionBlock::builder()
                .fields(Vec::<TextInput>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            SectionBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("section")),
                    ("fields".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "static-select-options-and-groups",
        build: || {
            let _ = StaticSelectElement::builder()
                .action_id("a")
                .options(vec![
                    SelectOption::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .value("a")
                        .build()?,
                ])
                .option_groups(vec![
                    SelectOptionGroup::builder()
                        .label(PlainText::builder().text("A").build()?)
                        .options(vec![
                            SelectOption::builder()
                                .text(PlainText::builder().text("B").build()?)
                                .value("b")
                                .build()?,
                        ])
                        .build()?,
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            StaticSelectElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("static_select")),
                    ("action_id".into(), Value::from("a")),
                    (
                        "options".into(),
                        Value::Array(vec![Value::Object(
                            [
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("A")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                ("value".into(), Value::from("a")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    (
                        "option_groups".into(),
                        Value::Array(vec![Value::Object(
                            [
                                (
                                    "label".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("A")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                (
                                    "options".into(),
                                    Value::Array(vec![Value::Object(
                                        [
                                            (
                                                "text".into(),
                                                Value::Object(
                                                    [
                                                        ("type".into(), Value::from("plain_text")),
                                                        ("text".into(), Value::from("B")),
                                                    ]
                                                    .into_iter()
                                                    .collect(),
                                                ),
                                            ),
                                            ("value".into(), Value::from("b")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    )]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-url-and-slack-file",
        build: || {
            let _ = ImageElement::builder()
                .alt_text("image")
                .image_url("https://example.com/image.png")
                .slack_file(SlackFile::builder().id("F0123ABC456").build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            ImageElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    ("alt_text".into(), Value::from("image")),
                    (
                        "image_url".into(),
                        Value::from("https://example.com/image.png"),
                    ),
                    (
                        "slack_file".into(),
                        Value::Object(
                            [("id".into(), Value::from("F0123ABC456"))]
                                .into_iter()
                                .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "number-input-inverted-range",
        build: || {
            let _ = NumberInputElement::builder()
                .action_id("a")
                .is_decimal_allowed(true)
                .min_value(2.0_f64)
                .max_value(1.0_f64)
                .build()?;
            Ok(())
        },
        parse: || {
            NumberInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("number_input")),
                    ("action_id".into(), Value::from("a")),
                    ("is_decimal_allowed".into(), Value::from(true)),
                    ("min_value".into(), serde_json::json!(2)),
                    ("max_value".into(), serde_json::json!(1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "context-invalid-element",
        build: || {
            ContextBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("context")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
        parse: || {
            ContextBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("context")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "input-invalid-element",
        build: || {
            InputBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("input")),
                    (
                        "label".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Label")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "element".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("button")),
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("A")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                ("action_id".into(), Value::from("a")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
        parse: || {
            InputBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("input")),
                    (
                        "label".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Label")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "element".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("button")),
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("A")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                ("action_id".into(), Value::from("a")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "block-id-too-long",
        build: || {
            let _ = DividerBlock::builder().block_id("x".repeat(256)).build()?;
            Ok(())
        },
        parse: || {
            DividerBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("divider")),
                    ("block_id".into(), Value::from("x".repeat(256))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "button-accessibility-label-too-long",
        build: || {
            let _ = ButtonElement::builder()
                .text(PlainText::builder().text("A").build()?)
                .action_id("a")
                .accessibility_label(
                    "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                )
                .build()?;
            Ok(())
        },
        parse: || {
            ButtonElement::try_from(Value::Object([("type".into(),Value::from("button")),("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("A"))].into_iter().collect())),("action_id".into(),Value::from("a")),("accessibility_label".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "workflow-button-text-too-long",
        build: || {
            let _ = WorkflowButtonElement::builder().text(PlainText::builder().text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).workflow(Workflow::builder().trigger(Trigger::builder().url("https://slack.com/shortcuts/Ft0/abc").build()?).build()?).action_id("a").build()?;
            Ok(())
        },
        parse: || {
            WorkflowButtonElement::try_from(Value::Object([("type".into(),Value::from("workflow_button")),("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())),("workflow".into(),Value::Object([("trigger".into(),Value::Object([("url".into(),Value::from("https://slack.com/shortcuts/Ft0/abc"))].into_iter().collect()))].into_iter().collect())),("action_id".into(),Value::from("a"))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "workflow-button-accessibility-label-too-long",
        build: || {
            let _ = WorkflowButtonElement::builder()
                .text(PlainText::builder().text("Run").build()?)
                .workflow(
                    Workflow::builder()
                        .trigger(
                            Trigger::builder()
                                .url("https://slack.com/shortcuts/Ft0/abc")
                                .build()?,
                        )
                        .build()?,
                )
                .action_id("a")
                .accessibility_label(
                    "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                )
                .build()?;
            Ok(())
        },
        parse: || {
            WorkflowButtonElement::try_from(Value::Object([("type".into(),Value::from("workflow_button")),("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("Run"))].into_iter().collect())),("workflow".into(),Value::Object([("trigger".into(),Value::Object([("url".into(),Value::from("https://slack.com/shortcuts/Ft0/abc"))].into_iter().collect()))].into_iter().collect())),("action_id".into(),Value::from("a")),("accessibility_label".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "alert-text-too-long",
        build: || {
            let _ = AlertBlock::builder()
                .text(PlainText::builder().text("x".repeat(201)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            AlertBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("alert")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(201))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "card-title-too-long",
        build: || {
            let _ = CardBlock::builder()
                .title(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            CardBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("card")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "card-subtitle-too-long",
        build: || {
            let _ = CardBlock::builder()
                .title(PlainText::builder().text("Card").build()?)
                .subtitle(PlainText::builder().text("x".repeat(151)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            CardBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("card")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Card")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "subtitle".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "card-body-too-long",
        build: || {
            let _ = CardBlock::builder()
                .body(PlainText::builder().text("x".repeat(201)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            CardBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("card")),
                    (
                        "body".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(201))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "card-too-many-actions",
        build: || {
            let _ = CardBlock::builder()
                .title(PlainText::builder().text("Card").build()?)
                .actions(vec![
                    ButtonElement::builder()
                        .text(PlainText::builder().text("A").build()?)
                        .action_id("a")
                        .build()?;
                    4
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            CardBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("card")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Card")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "actions".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("button")),
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("A"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("action_id".into(), Value::from("a"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            4
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "card-subtext-too-long",
        build: || {
            let _ = CardBlock::builder()
                .title(PlainText::builder().text("Card").build()?)
                .subtext(PlainText::builder().text("x".repeat(201)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            CardBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("card")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Card")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "subtext".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(201))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "carousel-empty",
        build: || {
            let _ = CarouselBlock::builder()
                .elements(Vec::<CardBlock>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            CarouselBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("carousel")),
                    ("elements".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "carousel-too-many-cards",
        build: || {
            let _ = CarouselBlock::builder()
                .elements(vec![
                    CardBlock::builder()
                        .title(PlainText::builder().text("Card").build()?)
                        .build()?;
                    11
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            CarouselBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("carousel")),
                    (
                        "elements".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("card")),
                                    (
                                        "title".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("Card"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    )
                                ]
                                .into_iter()
                                .collect()
                            );
                            11
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "container-title-too-long",
        build: || {
            let _ = ContainerBlock::builder()
                .title(PlainText::builder().text("x".repeat(151)).build()?)
                .child_blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .build()?;
            Ok(())
        },
        parse: || {
            ContainerBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("container")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "child_blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "container-subtitle-too-long",
        build: || {
            let _ = ContainerBlock::builder()
                .title(PlainText::builder().text("Container").build()?)
                .subtitle(PlainText::builder().text("x".repeat(151)).build()?)
                .child_blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .build()?;
            Ok(())
        },
        parse: || {
            ContainerBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("container")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Container")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "subtitle".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(151))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "child_blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "container-too-many-child-blocks",
        build: || {
            let _ = ContainerBlock::builder()
                .title(PlainText::builder().text("Container").build()?)
                .child_blocks(vec![Block::from(DividerBlock::builder().build()?); 11])
                .build()?;
            Ok(())
        },
        parse: || {
            ContainerBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("container")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Container")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "child_blocks".into(),
                        Value::Array(vec![
                            Value::Object(
                                [("type".into(), Value::from("divider"))]
                                    .into_iter()
                                    .collect()
                            );
                            11
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "context-actions-too-many-elements",
        build: || {
            let _ = ContextActionsBlock::builder()
                .elements(vec![
                    ContextActionsElement::from(
                        IconButtonElement::builder()
                            .text(PlainText::builder().text("Delete").build()?)
                            .icon(IconButtonIcon::Trash)
                            .build()?
                    );
                    6
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            ContextActionsBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("context_actions")),
                    (
                        "elements".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("icon_button")),
                                    (
                                        "text".into(),
                                        Value::Object(
                                            [
                                                ("type".into(), Value::from("plain_text")),
                                                ("text".into(), Value::from("Delete"))
                                            ]
                                            .into_iter()
                                            .collect()
                                        )
                                    ),
                                    ("icon".into(), Value::from("trash"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            6
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "feedback-button-text-too-long",
        build: || {
            let _ = FeedbackButtonsElement::builder().positive_button(FeedbackButton::builder().text(PlainText::builder().text("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).value("good").build()?).negative_button(FeedbackButton::builder().text(PlainText::builder().text("Bad").build()?).value("bad").build()?).build()?;
            Ok(())
        },
        parse: || {
            FeedbackButtonsElement::try_from(Value::Object([("type".into(),Value::from("feedback_buttons")),("positive_button".into(),Value::Object([("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())),("value".into(),Value::from("good"))].into_iter().collect())),("negative_button".into(),Value::Object([("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("Bad"))].into_iter().collect())),("value".into(),Value::from("bad"))].into_iter().collect()))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "feedback-button-value-too-long",
        build: || {
            let _ = FeedbackButtonsElement::builder()
                .positive_button(
                    FeedbackButton::builder()
                        .text(PlainText::builder().text("Good").build()?)
                        .value("x".repeat(2001))
                        .build()?,
                )
                .negative_button(
                    FeedbackButton::builder()
                        .text(PlainText::builder().text("Bad").build()?)
                        .value("bad")
                        .build()?,
                )
                .build()?;
            Ok(())
        },
        parse: || {
            FeedbackButtonsElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("feedback_buttons")),
                    (
                        "positive_button".into(),
                        Value::Object(
                            [
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("Good")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                ("value".into(), Value::from("x".repeat(2001))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "negative_button".into(),
                        Value::Object(
                            [
                                (
                                    "text".into(),
                                    Value::Object(
                                        [
                                            ("type".into(), Value::from("plain_text")),
                                            ("text".into(), Value::from("Bad")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                ("value".into(), Value::from("bad")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "feedback-button-accessibility-label-too-long",
        build: || {
            let _ = FeedbackButtonsElement::builder().positive_button(FeedbackButton::builder().text(PlainText::builder().text("Good").build()?).value("good").accessibility_label("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").build()?).negative_button(FeedbackButton::builder().text(PlainText::builder().text("Bad").build()?).value("bad").build()?).build()?;
            Ok(())
        },
        parse: || {
            FeedbackButtonsElement::try_from(Value::Object([("type".into(),Value::from("feedback_buttons")),("positive_button".into(),Value::Object([("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("Good"))].into_iter().collect())),("value".into(),Value::from("good")),("accessibility_label".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())),("negative_button".into(),Value::Object([("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("Bad"))].into_iter().collect())),("value".into(),Value::from("bad"))].into_iter().collect()))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "icon-button-too-many-visible-users",
        build: || {
            let _ = IconButtonElement::builder()
                .text(PlainText::builder().text("Delete").build()?)
                .icon(IconButtonIcon::Trash)
                .visible_to_user_ids(vec![
                    String::from("U0"),
                    String::from("U1"),
                    String::from("U2"),
                    String::from("U3"),
                    String::from("U4"),
                    String::from("U5"),
                    String::from("U6"),
                    String::from("U7"),
                    String::from("U8"),
                    String::from("U9"),
                    String::from("U10"),
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            IconButtonElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("icon_button")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Delete")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("icon".into(), Value::from("trash")),
                    (
                        "visible_to_user_ids".into(),
                        Value::Array(vec![
                            Value::from("U0"),
                            Value::from("U1"),
                            Value::from("U2"),
                            Value::from("U3"),
                            Value::from("U4"),
                            Value::from("U5"),
                            Value::from("U6"),
                            Value::from("U7"),
                            Value::from("U8"),
                            Value::from("U9"),
                            Value::from("U10"),
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "icon-button-value-too-long",
        build: || {
            let _ = IconButtonElement::builder()
                .text(PlainText::builder().text("Delete").build()?)
                .icon(IconButtonIcon::Trash)
                .value("x".repeat(2001))
                .build()?;
            Ok(())
        },
        parse: || {
            IconButtonElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("icon_button")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Delete")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("icon".into(), Value::from("trash")),
                    ("value".into(), Value::from("x".repeat(2001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "icon-button-accessibility-label-too-long",
        build: || {
            let _ = IconButtonElement::builder()
                .text(PlainText::builder().text("Delete").build()?)
                .icon(IconButtonIcon::Trash)
                .accessibility_label(
                    "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                )
                .build()?;
            Ok(())
        },
        parse: || {
            IconButtonElement::try_from(Value::Object([("type".into(),Value::from("icon_button")),("text".into(),Value::Object([("type".into(),Value::from("plain_text")),("text".into(),Value::from("Delete"))].into_iter().collect())),("icon".into(),Value::from("trash")),("accessibility_label".into(),Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"))].into_iter().collect())).map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-too-few-rows",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![vec![DataTableCell::from(
                    RawText::builder().text("Name").build()?,
                )]])
                .caption("Names")
                .clear_page_size()
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("raw_text")),
                                ("text".into(), Value::from("Name")),
                            ]
                            .into_iter()
                            .collect(),
                        )])]),
                    ),
                    ("caption".into(), Value::from("Names")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-too-many-rows",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?)],
                ])
                .caption("Names")
                .clear_page_size()
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("A"))
                                ]
                                .into_iter()
                                .collect()
                            )]);
                            202
                        ]),
                    ),
                    ("caption".into(), Value::from("Names")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-too-few-columns",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    Vec::<DataTableCell>::new(),
                    Vec::<DataTableCell>::new(),
                ])
                .caption("Empty")
                .clear_page_size()
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    ("rows".into(), Value::Array(vec![Value::Array(vec![]); 2])),
                    ("caption".into(), Value::from("Empty")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-too-many-columns",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    vec![DataTableCell::from(RawText::builder().text("A").build()?); 21],
                    vec![DataTableCell::from(RawText::builder().text("A").build()?); 21],
                ])
                .caption("Wide")
                .clear_page_size()
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![
                                Value::Object(
                                    [
                                        ("type".into(), Value::from("raw_text")),
                                        ("text".into(), Value::from("A"))
                                    ]
                                    .into_iter()
                                    .collect()
                                );
                                21
                            ]);
                            2
                        ]),
                    ),
                    ("caption".into(), Value::from("Wide")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-page-size-too-small",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    vec![DataTableCell::from(
                        RawText::builder().text("Name").build()?,
                    )],
                    vec![DataTableCell::from(
                        RawText::builder().text("Alice").build()?,
                    )],
                ])
                .caption("Names")
                .page_size(0_i64)
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Name")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Alice")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                        ]),
                    ),
                    ("caption".into(), Value::from("Names")),
                    ("page_size".into(), serde_json::json!(0)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-page-size-too-large",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    vec![DataTableCell::from(
                        RawText::builder().text("Name").build()?,
                    )],
                    vec![DataTableCell::from(
                        RawText::builder().text("Alice").build()?,
                    )],
                ])
                .caption("Names")
                .page_size(101_i64)
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Name")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Alice")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                        ]),
                    ),
                    ("caption".into(), Value::from("Names")),
                    ("page_size".into(), serde_json::json!(101)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-cell-text-empty",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    vec![DataTableCell::from(
                        RawText::builder().text("Name").build()?,
                    )],
                    vec![DataTableCell::from(RawText::builder().text("").build()?)],
                ])
                .caption("Names")
                .clear_page_size()
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Name")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                        ]),
                    ),
                    ("caption".into(), Value::from("Names")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-content-too-long",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    vec![DataTableCell::from(
                        RawText::builder().text("Name").build()?,
                    )],
                    vec![DataTableCell::from(
                        RawText::builder().text("x".repeat(20000)).build()?,
                    )],
                ])
                .caption("Names")
                .clear_page_size()
                .clear_row_header_column_index()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Name")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("x".repeat(20000))),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                        ]),
                    ),
                    ("caption".into(), Value::from("Names")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-visualization-title-too-long",
        build: || {
            let _ = DataVisualizationBlock::builder()
                .title("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                .chart(
                    PieChart::builder()
                        .segments(vec![
                            ChartSegment::builder().label("A").value(1_i64).build()?,
                        ])
                        .build()?,
                )
                .build()?;
            Ok(())
        },
        parse: || {
            DataVisualizationBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_visualization")),
                    (
                        "title".into(),
                        Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
                    ),
                    (
                        "chart".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("pie")),
                                (
                                    "segments".into(),
                                    Value::Array(vec![Value::Object(
                                        [
                                            ("label".into(), Value::from("A")),
                                            ("value".into(), serde_json::json!(1)),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    )]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "pie-chart-empty",
        build: || {
            let _ = PieChart::builder()
                .segments(Vec::<ChartSegment>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            PieChart::try_from(Value::Object(
                [
                    ("type".into(), Value::from("pie")),
                    ("segments".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "pie-chart-too-many-segments",
        build: || {
            let _ = PieChart::builder()
                .segments(vec![
                    ChartSegment::builder()
                        .label("S")
                        .value(1_i64)
                        .build()?;
                    13
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            PieChart::try_from(Value::Object(
                [
                    ("type".into(), Value::from("pie")),
                    (
                        "segments".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("label".into(), Value::from("S")),
                                    ("value".into(), serde_json::json!(1))
                                ]
                                .into_iter()
                                .collect()
                            );
                            13
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "chart-segment-label-too-long",
        build: || {
            let _ = ChartSegment::builder()
                .label("xxxxxxxxxxxxxxxxxxxxx")
                .value(1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            ChartSegment::try_from(Value::Object(
                [
                    ("label".into(), Value::from("xxxxxxxxxxxxxxxxxxxxx")),
                    ("value".into(), serde_json::json!(1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "chart-segment-value-not-positive",
        build: || {
            let _ = ChartSegment::builder().label("A").value(0_i64).build()?;
            Ok(())
        },
        parse: || {
            ChartSegment::try_from(Value::Object(
                [
                    ("label".into(), Value::from("A")),
                    ("value".into(), serde_json::json!(0)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "chart-series-empty",
        build: || {
            let _ = LineChart::builder()
                .axis_config(
                    AxisConfig::builder()
                        .categories(vec![String::from("A")])
                        .build()?,
                )
                .series(Vec::<DataSeries>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            LineChart::try_from(Value::Object(
                [
                    ("type".into(), Value::from("line")),
                    (
                        "axis_config".into(),
                        Value::Object(
                            [("categories".into(), Value::Array(vec![Value::from("A")]))]
                                .into_iter()
                                .collect(),
                        ),
                    ),
                    ("series".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "chart-duplicate-point-labels",
        build: || {
            let _ = LineChart::builder()
                .axis_config(
                    AxisConfig::builder()
                        .categories(vec![String::from("A"), String::from("B")])
                        .build()?,
                )
                .series(vec![
                    DataSeries::builder()
                        .name("Series")
                        .data(vec![
                            DataPoint::builder().label("A").value(1_i64).build()?,
                            DataPoint::builder().label("A").value(2_i64).build()?,
                        ])
                        .build()?,
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            LineChart::try_from(Value::Object(
                [
                    ("type".into(), Value::from("line")),
                    (
                        "axis_config".into(),
                        Value::Object(
                            [(
                                "categories".into(),
                                Value::Array(vec![Value::from("A"), Value::from("B")]),
                            )]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "series".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("name".into(), Value::from("Series")),
                                (
                                    "data".into(),
                                    Value::Array(vec![
                                        Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        ),
                                        Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(2)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        ),
                                    ]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "chart-too-many-series",
        build: || {
            let _ = LineChart::builder()
                .series(vec![
                    DataSeries::builder()
                        .name("S0")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S1")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S2")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S3")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S4")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S5")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S6")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S7")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S8")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S9")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S10")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S11")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                    DataSeries::builder()
                        .name("S12")
                        .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                        .build()?,
                ])
                .axis_config(
                    AxisConfig::builder()
                        .categories(vec![String::from("A")])
                        .build()?,
                )
                .build()?;
            Ok(())
        },
        parse: || {
            LineChart::try_from(Value::Object(
                [
                    ("type".into(), Value::from("line")),
                    (
                        "series".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S0")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S1")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S2")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S3")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S4")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S5")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S6")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S7")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S8")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S9")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S10")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S11")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("name".into(), Value::from("S12")),
                                    (
                                        "data".into(),
                                        Value::Array(vec![Value::Object(
                                            [
                                                ("label".into(), Value::from("A")),
                                                ("value".into(), serde_json::json!(1)),
                                            ]
                                            .into_iter()
                                            .collect(),
                                        )]),
                                    ),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                        ]),
                    ),
                    (
                        "axis_config".into(),
                        Value::Object(
                            [("categories".into(), Value::Array(vec![Value::from("A")]))]
                                .into_iter()
                                .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-series-name-too-long",
        build: || {
            let _ = DataSeries::builder()
                .name("xxxxxxxxxxxxxxxxxxxxx")
                .data(vec![DataPoint::builder().label("A").value(1_i64).build()?])
                .build()?;
            Ok(())
        },
        parse: || {
            DataSeries::try_from(Value::Object(
                [
                    ("name".into(), Value::from("xxxxxxxxxxxxxxxxxxxxx")),
                    (
                        "data".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("label".into(), Value::from("A")),
                                ("value".into(), serde_json::json!(1)),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-series-empty",
        build: || {
            let _ = DataSeries::builder()
                .name("Series")
                .data(Vec::<DataPoint>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            DataSeries::try_from(Value::Object(
                [
                    ("name".into(), Value::from("Series")),
                    ("data".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-series-too-many-points",
        build: || {
            let _ = DataSeries::builder()
                .name("Series")
                .data(vec![
                    DataPoint::builder().label("P0").value(0_i64).build()?,
                    DataPoint::builder().label("P1").value(1_i64).build()?,
                    DataPoint::builder().label("P2").value(2_i64).build()?,
                    DataPoint::builder().label("P3").value(3_i64).build()?,
                    DataPoint::builder().label("P4").value(4_i64).build()?,
                    DataPoint::builder().label("P5").value(5_i64).build()?,
                    DataPoint::builder().label("P6").value(6_i64).build()?,
                    DataPoint::builder().label("P7").value(7_i64).build()?,
                    DataPoint::builder().label("P8").value(8_i64).build()?,
                    DataPoint::builder().label("P9").value(9_i64).build()?,
                    DataPoint::builder().label("P10").value(10_i64).build()?,
                    DataPoint::builder().label("P11").value(11_i64).build()?,
                    DataPoint::builder().label("P12").value(12_i64).build()?,
                    DataPoint::builder().label("P13").value(13_i64).build()?,
                    DataPoint::builder().label("P14").value(14_i64).build()?,
                    DataPoint::builder().label("P15").value(15_i64).build()?,
                    DataPoint::builder().label("P16").value(16_i64).build()?,
                    DataPoint::builder().label("P17").value(17_i64).build()?,
                    DataPoint::builder().label("P18").value(18_i64).build()?,
                    DataPoint::builder().label("P19").value(19_i64).build()?,
                    DataPoint::builder().label("P20").value(20_i64).build()?,
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            DataSeries::try_from(Value::Object(
                [
                    ("name".into(), Value::from("Series")),
                    (
                        "data".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P0")),
                                    ("value".into(), serde_json::json!(0)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P1")),
                                    ("value".into(), serde_json::json!(1)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P2")),
                                    ("value".into(), serde_json::json!(2)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P3")),
                                    ("value".into(), serde_json::json!(3)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P4")),
                                    ("value".into(), serde_json::json!(4)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P5")),
                                    ("value".into(), serde_json::json!(5)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P6")),
                                    ("value".into(), serde_json::json!(6)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P7")),
                                    ("value".into(), serde_json::json!(7)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P8")),
                                    ("value".into(), serde_json::json!(8)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P9")),
                                    ("value".into(), serde_json::json!(9)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P10")),
                                    ("value".into(), serde_json::json!(10)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P11")),
                                    ("value".into(), serde_json::json!(11)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P12")),
                                    ("value".into(), serde_json::json!(12)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P13")),
                                    ("value".into(), serde_json::json!(13)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P14")),
                                    ("value".into(), serde_json::json!(14)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P15")),
                                    ("value".into(), serde_json::json!(15)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P16")),
                                    ("value".into(), serde_json::json!(16)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P17")),
                                    ("value".into(), serde_json::json!(17)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P18")),
                                    ("value".into(), serde_json::json!(18)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P19")),
                                    ("value".into(), serde_json::json!(19)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("label".into(), Value::from("P20")),
                                    ("value".into(), serde_json::json!(20)),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-point-label-too-long",
        build: || {
            let _ = DataPoint::builder()
                .label("xxxxxxxxxxxxxxxxxxxxx")
                .value(1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            DataPoint::try_from(Value::Object(
                [
                    ("label".into(), Value::from("xxxxxxxxxxxxxxxxxxxxx")),
                    ("value".into(), serde_json::json!(1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "axis-categories-empty",
        build: || {
            let _ = AxisConfig::builder()
                .categories(Vec::<String>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            AxisConfig::try_from(Value::Object(
                [("categories".into(), Value::Array(vec![]))]
                    .into_iter()
                    .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "axis-too-many-categories",
        build: || {
            let _ = AxisConfig::builder()
                .categories(vec![
                    String::from("X0"),
                    String::from("X1"),
                    String::from("X2"),
                    String::from("X3"),
                    String::from("X4"),
                    String::from("X5"),
                    String::from("X6"),
                    String::from("X7"),
                    String::from("X8"),
                    String::from("X9"),
                    String::from("X10"),
                    String::from("X11"),
                    String::from("X12"),
                    String::from("X13"),
                    String::from("X14"),
                    String::from("X15"),
                    String::from("X16"),
                    String::from("X17"),
                    String::from("X18"),
                    String::from("X19"),
                    String::from("X20"),
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            AxisConfig::try_from(Value::Object(
                [(
                    "categories".into(),
                    Value::Array(vec![
                        Value::from("X0"),
                        Value::from("X1"),
                        Value::from("X2"),
                        Value::from("X3"),
                        Value::from("X4"),
                        Value::from("X5"),
                        Value::from("X6"),
                        Value::from("X7"),
                        Value::from("X8"),
                        Value::from("X9"),
                        Value::from("X10"),
                        Value::from("X11"),
                        Value::from("X12"),
                        Value::from("X13"),
                        Value::from("X14"),
                        Value::from("X15"),
                        Value::from("X16"),
                        Value::from("X17"),
                        Value::from("X18"),
                        Value::from("X19"),
                        Value::from("X20"),
                    ]),
                )]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "axis-category-label-too-long",
        build: || {
            let _ = AxisConfig::builder()
                .categories(vec![String::from("xxxxxxxxxxxxxxxxxxxxx")])
                .build()?;
            Ok(())
        },
        parse: || {
            AxisConfig::try_from(Value::Object(
                [(
                    "categories".into(),
                    Value::Array(vec![Value::from("xxxxxxxxxxxxxxxxxxxxx")]),
                )]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "axis-label-too-long",
        build: || {
            let _ = AxisConfig::builder()
                .categories(vec![String::from("A")])
                .x_label("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                .build()?;
            Ok(())
        },
        parse: || {
            AxisConfig::try_from(Value::Object(
                [
                    ("categories".into(), Value::Array(vec![Value::from("A")])),
                    (
                        "x_label".into(),
                        Value::from("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plain-text-input-min-length-negative",
        build: || {
            let _ = PlainTextInputElement::builder()
                .action_id("a")
                .min_length(-1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            PlainTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plain_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("min_length".into(), serde_json::json!(-1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plain-text-input-min-length-too-large",
        build: || {
            let _ = PlainTextInputElement::builder()
                .action_id("a")
                .min_length(3001_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            PlainTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plain_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("min_length".into(), serde_json::json!(3001)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plain-text-input-max-length-too-small",
        build: || {
            let _ = PlainTextInputElement::builder()
                .action_id("a")
                .max_length(0_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            PlainTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plain_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("max_length".into(), serde_json::json!(0)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-input-min-lines-too-small",
        build: || {
            let _ = RichTextInputElement::builder()
                .action_id("a")
                .min_lines(0_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("min_lines".into(), serde_json::json!(0)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-input-min-lines-too-large",
        build: || {
            let _ = RichTextInputElement::builder()
                .action_id("a")
                .min_lines(101_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("min_lines".into(), serde_json::json!(101)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-input-max-lines-too-small",
        build: || {
            let _ = RichTextInputElement::builder()
                .action_id("a")
                .max_lines(0_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("max_lines".into(), serde_json::json!(0)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-input-max-lines-too-large",
        build: || {
            let _ = RichTextInputElement::builder()
                .action_id("a")
                .max_lines(101_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_input")),
                    ("action_id".into(), Value::from("a")),
                    ("max_lines".into(), serde_json::json!(101)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "multi-select-max-selected-items-too-small",
        build: || {
            let _ = UserMultiSelectElement::builder()
                .action_id("a")
                .max_selected_items(0_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            UserMultiSelectElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("multi_users_select")),
                    ("action_id".into(), Value::from("a")),
                    ("max_selected_items".into(), serde_json::json!(0)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "conversation-filter-include-empty",
        build: || {
            let _ = ConversationFilter::builder()
                .include(Vec::<String>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            ConversationFilter::try_from(Value::Object(
                [("include".into(), Value::Array(vec![]))]
                    .into_iter()
                    .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "conversation-filter-unknown-include",
        build: || {
            let _ = ConversationFilter::builder()
                .include(vec![String::from("channel")])
                .build()?;
            Ok(())
        },
        parse: || {
            ConversationFilter::try_from(Value::Object(
                [("include".into(), Value::Array(vec![Value::from("channel")]))]
                    .into_iter()
                    .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "workflow-button-missing-action-id",
        build: || {
            let _ = WorkflowButtonElement::builder()
                .text(PlainText::builder().text("Run").build()?)
                .workflow(
                    Workflow::builder()
                        .trigger(
                            Trigger::builder()
                                .url("https://slack.com/shortcuts/Ft0/abc")
                                .build()?,
                        )
                        .build()?,
                )
                .build()?;
            Ok(())
        },
        parse: || {
            WorkflowButtonElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("workflow_button")),
                    (
                        "text".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Run")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "workflow".into(),
                        Value::Object(
                            [(
                                "trigger".into(),
                                Value::Object(
                                    [(
                                        "url".into(),
                                        Value::from("https://slack.com/shortcuts/Ft0/abc"),
                                    )]
                                    .into_iter()
                                    .collect(),
                                ),
                            )]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "number-input-missing-decimal-flag",
        build: || {
            let _ = NumberInputElement::builder().action_id("a").build()?;
            Ok(())
        },
        parse: || {
            NumberInputElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("number_input")),
                    ("action_id".into(), Value::from("a")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "slack-icon-missing-name",
        build: || {
            let _ = SlackIcon::builder().build()?;
            Ok(())
        },
        parse: || {
            SlackIcon::try_from(Value::Object(
                [("type".into(), Value::from("icon"))].into_iter().collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "slack-file-id-malformed",
        build: || {
            let _ = SlackFile::builder().id("F0123456").build()?;
            Ok(())
        },
        parse: || {
            SlackFile::try_from(Value::Object(
                [("id".into(), Value::from("F0123456"))]
                    .into_iter()
                    .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-element-url-too-long",
        build: || {
            let _ = ImageElement::builder()
                .image_url("x".repeat(3001))
                .alt_text("Alt")
                .build()?;
            Ok(())
        },
        parse: || {
            ImageElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    ("image_url".into(), Value::from("x".repeat(3001))),
                    ("alt_text".into(), Value::from("Alt")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-element-alt-text-too-long",
        build: || {
            let _ = ImageElement::builder()
                .image_url("https://example.com/image.png")
                .alt_text("x".repeat(2001))
                .build()?;
            Ok(())
        },
        parse: || {
            ImageElement::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    (
                        "image_url".into(),
                        Value::from("https://example.com/image.png"),
                    ),
                    ("alt_text".into(), Value::from("x".repeat(2001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-block-title-too-long",
        build: || {
            let _ = ImageBlock::builder()
                .image_url("https://example.com/image.png")
                .alt_text("Alt")
                .title(PlainText::builder().text("x".repeat(2001)).build()?)
                .build()?;
            Ok(())
        },
        parse: || {
            ImageBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    (
                        "image_url".into(),
                        Value::from("https://example.com/image.png"),
                    ),
                    ("alt_text".into(), Value::from("Alt")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("x".repeat(2001))),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-block-url-and-slack-file",
        build: || {
            let _ = ImageBlock::builder()
                .image_url("https://example.com/image.png")
                .slack_file(SlackFile::builder().id("F0123ABC456").build()?)
                .alt_text("Alt")
                .build()?;
            Ok(())
        },
        parse: || {
            ImageBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    (
                        "image_url".into(),
                        Value::from("https://example.com/image.png"),
                    ),
                    (
                        "slack_file".into(),
                        Value::Object(
                            [("id".into(), Value::from("F0123ABC456"))]
                                .into_iter()
                                .collect(),
                        ),
                    ),
                    ("alt_text".into(), Value::from("Alt")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "image-block-missing-source",
        build: || {
            let _ = ImageBlock::builder().alt_text("Alt").build()?;
            Ok(())
        },
        parse: || {
            ImageBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("image")),
                    ("alt_text".into(), Value::from("Alt")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "container-no-child-blocks",
        build: || {
            let _ = ContainerBlock::builder()
                .title(PlainText::builder().text("Container").build()?)
                .child_blocks(Vec::<Block>::new())
                .build()?;
            Ok(())
        },
        parse: || {
            ContainerBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("container")),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Container")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("child_blocks".into(), Value::Array(vec![])),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "table-too-many-column-settings",
        build: || {
            let _ = TableBlock::builder()
                .rows(vec![vec![TableCell::from(
                    RawText::builder().text("A").build()?,
                )]])
                .column_settings(vec![
                    ColumnSettings::builder().is_wrapped(true).build()?;
                    21
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            TableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("table")),
                    (
                        "rows".into(),
                        Value::Array(vec![Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("raw_text")),
                                ("text".into(), Value::from("A")),
                            ]
                            .into_iter()
                            .collect(),
                        )])]),
                    ),
                    (
                        "column_settings".into(),
                        Value::Array(vec![
                            Value::Object(
                                [("is_wrapped".into(), Value::from(true))]
                                    .into_iter()
                                    .collect()
                            );
                            21
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-row-header-index-negative",
        build: || {
            let _ = DataTableBlock::builder()
                .rows(vec![
                    vec![DataTableCell::from(
                        RawText::builder().text("Name").build()?,
                    )],
                    vec![DataTableCell::from(
                        RawText::builder().text("Alice").build()?,
                    )],
                ])
                .caption("Names")
                .row_header_column_index(-1_i64)
                .clear_page_size()
                .build()?;
            Ok(())
        },
        parse: || {
            DataTableBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("data_table")),
                    (
                        "rows".into(),
                        Value::Array(vec![
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Name")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                            Value::Array(vec![Value::Object(
                                [
                                    ("type".into(), Value::from("raw_text")),
                                    ("text".into(), Value::from("Alice")),
                                ]
                                .into_iter()
                                .collect(),
                            )]),
                        ]),
                    ),
                    ("caption".into(), Value::from("Names")),
                    ("row_header_column_index".into(), serde_json::json!(-1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "data-table-total-content-too-long",
        build: || {
            let _ = MessagePayload::builder("C123")
                .blocks(vec![
                    Block::from(
                        DataTableBlock::builder()
                            .rows(vec![
                                vec![DataTableCell::from(
                                    RawText::builder().text("Name").build()?
                                )],
                                vec![DataTableCell::from(
                                    RawText::builder().text("x".repeat(9997)).build()?
                                )]
                            ])
                            .caption("Names")
                            .clear_page_size()
                            .clear_row_header_column_index()
                            .build()?
                    );
                    2
                ])
                .clear_mrkdwn()
                .clear_text()
                .build()?;
            Ok(())
        },
        parse: || {
            MessagePayload::try_from(Value::Object(
                [
                    ("channel".into(), Value::from("C123")),
                    (
                        "blocks".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("data_table")),
                                    (
                                        "rows".into(),
                                        Value::Array(vec![
                                            Value::Array(vec![Value::Object(
                                                [
                                                    ("type".into(), Value::from("raw_text")),
                                                    ("text".into(), Value::from("Name"))
                                                ]
                                                .into_iter()
                                                .collect()
                                            )]),
                                            Value::Array(vec![Value::Object(
                                                [
                                                    ("type".into(), Value::from("raw_text")),
                                                    ("text".into(), Value::from("x".repeat(9997)))
                                                ]
                                                .into_iter()
                                                .collect()
                                            )])
                                        ])
                                    ),
                                    ("caption".into(), Value::from("Names"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            2
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "markdown-total-too-long",
        build: || {
            let _ = MessagePayload::builder("C123")
                .blocks(vec![
                    Block::from(
                        MarkdownBlock::builder().text("x".repeat(6001)).build()?
                    );
                    2
                ])
                .clear_mrkdwn()
                .clear_text()
                .build()?;
            Ok(())
        },
        parse: || {
            MessagePayload::try_from(Value::Object(
                [
                    ("channel".into(), Value::from("C123")),
                    (
                        "blocks".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("type".into(), Value::from("markdown")),
                                    ("text".into(), Value::from("x".repeat(6001)))
                                ]
                                .into_iter()
                                .collect()
                            );
                            2
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plan-missing-tasks",
        build: || {
            let _ = PlanBlock::builder().title("Plan").build()?;
            Ok(())
        },
        parse: || {
            PlanBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plan")),
                    ("title".into(), Value::from("Plan")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plan-too-many-tasks",
        build: || {
            let _ = PlanBlock::builder()
                .title("Plan")
                .tasks(vec![
                    TaskCardBlock::builder()
                        .task_id("task_0")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_1")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_2")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_3")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_4")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_5")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_6")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_7")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_8")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_9")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_10")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_11")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_12")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_13")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_14")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_15")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_16")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_17")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_18")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_19")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_20")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_21")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_22")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_23")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_24")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_25")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_26")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_27")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_28")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_29")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_30")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_31")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_32")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_33")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_34")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_35")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_36")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_37")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_38")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_39")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_40")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_41")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_42")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_43")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_44")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_45")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_46")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_47")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_48")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_49")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                    TaskCardBlock::builder()
                        .task_id("task_50")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?,
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            PlanBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plan")),
                    ("title".into(), Value::from("Plan")),
                    (
                        "tasks".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_0")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_1")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_2")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_3")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_4")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_5")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_6")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_7")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_8")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_9")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_10")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_11")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_12")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_13")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_14")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_15")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_16")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_17")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_18")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_19")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_20")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_21")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_22")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_23")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_24")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_25")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_26")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_27")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_28")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_29")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_30")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_31")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_32")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_33")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_34")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_35")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_36")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_37")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_38")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_39")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_40")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_41")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_42")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_43")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_44")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_45")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_46")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_47")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_48")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_49")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task_50")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete")),
                                ]
                                .into_iter()
                                .collect(),
                            ),
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "plan-duplicate-task-ids",
        build: || {
            let _ = PlanBlock::builder()
                .title("Plan")
                .tasks(vec![
                    TaskCardBlock::builder()
                        .task_id("task")
                        .title("Task")
                        .status(TaskStatus::Complete)
                        .build()?;
                    2
                ])
                .build()?;
            Ok(())
        },
        parse: || {
            PlanBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("plan")),
                    ("title".into(), Value::from("Plan")),
                    (
                        "tasks".into(),
                        Value::Array(vec![
                            Value::Object(
                                [
                                    ("task_id".into(), Value::from("task")),
                                    ("title".into(), Value::from("Task")),
                                    ("status".into(), Value::from("complete"))
                                ]
                                .into_iter()
                                .collect()
                            );
                            2
                        ]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "task-card-missing-status",
        build: || {
            let _ = TaskCardBlock::builder()
                .task_id("task")
                .title("Task")
                .build()?;
            Ok(())
        },
        parse: || {
            TaskCardBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("task_card")),
                    ("task_id".into(), Value::from("task")),
                    ("title".into(), Value::from("Task")),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "task-card-pending-status",
        build: || {
            let _ = MessagePayload::builder("C123")
                .blocks(vec![Block::from(
                    TaskCardBlock::builder()
                        .task_id("task")
                        .title("Task")
                        .status(TaskStatus::Pending)
                        .build()?,
                )])
                .clear_mrkdwn()
                .clear_text()
                .build()?;
            Ok(())
        },
        parse: || {
            MessagePayload::try_from(Value::Object(
                [
                    ("channel".into(), Value::from("C123")),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("task_card")),
                                ("task_id".into(), Value::from("task")),
                                ("title".into(), Value::from("Task")),
                                ("status".into(), Value::from("pending")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-list-indent-negative",
        build: || {
            let _ = RichTextList::builder()
                .style(RichTextListStyle::Bullet)
                .elements(vec![
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder().text("Item").build()?,
                        )])
                        .build()?,
                ])
                .indent(-1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextList::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_list")),
                    ("style".into(), Value::from("bullet")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("rich_text_section")),
                                (
                                    "elements".into(),
                                    Value::Array(vec![Value::Object(
                                        [
                                            ("type".into(), Value::from("text")),
                                            ("text".into(), Value::from("Item")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    )]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("indent".into(), serde_json::json!(-1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-list-indent-too-large",
        build: || {
            let _ = RichTextList::builder()
                .style(RichTextListStyle::Bullet)
                .elements(vec![
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder().text("Item").build()?,
                        )])
                        .build()?,
                ])
                .indent(9_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextList::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_list")),
                    ("style".into(), Value::from("bullet")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("rich_text_section")),
                                (
                                    "elements".into(),
                                    Value::Array(vec![Value::Object(
                                        [
                                            ("type".into(), Value::from("text")),
                                            ("text".into(), Value::from("Item")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    )]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("indent".into(), serde_json::json!(9)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-list-offset-negative",
        build: || {
            let _ = RichTextList::builder()
                .style(RichTextListStyle::Bullet)
                .elements(vec![
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder().text("Item").build()?,
                        )])
                        .build()?,
                ])
                .offset(-1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextList::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_list")),
                    ("style".into(), Value::from("bullet")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("rich_text_section")),
                                (
                                    "elements".into(),
                                    Value::Array(vec![Value::Object(
                                        [
                                            ("type".into(), Value::from("text")),
                                            ("text".into(), Value::from("Item")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    )]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("offset".into(), serde_json::json!(-1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-list-border-negative",
        build: || {
            let _ = RichTextList::builder()
                .style(RichTextListStyle::Bullet)
                .elements(vec![
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder().text("Item").build()?,
                        )])
                        .build()?,
                ])
                .border(-1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextList::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_list")),
                    ("style".into(), Value::from("bullet")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("rich_text_section")),
                                (
                                    "elements".into(),
                                    Value::Array(vec![Value::Object(
                                        [
                                            ("type".into(), Value::from("text")),
                                            ("text".into(), Value::from("Item")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    )]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("border".into(), serde_json::json!(-1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-list-border-too-large",
        build: || {
            let _ = RichTextList::builder()
                .style(RichTextListStyle::Bullet)
                .elements(vec![
                    RichTextSection::builder()
                        .elements(vec![RichTextSectionElement::from(
                            RichTextText::builder().text("Item").build()?,
                        )])
                        .build()?,
                ])
                .border(2_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextList::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_list")),
                    ("style".into(), Value::from("bullet")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("rich_text_section")),
                                (
                                    "elements".into(),
                                    Value::Array(vec![Value::Object(
                                        [
                                            ("type".into(), Value::from("text")),
                                            ("text".into(), Value::from("Item")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    )]),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("border".into(), serde_json::json!(2)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-quote-border-negative",
        build: || {
            let _ = RichTextQuote::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Text").build()?,
                )])
                .border(-1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextQuote::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_quote")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("text")),
                                ("text".into(), Value::from("Text")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("border".into(), serde_json::json!(-1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-quote-border-too-large",
        build: || {
            let _ = RichTextQuote::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Text").build()?,
                )])
                .border(2_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextQuote::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_quote")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("text")),
                                ("text".into(), Value::from("Text")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("border".into(), serde_json::json!(2)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-preformatted-border-negative",
        build: || {
            let _ = RichTextCodeBlock::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Text").build()?,
                )])
                .border(-1_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextCodeBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_preformatted")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("text")),
                                ("text".into(), Value::from("Text")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("border".into(), serde_json::json!(-1)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "rich-text-preformatted-border-too-large",
        build: || {
            let _ = RichTextCodeBlock::builder()
                .elements(vec![RichTextSectionElement::from(
                    RichTextText::builder().text("Text").build()?,
                )])
                .border(2_i64)
                .build()?;
            Ok(())
        },
        parse: || {
            RichTextCodeBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("rich_text_preformatted")),
                    (
                        "elements".into(),
                        Value::Array(vec![Value::Object(
                            [
                                ("type".into(), Value::from("text")),
                                ("text".into(), Value::from("Text")),
                            ]
                            .into_iter()
                            .collect(),
                        )]),
                    ),
                    ("border".into(), serde_json::json!(2)),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-thumbnail-url-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("x".repeat(3001))
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/video.mp4")
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    ("thumbnail_url".into(), Value::from("x".repeat(3001))),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-url-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("x".repeat(3001))
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    ("video_url".into(), Value::from("x".repeat(3001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-title-url-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/video.mp4")
                .title_url("x".repeat(2001))
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                    ("title_url".into(), Value::from("x".repeat(2001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "video-provider-icon-url-too-long",
        build: || {
            let _ = VideoBlock::builder()
                .alt_text("Video")
                .thumbnail_url("https://example.com/thumbnail.png")
                .title(PlainText::builder().text("Title").build()?)
                .video_url("https://example.com/video.mp4")
                .provider_icon_url("x".repeat(2001))
                .build()?;
            Ok(())
        },
        parse: || {
            VideoBlock::try_from(Value::Object(
                [
                    ("type".into(), Value::from("video")),
                    ("alt_text".into(), Value::from("Video")),
                    (
                        "thumbnail_url".into(),
                        Value::from("https://example.com/thumbnail.png"),
                    ),
                    (
                        "title".into(),
                        Value::Object(
                            [
                                ("type".into(), Value::from("plain_text")),
                                ("text".into(), Value::from("Title")),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                    (
                        "video_url".into(),
                        Value::from("https://example.com/video.mp4"),
                    ),
                    ("provider_icon_url".into(), Value::from("x".repeat(2001))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "view-external-id-too-long",
        build: || {
            let _ = HomeTabView::builder()
                .blocks(vec![Block::from(DividerBlock::builder().build()?)])
                .external_id("x".repeat(256))
                .build()?;
            Ok(())
        },
        parse: || {
            HomeTabView::try_from(Value::Object(
                [
                    ("type".into(), Value::from("home")),
                    (
                        "blocks".into(),
                        Value::Array(vec![Value::Object(
                            [("type".into(), Value::from("divider"))]
                                .into_iter()
                                .collect(),
                        )]),
                    ),
                    ("external_id".into(), Value::from("x".repeat(256))),
                ]
                .into_iter()
                .collect(),
            ))
            .map(|_| ())
        },
    },
    InvalidCase {
        id: "attachment-missing-blocks",
        build: || {
            let _ = Attachment::builder().fallback("Summary").build()?;
            Ok(())
        },
        parse: || {
            Attachment::try_from(Value::Object(
                [("fallback".into(), Value::from("Summary"))]
                    .into_iter()
                    .collect(),
            ))
            .map(|_| ())
        },
    },
];
