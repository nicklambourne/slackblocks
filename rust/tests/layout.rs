use slackblocks::*;

fn grid() -> Result<LayoutBlock, ValidationError> {
    LayoutBlock::builder()
        .label("Grid")
        .row(
            LayoutRow::builder()
                .item(
                    LayoutItem::builder()
                        .block(DividerBlock::builder().build()?)
                        .build()?,
                )
                .build()?,
        )
        .build()
}
#[test]
fn boundaries_fallback_and_native_editing() -> Result<(), ValidationError> {
    let g = grid()?;
    MessagePayload::builder("C123")
        .blocks(vec![g.clone(); 25])
        .build()?;
    HomeTabView::builder().blocks(vec![g.clone(); 50]).build()?;
    let changed = g
        .clone()
        .into_builder()
        .fallback(vec![DividerBlock::builder().build()?; 50])
        .is_label_visible(false)
        .grid_lines(LayoutGridLines::Rows)
        .build()?;
    MessagePayload::builder("C123")
        .block(changed.clone())
        .build()?;
    assert_eq!(g.fallback(), None);
    assert_eq!(changed.is_label_visible(), Some(false));
    assert_eq!(changed.fallback().unwrap().len(), 50);
    let existing = ContainerBlock::builder()
        .title("Existing")
        .child_blocks(vec![DividerBlock::builder().build()?; 10])
        .build()?;
    MessagePayload::builder("C123")
        .blocks(vec![existing; 50])
        .build()?;
    let item = LayoutItem::builder()
        .block(DividerBlock::builder().build()?)
        .width(LayoutWidth::Wide)
        .horizontal_alignment(LayoutAlignment::End)
        .build()?;
    assert_eq!(item.width(), Some(LayoutWidth::Wide));
    assert_eq!(item.horizontal_alignment(), Some(LayoutAlignment::End));
    assert_eq!(
        LayoutItem::try_from(serde_json::to_value(&item).unwrap())?,
        item
    );
    Ok(())
}
