use slackblocks::{ErrorCategory, PlainTextInputElement};
#[test]
fn satisfiable_omitted_and_inverted_bounds() {
    for (min, max) in [
        (None, None),
        (Some(20), None),
        (None, Some(10)),
        (Some(0), Some(1)),
        (Some(10), Some(10)),
        (Some(3000), Some(3000)),
    ] {
        let mut builder = PlainTextInputElement::builder();
        if let Some(value) = min {
            builder = builder.min_length(value);
        }
        if let Some(value) = max {
            builder = builder.max_length(value);
        }
        let value = builder.build().unwrap();
        assert_eq!(value.min_length(), min);
        assert_eq!(value.max_length(), max);
    }
    let error = PlainTextInputElement::builder()
        .min_length(20)
        .max_length(10)
        .build()
        .unwrap_err();
    assert_eq!(error.category(), ErrorCategory::OutOfRange);
    assert_eq!(error.path(), "PlainTextInputElement.min_length");
}

#[test]
fn structural_controls_remain_accepted() {
    use slackblocks::{
        DatePickerElement, ExternalMultiSelectElement, ExternalSelectElement, ImageElement,
        TimePickerElement,
    };
    assert_eq!(
        ExternalSelectElement::builder()
            .min_query_length(0)
            .build()
            .unwrap()
            .min_query_length(),
        Some(0)
    );
    assert_eq!(
        ExternalMultiSelectElement::builder()
            .min_query_length(0)
            .build()
            .unwrap()
            .min_query_length(),
        Some(0)
    );
    for date in ["2028-02-29", "2028-02-30"] {
        assert_eq!(
            DatePickerElement::builder()
                .initial_date(date)
                .build()
                .unwrap()
                .initial_date(),
            Some(date)
        );
    }
    for time in ["00:00", "23:59"] {
        assert_eq!(
            TimePickerElement::builder()
                .initial_time(time)
                .build()
                .unwrap()
                .initial_time(),
            Some(time)
        );
    }
    assert_eq!(
        ImageElement::builder()
            .image_url("https://example.com/image.png")
            .alt_text("")
            .build()
            .unwrap()
            .alt_text(),
        ""
    );
}
