use serde::Deserialize;
use tower_http::trace::TraceLayer;
use std::fs;

use axum::{debug_handler, extract::State, routing::get, Json, Router};

use serde_json::{json, Value};
use tracing_subscriber;

const EMPTY: &'static str = "";

#[derive(Deserialize, Default)]
struct Password {
    wifi: String,
}

#[derive(Deserialize, Default)]
struct Config {
    passwords: Password,
}

#[debug_handler]
async fn res(State(state): State<String>) -> Json<Value> {
    Json(json!({"password": state}))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let file_path = if cfg!(target_os = "macos") {
        "config.toml"
    }else{
        "/usr/bin/config.toml"
    };

    let content = match fs::read_to_string(file_path) {
        Ok(f) => f,
        Err(_) => EMPTY.to_string()
    };

    let data: Config = match toml::from_str(&content) {
        Ok(d) => d,
        Err(_) => Config::default()
    };

    let app = Router::new()
        .route("/", get(res))
        .with_state(data.passwords.wifi)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
