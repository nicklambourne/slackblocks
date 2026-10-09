use proptest::prelude::*;
use slackblocks::*;
proptest! {
    #[test]
    fn unicode_limits_count_codepoints_not_bytes(chars in prop::collection::vec(any::<char>(), 1..=75)) {
        let text:String=chars.into_iter().collect();
        let button=ButtonElement::builder().text(text.clone()).build().unwrap();
        prop_assert_eq!(button.text().text(),text);
        let json=serde_json::to_value(&button).unwrap();
        prop_assert_eq!(ButtonElement::try_from(json).unwrap(),button);
    }
    #[test]
    fn integer_numbers_never_round(value in any::<i64>()) {
        let number=JsonNumber::from(value);
        let json=serde_json::to_value(&number).unwrap();
        prop_assert_eq!(json.as_i64(),Some(value));
        prop_assert_eq!(JsonNumber::try_from(json.as_number().unwrap().clone()).unwrap(),number);
    }
    #[test]
    fn editing_does_not_mutate_original(text in "[a-zA-Z0-9]{1,75}", edited in "[a-zA-Z0-9]{1,75}") {
        let original=ButtonElement::builder().text(text.clone()).build().unwrap();
        let changed=original.clone().into_builder().text(edited.clone()).build().unwrap();
        prop_assert_eq!(original.text().text(),text);
        prop_assert_eq!(changed.text().text(),edited);
    }
    #[test]
    fn paginator_slices_every_item_once(count in 1usize..150, size in 1usize..30) {
        let blocks:Vec<Block>=(0..count).map(|i|DividerBlock::builder().block_id(i.to_string()).build().unwrap().into()).collect();
        let mut actual=Vec::new();
        for page in 1..=count.div_ceil(size) {
            let result=Paginator::builder("pages",blocks.clone()).page_size(size).page(page).show_page_indicator(false).build().unwrap();
            actual.extend(result.into_iter().filter(|v| matches!(v,Block::Divider(_))));
        }
        prop_assert_eq!(actual,blocks);
    }
}

#[cfg(feature = "json-features")]
#[test]
fn feature_unification_rejects_rounding_and_keeps_exact_decimal_values() {
    for text in [
        "1.2345678901234567890123456789",
        "18446744073709551617",
        "1e400",
        "1e-400",
    ] {
        let value: serde_json::Number = serde_json::from_str(text).unwrap();
        assert!(JsonNumber::try_from(value).is_err(), "{text}");
    }
    for text in [
        "1e3",
        "1.0000",
        "-0.0",
        "0e99999999",
        "1e-3",
        "18446744073709551615",
    ] {
        let value: serde_json::Number = serde_json::from_str(text).unwrap();
        assert!(JsonNumber::try_from(value).is_ok(), "{text}");
    }
}
