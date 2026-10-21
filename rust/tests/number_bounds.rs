use slackblocks::NumberInputElement;
#[test]
fn numeric_convenience_and_string_wire() {
    let value = NumberInputElement::builder()
        .is_decimal_allowed(true)
        .min_value(-1.5)
        .max_value("10.25")
        .build()
        .unwrap();
    assert_eq!(value.min_value(), Some("-1.5"));
    assert_eq!(value.max_value(), Some("10.25"));
    let wire = serde_json::to_value(&value).unwrap();
    assert_eq!(wire["min_value"], "-1.5");
    let parsed: NumberInputElement = serde_json::from_value(wire).unwrap();
    assert_eq!(parsed.min_value(), Some("-1.5"));
    assert_eq!(
        NumberInputElement::builder()
            .is_decimal_allowed(false)
            .min_value(0)
            .build()
            .unwrap()
            .min_value(),
        Some("0")
    );
    assert_eq!(
        NumberInputElement::builder()
            .is_decimal_allowed(true)
            .build()
            .unwrap()
            .min_value(),
        None
    );
    for value in ["NaN", "inf", "invalid", ""] {
        assert!(
            NumberInputElement::builder()
                .is_decimal_allowed(true)
                .min_value(value)
                .build()
                .is_err()
        );
    }
    assert!(
        NumberInputElement::builder()
            .is_decimal_allowed(true)
            .min_value(f64::NAN)
            .build()
            .is_err()
    );
    assert!(
        NumberInputElement::builder()
            .is_decimal_allowed(true)
            .min_value("10")
            .max_value("2")
            .build()
            .is_err()
    );
}

#[test]
fn decimal_string_ordering_is_exact() {
    for (min, max) in [
        ("9007199254740993", "9007199254740992"),
        ("0.10000000000000001", "0.1"),
        ("-9007199254740992", "-9007199254740993"),
        ("-0.1", "-0.10000000000000001"),
        ("1e-1000", "9e-1001"),
    ] {
        assert!(
            NumberInputElement::builder()
                .is_decimal_allowed(true)
                .min_value(min)
                .max_value(max)
                .build()
                .is_err()
        );
        assert!(
            NumberInputElement::builder()
                .is_decimal_allowed(true)
                .min_value(max)
                .max_value(min)
                .build()
                .is_ok()
        );
    }
    assert!(
        NumberInputElement::builder()
            .is_decimal_allowed(true)
            .min_value("0.1000")
            .max_value("1e-1")
            .build()
            .is_ok()
    );
}
