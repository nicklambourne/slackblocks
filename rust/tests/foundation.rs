use serde_json::json;
use slackblocks::{ErrorCategory, JsonNumber, MessagePayload, PlainText, SectionBlock};

#[test]
fn unicode_limits_and_borrowed_inspection() {
    let text = PlainText::new("😀".repeat(3000)).unwrap();
    assert_eq!(text.text().chars().count(), 3000);
    assert_eq!(
        PlainText::new("😀".repeat(3001)).unwrap_err().category(),
        ErrorCategory::LengthExceeded
    );
    assert_eq!(
        text.clone()
            .into_builder()
            .emoji(false)
            .build()
            .unwrap()
            .emoji(),
        Some(false)
    );
}
#[test]
fn native_editing_and_parsing_preserve_absence() {
    let original = MessagePayload::builder("C")
        .clear_text()
        .clear_mrkdwn()
        .build()
        .unwrap();
    let parsed =
        MessagePayload::try_from(json!({"channel":"C","text":null,"mrkdwn":null})).unwrap();
    assert_eq!(original, parsed);
    assert_eq!(original.clone().into_builder().build().unwrap(), original);
    let supplied = original
        .into_builder()
        .text("")
        .mrkdwn(false)
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(supplied).unwrap(),
        json!({"channel":"C","text":"","mrkdwn":false})
    );
}
#[test]
fn nested_errors_and_extension_collisions_are_structured() {
    let bad = MessagePayload::try_from(
        json!({"channel":"C","blocks":[{"type":"section","text":{"type":"plain_text","text":""}}]}),
    )
    .unwrap_err();
    assert_eq!(bad.category(), ErrorCategory::LengthExceeded);
    assert_eq!(bad.path(), "MessagePayload.blocks[0].text.text");
    let bad = SectionBlock::builder()
        .text("hello")
        .extension("block_id", "reserved")
        .build()
        .unwrap_err();
    assert_eq!(bad.category(), ErrorCategory::InvalidUsage);
    assert_eq!(bad.path(), "SectionBlock.block_id");
    let valid = SectionBlock::builder()
        .text("hello")
        .extension("future", json!({"nullable":null}))
        .build()
        .unwrap();
    assert_eq!(
        SectionBlock::try_from(serde_json::to_value(&valid).unwrap()).unwrap(),
        valid
    );
}
#[test]
fn integer_precision_and_nonfinite_input() {
    for n in [9_007_199_254_740_991_u64, 9_007_199_254_740_993, u64::MAX] {
        let value = JsonNumber::from(n);
        assert_eq!(serde_json::to_string(&value).unwrap(), n.to_string());
        assert_eq!(
            serde_json::from_str::<JsonNumber>(&n.to_string()).unwrap(),
            value
        );
    }
    assert_eq!(
        serde_json::to_string(&JsonNumber::from(i64::MIN)).unwrap(),
        i64::MIN.to_string()
    );
    for n in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(JsonNumber::try_from(n).is_err());
    }
}
