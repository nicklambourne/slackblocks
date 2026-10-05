//! Send a validated payload with an application-owned HTTP client.
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use slackblocks::{MessagePayload, SectionBlock};
use std::time::Duration;

const MAX_RATE_LIMIT_RETRIES: usize = 2;

#[derive(Debug, Deserialize)]
pub struct PostedMessage {
    pub channel: String,
    pub ts: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("Slack rate limit exceeded (Retry-After: {retry_after:?})")]
    RateLimited { retry_after: Option<Duration> },
    #[error("Slack rejected the message: {0}")]
    Api(String),
    #[error("Slack returned an invalid response")]
    InvalidResponse,
}

/// Retry only explicit 429 rejections, never an ambiguous timeout or server error.
/// `api_url` is Slack's API base URL in production; tests use a local mock server.
pub async fn send_message(
    client: &Client,
    api_url: &str,
    token: &str,
    message: &MessagePayload,
) -> Result<PostedMessage, SendError> {
    let mut retries = 0;
    loop {
        let response = client
            .post(format!("{api_url}/chat.postMessage"))
            .bearer_auth(token)
            .json(message)
            .send()
            .await?;
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|value| value.to_str().ok()?.parse::<u64>().ok())
                .map(Duration::from_secs);
            if retries < MAX_RATE_LIMIT_RETRIES {
                if let Some(delay) = retry_after {
                    // Release the response before sleeping; do not retry without a delay.
                    drop(response);
                    retries += 1;
                    tokio::time::sleep(delay).await;
                    continue;
                }
            }
            return Err(SendError::RateLimited { retry_after });
        }
        let body: serde_json::Value = response.error_for_status()?.json().await?;
        match body.get("ok").and_then(serde_json::Value::as_bool) {
            Some(true) => {
                return serde_json::from_value(body).map_err(|_| SendError::InvalidResponse);
            }
            Some(false) => {
                let error = body
                    .get("error")
                    .and_then(serde_json::Value::as_str)
                    .ok_or(SendError::InvalidResponse)?;
                return Err(SendError::Api(error.to_owned()));
            }
            None => return Err(SendError::InvalidResponse),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("SLACK_BOT_TOKEN")?;
    let channel = std::env::var("SLACK_CHANNEL_ID")?;
    let message = MessagePayload::builder(channel)
        .text("Deployment complete")
        .block(
            SectionBlock::builder()
                .text("Deployment complete")
                .build()?,
        )
        .build()?;
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    // Bound the whole operation, including Retry-After waits.
    let posted = tokio::time::timeout(
        Duration::from_secs(60),
        send_message(&client, "https://slack.com/api", &token, &message),
    )
    .await??;
    println!("Posted {} to {}", posted.ts, posted.channel);
    Ok(())
}
