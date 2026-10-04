use slackblocks::{MessagePayload, SectionBlock};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = MessagePayload::builder("C0123456")
        .block(
            SectionBlock::builder()
                .text("Hello from slackblocks!")
                .block_id("hello")
                .build()?,
        )
        .build()?;
    println!("{}", serde_json::to_string(&message)?);
    Ok(())
}
