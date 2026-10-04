use serde_json::{Value, json};
use slackblocks::*;
fn divider() -> DividerBlock {
    DividerBlock::builder().build().unwrap()
}
fn wire(v: impl serde::Serialize) -> Value {
    serde_json::to_value(v).unwrap()
}
#[test]
fn accordion_preserves_native_options_and_order() -> Result<(), ValidationError> {
    let section = AccordionSection::builder("Details")
        .subtitle("More")
        .icon(
            ImageElement::builder()
                .image_url("https://example.com/icon.png")
                .alt_text("Icon")
                .build()?,
        )
        .width(ContainerWidth::Full)
        .expanded(true)
        .block_id("details")
        .has_header_divider(false)
        .block(divider())
        .blocks([
            SectionBlock::builder().text("Replaced").build()?.into(),
            Block::from(divider()),
        ])
        .build()?;
    assert_eq!(section.as_container().default_collapsed(), Some(false));
    assert_eq!(section.as_container().width(), Some(ContainerWidth::Full));
    let accordion = Accordion::new([section.clone()])?;
    assert_eq!(accordion.sections(), std::slice::from_ref(&section));
    assert_eq!(
        wire(accordion.clone().into_iter().collect::<Vec<_>>()),
        wire(accordion.render())
    );
    assert_eq!(
        wire(section.clone().into_container()),
        wire(Block::from(section))
    );
    assert!(Accordion::new([]).is_err());
    assert!(
        AccordionSection::builder("Invalid")
            .block(divider())
            .has_header_divider(true)
            .build()
            .is_err()
    );
    let sections = (0..51).map(|_| {
        AccordionSection::builder("Section")
            .block(divider())
            .build()
            .unwrap()
    });
    assert_eq!(
        MessagePayload::builder("C1")
            .blocks(Accordion::new(sections)?)
            .build()
            .unwrap_err()
            .category(),
        ErrorCategory::LengthExceeded
    );
    Ok(())
}
#[test]
fn pagination_navigation_uses_one_based_pages_and_stable_actions() -> Result<(), ValidationError> {
    let blocks = vec![divider(); 11];
    let first = Paginator::builder("items", blocks.clone()).build()?;
    assert_eq!(
        (first.page(), first.page_count(), first.blocks().len()),
        (1, 3, 7)
    );
    let middle = Paginator::builder("items", blocks.clone())
        .page(2)
        .previous_text("Back")
        .next_text("Forward")
        .block_id("nav")
        .build()?;
    let json = wire(middle.clone().into_iter().collect::<Vec<_>>());
    assert_eq!(json, wire(middle.render()));
    assert_eq!(json[5]["elements"][0]["text"], "Page 2 of 3");
    assert_eq!(json[6]["elements"][0]["action_id"], "items.previous");
    assert_eq!(json[6]["elements"][0]["value"], "1");
    assert_eq!(json[6]["elements"][1]["action_id"], "items.next");
    assert_eq!(json[6]["elements"][1]["value"], "3");
    assert_eq!(json[6]["block_id"], "nav");
    let last = Paginator::builder("items", blocks.clone())
        .page(3)
        .show_page_indicator(false)
        .build()?;
    assert_eq!(last.blocks().len(), 2);
    let one = Paginator::builder("items", blocks)
        .page_size(usize::MAX)
        .build()?;
    assert_eq!((one.page_count(), one.blocks().len()), (1, 11));
    assert!(Paginator::builder("", [divider()]).build().is_err());
    assert!(
        Paginator::builder("items", Vec::<Block>::new())
            .build()
            .is_err()
    );
    assert!(
        Paginator::builder("items", [divider()])
            .page(0)
            .build()
            .is_err()
    );
    assert!(
        Paginator::builder("items", [divider()])
            .page_size(0)
            .build()
            .is_err()
    );
    assert!(
        Paginator::builder("items", [divider()])
            .page(usize::MAX)
            .build()
            .is_err()
    );
    assert!(
        Paginator::builder("x".repeat(251), [divider(), divider()])
            .page_size(1)
            .build()
            .is_err()
    );
    assert!(
        Paginator::builder("items", [divider(), divider()])
            .page_size(1)
            .next_text("x".repeat(76))
            .build()
            .is_err()
    );
    let expanded = Paginator::builder("items", vec![divider(); 51])
        .page_size(49)
        .build()?;
    assert!(
        MessagePayload::builder("C1")
            .blocks(expanded)
            .build()
            .is_err()
    );
    Ok(())
}
#[test]
fn workflow_parameters_and_color_normalization() -> Result<(), ValidationError> {
    assert!(
        wire(Workflow::from_url("https://slack.com/shortcuts/x", [])?)["trigger"]
            .get("customizable_input_parameters")
            .is_none()
    );
    let workflow = Workflow::from_url(
        "https://slack.com/shortcuts/x",
        [InputParameter::builder()
            .name("team")
            .value("support")
            .build()?],
    )?;
    assert_eq!(
        wire(workflow)["trigger"]["customizable_input_parameters"][0],
        json!({"name":"team","value":"support"})
    );
    for (color, expected) in [
        (AttachmentColor::GOOD, "good"),
        (AttachmentColor::WARNING, "warning"),
        (AttachmentColor::DANGER, "danger"),
        (AttachmentColor::RED, "#ff0000"),
        (AttachmentColor::BLUE, "#0000ff"),
        (AttachmentColor::GREEN, "#00ff00"),
        (AttachmentColor::YELLOW, "#ffff00"),
        (AttachmentColor::ORANGE, "#ff8800"),
        (AttachmentColor::PURPLE, "#8800ff"),
        (AttachmentColor::BLACK, "#000000"),
    ] {
        assert_eq!(color.as_str(), expected);
        assert_eq!(color.to_string(), expected);
        assert_eq!(wire(&color), expected);
        assert_eq!(
            serde_json::from_value::<AttachmentColor>(json!(expected)).unwrap(),
            color
        );
        assert_eq!(
            wire(
                Attachment::builder()
                    .block(divider())
                    .color(color)
                    .build()?
            )["color"],
            expected
        );
    }
    let custom: AttachmentColor = "AbCdEf".parse()?;
    assert_eq!(String::from(custom), "#AbCdEf");
    for invalid in ["", "#123", "##abcdef", "#gggggg", "ﬀ0000", "GOOD"] {
        assert!(invalid.parse::<AttachmentColor>().is_err());
    }
    assert!(serde_json::from_value::<AttachmentColor>(json!(false)).is_err());
    Ok(())
}
fn decode_fragment(url: &str) -> Value {
    let encoded = url.split_once('#').unwrap().1.as_bytes();
    let mut bytes = Vec::new();
    let mut i = 0;
    while i < encoded.len() {
        if encoded[i] == b'%' {
            bytes.push(
                u8::from_str_radix(std::str::from_utf8(&encoded[i + 1..i + 3]).unwrap(), 16)
                    .unwrap(),
            );
            i += 3;
        } else {
            bytes.push(encoded[i]);
            i += 1;
        }
    }
    serde_json::from_slice(&bytes).unwrap()
}
#[test]
fn preview_url_shapes_roundtrip_utf8_without_transport() -> Result<(), Box<dyn std::error::Error>> {
    let block: Block = SectionBlock::builder()
        .text("🙂 / + # ? & ~")
        .build()?
        .into();
    let url = block_kit_builder_url(&block, Some("T1/other"))?;
    assert!(url.starts_with("https://app.slack.com/block-kit-builder/T1%2Fother#"));
    assert!(url.contains("%F0%9F%99%82"));
    assert_eq!(decode_fragment(&url), json!({"blocks":[block.clone()]}));
    let blocks = vec![block];
    assert_eq!(
        decode_fragment(&block_kit_builder_url(blocks.as_slice(), None)?),
        json!({"blocks":blocks})
    );
    let message = MessagePayload::builder("C1")
        .blocks(blocks.clone())
        .build()?;
    let webhook = WebhookMessage::builder().blocks(blocks.clone()).build()?;
    let response = MessageResponse::builder().blocks(blocks.clone()).build()?;
    let modal = ModalView::builder()
        .title("Preview")
        .blocks(blocks.clone())
        .build()?;
    let home = HomeTabView::builder().blocks(blocks).build()?;
    for (input, expected) in [
        (BuilderPayload::from(&message), wire(&message)),
        (BuilderPayload::from(&webhook), wire(&webhook)),
        (BuilderPayload::from(&response), wire(&response)),
        (BuilderPayload::from(&modal), wire(&modal)),
        (BuilderPayload::from(&home), wire(&home)),
    ] {
        assert_eq!(
            decode_fragment(&block_kit_builder_url(input, None)?),
            expected
        );
    }
    let raw = json!({"future":false,"unknown":null});
    assert_eq!(
        decode_fragment(&block_kit_builder_url(
            BuilderPayload::Raw(raw.as_object().unwrap()),
            Some("")
        )?),
        raw
    );
    Ok(())
}
