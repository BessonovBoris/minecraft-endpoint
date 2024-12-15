use axum::{response::{IntoResponse, Json}, routing::{get}, Router, extract::{Query, State}};
use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_order_by_uid));

    let listener = tokio::net::TcpListener::bind("localhost:8000").await.unwrap_or_else(|err| {
        eprintln!("Can't bind to localhost:8000: {}", err);
        std::process::exit(1);
    });

    axum::serve(listener, app).await.expect("Can't start server");
}

async fn get_order_by_uid() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok("Hello, World!")
}