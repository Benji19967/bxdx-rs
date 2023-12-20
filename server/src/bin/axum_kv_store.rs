use std::collections::HashMap;

use axum::{
    extract::Json,
    extract::Path,
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use std::sync::Arc;
use std::sync::RwLock;

#[derive(Deserialize)]
struct KvEntry {
    key: String,
    value: String,
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    db: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let shared_state = SharedState::default();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/kv/:key", get(get_key))
        .route("/kv", post(insert_key))
        .with_state(Arc::clone(&shared_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Alive!"
}

async fn get_key(
    // access the state via the `State` extractor
    // extracting a state of the wrong type results in a compile error
    State(state): State<SharedState>,
    Path(key): Path<String>,
) -> Result<String, StatusCode> {
    if let Some(value) = state.read().unwrap().db.get(&key) {
        Ok(value.clone())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn insert_key(State(state): State<SharedState>, Json(payload): Json<KvEntry>) {
    println!("Inserted {}:{}", &payload.key, &payload.value);
    state.write().unwrap().db.insert(payload.key, payload.value);
}
