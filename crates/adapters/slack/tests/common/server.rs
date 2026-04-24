#![allow(dead_code)]

use std::{
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use axum::{Json, Router, extract::State, routing::post};

use nautilus_common::testing::wait_until_async;

#[derive(Clone)]
pub struct TestServerState {
    pub connection_count: Arc<AtomicUsize>,
    pub messages_received: Arc<tokio::sync::Mutex<Vec<String>>>,
}

impl Default for TestServerState {
    fn default() -> Self {
        Self {
            connection_count: Arc::new(AtomicUsize::new(0)),
            messages_received: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }
}

impl TestServerState {
    pub async fn reset(&self) {
        let _count = self.connection_count.store(0, Ordering::Relaxed);

        let mut messages = self.messages_received.lock().await;
        messages.clear();
    }

    pub async fn increment_connection_count(&self) {
        self.connection_count.fetch_add(1, Ordering::Relaxed);
    }

    pub async fn add_message(&self, message: String) {
        let mut messages = self.messages_received.lock().await;
        messages.push(message);
    }
}

async fn handle_api_post_message(
    State(state): State<TestServerState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    state.increment_connection_count().await;
    if let Some(text) = payload.get("text").and_then(|v| v.as_str()) {
        state.add_message(text.to_string()).await;
    }
    Json(serde_json::json!({
        "ok": true
    }))
}

async fn handle_api_test(State(state): State<TestServerState>) -> Json<serde_json::Value> {
    state.increment_connection_count().await;

    Json(serde_json::json!({
        "ok": true
    }))
}

fn create_test_router(state: TestServerState) -> axum::Router {
    Router::new()
        .route("/api/chat.postMessage", post(handle_api_post_message))
        .route("/api/api.test", post(handle_api_test))
        .with_state(state)
}

pub async fn start_test_server()
-> Result<(SocketAddr, TestServerState), Box<dyn std::error::Error + Send + Sync>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let state = TestServerState::default();
    let router = create_test_router(state.clone());

    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    wait_until_async(
        || async { tokio::net::TcpStream::connect(addr).await.is_ok() },
        Duration::from_secs(5),
    )
    .await;

    Ok((addr, state))
}
