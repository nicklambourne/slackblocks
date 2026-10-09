// Import the actual runnable examples, so documentation and network tests cannot drift.
#[allow(dead_code)]
#[path = "../examples/slack_morphism.rs"]
mod morphism_example;
#[allow(dead_code)]
#[path = "../examples/reqwest.rs"]
mod reqwest_example;

use hyper_util::client::legacy::connect::HttpConnector;
use serde_json::{Value, json};
use slack_morphism::errors::SlackClientError;
use slack_morphism::prelude::*;
use slackblocks::{MessagePayload, SectionBlock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn message() -> MessagePayload {
    MessagePayload::builder("C01234567")
        .text("Deployment ✅")
        .unfurl_links(false)
        .block(
            SectionBlock::builder()
                .text("Deployment ✅")
                .extension(
                    "future_field",
                    json!({"large": u64::MAX, "null": null, "empty": []}),
                )
                .build()
                .unwrap(),
        )
        .extension("thread_ts", "1234567890.123456")
        .build()
        .unwrap()
}

fn success() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "ok": true, "channel": "C01234567", "ts": "1234567890.123457",
        // The returned message is deliberately outside both libraries' model sets.
        "message": {"blocks": [{"type": "future_block", "large": u64::MAX}]}
    }))
}

fn rate_limited(delay: &str) -> ResponseTemplate {
    ResponseTemplate::new(429)
        .insert_header("Retry-After", delay)
        .set_body_json(json!({"ok": false, "error": "ratelimited"}))
}

async fn mount(server: &MockServer, response: ResponseTemplate, calls: u64) {
    Mock::given(method("POST"))
        .and(path("/chat.postMessage"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(response)
        .expect(calls)
        .mount(server)
        .await;
}

fn reqwest_client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

fn morphism_client(server: &MockServer) -> SlackClient<SlackClientHyperConnector<HttpConnector>> {
    // Swap only the connector's TLS layer/URL for local HTTP; retain the real SDK
    // serialization, authentication, response parsing and production retry policy.
    SlackClient::new(
        SlackClientHyperConnector::with_connector(HttpConnector::new())
            .with_slack_api_url(&server.uri())
            .with_rate_control(morphism_example::rate_control()),
    )
}

fn token() -> SlackApiToken {
    SlackApiToken::new("test-token".into()).with_team_id("T01234567".into())
}

async fn assert_payloads(server: &MockServer, count: usize) {
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), count);
    let expected = json!({
        "channel": "C01234567", "text": "Deployment ✅", "mrkdwn": true,
        "unfurl_links": false, "thread_ts": "1234567890.123456",
        "blocks": [{"type": "section", "text": {"type": "mrkdwn", "text": "Deployment ✅"},
            "future_field": {"large": u64::MAX, "null": null, "empty": []}}]
    });
    for request in requests {
        assert!(
            request.headers["content-type"]
                .to_str()
                .unwrap()
                .starts_with("application/json")
        );
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body, expected);
    }
}

#[tokio::test]
async fn reqwest_preserves_payload_and_accepts_future_blocks_in_response() {
    let server = MockServer::start().await;
    mount(&server, success(), 1).await;
    let receipt =
        reqwest_example::send_message(&reqwest_client(), &server.uri(), "test-token", &message())
            .await
            .unwrap();
    assert_eq!(receipt.channel, "C01234567");
    assert_eq!(receipt.ts, "1234567890.123457");
    assert_payloads(&server, 1).await;
}

#[tokio::test]
async fn morphism_preserves_payload_and_accepts_future_blocks_in_response() {
    let server = MockServer::start().await;
    mount(&server, success(), 1).await;
    let client = morphism_client(&server);
    let token = token();
    let receipt = morphism_example::send_message(&client.open_session(&token), &message())
        .await
        .unwrap();
    assert_eq!(receipt.channel, "C01234567");
    assert_eq!(receipt.ts, "1234567890.123457");
    assert_payloads(&server, 1).await;
}

#[tokio::test]
async fn production_tls_clients_can_coexist() {
    // Catch feature-unification conflicts before a consumer ever contacts Slack.
    morphism_example::https_connector().unwrap();
    reqwest_client();
}

#[tokio::test]
async fn reqwest_reports_slack_error_without_retry() {
    let server = MockServer::start().await;
    mount(
        &server,
        ResponseTemplate::new(200).set_body_json(json!({"ok": false, "error": "invalid_blocks"})),
        1,
    )
    .await;
    let error =
        reqwest_example::send_message(&reqwest_client(), &server.uri(), "test-token", &message())
            .await
            .unwrap_err();
    assert!(matches!(error, reqwest_example::SendError::Api(code) if code == "invalid_blocks"));
}

#[tokio::test]
async fn morphism_reports_slack_error_without_retry() {
    let server = MockServer::start().await;
    mount(
        &server,
        ResponseTemplate::new(200).set_body_json(json!({"ok": false, "error": "invalid_blocks"})),
        1,
    )
    .await;
    let client = morphism_client(&server);
    let token = token();
    let error = morphism_example::send_message(&client.open_session(&token), &message())
        .await
        .unwrap_err();
    assert!(matches!(error, SlackClientError::ApiError(error) if error.code == "invalid_blocks"));
}

#[tokio::test]
async fn reqwest_reports_http_errors_without_retry() {
    for status in [401, 500, 503] {
        let server = MockServer::start().await;
        mount(
            &server,
            ResponseTemplate::new(status).set_body_string("unavailable"),
            1,
        )
        .await;
        let error = reqwest_example::send_message(
            &reqwest_client(),
            &server.uri(),
            "test-token",
            &message(),
        )
        .await
        .unwrap_err();
        assert!(
            matches!(error, reqwest_example::SendError::Http(error) if error.status().unwrap().as_u16() == status)
        );
    }
}

#[tokio::test]
async fn morphism_reports_http_errors_without_retry() {
    for status in [401, 500, 503] {
        let server = MockServer::start().await;
        mount(
            &server,
            ResponseTemplate::new(status).set_body_string("unavailable"),
            1,
        )
        .await;
        let client = morphism_client(&server);
        let token = token();
        let error = morphism_example::send_message(&client.open_session(&token), &message())
            .await
            .unwrap_err();
        assert!(
            matches!(error, SlackClientError::HttpError(error) if error.status_code.as_u16() == status)
        );
    }
}

async fn mount_retry(server: &MockServer) {
    let calls = AtomicUsize::new(0);
    Mock::given(method("POST"))
        .and(path("/chat.postMessage"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(move |_: &Request| {
            if calls.fetch_add(1, Ordering::SeqCst) == 0 {
                rate_limited("1")
            } else {
                success()
            }
        })
        .expect(2)
        .mount(server)
        .await;
}

#[tokio::test]
async fn reqwest_waits_for_retry_after_and_replays_identical_payload() {
    let server = MockServer::start().await;
    mount_retry(&server).await;
    let start = Instant::now();
    reqwest_example::send_message(&reqwest_client(), &server.uri(), "test-token", &message())
        .await
        .unwrap();
    assert!(start.elapsed() >= Duration::from_secs(1));
    assert_payloads(&server, 2).await;
}

#[tokio::test]
async fn morphism_waits_for_retry_after_and_replays_identical_payload() {
    let server = MockServer::start().await;
    mount_retry(&server).await;
    let client = morphism_client(&server);
    let token = token();
    let start = Instant::now();
    morphism_example::send_message(&client.open_session(&token), &message())
        .await
        .unwrap();
    assert!(start.elapsed() >= Duration::from_secs(1));
    assert_payloads(&server, 2).await;
}

#[tokio::test]
async fn reqwest_exhausts_rate_limit_retries() {
    let server = MockServer::start().await;
    mount(&server, rate_limited("0"), 3).await;
    let error =
        reqwest_example::send_message(&reqwest_client(), &server.uri(), "test-token", &message())
            .await
            .unwrap_err();
    assert!(
        matches!(error, reqwest_example::SendError::RateLimited { retry_after: Some(delay) } if delay.is_zero())
    );
    assert_payloads(&server, 3).await;
}

#[tokio::test]
async fn morphism_exhausts_rate_limit_retries() {
    let server = MockServer::start().await;
    mount(&server, rate_limited("0"), 3).await;
    let client = morphism_client(&server);
    let token = token();
    let error = morphism_example::send_message(&client.open_session(&token), &message())
        .await
        .unwrap_err();
    assert!(
        matches!(error, SlackClientError::RateLimitError(error) if error.retry_after == Some(Duration::ZERO))
    );
    assert_payloads(&server, 3).await;
}

#[tokio::test]
async fn reqwest_missing_or_invalid_retry_after_is_not_retried() {
    for delay in [None, Some("invalid"), Some("-1")] {
        let server = MockServer::start().await;
        let response = match delay {
            Some(delay) => rate_limited(delay),
            None => ResponseTemplate::new(429),
        };
        mount(&server, response, 1).await;
        let error = reqwest_example::send_message(
            &reqwest_client(),
            &server.uri(),
            "test-token",
            &message(),
        )
        .await
        .unwrap_err();
        assert!(matches!(
            error,
            reqwest_example::SendError::RateLimited { retry_after: None }
        ));
    }
}

#[tokio::test]
async fn both_clients_reject_malformed_responses() {
    for body in ["not json", "{}", r#"{"ok":true}"#] {
        let server = MockServer::start().await;
        mount(
            &server,
            ResponseTemplate::new(200).set_body_raw(body, "application/json"),
            2,
        )
        .await;
        assert!(
            reqwest_example::send_message(
                &reqwest_client(),
                &server.uri(),
                "test-token",
                &message()
            )
            .await
            .is_err()
        );
        let client = morphism_client(&server);
        let token = token();
        assert!(
            morphism_example::send_message(&client.open_session(&token), &message())
                .await
                .is_err()
        );
    }
}

#[tokio::test]
async fn reqwest_does_not_forward_payload_on_redirect() {
    let server = MockServer::start().await;
    let other = MockServer::start().await;
    mount(
        &server,
        ResponseTemplate::new(307).insert_header("Location", format!("{}/elsewhere", other.uri())),
        1,
    )
    .await;
    assert!(
        reqwest_example::send_message(&reqwest_client(), &server.uri(), "test-token", &message())
            .await
            .is_err()
    );
    assert!(other.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn both_clients_obey_application_deadline() {
    let server = MockServer::start().await;
    mount(&server, rate_limited("120"), 2).await;
    let deadline = Duration::from_millis(500);
    assert!(
        tokio::time::timeout(
            deadline,
            reqwest_example::send_message(
                &reqwest_client(),
                &server.uri(),
                "test-token",
                &message()
            )
        )
        .await
        .is_err()
    );
    let client = morphism_client(&server);
    let token = token();
    assert!(
        tokio::time::timeout(
            deadline,
            morphism_example::send_message(&client.open_session(&token), &message())
        )
        .await
        .is_err()
    );
    // A cancelled Retry-After wait must not leave a detached retry running.
    assert_payloads(&server, 2).await;
}
