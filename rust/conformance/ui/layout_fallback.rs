use slackblocks::*;
fn main() { LayoutBlock::builder().fallback(vec![ActionsBlock::builder().build().unwrap()]); }
