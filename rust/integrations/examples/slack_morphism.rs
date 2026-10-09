//! Keep Slack Morphism's authenticated transport and send slackblocks values directly.
use serde::Deserialize;
use slack_morphism::prelude::*;
use slackblocks::{MessagePayload, SectionBlock};
use std::time::Duration;

// Deserialize only the receipt, so new blocks echoed by Slack do not need SDK models.
#[derive(Debug, Deserialize)]
pub struct PostedMessage {
    pub channel: String,
    pub ts: String,
}

pub fn https_connector() -> std::io::Result<SlackClientHyperHttpsConnector> {
    // When both clients share an application, reqwest and Slack Morphism enable
    // different rustls providers. Choose ring unless the application chose one.
    let _ = rustls::crypto::ring::default_provider().install_default();
    SlackClientHyperConnector::new()
}

pub fn rate_control() -> SlackApiRateControlConfig {
    SlackApiRateControlConfig::new().with_max_retries(2)
}

pub async fn send_message<C>(
    session: &SlackClientSession<'_, C>,
    message: &MessagePayload,
) -> ClientResult<PostedMessage>
where
    C: SlackClientHttpConnector + Send,
{
    // The typed chat_post_message method requires Slack Morphism's own block types.
    // Its generic transport accepts Serialize and preserves all our payload fields.
    session
        .http_session_api
        .http_post(
            "chat.postMessage",
            message,
            Some(&CHAT_POST_MESSAGE_SPECIAL_LIMIT_RATE_CTL),
        )
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = SlackApiToken::new(std::env::var("SLACK_BOT_TOKEN")?.into())
        .with_team_id(std::env::var("SLACK_TEAM_ID")?.into());
    let channel = std::env::var("SLACK_CHANNEL_ID")?;
    let message = MessagePayload::builder(channel)
        .text("Deployment complete")
        .block(
            SectionBlock::builder()
                .text("Deployment complete")
                .build()?,
        )
        .build()?;
    let client = SlackClient::new(https_connector()?.with_rate_control(rate_control()));
    let session = client.open_session(&token);
    // Bound the whole operation, including the SDK's rate-limit retries.
    let posted =
        tokio::time::timeout(Duration::from_secs(60), send_message(&session, &message)).await??;
    println!("Posted {} to {}", posted.ts, posted.channel);
    Ok(())
}
