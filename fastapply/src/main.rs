use axum::{routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define our routes
    let app = Router::new()
        .route("/status", get(status_check))
        .route("/process-code", post(process[[8](https://www.google.com/url?sa=E&q=https%3A%2F%2Fvertexaisearch.cloud.google.com%2Fgrounding-api-redirect%2FAUZIYQHsMNbUHE_J0BXVSVEBhMKJSxy-20E3_r70pX6kfASim-AW9JYdFoVDxsd5h13nruNN6lBaiKyK5rPpzEQbmV4OdhloLfIG8DUkeJU2iJFslo_G2qBpZjupZ0jtPhR0OwEr15TVAW5L_7GXfspB0Yfe3UKDR4IWV55pgFK7sIDnQGdHq2hY88H0)]_code));

    // Run on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("AI Sidecar listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn status_check() -> &'static str {
    "AI Sidecar is Ready"
}

#[derive(Deserialize)]
struct CodeRequest {
    file_path: String,
    content: String,
}

#[derive(Serialize)]
struct CodeResponse {
    relevant_lines: String,
}

// This is where we will eventually add the "Tree Sitter" logic
async fn process_code(Json(payload): Json<CodeRequest>) -> Json<CodeResponse> {
    println!("Received file: {}", payload.file_path);

    // Mock response for now
    Json(CodeResponse {
        relevant_lines: "Analyzed 20,000 lines... found relevant section at 500-550".to_string(),
    })
}
