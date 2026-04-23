use std::sync::{
    Arc, 
    atomic::{AtomicUsize, Ordering}
};

use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};


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
        let mut count = self.connection_count.store(0, Ordering::Relaxed);

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
    Json(payload): Json<serde_json::Value>
) -> Json<serde_json::Value> {
    state.increment_connection_count().await;
    if let Some(text) = payload.get("text").and_then(|v| v.as_str()) {
        state.add_message(text.to_string()).await;
    }
    Json(serde_json::json!({
        "ok": true
    }))
}

async fn handle_api_test(
    State(state): State<TestServerState>
) -> Json<serde_json::Value> {
    state.increment_connection_count().await;

    Json(serde_json::json!({
        "ok": true
    }))
}

fn create_test_router(State(state): State<TestServerState>) -> axum::Router {
    Router::new()
        .route("/api/chat.postMessage", post(handle_api_post_message))
        .route("/api/api.test", get(handle_api_test))
        .with_state(state)
}
