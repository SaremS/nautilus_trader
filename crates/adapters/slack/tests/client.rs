use std::{collections::HashMap, net::SocketAddr, time::Duration};

use axum::{Json, Router, routing::post};
use rstest::rstest;
use serde_json::json;

use nautilus_common::testing::wait_until_async;
use nautilus_network::http::HttpClient;
use nautilus_slack::client::SlackClient;

async fn wait_for_server(addr: SocketAddr, path: &str) {
    let health_url = format!("http://{addr}{path}");
    let http_client =
        HttpClient::new(HashMap::new(), Vec::new(), Vec::new(), None, None, None).unwrap();
    wait_until_async(
        || {
            let url = health_url.clone();
            let client = http_client.clone();
            async move { client.get(url, None, None, Some(1), None).await.is_ok() }
        },
        Duration::from_secs(5),
    )
    .await;
}

#[rstest]
#[tokio::test]
async fn test_slack_client_send_message() {
    let router = Router::new().route(
        "/api/chat.postMessage",
        post(|| async {
            Json(json!({
                "ok": true
            }))
        }),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router.into_make_service())
            .await
            .unwrap();
    });

    wait_for_server(addr, "/api/chat.postMessage").await;

    let base_url = format!("http://{addr}");
    let client = SlackClient::new_with_base_url(base_url, "channel_id", "api_key").unwrap();

    let result = client.send_message("Hello, Slack!").await;

    assert!(result.is_ok());
}
